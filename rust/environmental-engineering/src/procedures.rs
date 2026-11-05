//! environmental_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/environmental_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "enveng_activated_sludge_cstr_design",
    "enveng_aeration_do_curve",
    "enveng_aeration_oxygen_requirement",
    "enveng_air_stripping_tower",
    "enveng_anaerobic_methane_yield",
    "enveng_background_noise_correction",
    "enveng_backwash_bed_expansion",
    "enveng_bod_dilution_test",
    "enveng_bod_exerted",
    "enveng_bod_rate_fit",
    "enveng_box_model",
    "enveng_briggs_dispersion_coefficients",
    "enveng_briggs_plume_rise",
    "enveng_briggs_stable_plume_rise",
    "enveng_chick_law",
    "enveng_chick_watson",
    "enveng_chlorine_dose_feed",
    "enveng_ct_log_inactivation",
    "enveng_cyclone_lapple",
    "enveng_day_night_level",
    "enveng_decibel_addition",
    "enveng_detention_time",
    "enveng_disinfection_ct",
    "enveng_do_saturation",
    "enveng_dulong_heating_value",
    "enveng_equivalent_sound_level",
    "enveng_esp_area",
    "enveng_esp_efficiency",
    "enveng_field_oxygen_transfer",
    "enveng_filter_clean_bed_headloss",
    "enveng_filtration_rate",
    "enveng_food_to_microorganism_ratio",
    "enveng_freundlich_carbon_dose",
    "enveng_freundlich_fit",
    "enveng_gaussian_plume_concentration",
    "enveng_ground_centerline_concentration",
    "enveng_henry_constant_conversion",
    "enveng_henry_law_equilibrium",
    "enveng_henry_temperature_dependence",
    "enveng_holland_plume_rise",
    "enveng_hrt_organic_loading",
    "enveng_hypochlorous_acid_fraction",
    "enveng_ideal_basin_removal",
    "enveng_kla_regression",
    "enveng_kla_two_point",
    "enveng_lake_steady_state",
    "enveng_lake_transient",
    "enveng_landfill_gas_generation",
    "enveng_landfill_life",
    "enveng_langmuir_carbon_dose",
    "enveng_langmuir_fit",
    "enveng_log_removal",
    "enveng_mass_concentration_to_ppm",
    "enveng_max_ground_concentration_briggs",
    "enveng_max_ground_concentration_ratio",
    "enveng_membrane_mass_balance",
    "enveng_minimum_srt",
    "enveng_mixing_power",
    "enveng_mixing_several_streams",
    "enveng_mixing_two_streams",
    "enveng_monod_growth_rate",
    "enveng_noise_distance_attenuation",
    "enveng_nrc_trickling_filter",
    "enveng_occupational_noise_dose",
    "enveng_osmotic_pressure",
    "enveng_overflow_rate_circular",
    "enveng_oxygen_sag_distance",
    "enveng_plume_concentration_stability",
    "enveng_plume_mixing_lid",
    "enveng_pollutant_load",
    "enveng_ppm_to_mass_concentration",
    "enveng_rate_constant_temperature",
    "enveng_reaeration_coefficient",
    "enveng_return_sludge_ratio",
    "enveng_river_first_order_decay",
    "enveng_sedimentation_basin_sizing",
    "enveng_settling_chamber_efficiency",
    "enveng_sludge_production",
    "enveng_sludge_thickening",
    "enveng_sludge_volume",
    "enveng_sludge_volume_index",
    "enveng_so2_emission",
    "enveng_solid_waste_generation",
    "enveng_solids_retention_time",
    "enveng_sound_power_to_pressure",
    "enveng_stability_class",
    "enveng_stack_plume_ground_concentration",
    "enveng_stokes_settling_velocity",
    "enveng_streeter_phelps_critical_point",
    "enveng_streeter_phelps_deficit",
    "enveng_terminal_settling_velocity",
    "enveng_theoretical_oxygen_demand",
    "enveng_two_film_transfer",
    "enveng_ultimate_bod",
    "enveng_velocity_gradient",
    "enveng_waste_moisture_content",
    "enveng_water_hardness",
    "enveng_weir_loading_rate",
    "enveng_wind_power_law",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "enveng_activated_sludge_cstr_design" => {
            let r = f_lawrence_mccarty(match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SRT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HRT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_33(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_aeration_do_curve" => {
            let r = f_aeration_curve(match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("KLa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_85(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_aeration_oxygen_requirement" => {
            let r = f_oxygen_demand(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_air_stripping_tower" => {
            let r = f_air_stripping(match v.get("Qw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HTU") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_201(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_anaerobic_methane_yield" => {
            let r = f_methane_yield(match v.get("COD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_background_noise_correction" => {
            let r = f_db_subtract(match v.get("Lt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lb") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_298(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_backwash_bed_expansion" => {
            let r = f_backwash_expansion(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_357(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_bod_dilution_test" => {
            let r = f_bod_dilution(match v.get("D1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("B2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_421(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_bod_exerted" => {
            let r = f_bod_exerted(match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_471(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_bod_rate_fit" => {
            let r = f_bod_rate_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_568(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_box_model" => {
            let r = f_box_model(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_618(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_briggs_dispersion_coefficients" => {
            let r = f_briggs_sigmas(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_674(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_briggs_plume_rise" => {
            let r = f_briggs_rise(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_728(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_briggs_stable_plume_rise" => {
            let r = f_briggs_stable_rise(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dtheta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_780(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chick_law" => {
            let r = f_chick_first_order(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_830(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chick_watson" => {
            let r = f_chick_watson(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_883(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chlorine_dose_feed" => {
            let r = f_chlorine_dose(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("residual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("purity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ct_log_inactivation" => {
            let r = f_ct_inactivation(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CT999") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_991(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_cyclone_lapple" => {
            let r = f_cyclone_lapple(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ne") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1037(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_day_night_level" => {
            let r = f_ldn(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("metric") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_decibel_addition" => {
            let r = f_db_add(match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1146(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_detention_time" => {
            let r = f_detention_time(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_disinfection_ct" => {
            let r = f_ct_value(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("BF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_do_saturation" => {
            let r = f_do_saturation(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1290(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_dulong_heating_value" => {
            let r = f_dulong(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1348(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_equivalent_sound_level" => {
            let r = f_leq(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1404(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_esp_area" => {
            let r = f_esp_area(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1460(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_esp_efficiency" => {
            let r = f_esp_efficiency(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1504(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_field_oxygen_transfer" => {
            let r = f_field_oxygen_transfer(match v.get("SOTR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CsT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs20") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Fp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1558(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_filter_clean_bed_headloss" => {
            let r = f_ergun_headloss(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1614(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_filtration_rate" => {
            let r = f_filter_rate(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1661(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_food_to_microorganism_ratio" => {
            let r = f_fm_ratio(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1705(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_freundlich_carbon_dose" => {
            let r = f_freundlich_dose(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1751(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_freundlich_fit" => {
            let r = f_freundlich_fit(match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1812(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_gaussian_plume_concentration" => {
            let r = f_plume_concentration(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1865(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ground_centerline_concentration" => {
            let r = f_ground_centerline(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1915(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_constant_conversion" => {
            let r = f_henry_convert(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("form") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1971(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_law_equilibrium" => {
            let r = f_henry_equilibrium(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2018(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_temperature_dependence" => {
            let r = f_henry_vant_hoff(match v.get("Hcp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_holland_plume_rise" => {
            let r = f_holland_rise(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_hrt_organic_loading" => {
            let r = f_hrt_olr(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2161(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_hypochlorous_acid_fraction" => {
            let r = f_hocl_fraction(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2216(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ideal_basin_removal" => {
            let r = f_removal_fraction(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_kla_regression" => {
            let r = f_kla_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2329(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_kla_two_point" => {
            let r = f_kla_two_point(match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2384(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_lake_steady_state" => {
            let r = f_lake_steady(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2434(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_lake_transient" => {
            let r = f_lake_transient(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2484(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_gas_generation" => {
            let r = f_landfill_gas(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2536(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_life" => {
            let r = f_landfill_life(match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cover") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2588(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_langmuir_carbon_dose" => {
            let r = f_langmuir_dose(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2634(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_langmuir_fit" => {
            let r = f_langmuir_fit(match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_log_removal" => {
            let r = f_log_removal(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2744(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mass_concentration_to_ppm" => {
            let r = f_mass_to_ppm(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_max_ground_concentration_briggs" => {
            let r = f_max_ground_briggs(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2876(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_max_ground_concentration_ratio" => {
            let r = f_max_ground_turner(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2920(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_membrane_mass_balance" => {
            let r = f_membrane_balance(match v.get("Qf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_minimum_srt" => {
            let r = f_min_srt(match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3026(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_power" => {
            let r = f_mixing_power(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3070(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_several_streams" => {
            let r = f_mix_many(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_two_streams" => {
            let r = f_mix_two(match v.get("Qw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3174(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_monod_growth_rate" => {
            let r = f_monod(match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3227(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_noise_distance_attenuation" => {
            let r = f_distance_attenuation(match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3280(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_nrc_trickling_filter" => {
            let r = f_nrc_trickling(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3330(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_occupational_noise_dose" => {
            let r = f_noise_dose(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_overflow_rate_circular" => {
            let r = f_overflow_rate(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_oxygen_sag_distance" => {
            let r = f_sag_distance(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3532(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_plume_concentration_stability" => {
            let r = f_plume_with_class(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_plume_mixing_lid" => {
            let r = f_plume_lid(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_pollutant_load" => {
            let r = f_pollutant_load(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3709(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ppm_to_mass_concentration" => {
            let r = f_ppm_to_mass(match v.get("ppm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3753(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_rate_constant_temperature" => {
            let r = f_k_temperature(match v.get("k20") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_reaeration_coefficient" => {
            let r = f_reaeration(match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_return_sludge_ratio" => {
            let r = f_return_ratio(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3896(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_river_first_order_decay" => {
            let r = f_river_decay(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sedimentation_basin_sizing" => {
            let r = f_sed_basin(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3990(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_settling_chamber_efficiency" => {
            let r = f_settling_chamber(match v.get("dp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4041(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_production" => {
            let r = f_sludge_production(match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SRT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4093(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_thickening" => {
            let r = f_thickening(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_volume" => {
            let r = f_sludge_volume(match v.get("Ms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ssl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_volume_index" => {
            let r = f_svi(match v.get("V30") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MLSS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_so2_emission" => {
            let r = f_so2_emission(match v.get("m_fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4309(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_solid_waste_generation" => {
            let r = f_waste_generation(match v.get("pop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_solids_retention_time" => {
            let r = f_srt(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qe") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4406(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sound_power_to_pressure" => {
            let r = f_sound_power_to_pressure(match v.get("Lw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4455(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stability_class" => {
            let r = f_stability_class(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("condition") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4513(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stack_plume_ground_concentration" => {
            let r = f_plume_chain(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stokes_settling_velocity" => {
            let r = f_stokes(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_streeter_phelps_critical_point" => {
            let r = f_sp_critical(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4691(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_streeter_phelps_deficit" => {
            let r = f_streeter_phelps(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4747(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_terminal_settling_velocity" => {
            let r = f_terminal_velocity(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4821(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_theoretical_oxygen_demand" => {
            let r = f_thod(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nitrification") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_two_film_transfer" => {
            let r = f_two_film(match v.get("kl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4945(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ultimate_bod" => {
            let r = f_ultimate_bod(match v.get("BOD_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4989(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_velocity_gradient" => {
            let r = f_velocity_gradient(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5035(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_waste_moisture_content" => {
            let r = f_waste_moisture(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5091(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_water_hardness" => {
            let r = f_hardness(match v.get("Ca") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5141(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_weir_loading_rate" => {
            let r = f_weir_loading(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5185(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_wind_power_law" => {
            let r = f_wind_power_law(match v.get("u1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5232(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_AW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("C"), Py::Float(12.0107)), (Py::str("H"), Py::Float(1.00794)), (Py::str("O"), Py::Float(15.9994)), (Py::str("N"), Py::Float(14.0067)), (Py::str("S"), Py::Float(32.065)), (Py::str("Ca"), Py::Float(40.078)), (Py::str("Mg"), Py::Float(24.305))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_DAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(86400.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HOUR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_O2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31.9988)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P_ATM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(101325.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_W_MOLAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(55344.59)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_GAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(273.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YEAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BRIGGS_RURAL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("a"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.22), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.2), Py::Float(0.0), Py::Float(1.0)])])), (Py::str("b"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.16), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.12), Py::Float(0.0), Py::Float(1.0)])])), (Py::str("c"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.11), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.08), Py::Float(0.0002), Py::Float(-0.5)])])), (Py::str("d"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.08), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.06), Py::Float(0.0015), Py::Float(-0.5)])])), (Py::str("e"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.06), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.03), Py::Float(0.0003), Py::Float(-1.0)])])), (Py::str("f"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.04), Py::Float(0.0001), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.016), Py::Float(0.0003), Py::Float(-1.0)])]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BRIGGS_URBAN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("a"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.32), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.24), Py::Float(0.001), Py::Float(0.5)])])), (Py::str("b"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.32), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.24), Py::Float(0.001), Py::Float(0.5)])])), (Py::str("c"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.22), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.2), Py::Float(0.0), Py::Float(1.0)])])), (Py::str("d"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.16), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.14), Py::Float(0.0003), Py::Float(-0.5)])])), (Py::str("e"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.11), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.08), Py::Float(0.0015), Py::Float(-0.5)])])), (Py::str("f"), Py::tuple(vec![Py::tuple(vec![Py::Float(0.11), Py::Float(0.0004), Py::Float(-0.5)]), Py::tuple(vec![Py::Float(0.08), Py::Float(0.0015), Py::Float(-0.5)])]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CLASSES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("a"), Py::str("b"), Py::str("c"), Py::str("d"), Py::str("e"), Py::str("f")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__HFORMS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("pa_m3_per_mol"), Py::str("atm_m3_per_mol"), Py::str("dimensionless"), Py::str("atm"), Py::str("mol_per_m3_pa"), Py::str("m_per_atm")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__REAER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("o_connor_dobbins"), Py::tuple(vec![Py::Float(3.93), Py::Float(0.5), Py::Float(1.5)])), (Py::str("churchill"), Py::tuple(vec![Py::Float(5.026), Py::Float(1.0), Py::Float(1.67)])), (Py::str("owens_gibbs"), Py::tuple(vec![Py::Float(5.32), Py::Float(0.67), Py::Float(1.85)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__STAB_ALL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("strong"), Py::str("moderate"), Py::str("slight"), Py::str("overcast"), Py::str("night_cloudy"), Py::str("night_clear")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4781 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t4781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t4781[0].clone();
    v_fb = t4781[1].clone();
    if ({ let t4782 = v_fa.clone(); Py::Bool({ let t4783 = Py::Int(0i64); if py::eq(&t4782, &t4783) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4784 = v_fb.clone(); Py::Bool({ let t4785 = Py::Int(0i64); if py::eq(&t4784, &t4785) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t4786 = { let t4788 = v_fa.clone(); Py::Bool({ let t4789 = Py::Int(0i64); if py::gt(&t4788, &t4789)? { true } else { false } }) }; Py::Bool({ let t4787 = { let t4790 = v_fb.clone(); Py::Bool({ let t4791 = Py::Int(0i64); if py::gt(&t4790, &t4791)? { true } else { false } }) }; if py::ne(&t4786, &t4787) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t4792 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4792;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4797 = { let t4793 = v_m.clone(); Py::Bool({ let t4794 = v_a.clone(); if py::eq(&t4793, &t4794) { true } else { false } }) }; if t4797.truthy() { t4797 } else { { let t4795 = v_m.clone(); Py::Bool({ let t4796 = v_b.clone(); if py::eq(&t4795, &t4796) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4798 = v_fm.clone(); Py::Bool({ let t4799 = Py::Int(0i64); if py::eq(&t4798, &t4799) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4800 = { let t4802 = v_fm.clone(); Py::Bool({ let t4803 = Py::Int(0i64); if py::gt(&t4802, &t4803)? { true } else { false } }) }; Py::Bool({ let t4801 = { let t4804 = v_fa.clone(); Py::Bool({ let t4805 = Py::Int(0i64); if py::gt(&t4804, &t4805)? { true } else { false } }) }; if py::eq(&t4800, &t4801) { true } else { false } }) }).truthy() {
            let t4806 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4806[0].clone();
            v_fa = t4806[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bod_L0(mut v_k: Py, mut v_t: Py, mut v_y: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_fi: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_ti: Py = Py::None;
    let mut v_yi: Py = Py::None;
    let t514 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t514[0].clone();
    v_den = t514[1].clone();
    for t515 in py::iter(&(py::zip(&[v_t.clone(), v_y.clone()])?))? {
        let t516 = (t515).items()?;
        if t516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ti = t516[0].clone();
        v_yi = t516[1].clone();
        v_fi = py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_ti.clone())?)?)?;
        v_num = py::iadd(&v_num, &py::mul(&v_yi.clone(), &v_fi.clone())?)?;
        v_den = py::iadd(&v_den, &py::mul(&v_fi.clone(), &v_fi.clone())?)?;
    }
    return Ok(py::truediv(&v_num.clone(), &v_den.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bod_h(mut v_k: Py, mut v_t: Py, mut v_y: Py) -> R {
    let mut v_L0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_ti: Py = Py::None;
    let mut v_yi: Py = Py::None;
    v_L0 = f__bod_L0(v_k.clone(), v_t.clone(), v_y.clone())?;
    v_s = Py::Float(0.0);
    for t517 in py::iter(&(py::zip(&[v_t.clone(), v_y.clone()])?))? {
        let t518 = (t517).items()?;
        if t518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ti = t518[0].clone();
        v_yi = t518[1].clone();
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::sub(&v_yi.clone(), &py::mul(&v_L0.clone(), &py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_ti.clone())?)?)?)?)?, &v_ti.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_ti.clone())?)?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__briggs(mut v_x: Py, mut v_cls: Py, mut v_terrain: Py) -> R {
    let mut v_cy: Py = Py::None;
    let mut v_cz: Py = Py::None;
    let _ = f__pos(Py::str("the downwind distance x"), Py::tuple(vec![v_x.clone()]))?;
    let _ = f__need({ let t4571 = Py::Float(100.0); Py::Bool({ let t4572 = v_x.clone(); if py::le(&t4571, &t4572)? { { let t4573 = Py::Float(10000.0); if py::le(&t4572, &t4573)? { true } else { false } } } else { false } }) }, Py::str("the Briggs coefficients hold for x between 100 m and 10 km"))?;
    let t4580 = (f__briggs_coef(v_cls.clone(), v_terrain.clone())?).items()?;
    if t4580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cy = t4580[0].clone();
    v_cz = t4580[1].clone();
    return Ok(Py::tuple(vec![f__sig_eval(v_cy.clone(), v_x.clone())?, f__sig_eval(v_cz.clone(), v_x.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__briggs_coef(mut v_cls: Py, mut v_terrain: Py) -> R {
    let _ = f__need({ let t4574 = v_cls.clone(); Py::Bool({ let t4575 = c__CLASSES()?; if py::contains(&t4575, &t4574)? { true } else { false } }) }, Py::str("the stability class must be one of A to F"))?;
    let _ = f__need({ let t4576 = v_terrain.clone(); Py::Bool({ let t4577 = Py::tuple(vec![Py::str("rural"), Py::str("urban")]); if py::contains(&t4577, &t4576)? { true } else { false } }) }, Py::str("terrain must be rural or urban"))?;
    return Ok(py::getitem(&(if ({ let t4578 = v_terrain.clone(); Py::Bool({ let t4579 = Py::str("rural"); if py::eq(&t4578, &t4579) { true } else { false } }) }).truthy() { c__BRIGGS_RURAL()? } else { c__BRIGGS_URBAN()? }), &v_cls.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__buoyancy_flux(mut v_vs: Py, mut v_d: Py, mut v_Ts: Py, mut v_Ta: Py) -> R {
    let _ = f__pos(Py::str("vs, d, Ts and Ta"), Py::tuple(vec![v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone()]))?;
    let _ = f__need({ let t4547 = v_Ts.clone(); Py::Bool({ let t4548 = v_Ta.clone(); if py::gt(&t4547, &t4548)? { true } else { false } }) }, Py::str("the stack gas must be warmer than the air (Ts > Ta) for a buoyant plume"))?;
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&c_G0()?, &v_vs.clone())?, &v_d.clone())?, &v_d.clone())?, &py::sub(&v_Ts.clone(), &v_Ta.clone())?)?, &py::mul(&Py::Float(4.0), &v_Ts.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__db_list(mut v_L: Py, mut v_name: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4432 in py::iter(&(v_L.clone()))? {
        v_v = t4432;
        let _ = f__need({ let t4436 = py::math1("isfinite", &v_v.clone())?; if !t4436.truthy() { t4436 } else { { let t4433 = py::neg(&Py::Float(50.0))?; Py::Bool({ let t4434 = v_v.clone(); if py::le(&t4433, &t4434)? { { let t4435 = Py::Float(250.0); if py::le(&t4434, &t4435)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a sound level in dB between -50 and 250".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4285 in py::iter(&(v_vals.clone()))? {
        v_v = t4285;
        let _ = f__need({ let t4287 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4287.truthy() { t4287 } else { { let t4286 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4286.truthy() { t4286 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__formula_counts(mut v_text: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_digits: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_lower: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_start: Py = Py::None;
    let mut v_upper: Py = Py::None;
    v_s = py::method(&py::method(&v_text.clone(), "strip", &[])?, "replace", &[Py::str(" "), Py::str("")])?;
    let _ = f__need({ let t4847 = py::len(&v_s.clone())?; Py::Bool({ let t4848 = Py::Int(0i64); if py::gt(&t4847, &t4848)? { true } else { false } }) }, Py::str("give a molecular formula such as C6H12O6"))?;
    let t4849 = (Py::tuple(vec![Py::str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Py::str("abcdefghijklmnopqrstuvwxyz"), Py::str("0123456789")])).items()?;
    if t4849.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_upper = t4849[0].clone();
    v_lower = t4849[1].clone();
    v_digits = t4849[2].clone();
    let t4850 = (Py::tuple(vec![Py::Int(0i64), Py::dict(vec![])])).items()?;
    if t4850.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t4850[0].clone();
    v_counts = t4850[1].clone();
    while ({ let t4867 = v_pos.clone(); Py::Bool({ let t4868 = py::len(&v_s.clone())?; if py::lt(&t4867, &t4868)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4851 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t4852 = v_upper.clone(); if py::contains(&t4852, &t4851)? { true } else { false } }) }, Py::str("the formula may contain only element symbols and counts (C, H, O, N)"))?;
        v_el = py::getitem(&v_s.clone(), &v_pos.clone())?;
        v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        if ({ let t4857 = { let t4853 = v_pos.clone(); Py::Bool({ let t4854 = py::len(&v_s.clone())?; if py::lt(&t4853, &t4854)? { true } else { false } }) }; if !t4857.truthy() { t4857 } else { { let t4855 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t4856 = v_lower.clone(); if py::contains(&t4856, &t4855)? { true } else { false } }) } } }).truthy() {
            v_el = py::add(&v_el.clone(), &py::getitem(&v_s.clone(), &v_pos.clone())?)?;
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_start = v_pos.clone();
        while ({ let t4862 = { let t4858 = v_pos.clone(); Py::Bool({ let t4859 = py::len(&v_s.clone())?; if py::lt(&t4858, &t4859)? { true } else { false } }) }; if !t4862.truthy() { t4862 } else { { let t4860 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t4861 = v_digits.clone(); if py::contains(&t4861, &t4860)? { true } else { false } }) } } }).truthy() {
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_n = (if ({ let t4863 = v_pos.clone(); Py::Bool({ let t4864 = v_start.clone(); if py::gt(&t4863, &t4864)? { true } else { false } }) }).truthy() { py::int(&py::slice(&v_s.clone(), Some((v_start.clone()).i()?), Some((v_pos.clone()).i()?), None)?)? } else { Py::Int(1i64) });
        let _ = f__need({ let t4865 = v_el.clone(); Py::Bool({ let t4866 = Py::tuple(vec![Py::str("C"), Py::str("H"), Py::str("O"), Py::str("N")]); if py::contains(&t4866, &t4865)? { true } else { false } }) }, Py::str("only C, H, O and N are allowed"))?;
        py::setitem(&v_counts.clone(), &v_el.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
    }
    let _ = f__need({ let t4873 = { let t4869 = py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?; Py::Bool({ let t4870 = Py::Int(0i64); if py::gt(&t4869, &t4870)? { true } else { false } }) }; if !t4873.truthy() { t4873 } else { { let t4871 = py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?; Py::Bool({ let t4872 = Py::Int(0i64); if py::gt(&t4871, &t4872)? { true } else { false } }) } } }, Py::str("the compound must contain carbon and hydrogen"))?;
    return Ok(Py::tuple(vec![py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("O"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("N"), Py::Int(0i64)])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t4296 = (if (v_lo_open.clone()).truthy() { { let t4288 = v_v.clone(); Py::Bool({ let t4289 = Py::Int(0i64); if py::gt(&t4288, &t4289)? { true } else { false } }) } } else { { let t4290 = v_v.clone(); Py::Bool({ let t4291 = Py::Int(0i64); if py::ge(&t4290, &t4291)? { true } else { false } }) } }); if !t4296.truthy() { t4296 } else { (if (v_hi_open.clone()).truthy() { { let t4292 = v_v.clone(); Py::Bool({ let t4293 = Py::Int(1i64); if py::lt(&t4292, &t4293)? { true } else { false } }) } } else { { let t4294 = v_v.clone(); Py::Bool({ let t4295 = Py::Int(1i64); if py::le(&t4294, &t4295)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iso_check(mut v_Ce: Py, mut v_qe: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2664 = { let t2660 = py::len(&v_Ce.clone())?; Py::Bool({ let t2661 = py::len(&v_qe.clone())?; if py::eq(&t2660, &t2661) { true } else { false } }) }; if !t2664.truthy() { t2664 } else { { let t2662 = py::len(&v_Ce.clone())?; Py::Bool({ let t2663 = Py::Int(3i64); if py::ge(&t2662, &t2663)? { true } else { false } }) } } }, Py::str("give at least three points (Ce and qe of equal length)"))?;
    for t2665 in py::iter(&(py::add(&Py::list((v_Ce.clone()).items()?), &Py::list((v_qe.clone()).items()?))?))? {
        v_v = t2665;
        let _ = f__need({ let t2668 = py::math1("isfinite", &v_v.clone())?; if !t2668.truthy() { t2668 } else { { let t2666 = v_v.clone(); Py::Bool({ let t2667 = Py::Int(0i64); if py::gt(&t2666, &t2667)? { true } else { false } }) } } }, Py::str("every concentration and loading must be positive"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    v_mx = py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?;
    v_my = py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?;
    v_sxx = py::sum(&{ let mut out = vec![]; for t2669 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t2669;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t2670 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t2670;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t2671 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2672 = (t2671).items()?;
if t2672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2672[0].clone();
v_b = t2672[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2677 = { let t2673 = v_sxx.clone(); Py::Bool({ let t2674 = Py::Int(0i64); if py::gt(&t2673, &t2674)? { true } else { false } }) }; if !t2677.truthy() { t2677 } else { { let t2675 = v_syy.clone(); Py::Bool({ let t2676 = Py::Int(0i64); if py::gt(&t2675, &t2676)? { true } else { false } }) } } }, Py::str("the data must vary"))?;
    v_slope = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::tuple(vec![v_slope.clone(), py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?, py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lm_S(mut v_Y: Py, mut v_k: Py, mut v_Ks: Py, mut v_kd: Py, mut v_SRT: Py) -> R {
    let mut v_den: Py = Py::None;
    v_den = py::sub(&py::mul(&v_SRT.clone(), &py::sub(&py::mul(&v_Y.clone(), &v_k.clone())?, &v_kd.clone())?)?, &Py::Float(1.0))?;
    let _ = f__need({ let t17 = v_den.clone(); Py::Bool({ let t18 = Py::Int(0i64); if py::gt(&t17, &t18)? { true } else { false } }) }, Py::str("the SRT is below the washout limit (SRT (Y k - kd) must exceed 1)"))?;
    return Ok(py::truediv(&py::mul(&v_Ks.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_kd.clone(), &v_SRT.clone())?)?)?, &v_den.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lsig(mut v_c: Py, mut v_x: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t2828 = (v_c.clone()).items()?;
    if t2828.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2828[0].clone();
    v_b = t2828[1].clone();
    v_p = t2828[2].clone();
    return Ok(py::add(&py::truediv(&Py::Float(1.0), &v_x.clone())?, &py::truediv(&py::mul(&v_p.clone(), &v_b.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_b.clone(), &v_x.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5117 in py::iter(&(v_vals.clone()))? {
        v_v = t5117;
        let _ = f__need({ let t5122 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5122.truthy() { t5122 } else { { let t5121 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5121.truthy() { t5121 } else { { let t5120 = py::math1("isfinite", &v_v.clone())?; if !t5120.truthy() { t5120 } else { { let t5118 = v_v.clone(); Py::Bool({ let t5119 = Py::Int(0i64); if py::ge(&t5118, &t5119)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5220 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5221 = (t5220).items()?;
if t5221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5221[0].clone();
v_v = t5221[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__plume(mut v_Q: Py, mut v_u: Py, mut v_sy: Py, mut v_sz: Py, mut v_H: Py, mut v_y: Py, mut v_z: Py) -> R {
    return Ok(py::mul(&py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_u.clone())?, &v_sy.clone())?, &v_sz.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_y.clone())?, &v_y.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sy.clone())?, &v_sy.clone())?)?)?)?, &py::add(&py::math1("exp", &py::truediv(&py::neg(&py::pow(&py::sub(&v_z.clone(), &v_H.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Float(2.0), &v_sz.clone())?, &v_sz.clone())?)?)?, &py::math1("exp", &py::truediv(&py::neg(&py::pow(&py::add(&v_z.clone(), &v_H.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Float(2.0), &v_sz.clone())?, &v_sz.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5211 in py::iter(&(v_vals.clone()))? {
        v_v = t5211;
        let _ = f__need({ let t5216 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5216.truthy() { t5216 } else { { let t5215 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5215.truthy() { t5215 } else { { let t5214 = py::math1("isfinite", &v_v.clone())?; if !t5214.truthy() { t5214 } else { { let t5212 = v_v.clone(); Py::Bool({ let t5213 = Py::Int(0i64); if py::gt(&t5212, &t5213)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5225 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5225.truthy() { t5225 } else { { let t5224 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5224.truthy() { t5224 } else { { let t5222 = v_x.clone(); Py::Bool({ let t5223 = Py::None; if py::is(&t5222, &t5223) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5226 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5226;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5229 = { let t5227 = v_x.clone(); Py::Bool({ let t5228 = Py::Int(0i64); if py::eq(&t5227, &t5228) { true } else { false } }) }; if t5229.truthy() { t5229 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5230 = v_x.clone(); Py::Bool({ let t5231 = Py::Int(0i64); if py::eq(&t5230, &t5231) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5238 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5238.truthy() { t5238 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5242 = py::not(&{ let t5239 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5239.truthy() { t5239 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5242.truthy() { t5242 } else { { let t5240 = py::len(&v_a.clone())?; Py::Bool({ let t5241 = py::len(&v_b.clone())?; if py::ne(&t5240, &t5241) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5243 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5244 = (t5243).items()?;
if t5244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5244[0].clone();
v_y = t5244[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5245 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5245.truthy() { t5245 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5246 = v_a.clone(); Py::Bool({ let t5247 = v_b.clone(); if py::eq(&t5246, &t5247) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5249: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5248 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5248[0].clone();
            v_b = t5248[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5249 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5250 = v_d.clone(); Py::Bool({ let t5251 = Py::Int(0i64); if py::eq(&t5250, &t5251) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5256 = { let t5252 = py::abs(&v_a.clone())?; Py::Bool({ let t5253 = Py::Float(1e-15); if py::lt(&t5252, &t5253)? { true } else { false } }) }; if !t5256.truthy() { t5256 } else { { let t5254 = py::abs(&v_b.clone())?; Py::Bool({ let t5255 = Py::Float(1e-15); if py::lt(&t5254, &t5255)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rouse_cd(mut v_Re: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&Py::Float(24.0), &v_Re.clone())?, &py::truediv(&Py::Float(3.0), &py::math1("sqrt", &v_Re.clone())?)?)?, &Py::Float(0.34))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sig_eval(mut v_c: Py, mut v_x: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4581 = (v_c.clone()).items()?;
    if t4581.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4581[0].clone();
    v_b = t4581[1].clone();
    v_p = t4581[2].clone();
    return Ok(py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_b.clone(), &v_x.clone())?)?, &v_p.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sp_checks(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_DOsat: Py) -> R {
    let _ = f__pos(Py::str("kd, ka, L0 and DOsat"), Py::tuple(vec![v_kd.clone(), v_ka.clone(), v_L0.clone(), v_DOsat.clone()]))?;
    let _ = f__nonneg(Py::str("the initial deficit D0"), Py::tuple(vec![v_D0.clone()]))?;
    let _ = f__need({ let t4729 = v_D0.clone(); Py::Bool({ let t4730 = v_DOsat.clone(); if py::le(&t4729, &t4730)? { true } else { false } }) }, Py::str("the initial deficit cannot exceed the saturation concentration"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sp_deficit(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_t: Py) -> R {
    if ({ let t4731 = py::abs(&py::sub(&v_ka.clone(), &v_kd.clone())?)?; Py::Bool({ let t4732 = py::mul(&Py::Float(1e-09), &v_ka.clone())?; if py::le(&t4731, &t4732)? { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::add(&py::mul(&py::mul(&v_kd.clone(), &v_L0.clone())?, &v_t.clone())?, &v_D0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_t.clone())?)?)?);
    }
    return Ok(py::add(&py::mul(&py::truediv(&py::mul(&v_kd.clone(), &v_L0.clone())?, &py::sub(&v_ka.clone(), &v_kd.clone())?)?, &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?)?)?, &py::mul(&v_D0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stokes_v(mut v_dp: Py, mut v_rho_p: Py, mut v_rho_f: Py, mut v_mu: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&c_G0()?, &py::sub(&v_rho_p.clone(), &v_rho_f.clone())?)?, &v_dp.clone())?, &v_dp.clone())?, &py::mul(&Py::Float(18.0), &v_mu.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_aeration_curve(mut v_Cs: Py, mut v_C0: Py, mut v_KLa: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("Cs and KLa"), Py::tuple(vec![v_Cs.clone(), v_KLa.clone()]))?;
    let _ = f__nonneg(Py::str("C0 and t"), Py::tuple(vec![v_C0.clone(), v_t.clone()]))?;
    let _ = f__need({ let t71 = v_C0.clone(); Py::Bool({ let t72 = v_Cs.clone(); if py::le(&t71, &t72)? { true } else { false } }) }, Py::str("the starting DO cannot exceed saturation"))?;
    v_C = py::sub(&v_Cs.clone(), &py::mul(&py::sub(&v_Cs.clone(), &v_C0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_KLa.clone())?, &v_t.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("OTR"), py::mul(&v_KLa.clone(), &py::sub(&v_Cs.clone(), &v_C.clone())?)?), (Py::str("OTR_mg_per_L_h"), py::mul(&py::mul(&py::mul(&v_KLa.clone(), &py::sub(&v_Cs.clone(), &v_C.clone())?)?, &Py::Float(1000.0))?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_air_stripping(mut v_Qw: Py, mut v_Qa: Py, mut v_H: Py, mut v_Cin: Py, mut v_Cout: Py, mut v_HTU: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_ntu: Py = Py::None;
    let mut v_ratio_min: Py = Py::None;
    let _ = f__pos(Py::str("Qw, Qa, H, Cin, Cout and HTU"), Py::tuple(vec![v_Qw.clone(), v_Qa.clone(), v_H.clone(), v_Cin.clone(), v_Cout.clone(), v_HTU.clone()]))?;
    let _ = f__need({ let t183 = v_Cout.clone(); Py::Bool({ let t184 = v_Cin.clone(); if py::lt(&t183, &t184)? { true } else { false } }) }, Py::str("the effluent concentration must be below the influent"))?;
    v_ratio_min = py::truediv(&py::sub(&v_Cin.clone(), &v_Cout.clone())?, &py::mul(&v_H.clone(), &v_Cin.clone())?)?;
    let _ = f__need({ let t185 = py::truediv(&v_Qa.clone(), &v_Qw.clone())?; Py::Bool({ let t186 = v_ratio_min.clone(); if py::gt(&t185, &t186)? { true } else { false } }) }, Py::str("the air-to-water ratio is below the minimum: the removal cannot be reached"))?;
    v_R = py::truediv(&py::mul(&v_H.clone(), &v_Qa.clone())?, &v_Qw.clone())?;
    if ({ let t187 = py::abs(&py::sub(&v_R.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t188 = Py::Float(1e-09); if py::le(&t187, &t188)? { true } else { false } }) }).truthy() {
        v_ntu = py::sub(&py::truediv(&v_Cin.clone(), &v_Cout.clone())?, &Py::Float(1.0))?;
    } else {
        v_ntu = py::mul(&py::truediv(&v_R.clone(), &py::sub(&v_R.clone(), &Py::Float(1.0))?)?, &py::math1("log", &py::truediv(&py::add(&py::mul(&py::truediv(&v_Cin.clone(), &v_Cout.clone())?, &py::sub(&v_R.clone(), &Py::Float(1.0))?)?, &Py::Float(1.0))?, &v_R.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("NTU"), v_ntu.clone()), (Py::str("Z"), py::mul(&v_HTU.clone(), &v_ntu.clone())?), (Py::str("air_to_water_min"), v_ratio_min.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_backwash_expansion(mut v_L: Py, mut v_eps: Py, mut v_vb: Py, mut v_vs: Py) -> R {
    let mut v_Le: Py = Py::None;
    let mut v_eps_e: Py = Py::None;
    let _ = f__pos(Py::str("L, vb and vs"), Py::tuple(vec![v_L.clone(), v_vb.clone(), v_vs.clone()]))?;
    let _ = f__frac(Py::str("the porosity eps"), v_eps.clone(), Py::Bool(true), Py::Bool(true))?;
    v_eps_e = py::pow(&py::truediv(&v_vb.clone(), &v_vs.clone())?, &Py::Float(0.22))?;
    let _ = f__need({ let t342 = v_eps.clone(); Py::Bool({ let t343 = v_eps_e.clone(); if py::lt(&t342, &t343)? { { let t344 = Py::Float(1.0); if py::lt(&t343, &t344)? { true } else { false } } } else { false } }) }, Py::str("the backwash rate must fluidise the bed (eps < (vb/vs)^0.22 < 1)"))?;
    v_Le = py::truediv(&py::mul(&v_L.clone(), &py::sub(&Py::Float(1.0), &v_eps.clone())?)?, &py::sub(&Py::Float(1.0), &v_eps_e.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eps_e"), v_eps_e.clone()), (Py::str("Le"), v_Le.clone()), (Py::str("expansion"), py::sub(&py::truediv(&v_Le.clone(), &v_L.clone())?, &Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bod_dilution(mut v_D1: Py, mut v_D2: Py, mut v_P: Py, mut v_B1: Py, mut v_B2: Py, mut v_f: Py) -> R {
    let mut v_BOD: Py = Py::None;
    let mut v_seed: Py = Py::None;
    let mut v_seeded: Py = Py::None;
    let _ = f__nonneg(Py::str("D1 and D2"), Py::tuple(vec![v_D1.clone(), v_D2.clone()]))?;
    let _ = f__frac(Py::str("the sample fraction P"), v_P.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t401 = v_D1.clone(); Py::Bool({ let t402 = v_D2.clone(); if py::gt(&t401, &t402)? { true } else { false } }) }, Py::str("the oxygen must fall during incubation (D1 > D2)"))?;
    v_seeded = { let mut out = vec![]; for t403 in py::iter(&(Py::tuple(vec![v_B1.clone(), v_B2.clone(), v_f.clone()])))? { let mut v_v: Py; 
v_v = t403;
out.push({ let t404 = v_v.clone(); Py::Bool({ let t405 = Py::None; if !py::is(&t404, &t405) { true } else { false } }) }); } Py::list(out) };
    let _ = f__need({ let t406 = Py::Bool((v_seeded.clone()).items()?.iter().all(|x| x.truthy())); if t406.truthy() { t406 } else { py::not(&Py::Bool((v_seeded.clone()).items()?.iter().any(|x| x.truthy()))) } }, Py::str("a seeded test needs B1, B2 and the seed fraction f together"))?;
    v_seed = Py::Float(0.0);
    if (Py::Bool((v_seeded.clone()).items()?.iter().all(|x| x.truthy()))).truthy() {
        let _ = f__nonneg(Py::str("B1 and B2"), Py::tuple(vec![v_B1.clone(), v_B2.clone()]))?;
        let _ = f__frac(Py::str("the seed fraction f"), v_f.clone(), Py::Bool(false), Py::Bool(false))?;
        v_seed = py::mul(&py::sub(&v_B1.clone(), &v_B2.clone())?, &v_f.clone())?;
    }
    v_BOD = py::truediv(&py::sub(&py::sub(&v_D1.clone(), &v_D2.clone())?, &v_seed.clone())?, &v_P.clone())?;
    let _ = f__need({ let t407 = v_BOD.clone(); Py::Bool({ let t408 = Py::Int(0i64); if py::gt(&t407, &t408)? { true } else { false } }) }, Py::str("the seed correction exceeds the oxygen used: no BOD can be stated"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("BOD"), v_BOD.clone()), (Py::str("BOD_mgL"), py::mul(&v_BOD.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bod_exerted(mut v_L0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_rem: Py = Py::None;
    let _ = f__pos(Py::str("L0 and k"), Py::tuple(vec![v_L0.clone(), v_k.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_rem = py::mul(&v_L0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("BOD_t"), py::sub(&v_L0.clone(), &v_rem.clone())?), (Py::str("L_t"), v_rem.clone()), (Py::str("BOD_t_mgL"), py::mul(&py::sub(&v_L0.clone(), &v_rem.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bod_rate_fit(mut v_t: Py, mut v_y: Py) -> R {
    let mut v_L0: Py = Py::None;
    let mut v_hs: Py = Py::None;
    let mut v_kd: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_rss: Py = Py::None;
    let mut v_ti: Py = Py::None;
    let mut v_yi: Py = Py::None;
    let _ = f__need({ let t501 = { let t497 = py::len(&v_t.clone())?; Py::Bool({ let t498 = py::len(&v_y.clone())?; if py::eq(&t497, &t498) { true } else { false } }) }; if !t501.truthy() { t501 } else { { let t499 = py::len(&v_t.clone())?; Py::Bool({ let t500 = Py::Int(3i64); if py::ge(&t499, &t500)? { true } else { false } }) } } }, Py::str("give at least three times and as many BOD values"))?;
    for t502 in py::iter(&(v_t.clone()))? {
        v_ti = t502;
        let _ = f__need({ let t505 = py::math1("isfinite", &v_ti.clone())?; if !t505.truthy() { t505 } else { { let t503 = v_ti.clone(); Py::Bool({ let t504 = Py::Int(0i64); if py::gt(&t503, &t504)? { true } else { false } }) } } }, Py::str("every time must be positive (days)"))?;
    }
    for t506 in py::iter(&(v_y.clone()))? {
        v_yi = t506;
        let _ = f__need({ let t509 = py::math1("isfinite", &v_yi.clone())?; if !t509.truthy() { t509 } else { { let t507 = v_yi.clone(); Py::Bool({ let t508 = Py::Int(0i64); if py::gt(&t507, &t508)? { true } else { false } }) } } }, Py::str("every BOD value must be positive (mg/L)"))?;
    }
    let _ = f__need({ let t510 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t511 = py::len(&v_t.clone())?; if py::eq(&t510, &t511) { true } else { false } }) }, Py::str("the times must be distinct"))?;
    v_n = Py::Int(400i64);
    v_ks = { let mut out = vec![]; for t512 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t512;
out.push(py::mul(&Py::Float(0.001), &py::pow(&Py::Float(20000.0), &py::truediv(&v_i.clone(), &v_n.clone())?)?)?); } Py::list(out) };
    v_hs = { let mut out = vec![]; for t513 in py::iter(&(v_ks.clone()))? { let mut v_k: Py; 
v_k = t513;
out.push(f__bod_h(v_k.clone(), v_t.clone(), v_y.clone())?); } Py::list(out) };
    v_roots = { let mut out = vec![]; for t519 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t519;
if ({ let t520 = { let t522 = py::getitem(&v_hs.clone(), &v_i.clone())?; Py::Bool({ let t523 = Py::Int(0i64); if py::gt(&t522, &t523)? { true } else { false } }) }; Py::Bool({ let t521 = { let t524 = py::getitem(&v_hs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t525 = Py::Int(0i64); if py::gt(&t524, &t525)? { true } else { false } }) }; if py::ne(&t520, &t521) { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    let _ = f__need({ let t526 = py::len(&v_roots.clone())?; Py::Bool({ let t527 = Py::Int(1i64); if py::eq(&t526, &t527) { true } else { false } }) }, Py::str("the data do not determine one first-order rate constant between 0.001 and 20 per day"))?;
    v_kd = f__bisect({ let v_hs = v_hs.clone(); let v_ks = v_ks.clone(); let v_n = v_n.clone(); let v_roots = v_roots.clone(); let v_t = v_t.clone(); let v_ti = v_ti.clone(); let v_y = v_y.clone(); let v_yi = v_yi.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__bod_h(v_k.clone(), v_t.clone(), v_y.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_ks.clone(), &py::getitem(&v_roots.clone(), &Py::Int(0i64))?)?, py::getitem(&v_ks.clone(), &py::add(&py::getitem(&v_roots.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, Py::Int(500i64))?;
    v_L0 = f__bod_L0(v_kd.clone(), v_t.clone(), v_y.clone())?;
    v_rss = Py::Float(0.0);
    for t554 in py::iter(&(py::zip(&[v_t.clone(), v_y.clone()])?))? {
        let t555 = (t554).items()?;
        if t555.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ti = t555[0].clone();
        v_yi = t555[1].clone();
        v_rss = py::iadd(&v_rss, &py::pow(&py::sub(&v_yi.clone(), &py::mul(&v_L0.clone(), &py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_ti.clone())?)?)?)?)?, &Py::Int(2i64))?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("k"), py::truediv(&v_kd.clone(), &c_DAY()?)?), (Py::str("k_per_day"), v_kd.clone()), (Py::str("L0"), py::mul(&v_L0.clone(), &c_MG()?)?), (Py::str("L0_mgL"), v_L0.clone()), (Py::str("rss"), v_rss.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_box_model(mut v_q: Py, mut v_L: Py, mut v_u: Py, mut v_Hm: Py, mut v_Cb: Py, mut v_C0: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Css: Py = Py::None;
    let _ = f__pos(Py::str("L, u and the mixing height Hm"), Py::tuple(vec![v_L.clone(), v_u.clone(), v_Hm.clone()]))?;
    let _ = f__nonneg(Py::str("q, Cb, C0 and t"), Py::tuple(vec![v_q.clone(), v_Cb.clone(), v_C0.clone(), v_t.clone()]))?;
    v_Css = py::add(&v_Cb.clone(), &py::truediv(&py::mul(&v_q.clone(), &v_L.clone())?, &py::mul(&v_u.clone(), &v_Hm.clone())?)?)?;
    v_C = py::add(&v_Css.clone(), &py::mul(&py::sub(&v_C0.clone(), &v_Css.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_u.clone())?, &v_t.clone())?, &v_L.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ss"), v_Css.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_briggs_rise(mut v_vs: Py, mut v_d: Py, mut v_Ts: Py, mut v_Ta: Py, mut v_u: Py, mut v_hs: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_dh: Py = Py::None;
    let mut v_xf: Py = Py::None;
    v_F = f__buoyancy_flux(v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone())?;
    let _ = f__pos(Py::str("u"), Py::tuple(vec![v_u.clone()]))?;
    let _ = f__nonneg(Py::str("the stack height hs"), Py::tuple(vec![v_hs.clone()]))?;
    v_xf = (if ({ let t4555 = v_F.clone(); Py::Bool({ let t4556 = Py::Float(55.0); if py::lt(&t4555, &t4556)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(49.0), &py::pow(&v_F.clone(), &Py::Float(0.625))?)? } else { py::mul(&Py::Float(119.0), &py::pow(&v_F.clone(), &Py::Float(0.4))?)? });
    v_dh = py::truediv(&py::mul(&py::mul(&Py::Float(1.6), &py::pow(&v_F.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?, &py::pow(&v_xf.clone(), &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?, &v_u.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("x_f"), v_xf.clone()), (Py::str("dh"), v_dh.clone()), (Py::str("H"), py::add(&v_hs.clone(), &v_dh.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_briggs_sigmas(mut v_x: Py, mut v_cls: Py, mut v_terrain: Py) -> R {
    let mut v_sy: Py = Py::None;
    let mut v_sz: Py = Py::None;
    let t661 = (f__briggs(v_x.clone(), v_cls.clone(), v_terrain.clone())?).items()?;
    if t661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t661[0].clone();
    v_sz = t661[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_y"), v_sy.clone()), (Py::str("sigma_z"), v_sz.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_briggs_stable_rise(mut v_vs: Py, mut v_d: Py, mut v_Ts: Py, mut v_Ta: Py, mut v_u: Py, mut v_dtheta: Py, mut v_hs: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_dh: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_F = f__buoyancy_flux(v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone())?;
    let _ = f__pos(Py::str("u and the potential-temperature gradient dtheta"), Py::tuple(vec![v_u.clone(), v_dtheta.clone()]))?;
    let _ = f__nonneg(Py::str("the stack height hs"), Py::tuple(vec![v_hs.clone()]))?;
    v_s = py::mul(&py::truediv(&c_G0()?, &v_Ta.clone())?, &v_dtheta.clone())?;
    v_dh = py::mul(&Py::Float(2.6), &py::pow(&py::truediv(&v_F.clone(), &py::mul(&v_u.clone(), &v_s.clone())?)?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("s"), v_s.clone()), (Py::str("dh"), v_dh.clone()), (Py::str("H"), py::add(&v_hs.clone(), &v_dh.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chick_first_order(mut v_N0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("N0 and k"), Py::tuple(vec![v_N0.clone(), v_k.clone()]))?;
    let _ = f__nonneg(Py::str("the contact time t"), Py::tuple(vec![v_t.clone()]))?;
    v_s = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N0.clone(), &v_s.clone())?), (Py::str("survival"), v_s.clone()), (Py::str("log_removal"), py::truediv(&py::mul(&v_k.clone(), &v_t.clone())?, &py::math1("log", &Py::Float(10.0))?)?), (Py::str("percent_removal"), py::mul(&py::sub(&Py::Float(1.0), &v_s.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chick_watson(mut v_k: Py, mut v_n: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_lnS: Py = Py::None;
    let _ = f__pos(Py::str("k, n and C"), Py::tuple(vec![v_k.clone(), v_n.clone(), v_C.clone()]))?;
    let _ = f__nonneg(Py::str("the contact time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t868 = Py::Float(0.3); Py::Bool({ let t869 = v_n.clone(); if py::le(&t868, &t869)? { { let t870 = Py::Float(3.0); if py::le(&t869, &t870)? { true } else { false } } } else { false } }) }, Py::str("the coefficient of dilution n must lie between 0.3 and 3"))?;
    v_lnS = py::mul(&py::mul(&py::neg(&v_k.clone())?, &py::pow(&py::mul(&v_C.clone(), &Py::Float(1000.0))?, &v_n.clone())?)?, &py::truediv(&v_t.clone(), &Py::Float(60.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("survival"), py::math1("exp", &v_lnS.clone())?), (Py::str("log_inactivation"), py::truediv(&py::neg(&v_lnS.clone())?, &py::math1("log", &Py::Float(10.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chlorine_dose(mut v_Q: Py, mut v_demand: Py, mut v_residual: Py, mut v_purity: Py) -> R {
    let mut v_dose: Py = Py::None;
    let mut v_feed: Py = Py::None;
    let _ = f__pos(Py::str("the flow Q"), Py::tuple(vec![v_Q.clone()]))?;
    let _ = f__nonneg(Py::str("the demand and the residual"), Py::tuple(vec![v_demand.clone(), v_residual.clone()]))?;
    let _ = f__frac(Py::str("the available chlorine fraction purity"), v_purity.clone(), Py::Bool(true), Py::Bool(false))?;
    v_dose = py::add(&v_demand.clone(), &v_residual.clone())?;
    let _ = f__need({ let t933 = v_dose.clone(); Py::Bool({ let t934 = Py::Int(0i64); if py::gt(&t933, &t934)? { true } else { false } }) }, Py::str("demand plus residual must be positive"))?;
    v_feed = py::mul(&v_Q.clone(), &v_dose.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("dose"), v_dose.clone()), (Py::str("dose_mgL"), py::mul(&v_dose.clone(), &Py::Float(1000.0))?), (Py::str("feed"), v_feed.clone()), (Py::str("feed_kg_per_day"), py::mul(&v_feed.clone(), &c_DAY()?)?), (Py::str("product_kg_per_day"), py::truediv(&py::mul(&v_feed.clone(), &c_DAY()?)?, &v_purity.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ct_inactivation(mut v_C: Py, mut v_t10: Py, mut v_CT999: Py) -> R {
    let mut v_CT: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("C, t10 and CT999"), Py::tuple(vec![v_C.clone(), v_t10.clone(), v_CT999.clone()]))?;
    v_CT = py::mul(&v_C.clone(), &v_t10.clone())?;
    v_ratio = py::truediv(&v_CT.clone(), &v_CT999.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("CT"), v_CT.clone()), (Py::str("ratio"), v_ratio.clone()), (Py::str("log_inactivation"), py::mul(&Py::Float(3.0), &v_ratio.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ct_value(mut v_C: Py, mut v_V: Py, mut v_Q: Py, mut v_BF: Py) -> R {
    let mut v_t10: Py = Py::None;
    let _ = f__pos(Py::str("C, V and Q"), Py::tuple(vec![v_C.clone(), v_V.clone(), v_Q.clone()]))?;
    let _ = f__frac(Py::str("the baffling factor BF"), v_BF.clone(), Py::Bool(true), Py::Bool(false))?;
    v_t10 = py::truediv(&py::mul(&v_BF.clone(), &v_V.clone())?, &v_Q.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t10"), v_t10.clone()), (Py::str("CT"), py::mul(&v_C.clone(), &v_t10.clone())?), (Py::str("CT_mg_min_L"), py::truediv(&py::mul(&py::mul(&v_C.clone(), &Py::Float(1000.0))?, &v_t10.clone())?, &Py::Float(60.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cyclone_lapple(mut v_mu: Py, mut v_W: Py, mut v_Ne: Py, mut v_Vi: Py, mut v_rho_p: Py, mut v_rho_g: Py, mut v_dp: Py) -> R {
    let mut v_d50: Py = Py::None;
    let _ = f__pos(Py::str("mu, W, Ne, Vi, rho_p, rho_g and dp"), Py::tuple(vec![v_mu.clone(), v_W.clone(), v_Ne.clone(), v_Vi.clone(), v_rho_p.clone(), v_rho_g.clone(), v_dp.clone()]))?;
    let _ = f__need({ let t1023 = v_rho_p.clone(); Py::Bool({ let t1024 = v_rho_g.clone(); if py::gt(&t1023, &t1024)? { true } else { false } }) }, Py::str("the particles must be denser than the gas"))?;
    v_d50 = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Float(9.0), &v_mu.clone())?, &v_W.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_Ne.clone())?, &v_Vi.clone())?, &py::sub(&v_rho_p.clone(), &v_rho_g.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d50"), v_d50.clone()), (Py::str("eta"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::pow(&py::truediv(&v_d50.clone(), &v_dp.clone())?, &Py::Int(2i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_db_add(mut v_L: Py) -> R {
    let _ = f__need({ let t1126 = py::len(&v_L.clone())?; Py::Bool({ let t1127 = Py::Int(2i64); if py::ge(&t1126, &t1127)? { true } else { false } }) }, Py::str("give at least two levels"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("L_total"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::sum(&{ let mut out = vec![]; for t1145 in py::iter(&(v_L.clone()))? { let mut v_v: Py; 
v_v = t1145;
out.push(py::pow(&Py::Float(10.0), &py::truediv(&v_v.clone(), &Py::Float(10.0))?)?); } Py::list(out) }, None)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_db_subtract(mut v_Lt: Py, mut v_Lb: Py) -> R {
    let _ = f__db_list(Py::list(vec![v_Lt.clone(), v_Lb.clone()]), Py::str("Lt and Lb"))?;
    let _ = f__need({ let t284 = v_Lt.clone(); Py::Bool({ let t285 = v_Lb.clone(); if py::gt(&t284, &t285)? { true } else { false } }) }, Py::str("the total level must exceed the background level"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("L_source"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::sub(&py::pow(&Py::Float(10.0), &py::truediv(&v_Lt.clone(), &Py::Float(10.0))?)?, &py::pow(&Py::Float(10.0), &py::truediv(&v_Lb.clone(), &Py::Float(10.0))?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_detention_time(mut v_V: Py, mut v_Q: Py) -> R {
    let _ = f__pos(Py::str("V and Q"), Py::tuple(vec![v_V.clone(), v_Q.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta"), py::truediv(&v_V.clone(), &v_Q.clone())?), (Py::str("theta_h"), py::truediv(&py::truediv(&v_V.clone(), &v_Q.clone())?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_attenuation(mut v_L1: Py, mut v_r1: Py, mut v_r2: Py, mut v_source: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__db_list(Py::list(vec![v_L1.clone()]), Py::str("L1"))?;
    let _ = f__pos(Py::str("r1 and r2"), Py::tuple(vec![v_r1.clone(), v_r2.clone()]))?;
    let _ = f__need({ let t3264 = v_source.clone(); Py::Bool({ let t3265 = Py::tuple(vec![Py::str("point"), Py::str("line")]); if py::contains(&t3265, &t3264)? { true } else { false } }) }, Py::str("source: point or line"))?;
    v_f = (if ({ let t3266 = v_source.clone(); Py::Bool({ let t3267 = Py::str("point"); if py::eq(&t3266, &t3267) { true } else { false } }) }).truthy() { Py::Float(20.0) } else { Py::Float(10.0) });
    return Ok(f__out(Py::dict(vec![(Py::str("L2"), py::sub(&v_L1.clone(), &py::mul(&v_f.clone(), &py::math1("log10", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_do_saturation(mut v_T: Py, mut v_S: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_lnC: Py = Py::None;
    let _ = f__need({ let t1272 = c_T0C()?; Py::Bool({ let t1273 = v_T.clone(); if py::le(&t1272, &t1273)? { { let t1274 = py::add(&c_T0C()?, &Py::Float(40.0))?; if py::le(&t1273, &t1274)? { true } else { false } } } else { false } }) }, Py::str("the formula holds between 0 and 40 degC"))?;
    let _ = f__need({ let t1275 = Py::Float(0.0); Py::Bool({ let t1276 = v_S.clone(); if py::le(&t1275, &t1276)? { { let t1277 = Py::Float(40.0); if py::le(&t1276, &t1277)? { true } else { false } } } else { false } }) }, Py::str("the salinity must lie between 0 and 40 g/kg"))?;
    v_lnC = py::sub(&py::sub(&py::add(&py::sub(&py::add(&py::neg(&Py::Float(139.34411))?, &py::truediv(&Py::Float(157570.1), &v_T.clone())?)?, &py::truediv(&Py::Float(66423080.0), &py::pow(&v_T.clone(), &Py::Int(2i64))?)?)?, &py::truediv(&Py::Float(12438000000.0), &py::pow(&v_T.clone(), &Py::Int(3i64))?)?)?, &py::truediv(&Py::Float(862194900000.0), &py::pow(&v_T.clone(), &Py::Int(4i64))?)?)?, &py::mul(&v_S.clone(), &py::add(&py::sub(&Py::Float(0.017674), &py::truediv(&Py::Float(10.754), &v_T.clone())?)?, &py::truediv(&Py::Float(2140.7), &py::pow(&v_T.clone(), &Py::Int(2i64))?)?)?)?)?;
    v_C = py::math1("exp", &v_lnC.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("DOsat"), py::mul(&v_C.clone(), &c_MG()?)?), (Py::str("DOsat_mgL"), v_C.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dulong(mut v_C: Py, mut v_H: Py, mut v_O: Py, mut v_S: Py, mut v_N: Py) -> R {
    let mut v_kJ: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t1316 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("C"), v_C.clone()]), Py::tuple(vec![Py::str("H"), v_H.clone()]), Py::tuple(vec![Py::str("O"), v_O.clone()]), Py::tuple(vec![Py::str("S"), v_S.clone()]), Py::tuple(vec![Py::str("N"), v_N.clone()])])))? {
        let t1329 = (t1316).items()?;
        if t1329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t1329[0].clone();
        v_v = t1329[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(false), Py::Bool(false))?;
    }
    let _ = f__need({ let t1330 = py::add(&py::add(&py::add(&py::add(&v_C.clone(), &v_H.clone())?, &v_O.clone())?, &v_S.clone())?, &v_N.clone())?; Py::Bool({ let t1331 = py::add(&Py::Float(1.0), &Py::Float(1e-12))?; if py::le(&t1330, &t1331)? { true } else { false } }) }, Py::str("the mass fractions cannot add to more than 1"))?;
    let _ = f__need({ let t1332 = v_C.clone(); Py::Bool({ let t1333 = Py::Int(0i64); if py::gt(&t1332, &t1333)? { true } else { false } }) }, Py::str("the waste must contain carbon"))?;
    v_kJ = py::add(&py::add(&py::add(&py::mul(&py::mul(&Py::Float(337.0), &v_C.clone())?, &Py::Int(100i64))?, &py::mul(&Py::Float(1419.0), &py::sub(&py::mul(&v_H.clone(), &Py::Int(100i64))?, &py::truediv(&py::mul(&v_O.clone(), &Py::Int(100i64))?, &Py::Float(8.0))?)?)?)?, &py::mul(&py::mul(&Py::Float(93.0), &v_S.clone())?, &Py::Int(100i64))?)?, &py::mul(&py::mul(&Py::Float(23.0), &v_N.clone())?, &Py::Int(100i64))?)?;
    let _ = f__need({ let t1334 = v_kJ.clone(); Py::Bool({ let t1335 = Py::Int(0i64); if py::gt(&t1334, &t1335)? { true } else { false } }) }, Py::str("the composition gives no positive heating value"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("HHV"), py::mul(&v_kJ.clone(), &Py::Float(1000.0))?), (Py::str("HHV_kJ_per_kg"), v_kJ.clone()), (Py::str("HHV_MJ_per_kg"), py::truediv(&v_kJ.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ergun_headloss(mut v_d: Py, mut v_eps: Py, mut v_psi: Py, mut v_L: Py, mut v_v: Py, mut v_rho: Py, mut v_mu: Py) -> R {
    let mut v_dPdL: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let _ = f__pos(Py::str("d, L, v, rho and mu"), Py::tuple(vec![v_d.clone(), v_L.clone(), v_v.clone(), v_rho.clone(), v_mu.clone()]))?;
    let _ = f__frac(Py::str("the porosity eps"), v_eps.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__frac(Py::str("the sphericity psi"), v_psi.clone(), Py::Bool(true), Py::Bool(false))?;
    v_dp = py::mul(&v_psi.clone(), &v_d.clone())?;
    v_dPdL = py::add(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(150.0), &v_mu.clone())?, &py::pow(&py::sub(&Py::Float(1.0), &v_eps.clone())?, &Py::Int(2i64))?)?, &v_v.clone())?, &py::mul(&py::mul(&py::pow(&v_eps.clone(), &Py::Int(3i64))?, &v_dp.clone())?, &v_dp.clone())?)?, &py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(1.75), &v_rho.clone())?, &py::sub(&Py::Float(1.0), &v_eps.clone())?)?, &v_v.clone())?, &v_v.clone())?, &py::mul(&py::pow(&v_eps.clone(), &Py::Int(3i64))?, &v_dp.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dP"), py::mul(&v_dPdL.clone(), &v_L.clone())?), (Py::str("h"), py::truediv(&py::mul(&v_dPdL.clone(), &v_L.clone())?, &py::mul(&v_rho.clone(), &c_G0()?)?)?), (Py::str("Re"), py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_v.clone())?, &v_dp.clone())?, &py::mul(&v_mu.clone(), &py::sub(&Py::Float(1.0), &v_eps.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_esp_area(mut v_w: Py, mut v_Q: Py, mut v_eta: Py) -> R {
    let _ = f__pos(Py::str("w and Q"), Py::tuple(vec![v_w.clone(), v_Q.clone()]))?;
    let _ = f__frac(Py::str("the efficiency eta"), v_eta.clone(), Py::Bool(true), Py::Bool(true))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), py::truediv(&py::mul(&py::neg(&v_Q.clone())?, &py::math1("log", &py::sub(&Py::Float(1.0), &v_eta.clone())?)?)?, &v_w.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_esp_efficiency(mut v_w: Py, mut v_A: Py, mut v_Q: Py) -> R {
    let mut v_eta: Py = Py::None;
    let _ = f__pos(Py::str("w, A and Q"), Py::tuple(vec![v_w.clone(), v_A.clone(), v_Q.clone()]))?;
    v_eta = py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_w.clone())?, &v_A.clone())?, &v_Q.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eta"), v_eta.clone()), (Py::str("penetration"), py::sub(&Py::Float(1.0), &v_eta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_field_oxygen_transfer(mut v_SOTR: Py, mut v_alpha: Py, mut v_beta: Py, mut v_theta: Py, mut v_CsT: Py, mut v_Cs20: Py, mut v_CL: Py, mut v_T: Py, mut v_Fp: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_drive: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("SOTR, alpha, beta, theta, CsT and Cs20"), Py::tuple(vec![v_SOTR.clone(), v_alpha.clone(), v_beta.clone(), v_theta.clone(), v_CsT.clone(), v_Cs20.clone()]))?;
    let _ = f__nonneg(Py::str("CL"), Py::tuple(vec![v_CL.clone()]))?;
    v_F = (if ({ let t1542 = v_Fp.clone(); Py::Bool({ let t1543 = Py::None; if py::is(&t1542, &t1543) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_Fp.clone() });
    let _ = f__pos(Py::str("the pressure correction Fp"), Py::tuple(vec![v_F.clone()]))?;
    v_drive = py::sub(&py::mul(&py::mul(&v_beta.clone(), &v_CsT.clone())?, &v_F.clone())?, &v_CL.clone())?;
    let _ = f__need({ let t1544 = v_drive.clone(); Py::Bool({ let t1545 = Py::Int(0i64); if py::gt(&t1544, &t1545)? { true } else { false } }) }, Py::str("the oxygen level is at or above the field saturation: no transfer"))?;
    v_ratio = py::truediv(&py::mul(&py::mul(&v_alpha.clone(), &py::pow(&v_theta.clone(), &py::sub(&v_T.clone(), &py::add(&c_T0C()?, &Py::Float(20.0))?)?)?)?, &v_drive.clone())?, &v_Cs20.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AOTR"), py::mul(&v_SOTR.clone(), &v_ratio.clone())?), (Py::str("AOTR_kg_per_h"), py::mul(&py::mul(&v_SOTR.clone(), &v_ratio.clone())?, &c_HOUR()?)?), (Py::str("ratio"), v_ratio.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filter_rate(mut v_Q: Py, mut v_A: Py, mut v_N: Py) -> R {
    let mut v_v: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let _ = f__pos(Py::str("Q and A"), Py::tuple(vec![v_Q.clone(), v_A.clone()]))?;
    let _ = f__need({ let t1648 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1648.truthy() { t1648 } else { { let t1646 = v_N.clone(); Py::Bool({ let t1647 = Py::Int(2i64); if py::ge(&t1646, &t1647)? { true } else { false } }) } } }, Py::str("give at least two filters (one may be out of service)"))?;
    v_v = py::truediv(&v_Q.clone(), &py::mul(&v_N.clone(), &v_A.clone())?)?;
    v_v1 = py::truediv(&v_Q.clone(), &py::mul(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_A.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("v_m_per_h"), py::mul(&v_v.clone(), &c_HOUR()?)?), (Py::str("v_one_out"), v_v1.clone()), (Py::str("v_one_out_m_per_h"), py::mul(&v_v1.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fm_ratio(mut v_Q: Py, mut v_S0: Py, mut v_V: Py, mut v_X: Py) -> R {
    let mut v_fm: Py = Py::None;
    let _ = f__pos(Py::str("Q, S0, V and X"), Py::tuple(vec![v_Q.clone(), v_S0.clone(), v_V.clone(), v_X.clone()]))?;
    v_fm = py::truediv(&py::mul(&v_Q.clone(), &v_S0.clone())?, &py::mul(&v_V.clone(), &v_X.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("FM"), v_fm.clone()), (Py::str("FM_per_day"), py::mul(&v_fm.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_freundlich_dose(mut v_C0: Py, mut v_Ce: Py, mut v_K: Py, mut v_n: Py) -> R {
    let mut v_dose: Py = Py::None;
    let mut v_qe: Py = Py::None;
    let _ = f__pos(Py::str("C0, Ce, K and n"), Py::tuple(vec![v_C0.clone(), v_Ce.clone(), v_K.clone(), v_n.clone()]))?;
    let _ = f__need({ let t1737 = v_Ce.clone(); Py::Bool({ let t1738 = v_C0.clone(); if py::lt(&t1737, &t1738)? { true } else { false } }) }, Py::str("the target concentration must be below the initial concentration"))?;
    v_qe = py::mul(&v_K.clone(), &py::pow(&py::mul(&v_Ce.clone(), &Py::Float(1000.0))?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?;
    v_dose = py::truediv(&py::mul(&py::sub(&v_C0.clone(), &v_Ce.clone())?, &Py::Float(1000.0))?, &v_qe.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("qe"), v_qe.clone()), (Py::str("dose"), v_dose.clone()), (Py::str("dose_mgL"), py::mul(&v_dose.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_freundlich_fit(mut v_Ce: Py, mut v_qe: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__iso_check(v_Ce.clone(), v_qe.clone())?;
    let t1797 = (f__linfit({ let mut out = vec![]; for t1795 in py::iter(&(v_Ce.clone()))? { let mut v_v: Py; 
v_v = t1795;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1796 in py::iter(&(v_qe.clone()))? { let mut v_v: Py; 
v_v = t1796;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t1797.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1797[0].clone();
    v_i = t1797[1].clone();
    v_r2 = t1797[2].clone();
    let _ = f__need({ let t1798 = v_s.clone(); Py::Bool({ let t1799 = Py::Int(0i64); if py::gt(&t1798, &t1799)? { true } else { false } }) }, Py::str("the loading must rise with concentration"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), py::pow(&Py::Float(10.0), &v_i.clone())?), (Py::str("inv_n"), v_s.clone()), (Py::str("n"), py::truediv(&Py::Float(1.0), &v_s.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ground_centerline(mut v_Q: Py, mut v_u: Py, mut v_sigma_y: Py, mut v_sigma_z: Py, mut v_H: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("Q, u, sigma_y and sigma_z"), Py::tuple(vec![v_Q.clone(), v_u.clone(), v_sigma_y.clone(), v_sigma_z.clone()]))?;
    let _ = f__nonneg(Py::str("the effective stack height H"), Py::tuple(vec![v_H.clone()]))?;
    v_C = py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_u.clone())?, &v_sigma_y.clone())?, &v_sigma_z.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_H.clone())?, &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sigma_z.clone())?, &v_sigma_z.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hardness(mut v_Ca: Py, mut v_Mg: Py) -> R {
    let mut v_ca_h: Py = Py::None;
    let mut v_cls: Py = Py::None;
    let mut v_eq_caco3: Py = Py::None;
    let mut v_mg_h: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__nonneg(Py::str("Ca and Mg"), Py::tuple(vec![v_Ca.clone(), v_Mg.clone()]))?;
    v_eq_caco3 = py::truediv(&py::add(&py::add(&py::getitem(&c_AW()?, &Py::str("Ca"))?, &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&Py::Int(3i64), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?, &Py::Float(2.0))?;
    v_ca_h = py::truediv(&py::mul(&py::mul(&v_Ca.clone(), &Py::Float(1000.0))?, &v_eq_caco3.clone())?, &py::truediv(&py::getitem(&c_AW()?, &Py::str("Ca"))?, &Py::Float(2.0))?)?;
    v_mg_h = py::truediv(&py::mul(&py::mul(&v_Mg.clone(), &Py::Float(1000.0))?, &v_eq_caco3.clone())?, &py::truediv(&py::getitem(&c_AW()?, &Py::str("Mg"))?, &Py::Float(2.0))?)?;
    v_th = py::add(&v_ca_h.clone(), &v_mg_h.clone())?;
    v_cls = (if ({ let t5123 = v_th.clone(); Py::Bool({ let t5124 = Py::Int(60i64); if py::lt(&t5123, &t5124)? { true } else { false } }) }).truthy() { Py::str("soft") } else { (if ({ let t5125 = v_th.clone(); Py::Bool({ let t5126 = Py::Int(120i64); if py::lt(&t5125, &t5126)? { true } else { false } }) }).truthy() { Py::str("moderately hard") } else { (if ({ let t5127 = v_th.clone(); Py::Bool({ let t5128 = Py::Int(180i64); if py::lt(&t5127, &t5128)? { true } else { false } }) }).truthy() { Py::str("hard") } else { Py::str("very hard") }) }) });
    return Ok(f__out(Py::dict(vec![(Py::str("total_hardness"), v_th.clone()), (Py::str("calcium_hardness"), v_ca_h.clone()), (Py::str("magnesium_hardness"), v_mg_h.clone()), (Py::str("classification"), v_cls.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_convert(mut v_H: Py, mut v_form: Py, mut v_T: Py) -> R {
    let mut v_hpc: Py = Py::None;
    let _ = f__pos(Py::str("H and T"), Py::tuple(vec![v_H.clone(), v_T.clone()]))?;
    let _ = f__need({ let t1947 = v_form.clone(); Py::Bool({ let t1948 = c__HFORMS()?; if py::contains(&t1948, &t1947)? { true } else { false } }) }, py::add(&Py::str("form: "), &py::method(&Py::str(", "), "join", &[c__HFORMS()?])?)?)?;
    if ({ let t1949 = v_form.clone(); Py::Bool({ let t1950 = Py::str("pa_m3_per_mol"); if py::eq(&t1949, &t1950) { true } else { false } }) }).truthy() {
        v_hpc = v_H.clone();
    } else {
        if ({ let t1951 = v_form.clone(); Py::Bool({ let t1952 = Py::str("atm_m3_per_mol"); if py::eq(&t1951, &t1952) { true } else { false } }) }).truthy() {
            v_hpc = py::mul(&v_H.clone(), &c_P_ATM()?)?;
        } else {
            if ({ let t1953 = v_form.clone(); Py::Bool({ let t1954 = Py::str("dimensionless"); if py::eq(&t1953, &t1954) { true } else { false } }) }).truthy() {
                v_hpc = py::mul(&py::mul(&v_H.clone(), &c_R_GAS()?)?, &v_T.clone())?;
            } else {
                if ({ let t1955 = v_form.clone(); Py::Bool({ let t1956 = Py::str("atm"); if py::eq(&t1955, &t1956) { true } else { false } }) }).truthy() {
                    v_hpc = py::truediv(&py::mul(&v_H.clone(), &c_P_ATM()?)?, &c_RHO_W_MOLAR()?)?;
                } else {
                    if ({ let t1957 = v_form.clone(); Py::Bool({ let t1958 = Py::str("mol_per_m3_pa"); if py::eq(&t1957, &t1958) { true } else { false } }) }).truthy() {
                        v_hpc = py::truediv(&Py::Float(1.0), &v_H.clone())?;
                    } else {
                        v_hpc = py::truediv(&c_P_ATM()?, &py::mul(&v_H.clone(), &Py::Float(1000.0))?)?;
                    }
                }
            }
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("H_pa_m3_per_mol"), v_hpc.clone()), (Py::str("H_atm_m3_per_mol"), py::truediv(&v_hpc.clone(), &c_P_ATM()?)?), (Py::str("H_dimensionless"), py::truediv(&v_hpc.clone(), &py::mul(&c_R_GAS()?, &v_T.clone())?)?), (Py::str("H_atm"), py::truediv(&py::mul(&v_hpc.clone(), &c_RHO_W_MOLAR()?)?, &c_P_ATM()?)?), (Py::str("H_mol_per_m3_pa"), py::truediv(&Py::Float(1.0), &v_hpc.clone())?), (Py::str("H_m_per_atm"), py::truediv(&c_P_ATM()?, &py::mul(&v_hpc.clone(), &Py::Float(1000.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_equilibrium(mut v_p: Py, mut v_H: Py, mut v_M: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("p and H"), Py::tuple(vec![v_p.clone(), v_H.clone()]))?;
    v_C = py::truediv(&v_p.clone(), &v_H.clone())?;
    v_out = Py::dict(vec![(Py::str("C"), v_C.clone())]);
    if ({ let t2003 = v_M.clone(); Py::Bool({ let t2004 = Py::None; if !py::is(&t2003, &t2004) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the molar mass M"), Py::tuple(vec![v_M.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("C_mass"), py::mul(&v_C.clone(), &v_M.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("C_mgL"), py::mul(&py::mul(&v_C.clone(), &v_M.clone())?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_vant_hoff(mut v_Hcp: Py, mut v_B: Py, mut v_T: Py, mut v_Tref: Py) -> R {
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::str("Hcp, T and Tref"), Py::tuple(vec![v_Hcp.clone(), v_T.clone(), v_Tref.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_B.clone()]))?;
    v_h = py::mul(&v_Hcp.clone(), &py::math1("exp", &py::mul(&v_B.clone(), &py::sub(&py::truediv(&Py::Float(1.0), &v_T.clone())?, &py::truediv(&Py::Float(1.0), &v_Tref.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Hcp_T"), v_h.clone()), (Py::str("Hpc_T"), py::truediv(&Py::Float(1.0), &v_h.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hocl_fraction(mut v_pH: Py, mut v_pKa: Py, mut v_T: Py) -> R {
    let mut v_a: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_pH.clone()]))?;
    let _ = f__need({ let t2190 = Py::Float(0.0); Py::Bool({ let t2191 = v_pH.clone(); if py::le(&t2190, &t2191)? { { let t2192 = Py::Float(14.0); if py::le(&t2191, &t2192)? { true } else { false } } } else { false } }) }, Py::str("pH must lie between 0 and 14"))?;
    let _ = f__need({ let t2193 = { let t2195 = v_pKa.clone(); Py::Bool({ let t2196 = Py::None; if py::is(&t2195, &t2196) { true } else { false } }) }; Py::Bool({ let t2194 = { let t2197 = v_T.clone(); Py::Bool({ let t2198 = Py::None; if py::is(&t2197, &t2198) { true } else { false } }) }; if py::ne(&t2193, &t2194) { true } else { false } }) }, Py::str("give either pKa or the temperature T (to compute pKa), not both"))?;
    if ({ let t2199 = v_pKa.clone(); Py::Bool({ let t2200 = Py::None; if py::is(&t2199, &t2200) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2201 = c_T0C()?; Py::Bool({ let t2202 = v_T.clone(); if py::le(&t2201, &t2202)? { { let t2203 = py::add(&c_T0C()?, &Py::Float(35.0))?; if py::le(&t2202, &t2203)? { true } else { false } } } else { false } }) }, Py::str("the pKa correlation holds between 0 and 35 degC"))?;
        v_pKa = py::add(&py::sub(&py::truediv(&Py::Float(3000.0), &v_T.clone())?, &Py::Float(10.0686))?, &py::mul(&Py::Float(0.0253), &v_T.clone())?)?;
    }
    v_a = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &v_pKa.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha_HOCl"), v_a.clone()), (Py::str("alpha_OCl"), py::sub(&Py::Float(1.0), &v_a.clone())?), (Py::str("pKa"), v_pKa.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_holland_rise(mut v_vs: Py, mut v_d: Py, mut v_u: Py, mut v_p: Py, mut v_Ts: Py, mut v_Ta: Py, mut v_hs: Py) -> R {
    let mut v_dh: Py = Py::None;
    let _ = f__pos(Py::str("vs, d, u, p, Ts and Ta"), Py::tuple(vec![v_vs.clone(), v_d.clone(), v_u.clone(), v_p.clone(), v_Ts.clone(), v_Ta.clone()]))?;
    let _ = f__need({ let t2097 = v_Ts.clone(); Py::Bool({ let t2098 = v_Ta.clone(); if py::gt(&t2097, &t2098)? { true } else { false } }) }, Py::str("the stack gas must be warmer than the air (Ts > Ta)"))?;
    let _ = f__nonneg(Py::str("the stack height hs"), Py::tuple(vec![v_hs.clone()]))?;
    v_dh = py::mul(&py::truediv(&py::mul(&v_vs.clone(), &v_d.clone())?, &v_u.clone())?, &py::add(&Py::Float(1.5), &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(0.00268), &py::truediv(&v_p.clone(), &Py::Float(100.0))?)?, &v_d.clone())?, &py::sub(&v_Ts.clone(), &v_Ta.clone())?)?, &v_Ts.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dh"), v_dh.clone()), (Py::str("H"), py::add(&v_hs.clone(), &v_dh.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hrt_olr(mut v_Q: Py, mut v_S0: Py, mut v_V: Py) -> R {
    let _ = f__pos(Py::str("Q, S0 and V"), Py::tuple(vec![v_Q.clone(), v_S0.clone(), v_V.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("HRT"), py::truediv(&v_V.clone(), &v_Q.clone())?), (Py::str("HRT_h"), py::truediv(&py::truediv(&v_V.clone(), &v_Q.clone())?, &c_HOUR()?)?), (Py::str("OLR"), py::truediv(&py::mul(&v_Q.clone(), &v_S0.clone())?, &v_V.clone())?), (Py::str("OLR_kg_per_m3_day"), py::mul(&py::truediv(&py::mul(&v_Q.clone(), &v_S0.clone())?, &v_V.clone())?, &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_temperature(mut v_k20: Py, mut v_theta: Py, mut v_T: Py) -> R {
    let mut v_kT: Py = Py::None;
    let _ = f__pos(Py::str("k20 and theta"), Py::tuple(vec![v_k20.clone(), v_theta.clone()]))?;
    let _ = f__need({ let t3785 = Py::Float(0.9); Py::Bool({ let t3786 = v_theta.clone(); if py::lt(&t3785, &t3786)? { { let t3787 = Py::Float(1.2); if py::lt(&t3786, &t3787)? { true } else { false } } } else { false } }) }, Py::str("theta must lie between 0.9 and 1.2 (typical 1.024 to 1.135)"))?;
    let _ = f__need({ let t3788 = py::sub(&c_T0C()?, &Py::Float(5.0))?; Py::Bool({ let t3789 = v_T.clone(); if py::le(&t3788, &t3789)? { { let t3790 = py::add(&c_T0C()?, &Py::Float(60.0))?; if py::le(&t3789, &t3790)? { true } else { false } } } else { false } }) }, Py::str("the temperature must lie between -5 and 60 degC"))?;
    v_kT = py::mul(&v_k20.clone(), &py::pow(&v_theta.clone(), &py::sub(&v_T.clone(), &py::add(&c_T0C()?, &Py::Float(20.0))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("kT"), v_kT.clone()), (Py::str("kT_per_day"), py::mul(&v_kT.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kla_fit(mut v_t: Py, mut v_C: Py, mut v_Cs: Py) -> R {
    let mut v_cs: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2290 = { let t2286 = py::len(&v_t.clone())?; Py::Bool({ let t2287 = py::len(&v_C.clone())?; if py::eq(&t2286, &t2287) { true } else { false } }) }; if !t2290.truthy() { t2290 } else { { let t2288 = py::len(&v_t.clone())?; Py::Bool({ let t2289 = Py::Int(3i64); if py::ge(&t2288, &t2289)? { true } else { false } }) } } }, Py::str("give at least three times and as many DO readings"))?;
    let _ = f__pos(Py::str("Cs"), Py::tuple(vec![v_Cs.clone()]))?;
    v_cs = py::mul(&v_Cs.clone(), &Py::Float(1000.0))?;
    for t2297 in py::iter(&(v_C.clone()))? {
        v_v = t2297;
        let _ = f__need({ let t2301 = py::math1("isfinite", &v_v.clone())?; if !t2301.truthy() { t2301 } else { { let t2298 = Py::Int(0i64); Py::Bool({ let t2299 = v_v.clone(); if py::le(&t2298, &t2299)? { { let t2300 = v_cs.clone(); if py::lt(&t2299, &t2300)? { true } else { false } } } else { false } }) } } }, Py::str("every DO reading must be below the saturation concentration"))?;
    }
    let _ = f__need({ let t2302 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t2303 = py::len(&v_t.clone())?; if py::eq(&t2302, &t2303) { true } else { false } }) }, Py::str("the times must be distinct"))?;
    let t2314 = (f__linfit(Py::list((v_t.clone()).items()?), { let mut out = vec![]; for t2313 in py::iter(&(v_C.clone()))? { let mut v_v: Py; 
v_v = t2313;
out.push(py::math1("log", &py::sub(&v_cs.clone(), &v_v.clone())?)?); } Py::list(out) })?).items()?;
    if t2314.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2314[0].clone();
    v_i = t2314[1].clone();
    v_r2 = t2314[2].clone();
    let _ = f__need({ let t2315 = v_s.clone(); Py::Bool({ let t2316 = Py::Int(0i64); if py::lt(&t2315, &t2316)? { true } else { false } }) }, Py::str("the deficit must fall with time"))?;
    v_k = py::neg(&v_s.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("KLa_per_min"), v_k.clone()), (Py::str("KLa"), py::truediv(&v_k.clone(), &Py::Float(60.0))?), (Py::str("KLa_per_h"), py::mul(&v_k.clone(), &Py::Float(60.0))?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kla_two_point(mut v_Cs: Py, mut v_C1: Py, mut v_C2: Py, mut v_t1: Py, mut v_t2: Py) -> R {
    let mut v_K: Py = Py::None;
    let _ = f__pos(Py::str("Cs"), Py::tuple(vec![v_Cs.clone()]))?;
    let _ = f__nonneg(Py::str("C1, t1 and t2"), Py::tuple(vec![v_C1.clone(), v_t1.clone(), v_t2.clone()]))?;
    let _ = f__need({ let t2367 = v_C1.clone(); Py::Bool({ let t2368 = v_C2.clone(); if py::lt(&t2367, &t2368)? { { let t2369 = v_Cs.clone(); if py::lt(&t2368, &t2369)? { true } else { false } } } else { false } }) }, Py::str("the concentrations must rise toward saturation: C1 < C2 < Cs"))?;
    let _ = f__need({ let t2370 = v_t2.clone(); Py::Bool({ let t2371 = v_t1.clone(); if py::gt(&t2370, &t2371)? { true } else { false } }) }, Py::str("t2 must be later than t1"))?;
    v_K = py::truediv(&py::math1("log", &py::truediv(&py::sub(&v_Cs.clone(), &v_C1.clone())?, &py::sub(&v_Cs.clone(), &v_C2.clone())?)?)?, &py::sub(&v_t2.clone(), &v_t1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("KLa"), v_K.clone()), (Py::str("KLa_per_h"), py::mul(&v_K.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lake_steady(mut v_W: Py, mut v_Q: Py, mut v_k: Py, mut v_V: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("W, Q and V"), Py::tuple(vec![v_W.clone(), v_Q.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the decay constant k"), Py::tuple(vec![v_k.clone()]))?;
    v_C = py::truediv(&v_W.clone(), &py::add(&v_Q.clone(), &py::mul(&v_k.clone(), &v_V.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("tau"), py::truediv(&v_V.clone(), &v_Q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lake_transient(mut v_W: Py, mut v_Q: Py, mut v_k: Py, mut v_V: Py, mut v_C0: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Css: Py = Py::None;
    let _ = f__pos(Py::str("W, Q and V"), Py::tuple(vec![v_W.clone(), v_Q.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("k, C0 and t"), Py::tuple(vec![v_k.clone(), v_C0.clone(), v_t.clone()]))?;
    v_Css = py::truediv(&v_W.clone(), &py::add(&v_Q.clone(), &py::mul(&v_k.clone(), &v_V.clone())?)?)?;
    v_C = py::add(&v_Css.clone(), &py::mul(&py::sub(&v_C0.clone(), &v_Css.clone())?, &py::math1("exp", &py::mul(&py::neg(&py::add(&py::truediv(&v_Q.clone(), &v_V.clone())?, &v_k.clone())?)?, &v_t.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("C_ss"), v_Css.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_landfill_gas(mut v_M: Py, mut v_L0: Py, mut v_k: Py, mut v_tc: Py, mut v_t: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_Vc: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let _ = f__pos(Py::str("M, L0, k and tc"), Py::tuple(vec![v_M.clone(), v_L0.clone(), v_k.clone(), v_tc.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_tm = py::minmax(&[v_t.clone(), v_tc.clone()], false)?;
    v_Q = py::mul(&py::mul(&v_M.clone(), &v_L0.clone())?, &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_t.clone(), &v_tm.clone())?)?)?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?;
    if ({ let t2522 = v_t.clone(); Py::Bool({ let t2523 = v_tc.clone(); if py::le(&t2522, &t2523)? { true } else { false } }) }).truthy() {
        v_Vc = py::mul(&py::mul(&v_M.clone(), &v_L0.clone())?, &py::sub(&v_t.clone(), &py::truediv(&py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?, &v_k.clone())?)?)?;
    } else {
        v_Vc = py::mul(&py::mul(&v_M.clone(), &v_L0.clone())?, &py::sub(&v_tc.clone(), &py::truediv(&py::sub(&py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_t.clone(), &v_tc.clone())?)?)?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?, &v_k.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Q_CH4"), v_Q.clone()), (Py::str("Q_CH4_m3_per_year"), py::mul(&v_Q.clone(), &c_YEAR()?)?), (Py::str("V_CH4"), v_Vc.clone()), (Py::str("Q_LFG"), py::mul(&Py::Float(2.0), &v_Q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_landfill_life(mut v_capacity: Py, mut v_M: Py, mut v_rho: Py, mut v_cover: Py, mut v_g: Py) -> R {
    let mut v_V0: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("capacity, M and rho"), Py::tuple(vec![v_capacity.clone(), v_M.clone(), v_rho.clone()]))?;
    let _ = f__need({ let t2568 = Py::Float(0.0); Py::Bool({ let t2569 = v_cover.clone(); if py::le(&t2568, &t2569)? { { let t2570 = Py::Float(1.0); if py::le(&t2569, &t2570)? { true } else { false } } } else { false } }) }, Py::str("the cover ratio must lie between 0 and 1"))?;
    let _ = f__need({ let t2571 = Py::Float(0.0); Py::Bool({ let t2572 = v_g.clone(); if py::le(&t2571, &t2572)? { { let t2573 = Py::Float(0.2); if py::le(&t2572, &t2573)? { true } else { false } } } else { false } }) }, Py::str("the growth rate must lie between 0 and 20 % a year"))?;
    v_V0 = py::mul(&py::truediv(&py::mul(&v_M.clone(), &c_YEAR()?)?, &v_rho.clone())?, &py::add(&Py::Float(1.0), &v_cover.clone())?)?;
    v_n = (if ({ let t2574 = v_g.clone(); Py::Bool({ let t2575 = Py::Int(0i64); if py::eq(&t2574, &t2575) { true } else { false } }) }).truthy() { py::truediv(&v_capacity.clone(), &v_V0.clone())? } else { py::truediv(&py::math1("log", &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_capacity.clone(), &v_g.clone())?, &v_V0.clone())?)?)?, &py::math1("log", &py::add(&Py::Float(1.0), &v_g.clone())?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("V_first_year"), v_V0.clone()), (Py::str("life_years"), v_n.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_langmuir_dose(mut v_C0: Py, mut v_Ce: Py, mut v_qmax: Py, mut v_b: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_dose: Py = Py::None;
    let mut v_qe: Py = Py::None;
    let _ = f__pos(Py::str("C0, Ce, qmax and b"), Py::tuple(vec![v_C0.clone(), v_Ce.clone(), v_qmax.clone(), v_b.clone()]))?;
    let _ = f__need({ let t2620 = v_Ce.clone(); Py::Bool({ let t2621 = v_C0.clone(); if py::lt(&t2620, &t2621)? { true } else { false } }) }, Py::str("the target concentration must be below the initial concentration"))?;
    v_c = py::mul(&v_Ce.clone(), &Py::Float(1000.0))?;
    v_qe = py::truediv(&py::mul(&py::mul(&v_qmax.clone(), &v_b.clone())?, &v_c.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_b.clone(), &v_c.clone())?)?)?;
    v_dose = py::truediv(&py::mul(&py::sub(&v_C0.clone(), &v_Ce.clone())?, &Py::Float(1000.0))?, &v_qe.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("qe"), v_qe.clone()), (Py::str("dose"), v_dose.clone()), (Py::str("dose_mgL"), py::mul(&v_dose.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_langmuir_fit(mut v_Ce: Py, mut v_qe: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__iso_check(v_Ce.clone(), v_qe.clone())?;
    let t2680 = (f__linfit(Py::list((v_Ce.clone()).items()?), { let mut out = vec![]; for t2678 in py::iter(&(py::zip(&[v_Ce.clone(), v_qe.clone()])?))? { let mut v_c: Py; let mut v_q: Py; 
let t2679 = (t2678).items()?;
if t2679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t2679[0].clone();
v_q = t2679[1].clone();
out.push(py::truediv(&v_c.clone(), &v_q.clone())?); } Py::list(out) })?).items()?;
    if t2680.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2680[0].clone();
    v_i = t2680[1].clone();
    v_r2 = t2680[2].clone();
    let _ = f__need({ let t2685 = { let t2681 = v_s.clone(); Py::Bool({ let t2682 = Py::Int(0i64); if py::gt(&t2681, &t2682)? { true } else { false } }) }; if !t2685.truthy() { t2685 } else { { let t2683 = v_i.clone(); Py::Bool({ let t2684 = Py::Int(0i64); if py::gt(&t2683, &t2684)? { true } else { false } }) } } }, Py::str("the data do not follow a Langmuir isotherm (the linearised slope and intercept must be positive)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("qmax"), py::truediv(&Py::Float(1.0), &v_s.clone())?), (Py::str("b"), py::truediv(&v_s.clone(), &v_i.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lawrence_mccarty(mut v_Y: Py, mut v_k: Py, mut v_Ks: Py, mut v_kd: Py, mut v_SRT: Py, mut v_HRT: Py, mut v_S0: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_X: Py = Py::None;
    let _ = f__pos(Py::str("Y, k, Ks, SRT, HRT and S0"), Py::tuple(vec![v_Y.clone(), v_k.clone(), v_Ks.clone(), v_SRT.clone(), v_HRT.clone(), v_S0.clone()]))?;
    let _ = f__nonneg(Py::str("kd"), Py::tuple(vec![v_kd.clone()]))?;
    let _ = f__need({ let t13 = v_Y.clone(); Py::Bool({ let t14 = Py::Float(1.0); if py::lt(&t13, &t14)? { true } else { false } }) }, Py::str("the yield Y must be below 1"))?;
    let _ = f__need({ let t15 = v_SRT.clone(); Py::Bool({ let t16 = v_HRT.clone(); if py::ge(&t15, &t16)? { true } else { false } }) }, Py::str("the SRT cannot be shorter than the HRT in a system with sludge recycle or wasting"))?;
    v_S = f__lm_S(v_Y.clone(), v_k.clone(), v_Ks.clone(), v_kd.clone(), v_SRT.clone())?;
    let _ = f__need({ let t19 = v_S.clone(); Py::Bool({ let t20 = v_S0.clone(); if py::lt(&t19, &t20)? { true } else { false } }) }, Py::str("the effluent substrate would exceed the influent: washout"))?;
    v_X = py::truediv(&py::mul(&py::mul(&v_SRT.clone(), &v_Y.clone())?, &py::sub(&v_S0.clone(), &v_S.clone())?)?, &py::mul(&v_HRT.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_kd.clone(), &v_SRT.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("S_mgL"), py::mul(&v_S.clone(), &Py::Float(1000.0))?), (Py::str("X"), v_X.clone()), (Py::str("X_mgL"), py::mul(&v_X.clone(), &Py::Float(1000.0))?), (Py::str("Y_obs"), py::truediv(&v_Y.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_kd.clone(), &v_SRT.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ldn(mut v_L: Py, mut v_metric: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_pen: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t1063 = py::len(&v_L.clone())?; Py::Bool({ let t1064 = Py::Int(24i64); if py::eq(&t1063, &t1064) { true } else { false } }) }, Py::str("give 24 hourly levels, starting with the hour from midnight to 01:00"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    let _ = f__need({ let t1070 = v_metric.clone(); Py::Bool({ let t1071 = Py::tuple(vec![Py::str("ldn"), Py::str("lden")]); if py::contains(&t1071, &t1070)? { true } else { false } }) }, Py::str("metric: ldn or lden"))?;
    v_s = Py::Float(0.0);
    for t1072 in py::iter(&(py::enumerate(&v_L.clone(), 0)?))? {
        let t1087 = (t1072).items()?;
        if t1087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h = t1087[0].clone();
        v_v = t1087[1].clone();
        if ({ let t1073 = v_metric.clone(); Py::Bool({ let t1074 = Py::str("ldn"); if py::eq(&t1073, &t1074) { true } else { false } }) }).truthy() {
            v_pen = (if ({ let t1079 = { let t1075 = v_h.clone(); Py::Bool({ let t1076 = Py::Int(7i64); if py::lt(&t1075, &t1076)? { true } else { false } }) }; if t1079.truthy() { t1079 } else { { let t1077 = v_h.clone(); Py::Bool({ let t1078 = Py::Int(22i64); if py::ge(&t1077, &t1078)? { true } else { false } }) } } }).truthy() { Py::Float(10.0) } else { Py::Float(0.0) });
        } else {
            v_pen = (if ({ let t1084 = { let t1080 = v_h.clone(); Py::Bool({ let t1081 = Py::Int(7i64); if py::lt(&t1080, &t1081)? { true } else { false } }) }; if t1084.truthy() { t1084 } else { { let t1082 = v_h.clone(); Py::Bool({ let t1083 = Py::Int(23i64); if py::ge(&t1082, &t1083)? { true } else { false } }) } } }).truthy() { Py::Float(10.0) } else { (if ({ let t1085 = v_h.clone(); Py::Bool({ let t1086 = Py::Int(19i64); if py::ge(&t1085, &t1086)? { true } else { false } }) }).truthy() { Py::Float(5.0) } else { Py::Float(0.0) }) });
        }
        v_s = py::iadd(&v_s, &py::pow(&Py::Float(10.0), &py::truediv(&py::add(&v_v.clone(), &v_pen.clone())?, &Py::Float(10.0))?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("L"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::truediv(&v_s.clone(), &Py::Float(24.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_leq(mut v_L: Py, mut v_t: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t1378 = { let t1374 = py::len(&v_L.clone())?; Py::Bool({ let t1375 = py::len(&v_t.clone())?; if py::eq(&t1374, &t1375) { true } else { false } }) }; if !t1378.truthy() { t1378 } else { { let t1376 = py::len(&v_L.clone())?; Py::Bool({ let t1377 = Py::Int(1i64); if py::ge(&t1376, &t1377)? { true } else { false } }) } } }, Py::str("give one duration per level"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    for t1384 in py::iter(&(v_t.clone()))? {
        v_v = t1384;
        let _ = f__need({ let t1387 = py::math1("isfinite", &v_v.clone())?; if !t1387.truthy() { t1387 } else { { let t1385 = v_v.clone(); Py::Bool({ let t1386 = Py::Int(0i64); if py::ge(&t1385, &t1386)? { true } else { false } }) } } }, Py::str("durations must be zero or positive"))?;
    }
    v_T = py::sum(&v_t.clone(), None)?;
    let _ = f__need({ let t1388 = v_T.clone(); Py::Bool({ let t1389 = Py::Int(0i64); if py::gt(&t1388, &t1389)? { true } else { false } }) }, Py::str("the total duration must be positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Leq"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::truediv(&py::sum(&{ let mut out = vec![]; for t1402 in py::iter(&(py::zip(&[v_L.clone(), v_t.clone()])?))? { let mut v_ti: Py; let mut v_v: Py; 
let t1403 = (t1402).items()?;
if t1403.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1403[0].clone();
v_ti = t1403[1].clone();
out.push(py::mul(&v_ti.clone(), &py::pow(&Py::Float(10.0), &py::truediv(&v_v.clone(), &Py::Float(10.0))?)?)?); } Py::list(out) }, None)?, &v_T.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_removal(mut v_N0: Py, mut v_N: Py) -> R {
    let _ = f__pos(Py::str("N0 and N"), Py::tuple(vec![v_N0.clone(), v_N.clone()]))?;
    let _ = f__need({ let t2730 = v_N.clone(); Py::Bool({ let t2731 = v_N0.clone(); if py::lt(&t2730, &t2731)? { true } else { false } }) }, Py::str("the final count must be below the initial count"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("log_removal"), py::math1("log10", &py::truediv(&v_N0.clone(), &v_N.clone())?)?), (Py::str("percent_removal"), py::mul(&py::sub(&Py::Float(1.0), &py::truediv(&v_N.clone(), &v_N0.clone())?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_to_ppm(mut v_C: Py, mut v_M: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_ppm: Py = Py::None;
    let _ = f__pos(Py::str("C, M, T and P"), Py::tuple(vec![v_C.clone(), v_M.clone(), v_T.clone(), v_P.clone()]))?;
    v_ppm = py::mul(&py::truediv(&py::mul(&py::mul(&v_C.clone(), &c_R_GAS()?)?, &v_T.clone())?, &py::mul(&v_P.clone(), &v_M.clone())?)?, &Py::Float(1000000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ppm"), v_ppm.clone()), (Py::str("ppb"), py::mul(&v_ppm.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_ground_briggs(mut v_Q: Py, mut v_u: Py, mut v_H: Py, mut v_cls: Py, mut v_terrain: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_cz: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_sz: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let _ = f__pos(Py::str("Q, u and H"), Py::tuple(vec![v_Q.clone(), v_u.clone(), v_H.clone()]))?;
    let t2826 = (f__briggs_coef(v_cls.clone(), v_terrain.clone())?).items()?;
    if t2826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cy = t2826[0].clone();
    v_cz = t2826[1].clone();
    v_g = { let v_H = v_H.clone(); let v_Q = v_Q.clone(); let v_cls = v_cls.clone(); let v_cy = v_cy.clone(); let v_cz = v_cz.clone(); let v_g = v_g.clone(); let v_terrain = v_terrain.clone(); let v_u = v_u.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_sz: Py = Py::None;
            v_sz = f__sig_eval(v_cz.clone(), v_x.clone())?;
            return Ok(py::add(&py::neg(&f__lsig(v_cy.clone(), v_x.clone())?)?, &py::mul(&f__lsig(v_cz.clone(), v_x.clone())?, &py::sub(&py::truediv(&py::mul(&v_H.clone(), &v_H.clone())?, &py::mul(&v_sz.clone(), &v_sz.clone())?)?, &Py::Float(1.0))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_n = Py::Int(400i64);
    v_xs = { let mut out = vec![]; for t2829 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t2829;
out.push(py::mul(&Py::Float(100.0), &py::pow(&Py::Float(100.0), &py::truediv(&v_i.clone(), &v_n.clone())?)?)?); } Py::list(out) };
    let _ = f__need({ let t2830 = py::call(&v_g.clone(), &[py::getitem(&v_xs.clone(), &Py::Int(0i64))?])?; Py::Bool({ let t2831 = Py::Int(0i64); if py::gt(&t2830, &t2831)? { true } else { false } }) }, Py::str("the maximum lies closer than 100 m: outside the range of the Briggs coefficients"))?;
    v_lo = Py::None;
    for t2832 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2832;
        if ({ let t2833 = py::call(&v_g.clone(), &[py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?; Py::Bool({ let t2834 = Py::Int(0i64); if py::le(&t2833, &t2834)? { true } else { false } }) }).truthy() {
            v_lo = v_i.clone();
            break;
        }
    }
    let _ = f__need({ let t2835 = v_lo.clone(); Py::Bool({ let t2836 = Py::None; if !py::is(&t2835, &t2836) { true } else { false } }) }, Py::str("the maximum lies beyond 10 km: outside the range of the Briggs coefficients"))?;
    v_x = f__bisect(v_g.clone(), py::getitem(&v_xs.clone(), &v_lo.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_lo.clone(), &Py::Int(1i64))?)?, Py::Int(500i64))?;
    let t2863 = (Py::tuple(vec![f__sig_eval(v_cy.clone(), v_x.clone())?, f__sig_eval(v_cz.clone(), v_x.clone())?])).items()?;
    if t2863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t2863[0].clone();
    v_sz = t2863[1].clone();
    v_C = py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_u.clone())?, &v_sy.clone())?, &v_sz.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_H.clone())?, &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sz.clone())?, &v_sz.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x_max"), v_x.clone()), (Py::str("C_max"), v_C.clone()), (Py::str("C_max_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("sigma_y"), v_sy.clone()), (Py::str("sigma_z"), v_sz.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_ground_turner(mut v_Q: Py, mut v_u: Py, mut v_H: Py, mut v_k: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("Q, u, H and the ratio k = sigma_y/sigma_z"), Py::tuple(vec![v_Q.clone(), v_u.clone(), v_H.clone(), v_k.clone()]))?;
    v_C = py::truediv(&py::mul(&Py::Float(2.0), &v_Q.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &Py::Float(std::f64::consts::E))?, &v_u.clone())?, &v_H.clone())?, &v_H.clone())?, &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_max"), v_C.clone()), (Py::str("C_max_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("sigma_z_max"), py::truediv(&v_H.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_balance(mut v_Qf: Py, mut v_Qp: Py, mut v_Cf: Py, mut v_Cp: Py, mut v_A: Py) -> R {
    let mut v_Cc: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_Qc: Py = Py::None;
    let _ = f__pos(Py::str("Qf, Qp, Cf and A"), Py::tuple(vec![v_Qf.clone(), v_Qp.clone(), v_Cf.clone(), v_A.clone()]))?;
    let _ = f__nonneg(Py::str("the permeate concentration Cp"), Py::tuple(vec![v_Cp.clone()]))?;
    let _ = f__need({ let t2958 = v_Qp.clone(); Py::Bool({ let t2959 = v_Qf.clone(); if py::lt(&t2958, &t2959)? { true } else { false } }) }, Py::str("the permeate flow must be below the feed flow"))?;
    let _ = f__need({ let t2960 = v_Cp.clone(); Py::Bool({ let t2961 = v_Cf.clone(); if py::lt(&t2960, &t2961)? { true } else { false } }) }, Py::str("the permeate must be cleaner than the feed"))?;
    v_Qc = py::sub(&v_Qf.clone(), &v_Qp.clone())?;
    v_Cc = py::truediv(&py::sub(&py::mul(&v_Qf.clone(), &v_Cf.clone())?, &py::mul(&v_Qp.clone(), &v_Cp.clone())?)?, &v_Qc.clone())?;
    v_J = py::truediv(&v_Qp.clone(), &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("recovery"), py::truediv(&v_Qp.clone(), &v_Qf.clone())?), (Py::str("rejection"), py::sub(&Py::Float(1.0), &py::truediv(&v_Cp.clone(), &v_Cf.clone())?)?), (Py::str("J"), v_J.clone()), (Py::str("J_LMH"), py::mul(&v_J.clone(), &Py::Float(3600000.0))?), (Py::str("Qc"), v_Qc.clone()), (Py::str("Cc"), v_Cc.clone()), (Py::str("Cc_mgL"), py::mul(&v_Cc.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_methane_yield(mut v_COD: Py, mut v_Px: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_Qstp: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_net: Py = Py::None;
    let _ = f__pos(Py::str("COD, T and P"), Py::tuple(vec![v_COD.clone(), v_T.clone(), v_P.clone()]))?;
    let _ = f__nonneg(Py::str("Px"), Py::tuple(vec![v_Px.clone()]))?;
    v_net = py::sub(&v_COD.clone(), &py::mul(&Py::Float(1.42), &v_Px.clone())?)?;
    let _ = f__need({ let t239 = v_net.clone(); Py::Bool({ let t240 = Py::Int(0i64); if py::gt(&t239, &t240)? { true } else { false } }) }, Py::str("the biomass produced exceeds the COD removed"))?;
    v_n = py::truediv(&v_net.clone(), &py::mul(&py::mul(&Py::Float(2.0), &c_MW_O2()?)?, &Py::Float(0.001))?)?;
    v_Q = py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R_GAS()?)?, &v_T.clone())?, &v_P.clone())?;
    v_Qstp = py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R_GAS()?)?, &c_T0C()?)?, &c_P_ATM()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q_CH4"), v_Q.clone()), (Py::str("Q_CH4_m3_per_day"), py::mul(&v_Q.clone(), &c_DAY()?)?), (Py::str("Q_CH4_stp_m3_per_day"), py::mul(&v_Qstp.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_srt(mut v_Y: Py, mut v_k: Py, mut v_Ks: Py, mut v_kd: Py, mut v_S0: Py) -> R {
    let mut v_inv: Py = Py::None;
    let _ = f__pos(Py::str("Y, k, Ks and S0"), Py::tuple(vec![v_Y.clone(), v_k.clone(), v_Ks.clone(), v_S0.clone()]))?;
    let _ = f__nonneg(Py::str("kd"), Py::tuple(vec![v_kd.clone()]))?;
    v_inv = py::sub(&py::truediv(&py::mul(&py::mul(&v_Y.clone(), &v_k.clone())?, &v_S0.clone())?, &py::add(&v_Ks.clone(), &v_S0.clone())?)?, &v_kd.clone())?;
    let _ = f__need({ let t3012 = v_inv.clone(); Py::Bool({ let t3013 = Py::Int(0i64); if py::gt(&t3012, &t3013)? { true } else { false } }) }, Py::str("the organisms cannot grow on this substrate (Y k S0/(Ks + S0) <= kd)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("SRT_min"), py::truediv(&Py::Float(1.0), &v_inv.clone())?), (Py::str("SRT_min_days"), py::truediv(&py::truediv(&Py::Float(1.0), &v_inv.clone())?, &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mix_many(mut v_Q: Py, mut v_C: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_tq: Py = Py::None;
    let _ = f__need({ let t3100 = { let t3096 = py::len(&v_Q.clone())?; Py::Bool({ let t3097 = py::len(&v_C.clone())?; if py::eq(&t3096, &t3097) { true } else { false } }) }; if !t3100.truthy() { t3100 } else { { let t3098 = py::len(&v_Q.clone())?; Py::Bool({ let t3099 = Py::Int(2i64); if py::ge(&t3098, &t3099)? { true } else { false } }) } } }, Py::str("give at least two flows and as many concentrations"))?;
    for t3101 in py::iter(&(v_Q.clone()))? {
        v_q = t3101;
        let _ = f__need({ let t3104 = py::math1("isfinite", &v_q.clone())?; if !t3104.truthy() { t3104 } else { { let t3102 = v_q.clone(); Py::Bool({ let t3103 = Py::Int(0i64); if py::gt(&t3102, &t3103)? { true } else { false } }) } } }, Py::str("every flow must be positive"))?;
    }
    for t3105 in py::iter(&(v_C.clone()))? {
        v_c = t3105;
        let _ = f__need({ let t3108 = py::math1("isfinite", &v_c.clone())?; if !t3108.truthy() { t3108 } else { { let t3106 = v_c.clone(); Py::Bool({ let t3107 = Py::Int(0i64); if py::ge(&t3106, &t3107)? { true } else { false } }) } } }, Py::str("every concentration must be zero or positive"))?;
    }
    let t3109 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tq = t3109[0].clone();
    v_m = t3109[1].clone();
    for t3110 in py::iter(&(py::zip(&[v_Q.clone(), v_C.clone()])?))? {
        let t3111 = (t3110).items()?;
        if t3111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q = t3111[0].clone();
        v_c = t3111[1].clone();
        v_tq = py::iadd(&v_tq, &v_q.clone())?;
        v_m = py::iadd(&v_m, &py::mul(&v_q.clone(), &v_c.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("C_mix"), py::truediv(&v_m.clone(), &v_tq.clone())?), (Py::str("Q_total"), v_tq.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mix_two(mut v_Qw: Py, mut v_Cw: Py, mut v_Qr: Py, mut v_Cr: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("the flows Qw and Qr"), Py::tuple(vec![v_Qw.clone(), v_Qr.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations Cw and Cr"), Py::tuple(vec![v_Cw.clone(), v_Cr.clone()]))?;
    v_C = py::truediv(&py::add(&py::mul(&v_Qw.clone(), &v_Cw.clone())?, &py::mul(&v_Qr.clone(), &v_Cr.clone())?)?, &py::add(&v_Qw.clone(), &v_Qr.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("Q"), py::add(&v_Qw.clone(), &v_Qr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixing_power(mut v_G: Py, mut v_mu: Py, mut v_V: Py) -> R {
    let _ = f__pos(Py::str("G, mu and V"), Py::tuple(vec![v_G.clone(), v_mu.clone(), v_V.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::mul(&py::mul(&py::mul(&v_G.clone(), &v_G.clone())?, &v_mu.clone())?, &v_V.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_monod(mut v_mu_max: Py, mut v_Ks: Py, mut v_S: Py, mut v_kd: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("mu_max, Ks and S"), Py::tuple(vec![v_mu_max.clone(), v_Ks.clone(), v_S.clone()]))?;
    v_mu = py::truediv(&py::mul(&v_mu_max.clone(), &v_S.clone())?, &py::add(&v_Ks.clone(), &v_S.clone())?)?;
    v_out = Py::dict(vec![(Py::str("mu"), v_mu.clone()), (Py::str("mu_per_day"), py::mul(&v_mu.clone(), &c_DAY()?)?)]);
    if ({ let t3206 = v_kd.clone(); Py::Bool({ let t3207 = Py::None; if !py::is(&t3206, &t3207) { true } else { false } }) }).truthy() {
        let _ = f__nonneg(Py::str("the decay coefficient kd"), Py::tuple(vec![v_kd.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("mu_net"), py::sub(&v_mu.clone(), &v_kd.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("mu_net_per_day"), py::mul(&py::sub(&v_mu.clone(), &v_kd.clone())?, &c_DAY()?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_noise_dose(mut v_L: Py, mut v_hours: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t3360 = { let t3356 = py::len(&v_L.clone())?; Py::Bool({ let t3357 = py::len(&v_hours.clone())?; if py::eq(&t3356, &t3357) { true } else { false } }) }; if !t3360.truthy() { t3360 } else { { let t3358 = py::len(&v_L.clone())?; Py::Bool({ let t3359 = Py::Int(1i64); if py::ge(&t3358, &t3359)? { true } else { false } }) } } }, Py::str("give one exposure time per level"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    for t3366 in py::iter(&(v_hours.clone()))? {
        v_v = t3366;
        let _ = f__need({ let t3369 = py::math1("isfinite", &v_v.clone())?; if !t3369.truthy() { t3369 } else { { let t3367 = v_v.clone(); Py::Bool({ let t3368 = Py::Int(0i64); if py::ge(&t3367, &t3368)? { true } else { false } }) } } }, Py::str("exposure times must be zero or positive"))?;
    }
    v_D = Py::Float(0.0);
    for t3370 in py::iter(&(py::zip(&[v_L.clone(), v_hours.clone()])?))? {
        let t3373 = (t3370).items()?;
        if t3373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t3373[0].clone();
        v_h = t3373[1].clone();
        if ({ let t3371 = v_v.clone(); Py::Bool({ let t3372 = Py::Float(80.0); if py::ge(&t3371, &t3372)? { true } else { false } }) }).truthy() {
            v_D = py::iadd(&v_D, &py::truediv(&v_h.clone(), &py::truediv(&Py::Float(8.0), &py::pow(&Py::Float(2.0), &py::truediv(&py::sub(&v_v.clone(), &Py::Float(90.0))?, &Py::Float(5.0))?)?)?)?)?;
        }
    }
    v_D = py::mul(&v_D, &Py::Float(100.0))?;
    let _ = f__need({ let t3374 = v_D.clone(); Py::Bool({ let t3375 = Py::Int(0i64); if py::gt(&t3374, &t3375)? { true } else { false } }) }, Py::str("no exposure at or above 80 dBA: the dose is zero"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("dose_percent"), v_D.clone()), (Py::str("TWA"), py::add(&py::mul(&Py::Float(16.61), &py::math1("log10", &py::truediv(&v_D.clone(), &Py::Float(100.0))?)?)?, &Py::Float(90.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nrc_trickling(mut v_W: Py, mut v_V: Py, mut v_R: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let _ = f__pos(Py::str("W and V"), Py::tuple(vec![v_W.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the recirculation ratio R"), Py::tuple(vec![v_R.clone()]))?;
    v_F = py::truediv(&py::add(&Py::Float(1.0), &v_R.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::truediv(&v_R.clone(), &Py::Float(10.0))?)?, &Py::Int(2i64))?)?;
    v_E = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::mul(&Py::Float(0.4432), &py::math1("sqrt", &py::truediv(&py::mul(&v_W.clone(), &c_DAY()?)?, &py::mul(&v_V.clone(), &v_F.clone())?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("E"), v_E.clone()), (Py::str("E_percent"), py::mul(&v_E.clone(), &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_C: Py, mut v_M: Py, mut v_i: Py, mut v_T: Py) -> R {
    let mut v_pi: Py = Py::None;
    let _ = f__pos(Py::str("C, M, i and T"), Py::tuple(vec![v_C.clone(), v_M.clone(), v_i.clone(), v_T.clone()]))?;
    v_pi = py::mul(&py::mul(&py::truediv(&py::mul(&v_i.clone(), &v_C.clone())?, &v_M.clone())?, &c_R_GAS()?)?, &v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("pi"), v_pi.clone()), (Py::str("pi_bar"), py::truediv(&v_pi.clone(), &Py::Float(100000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_overflow_rate(mut v_Q: Py, mut v_D: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__pos(Py::str("Q and D"), Py::tuple(vec![v_Q.clone(), v_D.clone()]))?;
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Float(4.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("v0"), py::truediv(&v_Q.clone(), &v_A.clone())?), (Py::str("v0_m_per_day"), py::mul(&py::truediv(&v_Q.clone(), &v_A.clone())?, &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_demand(mut v_Q: Py, mut v_S0: Py, mut v_S: Py, mut v_f: Py, mut v_Px: Py) -> R {
    let mut v_O2: Py = Py::None;
    let _ = f__pos(Py::str("Q and S0"), Py::tuple(vec![v_Q.clone(), v_S0.clone()]))?;
    let _ = f__nonneg(Py::str("S and Px"), Py::tuple(vec![v_S.clone(), v_Px.clone()]))?;
    let _ = f__frac(Py::str("the BOD5/BODu ratio f"), v_f.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t135 = v_S.clone(); Py::Bool({ let t136 = v_S0.clone(); if py::lt(&t135, &t136)? { true } else { false } }) }, Py::str("the effluent substrate must be below the influent"))?;
    v_O2 = py::sub(&py::truediv(&py::mul(&v_Q.clone(), &py::sub(&v_S0.clone(), &v_S.clone())?)?, &v_f.clone())?, &py::mul(&Py::Float(1.42), &v_Px.clone())?)?;
    let _ = f__need({ let t137 = v_O2.clone(); Py::Bool({ let t138 = Py::Int(0i64); if py::gt(&t137, &t138)? { true } else { false } }) }, Py::str("the biomass wasted exceeds the oxygen equivalent of the BOD removed"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("O2"), v_O2.clone()), (Py::str("O2_kg_per_day"), py::mul(&v_O2.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plume_chain(mut v_Q: Py, mut v_u: Py, mut v_vs: Py, mut v_d: Py, mut v_Ts: Py, mut v_Ta: Py, mut v_hs: Py, mut v_x: Py, mut v_cls: Py, mut v_terrain: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_sz: Py = Py::None;
    let _ = f__need({ let t4539 = v_cls.clone(); Py::Bool({ let t4540 = Py::tuple(vec![Py::str("a"), Py::str("b"), Py::str("c"), Py::str("d")]); if py::contains(&t4540, &t4539)? { true } else { false } }) }, Py::str("the neutral/unstable plume-rise formula needs a class A to D"))?;
    v_r = f_briggs_rise(v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone(), v_u.clone(), v_hs.clone())?;
    v_H = py::add(&v_hs.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(1.6), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?, &py::pow(&(if ({ let t4569 = py::getitem(&v_r.clone(), &Py::str("F"))?; Py::Bool({ let t4570 = Py::Float(55.0); if py::lt(&t4569, &t4570)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(49.0), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &Py::Float(0.625))?)? } else { py::mul(&Py::Float(119.0), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &Py::Float(0.4))?)? }), &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?, &v_u.clone())?)?;
    let t4582 = (f__briggs(v_x.clone(), v_cls.clone(), v_terrain.clone())?).items()?;
    if t4582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t4582[0].clone();
    v_sz = t4582[1].clone();
    v_C = py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_u.clone())?, &v_sy.clone())?, &v_sz.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_H.clone())?, &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sz.clone())?, &v_sz.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H"), v_H.clone()), (Py::str("sigma_y"), v_sy.clone()), (Py::str("sigma_z"), v_sz.clone()), (Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plume_concentration(mut v_Q: Py, mut v_u: Py, mut v_sigma_y: Py, mut v_sigma_z: Py, mut v_H: Py, mut v_y: Py, mut v_z: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("Q, u, sigma_y and sigma_z"), Py::tuple(vec![v_Q.clone(), v_u.clone(), v_sigma_y.clone(), v_sigma_z.clone()]))?;
    let _ = f__nonneg(Py::str("the effective stack height H and the receptor height z"), Py::tuple(vec![v_H.clone(), v_z.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_y.clone()]))?;
    v_C = f__plume(v_Q.clone(), v_u.clone(), v_sigma_y.clone(), v_sigma_z.clone(), v_H.clone(), v_y.clone(), v_z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plume_lid(mut v_Q: Py, mut v_u: Py, mut v_sigma_y: Py, mut v_sigma_z: Py, mut v_H: Py, mut v_Lm: Py, mut v_y: Py, mut v_z: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Cmix: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nmax: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__pos(Py::str("Q, u, sigma_y, sigma_z and the mixing height Lm"), Py::tuple(vec![v_Q.clone(), v_u.clone(), v_sigma_y.clone(), v_sigma_z.clone(), v_Lm.clone()]))?;
    let _ = f__nonneg(Py::str("H and z"), Py::tuple(vec![v_H.clone(), v_z.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_y.clone()]))?;
    let _ = f__need({ let t3642 = { let t3638 = v_H.clone(); Py::Bool({ let t3639 = v_Lm.clone(); if py::le(&t3638, &t3639)? { true } else { false } }) }; if !t3642.truthy() { t3642 } else { { let t3640 = v_z.clone(); Py::Bool({ let t3641 = v_Lm.clone(); if py::le(&t3640, &t3641)? { true } else { false } }) } } }, Py::str("the source and the receptor must lie below the mixing height"))?;
    let _ = f__need({ let t3643 = v_sigma_z.clone(); Py::Bool({ let t3644 = py::mul(&Py::Float(10.0), &v_Lm.clone())?; if py::le(&t3643, &t3644)? { true } else { false } }) }, Py::str("sigma_z may be at most ten mixing heights (use the uniformly mixed limit beyond)"))?;
    v_tot = Py::Float(0.0);
    v_nmax = py::add(&py::int(&py::mul(&py::truediv(&v_sigma_z.clone(), &v_Lm.clone())?, &Py::Float(4.0))?)?, &Py::Int(6i64))?;
    for t3645 in py::range_iter(&[(py::neg(&v_nmax.clone())?).i()?, (py::add(&v_nmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t3645;
        for t3646 in py::iter(&(Py::tuple(vec![py::add(&py::sub(&v_z.clone(), &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_n.clone())?, &v_Lm.clone())?)?, py::add(&py::add(&v_z.clone(), &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_n.clone())?, &v_Lm.clone())?)?])))? {
            v_a = t3646;
            v_tot = py::iadd(&v_tot, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_a.clone())?, &v_a.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sigma_z.clone())?, &v_sigma_z.clone())?)?)?)?;
        }
    }
    v_C = py::mul(&py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_u.clone())?, &v_sigma_y.clone())?, &v_sigma_z.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_y.clone())?, &v_y.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sigma_y.clone())?, &v_sigma_y.clone())?)?)?)?, &v_tot.clone())?;
    v_Cmix = py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&py::math1("sqrt", &py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?)?, &v_u.clone())?, &v_sigma_y.clone())?, &v_Lm.clone())?)?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_y.clone())?, &v_y.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_sigma_y.clone())?, &v_sigma_y.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("C_mixed"), v_Cmix.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plume_with_class(mut v_Q: Py, mut v_u: Py, mut v_H: Py, mut v_x: Py, mut v_y: Py, mut v_z: Py, mut v_cls: Py, mut v_terrain: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_sz: Py = Py::None;
    let _ = f__pos(Py::str("Q and u"), Py::tuple(vec![v_Q.clone(), v_u.clone()]))?;
    let _ = f__nonneg(Py::str("H and z"), Py::tuple(vec![v_H.clone(), v_z.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_y.clone()]))?;
    let t3584 = (f__briggs(v_x.clone(), v_cls.clone(), v_terrain.clone())?).items()?;
    if t3584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t3584[0].clone();
    v_sz = t3584[1].clone();
    v_C = f__plume(v_Q.clone(), v_u.clone(), v_sy.clone(), v_sz.clone(), v_H.clone(), v_y.clone(), v_z.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("sigma_y"), v_sy.clone()), (Py::str("sigma_z"), v_sz.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pollutant_load(mut v_Q: Py, mut v_C: Py) -> R {
    let mut v_W: Py = Py::None;
    let _ = f__pos(Py::str("the flow Q"), Py::tuple(vec![v_Q.clone()]))?;
    let _ = f__nonneg(Py::str("the concentration C"), Py::tuple(vec![v_C.clone()]))?;
    v_W = py::mul(&v_Q.clone(), &v_C.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("W"), v_W.clone()), (Py::str("W_kg_per_day"), py::mul(&v_W.clone(), &c_DAY()?)?), (Py::str("W_t_per_year"), py::truediv(&py::mul(&v_W.clone(), &c_YEAR()?)?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ppm_to_mass(mut v_ppm: Py, mut v_M: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("ppm, M, T and P"), Py::tuple(vec![v_ppm.clone(), v_M.clone(), v_T.clone(), v_P.clone()]))?;
    v_C = py::truediv(&py::mul(&py::mul(&py::mul(&v_ppm.clone(), &Py::Float(1e-06))?, &v_P.clone())?, &v_M.clone())?, &py::mul(&c_R_GAS()?, &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("C_mgm3"), py::mul(&v_C.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaeration(mut v_method: Py, mut v_u: Py, mut v_H: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_kday: Py = Py::None;
    let _ = f__need({ let t3829 = v_method.clone(); Py::Bool({ let t3830 = c__REAER()?; if py::contains(&t3830, &t3829)? { true } else { false } }) }, Py::str("method: o_connor_dobbins, churchill or owens_gibbs"))?;
    let _ = f__pos(Py::str("u and H"), Py::tuple(vec![v_u.clone(), v_H.clone()]))?;
    let t3837 = (py::getitem(&c__REAER()?, &v_method.clone())?).items()?;
    if t3837.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3837[0].clone();
    v_a = t3837[1].clone();
    v_b = t3837[2].clone();
    v_kday = py::truediv(&py::mul(&v_c.clone(), &py::pow(&v_u.clone(), &v_a.clone())?)?, &py::pow(&v_H.clone(), &v_b.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ka"), py::truediv(&v_kday.clone(), &c_DAY()?)?), (Py::str("ka_per_day"), v_kday.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_removal_fraction(mut v_vs: Py, mut v_Q: Py, mut v_A: Py) -> R {
    let mut v_v0: Py = Py::None;
    let _ = f__pos(Py::str("vs, Q and A"), Py::tuple(vec![v_vs.clone(), v_Q.clone(), v_A.clone()]))?;
    v_v0 = py::truediv(&v_Q.clone(), &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("v0"), v_v0.clone()), (Py::str("R"), py::minmax(&[Py::Float(1.0), py::truediv(&v_vs.clone(), &v_v0.clone())?], false)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_return_ratio(mut v_X: Py, mut v_Xr: Py, mut v_Q: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("X, Xr and Q"), Py::tuple(vec![v_X.clone(), v_Xr.clone(), v_Q.clone()]))?;
    let _ = f__need({ let t3882 = v_Xr.clone(); Py::Bool({ let t3883 = v_X.clone(); if py::gt(&t3882, &t3883)? { true } else { false } }) }, Py::str("the return sludge must be more concentrated than the mixed liquor"))?;
    v_R = py::truediv(&v_X.clone(), &py::sub(&v_Xr.clone(), &v_X.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("Qr"), py::mul(&v_R.clone(), &v_Q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_river_decay(mut v_C0: Py, mut v_k: Py, mut v_x: Py, mut v_u: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("C0, k and u"), Py::tuple(vec![v_C0.clone(), v_k.clone(), v_u.clone()]))?;
    let _ = f__nonneg(Py::str("the distance x"), Py::tuple(vec![v_x.clone()]))?;
    v_C = py::mul(&v_C0.clone(), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_k.clone())?, &v_x.clone())?, &v_u.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("t"), py::truediv(&v_x.clone(), &v_u.clone())?), (Py::str("fraction_remaining"), py::truediv(&v_C.clone(), &v_C0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sag_distance(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_x: Py, mut v_v: Py, mut v_DOsat: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the river velocity v"), Py::tuple(vec![v_v.clone()]))?;
    let _ = f__nonneg(Py::str("the distance x"), Py::tuple(vec![v_x.clone()]))?;
    v_r = f_streeter_phelps(v_kd.clone(), v_ka.clone(), v_L0.clone(), v_D0.clone(), py::truediv(&v_x.clone(), &v_v.clone())?, v_DOsat.clone())?;
    py::setitem(&v_r.clone(), &Py::str("t"), f__r(py::truediv(&v_x.clone(), &v_v.clone())?)?)?;
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sed_basin(mut v_Q: Py, mut v_v0: Py, mut v_theta: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::str("Q, v0 and theta"), Py::tuple(vec![v_Q.clone(), v_v0.clone(), v_theta.clone()]))?;
    v_A = py::truediv(&v_Q.clone(), &v_v0.clone())?;
    v_V = py::mul(&v_Q.clone(), &v_theta.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("V"), v_V.clone()), (Py::str("depth"), py::truediv(&v_V.clone(), &v_A.clone())?), (Py::str("D"), py::math1("sqrt", &py::truediv(&py::mul(&Py::Float(4.0), &v_A.clone())?, &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_settling_chamber(mut v_dp: Py, mut v_rho_p: Py, mut v_rho_g: Py, mut v_mu: Py, mut v_L: Py, mut v_W: Py, mut v_Q: Py, mut v_N: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_vt: Py = Py::None;
    let _ = f__pos(Py::str("dp, rho_p, rho_g, mu, L, W and Q"), Py::tuple(vec![v_dp.clone(), v_rho_p.clone(), v_rho_g.clone(), v_mu.clone(), v_L.clone(), v_W.clone(), v_Q.clone()]))?;
    let _ = f__need({ let t4022 = v_rho_p.clone(); Py::Bool({ let t4023 = v_rho_g.clone(); if py::gt(&t4022, &t4023)? { true } else { false } }) }, Py::str("the particles must be denser than the gas"))?;
    let _ = f__need({ let t4026 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4026.truthy() { t4026 } else { { let t4024 = v_N.clone(); Py::Bool({ let t4025 = Py::Int(1i64); if py::ge(&t4024, &t4025)? { true } else { false } }) } } }, Py::str("the number of trays N must be a whole number of at least 1"))?;
    v_vt = f__stokes_v(v_dp.clone(), v_rho_p.clone(), v_rho_g.clone(), v_mu.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho_g.clone(), &v_vt.clone())?, &v_dp.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t4027 = v_Re.clone(); Py::Bool({ let t4028 = Py::Float(1.0); if py::lt(&t4027, &t4028)? { true } else { false } }) }, Py::str("Stokes' law needs a particle Reynolds number below 1"))?;
    v_eta = py::minmax(&[Py::Float(1.0), py::truediv(&py::mul(&py::mul(&py::mul(&v_vt.clone(), &v_L.clone())?, &v_W.clone())?, &v_N.clone())?, &v_Q.clone())?], false)?;
    return Ok(f__out(Py::dict(vec![(Py::str("vt"), v_vt.clone()), (Py::str("eta"), v_eta.clone()), (Py::str("Re"), v_Re.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sludge_production(mut v_Y: Py, mut v_kd: Py, mut v_SRT: Py, mut v_Q: Py, mut v_S0: Py, mut v_S: Py) -> R {
    let mut v_Px: Py = Py::None;
    let mut v_yo: Py = Py::None;
    let _ = f__pos(Py::str("Y, SRT, Q and S0"), Py::tuple(vec![v_Y.clone(), v_SRT.clone(), v_Q.clone(), v_S0.clone()]))?;
    let _ = f__nonneg(Py::str("kd and S"), Py::tuple(vec![v_kd.clone(), v_S.clone()]))?;
    let _ = f__need({ let t4079 = v_S.clone(); Py::Bool({ let t4080 = v_S0.clone(); if py::lt(&t4079, &t4080)? { true } else { false } }) }, Py::str("the effluent substrate must be below the influent"))?;
    v_yo = py::truediv(&v_Y.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_kd.clone(), &v_SRT.clone())?)?)?;
    v_Px = py::mul(&py::mul(&v_yo.clone(), &v_Q.clone())?, &py::sub(&v_S0.clone(), &v_S.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Y_obs"), v_yo.clone()), (Py::str("Px"), v_Px.clone()), (Py::str("Px_kg_per_day"), py::mul(&v_Px.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sludge_volume(mut v_Ms: Py, mut v_Ps: Py, mut v_Ssl: Py) -> R {
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::str("Ms and Ssl"), Py::tuple(vec![v_Ms.clone(), v_Ssl.clone()]))?;
    let _ = f__frac(Py::str("the solids fraction Ps"), v_Ps.clone(), Py::Bool(true), Py::Bool(false))?;
    v_V = py::truediv(&v_Ms.clone(), &py::mul(&py::mul(&Py::Float(1000.0), &v_Ssl.clone())?, &v_Ps.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_m3_per_day"), py::mul(&v_V.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_so2_emission(mut v_m_fuel: Py, mut v_S: Py, mut v_eta: Py) -> R {
    let mut v_E0: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("the fuel rate m_fuel"), Py::tuple(vec![v_m_fuel.clone()]))?;
    let _ = f__frac(Py::str("the sulfur fraction S"), v_S.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("the control efficiency eta"), v_eta.clone(), Py::Bool(false), Py::Bool(true))?;
    v_ratio = py::truediv(&py::add(&py::getitem(&c_AW()?, &Py::str("S"))?, &py::mul(&Py::Int(2i64), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?, &py::getitem(&c_AW()?, &Py::str("S"))?)?;
    v_E0 = py::mul(&py::mul(&v_m_fuel.clone(), &v_S.clone())?, &v_ratio.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), py::mul(&v_E0.clone(), &py::sub(&Py::Float(1.0), &v_eta.clone())?)?), (Py::str("E_uncontrolled"), v_E0.clone()), (Py::str("E_kg_per_day"), py::mul(&py::mul(&v_E0.clone(), &py::sub(&Py::Float(1.0), &v_eta.clone())?)?, &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sound_power_to_pressure(mut v_Lw: Py, mut v_r: Py, mut v_Qd: Py) -> R {
    let _ = f__db_list(Py::list(vec![v_Lw.clone()]), Py::str("Lw"))?;
    let _ = f__pos(Py::str("r and the directivity Qd"), Py::tuple(vec![v_r.clone(), v_Qd.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Lp"), py::add(&v_Lw.clone(), &py::mul(&Py::Float(10.0), &py::math1("log10", &py::truediv(&v_Qd.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_r.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sp_critical(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_DOsat: Py) -> R {
    let mut v_Dc: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let _ = f__sp_checks(v_kd.clone(), v_ka.clone(), v_L0.clone(), v_D0.clone(), v_DOsat.clone())?;
    if ({ let t4671 = py::sub(&py::mul(&v_kd.clone(), &v_L0.clone())?, &py::mul(&v_ka.clone(), &v_D0.clone())?)?; Py::Bool({ let t4672 = Py::Int(0i64); if py::le(&t4671, &t4672)? { true } else { false } }) }).truthy() {
        v_tc = Py::Float(0.0);
    } else {
        if ({ let t4673 = py::abs(&py::sub(&v_ka.clone(), &v_kd.clone())?)?; Py::Bool({ let t4674 = py::mul(&Py::Float(1e-09), &v_ka.clone())?; if py::le(&t4673, &t4674)? { true } else { false } }) }).truthy() {
            v_tc = py::truediv(&py::sub(&Py::Float(1.0), &py::truediv(&v_D0.clone(), &v_L0.clone())?)?, &v_kd.clone())?;
        } else {
            v_tc = py::truediv(&py::math1("log", &py::mul(&py::truediv(&v_ka.clone(), &v_kd.clone())?, &py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&v_D0.clone(), &py::sub(&v_ka.clone(), &v_kd.clone())?)?, &py::mul(&v_kd.clone(), &v_L0.clone())?)?)?)?)?, &py::sub(&v_ka.clone(), &v_kd.clone())?)?;
        }
    }
    v_Dc = (if ({ let t4675 = v_tc.clone(); Py::Bool({ let t4676 = Py::Float(0.0); if py::eq(&t4675, &t4676) { true } else { false } }) }).truthy() { v_D0.clone() } else { py::mul(&py::mul(&py::truediv(&v_kd.clone(), &v_ka.clone())?, &v_L0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_tc.clone())?)?)? });
    let _ = f__need({ let t4677 = v_Dc.clone(); Py::Bool({ let t4678 = v_DOsat.clone(); if py::le(&t4677, &t4678)? { true } else { false } }) }, Py::str("the critical deficit exceeds saturation: the river turns anaerobic and the model no longer holds"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_c"), v_tc.clone()), (Py::str("t_c_days"), py::truediv(&v_tc.clone(), &c_DAY()?)?), (Py::str("D_c"), v_Dc.clone()), (Py::str("DO_min"), py::sub(&v_DOsat.clone(), &v_Dc.clone())?), (Py::str("DO_min_mgL"), py::mul(&py::sub(&v_DOsat.clone(), &v_Dc.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_srt(mut v_V: Py, mut v_X: Py, mut v_Qw: Py, mut v_Xw: Py, mut v_Qe: Py, mut v_Xe: Py) -> R {
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("V, X, Qw and Xw"), Py::tuple(vec![v_V.clone(), v_X.clone(), v_Qw.clone(), v_Xw.clone()]))?;
    let _ = f__nonneg(Py::str("Qe and Xe"), Py::tuple(vec![v_Qe.clone(), v_Xe.clone()]))?;
    v_th = py::truediv(&py::mul(&v_V.clone(), &v_X.clone())?, &py::add(&py::mul(&v_Qw.clone(), &v_Xw.clone())?, &py::mul(&v_Qe.clone(), &v_Xe.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("SRT"), v_th.clone()), (Py::str("SRT_days"), py::truediv(&v_th.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stability_class(mut v_u: Py, mut v_condition: Py) -> R {
    let mut v_col: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the surface wind speed u"), Py::tuple(vec![v_u.clone()]))?;
    let _ = f__need({ let t4487 = v_condition.clone(); Py::Bool({ let t4488 = c__STAB_ALL()?; if py::contains(&t4488, &t4487)? { true } else { false } }) }, Py::str("condition: strong, moderate or slight insolation, overcast, night_cloudy or night_clear"))?;
    if ({ let t4489 = v_condition.clone(); Py::Bool({ let t4490 = Py::str("overcast"); if py::eq(&t4489, &t4490) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("stability"), Py::str("D"))]))?);
    }
    if ({ let t4503 = v_u.clone(); Py::Bool({ let t4504 = Py::Float(2.0); if py::lt(&t4503, &t4504)? { true } else { false } }) }).truthy() {
        v_row = Py::tuple(vec![Py::str("A"), Py::str("A-B"), Py::str("B"), Py::str(""), Py::str("")]);
    } else {
        if ({ let t4505 = v_u.clone(); Py::Bool({ let t4506 = Py::Float(3.0); if py::lt(&t4505, &t4506)? { true } else { false } }) }).truthy() {
            v_row = Py::tuple(vec![Py::str("A-B"), Py::str("B"), Py::str("C"), Py::str("E"), Py::str("F")]);
        } else {
            if ({ let t4507 = v_u.clone(); Py::Bool({ let t4508 = Py::Float(5.0); if py::lt(&t4507, &t4508)? { true } else { false } }) }).truthy() {
                v_row = Py::tuple(vec![Py::str("B"), Py::str("B-C"), Py::str("C"), Py::str("D"), Py::str("E")]);
            } else {
                if ({ let t4509 = v_u.clone(); Py::Bool({ let t4510 = Py::Float(6.0); if py::lt(&t4509, &t4510)? { true } else { false } }) }).truthy() {
                    v_row = Py::tuple(vec![Py::str("C"), Py::str("C-D"), Py::str("D"), Py::str("D"), Py::str("D")]);
                } else {
                    v_row = Py::tuple(vec![Py::str("C"), Py::str("D"), Py::str("D"), Py::str("D"), Py::str("D")]);
                }
            }
        }
    }
    v_col = py::getitem(&Py::dict(vec![(Py::str("strong"), Py::Int(0i64)), (Py::str("moderate"), Py::Int(1i64)), (Py::str("slight"), Py::Int(2i64)), (Py::str("night_cloudy"), Py::Int(3i64)), (Py::str("night_clear"), Py::Int(4i64))]), &v_condition.clone())?;
    v_s = py::getitem(&v_row.clone(), &v_col.clone())?;
    let _ = f__need({ let t4511 = v_s.clone(); Py::Bool({ let t4512 = Py::str(""); if py::ne(&t4511, &t4512) { true } else { false } }) }, Py::str("the table gives no class for a night with a surface wind below 2 m/s"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("stability"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stokes(mut v_d: Py, mut v_rho_p: Py, mut v_rho_w: Py, mut v_mu: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__pos(Py::str("d, rho_p, rho_w and mu"), Py::tuple(vec![v_d.clone(), v_rho_p.clone(), v_rho_w.clone(), v_mu.clone()]))?;
    let _ = f__need({ let t4615 = v_rho_p.clone(); Py::Bool({ let t4616 = v_rho_w.clone(); if py::gt(&t4615, &t4616)? { true } else { false } }) }, Py::str("the particle must be denser than the water"))?;
    v_vs = f__stokes_v(v_d.clone(), v_rho_p.clone(), v_rho_w.clone(), v_mu.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho_w.clone(), &v_vs.clone())?, &v_d.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t4617 = v_Re.clone(); Py::Bool({ let t4618 = Py::Float(1.0); if py::le(&t4617, &t4618)? { true } else { false } }) }, Py::str("Stokes' law needs a particle Reynolds number of at most 1 (use the general terminal velocity)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("vs"), v_vs.clone()), (Py::str("Re"), v_Re.clone()), (Py::str("vs_m_per_h"), py::mul(&v_vs.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_streeter_phelps(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_t: Py, mut v_DOsat: Py) -> R {
    let mut v_D: Py = Py::None;
    let _ = f__sp_checks(v_kd.clone(), v_ka.clone(), v_L0.clone(), v_D0.clone(), v_DOsat.clone())?;
    let _ = f__nonneg(Py::str("the travel time t"), Py::tuple(vec![v_t.clone()]))?;
    v_D = f__sp_deficit(v_kd.clone(), v_ka.clone(), v_L0.clone(), v_D0.clone(), v_t.clone())?;
    let _ = f__need({ let t4733 = v_D.clone(); Py::Bool({ let t4734 = v_DOsat.clone(); if py::le(&t4733, &t4734)? { true } else { false } }) }, Py::str("the computed deficit exceeds saturation: the river turns anaerobic and the model no longer holds"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("DO"), py::sub(&v_DOsat.clone(), &v_D.clone())?), (Py::str("D_mgL"), py::mul(&v_D.clone(), &Py::Float(1000.0))?), (Py::str("DO_mgL"), py::mul(&py::sub(&v_DOsat.clone(), &v_D.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_svi(mut v_V30: Py, mut v_MLSS: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("V30 and MLSS"), Py::tuple(vec![v_V30.clone(), v_MLSS.clone()]))?;
    let _ = f__need({ let t4239 = v_V30.clone(); Py::Bool({ let t4240 = Py::Float(1000.0); if py::le(&t4239, &t4240)? { true } else { false } }) }, Py::str("the settled volume cannot exceed 1000 mL/L"))?;
    v_s = py::truediv(&v_V30.clone(), &v_MLSS.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("SVI"), v_s.clone()), (Py::str("Xr_estimate"), py::mul(&py::truediv(&Py::Float(1.0), &v_s.clone())?, &Py::Float(1000.0))?), (Py::str("Xr_estimate_mgL"), py::truediv(&Py::Float(1000000.0), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_terminal_velocity(mut v_d: Py, mut v_rho_p: Py, mut v_rho_f: Py, mut v_mu: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vst: Py = Py::None;
    let _ = f__pos(Py::str("d, rho_p, rho_f and mu"), Py::tuple(vec![v_d.clone(), v_rho_p.clone(), v_rho_f.clone(), v_mu.clone()]))?;
    let _ = f__need({ let t4779 = v_rho_p.clone(); Py::Bool({ let t4780 = v_rho_f.clone(); if py::gt(&t4779, &t4780)? { true } else { false } }) }, Py::str("the particle must be denser than the fluid"))?;
    v_rhs = py::truediv(&py::mul(&py::mul(&py::mul(&py::truediv(&Py::Float(4.0), &Py::Float(3.0))?, &c_G0()?)?, &v_d.clone())?, &py::sub(&v_rho_p.clone(), &v_rho_f.clone())?)?, &v_rho_f.clone())?;
    v_vst = f__stokes_v(v_d.clone(), v_rho_p.clone(), v_rho_f.clone(), v_mu.clone())?;
    v_f = { let v_d = v_d.clone(); let v_f = v_f.clone(); let v_mu = v_mu.clone(); let v_rho_f = v_rho_f.clone(); let v_rho_p = v_rho_p.clone(); let v_rhs = v_rhs.clone(); let v_vst = v_vst.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&v_rhs.clone(), &py::mul(&py::mul(&f__rouse_cd(py::truediv(&py::mul(&py::mul(&v_rho_f.clone(), &v_v.clone())?, &v_d.clone())?, &v_mu.clone())?)?, &v_v.clone())?, &v_v.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_v = f__bisect(v_f.clone(), py::mul(&v_vst.clone(), &Py::Float(1e-12))?, v_vst.clone(), Py::Int(500i64))?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho_f.clone(), &v_v.clone())?, &v_d.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t4807 = v_Re.clone(); Py::Bool({ let t4808 = Py::Float(200000.0); if py::le(&t4807, &t4808)? { true } else { false } }) }, Py::str("the drag law holds up to a Reynolds number of 2e5"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("vt"), v_v.clone()), (Py::str("Re"), v_Re.clone()), (Py::str("Cd"), f__rouse_cd(v_Re.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thickening(mut v_V1: Py, mut v_P1: Py, mut v_P2: Py) -> R {
    let _ = f__pos(Py::str("V1"), Py::tuple(vec![v_V1.clone()]))?;
    let _ = f__frac(Py::str("P1"), v_P1.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("P2"), v_P2.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t4137 = v_P2.clone(); Py::Bool({ let t4138 = v_P1.clone(); if py::gt(&t4137, &t4138)? { true } else { false } }) }, Py::str("thickening must raise the solids fraction (P2 > P1)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V2"), py::truediv(&py::mul(&v_V1.clone(), &v_P1.clone())?, &v_P2.clone())?), (Py::str("reduction"), py::sub(&Py::Float(1.0), &py::truediv(&v_P1.clone(), &v_P2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thod(mut v_formula: Py, mut v_nitrification: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4874 = (f__formula_counts(v_formula.clone())?).items()?;
    if t4874.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4874[0].clone();
    v_b = t4874[1].clone();
    v_c = t4874[2].clone();
    v_d = t4874[3].clone();
    let _ = f__need({ let t4875 = v_nitrification.clone(); Py::Bool({ let t4876 = Py::tuple(vec![Py::str("yes"), Py::str("no")]); if py::contains(&t4876, &t4875)? { true } else { false } }) }, Py::str("nitrification: yes or no"))?;
    let _ = f__need({ let t4877 = v_b.clone(); Py::Bool({ let t4878 = py::mul(&Py::Int(3i64), &v_d.clone())?; if py::ge(&t4877, &t4878)? { true } else { false } }) }, Py::str("too little hydrogen to form ammonia from the nitrogen"))?;
    v_n = py::add(&py::sub(&py::sub(&py::add(&v_a.clone(), &py::truediv(&v_b.clone(), &Py::Float(4.0))?)?, &py::truediv(&v_c.clone(), &Py::Float(2.0))?)?, &py::mul(&Py::Float(0.75), &v_d.clone())?)?, &(if ({ let t4879 = v_nitrification.clone(); Py::Bool({ let t4880 = Py::str("yes"); if py::eq(&t4879, &t4880) { true } else { false } }) }).truthy() { py::mul(&Py::Float(2.0), &v_d.clone())? } else { Py::Float(0.0) }))?;
    let _ = f__need({ let t4881 = v_n.clone(); Py::Bool({ let t4882 = Py::Int(0i64); if py::gt(&t4881, &t4882)? { true } else { false } }) }, Py::str("the compound needs no oxygen to be oxidised"))?;
    v_M = py::add(&py::add(&py::add(&py::mul(&v_a.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_b.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?, &py::mul(&v_c.clone(), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?, &py::mul(&v_d.clone(), &py::getitem(&c_AW()?, &Py::str("N"))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("thod"), py::truediv(&py::mul(&v_n.clone(), &c_MW_O2()?)?, &v_M.clone())?), (Py::str("o2_per_mol"), v_n.clone()), (Py::str("molar_mass"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_film(mut v_kl: Py, mut v_kg: Py, mut v_H: Py, mut v_C: Py, mut v_Cg: Py) -> R {
    let mut v_KL: Py = Py::None;
    let _ = f__pos(Py::str("kl, kg and H"), Py::tuple(vec![v_kl.clone(), v_kg.clone(), v_H.clone()]))?;
    let _ = f__nonneg(Py::str("C and Cg"), Py::tuple(vec![v_C.clone(), v_Cg.clone()]))?;
    v_KL = py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&Py::Float(1.0), &v_kl.clone())?, &py::truediv(&Py::Float(1.0), &py::mul(&v_H.clone(), &v_kg.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("KL"), v_KL.clone()), (Py::str("flux"), py::mul(&v_KL.clone(), &py::sub(&v_C.clone(), &py::truediv(&v_Cg.clone(), &v_H.clone())?)?)?), (Py::str("liquid_resistance_fraction"), py::truediv(&v_KL.clone(), &v_kl.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ultimate_bod(mut v_BOD_t: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_L0: Py = Py::None;
    let _ = f__pos(Py::str("BOD_t, k and t"), Py::tuple(vec![v_BOD_t.clone(), v_k.clone(), v_t.clone()]))?;
    v_L0 = py::truediv(&v_BOD_t.clone(), &py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L0"), v_L0.clone()), (Py::str("L0_mgL"), py::mul(&v_L0.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_velocity_gradient(mut v_P: Py, mut v_mu: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_G: Py = Py::None;
    let _ = f__pos(Py::str("P, mu and V"), Py::tuple(vec![v_P.clone(), v_mu.clone(), v_V.clone()]))?;
    v_G = py::math1("sqrt", &py::truediv(&v_P.clone(), &py::mul(&v_mu.clone(), &v_V.clone())?)?)?;
    if ({ let t5021 = v_t.clone(); Py::Bool({ let t5022 = Py::None; if py::is(&t5021, &t5022) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone())]))?);
    }
    let _ = f__pos(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("Gt"), py::mul(&v_G.clone(), &v_t.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_waste_generation(mut v_pop: Py, mut v_rate: Py, mut v_rho: Py, mut v_Vt: Py) -> R {
    let mut v_m_day: Py = Py::None;
    let mut v_vol_day: Py = Py::None;
    let _ = f__need({ let t4337 = Py::Bool(matches!(v_pop.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4337.truthy() { t4337 } else { { let t4335 = v_pop.clone(); Py::Bool({ let t4336 = Py::Int(1i64); if py::ge(&t4335, &t4336)? { true } else { false } }) } } }, Py::str("the population must be a whole number of at least 1"))?;
    let _ = f__pos(Py::str("rate, rho and Vt"), Py::tuple(vec![v_rate.clone(), v_rho.clone(), v_Vt.clone()]))?;
    v_m_day = py::mul(&v_pop.clone(), &v_rate.clone())?;
    v_vol_day = py::truediv(&v_m_day.clone(), &v_rho.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), py::truediv(&v_m_day.clone(), &c_DAY()?)?), (Py::str("m_kg_per_day"), v_m_day.clone()), (Py::str("m_t_per_year"), py::truediv(&py::mul(&v_m_day.clone(), &Py::Float(365.0))?, &Py::Float(1000.0))?), (Py::str("V_m3_per_day"), v_vol_day.clone()), (Py::str("trucks_per_day"), py::math1("ceil", &py::sub(&py::truediv(&v_vol_day.clone(), &v_Vt.clone())?, &Py::Float(1e-12))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_waste_moisture(mut v_m: Py, mut v_w: Py) -> R {
    let mut v_dry: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_water: Py = Py::None;
    let _ = f__need({ let t5065 = { let t5061 = py::len(&v_m.clone())?; Py::Bool({ let t5062 = py::len(&v_w.clone())?; if py::eq(&t5061, &t5062) { true } else { false } }) }; if !t5065.truthy() { t5065 } else { { let t5063 = py::len(&v_m.clone())?; Py::Bool({ let t5064 = Py::Int(1i64); if py::ge(&t5063, &t5064)? { true } else { false } }) } } }, Py::str("give one moisture fraction per component mass"))?;
    for t5066 in py::iter(&(v_m.clone()))? {
        v_v = t5066;
        let _ = f__need({ let t5069 = py::math1("isfinite", &v_v.clone())?; if !t5069.truthy() { t5069 } else { { let t5067 = v_v.clone(); Py::Bool({ let t5068 = Py::Int(0i64); if py::ge(&t5067, &t5068)? { true } else { false } }) } } }, Py::str("every mass must be zero or positive"))?;
    }
    for t5070 in py::iter(&(v_w.clone()))? {
        v_v = t5070;
        let _ = f__need({ let t5074 = py::math1("isfinite", &v_v.clone())?; if !t5074.truthy() { t5074 } else { { let t5071 = Py::Int(0i64); Py::Bool({ let t5072 = v_v.clone(); if py::le(&t5071, &t5072)? { { let t5073 = Py::Int(1i64); if py::lt(&t5072, &t5073)? { true } else { false } } } else { false } }) } } }, Py::str("every moisture fraction must lie between 0 and 1"))?;
    }
    v_tot = py::sum(&v_m.clone(), None)?;
    let _ = f__need({ let t5075 = v_tot.clone(); Py::Bool({ let t5076 = Py::Int(0i64); if py::gt(&t5075, &t5076)? { true } else { false } }) }, Py::str("the total mass must be positive"))?;
    v_water = py::sum(&{ let mut out = vec![]; for t5077 in py::iter(&(py::zip(&[v_m.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5078 = (t5077).items()?;
if t5078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5078[0].clone();
v_b = t5078[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_dry = py::sub(&v_tot.clone(), &v_water.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("moisture_wet"), py::truediv(&v_water.clone(), &v_tot.clone())?), (Py::str("dry_mass"), v_dry.clone()), (Py::str("moisture_dry"), py::truediv(&v_water.clone(), &v_dry.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weir_loading(mut v_Q: Py, mut v_D: Py) -> R {
    let mut v_WL: Py = Py::None;
    let _ = f__pos(Py::str("Q and D"), Py::tuple(vec![v_Q.clone(), v_D.clone()]))?;
    v_WL = py::truediv(&v_Q.clone(), &py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("weir_length"), py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?), (Py::str("WL"), v_WL.clone()), (Py::str("WL_m3_per_m_day"), py::mul(&v_WL.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_power_law(mut v_u1: Py, mut v_z1: Py, mut v_z2: Py, mut v_p: Py) -> R {
    let _ = f__pos(Py::str("u1, z1 and z2"), Py::tuple(vec![v_u1.clone(), v_z1.clone(), v_z2.clone()]))?;
    let _ = f__need({ let t5217 = Py::Float(0.0); Py::Bool({ let t5218 = v_p.clone(); if py::lt(&t5217, &t5218)? { { let t5219 = Py::Float(1.0); if py::lt(&t5218, &t5219)? { true } else { false } } } else { false } }) }, Py::str("the exponent p must lie between 0 and 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("u2"), py::mul(&v_u1.clone(), &py::pow(&py::truediv(&v_z2.clone(), &v_z1.clone())?, &v_p.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1037(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1038 = v_inputs.clone(); f_cyclone_lapple(py::getitem(&t1038, &Py::str("mu"))?, py::getitem(&t1038, &Py::str("W"))?, py::getitem(&t1038, &Py::str("Ne"))?, py::getitem(&t1038, &Py::str("Vi"))?, py::getitem(&t1038, &Py::str("rho_p"))?, py::getitem(&t1038, &Py::str("rho_g"))?, py::getitem(&t1038, &Py::str("dp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1040 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1062 = (t1040).items()?;
        if t1062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1062[0].clone();
        v_v = t1062[1].clone();
        if ({ let t1041 = v_k.clone(); Py::Bool({ let t1042 = v_outputs.clone(); if !py::contains(&t1042, &t1041)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1101 = v_inputs.clone(); f_ldn(py::getitem(&t1101, &Py::str("L"))?, py::getitem(&t1101, &Py::str("metric"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1125 = (t1103).items()?;
        if t1125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1125[0].clone();
        v_v = t1125[1].clone();
        if ({ let t1104 = v_k.clone(); Py::Bool({ let t1105 = v_outputs.clone(); if !py::contains(&t1105, &t1104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1146(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1148: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1147 = v_inputs.clone(); f_db_add(py::getitem(&t1147, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1148 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1149 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1171 = (t1149).items()?;
        if t1171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1171[0].clone();
        v_v = t1171[1].clone();
        if ({ let t1150 = v_k.clone(); Py::Bool({ let t1151 = v_outputs.clone(); if !py::contains(&t1151, &t1150)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1190(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1192: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1191 = v_inputs.clone(); f_detention_time(py::getitem(&t1191, &Py::str("V"))?, py::getitem(&t1191, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1192 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1193 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1215 = (t1193).items()?;
        if t1215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1215[0].clone();
        v_v = t1215[1].clone();
        if ({ let t1194 = v_k.clone(); Py::Bool({ let t1195 = v_outputs.clone(); if !py::contains(&t1195, &t1194)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1248: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1247 = v_inputs.clone(); f_ct_value(py::getitem(&t1247, &Py::str("C"))?, py::getitem(&t1247, &Py::str("V"))?, py::getitem(&t1247, &Py::str("Q"))?, py::getitem(&t1247, &Py::str("BF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1248 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1249 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1271 = (t1249).items()?;
        if t1271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1271[0].clone();
        v_v = t1271[1].clone();
        if ({ let t1250 = v_k.clone(); Py::Bool({ let t1251 = v_outputs.clone(); if !py::contains(&t1251, &t1250)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1290(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1292: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1291 = v_inputs.clone(); f_do_saturation(py::getitem(&t1291, &Py::str("T"))?, py::getitem(&t1291, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1292 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1293 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1315 = (t1293).items()?;
        if t1315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1315[0].clone();
        v_v = t1315[1].clone();
        if ({ let t1294 = v_k.clone(); Py::Bool({ let t1295 = v_outputs.clone(); if !py::contains(&t1295, &t1294)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1348(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1350: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1349 = v_inputs.clone(); f_dulong(py::getitem(&t1349, &Py::str("C"))?, py::getitem(&t1349, &Py::str("H"))?, py::getitem(&t1349, &Py::str("O"))?, py::getitem(&t1349, &Py::str("S"))?, py::getitem(&t1349, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1350 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1351 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1373 = (t1351).items()?;
        if t1373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1373[0].clone();
        v_v = t1373[1].clone();
        if ({ let t1352 = v_k.clone(); Py::Bool({ let t1353 = v_outputs.clone(); if !py::contains(&t1353, &t1352)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1404(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1406: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1405 = v_inputs.clone(); f_leq(py::getitem(&t1405, &Py::str("L"))?, py::getitem(&t1405, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1406 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1407 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1429 = (t1407).items()?;
        if t1429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1429[0].clone();
        v_v = t1429[1].clone();
        if ({ let t1408 = v_k.clone(); Py::Bool({ let t1409 = v_outputs.clone(); if !py::contains(&t1409, &t1408)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1460(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1461 = v_inputs.clone(); f_esp_area(py::getitem(&t1461, &Py::str("w"))?, py::getitem(&t1461, &Py::str("Q"))?, py::getitem(&t1461, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1462 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1463 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1485 = (t1463).items()?;
        if t1485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1485[0].clone();
        v_v = t1485[1].clone();
        if ({ let t1464 = v_k.clone(); Py::Bool({ let t1465 = v_outputs.clone(); if !py::contains(&t1465, &t1464)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1504(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1506: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1505 = v_inputs.clone(); f_esp_efficiency(py::getitem(&t1505, &Py::str("w"))?, py::getitem(&t1505, &Py::str("A"))?, py::getitem(&t1505, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1506 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1507 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1529 = (t1507).items()?;
        if t1529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1529[0].clone();
        v_v = t1529[1].clone();
        if ({ let t1508 = v_k.clone(); Py::Bool({ let t1509 = v_outputs.clone(); if !py::contains(&t1509, &t1508)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t153: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t152 = v_inputs.clone(); f_oxygen_demand(py::getitem(&t152, &Py::str("Q"))?, py::getitem(&t152, &Py::str("S0"))?, py::getitem(&t152, &Py::str("S"))?, py::getitem(&t152, &Py::str("f"))?, py::getitem(&t152, &Py::str("Px"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t153 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t154 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t176 = (t154).items()?;
        if t176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t176[0].clone();
        v_v = t176[1].clone();
        if ({ let t155 = v_k.clone(); Py::Bool({ let t156 = v_outputs.clone(); if !py::contains(&t156, &t155)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1558(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1560: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1559 = v_inputs.clone(); f_field_oxygen_transfer(py::getitem(&t1559, &Py::str("SOTR"))?, py::getitem(&t1559, &Py::str("alpha"))?, py::getitem(&t1559, &Py::str("beta"))?, py::getitem(&t1559, &Py::str("theta"))?, py::getitem(&t1559, &Py::str("CsT"))?, py::getitem(&t1559, &Py::str("Cs20"))?, py::getitem(&t1559, &Py::str("CL"))?, py::getitem(&t1559, &Py::str("T"))?, py::method(&t1559, "get", &[Py::str("Fp"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1560 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1561 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1583 = (t1561).items()?;
        if t1583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1583[0].clone();
        v_v = t1583[1].clone();
        if ({ let t1562 = v_k.clone(); Py::Bool({ let t1563 = v_outputs.clone(); if !py::contains(&t1563, &t1562)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1614(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1616: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1615 = v_inputs.clone(); f_ergun_headloss(py::getitem(&t1615, &Py::str("d"))?, py::getitem(&t1615, &Py::str("eps"))?, py::getitem(&t1615, &Py::str("psi"))?, py::getitem(&t1615, &Py::str("L"))?, py::getitem(&t1615, &Py::str("v"))?, py::getitem(&t1615, &Py::str("rho"))?, py::getitem(&t1615, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1616 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1617 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1639 = (t1617).items()?;
        if t1639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1639[0].clone();
        v_v = t1639[1].clone();
        if ({ let t1618 = v_k.clone(); Py::Bool({ let t1619 = v_outputs.clone(); if !py::contains(&t1619, &t1618)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1661(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1663: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1662 = v_inputs.clone(); f_filter_rate(py::getitem(&t1662, &Py::str("Q"))?, py::getitem(&t1662, &Py::str("A"))?, py::getitem(&t1662, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1663 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1664 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1686 = (t1664).items()?;
        if t1686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1686[0].clone();
        v_v = t1686[1].clone();
        if ({ let t1665 = v_k.clone(); Py::Bool({ let t1666 = v_outputs.clone(); if !py::contains(&t1666, &t1665)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1705(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1707: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1706 = v_inputs.clone(); f_fm_ratio(py::getitem(&t1706, &Py::str("Q"))?, py::getitem(&t1706, &Py::str("S0"))?, py::getitem(&t1706, &Py::str("V"))?, py::getitem(&t1706, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1707 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1708 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1730 = (t1708).items()?;
        if t1730.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1730[0].clone();
        v_v = t1730[1].clone();
        if ({ let t1709 = v_k.clone(); Py::Bool({ let t1710 = v_outputs.clone(); if !py::contains(&t1710, &t1709)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1753: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1752 = v_inputs.clone(); f_freundlich_dose(py::getitem(&t1752, &Py::str("C0"))?, py::getitem(&t1752, &Py::str("Ce"))?, py::getitem(&t1752, &Py::str("K"))?, py::getitem(&t1752, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1753 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1754 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1776 = (t1754).items()?;
        if t1776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1776[0].clone();
        v_v = t1776[1].clone();
        if ({ let t1755 = v_k.clone(); Py::Bool({ let t1756 = v_outputs.clone(); if !py::contains(&t1756, &t1755)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1812(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1814: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1813 = v_inputs.clone(); f_freundlich_fit(py::getitem(&t1813, &Py::str("Ce"))?, py::getitem(&t1813, &Py::str("qe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1814 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1815 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1837 = (t1815).items()?;
        if t1837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1837[0].clone();
        v_v = t1837[1].clone();
        if ({ let t1816 = v_k.clone(); Py::Bool({ let t1817 = v_outputs.clone(); if !py::contains(&t1817, &t1816)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1865(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1866 = v_inputs.clone(); f_plume_concentration(py::getitem(&t1866, &Py::str("Q"))?, py::getitem(&t1866, &Py::str("u"))?, py::getitem(&t1866, &Py::str("sigma_y"))?, py::getitem(&t1866, &Py::str("sigma_z"))?, py::getitem(&t1866, &Py::str("H"))?, py::getitem(&t1866, &Py::str("y"))?, py::getitem(&t1866, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1867 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1868 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1890 = (t1868).items()?;
        if t1890.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1890[0].clone();
        v_v = t1890[1].clone();
        if ({ let t1869 = v_k.clone(); Py::Bool({ let t1870 = v_outputs.clone(); if !py::contains(&t1870, &t1869)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1915(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1917: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1916 = v_inputs.clone(); f_ground_centerline(py::getitem(&t1916, &Py::str("Q"))?, py::getitem(&t1916, &Py::str("u"))?, py::getitem(&t1916, &Py::str("sigma_y"))?, py::getitem(&t1916, &Py::str("sigma_z"))?, py::getitem(&t1916, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1917 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1918 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1940 = (t1918).items()?;
        if t1940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1940[0].clone();
        v_v = t1940[1].clone();
        if ({ let t1919 = v_k.clone(); Py::Bool({ let t1920 = v_outputs.clone(); if !py::contains(&t1920, &t1919)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1971(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1973: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1972 = v_inputs.clone(); f_henry_convert(py::getitem(&t1972, &Py::str("H"))?, py::getitem(&t1972, &Py::str("form"))?, py::getitem(&t1972, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1973 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1974 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1996 = (t1974).items()?;
        if t1996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1996[0].clone();
        v_v = t1996[1].clone();
        if ({ let t1975 = v_k.clone(); Py::Bool({ let t1976 = v_outputs.clone(); if !py::contains(&t1976, &t1975)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_201(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t202 = v_inputs.clone(); f_air_stripping(py::getitem(&t202, &Py::str("Qw"))?, py::getitem(&t202, &Py::str("Qa"))?, py::getitem(&t202, &Py::str("H"))?, py::getitem(&t202, &Py::str("Cin"))?, py::getitem(&t202, &Py::str("Cout"))?, py::getitem(&t202, &Py::str("HTU"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t203 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t204 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t226 = (t204).items()?;
        if t226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t226[0].clone();
        v_v = t226[1].clone();
        if ({ let t205 = v_k.clone(); Py::Bool({ let t206 = v_outputs.clone(); if !py::contains(&t206, &t205)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2018(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2020: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2019 = v_inputs.clone(); f_henry_equilibrium(py::getitem(&t2019, &Py::str("p"))?, py::getitem(&t2019, &Py::str("H"))?, py::method(&t2019, "get", &[Py::str("M"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2020 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2021 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2043 = (t2021).items()?;
        if t2043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2043[0].clone();
        v_v = t2043[1].clone();
        if ({ let t2022 = v_k.clone(); Py::Bool({ let t2023 = v_outputs.clone(); if !py::contains(&t2023, &t2022)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2067: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2066 = v_inputs.clone(); f_henry_vant_hoff(py::getitem(&t2066, &Py::str("Hcp"))?, py::getitem(&t2066, &Py::str("B"))?, py::getitem(&t2066, &Py::str("T"))?, py::getitem(&t2066, &Py::str("Tref"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2067 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2068 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2090 = (t2068).items()?;
        if t2090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2090[0].clone();
        v_v = t2090[1].clone();
        if ({ let t2069 = v_k.clone(); Py::Bool({ let t2070 = v_outputs.clone(); if !py::contains(&t2070, &t2069)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2117(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2119: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2118 = v_inputs.clone(); f_holland_rise(py::getitem(&t2118, &Py::str("vs"))?, py::getitem(&t2118, &Py::str("d"))?, py::getitem(&t2118, &Py::str("u"))?, py::getitem(&t2118, &Py::str("p"))?, py::getitem(&t2118, &Py::str("Ts"))?, py::getitem(&t2118, &Py::str("Ta"))?, py::getitem(&t2118, &Py::str("hs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2119 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2120 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2142 = (t2120).items()?;
        if t2142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2142[0].clone();
        v_v = t2142[1].clone();
        if ({ let t2121 = v_k.clone(); Py::Bool({ let t2122 = v_outputs.clone(); if !py::contains(&t2122, &t2121)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2161(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2163: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2162 = v_inputs.clone(); f_hrt_olr(py::getitem(&t2162, &Py::str("Q"))?, py::getitem(&t2162, &Py::str("S0"))?, py::getitem(&t2162, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2163 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2164 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2186 = (t2164).items()?;
        if t2186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2186[0].clone();
        v_v = t2186[1].clone();
        if ({ let t2165 = v_k.clone(); Py::Bool({ let t2166 = v_outputs.clone(); if !py::contains(&t2166, &t2165)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2216(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2218: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2217 = v_inputs.clone(); f_hocl_fraction(py::getitem(&t2217, &Py::str("pH"))?, py::method(&t2217, "get", &[Py::str("pKa"), Py::None])?, py::method(&t2217, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2218 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2219 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2241 = (t2219).items()?;
        if t2241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2241[0].clone();
        v_v = t2241[1].clone();
        if ({ let t2220 = v_k.clone(); Py::Bool({ let t2221 = v_outputs.clone(); if !py::contains(&t2221, &t2220)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2261 = v_inputs.clone(); f_removal_fraction(py::getitem(&t2261, &Py::str("vs"))?, py::getitem(&t2261, &Py::str("Q"))?, py::getitem(&t2261, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2263 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2285 = (t2263).items()?;
        if t2285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2285[0].clone();
        v_v = t2285[1].clone();
        if ({ let t2264 = v_k.clone(); Py::Bool({ let t2265 = v_outputs.clone(); if !py::contains(&t2265, &t2264)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2329(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2331: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2330 = v_inputs.clone(); f_kla_fit(py::getitem(&t2330, &Py::str("t"))?, py::getitem(&t2330, &Py::str("C"))?, py::getitem(&t2330, &Py::str("Cs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2331 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2332 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2354 = (t2332).items()?;
        if t2354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2354[0].clone();
        v_v = t2354[1].clone();
        if ({ let t2333 = v_k.clone(); Py::Bool({ let t2334 = v_outputs.clone(); if !py::contains(&t2334, &t2333)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2384(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2386: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2385 = v_inputs.clone(); f_kla_two_point(py::getitem(&t2385, &Py::str("Cs"))?, py::getitem(&t2385, &Py::str("C1"))?, py::getitem(&t2385, &Py::str("C2"))?, py::getitem(&t2385, &Py::str("t1"))?, py::getitem(&t2385, &Py::str("t2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2386 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2387 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2409 = (t2387).items()?;
        if t2409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2409[0].clone();
        v_v = t2409[1].clone();
        if ({ let t2388 = v_k.clone(); Py::Bool({ let t2389 = v_outputs.clone(); if !py::contains(&t2389, &t2388)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2434(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2436: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2435 = v_inputs.clone(); f_lake_steady(py::getitem(&t2435, &Py::str("W"))?, py::getitem(&t2435, &Py::str("Q"))?, py::getitem(&t2435, &Py::str("k"))?, py::getitem(&t2435, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2436 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2437 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2459 = (t2437).items()?;
        if t2459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2459[0].clone();
        v_v = t2459[1].clone();
        if ({ let t2438 = v_k.clone(); Py::Bool({ let t2439 = v_outputs.clone(); if !py::contains(&t2439, &t2438)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2484(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2486: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2485 = v_inputs.clone(); f_lake_transient(py::getitem(&t2485, &Py::str("W"))?, py::getitem(&t2485, &Py::str("Q"))?, py::getitem(&t2485, &Py::str("k"))?, py::getitem(&t2485, &Py::str("V"))?, py::getitem(&t2485, &Py::str("C0"))?, py::getitem(&t2485, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2486 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2487 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2509 = (t2487).items()?;
        if t2509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2509[0].clone();
        v_v = t2509[1].clone();
        if ({ let t2488 = v_k.clone(); Py::Bool({ let t2489 = v_outputs.clone(); if !py::contains(&t2489, &t2488)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t254 = v_inputs.clone(); f_methane_yield(py::getitem(&t254, &Py::str("COD"))?, py::getitem(&t254, &Py::str("Px"))?, py::getitem(&t254, &Py::str("T"))?, py::getitem(&t254, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t278 = (t256).items()?;
        if t278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t278[0].clone();
        v_v = t278[1].clone();
        if ({ let t257 = v_k.clone(); Py::Bool({ let t258 = v_outputs.clone(); if !py::contains(&t258, &t257)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2536(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2538: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2537 = v_inputs.clone(); f_landfill_gas(py::getitem(&t2537, &Py::str("M"))?, py::getitem(&t2537, &Py::str("L0"))?, py::getitem(&t2537, &Py::str("k"))?, py::getitem(&t2537, &Py::str("tc"))?, py::getitem(&t2537, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2538 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2539 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2561 = (t2539).items()?;
        if t2561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2561[0].clone();
        v_v = t2561[1].clone();
        if ({ let t2540 = v_k.clone(); Py::Bool({ let t2541 = v_outputs.clone(); if !py::contains(&t2541, &t2540)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2588(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2590: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2589 = v_inputs.clone(); f_landfill_life(py::getitem(&t2589, &Py::str("capacity"))?, py::getitem(&t2589, &Py::str("M"))?, py::getitem(&t2589, &Py::str("rho"))?, py::getitem(&t2589, &Py::str("cover"))?, py::getitem(&t2589, &Py::str("g"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2590 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2591 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2613 = (t2591).items()?;
        if t2613.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2613[0].clone();
        v_v = t2613[1].clone();
        if ({ let t2592 = v_k.clone(); Py::Bool({ let t2593 = v_outputs.clone(); if !py::contains(&t2593, &t2592)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2634(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2636: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2635 = v_inputs.clone(); f_langmuir_dose(py::getitem(&t2635, &Py::str("C0"))?, py::getitem(&t2635, &Py::str("Ce"))?, py::getitem(&t2635, &Py::str("qmax"))?, py::getitem(&t2635, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2636 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2637 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2659 = (t2637).items()?;
        if t2659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2659[0].clone();
        v_v = t2659[1].clone();
        if ({ let t2638 = v_k.clone(); Py::Bool({ let t2639 = v_outputs.clone(); if !py::contains(&t2639, &t2638)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2699 = v_inputs.clone(); f_langmuir_fit(py::getitem(&t2699, &Py::str("Ce"))?, py::getitem(&t2699, &Py::str("qe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2723 = (t2701).items()?;
        if t2723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2723[0].clone();
        v_v = t2723[1].clone();
        if ({ let t2702 = v_k.clone(); Py::Bool({ let t2703 = v_outputs.clone(); if !py::contains(&t2703, &t2702)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2744(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2746: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2745 = v_inputs.clone(); f_log_removal(py::getitem(&t2745, &Py::str("N0"))?, py::getitem(&t2745, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2746 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2747 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2769 = (t2747).items()?;
        if t2769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2769[0].clone();
        v_v = t2769[1].clone();
        if ({ let t2748 = v_k.clone(); Py::Bool({ let t2749 = v_outputs.clone(); if !py::contains(&t2749, &t2748)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2790: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2789 = v_inputs.clone(); f_mass_to_ppm(py::getitem(&t2789, &Py::str("C"))?, py::getitem(&t2789, &Py::str("M"))?, py::getitem(&t2789, &Py::str("T"))?, py::getitem(&t2789, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2790 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2791 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2813 = (t2791).items()?;
        if t2813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2813[0].clone();
        v_v = t2813[1].clone();
        if ({ let t2792 = v_k.clone(); Py::Bool({ let t2793 = v_outputs.clone(); if !py::contains(&t2793, &t2792)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2876(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2878: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2877 = v_inputs.clone(); f_max_ground_briggs(py::getitem(&t2877, &Py::str("Q"))?, py::getitem(&t2877, &Py::str("u"))?, py::getitem(&t2877, &Py::str("H"))?, py::getitem(&t2877, &Py::str("cls"))?, py::getitem(&t2877, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2878 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2879 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2901 = (t2879).items()?;
        if t2901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2901[0].clone();
        v_v = t2901[1].clone();
        if ({ let t2880 = v_k.clone(); Py::Bool({ let t2881 = v_outputs.clone(); if !py::contains(&t2881, &t2880)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2920(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2922: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2921 = v_inputs.clone(); f_max_ground_turner(py::getitem(&t2921, &Py::str("Q"))?, py::getitem(&t2921, &Py::str("u"))?, py::getitem(&t2921, &Py::str("H"))?, py::getitem(&t2921, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2922 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2923 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2945 = (t2923).items()?;
        if t2945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2945[0].clone();
        v_v = t2945[1].clone();
        if ({ let t2924 = v_k.clone(); Py::Bool({ let t2925 = v_outputs.clone(); if !py::contains(&t2925, &t2924)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2976: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2975 = v_inputs.clone(); f_membrane_balance(py::getitem(&t2975, &Py::str("Qf"))?, py::getitem(&t2975, &Py::str("Qp"))?, py::getitem(&t2975, &Py::str("Cf"))?, py::getitem(&t2975, &Py::str("Cp"))?, py::getitem(&t2975, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2976 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2977 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2999 = (t2977).items()?;
        if t2999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2999[0].clone();
        v_v = t2999[1].clone();
        if ({ let t2978 = v_k.clone(); Py::Bool({ let t2979 = v_outputs.clone(); if !py::contains(&t2979, &t2978)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_298(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t300: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t299 = v_inputs.clone(); f_db_subtract(py::getitem(&t299, &Py::str("Lt"))?, py::getitem(&t299, &Py::str("Lb"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t300 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t301 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t323 = (t301).items()?;
        if t323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t323[0].clone();
        v_v = t323[1].clone();
        if ({ let t302 = v_k.clone(); Py::Bool({ let t303 = v_outputs.clone(); if !py::contains(&t303, &t302)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3026(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3028: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3027 = v_inputs.clone(); f_min_srt(py::getitem(&t3027, &Py::str("Y"))?, py::getitem(&t3027, &Py::str("k"))?, py::getitem(&t3027, &Py::str("Ks"))?, py::getitem(&t3027, &Py::str("kd"))?, py::getitem(&t3027, &Py::str("S0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3028 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3029 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3051 = (t3029).items()?;
        if t3051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3051[0].clone();
        v_v = t3051[1].clone();
        if ({ let t3030 = v_k.clone(); Py::Bool({ let t3031 = v_outputs.clone(); if !py::contains(&t3031, &t3030)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3070(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3072: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3071 = v_inputs.clone(); f_mixing_power(py::getitem(&t3071, &Py::str("G"))?, py::getitem(&t3071, &Py::str("mu"))?, py::getitem(&t3071, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3072 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3073 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3095 = (t3073).items()?;
        if t3095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3095[0].clone();
        v_v = t3095[1].clone();
        if ({ let t3074 = v_k.clone(); Py::Bool({ let t3075 = v_outputs.clone(); if !py::contains(&t3075, &t3074)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3125 = v_inputs.clone(); f_mix_many(py::getitem(&t3125, &Py::str("Q"))?, py::getitem(&t3125, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3126 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3127 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3149 = (t3127).items()?;
        if t3149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3149[0].clone();
        v_v = t3149[1].clone();
        if ({ let t3128 = v_k.clone(); Py::Bool({ let t3129 = v_outputs.clone(); if !py::contains(&t3129, &t3128)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3174(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3176: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3175 = v_inputs.clone(); f_mix_two(py::getitem(&t3175, &Py::str("Qw"))?, py::getitem(&t3175, &Py::str("Cw"))?, py::getitem(&t3175, &Py::str("Qr"))?, py::getitem(&t3175, &Py::str("Cr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3176 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3177 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3199 = (t3177).items()?;
        if t3199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3199[0].clone();
        v_v = t3199[1].clone();
        if ({ let t3178 = v_k.clone(); Py::Bool({ let t3179 = v_outputs.clone(); if !py::contains(&t3179, &t3178)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3227(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3229: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3228 = v_inputs.clone(); f_monod(py::getitem(&t3228, &Py::str("mu_max"))?, py::getitem(&t3228, &Py::str("Ks"))?, py::getitem(&t3228, &Py::str("S"))?, py::method(&t3228, "get", &[Py::str("kd"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3229 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3230 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3252 = (t3230).items()?;
        if t3252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3252[0].clone();
        v_v = t3252[1].clone();
        if ({ let t3231 = v_k.clone(); Py::Bool({ let t3232 = v_outputs.clone(); if !py::contains(&t3232, &t3231)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3280(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3282: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3281 = v_inputs.clone(); f_distance_attenuation(py::getitem(&t3281, &Py::str("L1"))?, py::getitem(&t3281, &Py::str("r1"))?, py::getitem(&t3281, &Py::str("r2"))?, py::getitem(&t3281, &Py::str("source"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3282 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3283 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3305 = (t3283).items()?;
        if t3305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3305[0].clone();
        v_v = t3305[1].clone();
        if ({ let t3284 = v_k.clone(); Py::Bool({ let t3285 = v_outputs.clone(); if !py::contains(&t3285, &t3284)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_33(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t35: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t34 = v_inputs.clone(); f_lawrence_mccarty(py::getitem(&t34, &Py::str("Y"))?, py::getitem(&t34, &Py::str("k"))?, py::getitem(&t34, &Py::str("Ks"))?, py::getitem(&t34, &Py::str("kd"))?, py::getitem(&t34, &Py::str("SRT"))?, py::getitem(&t34, &Py::str("HRT"))?, py::getitem(&t34, &Py::str("S0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t35 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t36 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t58 = (t36).items()?;
        if t58.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t58[0].clone();
        v_v = t58[1].clone();
        if ({ let t37 = v_k.clone(); Py::Bool({ let t38 = v_outputs.clone(); if !py::contains(&t38, &t37)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3330(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3332: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3331 = v_inputs.clone(); f_nrc_trickling(py::getitem(&t3331, &Py::str("W"))?, py::getitem(&t3331, &Py::str("V"))?, py::getitem(&t3331, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3332 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3333 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3355 = (t3333).items()?;
        if t3355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3355[0].clone();
        v_v = t3355[1].clone();
        if ({ let t3334 = v_k.clone(); Py::Bool({ let t3335 = v_outputs.clone(); if !py::contains(&t3335, &t3334)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3389 = v_inputs.clone(); f_noise_dose(py::getitem(&t3389, &Py::str("L"))?, py::getitem(&t3389, &Py::str("hours"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3413 = (t3391).items()?;
        if t3413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3413[0].clone();
        v_v = t3413[1].clone();
        if ({ let t3392 = v_k.clone(); Py::Bool({ let t3393 = v_outputs.clone(); if !py::contains(&t3393, &t3392)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3433 = v_inputs.clone(); f_osmotic_pressure(py::getitem(&t3433, &Py::str("C"))?, py::getitem(&t3433, &Py::str("M"))?, py::getitem(&t3433, &Py::str("i"))?, py::getitem(&t3433, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3457 = (t3435).items()?;
        if t3457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3457[0].clone();
        v_v = t3457[1].clone();
        if ({ let t3436 = v_k.clone(); Py::Bool({ let t3437 = v_outputs.clone(); if !py::contains(&t3437, &t3436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3476(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3478: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3477 = v_inputs.clone(); f_overflow_rate(py::getitem(&t3477, &Py::str("Q"))?, py::getitem(&t3477, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3478 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3479 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3501 = (t3479).items()?;
        if t3501.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3501[0].clone();
        v_v = t3501[1].clone();
        if ({ let t3480 = v_k.clone(); Py::Bool({ let t3481 = v_outputs.clone(); if !py::contains(&t3481, &t3480)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3532(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3533 = v_inputs.clone(); f_sag_distance(py::getitem(&t3533, &Py::str("kd"))?, py::getitem(&t3533, &Py::str("ka"))?, py::getitem(&t3533, &Py::str("L0"))?, py::getitem(&t3533, &Py::str("D0"))?, py::getitem(&t3533, &Py::str("x"))?, py::getitem(&t3533, &Py::str("v"))?, py::getitem(&t3533, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3535 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3557 = (t3535).items()?;
        if t3557.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3557[0].clone();
        v_v = t3557[1].clone();
        if ({ let t3536 = v_k.clone(); Py::Bool({ let t3537 = v_outputs.clone(); if !py::contains(&t3537, &t3536)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_357(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t358 = v_inputs.clone(); f_backwash_expansion(py::getitem(&t358, &Py::str("L"))?, py::getitem(&t358, &Py::str("eps"))?, py::getitem(&t358, &Py::str("vb"))?, py::getitem(&t358, &Py::str("vs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t359 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t360 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t382 = (t360).items()?;
        if t382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t382[0].clone();
        v_v = t382[1].clone();
        if ({ let t361 = v_k.clone(); Py::Bool({ let t362 = v_outputs.clone(); if !py::contains(&t362, &t361)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3598 = v_inputs.clone(); f_plume_with_class(py::getitem(&t3598, &Py::str("Q"))?, py::getitem(&t3598, &Py::str("u"))?, py::getitem(&t3598, &Py::str("H"))?, py::getitem(&t3598, &Py::str("x"))?, py::getitem(&t3598, &Py::str("y"))?, py::getitem(&t3598, &Py::str("z"))?, py::getitem(&t3598, &Py::str("cls"))?, py::getitem(&t3598, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3599 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3600 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3622 = (t3600).items()?;
        if t3622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3622[0].clone();
        v_v = t3622[1].clone();
        if ({ let t3601 = v_k.clone(); Py::Bool({ let t3602 = v_outputs.clone(); if !py::contains(&t3602, &t3601)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3659(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3661: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3660 = v_inputs.clone(); f_plume_lid(py::getitem(&t3660, &Py::str("Q"))?, py::getitem(&t3660, &Py::str("u"))?, py::getitem(&t3660, &Py::str("sigma_y"))?, py::getitem(&t3660, &Py::str("sigma_z"))?, py::getitem(&t3660, &Py::str("H"))?, py::getitem(&t3660, &Py::str("Lm"))?, py::getitem(&t3660, &Py::str("y"))?, py::getitem(&t3660, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3661 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3662 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3684 = (t3662).items()?;
        if t3684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3684[0].clone();
        v_v = t3684[1].clone();
        if ({ let t3663 = v_k.clone(); Py::Bool({ let t3664 = v_outputs.clone(); if !py::contains(&t3664, &t3663)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3709(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3711: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3710 = v_inputs.clone(); f_pollutant_load(py::getitem(&t3710, &Py::str("Q"))?, py::getitem(&t3710, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3711 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3712 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3734 = (t3712).items()?;
        if t3734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3734[0].clone();
        v_v = t3734[1].clone();
        if ({ let t3713 = v_k.clone(); Py::Bool({ let t3714 = v_outputs.clone(); if !py::contains(&t3714, &t3713)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3753(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3755: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3754 = v_inputs.clone(); f_ppm_to_mass(py::getitem(&t3754, &Py::str("ppm"))?, py::getitem(&t3754, &Py::str("M"))?, py::getitem(&t3754, &Py::str("T"))?, py::getitem(&t3754, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3755 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3756 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3778 = (t3756).items()?;
        if t3778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3778[0].clone();
        v_v = t3778[1].clone();
        if ({ let t3757 = v_k.clone(); Py::Bool({ let t3758 = v_outputs.clone(); if !py::contains(&t3758, &t3757)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3805: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3804 = v_inputs.clone(); f_k_temperature(py::getitem(&t3804, &Py::str("k20"))?, py::getitem(&t3804, &Py::str("theta"))?, py::getitem(&t3804, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3805 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3806 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3828 = (t3806).items()?;
        if t3828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3828[0].clone();
        v_v = t3828[1].clone();
        if ({ let t3807 = v_k.clone(); Py::Bool({ let t3808 = v_outputs.clone(); if !py::contains(&t3808, &t3807)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3850(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3852: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3851 = v_inputs.clone(); f_reaeration(py::getitem(&t3851, &Py::str("method"))?, py::getitem(&t3851, &Py::str("u"))?, py::getitem(&t3851, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3852 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3853 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3875 = (t3853).items()?;
        if t3875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3875[0].clone();
        v_v = t3875[1].clone();
        if ({ let t3854 = v_k.clone(); Py::Bool({ let t3855 = v_outputs.clone(); if !py::contains(&t3855, &t3854)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3896(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3898: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3897 = v_inputs.clone(); f_return_ratio(py::getitem(&t3897, &Py::str("X"))?, py::getitem(&t3897, &Py::str("Xr"))?, py::getitem(&t3897, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3898 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3899 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3921 = (t3899).items()?;
        if t3921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3921[0].clone();
        v_v = t3921[1].clone();
        if ({ let t3900 = v_k.clone(); Py::Bool({ let t3901 = v_outputs.clone(); if !py::contains(&t3901, &t3900)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3947 = v_inputs.clone(); f_river_decay(py::getitem(&t3947, &Py::str("C0"))?, py::getitem(&t3947, &Py::str("k"))?, py::getitem(&t3947, &Py::str("x"))?, py::getitem(&t3947, &Py::str("u"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3971 = (t3949).items()?;
        if t3971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3971[0].clone();
        v_v = t3971[1].clone();
        if ({ let t3950 = v_k.clone(); Py::Bool({ let t3951 = v_outputs.clone(); if !py::contains(&t3951, &t3950)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3990(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3992: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3991 = v_inputs.clone(); f_sed_basin(py::getitem(&t3991, &Py::str("Q"))?, py::getitem(&t3991, &Py::str("v0"))?, py::getitem(&t3991, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3992 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3993 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4015 = (t3993).items()?;
        if t4015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4015[0].clone();
        v_v = t4015[1].clone();
        if ({ let t3994 = v_k.clone(); Py::Bool({ let t3995 = v_outputs.clone(); if !py::contains(&t3995, &t3994)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4041(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4043: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4042 = v_inputs.clone(); f_settling_chamber(py::getitem(&t4042, &Py::str("dp"))?, py::getitem(&t4042, &Py::str("rho_p"))?, py::getitem(&t4042, &Py::str("rho_g"))?, py::getitem(&t4042, &Py::str("mu"))?, py::getitem(&t4042, &Py::str("L"))?, py::getitem(&t4042, &Py::str("W"))?, py::getitem(&t4042, &Py::str("Q"))?, py::getitem(&t4042, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4043 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4044 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4066 = (t4044).items()?;
        if t4066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4066[0].clone();
        v_v = t4066[1].clone();
        if ({ let t4045 = v_k.clone(); Py::Bool({ let t4046 = v_outputs.clone(); if !py::contains(&t4046, &t4045)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4093(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4095: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4094 = v_inputs.clone(); f_sludge_production(py::getitem(&t4094, &Py::str("Y"))?, py::getitem(&t4094, &Py::str("kd"))?, py::getitem(&t4094, &Py::str("SRT"))?, py::getitem(&t4094, &Py::str("Q"))?, py::getitem(&t4094, &Py::str("S0"))?, py::getitem(&t4094, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4095 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4096 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4118 = (t4096).items()?;
        if t4118.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4118[0].clone();
        v_v = t4118[1].clone();
        if ({ let t4097 = v_k.clone(); Py::Bool({ let t4098 = v_outputs.clone(); if !py::contains(&t4098, &t4097)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4153: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4152 = v_inputs.clone(); f_thickening(py::getitem(&t4152, &Py::str("V1"))?, py::getitem(&t4152, &Py::str("P1"))?, py::getitem(&t4152, &Py::str("P2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4153 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4154 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4176 = (t4154).items()?;
        if t4176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4176[0].clone();
        v_v = t4176[1].clone();
        if ({ let t4155 = v_k.clone(); Py::Bool({ let t4156 = v_outputs.clone(); if !py::contains(&t4156, &t4155)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4209: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4208 = v_inputs.clone(); f_sludge_volume(py::getitem(&t4208, &Py::str("Ms"))?, py::getitem(&t4208, &Py::str("Ps"))?, py::getitem(&t4208, &Py::str("Ssl"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4209 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4210 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4232 = (t4210).items()?;
        if t4232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4232[0].clone();
        v_v = t4232[1].clone();
        if ({ let t4211 = v_k.clone(); Py::Bool({ let t4212 = v_outputs.clone(); if !py::contains(&t4212, &t4211)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_421(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t423: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t422 = v_inputs.clone(); f_bod_dilution(py::getitem(&t422, &Py::str("D1"))?, py::getitem(&t422, &Py::str("D2"))?, py::getitem(&t422, &Py::str("P"))?, py::method(&t422, "get", &[Py::str("B1"), Py::None])?, py::method(&t422, "get", &[Py::str("B2"), Py::None])?, py::method(&t422, "get", &[Py::str("f"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t423 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t424 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t446 = (t424).items()?;
        if t446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t446[0].clone();
        v_v = t446[1].clone();
        if ({ let t425 = v_k.clone(); Py::Bool({ let t426 = v_outputs.clone(); if !py::contains(&t426, &t425)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4254 = v_inputs.clone(); f_svi(py::getitem(&t4254, &Py::str("V30"))?, py::getitem(&t4254, &Py::str("MLSS"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4278 = (t4256).items()?;
        if t4278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4278[0].clone();
        v_v = t4278[1].clone();
        if ({ let t4257 = v_k.clone(); Py::Bool({ let t4258 = v_outputs.clone(); if !py::contains(&t4258, &t4257)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4309(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4311: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4310 = v_inputs.clone(); f_so2_emission(py::getitem(&t4310, &Py::str("m_fuel"))?, py::getitem(&t4310, &Py::str("S"))?, py::getitem(&t4310, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4311 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4312 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4334 = (t4312).items()?;
        if t4334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4334[0].clone();
        v_v = t4334[1].clone();
        if ({ let t4313 = v_k.clone(); Py::Bool({ let t4314 = v_outputs.clone(); if !py::contains(&t4314, &t4313)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4358: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4357 = v_inputs.clone(); f_waste_generation(py::getitem(&t4357, &Py::str("pop"))?, py::getitem(&t4357, &Py::str("rate"))?, py::getitem(&t4357, &Py::str("rho"))?, py::getitem(&t4357, &Py::str("Vt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4358 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4359 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4381 = (t4359).items()?;
        if t4381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4381[0].clone();
        v_v = t4381[1].clone();
        if ({ let t4360 = v_k.clone(); Py::Bool({ let t4361 = v_outputs.clone(); if !py::contains(&t4361, &t4360)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4406(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4408: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4407 = v_inputs.clone(); f_srt(py::getitem(&t4407, &Py::str("V"))?, py::getitem(&t4407, &Py::str("X"))?, py::getitem(&t4407, &Py::str("Qw"))?, py::getitem(&t4407, &Py::str("Xw"))?, py::getitem(&t4407, &Py::str("Qe"))?, py::getitem(&t4407, &Py::str("Xe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4408 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4409 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4431 = (t4409).items()?;
        if t4431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4431[0].clone();
        v_v = t4431[1].clone();
        if ({ let t4410 = v_k.clone(); Py::Bool({ let t4411 = v_outputs.clone(); if !py::contains(&t4411, &t4410)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4455(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4457: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4456 = v_inputs.clone(); f_sound_power_to_pressure(py::getitem(&t4456, &Py::str("Lw"))?, py::getitem(&t4456, &Py::str("r"))?, py::getitem(&t4456, &Py::str("Qd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4457 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4458 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4480 = (t4458).items()?;
        if t4480.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4480[0].clone();
        v_v = t4480[1].clone();
        if ({ let t4459 = v_k.clone(); Py::Bool({ let t4460 = v_outputs.clone(); if !py::contains(&t4460, &t4459)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4513(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4515: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4514 = v_inputs.clone(); f_stability_class(py::getitem(&t4514, &Py::str("u"))?, py::getitem(&t4514, &Py::str("condition"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4515 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4516 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4538 = (t4516).items()?;
        if t4538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4538[0].clone();
        v_v = t4538[1].clone();
        if ({ let t4517 = v_k.clone(); Py::Bool({ let t4518 = v_outputs.clone(); if !py::contains(&t4518, &t4517)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4585: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4584 = v_inputs.clone(); f_plume_chain(py::getitem(&t4584, &Py::str("Q"))?, py::getitem(&t4584, &Py::str("u"))?, py::getitem(&t4584, &Py::str("vs"))?, py::getitem(&t4584, &Py::str("d"))?, py::getitem(&t4584, &Py::str("Ts"))?, py::getitem(&t4584, &Py::str("Ta"))?, py::getitem(&t4584, &Py::str("hs"))?, py::getitem(&t4584, &Py::str("x"))?, py::getitem(&t4584, &Py::str("cls"))?, py::getitem(&t4584, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4585 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4586 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4608 = (t4586).items()?;
        if t4608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4608[0].clone();
        v_v = t4608[1].clone();
        if ({ let t4587 = v_k.clone(); Py::Bool({ let t4588 = v_outputs.clone(); if !py::contains(&t4588, &t4587)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4631(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4633: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4632 = v_inputs.clone(); f_stokes(py::getitem(&t4632, &Py::str("d"))?, py::getitem(&t4632, &Py::str("rho_p"))?, py::getitem(&t4632, &Py::str("rho_w"))?, py::getitem(&t4632, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4633 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4634 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4656 = (t4634).items()?;
        if t4656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4656[0].clone();
        v_v = t4656[1].clone();
        if ({ let t4635 = v_k.clone(); Py::Bool({ let t4636 = v_outputs.clone(); if !py::contains(&t4636, &t4635)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4691(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4693: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4692 = v_inputs.clone(); f_sp_critical(py::getitem(&t4692, &Py::str("kd"))?, py::getitem(&t4692, &Py::str("ka"))?, py::getitem(&t4692, &Py::str("L0"))?, py::getitem(&t4692, &Py::str("D0"))?, py::getitem(&t4692, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4693 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4694 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4716 = (t4694).items()?;
        if t4716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4716[0].clone();
        v_v = t4716[1].clone();
        if ({ let t4695 = v_k.clone(); Py::Bool({ let t4696 = v_outputs.clone(); if !py::contains(&t4696, &t4695)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_471(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t473: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t472 = v_inputs.clone(); f_bod_exerted(py::getitem(&t472, &Py::str("L0"))?, py::getitem(&t472, &Py::str("k"))?, py::getitem(&t472, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t473 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t474 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t496 = (t474).items()?;
        if t496.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t496[0].clone();
        v_v = t496[1].clone();
        if ({ let t475 = v_k.clone(); Py::Bool({ let t476 = v_outputs.clone(); if !py::contains(&t476, &t475)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4747(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4749: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4748 = v_inputs.clone(); f_streeter_phelps(py::getitem(&t4748, &Py::str("kd"))?, py::getitem(&t4748, &Py::str("ka"))?, py::getitem(&t4748, &Py::str("L0"))?, py::getitem(&t4748, &Py::str("D0"))?, py::getitem(&t4748, &Py::str("t"))?, py::getitem(&t4748, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4749 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4750 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4772 = (t4750).items()?;
        if t4772.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4772[0].clone();
        v_v = t4772[1].clone();
        if ({ let t4751 = v_k.clone(); Py::Bool({ let t4752 = v_outputs.clone(); if !py::contains(&t4752, &t4751)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4821(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4823: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4822 = v_inputs.clone(); f_terminal_velocity(py::getitem(&t4822, &Py::str("d"))?, py::getitem(&t4822, &Py::str("rho_p"))?, py::getitem(&t4822, &Py::str("rho_f"))?, py::getitem(&t4822, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4823 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4824 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4846 = (t4824).items()?;
        if t4846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4846[0].clone();
        v_v = t4846[1].clone();
        if ({ let t4825 = v_k.clone(); Py::Bool({ let t4826 = v_outputs.clone(); if !py::contains(&t4826, &t4825)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4897: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4896 = v_inputs.clone(); f_thod(py::getitem(&t4896, &Py::str("formula"))?, py::getitem(&t4896, &Py::str("nitrification"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4897 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4898 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4920 = (t4898).items()?;
        if t4920.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4920[0].clone();
        v_v = t4920[1].clone();
        if ({ let t4899 = v_k.clone(); Py::Bool({ let t4900 = v_outputs.clone(); if !py::contains(&t4900, &t4899)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4945(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4947: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4946 = v_inputs.clone(); f_two_film(py::getitem(&t4946, &Py::str("kl"))?, py::getitem(&t4946, &Py::str("kg"))?, py::getitem(&t4946, &Py::str("H"))?, py::getitem(&t4946, &Py::str("C"))?, py::getitem(&t4946, &Py::str("Cg"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4947 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4948 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4970 = (t4948).items()?;
        if t4970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4970[0].clone();
        v_v = t4970[1].clone();
        if ({ let t4949 = v_k.clone(); Py::Bool({ let t4950 = v_outputs.clone(); if !py::contains(&t4950, &t4949)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4989(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4991: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4990 = v_inputs.clone(); f_ultimate_bod(py::getitem(&t4990, &Py::str("BOD_t"))?, py::getitem(&t4990, &Py::str("k"))?, py::getitem(&t4990, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4991 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4992 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5014 = (t4992).items()?;
        if t5014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5014[0].clone();
        v_v = t5014[1].clone();
        if ({ let t4993 = v_k.clone(); Py::Bool({ let t4994 = v_outputs.clone(); if !py::contains(&t4994, &t4993)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5035(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5037: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5036 = v_inputs.clone(); f_velocity_gradient(py::getitem(&t5036, &Py::str("P"))?, py::getitem(&t5036, &Py::str("mu"))?, py::getitem(&t5036, &Py::str("V"))?, py::method(&t5036, "get", &[Py::str("t"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5037 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5038 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5060 = (t5038).items()?;
        if t5060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5060[0].clone();
        v_v = t5060[1].clone();
        if ({ let t5039 = v_k.clone(); Py::Bool({ let t5040 = v_outputs.clone(); if !py::contains(&t5040, &t5039)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5091(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5093: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5092 = v_inputs.clone(); f_waste_moisture(py::getitem(&t5092, &Py::str("m"))?, py::getitem(&t5092, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5093 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5094 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5116 = (t5094).items()?;
        if t5116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5116[0].clone();
        v_v = t5116[1].clone();
        if ({ let t5095 = v_k.clone(); Py::Bool({ let t5096 = v_outputs.clone(); if !py::contains(&t5096, &t5095)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5141(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5143: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5142 = v_inputs.clone(); f_hardness(py::getitem(&t5142, &Py::str("Ca"))?, py::getitem(&t5142, &Py::str("Mg"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5143 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5144 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5166 = (t5144).items()?;
        if t5166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5166[0].clone();
        v_v = t5166[1].clone();
        if ({ let t5145 = v_k.clone(); Py::Bool({ let t5146 = v_outputs.clone(); if !py::contains(&t5146, &t5145)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5185(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5187: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5186 = v_inputs.clone(); f_weir_loading(py::getitem(&t5186, &Py::str("Q"))?, py::getitem(&t5186, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5187 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5188 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5210 = (t5188).items()?;
        if t5210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5210[0].clone();
        v_v = t5210[1].clone();
        if ({ let t5189 = v_k.clone(); Py::Bool({ let t5190 = v_outputs.clone(); if !py::contains(&t5190, &t5189)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5232(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5234: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5233 = v_inputs.clone(); f_wind_power_law(py::getitem(&t5233, &Py::str("u1"))?, py::getitem(&t5233, &Py::str("z1"))?, py::getitem(&t5233, &Py::str("z2"))?, py::getitem(&t5233, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5234 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5235 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5257 = (t5235).items()?;
        if t5257.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5257[0].clone();
        v_v = t5257[1].clone();
        if ({ let t5236 = v_k.clone(); Py::Bool({ let t5237 = v_outputs.clone(); if !py::contains(&t5237, &t5236)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_568(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t570: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t569 = v_inputs.clone(); f_bod_rate_fit(py::getitem(&t569, &Py::str("t"))?, py::getitem(&t569, &Py::str("y"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t570 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t571 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t593 = (t571).items()?;
        if t593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t593[0].clone();
        v_v = t593[1].clone();
        if ({ let t572 = v_k.clone(); Py::Bool({ let t573 = v_outputs.clone(); if !py::contains(&t573, &t572)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_618(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t620: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t619 = v_inputs.clone(); f_box_model(py::getitem(&t619, &Py::str("q"))?, py::getitem(&t619, &Py::str("L"))?, py::getitem(&t619, &Py::str("u"))?, py::getitem(&t619, &Py::str("Hm"))?, py::getitem(&t619, &Py::str("Cb"))?, py::getitem(&t619, &Py::str("C0"))?, py::getitem(&t619, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t620 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t621 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t643 = (t621).items()?;
        if t643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t643[0].clone();
        v_v = t643[1].clone();
        if ({ let t622 = v_k.clone(); Py::Bool({ let t623 = v_outputs.clone(); if !py::contains(&t623, &t622)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_674(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t676: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t675 = v_inputs.clone(); f_briggs_sigmas(py::getitem(&t675, &Py::str("x"))?, py::getitem(&t675, &Py::str("cls"))?, py::getitem(&t675, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t676 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t677 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t699 = (t677).items()?;
        if t699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t699[0].clone();
        v_v = t699[1].clone();
        if ({ let t678 = v_k.clone(); Py::Bool({ let t679 = v_outputs.clone(); if !py::contains(&t679, &t678)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_728(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t730: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t729 = v_inputs.clone(); f_briggs_rise(py::getitem(&t729, &Py::str("vs"))?, py::getitem(&t729, &Py::str("d"))?, py::getitem(&t729, &Py::str("Ts"))?, py::getitem(&t729, &Py::str("Ta"))?, py::getitem(&t729, &Py::str("u"))?, py::getitem(&t729, &Py::str("hs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t730 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t731 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t753 = (t731).items()?;
        if t753.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t753[0].clone();
        v_v = t753[1].clone();
        if ({ let t732 = v_k.clone(); Py::Bool({ let t733 = v_outputs.clone(); if !py::contains(&t733, &t732)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_780(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t782: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t781 = v_inputs.clone(); f_briggs_stable_rise(py::getitem(&t781, &Py::str("vs"))?, py::getitem(&t781, &Py::str("d"))?, py::getitem(&t781, &Py::str("Ts"))?, py::getitem(&t781, &Py::str("Ta"))?, py::getitem(&t781, &Py::str("u"))?, py::getitem(&t781, &Py::str("dtheta"))?, py::getitem(&t781, &Py::str("hs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t782 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t783 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t805 = (t783).items()?;
        if t805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t805[0].clone();
        v_v = t805[1].clone();
        if ({ let t784 = v_k.clone(); Py::Bool({ let t785 = v_outputs.clone(); if !py::contains(&t785, &t784)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_830(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t832: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t831 = v_inputs.clone(); f_chick_first_order(py::getitem(&t831, &Py::str("N0"))?, py::getitem(&t831, &Py::str("k"))?, py::getitem(&t831, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t832 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t833 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t855 = (t833).items()?;
        if t855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t855[0].clone();
        v_v = t855[1].clone();
        if ({ let t834 = v_k.clone(); Py::Bool({ let t835 = v_outputs.clone(); if !py::contains(&t835, &t834)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_85(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t87: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t86 = v_inputs.clone(); f_aeration_curve(py::getitem(&t86, &Py::str("Cs"))?, py::getitem(&t86, &Py::str("C0"))?, py::getitem(&t86, &Py::str("KLa"))?, py::getitem(&t86, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t87 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t88 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t110 = (t88).items()?;
        if t110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t110[0].clone();
        v_v = t110[1].clone();
        if ({ let t89 = v_k.clone(); Py::Bool({ let t90 = v_outputs.clone(); if !py::contains(&t90, &t89)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_883(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t885: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t884 = v_inputs.clone(); f_chick_watson(py::getitem(&t884, &Py::str("k"))?, py::getitem(&t884, &Py::str("n"))?, py::getitem(&t884, &Py::str("C"))?, py::getitem(&t884, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t885 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t886 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t908 = (t886).items()?;
        if t908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t908[0].clone();
        v_v = t908[1].clone();
        if ({ let t887 = v_k.clone(); Py::Bool({ let t888 = v_outputs.clone(); if !py::contains(&t888, &t887)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t949: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t948 = v_inputs.clone(); f_chlorine_dose(py::getitem(&t948, &Py::str("Q"))?, py::getitem(&t948, &Py::str("demand"))?, py::getitem(&t948, &Py::str("residual"))?, py::getitem(&t948, &Py::str("purity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t949 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t950 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t972 = (t950).items()?;
        if t972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t972[0].clone();
        v_v = t972[1].clone();
        if ({ let t951 = v_k.clone(); Py::Bool({ let t952 = v_outputs.clone(); if !py::contains(&t952, &t951)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_991(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t993: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t992 = v_inputs.clone(); f_ct_inactivation(py::getitem(&t992, &Py::str("C"))?, py::getitem(&t992, &Py::str("t10"))?, py::getitem(&t992, &Py::str("CT999"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t993 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t994 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1016 = (t994).items()?;
        if t1016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1016[0].clone();
        v_v = t1016[1].clone();
        if ({ let t995 = v_k.clone(); Py::Bool({ let t996 = v_outputs.clone(); if !py::contains(&t996, &t995)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
