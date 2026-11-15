//! Calculations worked out by rule, with no network (recipe A21, 2026-11-10): the question's values are read with
//! their units, each value is placed in the input of a calculation that takes that kind of quantity and whose meaning
//! the words around the value name, and the result asked for is found from the question's words. A calculation is
//! answered only when ONE reading fits: every value placed, every required input filled, the asked result named, no
//! other placement of the values as good, and every other calculation that also fits giving the same number.
//! Otherwise the reply says why not. The engine computes and checks every number.
//!
//! The steps, as a person works a problem:
//!   1. quantities  every number with its unit (`candidates`), the unit's dimension (`parse_unit`), its words
//!   2. asked       the result the question asks for (`asked_outputs`, guarded by `asked_supported`)
//!   3. law         every calculation whose inputs can take all the values and that has the asked result
//!   4. solve       the engine solves each reading; readings that disagree are refused

use crate::{asked_supported, candidates, everyday_unit, meaning_words, named_outputs, numbers, rule_values, slice};
use ax_engine::engine::{solve, Answer, Input};
use ax_engine::pack::{ProcInput, ProcOutput, Procedure};
use ax_engine::units::{parse_unit, Dim};
use ax_engine::Pack;
use serde_json::{json, Value};
use std::collections::HashSet;

const NOT_ASKED: &str = "no result of this calculation is asked for";

/// At most this many placements of the values are tried for one calculation.
const MAX_PLACEMENTS: usize = 20_000;
/// The best placement must beat every other by this much (one word naming the input); otherwise the readings are
/// solved and must agree.
const PLACE_MARGIN: f32 = 1.0;
/// A rival placement of the values counts when it reads within this much of the best (kept at one word: 0.75 let a
/// swapped pair of saturation values through, spent held-out 2026-11-10f c073; the networks' tags settle ties instead).
const RIVAL_MARGIN: f32 = 1.0;
/// A symbol written right before a value ("sigma y 160 m", "L0 = 200 mg/L") counts as this many words.
const SYMBOL_WORDS: f32 = 3.0;

/// A value written in the question.
#[derive(Clone, Debug)]
pub struct Mention {
    pub s: usize,
    pub e: usize,
    pub text: String,
    /// the unit as the engine reads it ("" when none is written)
    pub unit: String,
    /// the unit's dimensions (a bare "C" may be a temperature); empty when the unit is not readable
    pub dims: Vec<Dim>,
    pub dimless: bool,
    pub pct: bool,
    /// the words before the value that describe it, nearest last: (lowercase, as written)
    left: Vec<(String, String)>,
    /// the words after it that describe it ("4500 m2 OF PLATES", "40 m OFF THE CENTRELINE"), nearest first
    right: Vec<String>,
    /// where those words start in the question (characters), left then right
    at: Vec<usize>,
    /// words it takes from its phrase, with their weight: the first value's words for a value joined to it by "at"
    /// or "with", "initial" / "final" for the values of "from A to B"
    extra: Vec<(String, f32)>,
    /// the numbers of a list of values (empty for one value)
    pub list: Vec<String>,
}

/// Words that join two value phrases: what comes after one belongs to the next value.
const JOIN: &[&str] = &["and", "with", "plus", "while", "whereas", "but", "at", "into", "then", "where", "when", "in"];
/// Words that start a description written after a value ("40 m off the centreline", "8.4 mg/L at the process
/// temperature", "45 g/s of sulfur dioxide").
const AFTER: &[&str] = &["off", "above", "below", "from", "downstream", "downwind", "upstream", "away", "of", "at", "for",
                         "as", "across", "along", "under", "over", "behind", "beyond", "inside", "within"];
/// Words skipped when counting how far a describing word is from its value.
fn content(t: &str) -> bool {
    !meaning_words(t).is_empty()
}

/// A word of the question: char range, lowercase, as written.
struct Tok {
    s: usize,
    e: usize,
    low: String,
    raw: String,
}

fn toks(q: &str) -> Vec<Tok> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut start = 0;
    let mut n = 0;
    for (i, c) in q.chars().enumerate() {
        if c.is_alphanumeric() || c == '_' || (c == '.' && !cur.is_empty() && cur.chars().all(|x| x.is_ascii_digit())) {
            if cur.is_empty() {
                start = i;
            }
            cur.push(c);
        } else if !cur.is_empty() {
            out.push(Tok { s: start, e: i, low: cur.to_lowercase(), raw: std::mem::take(&mut cur) });
        }
        n = i + 1;
    }
    if !cur.is_empty() {
        out.push(Tok { s: start, e: n, low: cur.to_lowercase(), raw: cur });
    }
    out
}

/// "day-1" -> "day^-1", "-day" (of "5-day") -> "day".
pub(crate) fn tidy_unit(u: &str) -> String {
    let u = u.trim().trim_start_matches('-');
    let mut out = String::new();
    let cs: Vec<char> = u.chars().collect();
    for (i, c) in cs.iter().enumerate() {
        if *c == '-' && i > 0 && cs[i - 1].is_alphabetic() && cs.get(i + 1).is_some_and(|d| d.is_ascii_digit()) {
            out.push_str("^-");
        } else {
            out.push(*c);
        }
    }
    out
}

/// Every value of the question with its unit and the words that describe it.
pub fn mentions(q: &str) -> Vec<Mention> {
    let ts = toks(q);
    let qc: Vec<char> = q.chars().collect();
    // a number inside an earlier value's unit ("1" of "0.23 d-1") is part of it
    let mut spans: Vec<(usize, usize)> = vec![];
    for (s, e) in candidates(q) {
        if spans.last().is_none_or(|l| s >= l.1) {
            spans.push((s, e));
        }
    }
    // a list of values is one value: "88, 92, 97 dBA", "for 4, 2, 1 hours", "on days 1, 2, 3, 5, 7" (numbers joined only
    // by commas or "and", a unit written once after the last)
    let bare = |s: usize, e: usize| numbers(&slice(q, s, e)).len() == 1 && slice(q, s, e).trim().chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == ',' || c == '-' || c == '+');
    let mut groups: Vec<Vec<(usize, usize)>> = vec![];
    for &(s, e) in &spans {
        let joins = groups.last().and_then(|g| g.last()).is_some_and(|&(ps, pe)| {
            let gap: String = slice(q, pe, s).to_lowercase();
            let g = gap.trim();
            bare(ps, pe) && (g == "," || g == ", and" || g == "and" || g == ",and")
        });
        if joins {
            groups.last_mut().expect("group").push((s, e));
        } else {
            groups.push(vec![(s, e)]);
        }
    }
    let mut out: Vec<Mention> = vec![];
    for g in &groups {
        let (s, e) = (g[0].0, g[g.len() - 1].1);
        let list: Vec<String> = if g.len() >= 2 {
            g.iter().filter_map(|&(a, b)| {
                let t = slice(q, a, b);
                numbers(&t).first().map(|&(x, y)| slice(&t, x, y).replace(',', ""))
            }).collect()
        } else {
            vec![]
        };
        let text = if list.is_empty() { slice(q, s, e) } else { slice(q, g[g.len() - 1].0, e) };
        let tidy = everyday_unit(&text, false);
        let unit = tidy.split_once(' ').map(|x| tidy_unit(x.1)).unwrap_or_default();
        let pct = text.contains('%');
        let mut dims = vec![];
        let mut dimless = unit.is_empty();
        if !unit.is_empty() && !pct {
            if let Ok(u) = parse_unit(&unit) {
                if u.dim == Dim::one() {
                    dimless = true;
                } else {
                    dims.push(u.dim);
                }
            }
            if unit == "C" {
                if let Ok(t) = parse_unit("degC") {
                    dims.push(t.dim);
                }
            }
        }
        let text = if list.is_empty() { text } else { slice(q, s, e) };
        out.push(Mention { s, e, text, unit, dims, dimless, pct, left: vec![], right: vec![], at: vec![], extra: vec![], list });
    }
    // the words between two values (and before the first, after the last) are shared out: a clause mark ends one
    // value's words; otherwise a describing phrase written after a value (one starting "off", "at", "of", ...) is
    // that value's up to a joining word ("and", "at", ...), and the rest describes the next value
    let mark = |i: usize| ",;:()?!".contains(qc[i]) || (qc[i] == '.' && qc.get(i + 1).is_none_or(|c| c.is_whitespace()));
    for k in 0..=out.len() {
        let a = if k == 0 { 0 } else { out[k - 1].e };
        let b = if k == out.len() { qc.len() } else { out[k].s };
        if a >= b {
            continue;
        }
        let first_mark = (a..b).find(|&i| mark(i));
        let last_mark = (a..b).rev().find(|&i| mark(i));
        let gap: Vec<&Tok> = ts.iter().filter(|t| t.s >= a && t.e <= b).collect();
        let (to_prev, to_next): (Vec<&Tok>, Vec<&Tok>) = match (first_mark, last_mark) {
            (Some(f), Some(l)) => (gap.iter().filter(|t| t.e <= f).copied().collect(),
                                   gap.iter().filter(|t| t.s > l).copied().collect()),
            _ if k == 0 => (vec![], gap.clone()),
            _ if k == out.len() => (gap.clone(), vec![]),
            _ => {
                let starts_after = gap.first().is_some_and(|t| AFTER.contains(&t.low.as_str()));
                // a preposition right before the next value is that value's ("backwashed AT 50 m/h"), not a join
                let last = gap.len().saturating_sub(1);
                let join = gap.iter().enumerate().skip(1)
                    .find(|(i, t)| JOIN.contains(&t.low.as_str()) && !(*i == last && AFTER.contains(&t.low.as_str())))
                    .map(|(i, _)| i)
                    .or_else(|| gap.first().filter(|t| JOIN.contains(&t.low.as_str()) && !starts_after).map(|_| 0));
                match join {
                    Some(j) if starts_after || j > 0 => (gap[..j].to_vec(), gap[j..].to_vec()),
                    Some(j) => (vec![], gap[j..].to_vec()),
                    None if starts_after => (gap.clone(), vec![]),
                    // "50 m crosswind at 1 m height": the words may describe either value, so they count for both and
                    // a placement that hangs on them ties (refused) rather than guessing
                    None if gap.last().is_some_and(|t| AFTER.contains(&t.low.as_str()))
                        && gap[..last].iter().any(|t| content(&t.low)) => (gap[..last].to_vec(), gap.clone()),
                    None => (vec![], gap.clone()),
                }
            }
        };
        if k > 0 {
            out[k - 1].right = to_prev.iter().map(|t| t.low.clone()).collect();
            let pos: Vec<usize> = to_prev.iter().map(|t| t.s).collect();
            out[k - 1].at.extend(pos);
        }
        if k < out.len() {
            out[k].left = to_next.iter().map(|t| (t.low.clone(), t.raw.clone())).collect();
            let pos: Vec<usize> = to_next.iter().map(|t| t.s).collect();
            out[k].at.splice(0..0, pos);
        }
    }
    // values joined in one phrase: "discharges 0.5 m3/s AT 40 mg/L", "DO 2.4 mg/L AT 3 min": the second describes the
    // same thing as the first and takes its words; "FROM 500 ug/L (down) TO 5 ug/L": the first is the start (initial,
    // influent, before), the second the end (final, effluent, after, target)
    for k in 1..out.len() {
        let (a, b) = (out[k - 1].e, out[k].s);
        if (a..b).any(|i| mark(i)) {
            continue;
        }
        let gap: Vec<String> = ts.iter().filter(|t| t.s >= a && t.e <= b).map(|t| t.low.clone()).collect();
        let g: Vec<&str> = gap.iter().map(|s| s.as_str()).collect();
        let from = out[k - 1].left.last().is_some_and(|x| x.0 == "from");
        if from && matches!(g.as_slice(), ["to"] | ["down", "to"] | ["up", "to"]) {
            for w in ["initial", "influent", "before", "first", "start"] {
                out[k - 1].extra.push((w.to_string(), 1.0));
            }
            for w in ["final", "effluent", "after", "target", "required", "second", "end"] {
                out[k].extra.push((w.to_string(), 1.0));
            }
        } else if matches!(g.as_slice(), ["at"] | ["with"]) && !out[k].left.iter().any(|x| content(&x.0)) {
            let inherited: Vec<(String, f32)> = out[k - 1].left.iter().rev().filter(|x| content(&x.0))
                .enumerate().map(|(d, x)| (x.0.clone(), 0.9 * near(d))).collect();
            out[k].extra.extend(inherited);
        }
    }
    out
}

/// The weight of a describing word by how many content words separate it from its value (nearest = 1).
fn near(d: usize) -> f32 {
    [1.0, 0.8, 0.6, 0.5, 0.4, 0.3].get(d).copied().unwrap_or(0.0)
}

fn same_word(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    // "plate" / "plates", "settl(ing)": one word starts the other (4+ letters), or they differ only in their last two
    // letters; never "hypochlorite" / "hypochlorous"
    let (la, lb) = (a.chars().count(), b.chars().count());
    let n = a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count();
    // ("pack" / "packed": 4 letters in common, at most 2 more in the shorter word)
    (n == la.min(lb) && n >= 4) || (n >= 4 && n + 2 >= la.min(lb)) || (n >= 5 && n + 2 >= la.max(lb))
}

