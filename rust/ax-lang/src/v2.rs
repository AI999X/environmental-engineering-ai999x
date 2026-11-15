//! Language layer version 2 (docs/RECIPE_1_6.md part A): our own tiny BERT-style networks classify a question into a
//! book cell (concept x question type), a calculation, or unknown. The reply is the reviewed cell word for word, the
//! engine's result, or "I don't know that one." Nothing is generated.
//!
//! Package folder (`<lang package>/v2/`): tokenizer.json (one vocabulary shared by every member), labels.json (label
//! lists, cap_inputs, cap_outputs, twins, direction words, thresholds), book.json ({concept: {name, capabilities,
//! cells: {type: text}, vocab: [words]}}), and one folder per ensemble member `m0/`, `m1/`, ... with config.json
//! (BertConfig) and model.safetensors ("bert.*" + heads). A single model may also sit directly in `v2/`.
//!
//! Decision for a stored answer (each step measured on validation, docs/OVERHAUL_1_6_PLAN.md):
//!   1. the members' probabilities are averaged; every member must pick the same type and the same concept (or a twin)
//!   2. the averaged confidence (the least sure of intent, concept, type) must clear the calibrated threshold
//!   3. grounding: the question must share a word with the chosen concept's vocabulary (names, aliases, everyday
//!      words, cloud terms); an off-topic question rarely does
//!   4. direction: a question that only says "less" never gets a `more` cell, and the reverse
use candle_core::{DType, Device, Tensor, D};
use candle_nn::{linear, Linear, Module, VarBuilder};
use candle_transformers::models::bert::{BertModel, Config};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::{candidates, compute_reply, dont_know, numbers, Reading};
use ax_engine::pack::Pack;

struct Member {
    bert: BertModel,
    intent: Linear,
    concept: Linear,
    typ: Linear,
    capability: Linear,
    asked: Linear,
    tags: Linear,
}

/// Book questions answered by rule (recipe A23; `v2/book_rule.json`, written by tools/v2_book_rule.py, which is the
/// twin of this reader): a question that names a topic and asks in a recognisable way gets that topic's cell.
pub struct BookRule {
    /// (topic, strong phrases: name / aliases / everyday words, weak phrases: cloud terms, "not this" terms)
    topics: Vec<(String, Vec<Vec<String>>, Vec<Vec<String>>, Vec<(Vec<String>, String)>)>,
    /// each topic's own name and alias words: a cloud term counts only with one of them in the question
    name_words: HashMap<String, HashSet<String>>,
    /// the kinds of question in order of precedence, each with its cue phrases
    cues: Vec<(String, Vec<String>)>,
    weak_kinds: HashSet<String>,
    plain: HashSet<String>,
}

fn words_list(v: &Value) -> Vec<String> {
    v.as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect()
}

impl BookRule {
    fn load(v: &Value) -> BookRule {
        let mut topics = vec![];
        let mut name_words = HashMap::new();
        for (k, t) in v["topics"].as_object().into_iter().flatten() {
            name_words.insert(k.clone(), words_list(&t["name_words"]).into_iter().collect());
            let ph = |key: &str| -> Vec<Vec<String>> { t[key].as_array().into_iter().flatten().map(words_list).collect() };
            let nt = t["not_this"].as_array().into_iter().flatten()
                .map(|x| (words_list(&x["words"]), x["means"].as_str().unwrap_or("").to_string())).collect();
            topics.push((k.clone(), ph("phrases"), ph("weak"), nt));
        }
        topics.sort_by(|a, b| a.0.cmp(&b.0));
        let cues = words_list(&v["order"]).into_iter().map(|k| { let c = words_list(&v["cues"][&k]); (k, c) }).collect();
        BookRule { topics, name_words, cues, weak_kinds: words_list(&v["weak_kinds"]).into_iter().collect(),
                   plain: words_list(&v["plain"]).into_iter().collect() }
    }

