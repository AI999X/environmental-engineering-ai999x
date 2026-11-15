//! The 1.6 language layer: a small trained encoder (Ettin, ModernBERT architecture, run natively with candle) READS a
//! question; the engine answers it. It never produces a number: every value it hands on is text copied from the
//! question, and the engine converts, checks and computes. A reading it is not sure of becomes "I don't know that one."
//!
//! Order (docs/OVERHAUL_1_6_PLAN.md item 5): the documented-form reader (ax_engine::ask) first; when it does not solve
//! the question, the trained reader decides between
//!   compute  -> the engine solves the capability with the values read (or says, in words, which inputs are missing)
//!   explain  -> a stored explanation of the concept (beginner / intermediate / advanced), or what a calculation needs
//!   unknown  -> "I don't know that one."
//! Gates before the engine is called: confidence thresholds chosen on validation data (labels.json `thresholds`), one
//! span per input (a list input may have several), and every free-standing number of the question inside a span.

pub mod derive;
pub mod v2;

use ax_engine::engine::{solve, Input};
use ax_engine::Pack;
use candle_core::{DType, Device, Tensor, D};
use candle_nn::{linear, Linear, Module, VarBuilder};
use candle_transformers::models::modernbert::{Config, ModernBert};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

pub const DONT_KNOW: &str = "I don't know that one.";

/// Weight of the "question uses the concept's own name words" bonus (0 = off; chosen on validation questions).
const NAME_BONUS: f32 = 0.0;

/// The version-1 reader (one ModernBert encoder and four heads). A package with `v2/` does not need it: version 2
/// answers every question, so a v2 package may leave out model.safetensors and tokenizer.json (137 MB).
struct V1 {
    enc: ModernBert,
    intent: Linear,
    target: Linear,
    asked: Linear,
    tags: Linear,
    tok: tokenizers::Tokenizer,
}

pub struct Lang {
    v1: Option<V1>,
    dev: Device,
    pub labels: Value,
    explanations: Value,
    lex: std::sync::OnceLock<Lex>,
    /// reviewed short answers to everyday, safety, what-if and misconception questions (answers.json; may be empty)
    answers: Value,
    alex: std::sync::OnceLock<Lex>,
    /// the trained answer matcher (answers_model/), when the package has one
    matcher: Option<Matcher>,
    /// language layer version 2 (`<package>/v2/`): when present it answers every question
    pub v2: Option<v2::V2>,
}

/// A small encoder trained so that different ways of asking one question embed close together, plus the embeddings of
/// every stored wording (answers_model/index.safetensors, rows in index.json: [answer id, is the held-out wording]).
struct Matcher {
    enc: ModernBert,
    tok: tokenizers::Tokenizer,
    max_len: usize,
    emb: Tensor,
    rows: Vec<(String, bool)>,
    /// answers_model/verifier/: reads a question and a stored question together (cross-encoder)
    verifier: Option<Verifier>,
}

/// The answer verifier: "question [SEP] stored question" -> probability that both ask the same thing.
struct Verifier {
    enc: ModernBert,
    head: Linear,
    tok: tokenizers::Tokenizer,
    max_len: usize,
    /// how many of the matcher's best answers it reads (verifier.json "k"; 20 by default)
    k: usize,
}

impl Verifier {
    fn load(dir: &Path, dev: &Device) -> Result<Verifier, String> {
        let read = |n: &str| std::fs::read_to_string(dir.join(n)).map_err(|e| format!("{n}: {e}"));
        let cfg: Config = serde_json::from_str(&read("config.json")?).map_err(err)?;
        let meta: Value = serde_json::from_str(&read("verifier.json")?).map_err(err)?;
        let raw = candle_core::safetensors::load(dir.join("model.safetensors"), dev).map_err(err)?;
        let map: HashMap<String, Tensor> = raw
            .into_iter()
            .map(|(k, v)| (k.strip_prefix("encoder.").map(|r| format!("model.{r}")).unwrap_or(k), v))
            .collect();
        let vb = VarBuilder::from_tensors(map, DType::F32, dev);
        Ok(Verifier {
            enc: ModernBert::load(vb.clone(), &cfg).map_err(err)?,
            head: linear(cfg.hidden_size, 1, vb.pp("head")).map_err(err)?,
            tok: tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).map_err(err)?,
            max_len: meta["max_len"].as_u64().unwrap_or(96) as usize,
            k: meta["k"].as_u64().unwrap_or(20) as usize,
        })
    }

    fn prob(&self, a: &str, b: &str, dev: &Device) -> Result<f32, String> {
        let e = self.tok.encode((a, b), true).map_err(err)?;
        let mut ids: Vec<u32> = e.get_ids().to_vec();
        if ids.len() > self.max_len {
            let sep = *ids.last().unwrap();
            ids.truncate(self.max_len - 1);
            ids.push(sep);
        }
        let n = ids.len();
        let x = Tensor::from_vec(ids, (1, n), dev).map_err(err)?;
        let m = Tensor::ones((1, n), DType::U32, dev).map_err(err)?;
        let pooled = self.enc.forward(&x, &m).and_then(|h| h.mean(1)).map_err(err)?;
        let logit: f32 = self.head.forward(&pooled).and_then(|t| t.flatten_all()).and_then(|t| t.to_vec1::<f32>()).map_err(err)?[0];
        Ok(1.0 / (1.0 + (-logit).exp()))
    }
}

impl Matcher {
    fn load(dir: &Path, dev: &Device) -> Result<Matcher, String> {
        let read = |n: &str| std::fs::read_to_string(dir.join(n)).map_err(|e| format!("{n}: {e}"));
        let cfg: Config = serde_json::from_str(&read("config.json")?).map_err(err)?;
        let meta: Value = serde_json::from_str(&read("matcher.json")?).map_err(err)?;
        let raw = candle_core::safetensors::load(dir.join("model.safetensors"), dev).map_err(err)?;
        // saved by transformers' AutoModel: weights without a prefix; candle's ModernBert reads "model."
        let map: HashMap<String, Tensor> = raw
            .into_iter()
            .map(|(k, v)| (if k.starts_with("model.") { k } else { format!("model.{k}") }, v))
            .collect();
        let vb = VarBuilder::from_tensors(map, DType::F32, dev);
        let idx = candle_core::safetensors::load(dir.join("index.safetensors"), dev).map_err(err)?;
        let emb = idx.get("emb").ok_or("index.safetensors: no emb")?.to_dtype(DType::F32).map_err(err)?;
        let rows: Value = serde_json::from_str(&read("index.json")?).map_err(err)?;
        let rows: Vec<(String, bool)> = rows["rows"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|r| (r[0].as_str().unwrap_or("").to_string(), r[1].as_bool().unwrap_or(false)))
            .collect();
        Ok(Matcher {
            enc: ModernBert::load(vb, &cfg).map_err(err)?,
            tok: tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).map_err(err)?,
            max_len: meta["max_len"].as_u64().unwrap_or(64) as usize,
            emb,
            rows,
            verifier: if dir.join("verifier").is_dir() { Some(Verifier::load(&dir.join("verifier"), dev)?) } else { None },
        })
    }

    /// The unit-length mean-pooled embedding of one question.
    fn embed(&self, q: &str, dev: &Device) -> Result<Tensor, String> {
        let e = self.tok.encode(q, true).map_err(err)?;
        let mut ids: Vec<u32> = e.get_ids().to_vec();
        if ids.len() > self.max_len {
            let sep = *ids.last().unwrap();
            ids.truncate(self.max_len - 1);
            ids.push(sep);
        }
        let n = ids.len();
        let x = Tensor::from_vec(ids, (1, n), dev).map_err(err)?;
        let m = Tensor::ones((1, n), DType::U32, dev).map_err(err)?;
        let v = self.enc.forward(&x, &m).and_then(|h| h.mean(1)).map_err(err)?;
        let norm = v.sqr().and_then(|t| t.sum_keepdim(1)).and_then(|t| t.sqrt()).map_err(err)?;
        v.broadcast_div(&norm).map_err(err)
    }
}

/// Word-overlap search (TF-IDF, cosine) over every explanation target: a concept's name, aliases and learner
/// questions; a calculation's name, keywords and documented questions. Combined with the model's own probability when
/// a question asks for an explanation: distinctive words ("Monod", "backwash") then decide clearly.
struct Lex {
    idf: std::collections::HashMap<String, f32>,
    docs: Vec<(String, std::collections::HashMap<String, f32>)>,
}

const STOP: &[&str] = &["the", "a", "an", "of", "in", "on", "to", "for", "and", "or", "is", "are", "was", "be", "it", "its",
    "what", "why", "how", "does", "do", "did", "can", "could", "should", "would", "i", "me", "my", "we", "you", "your",
    "this", "that", "these", "those", "with", "from", "by", "as", "at", "about", "explain", "mean", "means", "meaning",
    "tell", "please", "pls", "plz", "u", "ur", "whats", "what's", "is", "difference", "between", "vs", "like", "im",
    "simple", "words", "work", "works", "used", "use", "when", "which", "there", "they", "them", "it's",
    "below", "above", "under", "over", "after", "before", "during", "along", "into", "onto", "out", "off", "near", "far",
    "down", "up", "between", "through", "across", "around", "per", "than", "then", "also", "very", "more", "less", "most",
    "some", "any", "all", "each", "other", "same", "much", "many", "get", "gets", "make", "makes", "need", "needs",
    "happen", "happens", "why", "matter", "matters", "important", "affect", "affects", "lose", "loses", "give", "gives",
    // words that ask for a survey, never a concept ("methods" once chose "Methods to estimate BOD rate constants" for
    // "Tell me some methods of water treatments", owner 2026-11-07)
    "method", "methods", "way", "ways", "type", "types", "kind", "kinds", "example", "examples", "different", "various",
    "common", "main", "basic", "basics", "overview", "describe", "list", "know", "learn", "show", "options", "option",
    "techniques", "technique", "approaches", "approach", "process", "processes"];

/// Words of a survey question about a whole topic or the model; they are left over after STOP and do not name a concept.
const SURVEY: &[&str] = &["topic", "topics", "model", "cover", "covers", "calculation", "calculations", "capability",
    "capabilities", "subject", "field", "area", "areas", "help", "everything", "anything"];

fn stem(w: &str) -> String {
    let n = w.chars().count();
    for suf in ["ing", "ed", "es", "s"] {
        if n > suf.len() + 3 && w.ends_with(suf) {
            return w[..w.len() - suf.len()].to_string();
        }
    }
    w.to_string()
}

fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '/')
        .filter(|w| w.chars().count() >= 2 && !STOP.contains(w))
        .map(stem)
        .collect()
}

impl Lex {
    fn build(docs: Vec<(String, String)>) -> Lex {
        use std::collections::HashMap;
        let mut df: HashMap<String, f32> = HashMap::new();
        let tfs: Vec<(String, HashMap<String, f32>)> = docs
            .into_iter()
            .map(|(k, text)| {
                let mut tf: HashMap<String, f32> = HashMap::new();
                for w in words(&text) {
                    *tf.entry(w).or_default() += 1.0;
                }
                for w in tf.keys() {
                    *df.entry(w.clone()).or_default() += 1.0;
                }
                (k, tf)
            })
            .collect();
        let n = tfs.len().max(1) as f32;
        let idf: HashMap<String, f32> = df.into_iter().map(|(w, d)| (w, (n / d).ln() + 1.0)).collect();
        let docs = tfs
            .into_iter()
            .map(|(k, tf)| {
                let mut v: HashMap<String, f32> = tf.into_iter().map(|(w, c)| { let i = idf[&w]; (w, (1.0 + c.ln()) * i) }).collect();
                let norm = v.values().map(|x| x * x).sum::<f32>().sqrt().max(1e-9);
                v.values_mut().for_each(|x| *x /= norm);
                (k, v)
            })
            .collect();
        Lex { idf, docs }
    }

