//! The calculations worked out by rule (ax-lang derive.rs, recipe A21) on this model's capabilities: each case is a
//! defect found while building it (on spent held-out files and validation), kept so it never comes back. A case reads
//! the expected capability, asked output and inputs; a refusal case must not be answered.
#![cfg(feature = "lang")]

use ax_lang::derive::derive;
use environmental_engineering_ai999x::pack;

fn reads(q: &str, cap: &str, asked: &str, inputs: &[(&str, &str)]) {
    let v = derive(pack(), q).unwrap_or_else(|why| panic!("refused: {why}\n  {q}"));
    let r = &v["request"];
    assert_eq!(r["capability"], cap, "{q}");
    assert_eq!(r["target"], asked, "{q}");
    for (k, want) in inputs {
        let got = r["inputs"][*k].as_str().map(String::from).unwrap_or_else(|| r["inputs"][*k].to_string());
        assert_eq!(got.replace(' ', ""), want.replace(' ', ""), "{k} in {q}");
    }
}

/// Answered with exactly this reading, or refused; never another reading.
fn reads_or_refuses(q: &str, cap: &str, asked: &str) {
    if let Ok(v) = derive(pack(), q) {
        assert_eq!(v["request"]["capability"], cap, "{q}");
        assert_eq!(v["request"]["target"], asked, "{q}");
    }
}

fn refuses(q: &str) {
    if let Ok(v) = derive(pack(), q) {
        panic!("answered {:?}\n  {q}", v["request"]);
    }
}

#[test]
fn a_label_after_a_value_belongs_to_it() {
    // "40 m off the centreline" is y, "1.5 m above ground" is z (the receptor word sits before 40 m)
    reads("A stack releases 95 g/s, wind 4.2 m/s at stack height, sigma y 160 m and sigma z 74 m at the receptor, effective \
           height 65 m. The receptor sits 40 m off the centreline at 1.5 m above ground. Concentration there in ug/m3?",
          "enveng_gaussian_plume_concentration", "C_ugm3", &[("y", "40 m"), ("z", "1.5 m")]);
    reads("my well water has 60 mg/L calcium and 18 mg/L magnesium, is it classed as soft or hard?",
          "enveng_water_hardness", "classification", &[("Ca", "60 mg/L"), ("Mg", "18 mg/L")]);
}

#[test]
fn numbered_inputs_follow_the_written_order_unless_one_is_asked_at() {
    reads("The anemometer at 10 m reads 3.5 m/s. With a power law exponent of 0.25, what's the wind speed up at 120 m?",
          "enveng_wind_power_law", "u2", &[("z1", "10 m"), ("z2", "120 m")]);
    reads("Estimate the wind speed at 162 m given 6.81 m/s at 8.34 m and a wind profile exponent of 0.405.",
          "enveng_wind_power_law", "u2", &[("z1", "8.34 m"), ("z2", "162 m")]);
}

#[test]
fn the_asked_result_is_named_by_the_question_not_by_a_value_label() {
    // hypochlorite is not hypochlorous; "a pKa of 7.6" gives the pKa, it does not ask for it
    reads_or_refuses("At pH 8.4 with a pKa of 7.6, what fraction of the free chlorine is hypochlorite ion?",
                     "enveng_hypochlorous_acid_fraction", "alpha_OCl");
    // "water flow ... air flow" label values; the packing depth is asked
    reads("what's the packing depth for water flow 0.0294 m^3/s, air flow 0.0858 m^3/s, Henry 0.447, influent 0.00179 kg/m^3, \
           effluent 1.380e-4 kg/m^3, HTU 1.29 m?", "enveng_air_stripping_tower", "Z", &[]);
    // "1.029atm" names no result at standard conditions
    reads_or_refuses("what's the methane flow from COD removed 3930 kg/d, biomass 0.00448 kg/s, gas at 299.2 K and 1.029atm?",
                     "enveng_anaerobic_methane_yield", "Q_CH4");
}

#[test]
fn a_look_alike_formula_never_answers_for_the_one_asked() {
    // an ESP and an ideal settling basin share the formula; the ESP reading is unclear (efficiency or penetration?)
    // only if the synonyms fail, and then the basin must not answer in its place
    let v = derive(pack(), "An electrostatic precipitator has 4500 m2 of plates, drift velocity 0.12 m/s and handles 150 m3/s \
                            of flue gas. What fraction of the dust slips through?");
    if let Ok(v) = v {
        assert_eq!(v["request"]["capability"], "enveng_esp_efficiency");
        assert_eq!(v["request"]["target"], "penetration");
    }
    // a drag coefficient is never answered with the Stokes velocity
    let v = derive(pack(), "Sand grain 0.6 mm, density 2500 kg/m3, settling in water of 1000 kg/m3 with viscosity 1.31 mPa*s. \
                            What's the drag coefficient at terminal velocity?");
    if let Ok(v) = v {
        assert_eq!(v["request"]["capability"], "enveng_terminal_settling_velocity");
    }
}