    /// The words the book reader compares: content words and short symbols (a token of one or two letters that is not
    /// an ordinary English word: "y", "ct", "uv"), the twin of tools/v2_book_rule.py `words`.
    fn words(t: &str) -> HashSet<String> {
        const SHORT_OK: &[&str] = &["a", "i", "is", "it", "of", "in", "on", "to", "do", "an", "or", "be", "my", "me", "we",
            "so", "if", "at", "as", "by", "up", "us", "no", "go", "ok", "am", "he", "oh", "hi", "vs"];
        let mut out = Self::book_content(t);
        let low = t.to_lowercase().replace('’', "'");
        for w in low.split(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit())).filter(|w| !w.is_empty()) {
            if w.len() <= 2 && !w.chars().all(|c| c.is_ascii_digit()) && !SHORT_OK.contains(&w) {
                out.insert(w.to_string());
            }
        }
        // names made of letters and digits ("co2", "pm25", "pm10"): words, not numbers
        for w in Self::pm_norm(t).split(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit())).filter(|w| !w.is_empty()) {
            if w.chars().any(|c| c.is_ascii_lowercase()) && w.chars().any(|c| c.is_ascii_digit()) {
                out.insert(w.to_string());
            }
        }
        out
    }

    /// Lowercase, and a name with a decimal number as one token: "PM2.5" and "PM 2.5" are "pm25" (the twin of
    /// tools/v2_book_rule.py `pm_norm`).
    fn pm_norm(t: &str) -> String {
        let low = t.to_lowercase().replace('’', "'");
        let re = fancy_regex::Regex::new(r"\b([a-z]+) ?(\d+)\.(\d+)\b").expect("pm");
        re.replace_all(&low, "$1$2$3").into_owned()
    }

    /// A number standing on its own (a value: the question is a calculation), not one inside a name like PM2.5.
    pub fn free_number(t: &str) -> bool {
        Self::pm_norm(t).split(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.'))
            .any(|w| w.chars().any(|c| c.is_ascii_digit()) && w.chars().all(|c| c.is_ascii_digit() || c == '.'))
    }

    /// `content_words` with the book reader's own stem: "-ies" becomes "-y" ("factories" is "factory", not "factor"),
    /// the twin of tools/v2_book_rule.py `book_content`.
    fn book_content(t: &str) -> HashSet<String> {
        let bstem = |w: &str| if w.len() > 4 && w.ends_with("ies") { format!("{}y", &w[..w.len() - 3]) } else { stem(w) };
        let mut out = HashSet::new();
        let mut cur = String::new();
        for ch in t.to_lowercase().chars().chain(std::iter::once(' ')) {
            if ch.is_ascii_lowercase() || ((ch == '\'' || ch == '-') && !cur.is_empty()) {
                cur.push(ch);
            } else if !cur.is_empty() {
                let x = cur.trim_matches(|c| c == '\'' || c == '-').to_string();
                cur.clear();
                if x.len() >= 3 && x.chars().next().is_some_and(|c| c.is_ascii_lowercase()) && !STOP.contains(&x.as_str()) {
                    out.insert(bstem(&x));
                }
            }
        }
        out
    }

    /// The longest of the topic's own (strong) phrases the question contains, in words (0: none), the twin of
    /// tools/v2_book_rule.py `phrase_len`.
    pub fn phrase_len(&self, q: &str, cid: &str) -> usize {
        let qw = Self::words(q);
        self.topics.iter().find(|t| t.0 == cid)
            .map(|t| t.1.iter().filter(|ph| ph.iter().all(|w| qw.contains(w))).map(|ph| ph.len()).max().unwrap_or(0))
            .unwrap_or(0)
    }

    /// The question's words as " w1 w2 ... " (lowercase letters, digits, apostrophes), for whole-phrase cues.
    fn norm(q: &str) -> String {
        let low = q.to_lowercase().replace('’', "'");
        let toks: Vec<&str> = low.split(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '\''))
            .filter(|s| !s.is_empty()).collect();
        format!(" {} ", toks.join(" "))
    }

    /// (topic, kind) the question names and asks, or why not. Cloud (weak) phrases count only for `net_topic`, the
    /// network's own top topic. Refused: no topic or two topics named equally, two kinds of question cued, or a word
    /// that is neither the topic's, a cue's nor a plain question word.
    pub fn read(&self, v2: &V2, q: &str, net_topic: Option<&str>) -> Result<(String, String), String> {
        if Self::free_number(q) {
            return Err("numbers".into());
        }
        let qw = Self::words(q);
        if qw.is_empty() {
            return Err("no words".into());
        }
        let mut best: Vec<(String, usize, Vec<String>)> = vec![];
        for (cid, strong, weak, _) in &self.topics {
            // (a cloud term only for the networks' topic, and only with a word of the topic's own name or aliases in
            // the question: "how does a heat pump work" names no word of "Noisy machines, fans and air conditioners")
            let named = self.name_words.get(cid).is_some_and(|n| !n.is_disjoint(&qw));
            let same_as_net = named && net_topic.is_some_and(|t| v2.same_concept(t, cid));
            let mut b: Option<&Vec<String>> = None;
            for ph in strong.iter().chain(if same_as_net { weak.iter() } else { [].iter() }) {
                if ph.iter().all(|w| qw.contains(w)) && !(ph.len() == 1 && v2.general.contains(&ph[0]))
                    && b.is_none_or(|x| ph.len() > x.len()) {
                    b = Some(ph);
                }
            }
            if let Some(ph) = b {
                best.push((cid.clone(), ph.len(), ph.clone()));
            }
        }
        // a "not this" term written points the question at the other topic
        for (cid, _, _, nts) in &self.topics {
            for (nw, _) in nts {
                if nw.iter().all(|w| qw.contains(w)) {
                    best.retain(|b| !(b.0 == *cid && b.1 <= nw.len()));
                }
            }
        }
        let top = best.iter().map(|b| b.1).max().ok_or("no topic named")?;
        let mut groups: Vec<&(String, usize, Vec<String>)> = vec![];
        for b in best.iter().filter(|b| b.1 == top) {
            if !groups.iter().any(|g| v2.same_concept(&g.0, &b.0)) {
                groups.push(b);
            }
        }
        if groups.len() > 1 {
            return Err(format!("several topics named: {}", groups.iter().map(|g| g.0.as_str()).collect::<Vec<_>>().join(", ")));
        }
        let (cid, _, ph) = groups[0];
        // cues, except a cue whose words are all inside the topic's phrase ("Henry's LAW" asks no rules, "WORKING
        // face" asks no how)
        let n = Self::norm(q);
        let phrase: HashSet<String> = ph.iter().cloned().collect();
        let mut kinds: Vec<&str> = vec![];
        let mut used: HashSet<String> = phrase.clone();
        for (k, cs) in &self.cues {
            for c in cs {
                let cw = Self::book_content(c);
                if n.contains(&format!(" {c} ")) && !(!cw.is_empty() && cw.is_subset(&phrase)) {
                    if !kinds.contains(&k.as_str()) {
                        kinds.push(k);
                    }
                    used.extend(cw);
                }
            }
        }
        let strong: Vec<&str> = kinds.iter().copied().filter(|k| !self.weak_kinds.contains(*k)).collect();
        if strong.len() > 1 {
            return Err(format!("several kinds asked: {}", strong.join(", ")));
        }
        // "why does warm water have LESS oxygen" asks why, with a direction word in it
        if kinds.contains(&"why") && strong.iter().any(|k| *k == "more" || *k == "less") {
            return Err("why with more or less".into());
        }
        let kind = strong.first().or(kinds.first()).map(|k| k.to_string()).unwrap_or_else(|| "what".into());
        // every content word is the topic's, a cue's or a plain question word
        let extra: Vec<&String> = qw.iter().filter(|w| !used.contains(*w) && !self.plain.contains(*w)).collect();
        if !extra.is_empty() {
            return Err(format!("words not explained: {extra:?}"));
        }
        if v2.book[cid]["cells"][&kind].as_str().is_none_or(|t| t.is_empty()) {
            return Err(format!("no {kind} cell"));
        }
        Ok((cid.clone(), kind))
    }
}

/// One reviewed fact of a topic (recipe A31; `v2/facts.json` from tools/v2_facts.py, the Python twin).
struct Fact {
    cues: Vec<HashSet<String>>,
    answer: String,
    /// the key and its alternatives (`alt_keys`: other spellings or names of the same answer); an option holds the
    /// answer when it has every word of one of them
    keys: Vec<HashSet<String>>,
    not: Vec<HashSet<String>>,
    statement: String,
}

/// One stem for the forms of a word (the twin of v2_facts.fstem): increase / increases / increased -> increas.
fn fstem(w: &str) -> String {
    let n = w.len();
    let mut s = if n > 4 && w.ends_with("ies") { format!("{}y", &w[..n - 3]) }
        else if n > 5 && w.ends_with("ing") { w[..n - 3].to_string() }
        else if n > 4 && w.ends_with("ed") { w[..n - 2].to_string() }
        else if n > 3 && w.ends_with('s') && !w.ends_with("ss") { w[..n - 1].to_string() }
        else { w.to_string() };
    // (a final "e" always goes: zone / zones, rate / rates share one stem; the twin of v2_facts.fstem)
    if s.len() > 3 && s.ends_with('e') {
        s.pop();
    }
    s
}

/// A number in one form (the twin of v2_facts.fnum): 1.0e-7 and 0.0000001 -> "1e-7".
fn fnum(t: &str) -> String {
    let Ok(x) = t.parse::<f64>() else { return t.to_string() };
    if x == 0.0 {
        return "0e0".into();
    }
    let (mut m, mut e) = (x.abs(), 0i32);
    while m >= 10.0 { m /= 10.0; e += 1; }
    while m < 1.0 { m *= 10.0; e -= 1; }
    let mut s = format!("{:.4}", (m * 1e4).round() / 1e4);
    while s.contains('.') && (s.ends_with('0') || s.ends_with('.')) {
        s.pop();
    }
    if s == "10" {
        s = "1".into();
        e += 1;
    }
    format!("{}{s}e{e}", if x < 0.0 { "-" } else { "" })
}

/// A cue's words (the twin of v2_facts.CW): in a cue "do" is always dissolved oxygen (cues are typed in lowercase; in a
/// question lowercase "do" stays a verb). At worst a cue gets stricter.
fn cue_words(c: &str) -> HashSet<String> {
    use std::sync::OnceLock;
    static DO: OnceLock<fancy_regex::Regex> = OnceLock::new();
    fact_words(&DO.get_or_init(|| fancy_regex::Regex::new(r"\bdo\b").expect("do")).replace_all(c, "DO"))
}