    /// (target, cosine) for every target, in build order.
    fn scores(&self, q: &str) -> Vec<(String, f32)> {
        use std::collections::HashMap;
        let mut qv: HashMap<String, f32> = HashMap::new();
        for w in words(q) {
            if let Some(i) = self.idf.get(&w) {
                *qv.entry(w).or_default() += *i;
            }
        }
        let norm = qv.values().map(|x| x * x).sum::<f32>().sqrt().max(1e-9);
        self.docs
            .iter()
            .map(|(k, d)| (k.clone(), qv.iter().map(|(w, x)| x / norm * d.get(w).copied().unwrap_or(0.0)).sum::<f32>()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Reading {
    pub intent: String,
    pub p_intent: f32,
    pub target: String,
    pub p_target: f32,
    pub asked: Option<String>,
    pub p_asked: f32,
    /// (start, end) in characters of the question, input symbol
    pub spans: Vec<(usize, usize, String)>,
    /// numbers (with their units) the model gave to no input
    pub unread: Vec<(usize, usize)>,
    /// the target head's probabilities (labels.json `targets` order)
    pub target_probs: Vec<f32>,
    pub p_tags: f32,
}

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn argmax(v: &[f32]) -> (usize, f32) {
    v.iter().cloned().enumerate().fold((0, f32::MIN), |a, (i, x)| if x > a.1 { (i, x) } else { a })
}

impl Lang {
    /// A model folder: config.json, model.safetensors, tokenizer.json, labels.json, explanations.json.
    pub fn load(dir: &Path) -> Result<Lang, String> {
        let dev = Device::Cpu;
        let read = |n: &str| std::fs::read_to_string(dir.join(n)).map_err(|e| format!("{n}: {e}"));
        let cfg: Config = serde_json::from_str(&read("config.json")?).map_err(err)?;
        let labels: Value = serde_json::from_str(&read("labels.json")?).map_err(err)?;
        let explanations: Value = read("explanations.json").ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({}));
        let answers: Value = read("answers.json").ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({}));
        // a package without a matcher still answers by word overlap; a broken matcher is an error, not a silent fallback
        let matcher = if dir.join("answers_model").is_dir() { Some(Matcher::load(&dir.join("answers_model"), &dev)?) } else { None };
        let v2 = if dir.join("v2").is_dir() { Some(v2::V2::load(&dir.join("v2"))?) } else { None };
        let n = |k: &str| labels[k].as_array().map(|a| a.len()).unwrap_or(0);
        // version 1's encoder: required without v2/, optional with it (a v2 package ships only labels and explanations)
        let v1 = if v2.is_some() && !dir.join("model.safetensors").exists() {
            None
        } else {
            let raw = candle_core::safetensors::load(dir.join("model.safetensors"), &dev).map_err(err)?;
            // the training script names the encoder "encoder.", candle's ModernBert reads "model."
            let map: HashMap<String, Tensor> = raw
                .into_iter()
                .map(|(k, v)| (k.strip_prefix("encoder.").map(|r| format!("model.{r}")).unwrap_or(k), v))
                .collect();
            let vb = VarBuilder::from_tensors(map, DType::F32, &dev);
            let h = cfg.hidden_size;
            Some(V1 {
                enc: ModernBert::load(vb.clone(), &cfg).map_err(err)?,
                intent: linear(h, n("intents"), vb.pp("intent")).map_err(err)?,
                target: linear(h, n("targets"), vb.pp("target")).map_err(err)?,
                asked: linear(h, n("asked"), vb.pp("asked")).map_err(err)?,
                tags: linear(h, n("tags"), vb.pp("tags")).map_err(err)?,
                tok: tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).map_err(err)?,
            })
        };
        Ok(Lang {
            v1,
            dev,
            labels,
            explanations,
            lex: std::sync::OnceLock::new(),
            answers,
            alex: std::sync::OnceLock::new(),
            matcher,
            v2,
        })
    }

    /// Concepts the question names (its name or an alias, as a whole word; a short all-capital alias such as BOD,
    /// DO or SVI only with its capitals), best first by the model's own target probability.
    pub fn mentioned(&self, q: &str, r: &Reading) -> Vec<String> {
        let ql = q.to_lowercase();
        let qc: Vec<char> = q.chars().collect();
        let qlc: Vec<char> = ql.chars().collect();
        let whole = |hay: &[char], needle: &str| -> bool {
            let n: Vec<char> = needle.chars().collect();
            if n.is_empty() || n.len() > hay.len() {
                return false;
            }
            (0..=hay.len() - n.len()).any(|i| {
                hay[i..i + n.len()] == n[..]
                    && (i == 0 || !hay[i - 1].is_alphanumeric())
                    && (i + n.len() == hay.len() || !hay[i + n.len()].is_alphanumeric())
            })
        };
        let targets: Vec<&str> = self.labels["targets"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect()).unwrap_or_default();
        let mut hits: Vec<(String, f32)> = vec![];
        if let Some(obj) = self.explanations.as_object() {
            for (id, e) in obj {
                if !e["alias_of"].is_null() {
                    continue;
                }
                let mut names: Vec<String> = vec![e["name"].as_str().unwrap_or("").to_string()];
                names.extend(e["aliases"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)));
                // an abbreviation inside a name ("BOD" in "Ultimate BOD", "SRT" in "Minimum SRT") names it too
                let abbr: Vec<String> = names
                    .iter()
                    .flat_map(|n| n.split(|c: char| !c.is_alphanumeric() && c != '/').map(str::to_string).collect::<Vec<_>>())
                    .filter(|w| w.chars().count() >= 2 && w.chars().count() <= 5 && w.chars().filter(|c| c.is_alphabetic()).count() >= 2
                        && w.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()))
                    .collect();
                names.extend(abbr);
                let hit = names.iter().any(|n| {
                    let n = n.trim();
                    if n.chars().count() < 2 {
                        return false;
                    }
                    let short_caps = n.chars().count() <= 4 && n.chars().any(|c| c.is_uppercase())
                        && n.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
                    if short_caps { whole(&qc, n) } else { whole(&qlc, &n.to_lowercase()) }
                });
                if hit {
                    let key = format!("concept:{id}");
                    let p = targets.iter().position(|t| *t == key).and_then(|k| r.target_probs.get(k)).copied().unwrap_or(0.0);
                    hits.push((id.clone(), p));
                }
            }
        }
        hits.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        hits.into_iter().map(|h| h.0).collect()
    }

    fn lex(&self, p: &Pack) -> &Lex {
        self.lex.get_or_init(|| {
            let mut docs: Vec<(String, String)> = vec![];
            if let Some(obj) = self.explanations.as_object() {
                for (id, e) in obj {
                    if !e["alias_of"].is_null() {
                        continue;
                    }
                    let mut text = String::new();
                    // the name and aliases count three times: they are what a person writes
                    for _ in 0..3 {
                        text.push_str(e["name"].as_str().unwrap_or(""));
                        text.push(' ');
                        for a in e["aliases"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
                            text.push_str(a);
                            text.push(' ');
                        }
                    }
                    for q in e["questions"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
                        text.push_str(q);
                        text.push(' ');
                    }
                    docs.push((format!("concept:{id}"), text));
                }
            }
            for pr in &p.procedures {
                let mut text = format!("{} {} {} ", pr.name, pr.name, pr.keywords.join(" "));
                for q in &pr.questions {
                    text.push_str(q);
                    text.push(' ');
                }
                docs.push((format!("cap:{}", pr.key), text));
            }
            Lex::build(docs)
        })
    }

    /// A calculation's introduction: the reviewed intermediate-level explanation of the concept linked to it whose name
    /// shares most words with the calculation's name (ties: the shorter concept name, then the id); (concept, text).
    pub fn intro_for(&self, cap: &str, cap_name: &str) -> Option<(String, String)> {
        let nw = words(cap_name);
        let obj = self.explanations.as_object()?;
        let mut best: Option<(f32, usize, &String, &Value)> = None;
        for (id, e) in obj {
            if !e["alias_of"].is_null() || !e["capabilities"].as_array().is_some_and(|a| a.iter().any(|x| x.as_str() == Some(cap))) {
                continue;
            }
            let cw = words(e["name"].as_str().unwrap_or(""));
            if cw.is_empty() {
                continue;
            }
            let score = cw.iter().filter(|w| nw.contains(w)).count() as f32 / cw.len() as f32;
            let better = match best {
                None => true,
                Some((s, l, bid, _)) => score > s || (score == s && (cw.len() < l || (cw.len() == l && id < bid))),
            };
            if better {
                best = Some((score, cw.len(), id, e));
            }
        }
        let (_, _, _, e) = best?;
        let text = e["intermediate"].as_str().or_else(|| e["beginner"].as_str())?;
        Some((e["name"].as_str().unwrap_or("").to_string(), text.to_string()))
    }

    fn alex(&self) -> &Lex {
        self.alex.get_or_init(|| {
            // AX_ANSWER_HOLDOUT=1 leaves each answer's last other wording out of the index (calibration only: those
            // wordings then measure how well an unseen way of asking finds its answer)
            let hold = std::env::var("AX_ANSWER_HOLDOUT").is_ok_and(|v| v == "1");
            let mut docs: Vec<(String, String)> = vec![];
            for (id, a) in self.answers.as_object().into_iter().flatten() {
                let q = a["q"].as_str().unwrap_or("");
                let mut ways: Vec<&str> = a["ways"].as_array().into_iter().flatten().filter_map(|x| x.as_str()).collect();
                if hold {
                    ways.pop();
                }
                docs.push((id.clone(), format!("{q} {q} {}", ways.join(" "))));
            }
            Lex::build(docs)
        })
    }

    /// The stored answer whose question (or another wording of it) is closest to `q`: (id, score, the next best
    /// answer's score, the matcher's cosine for that answer, 1 without a matcher). None when the model has no answers.
    pub fn best_answer(&self, q: &str) -> Option<(String, f32, f32, f32)> {
        if self.answers.as_object().is_none_or(|o| o.is_empty()) {
            return None;
        }
        if let Some(m) = &self.matcher {
            return self.best_answer_matched(m, q);
        }
        let mut s = self.alex().scores(q);
        s.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        let second = s.get(1).map(|x| x.1).unwrap_or(0.0);
        s.into_iter().next().map(|(id, sc)| (id, sc, second, 1.0))
    }

    /// best_answer with the trained matcher: the highest cosine over every stored wording of each answer.
    fn best_answer_matched(&self, m: &Matcher, q: &str) -> Option<(String, f32, f32, f32)> {
        let hold = std::env::var("AX_ANSWER_HOLDOUT").is_ok_and(|v| v == "1");
        let v = m.embed(q, &self.dev).ok()?;
        let sims: Vec<f32> = m.emb.matmul(&v.t().ok()?).and_then(|t| t.flatten_all()).and_then(|t| t.to_vec1()).ok()?;
        let mut best: HashMap<&str, f32> = HashMap::new();
        for ((id, last), s) in m.rows.iter().zip(sims) {
            if hold && *last {
                continue;
            }
            let e = best.entry(id.as_str()).or_insert(f32::MIN);
            *e = e.max(s);
        }
        let mut v: Vec<(&str, f32)> = best.into_iter().collect();
        v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(b.0)));
        // with a verifier: the matcher's k best answers (verifier.json), each stored question read together with the question; the
        // score is the verifier's probability that both ask the same thing
        if let Some(ver) = &m.verifier {
            let mut scored: Vec<(&str, f32)> = v
                .iter()
                .take(ver.k)
                .filter_map(|(id, _)| Some((*id, ver.prob(q, self.answers[*id]["q"].as_str()?, &self.dev).ok()?)))
                .collect();
            scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(b.0)));
            let second = scored.get(1).map(|x| x.1).unwrap_or(0.0);
            let cos = |id: &str| v.iter().find(|x| x.0 == id).map(|x| x.1).unwrap_or(0.0);
            return scored.first().map(|(id, p)| (id.to_string(), *p, second, cos(id)));
        }
        let second = v.get(1).map(|x| x.1).unwrap_or(0.0);
        v.first().map(|(id, s)| (id.to_string(), *s, second, *s))
    }

    /// The words of a question that are not the names of the concepts it mentions, nor level words: empty for "What is
    /// BOD?" (a definition, answered by the concept's explanation), not for "Can I dump solid waste anywhere?".
    fn beyond_names(&self, q: &str, named: &[String]) -> Vec<String> {
        let mut name_words: Vec<String> = vec![];
        for id in named {
            let e = &self.explanations[id];
            name_words.extend(words(e["name"].as_str().unwrap_or("")));
            for a in e["aliases"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
                name_words.extend(words(a));
            }
        }
        const LEVEL: &[&str] = &["simply", "simple", "detail", "detailed", "kid", "child", "beginner", "advanced",
            "technical", "plain", "english", "layman", "eli5", "short", "briefly", "quick", "again", "concept", "term",
            "definition", "define", "stand", "stands", "abbreviation", "for"];
        words(q).into_iter().filter(|w| !name_words.contains(w) && !LEVEL.contains(&w.as_str())).collect()
    }

    /// The explanation target: the model's probability plus the word-overlap score, best first; (target, score).
    pub fn explain_target(&self, p: &Pack, q: &str, r: &Reading) -> (String, f32) {
        let targets: Vec<&str> = self.labels["targets"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect()).unwrap_or_default();
        let prob = |t: &str| targets.iter().position(|x| *x == t).and_then(|k| r.target_probs.get(k)).copied().unwrap_or(0.0);
        let mut best = (String::new(), f32::MIN);
        // probability of a concept plus that of every concept merged into it
        let merged_prob = |t: &str| -> f32 {
            let mut v = prob(t);
            if let (Some(id), Some(obj)) = (t.strip_prefix("concept:"), self.explanations.as_object()) {
                for (g, e) in obj {
                    if e["alias_of"].as_str() == Some(id) {
                        v += prob(&format!("concept:{g}"));
                    }
                }
            }
            v
        };
        // the words of a concept's own name (or an alias) written in the question: a strong sign it is the one meant
        let qwords: Vec<String> = words(q);
        let name_bonus = |t: &str| -> f32 {
            let Some(id) = t.strip_prefix("concept:") else { return 0.0 };
            let e = &self.explanations[id];
            let mut names = vec![e["name"].as_str().unwrap_or("").to_string()];
            names.extend(e["aliases"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)));
            names.iter().map(|n| {
                let nw = words(n);
                if nw.is_empty() { return 0.0; }
                nw.iter().filter(|w| qwords.contains(w)).count() as f32 / nw.len() as f32
            }).fold(0.0, f32::max) * NAME_BONUS
        };
        for (t, s) in self.lex(p).scores(q) {
            if t.starts_with("concept:") && (self.explanations[&t[8..]].is_null() || !self.explanations[&t[8..]]["alias_of"].is_null()) {
                continue;
            }
            let c = merged_prob(&t) + s + name_bonus(&t);
            if c > best.1 {
                best = (t, c);
            }
        }
        best
    }

    /// Autocomplete: concepts whose name or an alias matches what is typed so far (every typed word the start of one
    /// of its words; the last word may be unfinished), best first: (concept id, name, score).
    pub fn suggest_concepts(&self, typed: &str, n: usize) -> Vec<(String, String, f32)> {
        let lead = ["what", "whats", "what's", "is", "are", "the", "a", "an", "why", "how", "explain", "define", "does",
                    "do", "of", "in", "mean", "means", "tell", "me", "about"];
        let tw: Vec<String> = typed
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '/')
            .filter(|w| !w.is_empty() && !lead.contains(w))
            .map(str::to_string)
            .collect();
        if tw.is_empty() || tw.iter().map(|w| w.chars().count()).sum::<usize>() < 2 {
            return vec![];
        }
        let mut out: Vec<(String, String, f32)> = vec![];
        if let Some(obj) = self.explanations.as_object() {
            for (id, e) in obj {
                if !e["alias_of"].is_null() {
                    continue;
                }
                let name = e["name"].as_str().unwrap_or(id).to_string();
                let mut best = 0.0f32;
                let mut cands = vec![name.clone()];
                cands.extend(e["aliases"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)));
                for c in cands {
                    let cw: Vec<String> = c.to_lowercase().split(|ch: char| !ch.is_alphanumeric() && ch != '/')
                        .filter(|w| !w.is_empty()).map(str::to_string).collect();
                    let all = tw.iter().all(|t| cw.iter().any(|w| w.starts_with(t.as_str())));
                    if all {
                        let s = tw.len() as f32 / cw.len().max(1) as f32 + if cw.first().is_some_and(|w| w.starts_with(&tw[0])) { 0.5 } else { 0.0 };
                        best = best.max(s);
                    }
                }
                if best > 0.0 {
                    out.push((id.clone(), name, best));
                }
            }
        }
        out.sort_by(|a, b| b.2.total_cmp(&a.2).then(a.1.len().cmp(&b.1.len())));
        out.truncate(n);
        out
    }

    fn th(&self, k: &str) -> f32 {
        self.labels["thresholds"][k].as_f64().unwrap_or(0.0) as f32
    }

    /// The model's reading of one question (no engine involved).
    pub fn read(&self, q: &str) -> Result<Reading, String> {
        let v1 = self.v1.as_ref().ok_or_else(|| "this package has no version-1 reader".to_string())?;
        let max_len = self.labels["max_len"].as_u64().unwrap_or(256) as usize;
        let e = v1.tok.encode_char_offsets(q, true).map_err(err)?;
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
        let m = Tensor::ones((1, n), DType::U32, &self.dev).map_err(err)?;
        let hs = v1.enc.forward(&x, &m).map_err(err)?;
        let pooled = hs.mean(1).map_err(err)?;
        let sm = |t: Tensor| -> Result<Vec<f32>, String> {
            candle_nn::ops::softmax(&t, D::Minus1).and_then(|t| t.squeeze(0)).and_then(|t| t.to_vec1()).map_err(err)
        };
        let pi = sm(v1.intent.forward(&pooled).map_err(err)?)?;
        let pt = sm(v1.target.forward(&pooled).map_err(err)?)?;
        let pa = sm(v1.asked.forward(&pooled).map_err(err)?)?;
        let pg: Vec<Vec<f32>> = candle_nn::ops::softmax(&v1.tags.forward(&hs).map_err(err)?, D::Minus1)
            .and_then(|t| t.squeeze(0))
            .and_then(|t| t.to_vec2())
            .map_err(err)?;
        let lab = &self.labels;
        let name = |k: &str, i: usize| lab[k][i].as_str().unwrap_or("").to_string();
        let (i, p_intent) = argmax(&pi);
        let (t, p_target) = argmax(&pt);
        let target = name("targets", t);
        let (mut asked, mut p_asked, mut spans, mut p_tags) = (None, 1.0f32, vec![], 1.0f32);
        let mut unread: Vec<(usize, usize)> = vec![];
        if let Some(cap) = target.strip_prefix("cap:") {
            // the asked output, among this capability's outputs only
            let outs: Vec<usize> = lab["cap_outputs"][cap]
                .as_array()
                .map(|a| a.iter().filter_map(|s| lab["asked"].as_array()?.iter().position(|x| x == s)).collect())
                .unwrap_or_default();
            let tot: f32 = outs.iter().map(|&k| pa[k]).sum();
            if let Some(&k) = outs.iter().max_by(|a, b| pa[**a].total_cmp(&pa[**b])) {
                asked = Some(name("asked", k));
                p_asked = if tot > 0.0 { pa[k] / tot } else { 0.0 };
            }
            // tags, among O and this capability's inputs only
            let tag_names: Vec<String> = lab["tags"].as_array().map(|a| a.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default();
            let ins: Vec<String> = lab["cap_inputs"][cap].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default();
            let ok: Vec<usize> = tag_names
                .iter()
                .enumerate()
                .filter(|(_, t)| *t == "O" || t.split_once('-').map(|(_, s)| ins.iter().any(|x| x == s)).unwrap_or(false))
                .map(|(k, _)| k)
                .collect();
            let mut cur: Option<(usize, usize, String)> = None;
            for (row, &(s, en)) in pg.iter().zip(&offs) {
                if en <= s {
                    continue;
                }
                let tot: f32 = ok.iter().map(|&k| row[k]).sum();
                let &best = ok.iter().max_by(|a, b| row[**a].total_cmp(&row[**b])).unwrap_or(&0);
                p_tags = p_tags.min(if tot > 0.0 { row[best] / tot } else { 0.0 });
                let tn = &tag_names[best];
                if tn == "O" {
                    spans.extend(cur.take());
                    continue;
                }
                let (pre, sym) = tn.split_once('-').unwrap_or(("B", tn));
                match &mut cur {
                    Some(c) if pre == "I" && c.2 == sym => c.1 = en,
                    _ => {
                        spans.extend(cur.take());
                        cur = Some((s, en, sym.to_string()));
                    }
                }
            }
            spans.extend(cur.take());
            let chars: Vec<char> = q.chars().collect();
            spans = spans
                .into_iter()
                .filter_map(|(mut s, mut en, y)| {
                    while s < en && chars[s].is_whitespace() {
                        s += 1;
                    }
                    while en > s && matches!(chars[en - 1], ' ' | ',' | ';' | '?') {
                        en -= 1;
                    }
                    (en > s).then_some((s, en, y))
                })
                .collect();
            // numbers and their units: the spans come from a rule (candidates), the model only says which input each
            // one is (the mean over its tokens of each input's B- and I- probability); token tags alone split "0.31"
            spans.retain(|(s, e, _)| !slice(q, *s, *e).chars().any(|c| c.is_ascii_digit()));
            p_tags = 1.0;
            for (cs, ce) in candidates(q) {
                let toks: Vec<usize> =
                    offs.iter().enumerate().filter(|(_, (a, b))| b > a && *a < ce && cs < *b).map(|(k, _)| k).collect();
                if toks.is_empty() {
                    unread.push((cs, ce));
                    continue;
                }
                let mut score: Vec<(Option<String>, f32)> = vec![];
                for &k in &toks {
                    let row = &pg[k];
                    let tot: f32 = ok.iter().map(|&i| row[i]).sum();
                    for &i in &ok {
                        // "O" is outside; an input may itself be called "O" (oxygen), tagged "B-O" / "I-O"
                        let key = if tag_names[i] == "O" { None } else { tag_names[i].split_once('-').map(|x| x.1.to_string()) };
                        let add = if tot > 0.0 { row[i] / tot } else { 0.0 };
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
        }
        Ok(Reading { intent: name("intents", i), p_intent, target, p_target, asked, p_asked, spans, unread,
                     target_probs: pt.clone(), p_tags })
    }
}

const ATOMS: &[&str] = &["m", "g", "s", "L", "l", "K", "C", "°C", "degC", "Pa", "atm", "bar", "mol", "W", "J", "N", "h",
    "hr", "hrs", "min", "mins", "d", "day", "days", "yr", "year", "years", "y", "t", "tonne", "tonnes", "ton", "tons", "Hz",
    "dB", "dBA", "ppm", "ppb", "%", "ha", "ft", "mi", "gal", "cP", "VSS", "BOD", "COD", "kWh", "Wh", "eq", "cells",
    "organisms", "CFU", "MPN", "pW", "deg", "°", "hours", "hour", "minutes", "minute", "seconds", "second", "sec", "metres",
    "meters", "metre", "meter", "litres", "liters", "litre", "liter", "kg", "km", "cm", "mm", "mg", "ug", "µg", "μg", "ng",
    "kPa", "MPa", "kW", "MW", "kJ", "MJ", "ML", "mL", "mPa", "um", "µm", "μm", "nm", "mmol", "kmol", "Mg", "Gg", "GJ",
    "persons", "person", "people", "capita", "cap"];

fn is_atom(a: &str) -> bool {
    if ATOMS.contains(&a) {
        return true;
    }
    let mut c = a.chars();
    match c.next() {
        Some(p) if "kMGmcuµμnp".contains(p) && a.chars().count() > 1 => {
            let rest: String = c.collect();
            ATOMS.contains(&rest.as_str()) && !["day", "days", "min", "hours", "tonnes"].contains(&rest.as_str())
        }
        _ => false,
    }
}

/// A written unit: atoms (optionally prefixed) joined by / * · ^ ( ) and exponents (m3, m^3, d^-1, kg/(m^2*s)).
/// The twin of ai999x_factory/training/lang_decode.py `is_unit`.
pub fn is_unit(tok: &str) -> bool {
    let first = match tok.chars().next() {
        Some(c) => c,
        None => return false,
    };
    if first.is_ascii_digit() && first != '1' {
        return false;
    }
    let mut parts: Vec<String> = vec![];
    let mut cur = String::new();
    for ch in tok.chars() {
        if "/*·^()-".contains(ch) || ch.is_ascii_digit() {
            if !cur.is_empty() {
                parts.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    !parts.is_empty() && parts.iter().all(|p| is_atom(p))
}

/// Every number of the question with the unit written after it, as character ranges (the twin of lang_decode.py
/// `candidates`).
pub fn candidates(q: &str) -> Vec<(usize, usize)> {
    use fancy_regex::Regex;
    use std::sync::OnceLock;
    static NUM: OnceLock<Regex> = OnceLock::new();
    let re = NUM.get_or_init(|| {
        Regex::new(concat!(
            // (not an exponent: "m^3", "s^-1" — a digit after "^-" was read as the value 1, 2026-11-11)
            r"(?<![\w.^])(?<!\^[-+−])[-+−]?(?:\d{1,3}(?:,\d{3})+|\d+)(?:\.\d+)?(?:[eE][-+]?\d+)?(?![\w])(?!\.\d)",
            r"|(?<![\w.^])(?<!\^[-+−])[-+−]?(?:\d{1,3}(?:,\d{3})+|\d+)(?:\.\d+)?(?:[eE][-+]?\d+)?(?=[A-Za-zµμ°%])",
            r"|(?<![\w.^])(?<!\^[-+−])[-+−]?\.\d+(?:[eE][-+]?\d+)?(?![\w])"
        ))
        .expect("number pattern")
    });
    let ch = |b: usize| q[..b].chars().count();
    let mut out = vec![];
    for m in re.find_iter(q).flatten() {
        let (s, e) = (m.start(), m.end());
        let rest = &q[e..];
        let k = if rest.starts_with(' ') { 1 } else { 0 };
        let tok: String = rest[k..].chars().take_while(|c| !c.is_whitespace() && *c != ',' && *c != ';').collect();
        let mut unit_end = e;
        if !tok.is_empty() {
            let mut t2 = tok.trim_end_matches(['.', '?', '!', ':', ')']).to_string();
            let open = t2.matches('(').count();
            let close = t2.matches(')').count();
            if open > close && tok[t2.len()..].starts_with(')') {
                t2 = tok[..t2.len() + (open - close).min(tok.len() - t2.len())].to_string();
            }
            if is_unit(&t2) {
                unit_end = e + k + t2.len();
                // "12 tonnes per day": a unit followed by "per <unit word>"
                let tail = &q[unit_end..];
                if let Some(t3) = tail.strip_prefix(" per ") {
                    let w: String = t3.chars().take_while(|c| c.is_alphabetic() || "µμ°".contains(*c)).collect();
                    if is_atom(&w) {
                        unit_end += 5 + w.len();
                    }
                }
                // a unit written in parts: "77000 Pa m3/mol", "0.6 atm m3/mol", "mg min/L" (held-out 2026-11-09e: "Pa
                // m3/mol" was sent to the engine as "Pa"). The next word joins when it is itself a unit WITH "/" or
                // "^" and is not a number ("Ks", an input's name, reads as kelvin-seconds: "k 6.292 /d Ks 29310
                // mg/m^3" must not join); at most two more words
                for _ in 0..2 {
                    let tail = &q[unit_end..];
                    let Some(t3) = tail.strip_prefix(' ') else { break };
                    let w: String = t3.chars().take_while(|c| !c.is_whitespace() && *c != ',' && *c != ';').collect();
                    let w = w.trim_end_matches(['.', '?', '!', ':', ')']).to_string();
                    let first_digit = w.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true);
                    if !first_digit && w.contains(['/', '^']) && is_unit(&w) {
                        unit_end += 1 + w.len();
                    } else {
                        break;
                    }
                }
                let after = &q[unit_end..];
                if ["Pa", "mPa", "°", "deg"].contains(&t2.as_str()) {
                    for extra in [" s", " C", "*s"] {
                        if after.starts_with(extra)
                            && after[extra.len()..].chars().next().map(|c| !c.is_alphanumeric()).unwrap_or(true)
                        {
                            unit_end += extra.len();
                            break;
                        }
                    }
                }
            } else if t2.eq_ignore_ascii_case("per") {
                let tail = &rest[k + 3..];
                let lead = tail.len() - tail.trim_start().len();
                let w: String = tail.trim_start().chars().take_while(|c| c.is_alphabetic() || "µμ°".contains(*c)).collect();
                if is_atom(&w) {
                    unit_end = e + k + 3 + lead + w.len();
                }
            }
        }
        out.push((ch(s), ch(unit_end)));
    }
    out
}

fn slice(q: &str, s: usize, e: usize) -> String {
    q.chars().skip(s).take(e - s).collect()
}

/// Free-standing numbers of the question (not part of a name such as BOD5, CO2 or PM2.5), as character ranges.
/// Exam notation written the way the readers take it (2026-11-11, the owner's 40-question exam set): "1.0 x 10^-5",
/// "3.2 × 10^(-4)", "2.5*10^6" -> "1.0e-5", "3.2e-4", "2.5e6". Only a number times a power of ten changes.
pub fn exam_notation(q: &str) -> String {
    use fancy_regex::Regex;
    use std::sync::OnceLock;
    static SCI: OnceLock<Regex> = OnceLock::new();
    let re = SCI.get_or_init(|| {
        Regex::new(r"(?<![\w.])(\d+(?:\.\d+)?)\s*(?:x|X|×|\*|·)\s*10\s*\^\s*\(?\s*([-+−]?)\s*(\d+)\s*\)?(?![\w.])")
            .expect("exam notation pattern")
    });
    let mut t = re.replace_all(q, |c: &fancy_regex::Captures| {
        let sign = if &c[2] == "−" || &c[2] == "-" { "-" } else { "" };
        format!("{}e{}{}", &c[1], sign, &c[3])
    })
    .into_owned();
    // a unit spelled out right after a number, as exams write it ("9,000 cubic meters per day", "3,200 Watts",
    // "1.2 meters wide"): its symbol. Longest phrases first; only directly after a number
    const SPELLED: &[(&str, &str)] = &[
        ("cubic meters per second", "m^3/s"), ("cubic metres per second", "m^3/s"),
        ("cubic meters per day", "m^3/day"), ("cubic metres per day", "m^3/day"),
        ("cubic meters per hour", "m^3/h"), ("cubic metres per hour", "m^3/h"),
        ("cubic meters", "m^3"), ("cubic metres", "m^3"), ("square meters", "m^2"), ("square metres", "m^2"),
        ("meters per second", "m/s"), ("metres per second", "m/s"), ("meters per day", "m/day"),
        ("metres per day", "m/day"), ("meters per hour", "m/h"), ("metres per hour", "m/h"),
        ("kilometers", "km"), ("kilometres", "km"), ("meters", "m"), ("metres", "m"), ("meter", "m"), ("metre", "m"),
        ("kilograms", "kg"), ("kilogram", "kg"), ("milligrams per liter", "mg/L"), ("milligrams per litre", "mg/L"),
        ("liters per day", "L/day"), ("litres per day", "L/day"), ("liters", "L"), ("litres", "L"), ("liter", "L"),
        ("litre", "L"), ("watts", "W"), ("kilowatts", "kW"), ("seconds", "s"), ("minutes", "min"), ("hours", "h"),
        ("degrees celsius", "degC"), ("degrees c", "degC"),
    ];
    static NUM_SP: OnceLock<Regex> = OnceLock::new();
    let num_sp = NUM_SP.get_or_init(|| Regex::new(r"\d\s+$").expect("number then space"));
    for (ph, sym) in SPELLED {
        let mut out = String::new();
        let mut rest = t.as_str();
        loop {
            let lower = rest.to_lowercase();
            let Some(k) = lower.find(ph) else { out.push_str(rest); break };
            let end = k + ph.len();
            let word_end = rest[end..].chars().next().map(|c| !c.is_alphanumeric()).unwrap_or(true);
            // only straight after a number, and only on a char boundary (lowercasing kept byte lengths for ASCII)
            if word_end && rest.is_char_boundary(k) && rest.is_char_boundary(end)
                && num_sp.is_match(&rest[..k]).unwrap_or(false) {
                out.push_str(rest[..k].trim_end());
                out.push(' ');
                out.push_str(sym);
            } else {
                out.push_str(&rest[..end.min(rest.len())]);
            }
            rest = &rest[end.min(rest.len())..];
        }
        t = out;
    }
    // a unit repeated in brackets right after itself: "0.48 m^3/s (m^3/s)" -> "0.48 m^3/s"
    static REP: OnceLock<Regex> = OnceLock::new();
    let rep = REP.get_or_init(|| Regex::new(r"(\d\s*)(\S+)\s*\(\s*\2\s*\)").expect("repeated unit"));
    let t = rep.replace_all(&t, "$1$2").into_owned();
    // an ion's charge written after its symbol names the same substance: "Ca^2+", "Mg2+", "NO3-", "CO3^2-" -> "Ca",
    // "Mg", "NO3", "CO3" (only a capitalised symbol, the charge followed by a space, bracket or punctuation)
    static ION: OnceLock<Regex> = OnceLock::new();
    let ion = ION.get_or_init(|| Regex::new(r"\b((?:[A-Z][a-z]?\d*)+)\^?\d?[+\-−](?=[\s,.;:)?]|$)").expect("ion charge"));
    let t = ion.replace_all(&t, "$1").into_owned();
    // "ppmv" (by volume, for gases) is the ppm the gas calculations take
    static PPMV: OnceLock<Regex> = OnceLock::new();
    let ppmv = PPMV.get_or_init(|| Regex::new(r"(?i)\bppmv\b").expect("ppmv"));
    let t = ppmv.replace_all(&t, "ppm").into_owned();
    // a short abbreviation in brackets after the words it abbreviates is part of the label: "hydroxide ion (OH)
    // concentration of ..." -> "hydroxide ion OH concentration of ..." (the bracket cut the label: exam set Q1)
    static ABBR: OnceLock<Regex> = OnceLock::new();
    let abbr = ABBR.get_or_init(|| Regex::new(r"(?<=[A-Za-z])\s*\(\s*([A-Za-z][A-Za-z0-9]{0,4})\s*\)(?=\s+[A-Za-z])").expect("abbreviation"));
    let t = abbr.replace_all(&t, " $1").into_owned();
    // a rate written around what it measures: "2.0 L of water per day" -> "2.0 L/day of water" (exam set Q40)
    static RATE: OnceLock<Regex> = OnceLock::new();
    let rate = RATE.get_or_init(|| Regex::new(r"(\d\s*(?:L|mL|m\^3|m3|kg|g|mg|t))\s+of\s+([A-Za-z]+)\s+(?:per|a|each)\s+(day|hour|h|year|yr|minute|min|second|s)\b")
        .expect("rate around a noun"));
    rate.replace_all(&t, "$1/$3 of $2").into_owned()
}

pub fn numbers(q: &str) -> Vec<(usize, usize)> {
    let c: Vec<char> = q.chars().collect();
    let mut out = vec![];
    let mut i = 0;
    while i < c.len() {
        // a digit right after "^" (or "^-", "^+") is an exponent, not a value: "m^3", "Ca^2+", "s^-1" (2026-11-11)
        if c[i].is_ascii_digit() && i > 0 && (c[i - 1] == '^' || (i > 1 && matches!(c[i - 1], '-' | '+' | '−') && c[i - 2] == '^')) {
            while i < c.len() && c[i].is_ascii_digit() {
                i += 1;
            }
            continue;
        }
        if c[i].is_alphanumeric() || c[i] == '.' {
            let s = i;
            // scientific notation is one number: "1.160e6", "3e-5", "2.5E+07" (before 2026-11-09 only "1.160" was kept,
            // a value a million times too small)
            if c[i].is_ascii_digit() {
                let mut j = i;
                while j < c.len() && c[j].is_ascii_digit() {
                    j += 1;
                }
                if j + 1 < c.len() && c[j] == '.' && c[j + 1].is_ascii_digit() {
                    j += 1;
                    while j < c.len() && c[j].is_ascii_digit() {
                        j += 1;
                    }
                }
                if j < c.len() && (c[j] == 'e' || c[j] == 'E') {
                    let mut k = j + 1;
                    if k < c.len() && (c[k] == '-' || c[k] == '+') {
                        k += 1;
                    }
                    let d0 = k;
                    while k < c.len() && c[k].is_ascii_digit() {
                        k += 1;
                    }
                    if k > d0 && (k == c.len() || !(c[k].is_alphanumeric() || c[k] == '.' && k + 1 < c.len() && c[k + 1].is_ascii_digit())) {
                        out.push((s, k));
                        i = k;
                        continue;
                    }
                }
            }
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '.' || c[i] == ',' && i + 1 < c.len() && c[i + 1].is_ascii_digit()) {
                i += 1;
            }
            let w: String = c[s..i].iter().collect();
            let w = w.trim_end_matches(['.', ',']);
            if w.chars().any(|x| x.is_ascii_digit()) && w.chars().all(|x| x.is_ascii_digit() || x == '.' || x == ',') {
                out.push((s, s + w.chars().count()));
            } else if w.chars().next().map(|x| x.is_ascii_digit()).unwrap_or(false) {
                // "20000m3/d", "3e-5": a number glued to its unit or exponent still has to be read
                out.push((s, s + w.chars().take_while(|x| x.is_ascii_digit() || *x == '.').count()));
            }
        } else {
            i += 1;
        }
    }
    out
}

/// The unit as people write it -> as the engine reads it ("m3/d" -> "m^3/day", "per day" -> "1/day", "°C" -> "degC").
pub fn everyday_unit(text: &str, temperature: bool) -> String {
    let t = text.trim();
    let split = t.char_indices().find(|(_, ch)| !(ch.is_ascii_digit() || matches!(ch, '.' | '-' | '+' | 'e' | 'E' | ',')))
        .map(|(k, _)| k)
        .unwrap_or(t.len());
    // "3e-5 1/s": an 'e' followed by a digit or sign belongs to the number; a bare 'e' does not occur in units here
    let (num, unit) = t.split_at(split);
    let num = num.replace(',', "");
    let mut u = unit.trim().to_string();
    if u.is_empty() {
        return num;
    }
    for (a, b) in [("·", "*"), ("Pa.s", "Pa*s"), ("Pa s", "Pa*s"), ("°C", "degC"), ("deg C", "degC"), ("° C", "degC"),
                   ("µ", "u"), ("μ", "u")] {
        u = u.replace(a, b);
    }
    let low = u.to_lowercase();
    if let Some(rest) = low.strip_prefix("per ") {
        u = format!("1/{rest}");
    }
    u = u.replace(" per ", "/");
    if u.starts_with('/') {
        u = format!("1{u}");
    }
    // atom by atom
    let mut out = String::new();
    let mut atom = String::new();
    let flush = |atom: &mut String, out: &mut String| {
        if atom.is_empty() {
            return;
        }
        let a = atom.as_str();
        let word = match a {
            "d" | "days" | "Day" | "Days" => "day".to_string(),
            "hours" | "hour" | "hrs" | "hr" => "h".to_string(),
            "mins" | "minutes" | "minute" => "min".to_string(),
            "sec" | "secs" | "seconds" | "second" => "s".to_string(),
            "years" | "year" | "yrs" | "y" | "a" => "yr".to_string(),
            "tonnes" | "tonne" | "tons" | "ton" => "t".to_string(),
            "litres" | "liters" | "litre" | "liter" | "l" => "L".to_string(),
            "C" if temperature => "degC".to_string(),
            _ => {
                // m3, km2, cm3 -> m^3, km^2, cm^3
                let letters: String = a.chars().take_while(|x| x.is_alphabetic()).collect();
                let digits: String = a.chars().skip(letters.chars().count()).collect();
                if !letters.is_empty() && matches!(digits.as_str(), "2" | "3") {
                    format!("{letters}^{digits}")
                } else {
                    a.to_string()
                }
            }
        };
        out.push_str(&word);
        atom.clear();
    };
    for ch in u.chars() {
        if ch.is_alphanumeric() && !(ch.is_ascii_digit() && atom.is_empty()) {
            atom.push(ch);
        } else {
            flush(&mut atom, &mut out);
            out.push(if ch == ' ' { '*' } else { ch });
        }
    }
    flush(&mut atom, &mut out);
    let out = out.replace("**", "*").trim_matches('*').to_string();
    format!("{num} {out}")
}

fn level(q: &str) -> &'static str {
    let l = q.to_lowercase();
    if ["simple", "simply", "like i'm", "like im", "eli5", "kid", "child", "beginner", "basic", "plain english", "layman"]
        .iter()
        .any(|w| l.contains(w))
    {
        "beginner"
    } else if ["in detail", "advanced", "technical", "assumption", "derive", "derivation", "limitation", "rigorous", "expert"]
        .iter()
        .any(|w| l.contains(w))
    {
        "advanced"
    } else {
        "intermediate"
    }
}

fn dont_know(r: Option<&Reading>, why: &str) -> Value {
    let mut v = json!({"outcome": "UNKNOWN", "message": DONT_KNOW, "reader": "language model"});
    if let Some(r) = r {
        v["reading"] = reading_json(r, "");
    }
    if !why.is_empty() {
        v["why"] = json!(why);
    }
    v
}

fn reading_json(r: &Reading, q: &str) -> Value {
    json!({"intent": r.intent, "p_intent": r.p_intent, "target": r.target, "p_target": r.p_target, "asked": r.asked,
           "p_asked": r.p_asked, "p_tags": r.p_tags,
           "values": r.spans.iter().map(|(s, e, y)| json!({"symbol": y, "text": if q.is_empty() { String::new() } else { slice(q, *s, *e) }})).collect::<Vec<_>>()})
}

fn unit_of(p: &Pack, kind: &str) -> String {
    p.kinds.get(kind).map(|k| k.unit.clone()).filter(|u| !u.is_empty() && u != "1").unwrap_or_default()
}

/// The question answered: the documented-form reader first, then (when given) the trained reader.
pub fn answer(p: &Pack, lang: Option<&Lang>, q: &str) -> Value {
    let rule = ax_engine::ask::ask_value(p, q);
    if rule["outcome"] == "SOLVED" {
        return rule;
    }
    let Some(lang) = lang else { return rule };
    if let Some(v) = &lang.v2 {
        return v2::answer_v2(p, v, q);
    }
    let r = match lang.read(q) {
        Ok(r) => r,
        Err(e) => return dont_know(None, &format!("reader error: {e}")),
    };
    // a question that names a concept and asks no calculation (no number in it) is explained, with that concept: the
    // name decides which concept, the model only ranks several named ones
    if r.intent != "compute" && numbers(q).is_empty() {
        // a question with a reviewed short answer ("Can I dump solid waste anywhere?", "what if the chimney were
        // taller"): answered when it says more than the names of the concepts it mentions and its closest stored
        // question is close enough (threshold chosen on validation wordings, labels.json `answer`)
        let named = lang.mentioned(q, &r);
        if !lang.beyond_names(q, &named).is_empty() {
            if let Some((id, score, second, cos)) = lang.best_answer(q) {
                // two gates: the verifier's probability (`answer`) and the matcher's cosine (`answer_match`, 0 = off),
                // both chosen on validation; the verifier alone was fooled by off-topic questions the matcher
                // placed far away ("explain kvl and kcl simply" -> a question about KL)
                let th = lang.th("answer");
                if th > 0.0 && score >= th && cos >= lang.th("answer_match") {
                    return answered(lang, &r, q, &id, score, second, cos);
                }
            }
        }
        if let Some(c) = named.into_iter().next() {
            let mut r2 = r.clone();
            r2.target = format!("concept:{c}");
            return explain(p, lang, &r2, q);
        }
        // a question about a whole topic ("methods of water treatment") or the model ("what does it cover"): an
        // overview made of the model's own calculations, nothing generated
        if let Some(v) = survey(p, q, &r) {
            return v;
        }
    }
    if r.intent == "unknown" {
        return dont_know(Some(&r), "not in this model");
    }
    if r.intent == "explain" {
        let (t, score) = lang.explain_target(p, q, &r);
        if r.p_intent.min(score) < lang.th("explain") {
            return dont_know(Some(&r), "explanation reading not confident enough");
        }
        let mut r2 = r.clone();
        r2.target = t;
        r2.p_target = score;
        return explain(p, lang, &r2, q);
    }
    compute_reply(p, lang.th("compute"), r, q)
}

/// Chemical formulas written out, so a meaning's "SO2" and a question's "sulfur dioxide" share words (`meaning_words`
/// drops digits and words under 3 letters, which leaves nothing of "SO2").
pub(crate) fn expand_terms(text: &str) -> String {
    let mut out = String::new();
    for (i, w) in text.split(' ').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        // "DO" in capitals is dissolved oxygen ("DO at t", "saturation DO"); "do" is a verb (2026-11-11)
        if w.chars().filter(|c| c.is_alphanumeric()).collect::<String>() == "DO" {
            out.push_str("dissolved oxygen");
            continue;
        }
        let core: String = w.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
        out.push_str(match core.as_str() {
            "so2" => "sulfur dioxide",
            "co2" => "carbon dioxide",
            "ch4" => "methane",
            "o2" => "oxygen",
            "nh3" => "ammonia",
            "h2s" => "hydrogen sulfide",
            "no3" => "nitrate",
            "sulphur" => "sulfur",
            "hocl" => "hocl hypochlorous acid",
            "ocl" => "ocl hypochlorite ion",
            // abbreviations written out (a meaning may give only "KLa"; a question "oxygen transfer coefficient")
            // field words for one thing (2026-11-11, two-step development rows: "effluent of 0.4 m3/s at 30 mg/L enters
            // a river carrying 4 m3/s" is the wastewater and the river stream)
            "wastewater" => "wastewater effluent sewage outfall",
            "river" => "river stream creek",
            "kla" => "kla volumetric oxygen transfer coefficient",
            "ct" => "ct concentration contact time",
            "fm" => "fm food microorganism ratio",
            "svi" => "svi sludge volume index",
            "hrt" => "hrt hydraulic retention residence time",
            "srt" => "srt solids retention time sludge age",
            "olr" => "olr organic loading rate",
            "hhv" => "hhv higher heating value",
            "lhv" => "lhv lower heating value",
            "aotr" => "aotr actual oxygen transfer rate",
            "sotr" => "sotr standard oxygen transfer rate",
            "otr" => "otr oxygen transfer rate",
            "htu" => "htu height transfer unit",
            "ntu" => "ntu number transfer units",
            "twa" => "twa time weighted average",
            "leq" => "leq equivalent continuous sound level",
            "ldn" => "ldn day night sound level",
            "lden" => "lden day evening night sound level",
            "thod" => "thod theoretical oxygen demand",
            "bod" => "bod biochemical oxygen demand",
            "cod" => "cod chemical oxygen demand",
            "mlss" => "mlss mixed liquor suspended solids",
            "mlvss" => "mlvss mixed liquor volatile suspended solids",
            "tds" => "tds total dissolved solids",
            "tss" => "tss total suspended solids",
            "vss" => "vss volatile suspended solids",
            "esp" => "esp electrostatic precipitator",
            _ => w,
        });
    }
    out
}

/// The words of an output's meaning that can tell it apart: lowercase, letters only (digits dropped: "log10" ->
/// "log"), a plural "s" dropped, 3+ letters, not a stop word or a unit word.
fn meaning_words(text: &str) -> std::collections::HashSet<String> {
    // compared after the plural "s" is dropped, so singular forms
    // (units are not meaning: "mol" of a given "mol/L" value named the H+ output "..., mol/L"; exam set 2026-11-12)
    const UNIT: &[&str] = &["mg", "per", "day", "unit", "same", "microgram", "milligram", "gram", "cubic", "metre",
                            "meter", "kg", "litre", "liter", "hour", "minute", "second", "with", "all", "into", "from",
                            "after", "given", "value", "min", "sec", "hr", "mol", "mole", "atm", "kpa"];
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .map(|w| w.chars().filter(|c| c.is_alphabetic()).collect::<String>())
        .map(|w| if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") { w[..w.len() - 1].to_string() } else { w })
        .filter(|w| w.len() >= 3 && !STOP.contains(&w.as_str()) && !UNIT.contains(&w.as_str()))
        .collect()
}

/// Everyday words of the asked-output check that the outputs' meanings write otherwise ("BOD still LEFT to be exerted"
/// is the BOD remaining).
fn asked_synonym(t: &str) -> &str {
    match t {
        "left" | "leftover" => "remaining",
        // landfill densities as exams write them (exam set Q33, 2026-11-11): "uncompacted / as delivered" waste is the
        // loose density, the "compacted" density the one after compaction ("final" stays itself: the final DO)
        "uncompacted" | "delivered" => "loose",
        "compacted" => "compaction",
        "deep" => "depth",
        "far" => "distance",
        "tall" => "height",
        "wide" => "width",
        "standard" => "stp",
        // 2026-11-10 (rule-based reader, A21): "how many years will it last", "what fraction slips through", "by what
        // fraction does the volume drop", "with one filter down"
        "last" | "lifetime" | "lifespan" => "life",
        "through" | "escape" | "escapes" | "slip" | "slips" | "pass" | "passes" => "passing",
        "drop" | "drops" | "decrease" | "decreases" | "shrink" | "shrinks" => "reduction",
        "down" | "offline" => "out",
        // 2026-11-11 (two-step development rows): "what is the lowest DO", "how loud is the road at 100 m"
        "lowest" | "least" | "smallest" => "minimum",
        "highest" | "peak" | "largest" | "biggest" => "maximum",
        "loud" | "loudness" | "noisy" => "level",
        _ => t,
    }
}

/// Outputs whose symbol is written in the question, read as naming the output asked for: "Re, d 0.44 mm ..." (Re),
/// "G t?" (Gt), "what sigma z gives ..." (sigma_z_max, a prefix no other output shares), "CT achieved?" (CT). Written
/// forms: the symbol's parts ("sigma z"), a 2-3 letter symbol letter by letter ("G t"), a unique prefix of 2+ parts.
/// Not an occurrence followed by a number or "=" (a given value: "H = 0.0011"), not a form that is an input's symbol,
/// not a one-letter lowercase form ("s" would match "Henry's"), and only forms that read as symbols (an uppercase
/// letter or a part of 1-2 letters; "dose" or "dry mass" are words). Case-sensitive below 4 letters. Longest first.
fn named_outputs(proc_: &ax_engine::pack::Procedure, q: &str) -> Vec<String> {
    let inputs: std::collections::HashSet<String> = proc_.inputs.iter().map(|i| i.symbol.to_lowercase()).collect();
    let mut forms: Vec<(Vec<String>, String)> = vec![];
    for o in &proc_.outputs {
        let parts: Vec<String> = o.symbol.split('_').filter(|p| !p.is_empty()).map(String::from).collect();
        if parts.is_empty() {
            continue;
        }
        let mut fs = vec![parts.clone()];
        if parts.len() == 1 && (2..=3).contains(&parts[0].chars().count()) && parts[0].chars().all(|c| c.is_ascii_alphabetic()) {
            fs.push(parts[0].chars().map(|c| c.to_string()).collect());
        }
        for k in 2..parts.len() {
            let pre = &parts[..k];
            let sharing = proc_.outputs.iter()
                .filter(|p| p.symbol.split('_').filter(|x| !x.is_empty()).take(k).eq(pre.iter().map(|s| s.as_str())))
                .count();
            if sharing == 1 {
                fs.push(pre.to_vec());
            }
        }
        for f in fs {
            let one = f.len() == 1 && f[0].chars().count() == 1;
            // only forms that read as symbols: plain words ("dose", "depth", "dry mass") are for the meaning check
            let symbolic = f.iter().any(|p| p.chars().any(|c| c.is_ascii_uppercase()) || p.chars().count() <= 2);
            if !symbolic || inputs.contains(&f.join("_").to_lowercase()) || (one && !f[0].chars().all(|c| c.is_ascii_uppercase()))
                || f == ["A"] {
                continue;
            }
            forms.push((f, o.symbol.clone()));
        }
    }
    // an input's written symbol takes its words first and names nothing: "mu max 5.5 per day" is mu_max, not mu
    // (held-out 2026-11-09c). Marked by an empty name
    for i in &proc_.inputs {
        let parts: Vec<String> = i.symbol.split('_').filter(|p| !p.is_empty()).map(String::from).collect();
        if parts.len() > 1 {
            forms.push((parts, String::new()));
        }
    }
    forms.sort_by_key(|f| std::cmp::Reverse(f.0.len()));
    // tokens (case kept) and what follows each: a number, "=", or a word
    let mut toks: Vec<(String, usize)> = vec![];
    let mut starts: Vec<usize> = vec![];
    let mut cur = String::new();
    for (i, c) in q.char_indices() {
        if c.is_alphanumeric() {
            if cur.is_empty() {
                starts.push(i);
            }
            cur.push(c);
        } else if !cur.is_empty() {
            toks.push((std::mem::take(&mut cur), i));
        }
    }
    if !cur.is_empty() {
        toks.push((cur, q.len()));
    }
    let numeric = |t: &str| t.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
    // ... or "of" / "is" / "was" / "equals" and a number: "a pKa of 7.6" gives the pKa (it was read as naming the pKa
    // output, so the hypochlorite fraction asked for was refused; 2026-11-10)
    let given = |end: usize, next: Option<&(String, usize)>| -> bool {
        if q[end..].trim_start().starts_with('=') || next.map(|n| numeric(&n.0)).unwrap_or(false) {
            return true;
        }
        let rest = q[end..].trim_start();
        let w: String = rest.chars().take_while(|c| c.is_alphabetic()).collect();
        ["of", "is", "was", "equals"].contains(&w.to_lowercase().as_str())
            && rest[w.len()..].trim_start().chars().next().is_some_and(|c| c.is_ascii_digit() || c == '.')
    };
    // a unit, not a symbol: "8.68 mg/L", "281.3 K", "17100 W", "kg*s/m^3"
    const UNIT_PREV: &[&str] = &["mg", "ug", "g", "kg", "m", "cm", "mm", "km", "k", "M", "mol", "kmol", "mPa", "Pa", "kPa",
                                 "kW", "MW", "J", "kJ", "MJ", "s", "h", "d", "min", "L", "mL", "ML", "per", "deg"];
    let unit_at = |i: usize| -> bool {
        let before = q[..starts[i]].trim_end();
        // not across a sentence or clause mark: "... 1.2 mg/L. CT in mg min/L?" names the CT (the "L" before it ends
        // the previous sentence's unit)
        let marked = i > 0 && q[toks[i - 1].1..starts[i]].contains(['.', '?', '!', ',', ';', ':']);
        before.ends_with(['/', '^', '*', '·', '°'])
            || (i > 0 && !marked && (numeric(&toks[i - 1].0) || UNIT_PREV.contains(&toks[i - 1].0.as_str())))
    };
    let same = |a: &str, b: &str, long: bool| if long { a.eq_ignore_ascii_case(b) } else { a == b };
    // a symbol followed by a word of ANOTHER output (and not its own) is part of that output's name: "CT ratio" names
    // the ratio, not the CT (2026-11-11, two-step development rows)
    let out_words = |s: &str| -> std::collections::HashSet<String> {
        proc_.outputs.iter().find(|o| o.symbol == s)
            .map(|o| meaning_words(&format!("{} {}", o.meaning, o.symbol.replace('_', " ")))).unwrap_or_default()
    };
    let names_other = |sym: &str, next: Option<&(String, usize)>| -> bool {
        let Some((w, _)) = next else { return false };
        let w = w.to_lowercase();
        !out_words(sym).contains(&w) && proc_.outputs.iter().any(|o| o.symbol != sym && out_words(&o.symbol).contains(&w))
    };
    let mut out = vec![];
    let mut i = 0;
    while i < toks.len() {
        let mut step = 1;
        for (f, sym) in &forms {
            let long = f.iter().map(|p| p.chars().count()).sum::<usize>() >= 4;
            if i + f.len() <= toks.len() && f.iter().enumerate().all(|(k, p)| same(&toks[i + k].0, p, long)) {
                let last = i + f.len() - 1;
                if !sym.is_empty() && !given(toks[last].1, toks.get(last + 1)) && !unit_at(i) && !out.contains(sym)
                    && !names_other(sym, toks.get(last + 1)) {
                    out.push(sym.clone());
                }
                step = f.len();
                break;
            }
        }
        i += step;
    }
    out
}

/// The asked output must be what the question asks for (held-out 2026-11-09b: 9 calculations answered a DIFFERENT
/// output of the right calculation, with a number: "how far downwind is the max" -> the maximum concentration,
/// "log removal" -> the surviving fraction). The asked head was trained on questions that ask only what each verified
/// example asks, so it names the main output whatever is asked. Outputs are grouped into quantities (an output and
/// its unit variants: BOD_t and BOD_t_mgL are one quantity). Refused when another quantity of the calculation
/// (a) has more of its own distinctive meaning words in the question than the chosen one has against it (a tie
/// refuses too), or (b) is a length and the question asks "how far / how deep / how tall / how wide / what distance /
/// what depth / what height" while the chosen quantity is not a length (the same for a time: "how many days"), or
/// (c) the question writes its symbol (`named_outputs`). A ratio cue ("relative to") refuses a dimensional output.
/// Is the output `variant` its `base` in another unit: its meaning starts with "in" and the rest of its symbol is a
/// unit ("Px_kg_per_day" = Px "in kg VSS/d"; not "Q_CH4_stp_m3_per_day", whose "stp" is a condition).
pub(crate) fn unit_restatement(variant: &str, base: &str, variant_meaning: &str) -> bool {
    let Some(suffix) = variant.strip_prefix(&format!("{base}_")) else { return false };
    if !variant_meaning.trim_start().to_lowercase().starts_with("in ") {
        return false;
    }
    // (a suffix that starts with "per" is a reciprocal unit: FM_per_day is the F/M ratio in 1/day; held-out 2026-11-12b)
    let suffix = match suffix.strip_prefix("per_") {
        Some(r) => format!("1_per_{r}"),
        None => suffix.to_string(),
    };
    let u = everyday_unit(&format!("1 {}", suffix.replace("_per_", "/").replace('_', "*")), false);
    u.split_once(' ').is_some_and(|x| ax_engine::units::parse_unit(&derive::tidy_unit(x.1)).is_ok())
}

pub fn asked_supported(proc_: &ax_engine::pack::Procedure, q: &str, asked: &str) -> Result<(), String> {
    asked_supported_by(proc_, q, asked, false)
}

/// `network`: the reading came from the network, which may pick a result the question never names; then a question that
/// names no output at all gets only the calculation's main result (the rule-based reading still runs when it is refused)
pub fn asked_supported_by(proc_: &ax_engine::pack::Procedure, q: &str, asked: &str, network: bool) -> Result<(), String> {
    // quantity groups: an output that only restates another in a unit belongs to it: its symbol extends the other's
    // ("C_max_ugm3" of "C_max") AND its meaning adds nothing but a unit ("in m/h", "BOD exerted in mg/L"). Not
    // "v_one_out" of "v": "rate with one filter out of service" is another quantity
    let meaning = |s: &str| proc_.outputs.iter().find(|o| o.symbol == s).map(|o| o.meaning.clone()).unwrap_or_default();
    let base = |s: &str| -> String {
        let ms = meaning(s);
        // (what a unit measures is not a new quantity: "loading in kg BOD/(m^3 d)" restates the organic loading;
        // suite 2026-11-12 c061)
        const SUBSTANCE: &[&str] = &["bod", "cod", "caco", "vss", "tss", "solid", "dry"];
        let ws: std::collections::HashSet<String> = meaning_words(&ms).into_iter()
            .filter(|w| !SUBSTANCE.contains(&w.as_str())).collect();
        proc_.outputs.iter().map(|o| o.symbol.as_str())
            .filter(|b| *b != s && s.starts_with(&format!("{b}_")))
            // the variant's meaning adds no word to the base's: "in m^3/d at T and P" restates the methane flow, "in m^3/d
            // at 0 degC and 1 atm" is another quantity (held-out 2026-11-09c answered it with the flow at T and P);
            // or it is the base in a unit its symbol spells ("Px_kg_per_day": "in kg VSS/d")
            .filter(|b| ws.is_subset(&meaning_words(&meaning(b))) || unit_restatement(s, b, &ms))
            .max_by_key(|b| b.len())
            .unwrap_or(s)
            .to_string()
    };
    let mut groups: Vec<(String, std::collections::HashSet<String>, bool)> = vec![];
    for o in &proc_.outputs {
        let b = base(&o.symbol);
        // the symbol's own words count too: "stp" of Q_CH4_stp_m3_per_day, "net" of mu_net
        let mut w = meaning_words(&o.meaning);
        w.extend(meaning_words(&o.symbol.replace('_', " ")));
        // ... and the words of the output its symbol extends (the same thing in another state: Q_CH4_stp_m3_per_day
        // is methane too), so they do not tell the two apart
        if let Some(p) = proc_.outputs.iter().filter(|p| o.symbol.starts_with(&format!("{}_", p.symbol)))
            .max_by_key(|p| p.symbol.len()) {
            w.extend(meaning_words(&p.meaning));
        }
        let is_len = o.kind == "length";
        match groups.iter_mut().find(|g| g.0 == b) {
            Some(g) => {
                g.1.extend(w);
                g.2 |= is_len;
            }
            None => groups.push((b, w, is_len)),
        }
    }
    let chosen = base(asked);
    let Some(cg) = groups.iter().find(|g| g.0 == chosen).cloned() else { return Ok(()) };
    // words that name an INPUT describe a given value, not the result asked for ("initial deficit 2.6 mg/L" is not a
    // request for the critical deficit; "calcium 16.5 g/m3" not for the calcium hardness): validation, 102 of 465
    // right calculations refused before this
    let inputs: std::collections::HashSet<String> = proc_.inputs.iter()
        .flat_map(|i| meaning_words(&format!("{} {}", i.symbol.replace('_', " "), i.meaning)))
        .collect();
    // ... only where the word sits next to a number (2 words before, 3 after): "with ONE filter down" and "WET basis?"
    // ask for a result although "one" and "wet" also name inputs. Judged per occurrence: "Calcium 68.9 mg/L, ...: how
    // much of the hardness comes from calcium?" asks for the calcium hardness (validation 2026-11-09, round 5)
    // (with each token's character position)
    let mut raw: Vec<String> = vec![];
    let mut pos: Vec<usize> = vec![];
    {
        let mut cur = String::new();
        let mut st = 0;
        for (i, c) in q.chars().enumerate() {
            if c.is_alphanumeric() || c == '.' {
                if cur.is_empty() {
                    st = i;
                }
                cur.extend(c.to_lowercase());
            } else if !cur.is_empty() {
                raw.push(std::mem::take(&mut cur));
                pos.push(st);
            }
        }
        if !cur.is_empty() {
            raw.push(cur);
            pos.push(st);
        }
    }
    let is_num: Vec<bool> = raw.iter().map(|t| t.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)).collect();
    let far: std::collections::HashSet<String> = raw.iter().enumerate()
        .filter(|(i, _)| !(i.saturating_sub(2)..=(i + 3).min(raw.len().saturating_sub(1))).any(|j| j != *i && is_num[j]))
        .flat_map(|(_, t)| meaning_words(asked_synonym(t)))
        .collect();
    let qw: std::collections::HashSet<String> = raw.iter().flat_map(|t| meaning_words(asked_synonym(t)))
        .filter(|w| !(inputs.contains(w) && !far.contains(w)))
        .collect();
    // an output's symbol written in the question names it ("Re, d 0.44 mm ...", "G t?", "what sigma z gives ...")
    let named: std::collections::HashSet<String> = named_outputs(proc_, &derive::asked_clause(q)).iter().map(|s| base(s)).collect();
    if !named.is_empty() && !named.contains(&chosen) {
        let mut n: Vec<_> = named.into_iter().collect();
        n.sort();
        return Err(format!("the question names {} rather than {}", n.join(", "), chosen));
    }
    // the asked quantity must not be GIVEN: a word of the chosen output that no input has, followed (after at most two
    // more of its words, never a word with a digit) by a number or by is / of / was / equals / are and a number
    // ("ultimate BOD is 240 mg/L ... how much is still left": the ultimate BOD is a value, not the answer; held-out
    // 2026-11-10a computed it from itself)
    let own: std::collections::HashSet<String> = cg.1.iter().filter(|w| !inputs.contains(*w)).cloned().collect();
    let words_of = |t: &str| meaning_words(asked_synonym(t));
    for (i, t) in raw.iter().enumerate() {
        if words_of(t).is_disjoint(&own) {
            continue;
        }
        let mut j = i + 1;
        while j < raw.len() && j <= i + 2 && !is_num[j] && !raw[j].chars().any(|c| c.is_ascii_digit())
            && !words_of(&raw[j]).is_empty() && words_of(&raw[j]).is_subset(&cg.1) {
            j += 1;
        }
        // ... or after up to three linking words: "the ultimate BOD CAME OUT AT 250 mg/L" (held-out 2026-11-10d c033
        // was the ultimate BOD computed from itself), "measured at", "was found to be"
        // ("at" only after another linking word: "the dissolved oxygen AT 30 km" asks for the DO there)
        const LINK: &[&str] = &["is", "of", "was", "equals", "are", "came", "comes", "out", "measured", "found", "to",
                                "be", "about", "around", "roughly", "approximately", "reads", "stands"];
        let mut k = j;
        while k < raw.len() && k < j + 3 && (LINK.contains(&raw[k].as_str()) || (k > j && raw[k] == "at")) {
            k += 1;
        }
        let num = if j < raw.len() && is_num[j] { Some(j) } else if k > j && k < raw.len() && is_num[k] { Some(k) } else { None };
        // ... not where the question itself asks: "what is the ground-level concentration 1500 m downwind?" asks AT
        // a distance (a two-step development question, 2026-11-10)
        let (ha, hb) = derive::asked_head(q);
        // ... a value in ppm / ppb / % is a pure number: it cannot be a result with a physical kind ("a CO concentration
        // of 8.0 ppmv ... convert to micrograms per cubic meter": the mass concentration is asked; exam set 2026-11-11)
        let pure = |n: usize| raw.get(n + 1).is_some_and(|u| ["ppm", "ppb", "ppmv", "ppbv", "percent"].contains(&u.as_str()));
        let chosen_kind = proc_.outputs.iter().find(|o| o.symbol == chosen).map(|o| o.kind.as_str()).unwrap_or("");
        if let Some(n) = num.filter(|&n| !(pos[i] >= ha && pos[n] < hb)) {
            if !(pure(n) && !chosen_kind.is_empty() && chosen_kind != "number") {
                return Err(format!("the question gives {chosen} as a value"));
            }
        }
    }
    // the words of the asking part itself (its question word to the end of the clause)
    let (hha, hhb) = derive::asked_head(q);
    let head_words: std::collections::HashSet<String> = raw.iter().zip(&pos)
        .filter(|(_, p)| **p >= hha && **p < hhb)
        .flat_map(|(t, _)| meaning_words(asked_synonym(t)))
        .collect();
    // the network's reading only: the asking part names an INPUT by a word no result of this calculation has ("estimate
    // the ULTIMATE BOD" is L0, an input of the BOD exerted: the network read the stated BOD as L0 and answered the BOD
    // exerted; held-out 2026-11-12b development probe). The calculation that has it as a result answers, or nobody does
    if network {
        let out_words: std::collections::HashSet<String> = proc_.outputs.iter()
            .flat_map(|o| meaning_words(&format!("{} {}", o.symbol.replace('_', " "), o.meaning)))
            .collect();
        // (only the words that name the result: the asking part up to its first "for", "at", "in", "of", ...; "find
        // the stability class FOR STRONG conditions and a WIND of 9 m/s" names inputs as conditions: validation, 47
        // right calculations refused by the first version of this check)
        const STOP: &[&str] = &["for", "at", "in", "of", "on", "to", "into", "when", "if", "given", "using", "with",
                                "from", "assuming", "after", "before", "between", "under", "per", "by", "during"];
        let noun_end = raw.iter().zip(&pos)
            .filter(|(_, p)| **p > hha && **p < hhb)
            .find(|(t, _)| STOP.contains(&t.as_str()))
            .map(|(_, p)| *p).unwrap_or(hhb);
        let noun_words: std::collections::HashSet<String> = raw.iter().zip(&pos)
            .filter(|(_, p)| **p >= hha && **p < noun_end)
            .flat_map(|(t, _)| meaning_words(asked_synonym(t)))
            .collect();
        let mut names_input: Vec<String> = noun_words.iter()
            .filter(|w| inputs.contains(*w) && !out_words.contains(*w))
            .cloned().collect();
        // ... and only when the asking part starts with a question word (without one it is the whole question: "Correct
        // a BOD rate constant of 0.3 per day ...") and has no word of the chosen result that no input has ("how much
        // POWER does a rapid mix TANK need": the tank is a condition, the power is named)
        const QWORDS: &[&str] = &["what", "whats", "how", "find", "calculate", "compute", "determine", "estimate", "which",
                                  "give", "work"];
        let starts_with_cue = raw.iter().zip(&pos).find(|(_, p)| **p >= hha).is_some_and(|(t, _)| QWORDS.contains(&t.as_str()));
        let names_chosen = noun_words.iter().any(|w| cg.1.contains(w) && !inputs.contains(w));
        if !names_input.is_empty() && starts_with_cue && !names_chosen {
            names_input.sort();
            return Err(format!("the asking part names an input ({}) rather than {chosen}", names_input.join(", ")));
        }
    }
    let ql = format!(" {} ", q.to_lowercase().split(|c: char| !c.is_alphanumeric()).collect::<Vec<_>>().join(" "));
    let cue = |cs: &[&str]| cs.iter().any(|c| ql.contains(&format!(" {c} ")));
    // a plain length question keeps a length against other kinds and refuses another kind: "how far downwind is the max
    // ground concentration" asks for the distance (regression g005). Not "how high": "how high does the worst
    // ground-level concentration get" asks a concentration (held-out 2026-11-09c)
    let length_cue = cue(&["how far", "how deep", "how tall", "how wide", "what distance", "what depth", "what height", "where is the", "where does the", "where will the",
                           "how long a distance"]);
    // the same for time: "how many days downstream does the oxygen sag bottom out" asks for the critical time
    let time_cue = cue(&["how many days", "how many hours", "how many minutes", "how many seconds", "how many years",
                         "how much time", "how long does", "how long will", "how long until", "how long before",
                         "how long would", "after how long"]);
    // "how much does the bed grow RELATIVE TO its original depth" asks for a dimensionless ratio, not a length
    let ratio_cue = cue(&["relative to", "as a fraction", "what fraction", "fraction of", "ratio of", "how many times"]);
    let kind = |s: &str| proc_.outputs.iter().find(|o| o.symbol == s).map(|o| o.kind.clone()).unwrap_or_default();
    let ck = kind(&chosen);
    for g in groups.iter().filter(|g| g.0 != chosen) {
        let gk = kind(&g.0);
        if ratio_cue && gk == "number" && ck != "number" {
            return Err(format!("the question asks for a ratio ({}), not {}", g.0, chosen));
        }
        if length_cue && g.2 && !cg.2 {
            return Err(format!("the question asks for a length ({}), not {}", g.0, chosen));
        }
        if time_cue && gk == "time" && ck != "time" {
            return Err(format!("the question asks for a time ({}), not {}", g.0, chosen));
        }
        if (length_cue && cg.2 && !g.2) || (time_cue && ck == "time" && gk != "time") {
            continue;
        }
        // the asking part writes the chosen result's own symbol ("what is the pH of this water"): a sibling's words that
        // also name an INPUT ("hydroxide ion", the given value's label) do not outweigh it (exam set Q1, 2026-11-11);
        // its other words still do ("what's the Camp number, G times t?" asks Gt although G is written: validation)
        // ... and only outside the asking part: "Determine the LOG INACTIVATION: ..., CT for three-log ..." asks the
        // log inactivation although "CT" is written and "log inactivation" also names the CT999 input (validation)
        let sym_named = named.contains(&chosen);
        let own = qw.iter().filter(|w| g.1.contains(*w) && !cg.1.contains(*w)
            && !(sym_named && inputs.contains(*w) && !head_words.contains(*w))).count();
        let theirs = qw.iter().filter(|w| cg.1.contains(*w) && !g.1.contains(*w)).count();
        if own > 0 && own >= theirs {
            return Err(format!("the question may ask for {} rather than {}", g.0, chosen));
        }
        // a tie with nothing to tell the two apart ("overall moisture content?": wet or dry basis) is read as the
        // calculation's MAIN output only (the first, the convention); any other choice is refused (held-out 2026-11-10b)
        let shared = qw.iter().filter(|w| g.1.contains(*w) && cg.1.contains(*w)).count();
        // ... and an output wins a tie against one that extends its symbol ("v" over "v_one_out", which must be asked
        // with its own words)
        let main = proc_.outputs.first().map(|o| base(&o.symbol)).unwrap_or_default();
        let extends_chosen = g.0.starts_with(&format!("{chosen}_"));
        if own == 0 && theirs == 0 && shared > 0 && chosen != main && !extends_chosen {
            return Err(format!("the question does not say whether it asks for {} or {}", chosen, g.0));
        }
    }
    // a result other than the main one must be named by some word of the question (or its kind asked by a cue): "Moisture
    // fractions [0.6, 0.1] ...: how much is left once the water is removed?" names no output, and the network read the
    // dry-basis moisture at 0.986 (validation 2026-11-11; the dry mass was asked)
    let main = proc_.outputs.first().map(|o| base(&o.symbol)).unwrap_or_default();
    let cued = (length_cue && cg.2) || (time_cue && ck == "time") || (ratio_cue && ck == "number");
    // ... narrowly: only when the question names NO output at all (the words that name a value excepted, as above), the
    // situation of the existing tie rule with nothing shared; a per-output test refused 31 right validation answers
    // ("return flow" names Qr through a word the inputs share)
    let silent = groups.iter().all(|g| qw.is_disjoint(&g.1));
    if network && std::env::var("AX_DERIVE_DEBUG").is_ok() {
        eprintln!("[asked] {chosen} main {main} silent {silent} qw {:?} groups {:?}", qw, groups.iter().map(|g| (&g.0, &g.1)).collect::<Vec<_>>());
    }
    if network && chosen != main && !cued && silent && named.is_empty() && named_outputs(proc_, q).is_empty() {
        return Err(format!("the question does not name {chosen}"));
    }
    Ok(())
}

/// Word inputs read by rule (a closed vocabulary needs no network): for every choice input the network did not tag,
/// the choices written in the question as whole words (parts of "o_connor_dobbins" may be joined by a space, hyphen,
/// apostrophe or nothing; a two-part one may be reversed: "clear night"). A coined identifier (with "_") or a word of
/// 7+ letters is read anywhere; any other choice ("urban", "point", "f", "no") only right next to the input's symbol
/// or a word of its meaning ("urban terrain", "cls f", "nitrification no"). A choice word that names the asked
/// output ("express it as the dimensionless H") is set aside when another choice is written. A text input whose
/// meaning says "formula" takes the one chemical formula written. Two candidates for one input: Err (refused).
pub fn rule_values(proc_: &ax_engine::pack::Procedure, q: &str, asked: Option<&str>, tagged: &[&str])
                   -> Result<Vec<(String, String)>, String> {
    use fancy_regex::Regex;
    let low = q.to_lowercase();
    let toks: Vec<(usize, usize)> = {
        let re = Regex::new(r"[a-z0-9_'’-]+").expect("token");
        re.find_iter(&low).flatten().map(|m| (m.start(), m.end())).collect()
    };
    let mut out = vec![];
    for inp in &proc_.inputs {
        if tagged.contains(&inp.symbol.as_str()) {
            continue;
        }
        if inp.ty == "choice" {
            let near: Vec<String> = std::iter::once(inp.symbol.to_lowercase())
                .chain(inp.meaning.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() >= 4).map(String::from))
                .collect();
            let mut hits: Vec<String> = vec![];
            for c in &inp.choices {
                let parts: Vec<String> = c.to_lowercase().split('_').map(|p| fancy_regex::escape(p).into_owned()).collect();
                let mut alts = vec![parts.join(r"[\s_'’\-]?")];
                if parts.len() == 2 {
                    alts.push(format!(r"{}[\s\-]{}", parts[1], parts[0]));   // night_clear: "clear night"
                }
                let pat = format!(r"(?<![a-z0-9_])(?:{})(?![a-z0-9_])", alts.join("|"));
                let Ok(re) = Regex::new(&pat) else { continue };
                // a coined identifier or a long distinctive word is read anywhere; an everyday word only next to the
                // input's name ("urban terrain", "point source", "cls f", "nitrification no")
                // (and a field word of 5+ letters that is no everyday word, when it stands as its own list item:
                // "..., class b, rural" reads the terrain; "a plume over an urban skyline" does not; 2026-11-11)
                const EVERYDAY: &[&str] = &["point", "strong", "slight", "clear", "cloudy", "light", "heavy"];
                let distinctive = c.chars().count() >= 5 && !EVERYDAY.contains(&c.to_lowercase().as_str());
                let short = !(c.contains('_') || c.chars().count() >= 7);
                for m in re.find_iter(&low).flatten() {
                    let before = low[..m.start()].trim_end();
                    let after = low[m.end()..].trim_start();
                    let item = (before.is_empty() || before.ends_with([',', ';', ':']))
                        && (after.is_empty() || after.starts_with([',', ';', '.', '?', '!']));
                    let ok = !short || (distinctive && item) || {
                        // the tokens just before and after the match: one of them names the input
                        let i = toks.iter().position(|t| t.0 == m.start()).unwrap_or(usize::MAX);
                        let win: Vec<&str> = toks.iter().enumerate()
                            .filter(|(j, _)| i != usize::MAX && *j + 3 >= i && *j <= i + 2 && *j != i)
                            .map(|(_, t)| &low[t.0..t.1]).collect();
                        win.iter().any(|w| near.iter().any(|n| w == n || (n.len() >= 4 && w.starts_with(n.as_str()))))
                    };
                    if ok && !hits.contains(c) {
                        hits.push(c.clone());
                    }
                }
            }
            // a choice that names the asked output is the result asked for, never the input, even when it is the only
            // one written ("convert H in the atm form to the dimensionless form": held-out 2026-11-09b c075 read the
            // form as "dimensionless"); with nothing left the app ASKS for the input
            // whole words only: class "c" is not named by "concentration"
            let words_of = |s: &str| format!(" {} ", s.to_lowercase().split(|x: char| !x.is_alphanumeric()).filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" "));
            // the asked output's meaning and its symbol WITHOUT the quantity letter ("H_m_per_atm" -> "m per atm";
            // "C_max" -> "max", so a class "c" is never taken for the output)
            let aw = words_of(&asked
                .and_then(|s| proc_.outputs.iter().find(|o| o.symbol == s))
                .map(|o| format!("{} {}", o.symbol.split_once('_').map(|x| x.1).unwrap_or(""), o.meaning))
                .unwrap_or_default());
            hits.retain(|c| !aw.contains(&words_of(c)));
            if hits.len() > 1 {
                return Err(format!("could not tell which {} is meant", inp.symbol));
            }
            if let Some(c) = hits.pop() {
                out.push((inp.symbol.clone(), c));
            }
        } else if inp.ty == "text" && inp.meaning.to_lowercase().contains("formula") {
            let re = Regex::new(r"(?<![A-Za-z0-9])(?:[A-Z][a-z]?\d*){2,}(?![A-Za-z0-9])").expect("formula");
            let f: Vec<String> = re.find_iter(q).flatten().map(|m| m.as_str().to_string())
                .filter(|s| s.chars().any(|c| c.is_ascii_digit()))   // "ThOD" is not a formula; "C6H5NO2" is
                .collect();
            match f.len() {
                0 => {}
                1 => out.push((inp.symbol.clone(), f[0].clone())),
                _ => return Err(format!("could not tell which {} is meant", inp.symbol)),
            }
        }
    }
    Ok(out)
}

