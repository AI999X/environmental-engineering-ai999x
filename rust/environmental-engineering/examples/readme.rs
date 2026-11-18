//! Environmental Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p environmental-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&environmental_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "enveng_lapse_rate_stability", "inputs": {"T1": "20 degC", "z1": "0 m", "T2": "14 degC", "z2": "500 m"}}));
    println!();
    show(json!({"key": "enveng_volatile_solids_reduction", "inputs": {"VS_in": "0.75", "VS_out": "0.55"}}));
    println!();
    show(json!({"key": "enveng_population_projection", "inputs": {"t1": "2000", "P1": "50000", "t2": "2020", "P2": "70000", "t": "2040"}}));
    println!();
    show(json!({"key": "enveng_henry_law_equilibrium", "inputs": {"p": "0.21 atm", "H": "0.79 atm*m^3/mol", "M": "31.9988 g/mol"}}));
    println!();
    show(json!({"key": "enveng_darcy_seepage_velocity", "inputs": {"K": "10 m/day", "i": "0.002", "A": "500 m^2", "n_e": "0.25"}}));
    println!();
    show(json!({"key": "enveng_hazen_williams", "inputs": {"C": "100", "D": "0.3 m", "S": "0.002"}}));
    println!();
    show(json!({"key": "enveng_rational_method", "inputs": {"C": "0.6", "i": "50 mm/h", "A": "0.25 km^2"}}));
    println!();
    show(json!({"key": "enveng_decibel_addition", "inputs": {"L": [85, 85]}}));
    println!();
    show(json!({"key": "enveng_first_order_decay", "inputs": {"C0": "100 mg/L", "k": "0.2 1/day", "t": "120 h"}}));
    println!();
    show(json!({"key": "enveng_chronic_daily_intake_risk", "inputs": {"C": "0.05 mg/L", "IR": "2 L/day", "BW": "70 kg", "RfD": "0.02"}}));
    println!();
    show(json!({"key": "enveng_landfill_compaction_ratio", "inputs": {"rho_i": "120 kg/m^3", "rho_c": "600 kg/m^3"}}));
    println!();
    show(json!({"key": "enveng_activated_sludge_cstr_design", "inputs": {"Y": "0.5", "k": "5 1/day", "Ks": "60 mg/L", "kd": "0.06 1/day", "SRT": "10 day", "HRT": "6 h", "S0": "250 mg/L"}}));
    println!();
    show(json!({"key": "enveng_bod_exerted", "inputs": {"L0": "250 mg/L", "k": "0.23 1/day", "t": "5 day"}}));
    println!();
    show(json!({"key": "enveng_freundlich_carbon_dose", "inputs": {"C0": "10 mg/L", "Ce": "1 mg/L", "K": "50", "n": "2"}}));
    println!();
    show(json!({"key": "enveng_box_model", "inputs": {"q": "4 ug/(m^2*s)", "L": "15000 m", "u": "3 m/s", "Hm": "600 m", "Cb": "20 ug/m^3", "C0": "20 ug/m^3", "t": "2 h"}}));
}