/// The words the facts compare (the twin of v2_facts.W): hyphens and dashes split (a single letter joined by a hyphen
/// stays with its word: "A-weighting"), stop words out but more / less / not kept, one stem per word, numbers in one
/// form ("1.0 x 10^-7" = "1e-7").
pub fn fact_words(t: &str) -> HashSet<String> {
    use std::sync::OnceLock;
    const FSTOP: &[&str] = &["the", "a", "an", "of", "to", "in", "on", "at", "by", "for", "with", "and", "or", "is", "are",
        "was", "were", "be", "been", "it", "its", "this", "that", "these", "those", "which", "what", "how", "does", "do",
        "did", "will", "would", "can", "could", "from", "as", "into", "than", "then", "there", "their", "they", "them",
        "you", "your", "we", "our", "i", "me", "my", "if", "when", "where", "who", "whom", "whose", "has", "have", "had",
        "also", "about"];
    static RE: OnceLock<(fancy_regex::Regex, fancy_regex::Regex, fancy_regex::Regex)> = OnceLock::new();
    let (sci, join, _tok) = RE.get_or_init(|| (
        fancy_regex::Regex::new(r"(\d+(?:\.\d+)?)\s*(?:x|×|\*|·)\s*10\s*\^\s*\(?\s*([-+−]?)\s*(\d+)\s*\)?").expect("sci"),
        fancy_regex::Regex::new(r"\b([a-z])[-‐–](?=[a-z])").expect("join"),
        fancy_regex::Regex::new(r"\d+(?:\.\d+)?(?:e-?\d+)?|[a-z]+").expect("tok"),
    ));
    // "DO" in capitals is dissolved oxygen (lowercase "do" is a verb and a stop word)
    static DO: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let t = DO.get_or_init(|| fancy_regex::Regex::new(r"\bDO\b").expect("DO")).replace_all(t, "dissolved oxygen");
    // a letter range names the range, not its last letter: "the A to F thing" is not class F (2026-11-12)
    static RANGE: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let t = RANGE.get_or_init(|| fancy_regex::Regex::new(r"\b[A-Z]\s+(?:to|through|thru)\s+[A-Z]\b").expect("range"))
        .replace_all(&t, " ");
    let low = t.to_lowercase();
    let low = sci.replace_all(&low, |c: &fancy_regex::Captures| {
        format!("{}e{}{}", &c[1], if &c[2] == "-" || &c[2] == "−" { "-" } else { "" }, &c[3])
    }).into_owned();
    let low = join.replace_all(&low, "$1").into_owned();
    // "Type I", "Phase III": the numeral is a number ("i" alone is a stop word)
    static ROMAN: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let roman = ROMAN.get_or_init(|| fancy_regex::Regex::new(r"\b(type|phase|class|stage|category|level|tier)\s+(iv|v|iii|ii|i)\b").expect("roman"));
    let low = roman.replace_all(&low, |c: &fancy_regex::Captures| {
        let n = match &c[2] { "i" => "1", "ii" => "2", "iii" => "3", "iv" => "4", _ => "5" };
        format!("{} {n}", &c[1])
    }).into_owned();
    // a chemical formula is one word: "CO2" is not "co" and the number 2 (the twin of v2_facts.FORMULA, 2026-11-12)
    static FORMULA: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let formula = FORMULA.get_or_init(|| {
        let mut fs: Vec<&str> = "o2 n2 h2 o3 cl2 co2 ch4 h2o h2s so2 so3 so4 no2 no3 n2o nh3 nh4 hno3 h2so4 h2co3 hco3 co3 \
            po4 caco3 caso4 clo2 chcl3 ccl4 nh2cl nhcl2 ncl3 fe2o3 al2o3 c2h6 c3h8 c6h12o6 ch3oh c2h5oh mgco3 caoh2 naoh2"
            .split_whitespace().collect();
        fs.sort_by_key(|f| std::cmp::Reverse(f.len()));
        fancy_regex::Regex::new(&format!(r"(?<![a-z0-9.])(?:{})(?![a-z0-9])", fs.join("|"))).expect("formula")
    });
    let low = formula.replace_all(&low, "§$0§").into_owned();
    static TOKF: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let tok = TOKF.get_or_init(|| fancy_regex::Regex::new(r"§[a-z0-9]+§|\d+(?:\.\d+)?(?:e-?\d+)?|[a-z]+").expect("tokf"));
    let mut out = HashSet::new();
    for m in tok.find_iter(&low).flatten() {
        let w = m.as_str();
        if w.starts_with('§') {
            out.insert(w.trim_matches('§').to_string());
        } else if w.starts_with(|c: char| c.is_ascii_digit()) {
            out.insert(fnum(w));
        } else if !FSTOP.contains(&w) {
            out.insert(fstem(w));
        }
    }
    out
}

/// The reviewed facts by topic: multiple-choice and "which / what is the term" questions (recipe A31).
pub struct Facts {
    by_topic: HashMap<String, Vec<Fact>>,
}

impl Fact {
    /// A cue matches, and the answer is not already written in the question (then the question asks something else:
    /// "what model describes the oxygen sag curve?" is not answered "the oxygen sag curve"; the twin of v2_facts.applies)
    fn applies(&self, sw: &HashSet<String>) -> bool {
        self.cues.iter().any(|c| c.is_subset(sw)) && !self.keys.iter().any(|k| k.is_subset(sw))
    }
}

impl Facts {
    fn load(v: &Value) -> Facts {
        let mut by_topic = HashMap::new();
        for (cid, fs) in v.as_object().into_iter().flatten() {
            let list: Vec<Fact> = fs.as_array().into_iter().flatten().map(|f| Fact {
                cues: f["cues"].as_array().into_iter().flatten().filter_map(|c| c.as_str()).map(cue_words)
                    .filter(|c| !c.is_empty()).collect(),
                answer: f["answer"].as_str().unwrap_or("").to_string(),
                keys: std::iter::once(&f["key"]).chain(f["alt_keys"].as_array().into_iter().flatten())
                    .map(|k| fact_words(&words_list(k).join(" "))).filter(|k| !k.is_empty()).collect(),
                not: words_list(&f["not"]).iter().map(|n| fact_words(n)).filter(|n| !n.is_empty()).collect(),
                statement: f["statement"].as_str().unwrap_or("").to_string(),
            }).filter(|f| !f.keys.is_empty() && !f.cues.is_empty()).collect();
            by_topic.insert(cid.clone(), list);
        }
        Facts { by_topic }
    }
}

/// "Stem? A) x B) y C) z D) w" -> (stem, [(letter, option text)]); three options at least, lettered A, B, C, ...
pub fn parse_choices(q: &str) -> Option<(String, Vec<(char, String)>)> {
    use std::sync::OnceLock;
    static OPT: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let re = OPT.get_or_init(|| fancy_regex::Regex::new(r"(?:^|\s)\(?([A-E])[).]\s+").expect("options"));
    let ms: Vec<_> = re.captures_iter(q).flatten().collect();
    if ms.len() < 3 || ms.iter().enumerate().any(|(i, m)| m[1].chars().next() != Some((b'A' + i as u8) as char)) {
        return None;
    }
    let starts: Vec<(usize, usize)> = ms.iter().map(|m| { let g = m.get(0).expect("match"); (g.start(), g.end()) }).collect();
    let opts = ms.iter().enumerate().map(|(i, m)| {
        let end = starts.get(i + 1).map(|s| s.0).unwrap_or(q.len());
        (m[1].chars().next().unwrap_or('A'), q[starts[i].1..end].trim().trim_end_matches('.').to_string())
    }).collect();
    Some((q[..starts[0].0].trim().to_string(), opts))
}

impl V2 {
    /// Topics a text names by a strong phrase (name, aliases, everyday words).
    fn topics_named(&self, text: &str) -> Vec<String> {
        let Some(br) = &self.book_rule else { return vec![] };
        let ws = BookRule::words(text);
        br.topics.iter().filter(|t| t.1.iter().any(|p| !p.is_empty() && p.iter().all(|w| ws.contains(w))))
            .map(|t| t.0.clone()).collect()
    }