/// A calculation from a reading (capability in `target` as "cap:<key>", the asked output, the values' spans): the
/// engine solves it, or the reply says why not ("I don't know that one." with the reason) or what is missing. Shared
/// by the version 1 reader and the version 2 tiny model; `th` is the calibrated confidence threshold.
pub fn compute_reply(p: &Pack, th: f32, r: Reading, q: &str) -> Value {
    let Some(cap) = r.target.strip_prefix("cap:") else { return dont_know(Some(&r), "compute without a capability") };
    let Some(proc_) = p.procedures.iter().find(|x| x.key == cap) else { return dont_know(Some(&r), "unknown capability") };
    // one confidence for a calculation: the least sure of its decisions (thresholds chosen on validation data)
    if r.p_intent.min(r.p_target).min(r.p_asked).min(r.p_tags) < th {
        return dont_know(Some(&r), "calculation reading not confident enough");
    }
    if let Some((s, e)) = r.unread.first() {
        return dont_know(Some(&r), &format!("the value {} was not read", slice(q, *s, *e)));
    }
    // every free-standing number must be one the reader assigned
    for (s, e) in numbers(q) {
        if !r.spans.iter().any(|(a, b, _)| *a <= s && e <= *b) {
            return dont_know(Some(&r), &format!("the number {} was not read", slice(q, s, e)));
        }
    }
    let mut given: Vec<(String, Input)> = vec![];
    let mut by: HashMap<&str, Vec<String>> = HashMap::new();
    for (s, e, y) in &r.spans {
        by.entry(y.as_str()).or_default().push(slice(q, *s, *e));
    }
    for inp in &proc_.inputs {
        let Some(texts) = by.get(inp.symbol.as_str()) else { continue };
        if inp.ty == "vector" {
            let nums: Vec<Value> = texts
                .iter()
                .flat_map(|t| numbers(t).into_iter().map(move |(s, e)| slice(t, s, e)))
                .filter_map(|n| n.replace(',', "").parse::<f64>().ok())
                .map(|x| json!(x))
                .collect();
            given.push((inp.symbol.clone(), Input::Json(Value::Array(nums))));
        } else if texts.len() > 1 {
            return dont_know(Some(&r), &format!("two values read for {}", inp.symbol));
        } else if inp.ty == "choice" {
            let t = texts[0].to_lowercase();
            let hit: Vec<&String> = inp.choices.iter().filter(|c| t == c.to_lowercase() || t.split(|x: char| !x.is_alphanumeric()).any(|w| w == c.to_lowercase())).collect();
            if hit.len() != 1 {
                return dont_know(Some(&r), &format!("could not tell which {} is meant", inp.symbol));
            }
            given.push((inp.symbol.clone(), Input::Text(hit[0].clone())));
        } else if inp.kind == "number" || inp.kind == "integer" || inp.kind.is_empty() {
            // a pure number: the words after it ("1.8 kg per person per day" for a rate in kg/person/day) are its
            // description, not a unit to convert; a percentage goes in as written when the input takes "fraction or %"
            let t = &texts[0];
            let Some((s, e)) = numbers(t).first().copied() else { return dont_know(Some(&r), "no number in the value") };
            let num = slice(t, s, e).replace(',', "");
            let pct = t.contains('%');
            let m = inp.meaning.to_lowercase();
            // the unit written after the number must be the one the meaning names ("1.8 kg per person per day" for
            // "rate in kg per person per day"); "4 lb per person per day" would otherwise go in as 4 kg (2026-11-11)
            if !pct && inp.kind == "number" && inp.ty != "integer" {
                // the words after the number: the rest of the value's text, then the question after the value (a unit
                // the candidate rule does not know, "4 lb", is left outside the span)
                let after_q: String = r.spans.iter().find(|x| x.2 == inp.symbol)
                    .map(|(_, se, _)| q.chars().skip(*se).collect()).unwrap_or_default();
                let rest: String = t.chars().skip(e).collect::<String>() + &after_q;
                let tok: String = rest.trim_start().chars().take_while(|c| !c.is_whitespace() && *c != ',').collect();
                let tok = tok.trim_end_matches(['.', '?', '!', ':', ')', ';']);
                const OTHER: &[&str] = &["lb", "lbs", "pound", "pounds", "oz", "ounce", "ounces", "ton", "tons", "gallon",
                                         "gallons", "gal", "ft", "feet", "foot", "inch", "inches", "mile", "miles", "yd"];
                let unit_word = is_unit(tok) || OTHER.contains(&tok.to_lowercase().as_str());
                if !tok.is_empty() && unit_word
                    && !m.split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')').any(|w| w == tok.to_lowercase())
                {
                    return dont_know(Some(&r), &format!("the unit {tok} of {} is not the one it is given in", inp.symbol));
                }
            }
            let v = if !pct {
                num
            } else if m.contains("or %") || m.contains("percent") {
                num
            } else if m.contains("fraction") {
                match num.parse::<f64>() {
                    Ok(x) => format!("{}", x / 100.0),
                    Err(_) => return dont_know(Some(&r), "unreadable percentage"),
                }
            } else {
                return dont_know(Some(&r), &format!("a percentage for {}, which is not a fraction", inp.symbol));
            };
            given.push((inp.symbol.clone(), Input::Text(v)));
        } else {
            given.push((inp.symbol.clone(), Input::Text(everyday_unit(&texts[0], inp.kind == "temperature"))));
        }
    }
    if by.keys().any(|k| !proc_.inputs.iter().any(|i| i.symbol == *k)) {
        return dont_know(Some(&r), "a value read for an input this calculation does not have");
    }
    if let Some(asked) = r.asked.as_deref() {
        if let Err(why) = asked_supported_by(proc_, q, asked, true) {
            return dont_know(Some(&r), &why);
        }
    }
    let tagged: Vec<&str> = by.keys().copied().collect();
    match rule_values(proc_, q, r.asked.as_deref(), &tagged) {
        Ok(v) => given.extend(v.into_iter().map(|(s, t)| (s, Input::Text(t)))),
        Err(why) => return dont_know(Some(&r), &why),
    }
    let a = solve(p, cap, &given, r.asked.as_deref());
    let mut v = serde_json::to_value(&a).unwrap_or(json!({}));
    v["reader"] = json!("language model");
    v["reading"] = reading_json(&r, q);
    // the same shape as ax_engine::ask's reply, so an app fills its form from `request` and shows "Read as"
    let inputs: serde_json::Map<String, Value> = given
        .iter()
        .map(|(s, i)| {
            let j = match i {
                Input::Text(t) => json!(t),
                Input::Int(n) => json!(n),
                Input::Float(x) => json!(x),
                Input::Json(j) => j.clone(),
            };
            (s.clone(), j)
        })
        .collect();
    v["capability"] = json!(cap);
    v["name"] = json!(proc_.name);
    v["request"] = json!({"capability": cap, "inputs": inputs, "target": r.asked});
    v["answer"] = if a.solved() { serde_json::to_value(&a).unwrap_or(Value::Null) } else { Value::Null };
    if a.outcome == "NEEDS_INPUT" {
        let have: Vec<&str> = given.iter().map(|g| g.0.as_str()).collect();
        let need: Vec<String> = proc_
            .inputs
            .iter()
            .filter(|i| i.required && !have.contains(&i.symbol.as_str()))
            .map(|i| {
                let u = unit_of(p, &i.kind);
                if u.is_empty() { i.meaning.clone() } else { format!("{} ({u})", i.meaning) }
            })
            .collect();
        let what = r.asked.as_deref().and_then(|s| proc_.outputs.iter().find(|o| o.symbol == s)).map(|o| o.meaning.clone()).unwrap_or_else(|| proc_.name.to_lowercase());
        if !need.is_empty() {
            v["message"] = json!(format!("To work out the {what} ({}), I also need: {}.", proc_.name, need.join("; ")));
            v["needs"] = json!(need);
        }
    }
    v
}