#[test]
fn a_value_described_as_a_result_is_not_computed_again() {
    let v = derive(pack(), "ultimate BOD of the waste is 280 mg/L and k is 0.23 per day. how much BOD is still left, not yet \
                            exerted, after 5 days?");
    if let Ok(v) = v {
        assert_ne!(v["request"]["capability"], "enveng_ultimate_bod");
    }
}

#[test]
fn lists_go_into_list_inputs() {
    // (the wording check refuses "oxygen transfer coefficient" while KLa's meaning is only "KLa": never r2)
    reads_or_refuses("Reaeration data, times 1, 9, 13, 19, 22, 28, 29 and DO 2.81, 7.49, 7.96, 8.08, 8.08, 8.08, 8.08, \
                      Cs 0.00833 kg/m^3. need the oxygen transfer coefficient", "enveng_kla_regression", "KLa");
    reads("Operator is exposed to 88, 92, 97 dBA for 4, 2, 1 hours. What is the time weighted average level?",
          "enveng_occupational_noise_dose", "TWA", &[]);
}

#[test]
fn questions_from_other_fields_are_refused() {
    refuses("A small car of mass m = 1,200 kg is traveling at a constant speed of 20 m/s around a flat, circular track with a \
             radius of r = 50 m. What is the magnitude of the car's centripetal acceleration?");
    refuses("my car does 30 miles per gallon and petrol is 1.45 a litre, how much for 200 miles?");
    refuses("I ran 5 km in 25 minutes, what was my pace?");
}

/// A two-step reading (recipe A22): the first calculation, the result passed on, and the final calculation.
fn reads_in_two_steps(q: &str, first: &str, passed: &str, cap: &str, asked: &str) -> serde_json::Value {
    let v = derive(pack(), q).unwrap_or_else(|why| panic!("refused: {why}\n  {q}"));
    assert_eq!(v["steps"][0]["capability"], first, "{q}");
    assert_eq!(v["steps"][0]["passed_to"], passed, "{q}");
    assert_eq!(v["request"]["capability"], cap, "{q}");
    assert_eq!(v["request"]["target"], asked, "{q}");
    assert_eq!(v["outcome"], "SOLVED", "{q}");
    v
}

fn value_of(v: &serde_json::Value, sym: &str) -> f64 {
    v["values"].as_array().and_then(|a| a.iter().find(|x| x[0] == sym)).and_then(|x| x[1].as_f64())
        .unwrap_or_else(|| panic!("no {sym} in {v}"))
}

#[test]
fn a_quantity_no_value_gives_is_worked_out_first() {
    // saturation DO from the temperature ("fresh water": salinity 0 by the contract's own wording), then the sag
    let v = reads_in_two_steps("A fresh water river at 20 °C receives a waste; just below the outfall the mixed ultimate BOD is \
        12 mg/L and the oxygen deficit is 1.5 mg/L. The deoxygenation rate constant is 0.25 1/d and the reaeration rate \
        constant 0.6 1/d. What is the minimum dissolved oxygen downstream?",
        "enveng_do_saturation", "DOsat", "enveng_streeter_phelps_critical_point", "DO_min");
    assert!((value_of(&v, "DO_min_mgL") - 6.0229).abs() < 1e-3);
    // the SRT from the tank and wasting data, then the sludge production (in kg/d: the unit variant asked)
    let v = reads_in_two_steps("An aeration tank of 1500 m3 holds an MLVSS of 2500 mg/L. Waste sludge is drawn at 40 m3/d with \
        8000 mg/L VSS, and the effluent of 3960 m3/d carries 15 mg/L VSS. The influent flow is 4000 m3/d with 200 mg/L BOD \
        and the effluent substrate is 10 mg/L; Y = 0.5 and kd = 0.06 1/d. What is the biomass production in kg/d?",
        "enveng_solids_retention_time", "SRT", "enveng_sludge_production", "Px_kg_per_day");
    assert!((value_of(&v, "Px_kg_per_day") - 238.537).abs() < 1e-2);
    // the effective stack height from the plume rise; the wind speed feeds both steps
    let v = reads_in_two_steps("A 60 m stack (inside diameter 2 m) releases gas at 15 m/s and 400 K into air at 290 K and 1 atm; \
        the wind at the stack top is 5 m/s. It emits 80 g/s of SO2. Under stability class C in rural terrain, what is the \
        concentration 1500 m downwind at a receptor height of 0 m and a crosswind distance of 0 m?",
        "enveng_holland_plume_rise", "H", "enveng_plume_concentration_stability", "C");
    assert_eq!(v["steps"][0]["inputs"]["u"], "5 m/s");
    assert!((v["steps"][0]["result"]["value"].as_f64().unwrap() - 77.961).abs() < 1e-2);
}