    /// A multiple-choice question answered from the facts: Some(Ok(reply)) a letter with its reason, Some(Err(why))
    /// a multiple-choice question no fact decides, None not a multiple-choice question. The twin of v2_facts.mcq.
    pub fn facts_choice(&self, q: &str) -> Option<Result<Value, String>> {
        let (stem, opts) = parse_choices(q)?;
        let Some(facts) = &self.facts else { return Some(Err("no reviewed facts in this package".into())) };
        // "which is NOT ...", "all EXCEPT": the facts say what is, not what is not
        if negated(&stem) {
            return Some(Err("a NOT / EXCEPT question: the facts say what is, not what is not".into()));
        }
        let sw = fact_words(&stem);
        // every topic: the options decide, and each cue carries its topic's key noun (the twin of v2_facts.mcq; short
        // answers stay topic-gated, they have no options)
        let mut topics: Vec<String> = facts.by_topic.keys().cloned().collect();
        topics.sort();
        let app: Vec<(&String, &Fact)> = topics.iter()
            .flat_map(|cid| facts.by_topic.get(cid).into_iter().flatten().map(move |f| (cid, f)))
            .filter(|(_, f)| f.applies(&sw))
            .collect();
        if app.is_empty() {
            return Some(Err("no reviewed fact matches the question".into()));
        }
        let ows: Vec<(char, HashSet<String>)> = opts.iter().map(|(l, t)| (*l, fact_words(t))).collect();
        // options that are only directions ("A) increase B) decrease C) stay the same"): which quantity is the cause and
        // which the effect is not in a bag of words (the twin of v2_facts DIRECTION_TEXT; development set m144)
        let dir = fact_words("increase increases increased decrease decreases decreased rise rises fall falls drop drops \
            higher lower more less greater smaller larger faster slower longer shorter stay stays remain remains same \
            unchanged constant double doubles halve halves half twice up down change changes unaffected fluctuate \
            fluctuates randomly below above one zero it will");
        if ows.iter().filter(|(_, ow)| !ow.is_empty() && ow.is_subset(&dir)).count() >= 2 {
            return Some(Err("the options are only directions".into()));
        }
        let mut picks: HashSet<char> = HashSet::new();
        for (cid, f) in &app {
            let hit: Vec<char> = ows.iter().filter(|(_, ow)| f.keys.iter().any(|k| k.is_subset(ow))).map(|x| x.0).collect();
            if hit.len() != 1 {
                return Some(Err(format!("a fact of {cid} holds for {} options", hit.len())));
            }
            let ow = &ows.iter().find(|x| x.0 == hit[0]).expect("option").1;
            if f.not.iter().any(|n| n.is_subset(ow)) {
                return Some(Err(format!("the option also holds a wrong answer of {cid}")));
            }
            picks.insert(hit[0]);
        }
        if picks.len() != 1 {
            return Some(Err("the facts point at different options".into()));
        }
        let l = *picks.iter().next().expect("one pick");
        let (cid, f) = app[0];
        let text = &opts.iter().find(|o| o.0 == l).expect("option").1;
        Some(Ok(json!({"outcome": "EXPLAINED", "level": "answer", "choice": l.to_string(),
                       "message": format!("{l}) {text}. {}", f.statement), "concept": cid, "answer": f.answer,
                       "reader": "reviewed facts", "note": "reviewed fact, not computed"})))
    }

    /// A short "which / what is the term" question answered by the one fact of a named topic whose cue it matches.
    /// The twin of v2_facts.open_q.
    pub fn facts_open(&self, q: &str) -> Option<Value> {
        const OPEN: &[&str] = &["which", "what is the term", "what is the name", "what term", "what is this",
            "what is called", "what are the", "what type", "what kind", "name the", "is called", "called?", "known as",
            "what do we call", "what do you call", "what process", "what is the process", "what is the biological process"];
        let facts = self.facts.as_ref()?;
        let low = format!(" {} ", q.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" "));
        // a negated question, or a choice between two named things ("short circuiting OR baffle factor, which is the
        // problem": a comparison, not a term to name; the twin of v2_facts.open_q)
        if !OPEN.iter().any(|c| low.contains(c)) || negated(q) || low.split(|c: char| !c.is_alphanumeric()).any(|w| w == "or") {
            return None;
        }
        let sw = fact_words(q);
        // (named topics only, the twin of v2_facts.open_q: every topic's facts gave 5 wrong of 28 on the development set)
        // (the networks' topics were tried too, 2026-11-12, and rejected: 2 more right on the development set, but "what
        // are the Pasquill stability categories, the A to F thing" got "Stable" from the class-F fact)
        let mut topics = self.topics_named(q);
        topics.sort();
        let app: Vec<(&String, &Fact)> = topics.iter()
            .flat_map(|cid| facts.by_topic.get(cid).into_iter().flatten().map(move |f| (cid, f)))
            .filter(|(_, f)| f.applies(&sw))
            .collect();
        let answers: HashSet<String> = app.iter().map(|(_, f)| f.answer.to_lowercase()).collect();
        if answers.len() != 1 {
            return None;
        }
        let (cid, f) = app[0];
        // the thing asked for must be what the fact is about: "which MEDICINE should I take" is not answered by "avoid
        // swimming after heavy rain" (held-out 2026-11-12c x14; the twin of v2_facts.asked_noun)
        if let Some(noun) = asked_noun(q) {
            let mut about = fact_words(&format!("{} {}", f.answer, f.statement));
            about.extend(f.cues.iter().flatten().cloned());
            if !about.contains(&noun) {
                return None;
            }
        }
        Some(json!({"outcome": "EXPLAINED", "level": "answer", "message": format!("{}. {}", capitalise(&f.answer), f.statement),
                    "concept": cid, "answer": f.answer, "reader": "reviewed facts", "note": "reviewed fact, not computed"}))
    }
}

/// Words that turn a question around ("which is NOT", "which CANNOT", "requires NO"): the facts say what is, so such a
/// question is refused (the twin of v2_facts.NEGATED; "can't" splits into "can" and "t", so "cannot" covers "can not")
const NEGATION_WORDS: &[&str] = &["not", "except", "false", "incorrect", "cannot", "never", "neither", "none",
                                  "without", "no", "t"];

fn negated(q: &str) -> bool {
    let ws: Vec<String> = q.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect();
    // "least" turns the question around ("which is LEAST effective"), but "AT LEAST once" does not
    ws.iter().enumerate().any(|(i, w)| NEGATION_WORDS.contains(&w.as_str())
        || (w == "least" && !(i > 0 && ws[i - 1] == "at")))
}

/// The stemmed noun a "which X / what X" question asks for; None when it asks generically ("what is", "which term",
/// "what process"). The twin of v2_facts.asked_noun.
fn asked_noun(q: &str) -> Option<String> {
    use std::sync::OnceLock;
    const GENERIC: &[&str] = &["is", "are", "was", "were", "does", "do", "did", "can", "could", "should", "would",
        "will", "may", "might", "has", "have", "the", "a", "an", "this", "that", "these", "those", "term", "terms", "name",
        "names", "called", "process", "processes", "type", "types", "kind", "kinds", "sort", "one", "ones", "word",
        "words", "thing", "things", "happens", "happen", "causes", "cause", "makes", "make", "method", "methods", "device",
        "devices", "unit", "units", "step", "steps", "stage", "stages", "value", "values", "quantity", "quantities",
        "parameter", "parameters", "factor", "factors", "number", "numbers", "main", "primary", "principal", "common",
        "usual", "typical", "most", "best", "first", "last", "two", "three", "four"];
    static RE: OnceLock<fancy_regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| fancy_regex::Regex::new(r"\b(?:which|what)\s+(?:kind of\s+|type of\s+|sort of\s+)?([a-z]+)")
        .expect("asked noun"));
    let low = q.to_lowercase();
    let w = re.captures(&low).ok().flatten()?.get(1)?.as_str().to_string();
    if GENERIC.contains(&w.as_str()) || fact_words(&w).is_empty() {
        return None;
    }
    Some(fstem(&w))
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