/// The topics of a model (the part of a calculation's topic before " / ": "water treatment / filtration" is in "Water
/// treatment", as the app's menu groups them), in the order first named, each with its calculations.
fn topics(p: &Pack) -> Vec<(String, Vec<&ax_engine::pack::Procedure>)> {
    let mut out: Vec<(String, Vec<&ax_engine::pack::Procedure>)> = vec![];
    for pr in &p.procedures {
        let top = pr.topic.split(" / ").next().unwrap_or("").trim();
        let mut c = top.chars();
        let name = match c.next() {
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            None => continue,
        };
        match out.iter_mut().find(|t| t.0 == name) {
            Some(t) => t.1.push(pr),
            None => out.push((name, vec![pr])),
        }
    }
    out
}

/// A survey question answered from the model's own catalogue: a topic all of whose words the question writes, when
/// nothing else is asked (no word left that is not the topic's, a STOP word or a SURVEY word); or the whole model, when
/// the question is only SURVEY words.
fn survey(p: &Pack, q: &str, r: &Reading) -> Option<Value> {
    let qw: Vec<String> = words(q);
    let survey_w: Vec<String> = SURVEY.iter().map(|w| stem(w)).collect();
    let rest: Vec<&String> = qw.iter().filter(|w| !survey_w.contains(w)).collect();
    let ts = topics(p);
    let mut best: Option<(&(String, Vec<&ax_engine::pack::Procedure>), usize)> = None;
    for t in &ts {
        let tw = words(&t.0);
        if !tw.is_empty() && tw.iter().all(|w| qw.contains(w)) && rest.iter().all(|w| tw.contains(w))
            && best.map_or(true, |b| tw.len() > b.1) {
            best = Some((t, tw.len()));
        }
    }
    let named = |prs: &[&ax_engine::pack::Procedure], n: usize| -> String {
        let mut names: Vec<String> = prs.iter().take(n).map(|x| lower_first(&x.name)).collect();
        if prs.len() > n {
            names.push(format!("and {} more", prs.len() - n));
        }
        names.join("; ")
    };
    if let Some(((topic, prs), _)) = best {
        // its areas (the part after " / ": disinfection, filtration, ...) are the methods a survey question asks for
        let mut areas: Vec<(String, usize)> = vec![];
        for pr in prs.iter() {
            let Some((_, a)) = pr.topic.split_once(" / ") else { continue };
            match areas.iter_mut().find(|x| x.0 == a.trim()) {
                Some(x) => x.1 += 1,
                None => areas.push((a.trim().to_string(), 1)),
            }
        }
        let text = if areas.len() >= 2 {
            let list: Vec<String> = areas.iter().map(|(a, n)| format!("{a} ({n})")).collect();
            format!("{topic} in this model covers {} areas: {}. That is {} calculations in all, each listed below with a \
                     worked example: open one to change its inputs, or ask about any of them in words.",
                    areas.len(), list.join(", "), prs.len())
        } else {
            format!("{topic} in this model is {} calculations: {}. Each one is listed below with a worked example: open \
                     one to change its inputs, or ask about any of them in words.", prs.len(), named(prs, 8))
        };
        return Some(json!({"outcome": "EXPLAINED", "message": text, "level": "overview", "name": topic,
                           "related": prs.iter().map(|x| x.key.clone()).collect::<Vec<_>>(),
                           "reader": "language model", "reading": reading_json(r, q),
                           "note": "an overview of this model's calculations"}));
    }
    if rest.is_empty() && !qw.is_empty() {
        let parts: Vec<String> = ts.iter().map(|(t, prs)| format!("{} ({})", lower_first(t), prs.len())).collect();
        let text = format!("This model has {} calculations in {} topics: {}. Ask about a topic or a calculation, or type \
                            the numbers of a problem and it is worked out.", p.procedures.len(), ts.len(), parts.join(", "));
        return Some(json!({"outcome": "EXPLAINED", "message": text, "level": "overview", "name": "What this model covers",
                           "related": [], "reader": "language model", "reading": reading_json(r, q),
                           "note": "an overview of this model's calculations"}));
    }
    None
}

