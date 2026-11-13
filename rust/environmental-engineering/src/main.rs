//! environmental-engineering: the command line (ax_engine::cli: list, info, solve, batch, catalog, --version).
//! With the `lang` feature: `environmental-engineering lang <model folder>` answers one question per stdin line with
//! the 1.6 language layer (ax_lang::answer), one JSON line each; `lang none` answers with the documented reader alone.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    #[cfg(feature = "lang")]
    if args.first().map(|a| a == "lang").unwrap_or(false) {
        use std::io::BufRead;
        let dir = std::path::PathBuf::from(args.get(1).cloned().unwrap_or_else(|| "lang".into()));
        let t0 = std::time::Instant::now();
        // "none": the documented reader alone, as the apps without a language package answer (release 1.5.0)
        let lang = if dir.as_os_str() == "none" {
            None
        } else {
            match ax_lang::Lang::load(&dir) {
                Ok(l) => Some(l),
                Err(e) => {
                    eprintln!("cannot load the language model from {}: {e}", dir.display());
                    std::process::exit(2);
                }
            }
        };
        eprintln!("language model loaded in {:.2} s", t0.elapsed().as_secs_f64());
        let p = environmental_engineering_ai999x::pack();
        for line in std::io::stdin().lock().lines() {
            let Ok(q) = line else { break };
            if q.trim().is_empty() {
                continue;
            }
            let t = std::time::Instant::now();
            let mut v = ax_lang::answer(p, lang.as_ref(), &q);
            v["question"] = serde_json::json!(q);
            v["ms"] = serde_json::json!((t.elapsed().as_secs_f64() * 1000.0).round());
            println!("{}", v);
        }
        return;
    }
    std::process::exit(ax_engine::cli::run(environmental_engineering_ai999x::pack(), "environmental-engineering", &args));
}