pub struct V2 {
    /// book questions by rule (`v2/book_rule.json`), when the package has it
    book_rule: Option<BookRule>,
    /// reviewed facts (`v2/facts.json`), when the package has them (recipe A31)
    facts: Option<Facts>,
    members: Vec<Member>,
    tok: tokenizers::Tokenizer,
    labels: Value,
    book: Value,
    twins: HashMap<String, HashSet<String>>,
    vocab: HashMap<String, HashSet<String>>,
    /// words that do not ground a question in a topic on their own (labels.json `general_words`)
    general: HashSet<String>,
    up: Vec<String>,
    down: Vec<String>,
    dev: Device,
}

/// What the networks read: the decision, its confidence (the least sure head it needed; 0 when the members disagree),
/// and for a calculation the version 1 shaped reading the engine path takes.
pub struct V2Reading {
    pub intent: String,
    pub p_intent: f32,
    pub concept: String,
    pub typ: String,
    pub conf: f32,
    pub agree: bool,
    pub reading: Option<Reading>,
    /// the most probable concept that is neither the chosen one nor its twin, and its probability
    pub runner_up: Option<(String, f32)>,
}

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn argmax(v: &[f32]) -> (usize, f32) {
    v.iter().cloned().enumerate().fold((0, f32::MIN), |a, (i, x)| if x > a.1 { (i, x) } else { a })
}

const STOP: &[&str] = &["the", "a", "an", "of", "in", "on", "to", "for", "and", "or", "is", "are", "was", "be", "it",
    "its", "what", "why", "how", "does", "do", "did", "can", "could", "should", "would", "i", "me", "my", "we", "you",
    "your", "this", "that", "these", "those", "with", "from", "by", "as", "at", "about", "if", "when", "which", "there",
    "they", "them", "than", "then", "also", "very", "more", "less", "most", "some", "any", "all", "each", "other", "same",
    "much", "many", "get", "make", "need", "not", "no", "yes", "who", "where", "will", "just", "like", "really", "still",
    "into", "out", "up", "down", "off", "over", "under"];

/// Content words of a text, lightly stemmed: the twin of tools/v2_package.py `words` (the vocabulary is built there).
pub fn content_words(t: &str) -> HashSet<String> {
    let low = t.to_lowercase();
    let mut out = HashSet::new();
    let mut cur = String::new();
    let flush = |w: &mut String, out: &mut HashSet<String>| {
        let x = w.trim_matches(|c| c == '\'' || c == '-').to_string();
        w.clear();
        if x.len() >= 3 && x.chars().next().is_some_and(|c| c.is_ascii_lowercase()) && !STOP.contains(&x.as_str()) {
            out.insert(stem(&x));
        }
    };
    for ch in low.chars() {
        if ch.is_ascii_lowercase() || ((ch == '\'' || ch == '-') && !cur.is_empty()) {
            cur.push(ch);
        } else {
            flush(&mut cur, &mut out);
        }
    }
    flush(&mut cur, &mut out);
    out
}

fn stem(w: &str) -> String {
    for suf in ["ies", "es", "s", "ing", "ed", "ly"] {
        if let Some(s) = w.strip_suffix(suf) {
            return s.to_string();
        }
    }
    w.to_string()
}

fn has_phrase(q: &str, list: &[String]) -> bool {
    let ql = format!(" {} ", q.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" "));
    list.iter().any(|w| ql.contains(&format!(" {} ", w.to_lowercase())))
}

impl Member {
    fn load(dir: &Path, labels: &Value, dev: &Device) -> Result<Member, String> {
        let cfg: Config = serde_json::from_str(&std::fs::read_to_string(dir.join("config.json")).map_err(err)?).map_err(err)?;
        let raw = candle_core::safetensors::load(dir.join("model.safetensors"), dev).map_err(err)?;
        // int8 storage (tools/v2_package.py --int8): a weight matrix W is stored as "<W>.__q8" (u8, value + 128) and
        // "<W>.__scale" (f32, one per row): W = (q8 - 128) * scale. Everything else is fp16 or f32
        let mut ts: HashMap<String, Tensor> = HashMap::new();
        for (k, t) in &raw {
            if let Some(name) = k.strip_suffix(".__q8") {
                let s = raw.get(&format!("{name}.__scale")).ok_or_else(|| format!("{name}: int8 weight without its scale"))?;
                let w = (t.to_dtype(DType::F32).map_err(err)? - 128.0).map_err(err)?
                    .broadcast_mul(&s.to_dtype(DType::F32).map_err(err)?.unsqueeze(1).map_err(err)?).map_err(err)?;
                ts.insert(name.to_string(), w);
            } else if !k.ends_with(".__scale") {
                ts.insert(k.clone(), t.clone());
            }
        }
        let vb = VarBuilder::from_tensors(ts, DType::F32, dev);
        let n = |k: &str| labels[k].as_array().map(|a| a.len()).unwrap_or(0);
        let h = cfg.hidden_size;
        Ok(Member {
            bert: BertModel::load(vb.pp("bert"), &cfg).map_err(err)?,
            intent: linear(h, n("intents"), vb.pp("intent")).map_err(err)?,
            concept: linear(h, n("concepts"), vb.pp("concept")).map_err(err)?,
            typ: linear(h, n("types"), vb.pp("qtype")).map_err(err)?,
            capability: linear(h, n("capabilities"), vb.pp("capability")).map_err(err)?,
            asked: linear(h, n("asked"), vb.pp("asked")).map_err(err)?,
            tags: linear(h, n("tags"), vb.pp("tags")).map_err(err)?,
        })
    }
}