/// A word's meaning words as compared: chemical formulas written out, and the everyday words for a dimension ("8 m
/// long" is a length; `asked_synonym` for the rest).
fn mwords(text: &str) -> HashSet<String> {
    meaning_words(&crate::expand_terms(text))
}

/// The words of a question word as they are compared (`meaning_words`, which drops short and unit words).
fn qwords(t: &str) -> Vec<String> {
    let t = match t {
        "long" => "length",
        "high" => "height",
        "across" => "diameter",
        "thick" => "thickness",
        _ => crate::asked_synonym(t),
    };
    mwords(t).into_iter().collect()
}

/// How strongly the words describing a value name a set of words (an input's or an output's).
fn words_score(m: &Mention, words: &HashSet<String>) -> f32 {
    let mut sc = 0.0;
    let mut seen: HashSet<String> = HashSet::new();
    let mut d = 0;
    for (l, _) in m.left.iter().rev() {
        let ws = qwords(l);
        if ws.is_empty() {
            continue;
        }
        for w in ws {
            if !seen.contains(&w) && words.iter().any(|x| same_word(&w, x)) {
                sc += near(d);
                seen.insert(w);
            }
        }
        d += 1;
    }
    let mut d = 0;
    for l in &m.right {
        let ws = qwords(l);
        if ws.is_empty() {
            continue;
        }
        for w in ws {
            if !seen.contains(&w) && words.iter().any(|x| same_word(&w, x)) {
                sc += near(d);
                seen.insert(w);
            }
        }
        d += 1;
    }
    for (l, wgt) in &m.extra {
        for w in qwords(l) {
            if !seen.contains(&w) && words.iter().any(|x| same_word(&w, x)) {
                sc += wgt;
                seen.insert(w);
            }
        }
    }
    sc
}

/// How strongly the words around a value name an input.
fn place_score(m: &Mention, inp: &ProcInput) -> f32 {
    let mut words: HashSet<String> = mwords(&inp.meaning);
    words.extend(mwords(&inp.symbol.replace('_', " ")));
    let mut sc = words_score(m, &words);
    // a list written in the unit its input states ("88, 92, 97 dBA" for "levels in dBA")
    if inp.ty == "vector" && list_unit(m, inp) == Some(true) {
        sc += 1.0;
    }
    // the symbol written right before the value ("sigma y 160 m", "L0 = 200 mg/L", "k of 0.23 per day")
    let parts: Vec<&str> = inp.symbol.split('_').filter(|p| !p.is_empty()).collect();
    // (a Greek letter written as a letter is its name: "σy 70 m" is sigma y; 2026-11-11)
    let greek = |w: &str| -> Vec<(String, String)> {
        const G: &[(char, &str)] = &[('σ', "sigma"), ('ρ', "rho"), ('θ', "theta"), ('η', "eta"), ('δ', "delta"),
                                     ('Δ', "delta"), ('λ', "lambda"), ('τ', "tau")];
        match w.chars().next().and_then(|c| G.iter().find(|g| g.0 == c)) {
            Some((c, name)) => {
                let rest: String = w.chars().skip(1).collect();
                let mut v = vec![(name.to_string(), name.to_string())];
                if !rest.is_empty() {
                    v.push((rest.to_lowercase(), rest));
                }
                let _ = c;
                v
            }
            None => vec![(w.to_lowercase(), w.to_string())],
        }
    };
    let lw_all: Vec<(String, String)> = m.left.iter()
        .filter(|(l, _)| !["of", "is", "was", "equals", "about", "around", "be"].contains(&l.as_str()))
        .flat_map(|(_, raw)| greek(raw)).collect();
    let lw: Vec<&(String, String)> = lw_all.iter().collect();
    if !parts.is_empty() && lw.len() >= parts.len() {
        let tail = &lw[lw.len() - parts.len()..];
        let long = inp.symbol.chars().filter(|c| c.is_alphanumeric()).count() >= 2;
        let hit = tail.iter().zip(&parts).all(|((l, raw), p)| if long { l == &p.to_lowercase() } else { raw == p });
        // ... or the initials of a symbol whose first part is a written-out name: "sy 40 m" is sigma y
        let initials = parts.len() == 2 && parts[0].chars().count() >= 3 && lw.last().is_some_and(|(l, _)| {
            *l == format!("{}{}", parts[0].chars().next().unwrap_or(' '), parts[1]).to_lowercase()
        });
        if hit || initials {
            sc += SYMBOL_WORDS;
        }
    }
    sc
}

/// Can this value go into this input? A dimensional input takes a value of its dimension; a pure number (or an
/// integer) takes a value with no unit, a percentage, or a unit of no dimension.
fn fits(p: &Pack, m: &Mention, inp: &ProcInput) -> bool {
    // a list goes into a list input (its numbers as written: the input is a pure number), a single value never does
    if inp.ty == "vector" {
        return !m.list.is_empty() && (inp.kind == "number" || inp.kind.is_empty()) && list_unit(m, inp) != Some(false);
    }
    if !m.list.is_empty() || !(inp.ty == "number" || inp.ty == "integer") {
        return false;
    }
    if inp.kind == "number" || inp.kind == "integer" || inp.kind.is_empty() {
        // the units of a pure number the engine does not read as units: "0.08 ppm", "75 dB", "88 dBA"
        let plain = ["ppm", "ppb", "db", "dba", "log", "logs"].contains(&m.unit.to_lowercase().as_str());
        return m.dims.is_empty() && (m.dimless || m.pct || m.unit.is_empty() || plain);
    }
    if inp.ty == "integer" && m.text.contains('.') {
        return false;
    }
    let Some(k) = p.kinds.get(&inp.kind) else { return false };
    m.dims.iter().any(|d| *d == k.dim)
}

/// A list's numbers go to the engine as written, so its unit must be the one the list input states ("times in
/// minutes", "levels in dBA"): Some(true) when it is, Some(false) when another unit is written (hours for minutes
/// would be read 60 times too small), None when the list or the input names no unit ("any one unit").
fn list_unit(m: &Mention, inp: &ProcInput) -> Option<bool> {
    let mean = inp.meaning.to_lowercase();
    if m.unit.is_empty() || mean.contains("any one unit") {
        return None;
    }
    let i = mean.find(" in ")?;
    let want: String = mean[i + 4..].split(|c: char| c.is_whitespace() || c == '(' || c == ',').next().unwrap_or("").to_string();
    if want.is_empty() {
        return None;
    }
    Some(unit_key(&want) == unit_key(&m.unit))
}

/// The text of a value as an input takes it (the same rules as the trained reader's `compute_reply`).
pub(crate) fn input_value(inp: &ProcInput, text: &str) -> Result<Input, String> {
    if inp.ty == "vector" {
        let xs: Vec<Value> = numbers(text).iter().map(|&(a, b)| slice(text, a, b).replace(',', ""))
            .map(|n| n.parse::<f64>().map(|x| json!(x)).map_err(|_| format!("unreadable number {n}")))
            .collect::<Result<_, _>>()?;
        return Ok(Input::Json(Value::Array(xs)));
    }
    if inp.kind == "number" || inp.kind == "integer" || inp.kind.is_empty() {
        let Some((s, e)) = numbers(text).first().copied() else { return Err("no number in the value".into()) };
        let num = slice(text, s, e).replace(',', "");
        let pct = text.contains('%');
        let m = inp.meaning.to_lowercase();
        // a percentage goes in as a fraction wherever the input takes a fraction ("fraction or %" takes both, and the
        // inputs of one calculation then agree: Dulong's C read "48" while H read 0.065)
        return if !pct {
            Ok(Input::Text(num))
        } else if m.contains("fraction") {
            num.parse::<f64>().map(|x| Input::Text(format!("{}", x / 100.0))).map_err(|_| "unreadable percentage".into())
        } else if m.contains("percent") || m.contains('%') {
            Ok(Input::Text(num))
        } else {
            Err(format!("a percentage for {}, which is not a fraction", inp.symbol))
        };
    }
    let t = everyday_unit(text, inp.kind == "temperature");
    let t = match t.split_once(' ') {
        Some((n, u)) => format!("{n} {}", tidy_unit(u)),
        None => t,
    };
    Ok(Input::Text(t))
}

/// Output groups: an output and the outputs that only restate it in another unit (C, C_ugm3), as `asked_supported`
/// groups them. (base symbol, member symbols, words)
fn groups(pr: &Procedure) -> Vec<(String, Vec<String>, HashSet<String>)> {
    let meaning = |s: &str| pr.outputs.iter().find(|o| o.symbol == s).map(|o| o.meaning.clone()).unwrap_or_default();
    let base = |s: &str| -> String {
        let ws = meaning_words(&meaning(s));
        pr.outputs.iter().map(|o| o.symbol.as_str())
            .filter(|b| *b != s && s.starts_with(&format!("{b}_")))
            .filter(|b| ws.is_subset(&meaning_words(&meaning(b))) || crate::unit_restatement(s, b, &meaning(s)))
            .max_by_key(|b| b.len())
            .unwrap_or(s)
            .to_string()
    };
    let mut out: Vec<(String, Vec<String>, HashSet<String>)> = vec![];
    for o in &pr.outputs {
        let b = base(&o.symbol);
        let mut w = mwords(&o.meaning);
        w.extend(mwords(&o.symbol.replace('_', " ")));
        // ... and the words of the output its symbol extends (Q_CH4_stp_m3_per_day "in m^3/d at 0 degC and 1 atm" is
        // methane too), as `asked_supported` counts them
        if let Some(p) = pr.outputs.iter().filter(|p| o.symbol.starts_with(&format!("{}_", p.symbol))).max_by_key(|p| p.symbol.len()) {
            w.extend(mwords(&p.meaning));
        }
        match out.iter_mut().find(|g| g.0 == b) {
            Some(g) => {
                g.1.push(o.symbol.clone());
                g.2.extend(w);
            }
            None => out.push((b, vec![o.symbol.clone()], w)),
        }
    }
    out
}

/// The part of the question that asks: the sentence with the last question mark; without one, from the last question
/// cue to the end of its sentence; without a cue, the last sentence. ("Contact tank 450 m3 ... CT in mg min/L?" asks
/// for the CT, not the contact time its first sentence describes.)
pub fn asked_clause(q: &str) -> String {
    let (a, b) = asked_span(q);
    q.chars().skip(a).take(b - a).collect::<String>().trim().to_string()
}

/// The part of the asked clause that names the result, before "given", "if", "when", "using", "with", "from": a value
/// written there is the condition the result is asked at ("the wind speed at 162 m given 6.81 m/s at 8.34 m").
pub(crate) fn asked_head(q: &str) -> (usize, usize) {
    let (a0, b) = asked_span(q);
    // from the last question word of the clause ("...68 dB at 20 m from the edge; WHAT level ... at 160 m?")
    const CUES: &[&str] = &["what", "whats", "how", "find", "calculate", "compute", "determine", "estimate", "which",
                            "give", "work"];
    let a = toks(q).iter().filter(|t| t.s >= a0 && t.e <= b && CUES.contains(&t.low.as_str())).map(|t| t.s).last()
        .unwrap_or(a0);
    let end = toks(q).iter().filter(|t| t.s > a && t.e <= b)
        .find(|t| ["given", "if", "when", "using", "with", "from", "assuming"].contains(&t.low.as_str()))
        .map(|t| t.s).unwrap_or(b);
    (a, end)
}

/// `asked_clause` as a character range.
fn asked_span(q: &str) -> (usize, usize) {
    let cs: Vec<char> = q.chars().collect();
    let end_of = |i: usize| cs.get(i).is_some_and(|c| "?!".contains(*c))
        || (cs.get(i) == Some(&'.') && cs.get(i + 1).is_none_or(|c| c.is_whitespace()));
    let start_before = |i: usize| (0..i).rev().find(|&j| end_of(j)).map(|j| j + 1).unwrap_or(0);
    let text = |a: usize, b: usize| (a, b.min(cs.len()));
    if let Some(qm) = (0..cs.len()).rev().find(|&i| cs[i] == '?') {
        return text(start_before(qm), qm + 1);
    }
    let low: Vec<char> = q.to_lowercase().chars().collect();
    // ("convert", "express": exam instructions, the owner's exam set 2026-11-11)
    const CUES: &[&str] = &["what", "how", "find", "calculate", "compute", "determine", "estimate", "which", "work out",
                            "give", "tell me", "convert", "express"];
    let mut at = None;
    for c in CUES {
        let cc: Vec<char> = c.chars().collect();
        for i in 0..low.len().saturating_sub(cc.len() - 1) {
            if low[i..i + cc.len()] == cc[..] && (i == 0 || !low[i - 1].is_alphanumeric())
                && low.get(i + cc.len()).is_none_or(|x| !x.is_alphanumeric()) && at.is_none_or(|a| i > a) {
                at = Some(i);
            }
        }
    }
    let from = at.unwrap_or_else(|| start_before(cs.len().saturating_sub(1)));
    let to = (from..cs.len()).find(|&i| end_of(i)).map(|i| i + 1).unwrap_or(cs.len());
    text(from, to)
}