/// "Briggs plume rise" stays as written; "Gas concentration" becomes "gas concentration" (a name that starts with an
/// abbreviation or a proper name keeps its capital).
fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match (c.next(), c.clone().next()) {
        (Some(f), Some(g)) if f.is_uppercase() && g.is_lowercase() && !PROPER.iter().any(|n| s.starts_with(n)) =>
            f.to_lowercase().collect::<String>() + c.as_str(),
        _ => s.to_string(),
    }
}

const PROPER: &[&str] = &["Briggs", "Gaussian", "Henry", "Stokes", "Streeter", "Pasquill", "Langmuir", "Freundlich", "Monod",
    "Chick", "Camp", "Thomas", "Lapple", "Deutsch", "Holland", "Darcy", "Fick", "Arrhenius", "O'C", "Dulong", "Newton",
    "Reynolds", "Hazen", "Manning", "Kozeny", "Carman", "Rose", "Fair"];

/// A stored, reviewed answer; shaped like an explanation so every app shows it the same way (title: the concept or the
/// part of the field it belongs to; related: that concept's calculations).
fn answered(lang: &Lang, r: &Reading, q: &str, id: &str, score: f32, second: f32, cos: f32) -> Value {
    let a = &lang.answers[id];
    let concept = a["concept"].as_str().unwrap_or("");
    let e = &lang.explanations[concept];
    let name = e["name"].as_str().filter(|s| !s.is_empty()).or_else(|| a["part"].as_str()).unwrap_or("");
    json!({"outcome": "EXPLAINED", "message": a["answer"], "level": "answer", "answer_id": id, "matched": a["q"],
           "kind": a["kind"], "concept": concept, "related": e["capabilities"].clone(), "name": name,
           "score": score, "next_score": second, "match": cos, "reader": "language model",
           "reading": reading_json(r, q), "note": "answer text reviewed, not computed"})
}