#[test]
fn a_fraction_is_not_asked_by_an_everyday_synonym() {
    // "what's left" of 20 mg/L is the concentration, not the fraction remaining (spent held-out 2026-11-09b c033)
    reads_or_refuses("Starting at 20 mg/L just below the outfall, decay rate 0.3 /d, river velocity 0.25 m/s - what's left \
                      15 km downstream?", "enveng_river_first_order_decay", "C_mgL");
    reads("C0 20 mg/L, k 0.3 /d, u 0.25 m/s: what fraction remains after 15 km?", "enveng_river_first_order_decay",
          "fraction_remaining", &[("x", "15 km")]);
}

#[test]
fn the_unit_asked_picks_the_variant_even_with_more_words_after_it() {
    reads("A landfill accepts 500 t/day for 20 years, L0 0.12 m3/kg, k 0.04 per year. Methane generation in m3 per year at \
           25 years after opening?", "enveng_landfill_gas_generation", "Q_CH4_m3_per_year", &[("t", "25 yr")]);
}

#[test]
fn chains_with_shared_values_and_listed_links() {
    // (spent held-out 2026-11-10e, two-step rows) "24 degrees C" is a temperature; the DO saturation feeds the sag
    reads_in_two_steps("River at 24 degrees C, fresh water (salinity 0 g/kg). Below the outfall kd = 0.35 /d, ka = 0.8 /d, \
        mixed ultimate BOD 18 mg/L, initial deficit 1.5 mg/L. What is the minimum dissolved oxygen?",
        "enveng_do_saturation", "DOsat", "enveng_streeter_phelps_critical_point", "DO_min");
    // the rate constant corrected to the water temperature is the BOD rate constant (links.json)
    reads_in_two_steps("k20 = 0.2 d-1, theta 1.047, water at 26 degrees C. L0 is 25 mg/L. how much BOD remains after 4 days?",
        "enveng_rate_constant_temperature", "k", "enveng_bod_exerted", "L_t");
    // the mass load of a discharge is the lake's pollutant input
    let v = reads_in_two_steps("treatment plant puts 25 ML/day at 4 mg/L of a conservative salt into a small reservoir: volume \
        2e6 m3, outflow 1.5 m3/s, no decay (0 per day). steady state concentration?",
        "enveng_pollutant_load", "W", "enveng_lake_steady_state", "C");
    assert!((value_of(&v, "C") - 7.716e-4).abs() < 1e-6);
    // "sigma_y 120 m" names sigma y; the wind speed feeds the plume rise and the dispersion
    let v = reads_in_two_steps("A 60 m stack releases 80 g/s of sulfur dioxide. The gas leaves at 15 m/s through a 2 m diameter \
        at 420 K; the air is 290 K and the wind 5 m/s. At the receptor sigma_y is 120 m and sigma_z 60 m. What is the \
        ground-level centreline concentration?", "enveng_briggs_plume_rise", "H", "enveng_ground_centerline_concentration", "C");
    assert_eq!(v["request"]["inputs"]["sigma_y"], "120 m");
    assert_eq!(v["steps"][0]["inputs"]["u"], "5 m/s");
}

#[test]
fn a_calculation_missing_an_input_asks_for_it_and_nothing_else_does() {
    // (recipe A24) one clear calculation, every value named, the effluent substrate not given: ask for S, no number
    let v = derive(pack(), "With an SRT of 10 d, a flow of 4000 m3/d, influent BOD 200 mg/L, Y = 0.5 and kd = 0.06 1/d, \
                            what is the biomass production?").expect("asks");
    assert_eq!(v["outcome"], "NEEDS_INPUT");
    assert_eq!(v["capability"], "enveng_sludge_production");
    assert_eq!(v["needs"], serde_json::json!(["S"]));
    assert!(v.get("value").is_none_or(|x| x.is_null()));
    // nothing of a calculation's name in the question: no question asked (spent held-out u071)
    refuses("my bike wheel is 700 mm across, how far does it go per turn");
}