/// Letters and digits of a unit or a symbol's unit part, "per" dropped and unit words abbreviated: "kg/h",
/// "kg_per_h" and "kilograms per hour" are all "kgh"; "micrograms per cubic metre" is "ugm3".
fn unit_key(t: &str) -> String {
    let mut out = String::new();
    let mut pow = "";
    for w in t.to_lowercase().replace('µ', "u").replace('μ', "u").split(|c: char| !c.is_alphanumeric()) {
        let w = match w {
            "" | "per" | "a" | "an" => continue,
            // what is measured, not the unit: "kg BOD/(m3 day)", "mg/L as CaCO3" (exam set 2026-11-11)
            "bod" | "bod5" | "cod" | "as" | "caco3" | "vss" | "tss" | "ss" | "dry" | "solids" => continue,
            "cubic" => { pow = "3"; continue }
            "square" | "sq" => { pow = "2"; continue }
            "metres" | "meters" | "metre" | "meter" => "m",
            "litres" | "liters" | "litre" | "liter" => "l",
            "hours" | "hour" | "hr" | "hrs" => "h",
            "days" | "d" => "day",
            "minutes" | "minute" | "mins" => "min",
            "seconds" | "second" | "sec" => "s",
            "years" | "year" | "yr" | "yrs" => "year",
            "micrograms" | "microgram" => "ug",
            "milligrams" | "milligram" => "mg",
            "kilograms" | "kilogram" => "kg",
            "grams" | "gram" => "g",
            "tonnes" | "tonne" | "tons" | "ton" => "t",
            "kilometres" | "kilometers" | "kilometre" | "kilometer" => "km",
            w => w,
        };
        out.push_str(w);
        out.push_str(pow);
        pow = "";
    }
    out
}

/// The member of an output group the question asks for by its unit ("in ug/m3" -> C_ugm3, "in LMH" -> J_LMH): the
/// base when no member's unit is written after "in" in the asked clause.
fn variant(pr: &Procedure, members: &[String], base: &str, clause: &str) -> String {
    let asked_units = asked_unit_keys(clause);
    for m in members {
        if m == base {
            continue;
        }
        let part = m.strip_prefix(&format!("{base}_")).unwrap_or("");
        let k = unit_key(part);
        if !k.is_empty() && asked_units.iter().any(|a| a == &k) {
            return m.clone();
        }
        // "in micrograms per cubic metre", "in litres per m2 per hour": the output's own meaning
        let mk = unit_key(&pr.outputs.iter().find(|o| &o.symbol == m).map(|o| o.meaning.clone()).unwrap_or_default());
        if mk.len() > 3 && asked_units.iter().any(|a| a.len() > 3 && mk.ends_with(a.as_str())) {
            return m.clone();
        }
    }
    base.to_string()
}

/// The units written after "in" / "to" in the asked clause, as `unit_key`s (whole phrase, first word, first 2-6 words).
fn asked_unit_keys(clause: &str) -> Vec<String> {
    // (a leading space: the clause may start with the "to" of "convert ... to")
    let low = format!(" {}", clause.to_lowercase());
    let mut asked_units: Vec<String> = vec![];
    // after "in", or after "to" ("convert this concentration TO micrograms per cubic meter", exam set 2026-11-11): only
    // a member's own unit can match, so "to" elsewhere does nothing
    let marks: Vec<usize> = low.match_indices(" in ").chain(low.match_indices(" to ")).map(|(i, _)| i).collect();
    for i in marks {
        let w: String = low[i + 4..].split(|c: char| c.is_whitespace() || c == '?' || c == ',').next().unwrap_or("").to_string();
        let w2: String = low[i + 4..].split(|c: char| c == '?' || c == ',').next().unwrap_or("").to_string();
        asked_units.push(unit_key(&w));
        asked_units.push(unit_key(&w2));
        // ... and its first words: "in m3 per year AT 25 YEARS after opening", "in tonnes per year: pop ..."
        let ws: Vec<&str> = w2.split(|c: char| c.is_whitespace() || c == ':' || c == ';').filter(|x| !x.is_empty()).collect();
        for n in 2..=ws.len().min(6) {
            asked_units.push(unit_key(&ws[..n].join(" ")));
        }
    }
    asked_units
}

/// The result the question asks for in this calculation: (base symbol, evidence), or why it cannot tell. `given` are
/// the words that label a value placed in an input they name ("water flow 0.0294 m^3/s"): they describe what is
/// given, not what is asked (the minimum air-to-water ratio was "asked" by "water flow ... air flow").
fn asked_output(p: &Pack, pr: &Procedure, q: &str, given: &HashSet<usize>) -> Result<(String, f32), String> {
    let gs = groups(pr);
    let (mut a, b) = asked_span(q);
    // "convert ... in the atm form TO the dimensionless form": the result is named after the last "to" / "into"
    let ts = toks(q);
    let inside: Vec<&Tok> = ts.iter().filter(|t| t.s >= a && t.e <= b).collect();
    if inside.iter().any(|t| ["convert", "express", "change", "turn", "correct", "adjust"].contains(&t.low.as_str())) {
        if let Some(t) = inside.iter().rev().find(|t| t.low == "to" || t.low == "into") {
            a = t.s;
        }
    }
    let clause: String = q.chars().skip(a).take(b - a).collect();
    let mut cw: Vec<String> = vec![];
    let mut cw_raw: Vec<String> = vec![];
    for t in ts.iter().filter(|t| t.s >= a && t.e <= b && !given.contains(&t.s)) {
        let w = t.low.as_str();
        cw_raw.push(w.to_string());
        cw.push(crate::asked_synonym(w).to_string());
        // "packing depth" is the packed height
        if w == "depth" || w == "deep" {
            cw.push("height".to_string());
        }
        // "the lowest DO", "what DO after 15 min": DO written in capitals is dissolved oxygen ("do" is a verb)
        if w == "do" && slice(q, t.s, t.e) == "DO" {
            cw.push("dissolved".to_string());
            cw.push("oxygen".to_string());
        }
    }
    // (only a symbol written where the question asks: "The CT for three-log is 50 mg·min/L. How many logs?" gives the
    // CT, it does not ask for it)
    let named: HashSet<String> = named_outputs(pr, &clause).into_iter().collect();
    // a result written as a ratio its meaning gives: "N over N naught" is N/N0
    let qn: String = format!(" {} ", q.to_lowercase()).replace(" over ", "/").replace(" naught", "0").replace(" nought", "0")
        .replace(" zero", "0").chars().filter(|c| !c.is_whitespace()).collect();
    // (never a unit: "in mg/L" or "1.46 mg/L" does not name a ratio)
    let ratio_named = |sym: &str| -> bool {
        pr.outputs.iter().filter(|o| o.symbol == sym).any(|o| o.meaning.split_whitespace().any(|raw| {
            let raw = raw.trim_matches(|c: char| !c.is_alphanumeric());
            let t = raw.to_lowercase();
            t.len() >= 3 && t.contains('/') && t.chars().all(|c| c.is_alphanumeric() || c == '/')
                && parse_unit(everyday_unit(&format!("1 {raw}"), false).get(2..).unwrap_or("")).is_err()
                && qn.match_indices(t.as_str()).any(|(i, _)| !qn[..i].chars().last().is_some_and(|c| c.is_ascii_digit() || c == '.'))
        }))
    };
    // "how far", "how many days", "what fraction": the result is a length, a time, a pure number
    let cl = format!(" {} ", clause.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" "));
    let cue = |cs: &[&str]| cs.iter().any(|c| cl.contains(&format!(" {c} ")));
    // "when does the sag bottom out, IN DAYS?": a unit asked for fixes the dimension of the result (held-out
    // 2026-11-10d c048 answered the lowest DO); so does "when does / when will"
    let mut asked_dim: Option<Dim> = None;
    let words_cl: Vec<&str> = cl.split_whitespace().collect();
    // (a plain time unit only, not "in mg/L per hour")
    const TIME: &[&str] = &["days", "day", "hours", "hour", "hrs", "h", "minutes", "minute", "mins", "min", "seconds", "s",
                            "years", "year", "yr", "d"];
    for (i, w) in words_cl.iter().enumerate() {
        // (never a time unit with an exponent: "in d^-1" is a rate, "in d 1" once its signs are dropped; held-out
        // 2026-11-12b c15 read the F/M ratio "in d^-1" as the sludge age in days)
        if *w == "in" && words_cl.get(i + 1).is_some_and(|u| TIME.contains(u))
            && !words_cl.get(i + 2).is_some_and(|x| *x == "per" || x.chars().all(|c| c.is_ascii_digit())) {
            asked_dim = parse_unit("s").ok().map(|u| u.dim);
        }
    }
    // ... and so does any unit asked for: "what's left IN MG/L 10 km down" asks a concentration (two-step development
    // rows 2026-11-11), the way "in days" asks a time
    if asked_dim.is_none() {
        let low = clause.to_lowercase();
        for (i, _) in low.match_indices(" in ") {
            let rest: Vec<&str> = low[i + 4..].split(|c: char| c.is_whitespace() || c == '?' || c == ',' || c == ';')
                .filter(|x| !x.is_empty()).take(3).collect();
            for n in (1..=rest.len()).rev() {
                let u = everyday_unit(&format!("1 {}", rest[..n].join(" ")), false);
                if let Some(d) = u.split_once(' ').and_then(|x| parse_unit(&tidy_unit(x.1)).ok()).map(|u| u.dim) {
                    if d != Dim::one() {
                        asked_dim = Some(d);
                    }
                    break;
                }
            }
            if asked_dim.is_some() {
                break;
            }
        }
    }
    if asked_dim.is_none() && cue(&["when does", "when will", "when is", "when do", "how soon", "at what time"]) {
        asked_dim = parse_unit("s").ok().map(|u| u.dim);
    }
    let dim_ok = |base: &str| -> bool {
        let Some(d) = asked_dim else { return true };
        let members = gs.iter().find(|g| g.0 == base).map(|g| g.1.clone()).unwrap_or_default();
        members.iter().any(|m| {
            let o = pr.outputs.iter().find(|o| &o.symbol == m);
            let kd = o.and_then(|o| p.kinds.get(&o.kind)).map(|k| k.dim);
            // a pure-number variant named by its unit: t_c_days, C_ugm3
            let suffix = m.strip_prefix(&format!("{base}_")).unwrap_or("");
            let by_name = !suffix.is_empty() && parse_unit(&everyday_unit(&format!("1 {}", suffix.replace("_per_", "/").replace('_', "*")), false)
                .split_once(' ').map(|x| tidy_unit(x.1)).unwrap_or_default()).is_ok_and(|u| u.dim == d);
            // ... or by the unit its meaning states ("total hardness, mg/L as CaCO3": a pure number in mg/L; the
            // owner's exam set 2026-11-11 asked it "in mg/L as CaCO3")
            let by_meaning = o.is_some_and(|o| o.meaning.split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')')
                .filter(|t| t.contains('/'))
                .any(|t| parse_unit(&tidy_unit(t)).is_ok_and(|u| u.dim == d)));
            kd == Some(d) || by_name || by_meaning
        })
    };
    let want_kind: Option<&str> = if cue(&["how far", "how deep", "how tall", "how wide", "what distance", "what depth", "what height", "where is the", "where does the", "where will the"]) {
        Some("length")
    } else if cue(&["how many days", "how many hours", "how many minutes", "how long does", "how long will", "how long until",
                    "after how long", "how much time"]) {
        Some("time")
    } else {
        None
    };
    let kind_of = |s: &str| pr.outputs.iter().find(|o| o.symbol == s).map(|o| o.kind.clone()).unwrap_or_default();
    // a fraction, ratio or percentage is asked in those words or by its own words ("collection efficiency"), not by an
    // everyday synonym: "what's LEFT 15 km downstream" of 20 mg/L asks the concentration, not the fraction remaining
    // (spent held-out 2026-11-09b c033)
    let ratio_words = cue(&["fraction", "fractions", "percent", "percentage", "ratio", "proportion", "share", "how much of",
                            "what part", "what portion", "portion"]) || clause.contains('%');
    let is_ratio = |b: &str| pr.outputs.iter().find(|o| o.symbol == b).is_some_and(|o| {
        // (its meaning or its symbol says so, or the meaning is a ratio of two symbols: "C/C0")
        let m = format!("{} {}", o.meaning.to_lowercase(), o.symbol.to_lowercase().replace('_', " "));
        let written_ratio = o.meaning.split_whitespace().any(|w| {
            let w = w.trim_matches(|c: char| !c.is_alphanumeric());
            w.split_once('/').is_some_and(|(x, y)| !x.is_empty() && !y.is_empty()
                && x.chars().chain(y.chars()).all(|c| c.is_alphanumeric() || c == '_'))
                && parse_unit(&tidy_unit(w)).is_err()
        });
        matches!(o.kind.as_str(), "number" | "")
            && (written_ratio || ["fraction", "ratio", "percent", "proportion", "share"].iter().any(|w| m.contains(w)))
    });
    let mut cands: Vec<(String, f32)> = vec![];
    let mut all: Vec<(String, f32)> = vec![];
    for (b, members, words) in &gs {
        if want_kind.is_some_and(|k| kind_of(b) != k) || !dim_ok(b) {
            continue;
        }
        let mut ev = 0.0;
        let mut seen = HashSet::new();
        // (a fraction without a ratio word: named by the question's own words only, not by an everyday synonym)
        let raw_only = is_ratio(b) && !ratio_words;
        for t in if raw_only { &cw_raw } else { &cw } {
            let ws: Vec<String> = if raw_only { mwords(t).into_iter().collect() } else { qwords(t) };
            for w in ws {
                if !seen.contains(&w) && words.iter().any(|x| same_word(&w, x)) {
                    ev += 1.0;
                    seen.insert(w);
                }
            }
        }
        if members.iter().any(|m| named.contains(m) || ratio_named(m)) {
            ev += SYMBOL_WORDS;
        }
        if ev > 0.0 {
            all.push((b.clone(), ev));
        }
    }
    // the kind asked for is itself the evidence when it leaves exactly one result ("when does the sag bottom out, in
    // days?": the critical time is the calculation's only time)
    if all.is_empty() && (want_kind.is_some() || asked_dim.is_some()) {
        let only: Vec<&String> = gs.iter().map(|g| &g.0)
            .filter(|b| want_kind.is_none_or(|k| kind_of(b) == k) && dim_ok(b)).collect();
        if only.len() == 1 {
            all.push((only[0].clone(), 1.0));
        }
    }
    dbg_why(&pr.key, &format!("asked words {cw:?} -> {all:?}"));
    // the symbol of a group that passes the wording checks: the unit variant the question asks for first, then the
    // base, then the other variants ("Px" and "Px_kg_per_day" are one result; the check may take only one of them)
    let passing = |base: &str| -> Result<String, String> {
        let members = gs.iter().find(|g| g.0 == base).map(|g| g.1.clone()).unwrap_or_default();
        let first = variant(pr, &members, base, &clause);
        let mut order = vec![first.clone(), base.to_string()];
        order.extend(members.iter().filter(|m| **m != first && *m != base).cloned());
        let mut err = String::new();
        for m in order {
            match asked_supported(pr, q, &m) {
                Ok(()) => return Ok(m),
                Err(w) if err.is_empty() => err = w,
                Err(_) => {}
            }
        }
        Err(err)
    };
    // the result named best must itself pass the wording checks: never fall back to a result named less well
    // ("what fraction is hypochlorite ion" was answered with the hypochlorous fraction when the check refused it)
    let best = all.iter().map(|c| c.1).fold(0.0f32, f32::max);
    let mut sym_of: Vec<(String, String)> = vec![];
    for c in all.iter().filter(|c| c.1 >= best) {
        sym_of.push((c.0.clone(), passing(&c.0)?));
    }
    cands.extend(all.into_iter().filter(|c| c.1 >= best));
    cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    // no word names a result, but the unit asked for is the own unit of exactly one result ("convert this
    // concentration to micrograms per cubic meter": C_ugm3; exam set 2026-11-11): that result is asked
    if cands.is_empty() {
        // every result whose symbol spells EXACTLY the unit asked for after its base ("OLR_kg_per_m3_day" for "in kg
        // BOD/(cubic meter * day)", "C_ugm3" for "to micrograms per cubic meter"); never a partial match ("in m per day"
        // matched the weir loading "m3 per m day" when SOR was not recognised; validation 2026-11-11)
        let asked = asked_unit_keys(&clause);
        // ... and only when the asking part converts ("convert", "express") or one of its own words (even a word that
        // also labels a given value) is the result's: "give IT in micrograms per cubic metre" names nothing and let a
        // 3-step chain answer the ground-centreline value for an off-axis receptor (spent c005, 2026-11-11)
        let full_clause = asked_clause(q).to_lowercase();
        // ... and its other content words must all be this calculation's own ("convert this CONCENTRATION": the ppm
        // conversion; "express the TOTAL HARDNESS of this WATER SAMPLE in mg/L" is not the BOD test's mg/L result;
        // exam set Q3 2026-11-11)
        let mut vocab: HashSet<String> = mwords(&pr.name);
        for o in &pr.outputs {
            vocab.extend(mwords(&o.meaning));
        }
        for i in &pr.inputs {
            vocab.extend(mwords(&i.meaning));
        }
        // (only the words before the unit asked: "... to micrograms per cubic meter", "... in kg BOD/(m3 d)")
        let lc = format!(" {full_clause} ");
        let cut = [" in ", " to ", " into "].iter().filter_map(|m| lc.find(m)).min().unwrap_or(lc.len());
        // (the asking verbs and pointers compared as normalised words too: "this" is "thi")
        let plain = mwords("convert express give value answer result this that these those sample calculate determine find \
                            compute estimate work out what how much");
        let extra: Vec<String> = mwords(&lc[..cut]).into_iter()
            .filter(|w| !plain.contains(w) && !vocab.iter().any(|x| same_word(w, x)))
            .collect();
        let converts = extra.is_empty()
            && ["convert", "express"].iter().any(|v| full_clause.split(|c: char| !c.is_alphanumeric()).any(|w| w == *v));
        let clause_words: HashSet<String> = full_clause.split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() >= 3).map(|w| w.to_string()).collect();
        let by_unit: Vec<String> = pr.outputs.iter()
            .filter(|o| pr.outputs.iter().any(|b| o.symbol.strip_prefix(&format!("{}_", b.symbol))
                .is_some_and(|part| { let k = unit_key(part); !k.is_empty() && asked.contains(&k) })))
            .filter(|o| converts || (extra.is_empty() && {
                let base = gs.iter().find(|g| g.1.contains(&o.symbol)).map(|g| g.2.clone()).unwrap_or_default();
                base.iter().any(|w| clause_words.contains(w))
            }))
            .map(|o| o.symbol.clone())
            .collect();
        dbg_why(&pr.key, &format!("by unit {by_unit:?} {:?} (extra words {extra:?}, clause {full_clause:?})",
                                  by_unit.first().map(|m| asked_supported(pr, q, m))));
        if let [m] = by_unit.as_slice() {
            match asked_supported(pr, q, m) {
                Ok(()) => {
                    dbg_why(&pr.key, &format!("asked by its unit only: {m}"));
                    return Ok((m.clone(), 1.0));
                }
                // the unit asks this calculation's result, only the wording is unclear: set aside for unclear wording
                // (it blocks a near-ask on another calculation), not "not asked" (suite 2026-11-12 c061)
                Err(w) => return Err(w),
            }
        }
    }
    let pick = match cands.len() {
        0 => return Err(NOT_ASKED.into()),
        1 => cands.remove(0),
        _ => {
            let main = gs.first().map(|g| g.0.clone()).unwrap_or_default();
            match cands.iter().find(|c| c.0 == main) {
                Some(c) => c.clone(),
                None => return Err(format!("the question may ask for {} or {}", cands[0].0, cands[1].0)),
            }
        }
    };
    let sym = sym_of.iter().find(|s| s.0 == pick.0).map(|s| s.1.clone()).unwrap_or(pick.0.clone());
    Ok((sym, pick.1))
}