fn explain(p: &Pack, lang: &Lang, r: &Reading, q: &str) -> Value {
    let lv = level(q);
    if let Some(c0) = r.target.strip_prefix("concept:") {
        let c = lang.explanations[c0]["alias_of"].as_str().unwrap_or(c0);
        let e = &lang.explanations[c];
        let text = e[lv].as_str().or_else(|| e["intermediate"].as_str()).unwrap_or("");
        if text.is_empty() {
            return dont_know(Some(r), "no stored explanation");
        }
        return json!({"outcome": "EXPLAINED", "message": text, "level": lv, "concept": c, "related": e["capabilities"].clone(),
                      "name": e["name"].as_str().unwrap_or(c), "reader": "language model", "reading": reading_json(r, q),
                      "note": "explanation text reviewed, not computed"});
    }
    if let Some(cap) = r.target.strip_prefix("cap:") {
        if let Some(pr) = p.procedures.iter().find(|x| x.key == cap) {
            let ins: Vec<String> = pr
                .inputs
                .iter()
                .filter(|i| i.required)
                .map(|i| {
                    let u = unit_of(p, &i.kind);
                    if u.is_empty() { i.meaning.clone() } else { format!("{} ({u})", i.meaning) }
                })
                .collect();
            let text = format!("{}: {}. Give me {} and I will work it out.", pr.name, pr.method.trim_end_matches('.'), ins.join("; "));
            return json!({"outcome": "EXPLAINED", "message": text, "capability": cap, "name": pr.name,
                          "reader": "language model", "reading": reading_json(r, q)});
        }
    }
    dont_know(Some(r), "nothing to explain")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_people_write() {
        for (a, t, b) in [("20000 m3/d", false, "20000 m^3/day"), ("0.23 per day", false, "0.23 1/day"),
                          ("0.2 /day", false, "0.2 1/day"), ("0.5 d^-1", false, "0.5 day^-1"), ("25 C", true, "25 degC"),
                          ("25 °C", true, "25 degC"), ("2000 kg/d", false, "2000 kg/day"), ("5 m3", false, "5 m^3"),
                          ("0.001 Pa s", false, "0.001 Pa*s"), ("12 mg/L", false, "12 mg/L"), ("100 ug/m3", false, "100 ug/m^3"),
                          ("3 hours", false, "3 h"), ("1,200 m3", false, "1200 m^3"), ("50 tonnes/yr", false, "50 t/yr"),
                          ("8", false, "8"), ("1.5 km2", false, "1.5 km^2"), ("3e-5 1/s", false, "3e-5 1/s")] {
            assert_eq!(everyday_unit(a, t), b, "{a}");
        }
    }

    fn choice_pack() -> Pack {
        let inp = |s: &str, m: &str, ty: &str, ch: Vec<&str>| json!({"symbol": s, "kind": "number", "meaning": m,
                                                                     "required": true, "type": ty, "choices": ch});
        let doc = json!({
            "model": "t", "distribution": "t-ai999x", "version": "1.0.0",
            "kinds": {"number": {"unit": "1", "dim": ["0","0","0","0","0","0","0","0"], "difference": false}},
            "relations": [], "labels": {}, "forms": [],
            "procedures": [{
                "key": "k", "name": "K", "method": "", "primary": "y", "native": false, "topic": "", "keywords": [],
                "templates": [], "questions": [], "conditions": [], "equations": [], "assumptions": [], "structured_example": "",
                "inputs": [inp("cls", "Pasquill stability class A-F", "choice", vec!["a", "b", "c", "d", "e", "f"]),
                           inp("terrain", "rural or urban dispersion coefficients", "choice", vec!["rural", "urban"]),
                           inp("method", "o_connor_dobbins, churchill or owens_gibbs", "choice", vec!["o_connor_dobbins", "churchill", "owens_gibbs"]),
                           inp("form", "form of H", "choice", vec!["dimensionless", "m_per_atm", "night_clear"]),
                           inp("formula", "molecular formula", "text", vec![])],
                "outputs": [{"symbol": "H_dimensionless", "kind": "number", "meaning": "dimensionless Henry constant", "expr": null}],
            }],
        });
        Pack::from_json(&doc.to_string()).unwrap()
    }

    /// held-out 2026-11-09b: the asked head named the main output whatever was asked; the wording check refuses
    #[test]
    fn the_asked_output_must_be_what_the_question_asks() {
        let out = |s: &str, k: &str, m: &str| json!({"symbol": s, "kind": k, "meaning": m, "expr": null});
        let doc = json!({
            "model": "t", "distribution": "t-ai999x", "version": "1.0.0",
            "kinds": {"number": {"unit": "1", "dim": ["0","0","0","0","0","0","0","0"], "difference": false}},
            "relations": [], "labels": {}, "forms": [],
            "procedures": [{
                "key": "k", "name": "K", "method": "", "primary": "C_max", "native": false, "topic": "", "keywords": [],
                "templates": [], "questions": [], "conditions": [], "equations": [], "assumptions": [], "structured_example": "",
                "inputs": [],
                "outputs": [out("x_max", "length", "downwind distance of the maximum"),
                            out("C_max", "mass_concentration", "maximum ground-level concentration"),
                            out("C_max_ugm3", "number", "maximum concentration in micrograms per cubic metre"),
                            out("v", "speed", "filtration rate with all filters"),
                            out("v_m_per_h", "number", "in m/h"),
                            out("v_one_out", "speed", "rate with one filter out of service"),
                            out("v_one_out_m_per_h", "number", "in m/h"),
                            out("survival", "number", "N/N0"),
                            out("log_removal", "number", "log10 removal")],
            }],
        });
        let p = Pack::from_json(&doc.to_string()).unwrap();
        let pr = &p.procedures[0];
        let ok = |q: &str, a: &str| asked_supported(pr, q, a).is_ok();
        // a different quantity asked: refused
        assert!(!ok("how far downwind is the max ground concentration, Q 250 g/s, u 5 m/s", "C_max"));
        assert!(!ok("filtration rate in m/h with one filter down for backwash", "v_m_per_h"));
        assert!(!ok("what log removal does Chick's law give", "survival"));
        // the quantity asked, or its unit variant: kept
        assert!(ok("maximum ground-level concentration in micrograms per cubic metre", "C_max_ugm3"));
        assert!(ok("maximum ground-level concentration in micrograms per cubic metre", "C_max"));
        assert!(ok("filtration rate in m/h, 60 ML/day over 300 m2", "v_m_per_h"));
        assert!(ok("how far downwind is the maximum", "x_max"));
        assert!(ok("log removal for 1e6 to 25", "log_removal"));
    }

    /// Round-5 validation (2026-11-09): the right calculation, another output, at confidence 0.86-0.98.
    #[test]
    fn a_written_symbol_or_a_word_away_from_numbers_names_the_asked_output() {
        let out = |s: &str, k: &str, m: &str| json!({"symbol": s, "kind": k, "meaning": m, "expr": null});
        let inp = |s: &str, m: &str| json!({"symbol": s, "kind": "number", "meaning": m, "required": true, "type": "number", "choices": []});
        let proc_ = |key: &str, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>| json!({
            "key": key, "name": key, "method": "", "primary": "", "native": false, "topic": "", "keywords": [],
            "templates": [], "questions": [], "conditions": [], "equations": [], "assumptions": [], "structured_example": "",
            "inputs": inputs, "outputs": outputs});
        let doc = json!({
            "model": "t", "distribution": "t-ai999x", "version": "1.0.0",
            "kinds": {"number": {"unit": "1", "dim": ["0","0","0","0","0","0","0","0"], "difference": false}},
            "relations": [], "labels": {}, "forms": [],
            "procedures": [
                proc_("settle", vec![inp("d", "particle diameter"), inp("mu", "fluid viscosity")],
                      vec![out("vt", "speed", "terminal settling velocity"), out("Re", "number", "particle Reynolds number"),
                           out("Cd", "number", "drag coefficient")]),
                proc_("g", vec![inp("P", "power dissipated in the water"), inp("V", "tank volume"), inp("t", "detention time (for G t)")],
                      vec![out("G", "velocity_gradient", "mean velocity gradient"), out("Gt", "number", "Camp number G t (dimensionless)")]),
                proc_("plume", vec![inp("H", "effective stack height"), inp("k", "ratio sigma_y/sigma_z")],
                      vec![out("C_max", "mass_concentration", "maximum ground-level concentration"),
                           out("C_max_ugm3", "number", "maximum concentration in micrograms per cubic metre"),
                           out("sigma_z_max", "length", "vertical dispersion coefficient at the maximum")]),
                proc_("hard", vec![inp("Ca", "calcium concentration (as Ca)"), inp("Mg", "magnesium concentration (as Mg)")],
                      vec![out("total_hardness", "mass_concentration", "total hardness, mg/L as CaCO3"),
                           out("calcium_hardness", "mass_concentration", "calcium hardness, mg/L as CaCO3")]),
                proc_("bod", vec![inp("L0", "ultimate (carbonaceous) BOD"), inp("t", "incubation or travel time")],
                      vec![out("BOD_t", "mass_concentration", "BOD exerted after t"), out("L_t", "mass_concentration", "BOD remaining after t")]),
                proc_("bed", vec![inp("L", "unexpanded bed depth"), inp("vb", "backwash rate")],
                      vec![out("Le", "length", "expanded bed depth"), out("expansion", "number", "fractional expansion Le/L - 1")]),
                proc_("ch4", vec![inp("COD", "COD removed"), inp("T", "gas temperature")],
                      vec![out("Q_CH4", "flow_rate", "methane flow at T and P"),
                           out("Q_CH4_m3_per_day", "number", "in m^3/d at T and P"),
                           out("Q_CH4_stp_m3_per_day", "number", "in m^3/d at 0 degC and 1 atm")]),
                proc_("monod", vec![inp("mu_max", "maximum specific growth rate"), inp("kd", "endogenous decay coefficient")],
                      vec![out("mu", "first_order_rate_constant", "specific growth rate"), out("mu_per_day", "number", "in 1/d"),
                           out("mu_net", "first_order_rate_constant", "net specific growth rate mu - kd"),
                           out("mu_net_per_day", "number", "net rate in 1/d")]),
                proc_("sag", vec![inp("kd", "deoxygenation rate"), inp("D0", "initial deficit")],
                      vec![out("t_c", "time", "time to the critical point"), out("DO_min", "mass_concentration", "minimum dissolved oxygen")]),
                proc_("ubod", vec![inp("BOD_t", "BOD measured after time t (e.g. BOD5)"), inp("k", "BOD rate constant (base e)"),
                                   inp("t", "incubation time")],
                      vec![out("L0", "mass_concentration", "ultimate BOD"), out("L0_mgL", "number", "ultimate BOD in mg/L")]),
                proc_("moist", vec![inp("m", "wet masses of the components (any one unit)"), inp("w", "moisture fractions of the components, wet basis")],
                      vec![out("moisture_wet", "number", "moisture content, wet basis"), out("dry_mass", "number", "dry mass, in the unit of m"),
                           out("moisture_dry", "number", "moisture content, dry basis")]),
            ],
        });
        let p = Pack::from_json(&doc.to_string()).unwrap();
        let ok = |i: usize, q: &str, a: &str| asked_supported(&p.procedures[i], q, a).is_ok();
        // a written symbol names the output: another one is refused, the named one kept
        assert!(!ok(0, "Re, d 0.0004412 m, mu 2.021 mPa*s", "vt"));
        assert!(ok(0, "Re, d 0.0004412 m, mu 2.021 mPa*s", "Re"));
        assert!(!ok(1, "G t? P 17100 W V 518 m^3 t 1272 s", "G"));
        assert!(ok(1, "G t? P 17100 W V 518 m^3 t 1272 s", "Gt"));
        assert!(ok(1, "G? P 17100 W V 518 m^3", "G"));
        assert!(!ok(2, "For an effective height of 163 m (ratio 3.49), what sigma z gives the highest concentration?", "C_max"));
        // a given value or a unit is not a name: "t 1272 s", "8.68 mg/L", "281 K"
        assert!(ok(1, "mean velocity gradient? P 17.1 kW, V 518 m^3, t 1272 s", "G"));
        // an input word away from its number asks for the result; next to it, it is a given value
        assert!(!ok(3, "Calcium 68.9 mg/L, magnesium 17.4 g/m3: how much of the hardness comes from calcium?", "total_hardness"));
        assert!(ok(3, "Calcium 68.9 mg/L, magnesium 17.4 g/m3: total hardness?", "total_hardness"));
        // "left" is "remaining"
        assert!(!ok(4, "With k 0.2679 /d and an ultimate BOD of 143.1 mg/l, find the BOD still left to be exerted at 16 days.", "BOD_t"));
        assert!(ok(4, "ultimate BOD 143.1 mg/l, k 0.27 /d: BOD exerted after 5 days?", "BOD_t"));
        // "relative to" asks for a ratio, not a length
        assert!(!ok(5, "Backwashing a bed of depth 0.508 m: how much does the bed grow relative to its original depth?", "Le"));
        assert!(ok(5, "Backwashing a bed of depth 0.508 m: expanded bed depth?", "Le"));
        // a plain length question keeps a length (regression g005); "deep" is depth (g008)
        assert!(ok(2, "how far downwind is the max ground concentration, ratio 3.49, height 130 m", "sigma_z_max"));
        assert!(!ok(2, "how far downwind is the max ground concentration, ratio 3.49, height 130 m", "C_max"));
        // held-out 2026-11-09c: "how high ... concentration" asks the concentration; STP is another quantity than
        // the flow at T and P; "mu max 5.5" is an input; "how many days" asks a time
        assert!(ok(2, "How high does the worst ground-level concentration get, in ug/m3, for an effective height of 130 m?", "C_max_ugm3"));
        assert!(!ok(6, "removes 3200 kg/day of COD, gas at 35 C. How much methane at standard conditions in m3/d?", "Q_CH4_m3_per_day"));
        assert!(ok(6, "removes 3200 kg/day of COD, gas at 35 C. How much methane at standard conditions in m3/d?", "Q_CH4_stp_m3_per_day"));
        assert!(ok(6, "removes 3200 kg/day of COD, gas at 35 C. Methane flow in m3/d?", "Q_CH4_m3_per_day"));
        assert!(ok(7, "mu max 5.5 per day, endogenous decay 0.08 per day. What's the net specific growth rate per day?", "mu_net_per_day"));
        assert!(!ok(7, "mu max 5.5 per day, endogenous decay 0.08 per day. What's the net specific growth rate per day?", "mu_per_day"));
        assert!(ok(8, "How many days downstream does the oxygen sag bottom out? kd 0.3 per day, initial deficit 1.2 mg/L", "t_c"));
        assert!(!ok(8, "How many days downstream does the oxygen sag bottom out? kd 0.3 per day, initial deficit 1.2 mg/L", "DO_min"));
        // the asked quantity is not given as a value (held-out 2026-11-10a c012)
        assert!(!ok(9, "ultimate BOD is 240 mg/L and k is 0.21 per day. how much oxygen demand is still left after 3 days?", "L0"));
        assert!(ok(9, "BOD5 is 180 mg/L and k is 0.23 per day: what is the ultimate BOD?", "L0"));
        assert!(ok(9, "Ultimate BOD? BOD5 is 180 mg/L, k 0.23 per day, 5 days", "L0"));
        // a tie is read as the main output only (held-out 2026-11-10b c080)
        let q = "mixed waste: the components weigh 45, 30, 10, 15 kg with moisture fractions 0.65, 0.08, 0.03, 0.25 (wet basis). overall moisture content?";
        assert!(!ok(10, q, "moisture_dry"));
        assert!(ok(10, q, "moisture_wet"));
        assert!(ok(10, "components 45, 30 kg, moisture fractions 0.65, 0.08: moisture content on a dry basis?", "moisture_dry"));
    }

    #[test]
    fn word_inputs_are_read_by_rule_and_never_guessed() {
        let p = choice_pack();
        let pr = &p.procedures[0];
        let get = |q: &str| rule_values(pr, q, Some("H_dimensionless"), &[]).map(|v| v.into_iter().collect::<HashMap<_, _>>());
        let r = get("sigma_z, x 4649 m, cls f, urban terrain, method churchill").unwrap();
        assert_eq!((r["cls"].as_str(), r["terrain"].as_str(), r["method"].as_str()), ("f", "urban", "churchill"));
        // everyday words away from the input's name are not read: "a", "urban" here
        let r = get("a plume over an urban skyline").unwrap();
        assert!(!r.contains_key("cls") && !r.contains_key("terrain"));
        // identifiers with their parts joined differently, and reversed
        assert_eq!(get("using the O'Connor-Dobbins formula").unwrap()["method"], "o_connor_dobbins");
        assert_eq!(get("a clear night").unwrap()["form"], "night_clear");
        // the asked output's own word is set aside; two other choices are refused
        assert_eq!(get("H 9.5 in m_per_atm, give the dimensionless H").unwrap()["form"], "m_per_atm");
        assert!(get("churchill or owens gibbs?").is_err());
        // ... even when it is the only choice written: then the input is not read (the app asks for it)
        assert!(!get("convert 550 in the atm (mole fraction) form to the dimensionless form").unwrap().contains_key("form"));
        // a formula needs a digit: ThOD is the quantity, C6H5NO2 the formula
        let r = get("ThOD of C6H5NO2").unwrap();
        assert_eq!(r["formula"], "C6H5NO2");
        assert!(!get("ThOD please").unwrap().contains_key("formula"));
    }

    #[test]
    fn value_candidates() {
        for (q, want) in [
            ("Chlorine dose for 20000 m3/d, demand 2.5 mg/L, residual 0.5", vec!["20000 m3/d", "2.5 mg/L", "0.5"]),
            ("k = 0.2 per day and 25 °C, 0.001 Pa s", vec!["0.2 per day", "25 °C", "0.001 Pa s"]),
            ("BOD5 is 200 mg/L at 4.090e-8 kg/(m^2*s). Next", vec!["200 mg/L", "4.090e-8 kg/(m^2*s)"]),
            ("convert 50 ug/m3 of SO2 to ppm", vec!["50 ug/m3"]),
            ("carbon 0.509, hydrogen 0.0704", vec!["0.509", "0.0704"]),
            // a unit in parts (held-out 2026-11-09e: "Pa m3/mol" was read as "Pa")
            ("Henry volatility constant 77000 Pa m3/mol, 0.21 atm, CT 45 mg min/L", vec!["77000 Pa m3/mol", "0.21 atm", "45 mg min/L"]),
            ("a 5 m wide channel at 20 °C for 3 h at night", vec!["5 m", "20 °C", "3 h"]),
            ("Y 0.688 k 6.292 /d Ks 29310mg/m^3", vec!["0.688", "6.292 /d", "29310mg/m^3"]),
        ] {
            let got: Vec<String> = candidates(q).into_iter().map(|(s, e)| slice(q, s, e)).collect();
            assert_eq!(got, want, "{q}");
        }
    }

    #[test]
    fn numbers_not_names() {
        let q = "BOD5 is 200 mg/L, CO2 at 25 C and PM2.5 of 1,200 ug/m3";
        let got: Vec<String> = numbers(q).into_iter().map(|(s, e)| slice(q, s, e)).collect();
        assert_eq!(got, vec!["200", "25", "1,200"]);
        // scientific notation is one number, never its mantissa alone
        let q = "influent 1.160e6, effluent 25400, k = 3e-5 per s and 2.5E+07 cells; 20000m3/d";
        let got: Vec<String> = numbers(q).into_iter().map(|(s, e)| slice(q, s, e)).collect();
        assert_eq!(got, vec!["1.160e6", "25400", "3e-5", "2.5E+07", "20000"]);
    }

    /// Every everyday unit the training data writes (tools/v2_units.py ALT) is attached to its number by `candidates`,
    /// tidied by `everyday_unit` and converted by the engine with the same factor as the data: a unit the app cannot
    /// read would silently turn "8.68 mg/L" into the pure number 8.68.
    #[test]
    fn everyday_units_of_the_training_data_convert_exactly() {
        use ax_engine::units::parse_unit;
        let table: &[(&str, &str, f64)] = &[
            ("kg/m^3", "mg/L", 1e3), ("kg/m^3", "mg/l", 1e3), ("kg/m^3", "g/L", 1.0), ("kg/m^3", "g/m^3", 1e3),
            ("kg/m^3", "g/m3", 1e3), ("kg/m^3", "ug/m^3", 1e9), ("kg/m^3", "mg/m^3", 1e6),
            ("m^3/s", "m^3/day", 86400.0), ("m^3/s", "m3/d", 86400.0), ("m^3/s", "L/s", 1e3), ("m^3/s", "m^3/h", 3600.0),
            ("m^3/s", "ML/day", 86.4), ("m^3/s", "m3/hr", 3600.0),
            ("s", "min", 1.0 / 60.0), ("s", "minutes", 1.0 / 60.0), ("s", "h", 1.0 / 3600.0), ("s", "hours", 1.0 / 3600.0),
            ("s", "hr", 1.0 / 3600.0), ("s", "days", 1.0 / 86400.0), ("s", "d", 1.0 / 86400.0),
            ("1/s", "1/day", 86400.0), ("1/s", "per day", 86400.0), ("1/s", "/d", 86400.0), ("1/s", "1/h", 3600.0),
            ("1/s", "per hour", 3600.0), ("1/s", "d^-1", 86400.0),
            ("m", "km", 1e-3), ("m", "cm", 100.0), ("m", "mm", 1e3), ("m", "ft", 1.0 / 0.3048),
            ("m^2", "km^2", 1e-6), ("m^2", "cm^2", 1e4), ("m^2", "m2", 1.0),
            ("m^3", "L", 1e3), ("m^3", "ML", 1e-3), ("m^3", "m3", 1.0), ("m^3", "litres", 1e3),
            ("Pa", "kPa", 1e-3), ("Pa", "atm", 1.0 / 101325.0), ("Pa", "bar", 1e-5),
            ("kg/s", "kg/day", 86400.0), ("kg/s", "kg/d", 86400.0), ("kg/s", "t/day", 86.4),
            ("kg/s", "tonnes per day", 86.4), ("kg/s", "g/s", 1e3), ("kg/s", "kg/h", 3600.0),
            ("m/s", "m/h", 3600.0), ("m/s", "m/day", 86400.0), ("m/s", "m/d", 86400.0), ("m/s", "cm/s", 100.0),
            ("m/s", "km/h", 3.6), ("m/s", "mm/s", 1e3),
            ("kg", "g", 1e3), ("kg", "t", 1e-3), ("kg", "tonnes", 1e-3), ("kg/mol", "g/mol", 1e3),
            ("Pa*s", "mPa*s", 1e3), ("Pa*s", "mPa s", 1e3), ("W", "kW", 1e-3),
        ];
        let mut bad = vec![];
        for (si, u, f) in table {
            for sep in [" ", ""] {
                if sep.is_empty() && (u.starts_with(['/', 'd', 'h', 't']) || u.contains(' ')) {
                    continue; // tools/v2_units.py writes these only with a space
                }
                let q = format!("value 12.5{sep}{u} here");
                let c = candidates(&q);
                let text = c.first().map(|(s, e)| slice(&q, *s, *e)).unwrap_or_default();
                if text != format!("12.5{sep}{u}") {
                    bad.push(format!("{u:?}: candidate {text:?}"));
                    continue;
                }
                let tidy = everyday_unit(&text, false);
                let unit = tidy.split_once(' ').map(|x| x.1).unwrap_or("");
                match (parse_unit(unit), parse_unit(si)) {
                    (Ok(a), Ok(b)) => {
                        if a.dim != b.dim || ((b.factor / a.factor) / f - 1.0).abs() > 1e-9 {
                            bad.push(format!("{u:?} -> {unit:?}: factor {} want {f}", b.factor / a.factor));
                        }
                    }
                    (e1, e2) => bad.push(format!("{u:?} -> {unit:?}: {:?} / {:?}", e1.err(), e2.err())),
                }
            }
        }
        // temperature: "°C" and "C" with the temperature flag
        for u in ["°C", " °C", " degC", " C"] {
            let q = format!("at 21.5{u} now");
            let text = candidates(&q).first().map(|(s, e)| slice(&q, *s, *e)).unwrap_or_default();
            let tidy = everyday_unit(&text, true);
            let unit = tidy.split_once(' ').map(|x| x.1).unwrap_or("");
            if parse_unit(unit).map(|x| (x.offset - 273.15).abs() > 1e-9).unwrap_or(true) {
                bad.push(format!("{u:?}: {text:?} -> {unit:?}"));
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
}