impl V2 {
    pub fn load(dir: &Path) -> Result<V2, String> {
        let dev = Device::Cpu;
        let read = |n: &str| std::fs::read_to_string(dir.join(n)).map_err(|e| format!("v2/{n}: {e}"));
        let labels: Value = serde_json::from_str(&read("labels.json")?).map_err(err)?;
        let book: Value = serde_json::from_str(&read("book.json")?).map_err(err)?;
        let mut members = vec![];
        for i in 0.. {
            let m = dir.join(format!("m{i}"));
            if !m.is_dir() {
                break;
            }
            members.push(Member::load(&m, &labels, &dev)?);
        }
        if members.is_empty() {
            members.push(Member::load(dir, &labels, &dev)?);
        }
        let mut twins: HashMap<String, HashSet<String>> = HashMap::new();
        for p in labels["twins"].as_array().into_iter().flatten() {
            let (a, b) = (p[0].as_str().unwrap_or("").to_string(), p[1].as_str().unwrap_or("").to_string());
            twins.entry(a.clone()).or_default().insert(b.clone());
            twins.entry(b).or_default().insert(a);
        }
        let vocab = book
            .as_object()
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v["vocab"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect())).collect())
            .unwrap_or_default();
        let list = |k: &str| -> Vec<String> { labels["direction"][k].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect() };
        let (up, down) = (list("up"), list("down"));
        let general: HashSet<String> = labels["general_words"].as_array().into_iter().flatten()
            .filter_map(|x| x.as_str().map(String::from)).collect();
        let book_rule = std::fs::read_to_string(dir.join("book_rule.json")).ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|v| BookRule::load(&v));
        let facts = std::fs::read_to_string(dir.join("facts.json")).ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|v| Facts::load(&v));
        Ok(V2 { book_rule, facts, members, tok: tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).map_err(err)?, labels,
                book, twins, vocab, general, up, down, dev })
    }

    pub fn th(&self, k: &str) -> f32 {
        self.labels["thresholds"][k].as_f64().unwrap_or(0.0) as f32
    }

    fn name(&self, k: &str, i: usize) -> String {
        self.labels[k][i].as_str().unwrap_or("").to_string()
    }

    fn same_concept(&self, a: &str, b: &str) -> bool {
        a == b || self.twins.get(a).is_some_and(|s| s.contains(b))
    }

    /// Why the cell may not be shown although the networks chose it, or None. Grounding: the question shares a content
    /// word with the topic's vocabulary. Runner-up: the question's words do not support the next most probable topic
    /// better than the chosen one (Env Eng r2: blocked the most confident wrong text, "weir" read as the clarifier
    /// weir instead of the sewer overflow), and the question has a word of the chosen topic the runner-up lacks.
    /// Direction: a "less"-only question never gets the "more" cell, and back.
    pub fn rule_block(&self, q: &str, concept: &str, typ: &str, runner_up: Option<&str>) -> Option<&'static str> {
        let qw = content_words(q);
        if let Some(v) = self.vocab.get(concept) {
            if !v.is_empty() && qw.is_disjoint(v) {
                return Some("the question shares no word with the topic it was read as");
            }
            // ... and a word of the topic that is not GENERAL (labels.json `general_words`, tools/v2_package.py
            // --general: as frequent in the off-topic training questions as in the book's). "what happens if bee
            // populations keep falling" shares only "population" with Growth Rate Projection (regression g004)
            if !self.general.is_empty() && !v.is_empty() && qw.iter().all(|w| !v.contains(w) || self.general.contains(w)) {
                return Some("the question shares only general words with the topic it was read as");
            }
            if let Some(v2) = runner_up.and_then(|c| self.vocab.get(c)) {
                if qw.intersection(v2).count() > qw.intersection(v).count() {
                    return Some("the question's words fit another topic better");
                }
                // the question must name something of the chosen topic that the runner-up does not have (Env Eng r2ab
                // on validation: 2,260 answers kept at 0 wrong with this rule, 1,992 without)
                if !qw.iter().any(|w| v.contains(w) && !v2.contains(w)) {
                    return Some("nothing in the question sets this topic apart from a neighbouring one");
                }
            }
        }
        if typ == "more" || typ == "less" {
            let (u, d) = (has_phrase(q, &self.up), has_phrase(q, &self.down));
            if (typ == "more" && d && !u) || (typ == "less" && u && !d) {
                return Some("the question asks the other direction");
            }        }
        None
    }

    /// What the networks' value tagger says about each number of the question (recipe A26): for every value span
    /// (`candidates`, character offsets), the members' averaged probability that it is each input symbol (the B- and I-
    /// tags of the symbol over the number's tokens). The rule-based calculations use it only to break a tie between
    /// placements their words leave open, and to name a value no word names.
    pub fn tag_hints(&self, q: &str) -> Result<Vec<(usize, usize, HashMap<String, f32>)>, String> {
        let max_len = self.labels["max_len"].as_u64().unwrap_or(64) as usize;
        let e = self.tok.encode_char_offsets(q, true).map_err(err)?;
        let mut ids: Vec<u32> = e.get_ids().to_vec();
        let mut offs = e.get_offsets().to_vec();
        if ids.len() > max_len {
            let sep = *ids.last().unwrap();
            ids.truncate(max_len - 1);
            ids.push(sep);
            offs.truncate(max_len - 1);
            offs.push((0, 0));
        }
        let n = ids.len();
        let x = Tensor::from_vec(ids, (1, n), &self.dev).map_err(err)?;
        let tt = Tensor::zeros((1, n), DType::U32, &self.dev).map_err(err)?;
        let m = Tensor::ones((1, n), DType::U32, &self.dev).map_err(err)?;
        let k = self.members.len() as f32;
        let mut pg: Vec<Vec<f32>> = vec![];
        for mb in &self.members {
            let hs = mb.bert.forward(&x, &tt, Some(&m)).map_err(err)?;
            let g: Vec<Vec<f32>> = candle_nn::ops::softmax(&mb.tags.forward(&hs).map_err(err)?, D::Minus1)
                .and_then(|t| t.squeeze(0)).and_then(|t| t.to_vec2()).map_err(err)?;
            if pg.is_empty() {
                pg = g.iter().map(|r| r.iter().map(|v| v / k).collect()).collect();
            } else {
                pg.iter_mut().zip(g).for_each(|(a, b)| a.iter_mut().zip(b).for_each(|(x, y)| *x += y / k));
            }
        }
        let tag_names: Vec<String> = self.labels["tags"].as_array()
            .map(|a| a.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default();
        let nums = numbers(q);
        let mut out = vec![];
        for (cs, ce) in candidates(q) {
            let ne = nums.iter().find(|(s, e)| *s >= cs && *e <= ce).map(|x| x.1).unwrap_or(ce);
            let toks: Vec<usize> = offs.iter().enumerate().filter(|(_, (a, b))| b > a && *a < ne && cs < *b).map(|(j, _)| j).collect();
            if toks.is_empty() {
                continue;
            }
            let mut by: HashMap<String, f32> = HashMap::new();
            for &tk in &toks {
                for (j, p) in pg[tk].iter().enumerate() {
                    if let Some((_, sym)) = tag_names.get(j).and_then(|t| t.split_once('-')) {
                        *by.entry(sym.to_string()).or_default() += p / toks.len() as f32;
                    }
                }
            }
            out.push((cs, ce, by));
        }
        Ok(out)
    }

    pub fn read(&self, q: &str) -> Result<V2Reading, String> {
        let max_len = self.labels["max_len"].as_u64().unwrap_or(64) as usize;
        let e = self.tok.encode_char_offsets(q, true).map_err(err)?;
        let mut ids: Vec<u32> = e.get_ids().to_vec();
        let mut offs = e.get_offsets().to_vec();
        if ids.len() > max_len {
            let sep = *ids.last().unwrap();
            ids.truncate(max_len - 1);
            ids.push(sep);
            offs.truncate(max_len - 1);
            offs.push((0, 0));
        }
        let n = ids.len();
        let x = Tensor::from_vec(ids, (1, n), &self.dev).map_err(err)?;
        let tt = Tensor::zeros((1, n), DType::U32, &self.dev).map_err(err)?;
        let m = Tensor::ones((1, n), DType::U32, &self.dev).map_err(err)?;
        let sm = |t: Tensor| -> Result<Vec<f32>, String> {
            candle_nn::ops::softmax(&t, D::Minus1).and_then(|t| t.squeeze(0)).and_then(|t| t.to_vec1()).map_err(err)
        };
        // every member reads the question; probabilities are averaged, each member's own choice kept for agreement
        let k = self.members.len() as f32;
        let (mut pi, mut pc, mut pt, mut pk, mut pa) = (vec![], vec![], vec![], vec![], vec![]);
        let mut pg: Vec<Vec<f32>> = vec![];
        let mut choices: Vec<(usize, usize, usize)> = vec![];
        let add = |acc: &mut Vec<f32>, v: Vec<f32>| {
            if acc.is_empty() {
                *acc = v.iter().map(|x| x / k).collect();
            } else {
                acc.iter_mut().zip(v).for_each(|(a, b)| *a += b / k);
            }
        };
        for mb in &self.members {
            let hs = mb.bert.forward(&x, &tt, Some(&m)).map_err(err)?;
            let pooled = hs.mean(1).map_err(err)?;
            let (vi, vc, vt) = (sm(mb.intent.forward(&pooled).map_err(err)?)?, sm(mb.concept.forward(&pooled).map_err(err)?)?,
                                sm(mb.typ.forward(&pooled).map_err(err)?)?);
            choices.push((argmax(&vi).0, argmax(&vc).0, argmax(&vt).0));
            add(&mut pi, vi);
            add(&mut pc, vc);
            add(&mut pt, vt);
            add(&mut pk, sm(mb.capability.forward(&pooled).map_err(err)?)?);
            add(&mut pa, sm(mb.asked.forward(&pooled).map_err(err)?)?);
            let g: Vec<Vec<f32>> = candle_nn::ops::softmax(&mb.tags.forward(&hs).map_err(err)?, D::Minus1)
                .and_then(|t| t.squeeze(0))
                .and_then(|t| t.to_vec2())
                .map_err(err)?;
            if pg.is_empty() {
                pg = g.iter().map(|r| r.iter().map(|x| x / k).collect()).collect();
            } else {
                pg.iter_mut().zip(g).for_each(|(a, b)| a.iter_mut().zip(b).for_each(|(x, y)| *x += y / k));
            }
        }
        let (i, p_intent) = argmax(&pi);
        let intent = self.name("intents", i);
        if intent == "answer" {
            let (c, p_c) = argmax(&pc);
            let (t, p_t) = argmax(&pt);
            let concept = self.name("concepts", c);
            // labels.json "conf_mode": "topic" judges the TOPIC only (the members agree on intent and concept; the
            // confidence leaves out the type head): a neighbouring kind on the right topic is shown under the
            // topic's heading, and the threshold is calibrated on judged picks like any other (recipe A10f).
            // Default "cell": the type head counts too and every member must pick the same type
            // "topic_agree": the confidence of "topic", but the members must still pick the same type (a wrong kind can
            // mislead on the right topic: "am i responsible if my waste is dumped" -> the littering MYTH text)
            let mode = self.labels["conf_mode"].as_str().unwrap_or("cell");
            let topic = mode == "topic" || mode == "topic_agree";
            let type_free = mode == "topic";
            let agree = choices.iter().all(|(ci, cc, ct)| {
                *ci == i && (type_free || *ct == t) && self.same_concept(&concept, &self.name("concepts", *cc))
            });
            // a topic's probability includes its twins' (the network splits "ozone water treatment" and its twin "ozone
            // inactivation": 0.63 for a plain training question although every member agreed on the pair)
            let p_topic: f32 = pc.iter().enumerate()
                .filter(|(j, _)| self.same_concept(&concept, &self.name("concepts", *j)))
                .map(|(_, p)| *p)
                .sum();
            let conf = if !agree { 0.0 } else if topic { p_intent.min(p_topic) } else { p_intent.min(p_c).min(p_t) };
            let runner_up = pc.iter().enumerate()
                .filter(|(j, _)| !self.same_concept(&concept, &self.name("concepts", *j)))
                .max_by(|a, b| a.1.total_cmp(b.1))
                .map(|(j, p)| (self.name("concepts", j), *p));
            return Ok(V2Reading { intent, p_intent, concept, typ: self.name("types", t), conf, agree, reading: None, runner_up });
        }
        if intent != "compute" {
            return Ok(V2Reading { intent, p_intent, concept: String::new(), typ: String::new(), conf: p_intent, agree: true,
                                  reading: None, runner_up: None });
        }
        let (kc, p_k) = argmax(&pk);
        let cap = self.name("capabilities", kc);
        let agree = choices.iter().all(|(ci, _, _)| *ci == i);
        let lab = &self.labels;
        // the asked output among this capability's outputs only
        let outs: Vec<usize> = lab["cap_outputs"][&cap]
            .as_array()
            .map(|a| a.iter().filter_map(|s| lab["asked"].as_array()?.iter().position(|x| x == s)).collect())
            .unwrap_or_default();
        let tot: f32 = outs.iter().map(|&j| pa[j]).sum();
        let (asked, p_asked) = match outs.iter().max_by(|a, b| pa[**a].total_cmp(&pa[**b])) {
            Some(&j) => (Some(self.name("asked", j)), if tot > 0.0 { pa[j] / tot } else { 0.0 }),
            None => (None, 1.0),
        };
        // values: spans from the rule (candidates); the networks only say which input each one is (mean over its
        // tokens of each input's B- and I- probability, among this capability's inputs and "O")
        let tag_names: Vec<String> = lab["tags"].as_array().map(|a| a.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default();
        let ins: Vec<String> = lab["cap_inputs"][&cap].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
        let ok: Vec<usize> = tag_names
            .iter()
            .enumerate()
            .filter(|(_, t)| *t == "O" || t.split_once('-').map(|(_, s)| ins.iter().any(|x| x == s)).unwrap_or(false))
            .map(|(j, _)| j)
            .collect();
        let (mut spans, mut unread, mut p_tags) = (vec![], vec![], 1.0f32);
        let nums = numbers(q);
        for (cs, ce) in candidates(q) {
            // the network tags the NUMBER (training spans: "2.55" of "2.55 ppm" when the unit is the input's name), so a
            // candidate is scored on its number's tokens; its unit still goes to the engine with the value. Scoring the
            // unit's tokens too averaged in their "O" and put readings at exactly 0.5 (Env Eng r2: 18 of 226).
            let ne = nums.iter().find(|(s, e)| *s >= cs && *e <= ce).map(|x| x.1).unwrap_or(ce);
            let toks: Vec<usize> = offs.iter().enumerate().filter(|(_, (a, b))| b > a && *a < ne && cs < *b).map(|(j, _)| j).collect();
            if toks.is_empty() {
                unread.push((cs, ce));
                continue;
            }
            let mut score: Vec<(Option<String>, f32)> = vec![];
            for &tk in &toks {
                let row = &pg[tk];
                let tot: f32 = ok.iter().map(|&j| row[j]).sum();
                for &j in &ok {
                    let key = if tag_names[j] == "O" { None } else { tag_names[j].split_once('-').map(|x| x.1.to_string()) };
                    let add = if tot > 0.0 { row[j] / tot } else { 0.0 };
                    match score.iter_mut().find(|(y, _)| *y == key) {
                        Some(s) => s.1 += add,
                        None => score.push((key, add)),
                    }
                }
            }
            let (best, sc) = score.into_iter().max_by(|a, b| a.1.total_cmp(&b.1)).unwrap_or((None, 0.0));
            p_tags = p_tags.min(sc / toks.len() as f32);
            match best {
                Some(y) => spans.push((cs, ce, y)),
                None => unread.push((cs, ce)),
            }
        }
        spans.sort();
        let p_target = if agree { p_k } else { 0.0 };
        let r = Reading { intent: intent.clone(), p_intent, target: format!("cap:{cap}"), p_target, asked, p_asked,
                          spans, unread, target_probs: vec![], p_tags };
        Ok(V2Reading { intent, p_intent, concept: String::new(), typ: String::new(),
                       conf: p_intent.min(p_target).min(p_asked).min(p_tags), agree, reading: Some(r), runner_up: None })
    }
}

/// The reply to one question with the version 2 layer: the network's reply when it answers or computes; otherwise,
/// for a question with numbers, the calculation worked out by rule (`derive`, recipe A21) when one reading fits.
/// AX_DERIVE=off leaves the rule-based calculations out, =only uses them alone for questions with numbers
/// (measurement only). A computed network reply carries the rule-based reading's result in `derive` for comparison.
pub fn answer_v2(p: &Pack, v2: &V2, q: &str) -> Value {
    let q = &crate::exam_notation(q);
    if std::env::var("AX_DERIVE_DEBUG").is_ok() {
        eprintln!("[notation] {q}");
    }
    // multiple choice: a letter only when the reviewed facts decide (recipe A31); otherwise "I don't know", never the
    // stem read as an ordinary question (a book cell next to options would read as a choice)
    if std::env::var("AX_FACTS").is_ok_and(|v| v == "off") {
    } else if let Some(r) = v2.facts_choice(q) {
        return r.unwrap_or_else(|why| crate::dont_know(None, &why));
    } else if !BookRule::free_number(q) {
        if let Some(v) = v2.facts_open(q) {
            return v;
        }
    }
    let mode = std::env::var("AX_DERIVE").unwrap_or_default();
    if mode == "off" || crate::numbers(q).is_empty() {
        return answer_v2_net(p, v2, q);
    }
    // the networks' value tags help the rule-based placement (recipe A26); AX_DERIVE_TAGS=off leaves them out
    let hints = if std::env::var("AX_DERIVE_TAGS").is_ok_and(|v| v == "off") { vec![] } else { v2.tag_hints(q).unwrap_or_default() };
    let d = crate::derive::derive_with(p, q, hints);
    if mode == "only" {
        return d.unwrap_or_else(|why| crate::dont_know(None, &why));
    }
    let mut net = answer_v2_net(p, v2, q);
    let net_done = net["outcome"] == "SOLVED" || net["level"] == "answer";
    match d {
        Ok(dv) if !net_done => dv,
        Ok(dv) => {
            // two independent readings of one calculation must agree (they did on all 345 development questions both
            // solved, 2026-11-10): the network's calculation and the rule-based one giving different results is refused
            let differ = net["outcome"] == "SOLVED" && dv["outcome"] == "SOLVED"
                && (net["request"]["capability"] != dv["request"]["capability"]
                    || match (net["value"].as_f64(), dv["value"].as_f64()) {
                        (Some(a), Some(b)) => (a - b).abs() > 1e-9 * a.abs().max(b.abs()),
                        _ => net["value"] != dv["value"],
                    });
            if differ {
                let mut v = crate::dont_know(None, "the two readings of the calculation disagree");
                v["reading"] = net["reading"].clone();
                return v;
            }
            net["derive"] = json!({"outcome": dv["outcome"], "request": dv["request"], "value": dv["value"]});
            net
        }
        Err(why) => {
            net["derive_why"] = json!(why);
            net
        }
    }
}

/// The networks' reply, then the topics they were not trained on (recipe A23, policy C): a question that names such a
/// topic by its reviewed name, aliases or everyday words, every word explained, gets that topic's cell; when the
/// networks answered too, the topic the question names more specifically (its longer phrase) wins ("what if too much
/// chlorine is added" is the new Chlorination topic, not the networks' Chlorine Dose).
fn answer_v2_net(p: &Pack, v2: &V2, q: &str) -> Value {
    let v = answer_v2_net_inner(p, v2, q);
    if BookRule::free_number(q) || v["outcome"] == "SOLVED" {
        return v;
    }
    let Some(br) = &v2.book_rule else { return v };
    let Ok((t, k)) = br.read(v2, q, None) else { return v };
    let known = v2.labels["concepts"].as_array().is_some_and(|a| a.iter().any(|x| x.as_str() == Some(t.as_str())));
    if known {
        return v;
    }
    if v["level"] == "answer" && br.phrase_len(q, &t) <= br.phrase_len(q, v["concept"].as_str().unwrap_or("")) {
        return v;
    }
    let b = &v2.book[&t];
    let Some(text) = b["cells"][&k].as_str().filter(|x| !x.is_empty()) else { return v };
    let heading = match v2.labels["type_headings"][&k].as_str() {
        Some(h) => format!("{} — {h}", b["name"].as_str().unwrap_or("")),
        None => b["name"].as_str().unwrap_or("").to_string(),
    };
    json!({"outcome": "EXPLAINED", "message": text, "level": "answer", "name": b["name"], "concept": t, "type": k,
           "heading": heading, "shown_as": "the closest topic in the book", "related": b["capabilities"].clone(),
           "reader": "book by rule (a topic the networks were not trained on)", "reading": v["reading"].clone(),
           "networks_reply": v["heading"].clone(), "note": "reviewed text, not computed"})
}

fn answer_v2_net_inner(p: &Pack, v2: &V2, q: &str) -> Value {
    let rule = ax_engine::ask::ask_value(p, q);
    if rule["outcome"] == "SOLVED" {
        return rule;
    }
    let r = match v2.read(q) {
        Ok(r) => r,
        Err(e) => return dont_know(None, &format!("reader error: {e}")),
    };
    let reading = json!({"intent": r.intent, "p_intent": r.p_intent, "concept": r.concept, "type": r.typ,
                         "confidence": r.conf, "agree": r.agree,
                         "runner_up": r.runner_up.as_ref().map(|(c, p)| json!({"concept": c, "p": p}))});
    let unknown = |why: &str| {
        let mut v = dont_know(None, why);
        v["reading"] = reading.clone();
        v
    };
    // the reply names the topic and kind it shows: the book's closest cell, not an answer written for the question,
    // so a neighbouring cell never poses as an exact answer
    let cell = |concept: &str, typ: &str, reader: &str| -> Option<Value> {
        let b = &v2.book[concept];
        let text = b["cells"][typ].as_str().filter(|t| !t.is_empty())?;
        let heading = match v2.labels["type_headings"][typ].as_str() {
            Some(h) => format!("{} — {h}", b["name"].as_str().unwrap_or("")),
            None => b["name"].as_str().unwrap_or("").to_string(),
        };
        Some(json!({"outcome": "EXPLAINED", "message": text, "level": "answer", "name": b["name"], "concept": concept,
                    "type": typ, "heading": heading, "shown_as": "the closest topic in the book",
                    "related": b["capabilities"].clone(), "score": r.conf, "reader": reader,
                    "reading": reading, "note": "reviewed text, not computed"}))
    };
    // a book question the networks did not answer: answered by rule when the question names the networks' own top
    // topic and asks in a recognisable way (recipe A23: "How water treatment works?" was refused because one member
    // read another kind of question)
    let by_rule = |why: &str| -> Value {
        if !BookRule::free_number(q) && !r.concept.is_empty() {
            if let Some(br) = &v2.book_rule {
                // (a cloud term counts for the networks' topic only with a word of the topic's own name in the
                // question: Env Eng development questions +260 answers, no new off-topic answer)
                match br.read(v2, q, Some(&r.concept)) {
                    Ok((c, k)) if v2.same_concept(&r.concept, &c) => {
                        if let Some(mut v) = cell(&c, &k, "book by rule (the networks' topic)") {
                            v["network_why"] = json!(why);
                            return v;
                        }
                    }
                    Ok((c, _)) => {
                        let mut v = unknown(why);
                        v["rule_why"] = json!(format!("the rule names {c}, the networks {}", r.concept));
                        return v;
                    }
                    Err(e) => {
                        let mut v = unknown(why);
                        v["rule_why"] = json!(e);
                        return v;
                    }
                }
            }
        }
        unknown(why)
    };
    match r.intent.as_str() {
        "answer" => {
            // a question with numbers is a calculation, never a stored answer
            if BookRule::free_number(q) {
                return unknown("numbers in a question read as a stored answer");
            }
            // labels thresholds.answer_by_type (recipe A10i, tools/v2_calibrate.py --by-type): one threshold per shown
            // type, each above that type's most confident wrong validation reply and never below the calibration floor
            let th = v2.labels["thresholds"]["answer_by_type"][&r.typ].as_f64().map(|x| x as f32).unwrap_or(v2.th("answer"));
            if r.conf < th {
                return by_rule(if r.agree { "not sure which answer is meant" } else { "the readers disagree" });
            }
            if let Some(why) = v2.rule_block(q, &r.concept, &r.typ, r.runner_up.as_ref().map(|x| x.0.as_str())) {
                return by_rule(why);
            }
            cell(&r.concept, &r.typ, "language model v2").unwrap_or_else(|| by_rule("no answer of that kind for this topic"))
        }
        "compute" => {
            let Some(rd) = r.reading else { return unknown("compute without a reading") };
            let mut v = compute_reply(p, v2.th("compute"), rd, q);
            v["reader"] = json!("language model v2");
            v
        }
        _ => unknown(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_words_match_the_packager() {
        let w = content_words("Is it safe to swim in rivers after storms?");
        assert!(w.contains("safe") && w.contains("swim") && w.contains("river") && w.contains("storm"));
        assert!(!w.contains("is") && !w.contains("it"));
    }
}