/// A first step of a chained reading (recipe A22): the calculation `key`, the results it passes on (its output, the
/// final calculation's input) and where each of its inputs sits in the combined calculation (index, its own symbol).
#[derive(Clone, Debug)]
struct Step {
    key: String,
    outs: Vec<(String, String)>,
    map: Vec<(usize, String)>,
    /// (its input, an input of the combined calculation that is the same quantity): with no value of its own, the
    /// input takes that one's ("wind 5 m/s" feeds the plume rise and the dispersion; "after 5 days ... after 10 days"
    /// gives each its own time)
    fallback: Vec<(usize, usize)>,
}

/// A chained reading: the final calculation's inputs that no value gives are the results of first steps, worked out
/// from values the question gives. Inputs [0, n2) of the combined calculation are the final calculation's own.
#[derive(Clone, Debug)]
struct Chain {
    steps: Vec<Step>,
    n2: usize,
}

/// One way of reading the question as a calculation.
#[derive(Clone, Debug)]
struct Reading {
    key: String,
    asked: String,
    asked_ev: f32,
    place: f32,
    given: Vec<(String, Input)>,
    texts: Vec<(String, String)>,
    chain: Option<Chain>,
    /// each first step's values (chained readings only)
    firsts: Vec<Given>,
}

/// Values for one calculation: (symbol, value) and (symbol, text as written).
type Given = (Vec<(String, Input)>, Vec<(String, String)>);

/// Words of a meaning that say which quantity it is: a note in brackets, generic words ("rate", "constant") and words
/// of one or two letters ("at x", "hs + dh") left out.
fn link_words(t: &str) -> HashSet<String> {
    const STOP: &[&str] = &["the", "and", "for", "per", "from", "with", "its", "into", "day", "mol", "rate", "constant",
                            "coefficient", "value", "given"];
    let mut plain = String::new();
    let mut depth = 0;
    for c in t.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = (depth - 1).max(0),
            _ if depth == 0 => plain.push(c),
            _ => {}
        }
    }
    plain.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| w.len() >= 3 && !STOP.contains(w))
        .map(String::from).collect()
}

/// Two meanings that say the same quantity in the same words (at least two).
fn same_words(a: &str, b: &str) -> bool {
    let a = link_words(a);
    a.len() >= 2 && a == link_words(b)
}

/// Is the result `o` of one calculation the quantity the input `i` of another takes: the same kind (never a pure
/// number) and the same words ("effective stack height" of a plume-rise calculation is the dispersion calculation's
/// "effective stack height (stack height plus plume rise)"; "dissolved oxygen at t" is not "saturation dissolved
/// oxygen", "expanded bed depth" is not "bed depth", a landfill's methane volume is not a mixing basin's volume).
fn links(o: &ProcOutput, i: &ProcInput) -> bool {
    o.kind == i.kind && !matches!(i.kind.as_str(), "number" | "integer" | "") && i.ty == "number" && same_words(&o.meaning, &i.meaning)
}

/// Do two inputs of two calculations take the same quantity, so one value feeds both: the same kind, and the same
/// words, or the same symbol and half of all their words ("wind speed at the stack top" of a plume rise and "mean wind
/// speed at the stack height" of a dispersion calculation); a pure number needs the same symbol and the same words; a
/// choice the same symbol and choices.
fn shared_input(a: &ProcInput, b: &ProcInput) -> bool {
    if a.kind != b.kind || a.ty != b.ty {
        return false;
    }
    if a.ty == "choice" {
        return a.symbol == b.symbol && a.choices == b.choices;
    }
    if matches!(a.kind.as_str(), "number" | "integer" | "") {
        return a.symbol == b.symbol && same_words(&a.meaning, &b.meaning);
    }
    let (wa, wb) = (link_words(&a.meaning), link_words(&b.meaning));
    let shared = wa.intersection(&wb).count();
    // (a fallback only: a question that gives each its own value keeps both)
    same_words(&a.meaning, &b.meaning) || (a.symbol == b.symbol && shared >= 1)
}

/// Solve a reading: one calculation, or its first steps and then the final calculation with their results. Returns
/// the first steps' answers (as far as they were solved) and the final answer (a first step's refusal when one failed).
fn solve_reading(p: &Pack, key: &str, chain: Option<&Chain>, given: &[(String, Input)], firsts: &[Given],
                 asked: &str) -> (Vec<Answer>, Answer) {
    let Some(c) = chain else { return (vec![], solve(p, key, given, Some(asked))) };
    let mut g2: Vec<(String, Input)> = given.to_vec();
    let mut done = vec![];
    for (st, g1) in c.steps.iter().zip(firsts) {
        let a1 = solve(p, &st.key, &g1.0, st.outs.first().map(|o| o.0.as_str()));
        if !a1.solved() {
            return (done, a1);
        }
        for (out, slot) in &st.outs {
            let mid = a1.values.iter().find(|v| &v.0 == out)
                .and_then(|v| v.1.as_f64().filter(|x| x.is_finite()).map(|x| (x, v.2.clone())));
            let Some((x, u)) = mid else { return (done, a1) };
            g2.push((slot.clone(), Input::Text(if u.is_empty() { format!("{x:e}") } else { format!("{x:e} {u}") })));
        }
        done.push(a1);
    }
    let a2 = solve(p, key, &g2, Some(asked));
    (done, a2)
}

/// Every complete placement of the values into the calculation's inputs, best first: (score, input of each value).
fn placements(p: &Pack, pr: &Procedure, ms: &[Mention], head: (usize, usize)) -> Vec<(f32, Vec<usize>)> {
    let slots: Vec<usize> = (0..pr.inputs.len()).collect();
    let score: Vec<Vec<Option<f32>>> = ms.iter()
        .map(|m| slots.iter().map(|&i| fits(p, m, &pr.inputs[i]).then(|| place_score(m, &pr.inputs[i]))).collect())
        .collect();
    let mut out = vec![];
    let mut cur: Vec<usize> = vec![];
    let mut used = vec![false; pr.inputs.len()];
    fn go(k: usize, score: &[Vec<Option<f32>>], cur: &mut Vec<usize>, used: &mut Vec<bool>, acc: f32,
          out: &mut Vec<(f32, Vec<usize>)>, budget: &mut usize) {
        if *budget == 0 {
            return;
        }
        if k == score.len() {
            out.push((acc, cur.clone()));
            *budget -= 1;
            return;
        }
        for (i, s) in score[k].iter().enumerate() {
            if let Some(s) = s {
                if !used[i] {
                    used[i] = true;
                    cur.push(i);
                    go(k + 1, score, cur, used, acc + s, out, budget);
                    cur.pop();
                    used[i] = false;
                }
            }
        }
    }
    let mut budget = MAX_PLACEMENTS;
    go(0, &score, &mut cur, &mut used, 0.0, &mut out, &mut budget);
    if budget == 0 {
        return vec![];
    }
    // numbered inputs follow the written order: the first-written of C1 and C2 is C1 ("DO 2.4 mg/L at 3 min and 6 mg/L
    // at 12 min"); one word's weight per family of numbered inputs placed in order
    let mut fams: Vec<(String, Vec<(char, usize)>)> = vec![];
    for (i, inp) in pr.inputs.iter().enumerate() {
        let mut cs = inp.symbol.chars();
        if let Some(d @ ('1' | '2' | '3')) = cs.next_back() {
            let base: String = cs.collect();
            if !base.is_empty() && !base.ends_with(|c: char| c.is_ascii_digit()) {
                match fams.iter_mut().find(|f| f.0 == base) {
                    Some(f) => f.1.push((d, i)),
                    None => fams.push((base, vec![(d, i)])),
                }
            }
        }
    }
    fams.retain(|f| f.1.len() >= 2);
    if !fams.is_empty() {
        for (sc, assign) in out.iter_mut() {
            for (_, members) in &fams {
                let mut placed: Vec<(char, usize)> = members.iter()
                    .filter_map(|(d, i)| assign.iter().position(|a| a == i).map(|k| (*d, ms[k].s))).collect();
                // (not when one of them is written where the question asks: "the wind speed AT 162 M given 6.81 m/s at
                // 8.34 m" names the second height first)
                let asked_one = members.iter().filter_map(|(_, i)| assign.iter().position(|a| a == i))
                    .any(|k| ms[k].s >= head.0 && ms[k].e <= head.1);
                if placed.len() >= 2 && !asked_one {
                    placed.sort();
                    if placed.windows(2).all(|w| w[0].1 < w[1].1) {
                        *sc += 1.0;
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// AX_DERIVE_DEBUG=1: why each calculation was not a reading (to stderr; development only).
fn dbg_why(key: &str, why: &str) {
    if std::env::var("AX_DERIVE_DEBUG").is_ok_and(|v| v == "1") {
        eprintln!("[derive] {key}: {why}");
    }
}

/// The value of the asked output in a solved answer.
fn asked_value(a: &Answer, asked: &str) -> Option<Value> {
    a.values.iter().find(|v| v.0 == asked).map(|v| v.1.clone()).or_else(|| a.value.map(|x| json!(x)))
}

fn agree(a: &Value, b: &Value) -> bool {
    match (a.as_f64(), b.as_f64()) {
        (Some(x), Some(y)) => (x - y).abs() <= 1e-9 * x.abs().max(y.abs()).max(1e-300),
        _ => a == b,
    }
}

/// Inputs the question sets to zero by naming the case their meaning gives for zero: "salinity in g/kg (0 for fresh
/// water)" with "fresh water" written, "decay constant (0 if conservative)" with "a conservative pollutant", "effective
/// stack height (0 for a ground-level source)" with "ground-level source". Every word of the case must be written.
fn zero_by_wording(pr: &Procedure, q: &str) -> Vec<String> {
    let qw: HashSet<String> = q.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty())
        .map(String::from).collect();
    let mut out = vec![];
    for inp in pr.inputs.iter().filter(|i| i.ty == "number") {
        let m = inp.meaning.to_lowercase();
        for pre in ["(0 for ", "(0 if ", "(0 when ", "(zero for ", "(zero if "] {
            let Some(at) = m.find(pre) else { continue };
            let case = &m[at + pre.len()..];
            let case = &case[..case.find(')').unwrap_or(case.len())];
            let ws: Vec<&str> = case.split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() >= 3 && *w != "the").collect();
            if !ws.is_empty() && ws.iter().all(|w| qw.contains(*w)) {
                out.push(inp.symbol.clone());
            }
        }
    }
    out
}

/// What reading the question as one calculation found: readings, calculations that block the question, reasons.
#[derive(Default)]
struct Found {
    readings: Vec<Reading>,
    blockers: Vec<(String, f32)>,
    /// how strongly the question's words name each blocker's result (when it names one)
    blocker_ev: std::collections::HashMap<String, f32>,
    /// blockers set aside because they do not give the asked result at all (they do not block a two-step reading)
    unasked: Vec<String>,
    why: Vec<String>,
    /// calculations that take every value (each named by its words, one clear placement) and name the asked result,
    /// but miss one or two required inputs: the question to ask instead of refusing (recipe A24)
    near: Vec<Near>,
}

#[derive(Clone, Debug)]
struct Near {
    key: String,
    asked: String,
    evidence: f32,
    texts: Vec<(String, String)>,
    missing: Vec<String>,
}

/// The pack's words that set an input of a calculation to zero (`links.json` "zero", reviewed): "centreline" is y = 0,
/// "ground level" is z = 0. A whole word or phrase of the question must be one of them.
fn zero_by_pack(p: &Pack, key: &str, sym: &str, q: &str) -> bool {
    let norm = |t: &str| t.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty())
        .collect::<Vec<_>>().join(" ");
    let low = format!(" {} ", norm(q));
    p.zero_words.iter().filter(|z| z.0 == key && z.1 == sym).flat_map(|z| z.2.iter())
        .any(|w| !norm(w).is_empty() && low.contains(&format!(" {} ", norm(w))))
}

/// Read the question as the calculation `pr`: a real one, or for a chained reading the final calculation with its
/// first steps' inputs in place of the inputs they give (`chain`).
/// The conditions a calculation's assumptions state as a value with a unit ("25 degC: Kw = 1e-14"), as (number, unit).
fn assumed_values(pr: &Procedure) -> Vec<(f64, String)> {
    use fancy_regex::Regex;
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(?<![\w.])(\d+(?:\.\d+)?)\s*(degC|°C|K|atm)\b").expect("assumed value"));
    pr.assumptions.iter()
        .flat_map(|a| re.captures_iter(a).flatten()
            .filter_map(|c| Some((c[1].parse::<f64>().ok()?, c[2].replace("°C", "degC"))))
            .collect::<Vec<_>>())
        .collect()
}

fn consider(p: &Pack, q: &str, ms: &[Mention], pr: &Procedure, chain: Option<&Chain>, f: &mut Found) {
    // a value the question states that is a condition the calculation itself assumes ("... at 25 degrees Celsius"
    // for pH + pOH = 14) is that condition, not an input (exam set 2026-11-11 Q1); only for this calculation
    let assumed = assumed_values(pr);
    let kept: Vec<Mention>;
    let ms: &[Mention] = if assumed.is_empty() || chain.is_some() {
        ms
    } else {
        kept = ms.iter().filter(|m| {
            let unit = m.unit.replace("°C", "degC");
            let x = numbers(&m.text).first().and_then(|(s, e)| slice(&m.text, *s, *e).replace(',', "").parse::<f64>().ok());
            !x.is_some_and(|x| assumed.iter().any(|(v, u)| (x - v).abs() < 1e-9 && *u == unit))
        }).cloned().collect();
        if kept.len() < ms.len() {
            dbg_why(&pr.key, &format!("{} value(s) are conditions it assumes", ms.len() - kept.len()));
        }
        &kept
    };
    if ms.is_empty() {
        return;
    }
    let label = match chain {
        Some(c) => format!("{} via {}", pr.key, c.steps.iter().map(|s| s.key.as_str()).collect::<Vec<_>>().join(" + ")),
        None => pr.key.clone(),
    };
    if pr.inputs.iter().any(|i| i.required && !matches!(i.ty.as_str(), "number" | "integer" | "vector" | "choice" | "text")) {
        return;
    }
    let numeric = pr.inputs.iter().filter(|i| matches!(i.ty.as_str(), "number" | "integer" | "vector")).count();
    if numeric < ms.len() {
        return;
    }
    let pl = placements(p, pr, ms, asked_head(q));
    if pl.is_empty() {
        dbg_why(&label, "the values do not fit its inputs");
        return;
    }
    // a calculation that takes every value and has a value for each of its numeric inputs, set aside below only for
    // what the wording leaves unclear (which result, which value, a choice), still blocks a reading whose values
    // its words name less well: the ESP whose "efficiency or penetration" was unclear must not leave the question to
    // the settling basin with the same formula (spent held-out 2026-11-10c c012)
    let numeric_full = pl.iter().find(|x| pr.inputs.iter().enumerate()
        .all(|(i, inp)| !inp.required || !matches!(inp.ty.as_str(), "number" | "integer" | "vector") || x.1.contains(&i)))
        .map(|x| x.0);
    // the words labelling each value of the best placement that name the input it is placed in
    // (the values themselves too: "1.029atm" names no result "at 0 degC and 1 atm")
    let mut given_words: HashSet<usize> = toks(q).iter().filter(|t| ms.iter().any(|m| t.s < m.e && m.s < t.e))
        .map(|t| t.s).collect();
    for (k, &i) in pl[0].1.iter().enumerate() {
        let inp = &pr.inputs[i];
        let mut iw = mwords(&inp.meaning);
        iw.extend(mwords(&inp.symbol.replace('_', " ")));
        for (t, at) in ms[k].left.iter().map(|x| &x.0).chain(ms[k].right.iter()).zip(&ms[k].at) {
            if qwords(t).iter().any(|w| iw.iter().any(|x| same_word(w, x))) {
                given_words.insert(*at);
            }
        }
    }
    let (asked, asked_ev) = match asked_output(p, pr, q, &given_words) {
        Ok(a) => a,
        Err(w) => {
            dbg_why(&label, &w);
            if let Some(s) = numeric_full {
                f.blockers.push((pr.key.clone(), s));
                if w == NOT_ASKED {
                    f.unasked.push(pr.key.clone());
                }
            }
            return;
        }
    };
    // choices and formulas by rule
    let words = match rule_values(pr, q, Some(&asked), &[]) {
        Ok(v) => v,
        Err(w) => {
            dbg_why(&label, &w);
            if let Some(s) = numeric_full {
                f.blockers.push((pr.key.clone(), s));
            }
            return;
        }
    };
    // where each input of the combined calculation comes from: (calculation, its own symbol), and which part it is
    // (0 = the final calculation, 1.. = a first step)
    let n2 = chain.map(|c| c.n2).unwrap_or(pr.inputs.len());
    let origin = |j: usize| -> (String, String, usize) {
        if j < n2 {
            return (pr.key.clone(), pr.inputs[j].symbol.clone(), 0);
        }
        chain.and_then(|c| c.steps.iter().enumerate().find_map(|(si, st)| {
            st.map.iter().find(|m| m.0 == j).map(|m| (st.key.clone(), m.1.clone(), si + 1))
        })).unwrap_or_else(|| (pr.key.clone(), pr.inputs[j].symbol.clone(), 0))
    };
    // inputs the wording sets to zero: the contract's own "(0 for fresh water)", or the pack's zero words
    let zsyms = zero_by_wording(pr, q);
    let zeros: HashSet<usize> = (0..pr.inputs.len()).filter(|&j| {
        let (key, sym, _) = origin(j);
        zsyms.contains(&pr.inputs[j].symbol) || zero_by_pack(p, &key, &sym, q)
    }).collect();
    let has = |assign: &[usize], i: usize| -> bool {
        assign.contains(&i) || words.iter().any(|w| w.0 == pr.inputs[i].symbol) || zeros.contains(&i)
    };
    let fallback_of = |i: usize| chain.and_then(|c| c.steps.iter().find_map(|st| st.fallback.iter()
        .find_map(|x| if x.0 == i { Some(x.1) } else if x.1 == i { Some(x.0) } else { None })));
    let complete = |assign: &[usize]| -> bool {
        pr.inputs.iter().enumerate().all(|(i, inp)| {
            !inp.required || has(assign, i) || fallback_of(i).is_some_and(|j| has(assign, j))
        })
    };
    // a value in the sentence that asks belongs with the input the asked result is stated at ("what's the wind
    // speed up AT 120 M": u2 is "wind speed at height z2", so 120 m is z2, not the anemometer height z1)
    let (qa0, qb0) = asked_head(q);
    let ameaning = pr.outputs.iter().filter(|o| asked.starts_with(o.symbol.as_str())).map(|o| o.meaning.clone())
        .collect::<Vec<_>>().join(" ");
    let linked = |i: usize| ameaning.split(|c: char| !(c.is_alphanumeric() || c == '_')).any(|w| w == pr.inputs[i].symbol);
    // one value for two inputs of the same quantity sits in the earlier one (the other takes it): placing it in either
    // is one reading, not two rivals
    let canonical = |assign: &[usize]| -> bool {
        chain.is_none_or(|c| c.steps.iter().all(|st| st.fallback.iter()
            .all(|&(own, j)| !(assign.contains(&own) && !assign.contains(&j)))))
    };
    let mut full: Vec<(f32, Vec<usize>)> = pl.iter().filter(|x| complete(&x.1) && canonical(&x.1)).map(|x| {
        let bonus: f32 = x.1.iter().enumerate()
            .filter(|(k, &i)| ms[*k].s >= qa0 && ms[*k].e <= qb0 && linked(i)).count() as f32;
        (x.0 + bonus, x.1.clone())
    }).collect();
    full.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    // the networks' value tags settle a tie the words leave open (recipe A26): a placement wins against another when,
    // on every value the two place differently, the tags give its input at least 0.6 and twice the other's; among the
    // placements within the margin of the best, one that wins against all the others goes first
    let tag_wins = |a: &[usize], b: &[usize]| -> bool {
        let diff: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).collect();
        !diff.is_empty() && diff.iter().all(|&k| {
            let (sa, sb) = (&pr.inputs[a[k]].symbol, &pr.inputs[b[k]].symbol);
            let (pa, pb) = (tag_p(&ms[k], sa), tag_p(&ms[k], sb));
            sa != sb && pa >= 0.6 && pa >= 2.0 * pb
        })
    };
    if let Some(top) = full.first().map(|x| x.0) {
        let close: Vec<usize> = (0..full.len()).filter(|&j| full[j].0 > top - RIVAL_MARGIN).take(8).collect();
        if close.len() > 1 {
            if let Some(&w) = close.iter().find(|&&c| close.iter().all(|&o| o == c || tag_wins(&full[c].1, &full[o].1))) {
                if w != 0 {
                    dbg_why(&label, &format!("the networks' tags choose placement {:?} over {:?}", full[w].1, full[0].1));
                    let x = full.remove(w);
                    full.insert(0, x);
                }
            }
        }
    }
    let Some(best) = full.first() else {
        if std::env::var("AX_DERIVE_DEBUG").is_ok_and(|v| v == "1") {
            let missing: Vec<&str> = pr.inputs.iter().enumerate()
                .filter(|(i, inp)| inp.required && !has(&pl[0].1, *i) && !fallback_of(*i).is_some_and(|j| has(&pl[0].1, j)))
                .map(|(_, inp)| inp.symbol.as_str()).collect();
            dbg_why(&label, &format!("a required input has no value ({})", missing.join(", ")));
        }
        if let Some(s) = numeric_full {
            f.blockers.push((pr.key.clone(), s));
        }
        // a one-step calculation that takes every value, each named by its own words in one clear placement, and names
        // the asked result, missing only one or two inputs: the question to ask for them (recipe A24)
        // (strictly the best placement: a question asked shows no number, and it lists the values as read)
        let clear = pl.len() == 1 || pl[1].0 < pl[0].0;
        let named_all = pl[0].1.iter().enumerate().all(|(k, &i)| place_score(&ms[k], &pr.inputs[i]) > 0.0);
        let missing: Vec<String> = pr.inputs.iter().enumerate()
            .filter(|(i, inp)| inp.required && !has(&pl[0].1, *i))
            .map(|(_, inp)| inp.symbol.clone()).collect();
        // never ask for a choice the question writes one of ("..., class b, rural": the choice was there, only not
        // read), and the question must name the calculation by a word of its name ("my bike wheel is 700 mm across,
        // how far does it go" names nothing of the weir loading rate)
        let qlow = format!(" {} ", q.to_lowercase().split(|c: char| !c.is_alphanumeric()).collect::<Vec<_>>().join(" "));
        let choice_written = missing.iter().any(|s| pr.inputs.iter().find(|i| &i.symbol == s)
            .is_some_and(|i| i.choices.iter().any(|c| qlow.contains(&format!(" {} ", c.to_lowercase().replace('_', " "))))));
        let qw = mwords(q);
        let names_it = mwords(&pr.name).iter().any(|w| w.len() >= 4 && qw.iter().any(|x| same_word(x, w)));
        if chain.is_none() {
            dbg_why(&label, &format!("ask? clear {clear} ({:?}) named {named_all} missing {missing:?} asked {asked_ev} \
                                      choice written {choice_written} names it {names_it}",
                                     pl.iter().take(2).map(|x| x.0).collect::<Vec<_>>()));
        }
        if chain.is_none() && clear && named_all && (1..=2).contains(&missing.len()) && asked_ev >= 1.0 && !choice_written
            && names_it {
            let mut texts: Vec<(String, String)> = pl[0].1.iter().enumerate()
                .map(|(k, &i)| (pr.inputs[i].symbol.clone(), ms[k].text.clone())).collect();
            texts.extend(words.iter().cloned());
            f.near.push(Near { key: pr.key.clone(), asked: asked.clone(), evidence: asked_ev + pl[0].0, texts, missing });
        }
        return;
    };
    // a chained reading needs words for every value (the combined calculation has many inputs, so a value placed only
    // by its unit is a guess) and at least one value for each first step
    if let Some(c) = chain {
        // (by elimination it may: "400 K and ... air at 290 K": the air's value named, the other temperature is the gas's
        // (the swapped placement reads worse by the margin, checked below); or it is the only input that can take the
        // value at all: "1 atm" is the pressure)
        // (a value written where the question asks, in the input the asked result is stated at, is named by the
        // question itself: "how much BOD is exerted AFTER 10 DAYS" names the time of the final step, and the other
        // time, "160 mg/L after 5 days", is then the first step's by elimination)
        let named = |k: usize, i: usize| place_score(&ms[k], &pr.inputs[i]) > 0.0
            || pr.inputs.iter().filter(|x| fits(p, &ms[k], x)).count() == 1
            || (ms[k].s >= qa0 && ms[k].e <= qb0 && linked(i))
            || tag_p(&ms[k], &pr.inputs[i].symbol) >= 0.8;
        if let Some((k, &i)) = best.1.iter().enumerate().find(|(k, &i)| !named(*k, i)
            && !best.1.iter().enumerate().any(|(k2, &i2)| k2 != *k && ms[k2].dims == ms[*k].dims && named(k2, i2))) {
            dbg_why(&label, &format!("the value {} is placed in {} by its unit only", ms[k].text, pr.inputs[i].symbol));
            return;
        }
        if let Some(st) = c.steps.iter().find(|st| !st.map.iter().any(|m| best.1.contains(&m.0))) {
            dbg_why(&label, &format!("no value for the first step {}", st.key));
            return;
        }
    }
    // a value described as one of the calculation's RESULTS is not an input ("the ultimate BOD is 280 mg/L ... how
    // much is left" is not the ultimate-BOD calculation with 280 as the BOD exerted; held-out 2026-11-10c c016)
    // (only for a value outside the sentence that asks: "what's the concentration after 4 hours?" describes the
    // result asked for, not the 4 hours); in a chained reading a first step's results too, for the values placed in that
    // step's inputs ("the river flows at 0.3 m/s" is not the mixing calculation's combined flow, held-out 2026-11-10e)
    let (qa, qb) = asked_span(q);
    let mut outs: Vec<(usize, (String, Vec<String>, HashSet<String>))> = groups(pr).into_iter().map(|g| (0, g)).collect();
    for (si, st) in chain.map(|c| c.steps.as_slice()).unwrap_or(&[]).iter().enumerate() {
        if let Some(p1) = p.procedure(&st.key) {
            outs.extend(groups(p1).into_iter().map(|g| (si + 1, g)));
        }
    }
    if let Some(k) = best.1.iter().enumerate().find_map(|(k, &i)| {
        if ms[k].s >= qa && ms[k].e <= qb {
            return None;
        }
        let inp = &pr.inputs[i];
        let mut iw = mwords(&inp.meaning);
        iw.extend(mwords(&inp.symbol.replace('_', " ")));
        let part = origin(i).2;
        outs.iter().filter(|g| g.0 == 0 || g.0 == part).map(|g| &g.1).find_map(|(base, _, ow)| {
            // a pure number (ppm, %) cannot be a result of a physical kind: "a CO concentration of 8.0 ppm ... convert to
            // micrograms per cubic meter" gives the ppm, not the mass concentration (exam set 2026-11-11)
            // (a result that restates another in a unit counts as that one: C_ugm3 is the mass concentration C)
            let phys = |s: &str| pr.outputs.iter().find(|o| o.symbol == s).and_then(|o| p.kinds.get(&o.kind))
                .is_some_and(|kd| kd.dim != Dim::one());
            let physical = phys(base) || pr.outputs.iter().any(|o| base.starts_with(&format!("{}_", o.symbol)) && phys(&o.symbol));
            let pure = ms[k].dimless || ms[k].pct || ["ppm", "ppb", "ppmv", "ppbv"].contains(&ms[k].unit.as_str())
                || (!ms[k].dims.is_empty() && ms[k].dims.iter().all(|d| *d == Dim::one()));
            if pure && physical {
                return None;
            }
            let only_out: HashSet<String> = ow.difference(&iw).cloned().collect();
            let only_in: HashSet<String> = iw.difference(ow).cloned().collect();
            let o = words_score(&ms[k], &only_out);
            (o >= 0.5 && o > words_score(&ms[k], &only_in)).then_some(k)
        })
    }) {
        dbg_why(&label, &format!("the value {} is described as a result (unit {:?} dimless {} dims {})", ms[k].text,
                                 ms[k].unit, ms[k].dimless, ms[k].dims.len()));
        return;
    }
    // the values of a placement for the final calculation and for each first step
    let build = |assign: &[usize]| -> Option<(Given, Vec<Given>)> {
        let mut val: Vec<Option<(Input, String)>> = vec![None; pr.inputs.len()];
        for (k, &i) in assign.iter().enumerate() {
            val[i] = Some((input_value(&pr.inputs[i], &ms[k].text).ok()?, ms[k].text.clone()));
        }
        // a choice goes to every input that takes it (the stability class of the plume rise and of the dispersion)
        for (s, w) in &words {
            for (j, inp) in pr.inputs.iter().enumerate() {
                if &inp.symbol == s && val[j].is_none() {
                    val[j] = Some((Input::Text(w.clone()), w.clone()));
                }
            }
        }
        // an input set to zero by the wording, when no value is placed in it
        for &j in &zeros {
            if val[j].is_none() {
                val[j] = Some((Input::Text("0".into()), "0 (by the wording)".into()));
            }
        }
        // an input with no value of its own takes the value of the same quantity in the other step, either way
        for st in chain.map(|c| c.steps.as_slice()).unwrap_or(&[]) {
            for &(own, j) in &st.fallback {
                if val[own].is_none() && val[j].is_some() {
                    val[own] = val[j].clone();
                } else if val[j].is_none() && val[own].is_some() {
                    val[j] = val[own].clone();
                }
            }
        }
        let mut given: Given = (vec![], vec![]);
        for j in 0..n2 {
            if let Some((v, t)) = &val[j] {
                given.0.push((pr.inputs[j].symbol.clone(), v.clone()));
                given.1.push((pr.inputs[j].symbol.clone(), t.clone()));
            }
        }
        let mut firsts = vec![];
        for st in chain.map(|c| c.steps.as_slice()).unwrap_or(&[]) {
            let mut g: Given = (vec![], vec![]);
            for (j, sym) in &st.map {
                if let Some((v, t)) = &val[*j] {
                    g.0.push((sym.clone(), v.clone()));
                    g.1.push((sym.clone(), t.clone()));
                }
            }
            firsts.push(g);
        }
        Some((given, firsts))
    };
    // a word next to a value that names the best placement's input and NOT the rival's (while no word names the
    // rival's and not the best's) decides it: "hydroxide ion concentration of 1.0e-5 mol/L" is [OH-], not [H+], although
    // "ion concentration" names both (exam set Q1, 2026-11-11)
    // ... over the values the two placements put differently: at least one such word for the best, none anywhere for
    // the rival ("final managed density of 600 kg/m^3" is the compacted density, so the other value is the loose one;
    // exam set Q33)
    let distinct_wins = |a: &[usize], b: &[usize]| -> bool {
        let diff: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).collect();
        let ev: Vec<(bool, bool)> = diff.iter().map(|&k| {
            let mut wa = mwords(&pr.inputs[a[k]].meaning);
            wa.extend(mwords(&pr.inputs[a[k]].symbol.replace('_', " ")));
            let mut wb = mwords(&pr.inputs[b[k]].meaning);
            wb.extend(mwords(&pr.inputs[b[k]].symbol.replace('_', " ")));
            let only_a: HashSet<String> = wa.difference(&wb).cloned().collect();
            let only_b: HashSet<String> = wb.difference(&wa).cloned().collect();
            let near: Vec<String> = ms[k].left.iter().map(|x| x.0.clone()).chain(ms[k].right.iter().cloned())
                .flat_map(|t| qwords(&t)).collect();
            (near.iter().any(|w| only_a.iter().any(|x| same_word(w, x))),
             near.iter().any(|w| only_b.iter().any(|x| same_word(w, x))))
        }).collect();
        !diff.is_empty() && ev.iter().any(|e| e.0) && !ev.iter().any(|e| e.1)
    };
    // rival placements as good as the best (within the margin) must give the same result
    let rivals: Vec<&(f32, Vec<usize>)> = full.iter().skip(1)
        .filter(|x| x.0 > best.0 - RIVAL_MARGIN && !tag_wins(&best.1, &x.1) && !distinct_wins(&best.1, &x.1)).collect();
    let Some((given, firsts)) = build(&best.1) else { return };
    if !rivals.is_empty() {
        let (_, a) = solve_reading(p, &pr.key, chain, &given.0, &firsts, &asked);
        let va = if a.solved() { asked_value(&a, &asked) } else { None };
        let mut same = va.is_some();
        for r in rivals.iter().take(6) {
            let Some((g2, f2)) = build(&r.1) else { same = false; break };
            let (_, b) = solve_reading(p, &pr.key, chain, &g2.0, &f2, &asked);
            let vb = if b.solved() { asked_value(&b, &asked) } else { None };
            same &= matches!((&va, &vb), (Some(x), Some(y)) if agree(x, y));
        }
        if !same || rivals.len() > 6 {
            f.why.push(format!("{}: could not tell which value is which", pr.key));
            f.blockers.push((pr.key.clone(), best.0));
            f.blocker_ev.insert(pr.key.clone(), asked_ev);
            dbg_why(&label, &format!("rival placements {:?} vs {:?}", best, rivals.iter().take(3).collect::<Vec<_>>()));
            return;
        }
    }
    f.readings.push(Reading { key: pr.key.clone(), asked, asked_ev, place: best.0, given: given.0, texts: given.1,
                              chain: chain.cloned(), firsts });
}

/// Every link of the pack: (first calculation, its output, final calculation, the input it fills), as indices into
/// `p.procedures` and the final calculation's inputs: the reviewed `links.json` (same kind checked again) and the
/// results whose meaning says the input's quantity in the same words (`links`). Computed once per model.
fn link_list(p: &Pack) -> std::sync::Arc<Vec<(usize, String, usize, usize)>> {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<Vec<(usize, String, usize, usize)>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(l) = cache.lock().ok().and_then(|c| c.get(&p.model).cloned()) {
        return l;
    }
    let mut out = vec![];
    for (i2, p2) in p.procedures.iter().enumerate() {
        for (si, slot) in p2.inputs.iter().enumerate() {
            if !slot.required || slot.ty != "number" {
                continue;
            }
            for (i1, p1) in p.procedures.iter().enumerate().filter(|x| x.0 != i2) {
                for o in &p1.outputs {
                    let listed = o.kind == slot.kind
                        && p.links.iter().any(|l| l.0 == p1.key && l.1 == o.symbol && l.2 == p2.key && l.3 == slot.symbol);
                    if listed || links(o, slot) {
                        out.push((i1, o.symbol.clone(), i2, si));
                    }
                }
            }
        }
    }
    let l = Arc::new(out);
    if let Ok(mut c) = cache.lock() {
        c.insert(p.model.clone(), l.clone());
    }
    l
}

/// The final calculation `p2` with the inputs `picks` (first calculation, its output, input of p2) given by first steps:
/// the combined calculation and the chain. An input of a first step that is one already in the combined calculation
/// (`shared_input`: the wind speed of a plume rise and of the dispersion) takes the same value.
fn combine(p: &Pack, p2: &Procedure, picks: &[&(usize, String, usize, usize)]) -> (Procedure, Chain) {
    let slots: Vec<usize> = picks.iter().map(|x| x.3).collect();
    let mut v = p2.clone();
    v.inputs = p2.inputs.iter().enumerate().filter(|(i, _)| !slots.contains(i)).map(|x| x.1.clone()).collect();
    let n2 = v.inputs.len();
    let mut steps: Vec<Step> = vec![];
    for (i1, out, _, si) in picks.iter().map(|x| (x.0, &x.1, x.2, x.3)) {
        let p1 = &p.procedures[i1];
        let pass = (out.clone(), p2.inputs[si].symbol.clone());
        if let Some(st) = steps.iter_mut().find(|s| s.key == p1.key) {
            st.outs.push(pass);
            continue;
        }
        let mut map: Vec<(usize, String)> = vec![];
        let mut fallback = vec![];
        let before = v.inputs.len();
        for inp in &p1.inputs {
            let partner = (0..before).find(|&j| shared_input(&v.inputs[j], inp));
            v.inputs.push(inp.clone());
            let own = v.inputs.len() - 1;
            map.push((own, inp.symbol.clone()));
            if let Some(j) = partner {
                fallback.push((own, j));
            }
        }
        steps.push(Step { key: p1.key.clone(), outs: vec![pass], map, fallback });
    }
    (v, Chain { steps, n2 })
}

/// Every chained reading: for each calculation, one required input no value gives (or, when none fits that way, two)
/// is worked out by a calculation whose result is that quantity (`link_list`).
fn chains(p: &Pack, q: &str, ms: &[Mention]) -> Found {
    let all = link_list(p);
    let mut f = Found::default();
    for (i2, p2) in p.procedures.iter().enumerate() {
        for l in all.iter().filter(|l| l.2 == i2) {
            let (v, c) = combine(p, p2, &[l]);
            consider(p, q, ms, &v, Some(&c), &mut f);
        }
    }
    if !f.readings.is_empty() || !f.blockers.is_empty() {
        return f;
    }
    for (i2, p2) in p.procedures.iter().enumerate() {
        let ls: Vec<&(usize, String, usize, usize)> = all.iter().filter(|l| l.2 == i2).collect();
        for (a, la) in ls.iter().enumerate() {
            for lb in ls.iter().skip(a + 1).filter(|lb| lb.3 != la.3) {
                let (v, c) = combine(p, p2, &[la, lb]);
                consider(p, q, ms, &v, Some(&c), &mut f);
            }
        }
    }
    if !f.readings.is_empty() || !f.blockers.is_empty() {
        return f;
    }
    // three inputs from at most two first steps (one giving two: the dispersion coefficients sigma y and sigma z, the
    // other the effective stack height)
    for (i2, p2) in p.procedures.iter().enumerate() {
        let ls: Vec<&(usize, String, usize, usize)> = all.iter().filter(|l| l.2 == i2).collect();
        for (a, la) in ls.iter().enumerate() {
            for (b, lb) in ls.iter().enumerate().skip(a + 1).filter(|(_, lb)| lb.3 != la.3) {
                for lc in ls.iter().skip(b + 1).filter(|lc| lc.3 != la.3 && lc.3 != lb.3) {
                    let mut ps = vec![la.0, lb.0, lc.0];
                    ps.sort();
                    ps.dedup();
                    if ps.len() <= 2 {
                        let (v, c) = combine(p, p2, &[la, lb, lc]);
                        consider(p, q, ms, &v, Some(&c), &mut f);
                    }
                }
            }
        }
    }
    f
}

/// "24 degrees C", "150 degrees Celsius", "deg F" written as "24 °C", "150 °C", "°F", padded with spaces to the same
/// length so every position in the question stays the same (held-out 2026-11-10e: "24 degrees C" was no temperature).
fn degree_signs(q: &str) -> String {
    let re = fancy_regex::Regex::new(r"(?i)\bdeg(?:rees?|s|\.)?\s*(celsius|fahrenheit|c|f)\b").expect("degrees");
    let mut out = String::new();
    let mut last = 0;
    for m in re.captures_iter(q).flatten() {
        let all = m.get(0).expect("match");
        let unit = if m.get(1).is_some_and(|u| u.as_str().to_lowercase().starts_with('c')) { "°C" } else { "°F" };
        out.push_str(&q[last..all.start()]);
        out.push_str(unit);
        out.push_str(&" ".repeat(all.as_str().chars().count().saturating_sub(2)));
        last = all.end();
    }
    out.push_str(&q[last..]);
    out
}

fn input_json(i: &Input) -> Value {
    match i {
        Input::Text(t) => json!(t),
        Input::Int(n) => json!(n),
        Input::Float(x) => json!(x),
        Input::Json(j) => j.clone(),
    }
}

/// The question as a calculation worked out by rule: Ok(reply) when one reading fits (solved, or refused by the
/// engine with its reason), Err(why) when none does or the readings disagree.
/// The networks' value tags for the question being read (recipe A26; `V2::tag_hints`), set by `derive_with` for the
/// length of one question.
type Hints = Vec<(usize, usize, std::collections::HashMap<String, f32>)>;
thread_local! {
    static HINTS: std::cell::RefCell<Hints> = const { std::cell::RefCell::new(Vec::new()) };
}

/// The networks' probability that the value `m` is the input symbol `sym` (0 when there are no tags).
fn tag_p(m: &Mention, sym: &str) -> f32 {
    HINTS.with(|h| h.borrow().iter().find(|(s, e, _)| *s < m.e && m.s < *e).and_then(|x| x.2.get(sym).copied()).unwrap_or(0.0))
}

/// `derive` with the networks' value tags as a third reader: they break a tie between placements the words leave open
/// and name a value no word names; they never decide a calculation or the asked result.
pub fn derive_with(p: &Pack, q: &str, hints: Hints) -> Result<Value, String> {
    HINTS.with(|h| *h.borrow_mut() = hints);
    let r = derive(p, q);
    HINTS.with(|h| h.borrow_mut().clear());
    r
}

/// Known constants an exam question states as data ("Equivalent weight = 20.0 mg/meq", "Molar masses: C=12, H=1, O=16
/// g/mol", "R = 0.08206 L*atm/(mol*K)"): the engine has its own values. A stated value that agrees to its written
/// precision is CONFIRMED (read, needs no input); one that disagrees is refused (the question uses other data). The
/// compound's own molar mass is never one of these: it is an input of the calculations that take it. (2026-11-11, the
/// owner's exam set)
pub fn stated_constants(q: &str) -> Result<Vec<(usize, usize, String)>, String> {
    use fancy_regex::Regex;
    use std::sync::OnceLock;
    // (label pattern at the end of the text before the value, accepted values, what it is)
    static PATS: OnceLock<Vec<(Regex, Vec<f64>, &'static str)>> = OnceLock::new();
    let pats = PATS.get_or_init(|| {
        let r = |p: &str| Regex::new(&format!(r"(?i)(?:{p})\s*(?:=|:|is|of)?\s*$")).expect("constant label");
        let el = |sym: &str| Regex::new(&format!(r"(?:molar mass(?:es)?|atomic (?:weight|mass)(?:es|s)?)[^.;?]*\b{sym}\s*=\s*$")).expect("element");
        vec![
            (r(r"(?:universal\s+)?gas constant(?:\s*\(?R\)?)?|\bR"), vec![0.082057, 8.314462, 0.082057e-3 * 1e3], "gas constant R"),
            (r(r"\bKw|ion product of water"), vec![1.0e-14], "Kw"),
            (r(r"\bg|gravitational acceleration|acceleration (?:due to|of) gravity"), vec![9.80665], "g"),
            (el("C"), vec![12.011], "atomic mass of C"), (el("H"), vec![1.008], "atomic mass of H"),
            (el("O"), vec![15.999], "atomic mass of O"), (el("N"), vec![14.007], "atomic mass of N"),
            (el("S"), vec![32.06], "atomic mass of S"), (el("Cl"), vec![35.45], "atomic mass of Cl"),
            (el("Ca"), vec![40.078], "atomic mass of Ca"), (el("Mg"), vec![24.305], "atomic mass of Mg"),
            (el("Na"), vec![22.990], "atomic mass of Na"),
            (r(r"equivalent weight|eq\.?\s*wt\.?"), vec![20.04, 12.15, 50.04, 23.0, 61.02, 30.0, 17.01, 39.1, 35.45, 48.03],
             "equivalent weight"),
        ]
    });
    let qd = degree_signs(q);
    let chars: Vec<char> = qd.chars().collect();
    let mut out = vec![];
    for m in mentions(&qd) {
        let start = m.s.saturating_sub(60);
        let before: String = chars[start..m.s].iter().collect();
        // the label must sit in the same clause or bracket as the value
        let before = before.rsplit(['(', '.', ';', '?']).next().unwrap_or("").to_string();
        let Some((ns, ne)) = numbers(&m.text).first().copied() else { continue };
        let Ok(x) = slice(&m.text, ns, ne).replace(',', "").parse::<f64>() else { continue };
        for (re, vals, what) in pats {
            let Ok(Some(hit)) = re.find(&before) else { continue };
            // where the label starts (characters), so the label and the value can be blanked out of the question
            let label_s = m.s - before.chars().count() + before[..hit.start()].chars().count();
            // written precision: half a unit of the last digit written, at least 1 %
            let digits = slice(&m.text, ns, ne);
            let dec = digits.split_once('.').map(|(_, d)| d.chars().take_while(|c| c.is_ascii_digit()).count()).unwrap_or(0);
            let half = 0.5 * 10f64.powi(-(dec as i32));
            let Some(v) = vals.iter().find(|v| (x - **v).abs() <= half.max(0.01 * v.abs())) else {
                return Err(format!("the {what} stated ({}) is not the one the calculations use", m.text.trim()));
            };
            out.push((label_s, m.e, format!("{what}: {} stated, {v} used (agrees to the precision written)", m.text.trim())));
            break;
        }
    }
    Ok(out)
}

/// The question with its confirmed constants (label and value) blanked out, character positions kept.
pub fn without_constants(q: &str, known: &[(usize, usize, String)]) -> String {
    q.chars().enumerate().map(|(i, c)| if known.iter().any(|k| k.0 <= i && i < k.1) { ' ' } else { c }).collect()
}

pub fn derive(p: &Pack, q: &str) -> Result<Value, String> {
    // known constants stated as data are confirmed and taken out of the question (or the question is refused)
    let known = stated_constants(q)?;
    if known.is_empty() {
        return derive_text(p, q);
    }
    dbg_why("constants", &format!("{:?}", known.iter().map(|k| &k.2).collect::<Vec<_>>()));
    let mut v = derive_text(p, &without_constants(q, &known))?;
    v["stated_constants"] = json!(known.iter().map(|k| k.2.clone()).collect::<Vec<_>>());
    Ok(v)
}

fn derive_text(p: &Pack, q: &str) -> Result<Value, String> {
    // (an underscore inside a written symbol is read as a space: "sigma_y 110 m" names sigma y, held-out 2026-11-10e)
    let ms = mentions(&degree_signs(q).replace('_', " "));
    if ms.is_empty() {
        return Err("no values in the question".into());
    }
    if ms.len() > 12 {
        return Err("too many values".into());
    }
    // every free-standing number must be a value read
    for (s, e) in numbers(q) {
        if !ms.iter().any(|m| m.s <= s && e <= m.e) {
            return Err(format!("the number {} was not read", slice(q, s, e)));
        }
    }
    let mut f = Found::default();
    for pr in &p.procedures {
        consider(p, q, &ms, pr, None, &mut f);
    }
    // chained readings only when no single calculation takes the values (nor is set aside for unclear wording), and
    // never when AX_DERIVE_CHAIN=off (a calculation that takes the values but gives nothing asked does not stop them)
    let strong: Vec<(String, f32)> = f.blockers.iter().filter(|b| !f.unasked.contains(&b.0)).cloned().collect();
    if f.readings.is_empty() && !strong.is_empty() {
        dbg_why("chains", &format!("not tried: {strong:?} take the values"));
    }
    if f.readings.is_empty() && strong.is_empty() && !std::env::var("AX_DERIVE_CHAIN").is_ok_and(|v| v == "off") {
        let mut c = chains(p, q, &ms);
        if !c.readings.is_empty() || !c.blockers.is_empty() {
            // (a one-step calculation that only lacks an input stays the question to ask)
            c.near.extend(f.near.drain(..));
            f = c;
        }
    }
    let Found { mut readings, blockers, blocker_ev, why, near, unasked } = f;
    if readings.is_empty() {
        // one clear calculation that only lacks an input or two: ask for them (recipe A24) instead of refusing; never
        // while another calculation takes every value and was set aside only for unclear wording (it may be the one
        // meant: "how far downwind is the maximum ..." asked for an input of the sigma-ratio method while the Briggs
        // maximum, the calculation meant, was set aside; spent held-out 2026-11-09b c006)
        let mut near = near;
        near.sort_by(|a, b| b.evidence.partial_cmp(&a.evidence).unwrap_or(std::cmp::Ordering::Equal));
        let other_takes_all = |key: &str| blockers.iter().any(|b| b.0 != key && !unasked.contains(&b.0));
        if let Some(n) = near.first().filter(|n| near.get(1).is_none_or(|m| m.evidence <= n.evidence - PLACE_MARGIN)
            && !other_takes_all(&n.key)) {
            let pr = p.procedure(&n.key).expect("procedure");
            let what: Vec<String> = n.missing.iter().map(|s| {
                let m = pr.inputs.iter().find(|i| &i.symbol == s).map(|i| i.meaning.clone()).unwrap_or_default();
                if m.is_empty() { s.clone() } else { format!("{m} ({s})") }
            }).collect();
            let inputs: serde_json::Map<String, Value> = n.texts.iter().map(|(s, t)| (s.clone(), json!(t))).collect();
            dbg_why(&n.key, &format!("ASK for {:?}", n.missing));
            return Ok(json!({"outcome": "NEEDS_INPUT", "capability": n.key, "name": pr.name,
                             "message": format!("To work out {}, I also need: {}.", pr.name, what.join("; ")),
                             "needs": n.missing, "request": {"capability": n.key, "inputs": inputs, "target": n.asked},
                             "reader": "worked out by rule (asks for what the question does not give)"}));
        }
        return Err(why.first().cloned().unwrap_or_else(|| "no calculation takes these values".into()));
    }
    // a calculation the question's words name less well, by at least one word, is not what is asked: the words that
    // name the result and the words that name each value together ("surface overflow rate" is the overflow rate, not
    // the weir loading RATE; "drift velocity", "plates" make an ESP, not a settling basin of the same formula).
    // Readings within one word of the best must agree below
    let total = |r: &Reading| r.asked_ev + r.place;
    let top = readings.iter().map(total).fold(0.0f32, f32::max);
    for r in readings.iter().filter(|r| total(r) <= top - PLACE_MARGIN) {
        dbg_why(&r.key, &format!("named in fewer words ({} vs {top})", total(r)));
    }
    readings.retain(|r| total(r) > top - PLACE_MARGIN);
    let place_top = readings.iter().map(|r| r.place).fold(f32::MIN, f32::max);
    // a calculation none of whose results the question asks does not block a reading whose result the question NAMES
    // by its words: the landfill compaction ratio takes "60 mg/L ... 24 mg/L" (two densities) but "express the total
    // hardness" asks the hardness (exam set Q3, 2026-11-11, when the compaction ratio was added)
    let named_best = readings.iter().map(|r| r.asked_ev).fold(0.0f32, f32::max) >= 1.0;
    // ... nor does one the question names by at least a word less, with no better placement ("express the TOTAL
    // HARDNESS ... as CaCO3": the total alkalinity is named by "total" and "CaCO3" only; exam set Q3, batch 2)
    let best_ev = readings.iter().map(|r| r.asked_ev).fold(0.0f32, f32::max);
    let named_less = |b: &(String, f32)| blocker_ev.get(&b.0).is_some_and(|ev| *ev <= best_ev - 1.0) && b.1 <= place_top;
    if let Some(b) = blockers.iter().find(|b| b.1 > place_top - PLACE_MARGIN && !(named_best && unasked.contains(&b.0))
                                          && !named_less(b)) {
        return Err(format!("{} also takes these values, and the question does not make it clear", b.0));
    }
    for r in &readings {
        let via = r.chain.as_ref().map(|c| format!(" via {:?} {:?}", c.steps.iter().map(|s| &s.key).collect::<Vec<_>>(),
                                                   r.firsts.iter().map(|g| &g.1).collect::<Vec<_>>())).unwrap_or_default();
        dbg_why(&r.key, &format!("READING asked {} ({}), placement {}, values {:?}{via}", r.asked, r.asked_ev, r.place, r.texts));
    }
    // solve every reading; all that the engine solves must agree on the asked result
    let mut solved: Vec<(Reading, Vec<Answer>, Answer, Value)> = vec![];
    let mut refused: Vec<(Reading, Answer)> = vec![];
    for r in readings {
        let (a1s, a) = solve_reading(p, &r.key, r.chain.as_ref(), &r.given, &r.firsts, &r.asked);
        match (a.solved(), asked_value(&a, &r.asked)) {
            (true, Some(v)) => solved.push((r, a1s, a, v)),
            _ => refused.push((r, a)),
        }
    }
    let (r, a1s, a) = if let Some(first) = solved.first() {
        if solved.iter().any(|x| !agree(&x.3, &first.3)) {
            let mut ks: Vec<&str> = solved.iter().map(|x| x.0.key.as_str()).collect();
            ks.dedup();
            return Err(format!("several calculations fit and disagree: {}", ks.join(", ")));
        }
        if !refused.is_empty() {
            return Err(format!("several calculations fit: {} and {}", first.0.key, refused[0].0.key));
        }
        // agreeing readings: the one whose words fit best
        solved.sort_by(|a, b| (b.0.asked_ev + b.0.place).partial_cmp(&(a.0.asked_ev + a.0.place)).unwrap_or(std::cmp::Ordering::Equal));
        let (r, a1s, a, _) = solved.remove(0);
        (r, a1s, a)
    } else {
        // the engine refused every reading's values: a reading by rule is never shown on the engine's refusal (it may
        // be the wrong calculation of the same inputs: Stokes for a drag coefficient), so the question is not answered
        let (r, a) = &refused[0];
        return Err(format!("{}: the engine refused the values ({})", r.key, a.message.chars().take(100).collect::<String>()));
    };
    let pr = p.procedure(&r.key).expect("procedure");
    let mut v = serde_json::to_value(&a).unwrap_or(json!({}));
    let inputs: serde_json::Map<String, Value> = r.given.iter().map(|(s, i)| (s.clone(), input_json(i))).collect();
    v["reader"] = json!("worked out by rule");
    v["capability"] = json!(r.key);
    v["name"] = json!(pr.name);
    v["request"] = json!({"capability": r.key, "inputs": inputs, "target": r.asked});
    v["answer"] = if a.solved() { serde_json::to_value(&a).unwrap_or(Value::Null) } else { Value::Null };
    v["reading"] = json!({"derive": {"values": r.texts.iter().map(|(s, t)| json!({"symbol": s, "text": t})).collect::<Vec<_>>(),
                                      "asked": r.asked, "asked_words": r.asked_ev, "placement_words": r.place}});
    // a chained reading shows every step: which calculation, the values it took, the results passed on
    if let Some(c) = &r.chain {
        let mut steps = vec![];
        for ((st, g), a1) in c.steps.iter().zip(&r.firsts).zip(&a1s) {
            let p1 = p.procedure(&st.key).expect("procedure");
            let results: Vec<Value> = st.outs.iter().map(|(out, slot)| {
                let x = a1.values.iter().find(|x| &x.0 == out);
                json!({"symbol": out, "value": x.map(|x| x.1.clone()), "unit": x.map(|x| x.2.clone()), "passed_to": slot})
            }).collect();
            let ins: serde_json::Map<String, Value> = g.0.iter().map(|(s, i)| (s.clone(), input_json(i))).collect();
            steps.push(json!({"capability": st.key, "name": p1.name, "inputs": ins, "result": results[0].clone(),
                              "passed_to": st.outs[0].1, "results": results,
                              "values": g.1.iter().map(|(s, t)| json!({"symbol": s, "text": t})).collect::<Vec<_>>()}));
        }
        steps.push(json!({"capability": r.key, "name": pr.name, "inputs": v["request"]["inputs"].clone(), "target": r.asked}));
        v["reader"] = json!(format!("worked out by rule, in {} steps", steps.len()));
        v["steps"] = json!(steps);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_with_units_and_words() {
        let ms = mentions("An electrostatic precipitator has 4500 m2 of plates, drift velocity 0.12 m/s and handles 150 m3/s of flue gas.");
        assert_eq!(ms.len(), 3);
        assert_eq!(ms[0].unit, "m^2");
        assert!(ms[0].right.contains(&"plates".to_string()));
        assert!(ms[1].left.iter().any(|w| w.0 == "drift"));
        let ms = mentions("Operator is exposed to 88, 92, 97 dBA for 4, 2 and 1 hours. What is the TWA?");
        assert_eq!(ms.len(), 2);
        assert_eq!(ms[0].list, vec!["88", "92", "97"]);
        assert_eq!(ms[1].list, vec!["4", "2", "1"]);
        assert_eq!(ms[1].unit, "h");
        let ms = mentions("kd 0.28 1/d, ka 0.55 1/d");
        assert!(ms.iter().all(|m| m.list.is_empty()));
        let ms = mentions("k = 0.23 d-1 and a 5-day BOD of 180 mg/L");
        assert_eq!(ms[0].unit, "day^-1");
        assert!(!ms[0].dims.is_empty());
        assert_eq!(ms[1].unit, "day");
    }
}
