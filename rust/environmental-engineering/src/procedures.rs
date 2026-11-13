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
    "enveng_channel_flow_velocity",
    "enveng_chemical_feed_rate",
    "enveng_chick_law",
    "enveng_chick_watson",
    "enveng_chlorine_dose_feed",
    "enveng_chronic_daily_intake_risk",
    "enveng_cmfr_series",
    "enveng_collection_efficiency_series",
    "enveng_combustion_oxygen",
    "enveng_contact_time_from_ct",
    "enveng_ct_log_inactivation",
    "enveng_cyclone_lapple",
    "enveng_darcy_seepage_velocity",
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
    "enveng_filter_flow_from_loading",
    "enveng_filtration_rate",
    "enveng_first_order_decay",
    "enveng_food_to_microorganism_ratio",
    "enveng_freundlich_carbon_dose",
    "enveng_freundlich_fit",
    "enveng_gaussian_plume_concentration",
    "enveng_geometric_mean_counts",
    "enveng_ground_centerline_concentration",
    "enveng_hazen_williams",
    "enveng_heating_value_dry_basis",
    "enveng_henry_constant_conversion",
    "enveng_henry_law_equilibrium",
    "enveng_henry_law_solubility",
    "enveng_henry_temperature_dependence",
    "enveng_holland_plume_rise",
    "enveng_hrt_organic_loading",
    "enveng_hypochlorous_acid_fraction",
    "enveng_ideal_basin_removal",
    "enveng_indoor_air_steady_state",
    "enveng_kla_regression",
    "enveng_kla_two_point",
    "enveng_lake_steady_state",
    "enveng_lake_transient",
    "enveng_landfill_compaction_ratio",
    "enveng_landfill_daily_volume",
    "enveng_landfill_gas_generation",
    "enveng_landfill_life",
    "enveng_langmuir_carbon_dose",
    "enveng_langmuir_fit",
    "enveng_lapse_rate_stability",
    "enveng_log_removal",
    "enveng_manning_flow",
    "enveng_mass_concentration_to_ppm",
    "enveng_max_ground_concentration_briggs",
    "enveng_max_ground_concentration_ratio",
    "enveng_mean_cell_residence_time",
    "enveng_membrane_flux",
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
    "enveng_ph_poh",
    "enveng_plume_concentration_stability",
    "enveng_plume_mixing_lid",
    "enveng_pollutant_load",
    "enveng_population_logistic",
    "enveng_population_projection",
    "enveng_ppm_to_mass_concentration",
    "enveng_pump_power",
    "enveng_rate_constant_temperature",
    "enveng_rational_method",
    "enveng_reaeration_coefficient",
    "enveng_retardation_factor",
    "enveng_return_period_risk",
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
    "enveng_thod_solution",
    "enveng_total_alkalinity",
    "enveng_transmembrane_pressure",
    "enveng_two_film_transfer",
    "enveng_ultimate_bod",
    "enveng_velocity_gradient",
    "enveng_volatile_solids_reduction",
    "enveng_waste_moisture_content",
    "enveng_water_demand",
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
        "enveng_channel_flow_velocity" => {
            let r = f_channel_velocity(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("v") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_832(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chemical_feed_rate" => {
            let r = f_chemical_feed(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dose") { Some(x) => py::from_val(x), None => { return None } }, match v.get("purity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_890(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chick_law" => {
            let r = f_chick_first_order(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chick_watson" => {
            let r = f_chick_watson(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_993(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chlorine_dose_feed" => {
            let r = f_chlorine_dose(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("residual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("purity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_chronic_daily_intake_risk" => {
            let r = f_chronic_intake(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("BW") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EF") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("ED") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("AT") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("RfD") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("SF") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1125(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_cmfr_series" => {
            let r = f_cmfr_series(match v.get("Cin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("N") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1189(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_collection_efficiency_series" => {
            let r = f_series_efficiency(match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1245(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_combustion_oxygen" => {
            let r = f_combustion_oxygen(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_contact_time_from_ct" => {
            let r = f_ct_contact_time(match v.get("CT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ct_log_inactivation" => {
            let r = f_ct_inactivation(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CT999") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_cyclone_lapple" => {
            let r = f_cyclone_lapple(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ne") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1453(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_darcy_seepage_velocity" => {
            let r = f_darcy(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("dh") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_e") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1526(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_day_night_level" => {
            let r = f_ldn(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("metric") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1589(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_decibel_addition" => {
            let r = f_db_add(match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1635(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_detention_time" => {
            let r = f_detention_time(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_disinfection_ct" => {
            let r = f_ct_value(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("BF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_do_saturation" => {
            let r = f_do_saturation(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1779(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_dulong_heating_value" => {
            let r = f_dulong(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1837(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_equivalent_sound_level" => {
            let r = f_leq(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_esp_area" => {
            let r = f_esp_area(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1949(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_esp_efficiency" => {
            let r = f_esp_efficiency(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1993(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_field_oxygen_transfer" => {
            let r = f_field_oxygen_transfer(match v.get("SOTR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CsT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs20") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Fp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2047(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_filter_clean_bed_headloss" => {
            let r = f_ergun_headloss(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_filter_flow_from_loading" => {
            let r = f_filter_flow(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2147(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_filtration_rate" => {
            let r = f_filter_rate(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2194(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_first_order_decay" => {
            let r = f_first_order_decay(match v.get("C0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("C") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("t_half") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2275(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_food_to_microorganism_ratio" => {
            let r = f_fm_ratio(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_freundlich_carbon_dose" => {
            let r = f_freundlich_dose(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2365(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_freundlich_fit" => {
            let r = f_freundlich_fit(match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_gaussian_plume_concentration" => {
            let r = f_plume_concentration(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2479(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_geometric_mean_counts" => {
            let r = f_geometric_mean(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2527(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ground_centerline_concentration" => {
            let r = f_ground_centerline(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2577(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_hazen_williams" => {
            let r = f_hazen_williams(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2640(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_heating_value_dry_basis" => {
            let r = f_hhv_dry_basis(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HHV_ar") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("HHV_dry") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("ash") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2709(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_constant_conversion" => {
            let r = f_henry_convert(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("form") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2765(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_law_equilibrium" => {
            let r = f_henry_equilibrium(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2812(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_law_solubility" => {
            let r = f_henry_solubility(match v.get("KH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2859(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_henry_temperature_dependence" => {
            let r = f_henry_vant_hoff(match v.get("Hcp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2906(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_holland_plume_rise" => {
            let r = f_holland_rise(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_hrt_organic_loading" => {
            let r = f_hrt_olr(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_hypochlorous_acid_fraction" => {
            let r = f_hocl_fraction(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ideal_basin_removal" => {
            let r = f_removal_fraction(match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3101(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_indoor_air_steady_state" => {
            let r = f_indoor_air(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Camb") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3157(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_kla_regression" => {
            let r = f_kla_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3226(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_kla_two_point" => {
            let r = f_kla_two_point(match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3281(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_lake_steady_state" => {
            let r = f_lake_steady(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_lake_transient" => {
            let r = f_lake_transient(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3381(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_compaction_ratio" => {
            let r = f_compaction_ratio(match v.get("rho_i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("rho_c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V_i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("V_c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3449(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_daily_volume" => {
            let r = f_landfill_volume(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cover") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3504(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_gas_generation" => {
            let r = f_landfill_gas(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3556(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_landfill_life" => {
            let r = f_landfill_life(match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cover") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3608(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_langmuir_carbon_dose" => {
            let r = f_langmuir_dose(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3654(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_langmuir_fit" => {
            let r = f_langmuir_fit(match v.get("Ce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3718(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_lapse_rate_stability" => {
            let r = f_lapse_stability(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3773(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_log_removal" => {
            let r = f_log_removal(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3819(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_manning_flow" => {
            let r = f_manning(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mass_concentration_to_ppm" => {
            let r = f_mass_to_ppm(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3925(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_max_ground_concentration_briggs" => {
            let r = f_max_ground_briggs(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_max_ground_concentration_ratio" => {
            let r = f_max_ground_turner(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mean_cell_residence_time" => {
            let r = f_mcrt_masses(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Me") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_membrane_flux" => {
            let r = f_membrane_flux(match v.get("Qp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("J") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_membrane_mass_balance" => {
            let r = f_membrane_balance(match v.get("Qf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_minimum_srt" => {
            let r = f_min_srt(match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_power" => {
            let r = f_mixing_power(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_several_streams" => {
            let r = f_mix_many(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4372(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_mixing_two_streams" => {
            let r = f_mix_two(match v.get("Qw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4422(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_monod_growth_rate" => {
            let r = f_monod(match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_noise_distance_attenuation" => {
            let r = f_distance_attenuation(match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4528(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_nrc_trickling_filter" => {
            let r = f_nrc_trickling(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4578(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_occupational_noise_dose" => {
            let r = f_noise_dose(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4636(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4680(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_overflow_rate_circular" => {
            let r = f_overflow_rate(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4724(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_oxygen_sag_distance" => {
            let r = f_sag_distance(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4780(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ph_poh" => {
            let r = f_ph_poh(match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("OH") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4837(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_plume_concentration_stability" => {
            let r = f_plume_with_class(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4902(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_plume_mixing_lid" => {
            let r = f_plume_lid(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma_z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_pollutant_load" => {
            let r = f_pollutant_load(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5014(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_population_logistic" => {
            let r = f_population_logistic(match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5075(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_population_projection" => {
            let r = f_population_projection(match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5131(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ppm_to_mass_concentration" => {
            let r = f_ppm_to_mass(match v.get("ppm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_pump_power" => {
            let r = f_pump_power(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("hours") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("price") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5248(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_rate_constant_temperature" => {
            let r = f_k_temperature(match v.get("k20") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5298(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_rational_method" => {
            let r = f_rational_method(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5354(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_reaeration_coefficient" => {
            let r = f_reaeration(match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5401(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_retardation_factor" => {
            let r = f_retardation(match v.get("rho_b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Koc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("foc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("v") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5503(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_return_period_risk" => {
            let r = f_return_period(match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("risk") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5575(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_return_sludge_ratio" => {
            let r = f_return_ratio(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_river_first_order_decay" => {
            let r = f_river_decay(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5671(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sedimentation_basin_sizing" => {
            let r = f_sed_basin(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5715(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_settling_chamber_efficiency" => {
            let r = f_settling_chamber(match v.get("dp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5766(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_production" => {
            let r = f_sludge_production(match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SRT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5818(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_thickening" => {
            let r = f_thickening(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5876(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_volume" => {
            let r = f_sludge_volume(match v.get("Ms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ssl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5932(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sludge_volume_index" => {
            let r = f_svi(match v.get("V30") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MLSS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5978(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_so2_emission" => {
            let r = f_so2_emission(match v.get("m_fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6034(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_solid_waste_generation" => {
            let r = f_waste_generation(match v.get("pop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_solids_retention_time" => {
            let r = f_srt(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Xw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qe") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Xe") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6137(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_sound_power_to_pressure" => {
            let r = f_sound_power_to_pressure(match v.get("Lw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stability_class" => {
            let r = f_stability_class(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("condition") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6244(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stack_plume_ground_concentration" => {
            let r = f_plume_chain(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("terrain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_stokes_settling_velocity" => {
            let r = f_stokes(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6362(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_streeter_phelps_critical_point" => {
            let r = f_sp_critical(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6422(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_streeter_phelps_deficit" => {
            let r = f_streeter_phelps(match v.get("kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DOsat") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6478(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_terminal_settling_velocity" => {
            let r = f_terminal_velocity(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_theoretical_oxygen_demand" => {
            let r = f_thod(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nitrification") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_thod_solution" => {
            let r = f_thod_solution(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nitrification") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6708(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_total_alkalinity" => {
            let r = f_alkalinity(match v.get("HCO3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("CO3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("OH") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("HCO3_M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("CO3_M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("OH_M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("H_M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("pH") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6799(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_transmembrane_pressure" => {
            let r = f_transmembrane_pressure(match v.get("P_feed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_conc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_perm") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6844(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_two_film_transfer" => {
            let r = f_two_film(match v.get("kl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6894(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_ultimate_bod" => {
            let r = f_ultimate_bod(match v.get("BOD_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6938(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_velocity_gradient" => {
            let r = f_velocity_gradient(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6984(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_volatile_solids_reduction" => {
            let r = f_vs_reduction(match v.get("VS_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VS_out") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7036(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_waste_moisture_content" => {
            let r = f_waste_moisture(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7092(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_water_demand" => {
            let r = f_water_demand(match v.get("pop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_md") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_ph") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_ww") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7164(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_water_hardness" => {
            let r = f_hardness(match v.get("Ca") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7214(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_weir_loading_rate" => {
            let r = f_weir_loading(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7258(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enveng_wind_power_law" => {
            let r = f_wind_power_law(match v.get("u1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c_EQ_CACO3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(50000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_GAMMA_DRY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.8)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HOUR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KW_25C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-14)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.45359237)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MGD_SI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.04381263638888889)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_CO3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(60.0089)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_HCO3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(61.016839999999995)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_O2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31.9988)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_OH() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(17.00734)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P_ATM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(101325.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_WATER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1000.0)) })());
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
    let t6512 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t6512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t6512[0].clone();
    v_fb = t6512[1].clone();
    if ({ let t6513 = v_fa.clone(); Py::Bool({ let t6514 = Py::Int(0i64); if py::eq(&t6513, &t6514) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t6515 = v_fb.clone(); Py::Bool({ let t6516 = Py::Int(0i64); if py::eq(&t6515, &t6516) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t6517 = { let t6519 = v_fa.clone(); Py::Bool({ let t6520 = Py::Int(0i64); if py::gt(&t6519, &t6520)? { true } else { false } }) }; Py::Bool({ let t6518 = { let t6521 = v_fb.clone(); Py::Bool({ let t6522 = Py::Int(0i64); if py::gt(&t6521, &t6522)? { true } else { false } }) }; if py::ne(&t6517, &t6518) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t6523 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t6523;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t6528 = { let t6524 = v_m.clone(); Py::Bool({ let t6525 = v_a.clone(); if py::eq(&t6524, &t6525) { true } else { false } }) }; if t6528.truthy() { t6528 } else { { let t6526 = v_m.clone(); Py::Bool({ let t6527 = v_b.clone(); if py::eq(&t6526, &t6527) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t6529 = v_fm.clone(); Py::Bool({ let t6530 = Py::Int(0i64); if py::eq(&t6529, &t6530) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t6531 = { let t6533 = v_fm.clone(); Py::Bool({ let t6534 = Py::Int(0i64); if py::gt(&t6533, &t6534)? { true } else { false } }) }; Py::Bool({ let t6532 = { let t6535 = v_fa.clone(); Py::Bool({ let t6536 = Py::Int(0i64); if py::gt(&t6535, &t6536)? { true } else { false } }) }; if py::eq(&t6531, &t6532) { true } else { false } }) }).truthy() {
            let t6537 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t6537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t6537[0].clone();
            v_fa = t6537[1].clone();
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
    let _ = f__need({ let t6302 = Py::Float(100.0); Py::Bool({ let t6303 = v_x.clone(); if py::le(&t6302, &t6303)? { { let t6304 = Py::Float(10000.0); if py::le(&t6303, &t6304)? { true } else { false } } } else { false } }) }, Py::str("the Briggs coefficients hold for x between 100 m and 10 km"))?;
    let t6311 = (f__briggs_coef(v_cls.clone(), v_terrain.clone())?).items()?;
    if t6311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cy = t6311[0].clone();
    v_cz = t6311[1].clone();
    return Ok(Py::tuple(vec![f__sig_eval(v_cy.clone(), v_x.clone())?, f__sig_eval(v_cz.clone(), v_x.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__briggs_coef(mut v_cls: Py, mut v_terrain: Py) -> R {
    let _ = f__need({ let t6305 = v_cls.clone(); Py::Bool({ let t6306 = c__CLASSES()?; if py::contains(&t6306, &t6305)? { true } else { false } }) }, Py::str("the stability class must be one of A to F"))?;
    let _ = f__need({ let t6307 = v_terrain.clone(); Py::Bool({ let t6308 = Py::tuple(vec![Py::str("rural"), Py::str("urban")]); if py::contains(&t6308, &t6307)? { true } else { false } }) }, Py::str("terrain must be rural or urban"))?;
    return Ok(py::getitem(&(if ({ let t6309 = v_terrain.clone(); Py::Bool({ let t6310 = Py::str("rural"); if py::eq(&t6309, &t6310) { true } else { false } }) }).truthy() { c__BRIGGS_RURAL()? } else { c__BRIGGS_URBAN()? }), &v_cls.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__buoyancy_flux(mut v_vs: Py, mut v_d: Py, mut v_Ts: Py, mut v_Ta: Py) -> R {
    let _ = f__pos(Py::str("vs, d, Ts and Ta"), Py::tuple(vec![v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone()]))?;
    let _ = f__need({ let t6278 = v_Ts.clone(); Py::Bool({ let t6279 = v_Ta.clone(); if py::gt(&t6278, &t6279)? { true } else { false } }) }, Py::str("the stack gas must be warmer than the air (Ts > Ta) for a buoyant plume"))?;
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&c_G0()?, &v_vs.clone())?, &v_d.clone())?, &v_d.clone())?, &py::sub(&v_Ts.clone(), &v_Ta.clone())?)?, &py::mul(&Py::Float(4.0), &v_Ts.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cho_counts(mut v_text: Py) -> R {
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
    let _ = f__need({ let t1271 = py::len(&v_s.clone())?; Py::Bool({ let t1272 = Py::Int(0i64); if py::gt(&t1271, &t1272)? { true } else { false } }) }, Py::str("give a fuel formula such as CH4, C2H5OH or C"))?;
    let t1273 = (Py::tuple(vec![Py::str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Py::str("abcdefghijklmnopqrstuvwxyz"), Py::str("0123456789")])).items()?;
    if t1273.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_upper = t1273[0].clone();
    v_lower = t1273[1].clone();
    v_digits = t1273[2].clone();
    let t1274 = (Py::tuple(vec![Py::Int(0i64), Py::dict(vec![])])).items()?;
    if t1274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t1274[0].clone();
    v_counts = t1274[1].clone();
    while ({ let t1291 = v_pos.clone(); Py::Bool({ let t1292 = py::len(&v_s.clone())?; if py::lt(&t1291, &t1292)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t1275 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t1276 = v_upper.clone(); if py::contains(&t1276, &t1275)? { true } else { false } }) }, Py::str("the formula may contain only element symbols and counts (C, H, O)"))?;
        v_el = py::getitem(&v_s.clone(), &v_pos.clone())?;
        v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        if ({ let t1281 = { let t1277 = v_pos.clone(); Py::Bool({ let t1278 = py::len(&v_s.clone())?; if py::lt(&t1277, &t1278)? { true } else { false } }) }; if !t1281.truthy() { t1281 } else { { let t1279 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t1280 = v_lower.clone(); if py::contains(&t1280, &t1279)? { true } else { false } }) } } }).truthy() {
            v_el = py::add(&v_el.clone(), &py::getitem(&v_s.clone(), &v_pos.clone())?)?;
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_start = v_pos.clone();
        while ({ let t1286 = { let t1282 = v_pos.clone(); Py::Bool({ let t1283 = py::len(&v_s.clone())?; if py::lt(&t1282, &t1283)? { true } else { false } }) }; if !t1286.truthy() { t1286 } else { { let t1284 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t1285 = v_digits.clone(); if py::contains(&t1285, &t1284)? { true } else { false } }) } } }).truthy() {
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_n = (if ({ let t1287 = v_pos.clone(); Py::Bool({ let t1288 = v_start.clone(); if py::gt(&t1287, &t1288)? { true } else { false } }) }).truthy() { py::int(&py::slice(&v_s.clone(), Some((v_start.clone()).i()?), Some((v_pos.clone()).i()?), None)?)? } else { Py::Int(1i64) });
        let _ = f__need({ let t1289 = v_el.clone(); Py::Bool({ let t1290 = Py::tuple(vec![Py::str("C"), Py::str("H"), Py::str("O")]); if py::contains(&t1290, &t1289)? { true } else { false } }) }, Py::str("only C, H and O are allowed"))?;
        py::setitem(&v_counts.clone(), &v_el.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
    }
    let _ = f__need({ let t1293 = py::add(&py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, &py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?)?; Py::Bool({ let t1294 = Py::Int(0i64); if py::gt(&t1293, &t1294)? { true } else { false } }) }, Py::str("the fuel must contain carbon or hydrogen"))?;
    return Ok(Py::tuple(vec![py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("O"), Py::Int(0i64)])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__db_list(mut v_L: Py, mut v_name: Py) -> R {
    let mut v_v: Py = Py::None;
    for t6163 in py::iter(&(v_L.clone()))? {
        v_v = t6163;
        let _ = f__need({ let t6167 = py::math1("isfinite", &v_v.clone())?; if !t6167.truthy() { t6167 } else { { let t6164 = py::neg(&Py::Float(50.0))?; Py::Bool({ let t6165 = v_v.clone(); if py::le(&t6164, &t6165)? { { let t6166 = Py::Float(250.0); if py::le(&t6165, &t6166)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a sound level in dB between -50 and 250".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t7131 in py::iter(&(v_vals.clone()))? {
        v_v = t7131;
        let _ = f__need({ let t7133 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t7133.truthy() { t7133 } else { { let t7132 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t7132.truthy() { t7132 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
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
    let _ = f__need({ let t6660 = py::len(&v_s.clone())?; Py::Bool({ let t6661 = Py::Int(0i64); if py::gt(&t6660, &t6661)? { true } else { false } }) }, Py::str("give a molecular formula such as C6H12O6"))?;
    let t6662 = (Py::tuple(vec![Py::str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Py::str("abcdefghijklmnopqrstuvwxyz"), Py::str("0123456789")])).items()?;
    if t6662.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_upper = t6662[0].clone();
    v_lower = t6662[1].clone();
    v_digits = t6662[2].clone();
    let t6663 = (Py::tuple(vec![Py::Int(0i64), Py::dict(vec![])])).items()?;
    if t6663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t6663[0].clone();
    v_counts = t6663[1].clone();
    while ({ let t6680 = v_pos.clone(); Py::Bool({ let t6681 = py::len(&v_s.clone())?; if py::lt(&t6680, &t6681)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6664 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t6665 = v_upper.clone(); if py::contains(&t6665, &t6664)? { true } else { false } }) }, Py::str("the formula may contain only element symbols and counts (C, H, O, N)"))?;
        v_el = py::getitem(&v_s.clone(), &v_pos.clone())?;
        v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        if ({ let t6670 = { let t6666 = v_pos.clone(); Py::Bool({ let t6667 = py::len(&v_s.clone())?; if py::lt(&t6666, &t6667)? { true } else { false } }) }; if !t6670.truthy() { t6670 } else { { let t6668 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t6669 = v_lower.clone(); if py::contains(&t6669, &t6668)? { true } else { false } }) } } }).truthy() {
            v_el = py::add(&v_el.clone(), &py::getitem(&v_s.clone(), &v_pos.clone())?)?;
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_start = v_pos.clone();
        while ({ let t6675 = { let t6671 = v_pos.clone(); Py::Bool({ let t6672 = py::len(&v_s.clone())?; if py::lt(&t6671, &t6672)? { true } else { false } }) }; if !t6675.truthy() { t6675 } else { { let t6673 = py::getitem(&v_s.clone(), &v_pos.clone())?; Py::Bool({ let t6674 = v_digits.clone(); if py::contains(&t6674, &t6673)? { true } else { false } }) } } }).truthy() {
            v_pos = py::iadd(&v_pos, &Py::Int(1i64))?;
        }
        v_n = (if ({ let t6676 = v_pos.clone(); Py::Bool({ let t6677 = v_start.clone(); if py::gt(&t6676, &t6677)? { true } else { false } }) }).truthy() { py::int(&py::slice(&v_s.clone(), Some((v_start.clone()).i()?), Some((v_pos.clone()).i()?), None)?)? } else { Py::Int(1i64) });
        let _ = f__need({ let t6678 = v_el.clone(); Py::Bool({ let t6679 = Py::tuple(vec![Py::str("C"), Py::str("H"), Py::str("O"), Py::str("N")]); if py::contains(&t6679, &t6678)? { true } else { false } }) }, Py::str("only C, H, O and N are allowed"))?;
        py::setitem(&v_counts.clone(), &v_el.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
    }
    let _ = f__need({ let t6686 = { let t6682 = py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?; Py::Bool({ let t6683 = Py::Int(0i64); if py::gt(&t6682, &t6683)? { true } else { false } }) }; if !t6686.truthy() { t6686 } else { { let t6684 = py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?; Py::Bool({ let t6685 = Py::Int(0i64); if py::gt(&t6684, &t6685)? { true } else { false } }) } } }, Py::str("the compound must contain carbon and hydrogen"))?;
    return Ok(Py::tuple(vec![py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("O"), Py::Int(0i64)])?, py::method(&v_counts.clone(), "get", &[Py::str("N"), Py::Int(0i64)])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t7150 = (if (v_lo_open.clone()).truthy() { { let t7142 = v_v.clone(); Py::Bool({ let t7143 = Py::Int(0i64); if py::gt(&t7142, &t7143)? { true } else { false } }) } } else { { let t7144 = v_v.clone(); Py::Bool({ let t7145 = Py::Int(0i64); if py::ge(&t7144, &t7145)? { true } else { false } }) } }); if !t7150.truthy() { t7150 } else { (if (v_hi_open.clone()).truthy() { { let t7146 = v_v.clone(); Py::Bool({ let t7147 = Py::Int(1i64); if py::lt(&t7146, &t7147)? { true } else { false } }) } } else { { let t7148 = v_v.clone(); Py::Bool({ let t7149 = Py::Int(1i64); if py::le(&t7148, &t7149)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iso_check(mut v_Ce: Py, mut v_qe: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t3684 = { let t3680 = py::len(&v_Ce.clone())?; Py::Bool({ let t3681 = py::len(&v_qe.clone())?; if py::eq(&t3680, &t3681) { true } else { false } }) }; if !t3684.truthy() { t3684 } else { { let t3682 = py::len(&v_Ce.clone())?; Py::Bool({ let t3683 = Py::Int(3i64); if py::ge(&t3682, &t3683)? { true } else { false } }) } } }, Py::str("give at least three points (Ce and qe of equal length)"))?;
    for t3685 in py::iter(&(py::add(&Py::list((v_Ce.clone()).items()?), &Py::list((v_qe.clone()).items()?))?))? {
        v_v = t3685;
        let _ = f__need({ let t3688 = py::math1("isfinite", &v_v.clone())?; if !t3688.truthy() { t3688 } else { { let t3686 = v_v.clone(); Py::Bool({ let t3687 = Py::Int(0i64); if py::gt(&t3686, &t3687)? { true } else { false } }) } } }, Py::str("every concentration and loading must be positive"))?;
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
    v_sxx = py::sum(&{ let mut out = vec![]; for t3689 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t3689;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t3690 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t3690;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t3691 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3692 = (t3691).items()?;
if t3692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3692[0].clone();
v_b = t3692[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t3697 = { let t3693 = v_sxx.clone(); Py::Bool({ let t3694 = Py::Int(0i64); if py::gt(&t3693, &t3694)? { true } else { false } }) }; if !t3697.truthy() { t3697 } else { { let t3695 = v_syy.clone(); Py::Bool({ let t3696 = Py::Int(0i64); if py::gt(&t3695, &t3696)? { true } else { false } }) } } }, Py::str("the data must vary"))?;
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
    let t3965 = (v_c.clone()).items()?;
    if t3965.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3965[0].clone();
    v_b = t3965[1].clone();
    v_p = t3965[2].clone();
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
    for t7190 in py::iter(&(v_vals.clone()))? {
        v_v = t7190;
        let _ = f__need({ let t7195 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t7195.truthy() { t7195 } else { { let t7194 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t7194.truthy() { t7194 } else { { let t7193 = py::math1("isfinite", &v_v.clone())?; if !t7193.truthy() { t7193 } else { { let t7191 = v_v.clone(); Py::Bool({ let t7192 = Py::Int(0i64); if py::ge(&t7191, &t7192)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t7293 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t7294 = (t7293).items()?;
if t7294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t7294[0].clone();
v_v = t7294[1].clone();
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
    for t7284 in py::iter(&(v_vals.clone()))? {
        v_v = t7284;
        let _ = f__need({ let t7289 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t7289.truthy() { t7289 } else { { let t7288 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t7288.truthy() { t7288 } else { { let t7287 = py::math1("isfinite", &v_v.clone())?; if !t7287.truthy() { t7287 } else { { let t7285 = v_v.clone(); Py::Bool({ let t7286 = Py::Int(0i64); if py::gt(&t7285, &t7286)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t7298 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t7298.truthy() { t7298 } else { { let t7297 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t7297.truthy() { t7297 } else { { let t7295 = v_x.clone(); Py::Bool({ let t7296 = Py::None; if py::is(&t7295, &t7296) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t7299 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t7299;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t7302 = { let t7300 = v_x.clone(); Py::Bool({ let t7301 = Py::Int(0i64); if py::eq(&t7300, &t7301) { true } else { false } }) }; if t7302.truthy() { t7302 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t7303 = v_x.clone(); Py::Bool({ let t7304 = Py::Int(0i64); if py::eq(&t7303, &t7304) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t7311 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t7311.truthy() { t7311 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t7315 = py::not(&{ let t7312 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t7312.truthy() { t7312 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t7315.truthy() { t7315 } else { { let t7313 = py::len(&v_a.clone())?; Py::Bool({ let t7314 = py::len(&v_b.clone())?; if py::ne(&t7313, &t7314) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t7316 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t7317 = (t7316).items()?;
if t7317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t7317[0].clone();
v_y = t7317[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t7318 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t7318.truthy() { t7318 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t7319 = v_a.clone(); Py::Bool({ let t7320 = v_b.clone(); if py::eq(&t7319, &t7320) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t7322: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t7321 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t7321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t7321[0].clone();
            v_b = t7321[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7322 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t7323 = v_d.clone(); Py::Bool({ let t7324 = Py::Int(0i64); if py::eq(&t7323, &t7324) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t7329 = { let t7325 = py::abs(&v_a.clone())?; Py::Bool({ let t7326 = Py::Float(1e-15); if py::lt(&t7325, &t7326)? { true } else { false } }) }; if !t7329.truthy() { t7329 } else { { let t7327 = py::abs(&v_b.clone())?; Py::Bool({ let t7328 = Py::Float(1e-15); if py::lt(&t7327, &t7328)? { true } else { false } }) } } }).truthy() {
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
    let t6312 = (v_c.clone()).items()?;
    if t6312.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6312[0].clone();
    v_b = t6312[1].clone();
    v_p = t6312[2].clone();
    return Ok(py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_b.clone(), &v_x.clone())?)?, &v_p.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sp_checks(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_DOsat: Py) -> R {
    let _ = f__pos(Py::str("kd, ka, L0 and DOsat"), Py::tuple(vec![v_kd.clone(), v_ka.clone(), v_L0.clone(), v_DOsat.clone()]))?;
    let _ = f__nonneg(Py::str("the initial deficit D0"), Py::tuple(vec![v_D0.clone()]))?;
    let _ = f__need({ let t6460 = v_D0.clone(); Py::Bool({ let t6461 = v_DOsat.clone(); if py::le(&t6460, &t6461)? { true } else { false } }) }, Py::str("the initial deficit cannot exceed the saturation concentration"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sp_deficit(mut v_kd: Py, mut v_ka: Py, mut v_L0: Py, mut v_D0: Py, mut v_t: Py) -> R {
    if ({ let t6462 = py::abs(&py::sub(&v_ka.clone(), &v_kd.clone())?)?; Py::Bool({ let t6463 = py::mul(&Py::Float(1e-09), &v_ka.clone())?; if py::le(&t6462, &t6463)? { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::add(&py::mul(&py::mul(&v_kd.clone(), &v_L0.clone())?, &v_t.clone())?, &v_D0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_t.clone())?)?)?);
    }
    return Ok(py::add(&py::mul(&py::truediv(&py::mul(&v_kd.clone(), &v_L0.clone())?, &py::sub(&v_ka.clone(), &v_kd.clone())?)?, &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?)?)?, &py::mul(&v_D0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__species(mut v_mass: Py, mut v_molar: Py, mut v_M: Py, mut v_name: Py) -> R {
    if ({ let t6738 = { let t6734 = v_mass.clone(); Py::Bool({ let t6735 = Py::None; if py::is(&t6734, &t6735) { true } else { false } }) }; if !t6738.truthy() { t6738 } else { { let t6736 = v_molar.clone(); Py::Bool({ let t6737 = Py::None; if py::is(&t6736, &t6737) { true } else { false } }) } } }).truthy() {
        return Ok(Py::None);
    }
    let _ = f__need({ let t6743 = { let t6739 = v_mass.clone(); Py::Bool({ let t6740 = Py::None; if py::is(&t6739, &t6740) { true } else { false } }) }; if t6743.truthy() { t6743 } else { { let t6741 = v_molar.clone(); Py::Bool({ let t6742 = Py::None; if py::is(&t6741, &t6742) { true } else { false } }) } } }, Py::str(&["give ".to_string(), py::format(&v_name.clone(), "")?, " once, as a mass or as a molar concentration".to_string()].concat()))?;
    if ({ let t6744 = v_mass.clone(); Py::Bool({ let t6745 = Py::None; if !py::is(&t6744, &t6745) { true } else { false } }) }).truthy() {
        let _ = f__nonneg(Py::str(&[py::format(&v_name.clone(), "")?, " concentration".to_string()].concat()), Py::tuple(vec![v_mass.clone()]))?;
        return Ok(py::truediv(&v_mass.clone(), &v_M.clone())?);
    }
    let _ = f__nonneg(Py::str(&[py::format(&v_name.clone(), "")?, " concentration".to_string()].concat()), Py::tuple(vec![v_molar.clone()]))?;
    return Ok(py::truediv(&v_molar.clone(), &Py::Float(1000.0))?);
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
fn f_alkalinity(mut v_HCO3: Py, mut v_CO3: Py, mut v_OH: Py, mut v_H: Py, mut v_HCO3_M: Py, mut v_CO3_M: Py, mut v_OH_M: Py, mut v_H_M: Py, mut v_pH: Py) -> R {
    let mut v_co3: Py = Py::None;
    let mut v_eq: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hco3: Py = Py::None;
    let mut v_oh: Py = Py::None;
    v_hco3 = f__species(v_HCO3.clone(), v_HCO3_M.clone(), c_MW_HCO3()?, Py::str("the bicarbonate"))?;
    v_co3 = f__species(v_CO3.clone(), v_CO3_M.clone(), c_MW_CO3()?, Py::str("the carbonate"))?;
    v_oh = f__species(v_OH.clone(), v_OH_M.clone(), c_MW_OH()?, Py::str("the hydroxide"))?;
    v_h = f__species(v_H.clone(), v_H_M.clone(), py::getitem(&c_AW()?, &Py::str("H"))?, Py::str("the hydrogen ion"))?;
    if ({ let t6752 = v_pH.clone(); Py::Bool({ let t6753 = Py::None; if !py::is(&t6752, &t6753) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6754 = v_h.clone(); Py::Bool({ let t6755 = Py::None; if py::is(&t6754, &t6755) { true } else { false } }) }, Py::str("give the hydrogen ion as pH or as a concentration, not both"))?;
        let _ = f__fin(Py::tuple(vec![v_pH.clone()]))?;
        let _ = f__need({ let t6759 = Py::Float(0.0); Py::Bool({ let t6760 = v_pH.clone(); if py::le(&t6759, &t6760)? { { let t6761 = Py::Float(14.0); if py::le(&t6760, &t6761)? { true } else { false } } } else { false } }) }, Py::str("the pH must lie between 0 and 14"))?;
        v_h = py::pow(&Py::Float(10.0), &py::neg(&v_pH.clone())?)?;
    }
    let _ = f__need({ let t6769 = { let t6762 = v_hco3.clone(); Py::Bool({ let t6763 = Py::None; if !py::is(&t6762, &t6763) { true } else { false } }) }; if t6769.truthy() { t6769 } else { { let t6768 = { let t6764 = v_co3.clone(); Py::Bool({ let t6765 = Py::None; if !py::is(&t6764, &t6765) { true } else { false } }) }; if t6768.truthy() { t6768 } else { { let t6766 = v_oh.clone(); Py::Bool({ let t6767 = Py::None; if !py::is(&t6766, &t6767) { true } else { false } }) } } } } }, Py::str("give at least one of the bicarbonate (HCO3), carbonate (CO3) or hydroxide (OH) concentrations"))?;
    if ({ let t6770 = v_hco3.clone(); Py::Bool({ let t6771 = Py::None; if py::is(&t6770, &t6771) { true } else { false } }) }).truthy() {
        v_hco3 = Py::Float(0.0);
    }
    if ({ let t6772 = v_co3.clone(); Py::Bool({ let t6773 = Py::None; if py::is(&t6772, &t6773) { true } else { false } }) }).truthy() {
        v_co3 = Py::Float(0.0);
    }
    if ({ let t6778 = { let t6774 = v_h.clone(); Py::Bool({ let t6775 = Py::None; if py::is(&t6774, &t6775) { true } else { false } }) }; if !t6778.truthy() { t6778 } else { { let t6776 = v_oh.clone(); Py::Bool({ let t6777 = Py::None; if py::is(&t6776, &t6777) { true } else { false } }) } } }).truthy() {
        v_h = Py::Float(0.0);
        v_oh = Py::Float(0.0);
    } else {
        if ({ let t6779 = v_oh.clone(); Py::Bool({ let t6780 = Py::None; if py::is(&t6779, &t6780) { true } else { false } }) }).truthy() {
            v_oh = (if ({ let t6781 = v_h.clone(); Py::Bool({ let t6782 = Py::Int(0i64); if py::gt(&t6781, &t6782)? { true } else { false } }) }).truthy() { py::truediv(&c_KW_25C()?, &v_h.clone())? } else { Py::Float(0.0) });
        } else {
            if ({ let t6783 = v_h.clone(); Py::Bool({ let t6784 = Py::None; if py::is(&t6783, &t6784) { true } else { false } }) }).truthy() {
                v_h = (if ({ let t6785 = v_oh.clone(); Py::Bool({ let t6786 = Py::Int(0i64); if py::gt(&t6785, &t6786)? { true } else { false } }) }).truthy() { py::truediv(&c_KW_25C()?, &v_oh.clone())? } else { Py::Float(0.0) });
            }
        }
    }
    v_eq = py::sub(&py::add(&py::add(&v_hco3.clone(), &py::mul(&Py::Float(2.0), &v_co3.clone())?)?, &v_oh.clone())?, &v_h.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("alkalinity"), py::mul(&v_eq.clone(), &c_EQ_CACO3()?)?), (Py::str("alk_meq_L"), py::mul(&v_eq.clone(), &Py::Float(1000.0))?), (Py::str("alk_eq_L"), v_eq.clone()), (Py::str("HCO3_caco3"), py::mul(&v_hco3.clone(), &c_EQ_CACO3()?)?), (Py::str("CO3_caco3"), py::mul(&py::mul(&Py::Float(2.0), &v_co3.clone())?, &c_EQ_CACO3()?)?), (Py::str("OH_caco3"), py::mul(&v_oh.clone(), &c_EQ_CACO3()?)?), (Py::str("H_caco3"), py::mul(&v_h.clone(), &c_EQ_CACO3()?)?), (Py::str("H_molL"), v_h.clone()), (Py::str("OH_molL"), v_oh.clone())]))?);
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
    v_xf = (if ({ let t6286 = v_F.clone(); Py::Bool({ let t6287 = Py::Float(55.0); if py::lt(&t6286, &t6287)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(49.0), &py::pow(&v_F.clone(), &Py::Float(0.625))?)? } else { py::mul(&Py::Float(119.0), &py::pow(&v_F.clone(), &Py::Float(0.4))?)? });
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
fn f_channel_velocity(mut v_W: Py, mut v_d: Py, mut v_Q: Py, mut v_v: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__need({ let t806 = { let t808 = v_Q.clone(); Py::Bool({ let t809 = Py::None; if py::is(&t808, &t809) { true } else { false } }) }; Py::Bool({ let t807 = { let t810 = v_v.clone(); Py::Bool({ let t811 = Py::None; if py::is(&t810, &t811) { true } else { false } }) }; if py::ne(&t806, &t807) { true } else { false } }) }, Py::str("give either the flow Q or the velocity v, not both"))?;
    let _ = f__pos(Py::str("the width W and the depth d"), Py::tuple(vec![v_W.clone(), v_d.clone()]))?;
    v_A = py::mul(&v_W.clone(), &v_d.clone())?;
    if ({ let t818 = v_Q.clone(); Py::Bool({ let t819 = Py::None; if !py::is(&t818, &t819) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the flow Q"), Py::tuple(vec![v_Q.clone()]))?;
        return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("v"), py::truediv(&v_Q.clone(), &v_A.clone())?), (Py::str("Q"), v_Q.clone())]))?);
    }
    let _ = f__pos(Py::str("the velocity v"), Py::tuple(vec![v_v.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("v"), v_v.clone()), (Py::str("Q"), py::mul(&v_v.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chemical_feed(mut v_Q: Py, mut v_dose: Py, mut v_purity: Py) -> R {
    let mut v_active: Py = Py::None;
    let mut v_feed: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__pos(Py::str("the flow Q and the dose"), Py::tuple(vec![v_Q.clone(), v_dose.clone()]))?;
    v_p = (if ({ let t864 = v_purity.clone(); Py::Bool({ let t865 = Py::None; if py::is(&t864, &t865) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_purity.clone() });
    let _ = f__frac(Py::str("the purity (active fraction of the product)"), v_p.clone(), Py::Bool(true), Py::Bool(false))?;
    v_active = py::mul(&v_Q.clone(), &v_dose.clone())?;
    v_feed = py::truediv(&v_active.clone(), &v_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("feed"), v_feed.clone()), (Py::str("feed_kg_per_day"), py::mul(&v_feed.clone(), &c_DAY()?)?), (Py::str("feed_lb_per_day"), py::truediv(&py::mul(&v_feed.clone(), &c_DAY()?)?, &c_LB()?)?), (Py::str("active_kg_per_day"), py::mul(&v_active.clone(), &c_DAY()?)?), (Py::str("active_lb_per_day"), py::truediv(&py::mul(&v_active.clone(), &c_DAY()?)?, &c_LB()?)?), (Py::str("Q_MGD"), py::truediv(&v_Q.clone(), &c_MGD_SI()?)?)]))?);
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
    let _ = f__need({ let t978 = Py::Float(0.3); Py::Bool({ let t979 = v_n.clone(); if py::le(&t978, &t979)? { { let t980 = Py::Float(3.0); if py::le(&t979, &t980)? { true } else { false } } } else { false } }) }, Py::str("the coefficient of dilution n must lie between 0.3 and 3"))?;
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
    let _ = f__need({ let t1043 = v_dose.clone(); Py::Bool({ let t1044 = Py::Int(0i64); if py::gt(&t1043, &t1044)? { true } else { false } }) }, Py::str("demand plus residual must be positive"))?;
    v_feed = py::mul(&v_Q.clone(), &v_dose.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("dose"), v_dose.clone()), (Py::str("dose_mgL"), py::mul(&v_dose.clone(), &Py::Float(1000.0))?), (Py::str("feed"), v_feed.clone()), (Py::str("feed_kg_per_day"), py::mul(&v_feed.clone(), &c_DAY()?)?), (Py::str("product_kg_per_day"), py::truediv(&py::mul(&v_feed.clone(), &c_DAY()?)?, &v_purity.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chronic_intake(mut v_C: Py, mut v_IR: Py, mut v_BW: Py, mut v_EF: Py, mut v_ED: Py, mut v_AT: Py, mut v_RfD: Py, mut v_SF: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cdi: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_risk: Py = Py::None;
    let _ = f__pos(Py::str("C, IR and BW"), Py::tuple(vec![v_C.clone(), v_IR.clone(), v_BW.clone()]))?;
    v_given = py::add(&py::add(&{ let t1089 = v_EF.clone(); Py::Bool({ let t1090 = Py::None; if !py::is(&t1089, &t1090) { true } else { false } }) }, &{ let t1091 = v_ED.clone(); Py::Bool({ let t1092 = Py::None; if !py::is(&t1091, &t1092) { true } else { false } }) })?, &{ let t1093 = v_AT.clone(); Py::Bool({ let t1094 = Py::None; if !py::is(&t1093, &t1094) { true } else { false } }) })?;
    let _ = f__need({ let t1099 = { let t1095 = v_given.clone(); Py::Bool({ let t1096 = Py::Int(0i64); if py::eq(&t1095, &t1096) { true } else { false } }) }; if t1099.truthy() { t1099 } else { { let t1097 = v_given.clone(); Py::Bool({ let t1098 = Py::Int(3i64); if py::eq(&t1097, &t1098) { true } else { false } }) } } }, Py::str("give EF, ED and AT together, or none of them (continuous lifetime exposure)"))?;
    v_c = py::mul(&v_C.clone(), &Py::Float(1000.0))?;
    v_ir = py::mul(&py::mul(&v_IR.clone(), &Py::Float(1000.0))?, &c_DAY()?)?;
    v_f = Py::Float(1.0);
    if ({ let t1100 = v_given.clone(); Py::Bool({ let t1101 = Py::Int(3i64); if py::eq(&t1100, &t1101) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("EF, ED and AT"), Py::tuple(vec![v_EF.clone(), v_ED.clone(), v_AT.clone()]))?;
        let _ = f__need({ let t1102 = v_EF.clone(); Py::Bool({ let t1103 = Py::Float(365.0); if py::le(&t1102, &t1103)? { true } else { false } }) }, Py::str("the exposure frequency EF cannot exceed 365 days per year"))?;
        let _ = f__need({ let t1104 = py::mul(&v_EF.clone(), &v_ED.clone())?; Py::Bool({ let t1105 = py::mul(&v_AT.clone(), &py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?; if py::le(&t1104, &t1105)? { true } else { false } }) }, Py::str("the exposure days EF x ED cannot exceed the averaging time AT"))?;
        v_f = py::truediv(&py::mul(&v_EF.clone(), &v_ED.clone())?, &v_AT.clone())?;
    }
    v_cdi = py::truediv(&py::mul(&py::mul(&v_c.clone(), &v_ir.clone())?, &v_f.clone())?, &v_BW.clone())?;
    v_out = Py::dict(vec![(Py::str("CDI"), v_cdi.clone())]);
    if ({ let t1106 = v_RfD.clone(); Py::Bool({ let t1107 = Py::None; if !py::is(&t1106, &t1107) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the reference dose RfD"), Py::tuple(vec![v_RfD.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("HQ"), py::truediv(&v_cdi.clone(), &v_RfD.clone())?)?;
    }
    if ({ let t1108 = v_SF.clone(); Py::Bool({ let t1109 = Py::None; if !py::is(&t1108, &t1109) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the slope factor SF"), Py::tuple(vec![v_SF.clone()]))?;
        v_risk = py::mul(&v_cdi.clone(), &v_SF.clone())?;
        let _ = f__need({ let t1110 = v_risk.clone(); Py::Bool({ let t1111 = Py::Float(0.01); if py::le(&t1110, &t1111)? { true } else { false } }) }, Py::str("the linear low-dose risk model holds only for a risk up to 0.01"))?;
        py::setitem(&v_out.clone(), &Py::str("risk"), v_risk.clone())?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cmfr_series(mut v_Cin: Py, mut v_k: Py, mut v_theta: Py, mut v_N: Py, mut v_V: Py, mut v_Q: Py) -> R {
    let mut v_Cone: Py = Py::None;
    let mut v_Cout: Py = Py::None;
    let mut v_Cpfr: Py = Py::None;
    let mut v_kt: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("Cin and k"), Py::tuple(vec![v_Cin.clone(), v_k.clone()]))?;
    if ({ let t1157 = v_theta.clone(); Py::Bool({ let t1158 = Py::None; if py::is(&t1157, &t1158) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t1163 = { let t1159 = v_V.clone(); Py::Bool({ let t1160 = Py::None; if !py::is(&t1159, &t1160) { true } else { false } }) }; if !t1163.truthy() { t1163 } else { { let t1161 = v_Q.clone(); Py::Bool({ let t1162 = Py::None; if !py::is(&t1161, &t1162) { true } else { false } }) } } }, Py::str("give the residence time theta, or the reactor volume V and the flow Q"))?;
        let _ = f__pos(Py::str("the volume V and the flow Q"), Py::tuple(vec![v_V.clone(), v_Q.clone()]))?;
        v_theta = py::truediv(&v_V.clone(), &v_Q.clone())?;
    } else {
        let _ = f__need({ let t1168 = { let t1164 = v_V.clone(); Py::Bool({ let t1165 = Py::None; if py::is(&t1164, &t1165) { true } else { false } }) }; if !t1168.truthy() { t1168 } else { { let t1166 = v_Q.clone(); Py::Bool({ let t1167 = Py::None; if py::is(&t1166, &t1167) { true } else { false } }) } } }, Py::str("give the residence time theta, or V and Q, not both"))?;
        let _ = f__pos(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    }
    v_n = (if ({ let t1169 = v_N.clone(); Py::Bool({ let t1170 = Py::None; if py::is(&t1169, &t1170) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { v_N.clone() });
    let _ = f__need({ let t1176 = { let t1171 = v_n.clone(); Py::Bool({ let t1172 = py::int(&v_n.clone())?; if py::eq(&t1171, &t1172) { true } else { false } }) }; if !t1176.truthy() { t1176 } else { { let t1173 = Py::Int(1i64); Py::Bool({ let t1174 = v_n.clone(); if py::le(&t1173, &t1174)? { { let t1175 = Py::Int(1000i64); if py::le(&t1174, &t1175)? { true } else { false } } } else { false } }) } } }, Py::str("the number of reactors N must be a whole number from 1 to 1000"))?;
    v_n = py::int(&v_n.clone())?;
    v_kt = py::mul(&v_k.clone(), &v_theta.clone())?;
    v_Cout = py::truediv(&v_Cin.clone(), &py::pow(&py::add(&Py::Float(1.0), &py::truediv(&v_kt.clone(), &v_n.clone())?)?, &v_n.clone())?)?;
    v_Cone = py::truediv(&v_Cin.clone(), &py::add(&Py::Float(1.0), &v_kt.clone())?)?;
    v_Cpfr = py::mul(&v_Cin.clone(), &py::math1("exp", &py::neg(&v_kt.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cout"), v_Cout.clone()), (Py::str("Cout_mgL"), py::mul(&v_Cout.clone(), &Py::Float(1000.0))?), (Py::str("removal"), py::sub(&Py::Float(1.0), &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?), (Py::str("removal_percent"), py::mul(&py::sub(&Py::Float(1.0), &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?, &Py::Float(100.0))?), (Py::str("Cout_one"), v_Cone.clone()), (Py::str("Cout_one_mgL"), py::mul(&v_Cone.clone(), &Py::Float(1000.0))?), (Py::str("Cout_pfr"), v_Cpfr.clone()), (Py::str("Cout_pfr_mgL"), py::mul(&v_Cpfr.clone(), &Py::Float(1000.0))?), (Py::str("removal_pfr"), py::sub(&Py::Float(1.0), &py::truediv(&v_Cpfr.clone(), &v_Cin.clone())?)?), (Py::str("theta_each"), py::truediv(&v_theta.clone(), &v_n.clone())?), (Py::str("theta_each_h"), py::truediv(&py::truediv(&v_theta.clone(), &v_n.clone())?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combustion_oxygen(mut v_formula: Py, mut v_m: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1295 = (f__cho_counts(v_formula.clone())?).items()?;
    if t1295.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1295[0].clone();
    v_y = t1295[1].clone();
    v_z = t1295[2].clone();
    v_n = py::sub(&py::add(&v_x.clone(), &py::truediv(&v_y.clone(), &Py::Float(4.0))?)?, &py::truediv(&v_z.clone(), &Py::Float(2.0))?)?;
    let _ = f__need({ let t1296 = v_n.clone(); Py::Bool({ let t1297 = Py::Int(0i64); if py::gt(&t1296, &t1297)? { true } else { false } }) }, Py::str("the fuel needs no oxygen to burn"))?;
    v_M = py::add(&py::add(&py::mul(&v_x.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_y.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?, &py::mul(&v_z.clone(), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?;
    v_o2 = py::truediv(&py::mul(&v_n.clone(), &c_MW_O2()?)?, &v_M.clone())?;
    v_out = Py::dict(vec![(Py::str("o2_per_mol"), v_n.clone()), (Py::str("molar_mass"), v_M.clone()), (Py::str("O2_per_kg"), v_o2.clone()), (Py::str("CO2_per_kg"), py::truediv(&py::mul(&v_x.clone(), &py::add(&py::getitem(&c_AW()?, &Py::str("C"))?, &py::mul(&Py::Int(2i64), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?)?, &v_M.clone())?), (Py::str("H2O_per_kg"), py::truediv(&py::mul(&py::truediv(&v_y.clone(), &Py::Float(2.0))?, &py::add(&py::mul(&Py::Int(2i64), &py::getitem(&c_AW()?, &Py::str("H"))?)?, &py::getitem(&c_AW()?, &Py::str("O"))?)?)?, &v_M.clone())?)]);
    if ({ let t1298 = v_m.clone(); Py::Bool({ let t1299 = Py::None; if !py::is(&t1298, &t1299) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the fuel mass m"), Py::tuple(vec![v_m.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("O2_mass"), py::mul(&v_o2.clone(), &v_m.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compaction_ratio(mut v_rho_i: Py, mut v_rho_c: Py, mut v_V_i: Py, mut v_V_c: Py) -> R {
    let mut v_CR: Py = Py::None;
    let mut v_dens: Py = Py::None;
    let mut v_red: Py = Py::None;
    let mut v_vols: Py = Py::None;
    v_dens = { let t3411 = { let t3407 = v_rho_i.clone(); Py::Bool({ let t3408 = Py::None; if !py::is(&t3407, &t3408) { true } else { false } }) }; if t3411.truthy() { t3411 } else { { let t3409 = v_rho_c.clone(); Py::Bool({ let t3410 = Py::None; if !py::is(&t3409, &t3410) { true } else { false } }) } } };
    v_vols = { let t3416 = { let t3412 = v_V_i.clone(); Py::Bool({ let t3413 = Py::None; if !py::is(&t3412, &t3413) { true } else { false } }) }; if t3416.truthy() { t3416 } else { { let t3414 = v_V_c.clone(); Py::Bool({ let t3415 = Py::None; if !py::is(&t3414, &t3415) { true } else { false } }) } } };
    let _ = f__need({ let t3417 = v_dens.clone(); Py::Bool({ let t3418 = v_vols.clone(); if py::ne(&t3417, &t3418) { true } else { false } }) }, Py::str("give either the two densities (rho_i, rho_c) or the two volumes (V_i, V_c)"))?;
    if (v_dens.clone()).truthy() {
        let _ = f__need({ let t3423 = { let t3419 = v_rho_i.clone(); Py::Bool({ let t3420 = Py::None; if !py::is(&t3419, &t3420) { true } else { false } }) }; if !t3423.truthy() { t3423 } else { { let t3421 = v_rho_c.clone(); Py::Bool({ let t3422 = Py::None; if !py::is(&t3421, &t3422) { true } else { false } }) } } }, Py::str("give both densities, before (rho_i) and after (rho_c) compaction"))?;
        let _ = f__pos(Py::str("the densities"), Py::tuple(vec![v_rho_i.clone(), v_rho_c.clone()]))?;
        v_CR = py::truediv(&v_rho_c.clone(), &v_rho_i.clone())?;
    } else {
        let _ = f__need({ let t3434 = { let t3430 = v_V_i.clone(); Py::Bool({ let t3431 = Py::None; if !py::is(&t3430, &t3431) { true } else { false } }) }; if !t3434.truthy() { t3434 } else { { let t3432 = v_V_c.clone(); Py::Bool({ let t3433 = Py::None; if !py::is(&t3432, &t3433) { true } else { false } }) } } }, Py::str("give both volumes, before (V_i) and after (V_c) compaction"))?;
        let _ = f__pos(Py::str("the volumes"), Py::tuple(vec![v_V_i.clone(), v_V_c.clone()]))?;
        v_CR = py::truediv(&v_V_i.clone(), &v_V_c.clone())?;
    }
    let _ = f__need({ let t3435 = v_CR.clone(); Py::Bool({ let t3436 = Py::Float(1.0); if py::ge(&t3435, &t3436)? { true } else { false } }) }, Py::str("compaction makes the waste denser: the compacted density must be at least the initial one"))?;
    v_red = py::sub(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_CR.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("CR"), v_CR.clone()), (Py::str("volume_reduction"), v_red.clone()), (Py::str("volume_reduction_percent"), py::mul(&v_red.clone(), &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ct_contact_time(mut v_CT: Py, mut v_C: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the required CT and the residual C"), Py::tuple(vec![v_CT.clone(), v_C.clone()]))?;
    v_t = py::truediv(&v_CT.clone(), &v_C.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &Py::Float(60.0))?), (Py::str("t_h"), py::truediv(&v_t.clone(), &c_HOUR()?)?)]))?);
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
    let _ = f__need({ let t1439 = v_rho_p.clone(); Py::Bool({ let t1440 = v_rho_g.clone(); if py::gt(&t1439, &t1440)? { true } else { false } }) }, Py::str("the particles must be denser than the gas"))?;
    v_d50 = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Float(9.0), &v_mu.clone())?, &v_W.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_Ne.clone())?, &v_Vi.clone())?, &py::sub(&v_rho_p.clone(), &v_rho_g.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d50"), v_d50.clone()), (Py::str("eta"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::pow(&py::truediv(&v_d50.clone(), &v_dp.clone())?, &Py::Int(2i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_darcy(mut v_K: Py, mut v_i: Py, mut v_dh: Py, mut v_L: Py, mut v_A: Py, mut v_n_e: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the hydraulic conductivity K"), Py::tuple(vec![v_K.clone()]))?;
    if ({ let t1485 = v_i.clone(); Py::Bool({ let t1486 = Py::None; if py::is(&t1485, &t1486) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t1491 = { let t1487 = v_dh.clone(); Py::Bool({ let t1488 = Py::None; if !py::is(&t1487, &t1488) { true } else { false } }) }; if !t1491.truthy() { t1491 } else { { let t1489 = v_L.clone(); Py::Bool({ let t1490 = Py::None; if !py::is(&t1489, &t1490) { true } else { false } }) } } }, Py::str("give the hydraulic gradient i, or the head difference dh and the flow length L"))?;
        let _ = f__pos(Py::str("dh and L"), Py::tuple(vec![v_dh.clone(), v_L.clone()]))?;
        v_i = py::truediv(&v_dh.clone(), &v_L.clone())?;
    } else {
        let _ = f__need({ let t1496 = { let t1492 = v_dh.clone(); Py::Bool({ let t1493 = Py::None; if py::is(&t1492, &t1493) { true } else { false } }) }; if !t1496.truthy() { t1496 } else { { let t1494 = v_L.clone(); Py::Bool({ let t1495 = Py::None; if py::is(&t1494, &t1495) { true } else { false } }) } } }, Py::str("give the gradient i or dh and L, not both"))?;
        let _ = f__pos(Py::str("the hydraulic gradient i"), Py::tuple(vec![v_i.clone()]))?;
    }
    v_q = py::mul(&v_K.clone(), &v_i.clone())?;
    v_out = Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("q"), v_q.clone()), (Py::str("q_m_per_day"), py::mul(&v_q.clone(), &c_DAY()?)?)]);
    if ({ let t1497 = v_A.clone(); Py::Bool({ let t1498 = Py::None; if !py::is(&t1497, &t1498) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the flow area A"), Py::tuple(vec![v_A.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("Q"), py::mul(&v_q.clone(), &v_A.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("Q_m3_per_day"), py::mul(&py::mul(&v_q.clone(), &v_A.clone())?, &c_DAY()?)?)?;
    }
    if ({ let t1499 = v_n_e.clone(); Py::Bool({ let t1500 = Py::None; if !py::is(&t1499, &t1500) { true } else { false } }) }).truthy() {
        let _ = f__frac(Py::str("the effective porosity n_e"), v_n_e.clone(), Py::Bool(true), Py::Bool(true))?;
        py::setitem(&v_out.clone(), &Py::str("v_s"), py::truediv(&v_q.clone(), &v_n_e.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("v_s_m_per_day"), py::mul(&py::truediv(&v_q.clone(), &v_n_e.clone())?, &c_DAY()?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_db_add(mut v_L: Py) -> R {
    let _ = f__need({ let t1615 = py::len(&v_L.clone())?; Py::Bool({ let t1616 = Py::Int(2i64); if py::ge(&t1615, &t1616)? { true } else { false } }) }, Py::str("give at least two levels"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("L_total"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::sum(&{ let mut out = vec![]; for t1634 in py::iter(&(v_L.clone()))? { let mut v_v: Py; 
v_v = t1634;
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
    let _ = f__need({ let t4512 = v_source.clone(); Py::Bool({ let t4513 = Py::tuple(vec![Py::str("point"), Py::str("line")]); if py::contains(&t4513, &t4512)? { true } else { false } }) }, Py::str("source: point or line"))?;
    v_f = (if ({ let t4514 = v_source.clone(); Py::Bool({ let t4515 = Py::str("point"); if py::eq(&t4514, &t4515) { true } else { false } }) }).truthy() { Py::Float(20.0) } else { Py::Float(10.0) });
    return Ok(f__out(Py::dict(vec![(Py::str("L2"), py::sub(&v_L1.clone(), &py::mul(&v_f.clone(), &py::math1("log10", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_do_saturation(mut v_T: Py, mut v_S: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_lnC: Py = Py::None;
    let _ = f__need({ let t1761 = c_T0C()?; Py::Bool({ let t1762 = v_T.clone(); if py::le(&t1761, &t1762)? { { let t1763 = py::add(&c_T0C()?, &Py::Float(40.0))?; if py::le(&t1762, &t1763)? { true } else { false } } } else { false } }) }, Py::str("the formula holds between 0 and 40 degC"))?;
    let _ = f__need({ let t1764 = Py::Float(0.0); Py::Bool({ let t1765 = v_S.clone(); if py::le(&t1764, &t1765)? { { let t1766 = Py::Float(40.0); if py::le(&t1765, &t1766)? { true } else { false } } } else { false } }) }, Py::str("the salinity must lie between 0 and 40 g/kg"))?;
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
    for t1805 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("C"), v_C.clone()]), Py::tuple(vec![Py::str("H"), v_H.clone()]), Py::tuple(vec![Py::str("O"), v_O.clone()]), Py::tuple(vec![Py::str("S"), v_S.clone()]), Py::tuple(vec![Py::str("N"), v_N.clone()])])))? {
        let t1818 = (t1805).items()?;
        if t1818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t1818[0].clone();
        v_v = t1818[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(false), Py::Bool(false))?;
    }
    let _ = f__need({ let t1819 = py::add(&py::add(&py::add(&py::add(&v_C.clone(), &v_H.clone())?, &v_O.clone())?, &v_S.clone())?, &v_N.clone())?; Py::Bool({ let t1820 = py::add(&Py::Float(1.0), &Py::Float(1e-12))?; if py::le(&t1819, &t1820)? { true } else { false } }) }, Py::str("the mass fractions cannot add to more than 1"))?;
    let _ = f__need({ let t1821 = v_C.clone(); Py::Bool({ let t1822 = Py::Int(0i64); if py::gt(&t1821, &t1822)? { true } else { false } }) }, Py::str("the waste must contain carbon"))?;
    v_kJ = py::add(&py::add(&py::add(&py::mul(&py::mul(&Py::Float(337.0), &v_C.clone())?, &Py::Int(100i64))?, &py::mul(&Py::Float(1419.0), &py::sub(&py::mul(&v_H.clone(), &Py::Int(100i64))?, &py::truediv(&py::mul(&v_O.clone(), &Py::Int(100i64))?, &Py::Float(8.0))?)?)?)?, &py::mul(&py::mul(&Py::Float(93.0), &v_S.clone())?, &Py::Int(100i64))?)?, &py::mul(&py::mul(&Py::Float(23.0), &v_N.clone())?, &Py::Int(100i64))?)?;
    let _ = f__need({ let t1823 = v_kJ.clone(); Py::Bool({ let t1824 = Py::Int(0i64); if py::gt(&t1823, &t1824)? { true } else { false } }) }, Py::str("the composition gives no positive heating value"))?;
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
    v_F = (if ({ let t2031 = v_Fp.clone(); Py::Bool({ let t2032 = Py::None; if py::is(&t2031, &t2032) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_Fp.clone() });
    let _ = f__pos(Py::str("the pressure correction Fp"), Py::tuple(vec![v_F.clone()]))?;
    v_drive = py::sub(&py::mul(&py::mul(&v_beta.clone(), &v_CsT.clone())?, &v_F.clone())?, &v_CL.clone())?;
    let _ = f__need({ let t2033 = v_drive.clone(); Py::Bool({ let t2034 = Py::Int(0i64); if py::gt(&t2033, &t2034)? { true } else { false } }) }, Py::str("the oxygen level is at or above the field saturation: no transfer"))?;
    v_ratio = py::truediv(&py::mul(&py::mul(&v_alpha.clone(), &py::pow(&v_theta.clone(), &py::sub(&v_T.clone(), &py::add(&c_T0C()?, &Py::Float(20.0))?)?)?)?, &v_drive.clone())?, &v_Cs20.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AOTR"), py::mul(&v_SOTR.clone(), &v_ratio.clone())?), (Py::str("AOTR_kg_per_h"), py::mul(&py::mul(&v_SOTR.clone(), &v_ratio.clone())?, &c_HOUR()?)?), (Py::str("ratio"), v_ratio.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filter_flow(mut v_v: Py, mut v_A: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::str("the loading rate v and the area A"), Py::tuple(vec![v_v.clone(), v_A.clone()]))?;
    v_Q = py::mul(&v_v.clone(), &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("Q_m3_per_h"), py::mul(&v_Q.clone(), &c_HOUR()?)?), (Py::str("Q_m3_per_day"), py::mul(&v_Q.clone(), &c_DAY()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filter_rate(mut v_Q: Py, mut v_A: Py, mut v_N: Py) -> R {
    let mut v_v: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let _ = f__pos(Py::str("Q and A"), Py::tuple(vec![v_Q.clone(), v_A.clone()]))?;
    let _ = f__need({ let t2181 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2181.truthy() { t2181 } else { { let t2179 = v_N.clone(); Py::Bool({ let t2180 = Py::Int(2i64); if py::ge(&t2179, &t2180)? { true } else { false } }) } } }, Py::str("give at least two filters (one may be out of service)"))?;
    v_v = py::truediv(&v_Q.clone(), &py::mul(&v_N.clone(), &v_A.clone())?)?;
    v_v1 = py::truediv(&v_Q.clone(), &py::mul(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_A.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("v_m_per_h"), py::mul(&v_v.clone(), &c_HOUR()?)?), (Py::str("v_one_out"), v_v1.clone()), (Py::str("v_one_out_m_per_h"), py::mul(&v_v1.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_decay(mut v_C0: Py, mut v_C: Py, mut v_k: Py, mut v_t: Py, mut v_t_half: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_known: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2220 = v_t_half.clone(); Py::Bool({ let t2221 = Py::None; if !py::is(&t2220, &t2221) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2222 = v_k.clone(); Py::Bool({ let t2223 = Py::None; if py::is(&t2222, &t2223) { true } else { false } }) }, Py::str("give the rate constant k or the half-life t_half, not both"))?;
        let _ = f__pos(Py::str("the half-life t_half"), Py::tuple(vec![v_t_half.clone()]))?;
        v_k = py::truediv(&c_LN2()?, &v_t_half.clone())?;
    }
    v_known = Py::Int(0i64);
    for t2230 in py::iter(&(Py::tuple(vec![v_C0.clone(), v_C.clone(), v_k.clone(), v_t.clone()])))? {
        v_v = t2230;
        if ({ let t2231 = v_v.clone(); Py::Bool({ let t2232 = Py::None; if !py::is(&t2231, &t2232) { true } else { false } }) }).truthy() {
            v_known = py::add(&v_known.clone(), &Py::Int(1i64))?;
        }
    }
    let _ = f__need({ let t2233 = v_known.clone(); Py::Bool({ let t2234 = Py::Int(3i64); if py::eq(&t2233, &t2234) { true } else { false } }) }, Py::str("give exactly three of C0, C, k (or t_half) and t: the fourth is computed"))?;
    if ({ let t2235 = v_C0.clone(); Py::Bool({ let t2236 = Py::None; if !py::is(&t2235, &t2236) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the initial concentration C0"), Py::tuple(vec![v_C0.clone()]))?;
    }
    if ({ let t2237 = v_C.clone(); Py::Bool({ let t2238 = Py::None; if !py::is(&t2237, &t2238) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the concentration C"), Py::tuple(vec![v_C.clone()]))?;
    }
    if ({ let t2239 = v_k.clone(); Py::Bool({ let t2240 = Py::None; if !py::is(&t2239, &t2240) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the rate constant k"), Py::tuple(vec![v_k.clone()]))?;
    }
    if ({ let t2241 = v_t.clone(); Py::Bool({ let t2242 = Py::None; if !py::is(&t2241, &t2242) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    }
    if ({ let t2247 = { let t2243 = v_C0.clone(); Py::Bool({ let t2244 = Py::None; if !py::is(&t2243, &t2244) { true } else { false } }) }; if !t2247.truthy() { t2247 } else { { let t2245 = v_C.clone(); Py::Bool({ let t2246 = Py::None; if !py::is(&t2245, &t2246) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t2248 = v_C.clone(); Py::Bool({ let t2249 = v_C0.clone(); if py::lt(&t2248, &t2249)? { true } else { false } }) }, Py::str("a first-order decay needs C below C0"))?;
    }
    if ({ let t2254 = { let t2250 = v_k.clone(); Py::Bool({ let t2251 = Py::None; if !py::is(&t2250, &t2251) { true } else { false } }) }; if !t2254.truthy() { t2254 } else { { let t2252 = v_t.clone(); Py::Bool({ let t2253 = Py::None; if !py::is(&t2252, &t2253) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t2255 = py::mul(&v_k.clone(), &v_t.clone())?; Py::Bool({ let t2256 = Py::Float(700.0); if py::le(&t2255, &t2256)? { true } else { false } }) }, Py::str("k t is too large (above 700): nothing measurable is left"))?;
    }
    if ({ let t2257 = v_C.clone(); Py::Bool({ let t2258 = Py::None; if py::is(&t2257, &t2258) { true } else { false } }) }).truthy() {
        v_C = py::mul(&v_C0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?;
    } else {
        if ({ let t2259 = v_C0.clone(); Py::Bool({ let t2260 = Py::None; if py::is(&t2259, &t2260) { true } else { false } }) }).truthy() {
            v_C0 = py::mul(&v_C.clone(), &py::math1("exp", &py::mul(&v_k.clone(), &v_t.clone())?)?)?;
        } else {
            if ({ let t2261 = v_k.clone(); Py::Bool({ let t2262 = Py::None; if py::is(&t2261, &t2262) { true } else { false } }) }).truthy() {
                v_k = py::truediv(&py::math1("log", &py::truediv(&v_C0.clone(), &v_C.clone())?)?, &v_t.clone())?;
            } else {
                v_t = py::truediv(&py::math1("log", &py::truediv(&v_C0.clone(), &v_C.clone())?)?, &v_k.clone())?;
            }
        }
    }
    v_f = py::truediv(&v_C.clone(), &v_C0.clone())?;
    v_th = py::truediv(&c_LN2()?, &v_k.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mgL"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("C0"), v_C0.clone()), (Py::str("C0_mgL"), py::mul(&v_C0.clone(), &Py::Float(1000.0))?), (Py::str("k"), v_k.clone()), (Py::str("k_per_day"), py::mul(&v_k.clone(), &c_DAY()?)?), (Py::str("k_per_h"), py::mul(&v_k.clone(), &c_HOUR()?)?), (Py::str("t"), v_t.clone()), (Py::str("t_days"), py::truediv(&v_t.clone(), &c_DAY()?)?), (Py::str("t_h"), py::truediv(&v_t.clone(), &c_HOUR()?)?), (Py::str("t_half"), v_th.clone()), (Py::str("t_half_days"), py::truediv(&v_th.clone(), &c_DAY()?)?), (Py::str("t_half_h"), py::truediv(&v_th.clone(), &c_HOUR()?)?), (Py::str("fraction_remaining"), v_f.clone()), (Py::str("removal_percent"), py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &Py::Float(100.0))?)]))?);
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
    let _ = f__need({ let t2351 = v_Ce.clone(); Py::Bool({ let t2352 = v_C0.clone(); if py::lt(&t2351, &t2352)? { true } else { false } }) }, Py::str("the target concentration must be below the initial concentration"))?;
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
    let t2411 = (f__linfit({ let mut out = vec![]; for t2409 in py::iter(&(v_Ce.clone()))? { let mut v_v: Py; 
v_v = t2409;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2410 in py::iter(&(v_qe.clone()))? { let mut v_v: Py; 
v_v = t2410;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t2411.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2411[0].clone();
    v_i = t2411[1].clone();
    v_r2 = t2411[2].clone();
    let _ = f__need({ let t2412 = v_s.clone(); Py::Bool({ let t2413 = Py::Int(0i64); if py::gt(&t2412, &t2413)? { true } else { false } }) }, Py::str("the loading must rise with concentration"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), py::pow(&Py::Float(10.0), &v_i.clone())?), (Py::str("inv_n"), v_s.clone()), (Py::str("n"), py::truediv(&Py::Float(1.0), &v_s.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geometric_mean(mut v_x: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2505 = py::len(&v_x.clone())?; Py::Bool({ let t2506 = Py::Int(2i64); if py::ge(&t2505, &t2506)? { true } else { false } }) }, Py::str("give at least two values"))?;
    v_s = Py::Float(0.0);
    v_t = Py::Float(0.0);
    for t2507 in py::iter(&(v_x.clone()))? {
        v_v = t2507;
        let _ = f__pos(Py::str("every value (a count of zero has no logarithm)"), Py::tuple(vec![v_v.clone()]))?;
        v_s = py::add(&v_s.clone(), &py::math1("log", &v_v.clone())?)?;
        v_t = py::add(&v_t.clone(), &v_v.clone())?;
    }
    v_m = py::len(&v_x.clone())?;
    v_mean = py::truediv(&v_t.clone(), &v_m.clone())?;
    v_d = Py::Float(0.0);
    for t2514 in py::iter(&(v_x.clone()))? {
        v_v = t2514;
        v_d = py::add(&v_d.clone(), &py::pow(&py::sub(&v_v.clone(), &v_mean.clone())?, &Py::Int(2i64))?)?;
    }
    v_gm = py::math1("exp", &py::truediv(&v_s.clone(), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("geometric_mean"), v_gm.clone()), (Py::str("mean"), v_mean.clone()), (Py::str("sd"), py::math1("sqrt", &py::truediv(&v_d.clone(), &py::sub(&v_m.clone(), &Py::Int(1i64))?)?)?), (Py::str("mean_log10"), py::truediv(&py::truediv(&v_s.clone(), &v_m.clone())?, &py::math1("log", &Py::Float(10.0))?)?), (Py::str("count"), py::float(&v_m.clone())?)]))?);
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
    v_cls = (if ({ let t7196 = v_th.clone(); Py::Bool({ let t7197 = Py::Int(60i64); if py::lt(&t7196, &t7197)? { true } else { false } }) }).truthy() { Py::str("soft") } else { (if ({ let t7198 = v_th.clone(); Py::Bool({ let t7199 = Py::Int(120i64); if py::lt(&t7198, &t7199)? { true } else { false } }) }).truthy() { Py::str("moderately hard") } else { (if ({ let t7200 = v_th.clone(); Py::Bool({ let t7201 = Py::Int(180i64); if py::lt(&t7200, &t7201)? { true } else { false } }) }).truthy() { Py::str("hard") } else { Py::str("very hard") }) }) });
    return Ok(f__out(Py::dict(vec![(Py::str("total_hardness"), v_th.clone()), (Py::str("calcium_hardness"), v_ca_h.clone()), (Py::str("magnesium_hardness"), v_mg_h.clone()), (Py::str("classification"), v_cls.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hazen_williams(mut v_C: Py, mut v_D: Py, mut v_S: Py, mut v_Q: Py, mut v_L: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the Hazen-Williams coefficient C and the diameter D"), Py::tuple(vec![v_C.clone(), v_D.clone()]))?;
    let _ = f__need({ let t2609 = { let t2611 = v_S.clone(); Py::Bool({ let t2612 = Py::None; if py::is(&t2611, &t2612) { true } else { false } }) }; Py::Bool({ let t2610 = { let t2613 = v_Q.clone(); Py::Bool({ let t2614 = Py::None; if py::is(&t2613, &t2614) { true } else { false } }) }; if py::ne(&t2609, &t2610) { true } else { false } }) }, Py::str("give either the slope of the energy line S or the flow Q, not both"))?;
    v_R = py::truediv(&v_D.clone(), &Py::Float(4.0))?;
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Float(4.0))?;
    v_k = py::mul(&py::mul(&Py::Float(0.849), &v_C.clone())?, &py::pow(&v_R.clone(), &Py::Float(0.63))?)?;
    if ({ let t2615 = v_S.clone(); Py::Bool({ let t2616 = Py::None; if !py::is(&t2615, &t2616) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::tuple(vec![v_S.clone()]))?;
        let _ = f__need({ let t2620 = Py::Float(0.0); Py::Bool({ let t2621 = v_S.clone(); if py::lt(&t2620, &t2621)? { { let t2622 = Py::Float(1.0); if py::lt(&t2621, &t2622)? { true } else { false } } } else { false } }) }, Py::str("the slope S must be between 0 and 1 (a fraction, or written with %)"))?;
        v_v = py::mul(&v_k.clone(), &py::pow(&v_S.clone(), &Py::Float(0.54))?)?;
        v_Q = py::mul(&v_v.clone(), &v_A.clone())?;
    } else {
        let _ = f__pos(Py::str("the flow Q"), Py::tuple(vec![v_Q.clone()]))?;
        v_v = py::truediv(&v_Q.clone(), &v_A.clone())?;
        v_S = py::pow(&py::truediv(&v_v.clone(), &v_k.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(0.54))?)?;
        let _ = f__need({ let t2623 = v_S.clone(); Py::Bool({ let t2624 = Py::Float(1.0); if py::lt(&t2623, &t2624)? { true } else { false } }) }, Py::str("the head loss gradient would exceed 1: outside the range of the Hazen-Williams formula"))?;
    }
    v_out = Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("S"), v_S.clone()), (Py::str("R"), v_R.clone()), (Py::str("A"), v_A.clone())]);
    if ({ let t2625 = v_L.clone(); Py::Bool({ let t2626 = Py::None; if !py::is(&t2625, &t2626) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the pipe length L"), Py::tuple(vec![v_L.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("hf"), py::mul(&v_S.clone(), &v_L.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_convert(mut v_H: Py, mut v_form: Py, mut v_T: Py) -> R {
    let mut v_hpc: Py = Py::None;
    let _ = f__pos(Py::str("H and T"), Py::tuple(vec![v_H.clone(), v_T.clone()]))?;
    let _ = f__need({ let t2741 = v_form.clone(); Py::Bool({ let t2742 = c__HFORMS()?; if py::contains(&t2742, &t2741)? { true } else { false } }) }, py::add(&Py::str("form: "), &py::method(&Py::str(", "), "join", &[c__HFORMS()?])?)?)?;
    if ({ let t2743 = v_form.clone(); Py::Bool({ let t2744 = Py::str("pa_m3_per_mol"); if py::eq(&t2743, &t2744) { true } else { false } }) }).truthy() {
        v_hpc = v_H.clone();
    } else {
        if ({ let t2745 = v_form.clone(); Py::Bool({ let t2746 = Py::str("atm_m3_per_mol"); if py::eq(&t2745, &t2746) { true } else { false } }) }).truthy() {
            v_hpc = py::mul(&v_H.clone(), &c_P_ATM()?)?;
        } else {
            if ({ let t2747 = v_form.clone(); Py::Bool({ let t2748 = Py::str("dimensionless"); if py::eq(&t2747, &t2748) { true } else { false } }) }).truthy() {
                v_hpc = py::mul(&py::mul(&v_H.clone(), &c_R_GAS()?)?, &v_T.clone())?;
            } else {
                if ({ let t2749 = v_form.clone(); Py::Bool({ let t2750 = Py::str("atm"); if py::eq(&t2749, &t2750) { true } else { false } }) }).truthy() {
                    v_hpc = py::truediv(&py::mul(&v_H.clone(), &c_P_ATM()?)?, &c_RHO_W_MOLAR()?)?;
                } else {
                    if ({ let t2751 = v_form.clone(); Py::Bool({ let t2752 = Py::str("mol_per_m3_pa"); if py::eq(&t2751, &t2752) { true } else { false } }) }).truthy() {
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
    if ({ let t2797 = v_M.clone(); Py::Bool({ let t2798 = Py::None; if !py::is(&t2797, &t2798) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the molar mass M"), Py::tuple(vec![v_M.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("C_mass"), py::mul(&v_C.clone(), &v_M.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("C_mgL"), py::mul(&py::mul(&v_C.clone(), &v_M.clone())?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_solubility(mut v_KH: Py, mut v_P: Py, mut v_M: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("KH and P"), Py::tuple(vec![v_KH.clone(), v_P.clone()]))?;
    v_C = py::mul(&v_KH.clone(), &v_P.clone())?;
    v_out = Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_molL"), py::truediv(&v_C.clone(), &Py::Float(1000.0))?)]);
    if ({ let t2844 = v_M.clone(); Py::Bool({ let t2845 = Py::None; if !py::is(&t2844, &t2845) { true } else { false } }) }).truthy() {
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
fn f_hhv_dry_basis(mut v_w: Py, mut v_HHV_ar: Py, mut v_HHV_dry: Py, mut v_ash: Py) -> R {
    let mut v_ar: Py = Py::None;
    let mut v_dry: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__frac(Py::str("the moisture fraction w"), v_w.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t2678 = { let t2680 = v_HHV_ar.clone(); Py::Bool({ let t2681 = Py::None; if py::is(&t2680, &t2681) { true } else { false } }) }; Py::Bool({ let t2679 = { let t2682 = v_HHV_dry.clone(); Py::Bool({ let t2683 = Py::None; if py::is(&t2682, &t2683) { true } else { false } }) }; if py::ne(&t2678, &t2679) { true } else { false } }) }, Py::str("give the heating value as received (HHV_ar) or on a dry basis (HHV_dry), not both"))?;
    if ({ let t2684 = v_HHV_ar.clone(); Py::Bool({ let t2685 = Py::None; if !py::is(&t2684, &t2685) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the heating value"), Py::tuple(vec![v_HHV_ar.clone()]))?;
        v_ar = v_HHV_ar.clone();
        v_dry = py::truediv(&v_ar.clone(), &py::sub(&Py::Float(1.0), &v_w.clone())?)?;
    } else {
        let _ = f__pos(Py::str("the heating value"), Py::tuple(vec![v_HHV_dry.clone()]))?;
        v_dry = v_HHV_dry.clone();
        v_ar = py::mul(&v_dry.clone(), &py::sub(&Py::Float(1.0), &v_w.clone())?)?;
    }
    v_out = Py::dict(vec![(Py::str("HHV_dry"), v_dry.clone()), (Py::str("HHV_ar"), v_ar.clone()), (Py::str("HHV_dry_MJ_per_kg"), py::truediv(&v_dry.clone(), &Py::Float(1000000.0))?), (Py::str("HHV_ar_MJ_per_kg"), py::truediv(&v_ar.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t2692 = v_ash.clone(); Py::Bool({ let t2693 = Py::None; if !py::is(&t2692, &t2693) { true } else { false } }) }).truthy() {
        let _ = f__frac(Py::str("the ash fraction (as received)"), v_ash.clone(), Py::Bool(false), Py::Bool(false))?;
        let _ = f__need({ let t2694 = py::add(&v_w.clone(), &v_ash.clone())?; Py::Bool({ let t2695 = Py::Float(1.0); if py::lt(&t2694, &t2695)? { true } else { false } }) }, Py::str("the moisture and ash fractions must add up to less than 1"))?;
        py::setitem(&v_out.clone(), &Py::str("HHV_daf"), py::truediv(&v_ar.clone(), &py::sub(&py::sub(&Py::Float(1.0), &v_w.clone())?, &v_ash.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("HHV_daf_MJ_per_kg"), py::truediv(&py::truediv(&v_ar.clone(), &py::sub(&py::sub(&Py::Float(1.0), &v_w.clone())?, &v_ash.clone())?)?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hocl_fraction(mut v_pH: Py, mut v_pKa: Py, mut v_T: Py) -> R {
    let mut v_a: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_pH.clone()]))?;
    let _ = f__need({ let t3031 = Py::Float(0.0); Py::Bool({ let t3032 = v_pH.clone(); if py::le(&t3031, &t3032)? { { let t3033 = Py::Float(14.0); if py::le(&t3032, &t3033)? { true } else { false } } } else { false } }) }, Py::str("pH must lie between 0 and 14"))?;
    let _ = f__need({ let t3034 = { let t3036 = v_pKa.clone(); Py::Bool({ let t3037 = Py::None; if py::is(&t3036, &t3037) { true } else { false } }) }; Py::Bool({ let t3035 = { let t3038 = v_T.clone(); Py::Bool({ let t3039 = Py::None; if py::is(&t3038, &t3039) { true } else { false } }) }; if py::ne(&t3034, &t3035) { true } else { false } }) }, Py::str("give either pKa or the temperature T (to compute pKa), not both"))?;
    if ({ let t3040 = v_pKa.clone(); Py::Bool({ let t3041 = Py::None; if py::is(&t3040, &t3041) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3042 = c_T0C()?; Py::Bool({ let t3043 = v_T.clone(); if py::le(&t3042, &t3043)? { { let t3044 = py::add(&c_T0C()?, &Py::Float(35.0))?; if py::le(&t3043, &t3044)? { true } else { false } } } else { false } }) }, Py::str("the pKa correlation holds between 0 and 35 degC"))?;
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
    let _ = f__need({ let t2938 = v_Ts.clone(); Py::Bool({ let t2939 = v_Ta.clone(); if py::gt(&t2938, &t2939)? { true } else { false } }) }, Py::str("the stack gas must be warmer than the air (Ts > Ta)"))?;
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
fn f_indoor_air(mut v_S: Py, mut v_Qv: Py, mut v_V: Py, mut v_Camb: Py, mut v_k: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_ca: Py = Py::None;
    let mut v_kk: Py = Py::None;
    let _ = f__nonneg(Py::str("the indoor emission rate S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__pos(Py::str("the ventilation rate Qv and the room volume V"), Py::tuple(vec![v_Qv.clone(), v_V.clone()]))?;
    v_ca = (if ({ let t3139 = v_Camb.clone(); Py::Bool({ let t3140 = Py::None; if py::is(&t3139, &t3140) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_Camb.clone() });
    v_kk = (if ({ let t3141 = v_k.clone(); Py::Bool({ let t3142 = Py::None; if py::is(&t3141, &t3142) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_k.clone() });
    let _ = f__nonneg(Py::str("the outdoor concentration Camb"), Py::tuple(vec![v_ca.clone()]))?;
    let _ = f__nonneg(Py::str("the decay rate k"), Py::tuple(vec![v_kk.clone()]))?;
    let _ = f__need({ let t3143 = py::add(&v_S.clone(), &py::mul(&v_Qv.clone(), &v_ca.clone())?)?; Py::Bool({ let t3144 = Py::Int(0i64); if py::gt(&t3143, &t3144)? { true } else { false } }) }, Py::str("nothing enters the room: give an emission rate S or an outdoor concentration Camb"))?;
    v_C = py::truediv(&py::add(&py::mul(&v_Qv.clone(), &v_ca.clone())?, &v_S.clone())?, &py::add(&v_Qv.clone(), &py::mul(&v_kk.clone(), &v_V.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_ugm3"), py::mul(&v_C.clone(), &Py::Float(1000000000.0))?), (Py::str("C_mgm3"), py::mul(&v_C.clone(), &Py::Float(1000000.0))?), (Py::str("ach"), py::mul(&py::truediv(&v_Qv.clone(), &v_V.clone())?, &c_HOUR()?)?), (Py::str("tau_h"), py::truediv(&py::truediv(&v_V.clone(), &py::add(&v_Qv.clone(), &py::mul(&v_kk.clone(), &v_V.clone())?)?)?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_temperature(mut v_k20: Py, mut v_theta: Py, mut v_T: Py) -> R {
    let mut v_kT: Py = Py::None;
    let _ = f__pos(Py::str("k20 and theta"), Py::tuple(vec![v_k20.clone(), v_theta.clone()]))?;
    let _ = f__need({ let t5280 = Py::Float(0.9); Py::Bool({ let t5281 = v_theta.clone(); if py::lt(&t5280, &t5281)? { { let t5282 = Py::Float(1.2); if py::lt(&t5281, &t5282)? { true } else { false } } } else { false } }) }, Py::str("theta must lie between 0.9 and 1.2 (typical 1.024 to 1.135)"))?;
    let _ = f__need({ let t5283 = py::sub(&c_T0C()?, &Py::Float(5.0))?; Py::Bool({ let t5284 = v_T.clone(); if py::le(&t5283, &t5284)? { { let t5285 = py::add(&c_T0C()?, &Py::Float(60.0))?; if py::le(&t5284, &t5285)? { true } else { false } } } else { false } }) }, Py::str("the temperature must lie between -5 and 60 degC"))?;
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
    let _ = f__need({ let t3187 = { let t3183 = py::len(&v_t.clone())?; Py::Bool({ let t3184 = py::len(&v_C.clone())?; if py::eq(&t3183, &t3184) { true } else { false } }) }; if !t3187.truthy() { t3187 } else { { let t3185 = py::len(&v_t.clone())?; Py::Bool({ let t3186 = Py::Int(3i64); if py::ge(&t3185, &t3186)? { true } else { false } }) } } }, Py::str("give at least three times and as many DO readings"))?;
    let _ = f__pos(Py::str("Cs"), Py::tuple(vec![v_Cs.clone()]))?;
    v_cs = py::mul(&v_Cs.clone(), &Py::Float(1000.0))?;
    for t3194 in py::iter(&(v_C.clone()))? {
        v_v = t3194;
        let _ = f__need({ let t3198 = py::math1("isfinite", &v_v.clone())?; if !t3198.truthy() { t3198 } else { { let t3195 = Py::Int(0i64); Py::Bool({ let t3196 = v_v.clone(); if py::le(&t3195, &t3196)? { { let t3197 = v_cs.clone(); if py::lt(&t3196, &t3197)? { true } else { false } } } else { false } }) } } }, Py::str("every DO reading must be below the saturation concentration"))?;
    }
    let _ = f__need({ let t3199 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t3200 = py::len(&v_t.clone())?; if py::eq(&t3199, &t3200) { true } else { false } }) }, Py::str("the times must be distinct"))?;
    let t3211 = (f__linfit(Py::list((v_t.clone()).items()?), { let mut out = vec![]; for t3210 in py::iter(&(v_C.clone()))? { let mut v_v: Py; 
v_v = t3210;
out.push(py::math1("log", &py::sub(&v_cs.clone(), &v_v.clone())?)?); } Py::list(out) })?).items()?;
    if t3211.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3211[0].clone();
    v_i = t3211[1].clone();
    v_r2 = t3211[2].clone();
    let _ = f__need({ let t3212 = v_s.clone(); Py::Bool({ let t3213 = Py::Int(0i64); if py::lt(&t3212, &t3213)? { true } else { false } }) }, Py::str("the deficit must fall with time"))?;
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
    let _ = f__need({ let t3264 = v_C1.clone(); Py::Bool({ let t3265 = v_C2.clone(); if py::lt(&t3264, &t3265)? { { let t3266 = v_Cs.clone(); if py::lt(&t3265, &t3266)? { true } else { false } } } else { false } }) }, Py::str("the concentrations must rise toward saturation: C1 < C2 < Cs"))?;
    let _ = f__need({ let t3267 = v_t2.clone(); Py::Bool({ let t3268 = v_t1.clone(); if py::gt(&t3267, &t3268)? { true } else { false } }) }, Py::str("t2 must be later than t1"))?;
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
    if ({ let t3542 = v_t.clone(); Py::Bool({ let t3543 = v_tc.clone(); if py::le(&t3542, &t3543)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t3588 = Py::Float(0.0); Py::Bool({ let t3589 = v_cover.clone(); if py::le(&t3588, &t3589)? { { let t3590 = Py::Float(1.0); if py::le(&t3589, &t3590)? { true } else { false } } } else { false } }) }, Py::str("the cover ratio must lie between 0 and 1"))?;
    let _ = f__need({ let t3591 = Py::Float(0.0); Py::Bool({ let t3592 = v_g.clone(); if py::le(&t3591, &t3592)? { { let t3593 = Py::Float(0.2); if py::le(&t3592, &t3593)? { true } else { false } } } else { false } }) }, Py::str("the growth rate must lie between 0 and 20 % a year"))?;
    v_V0 = py::mul(&py::truediv(&py::mul(&v_M.clone(), &c_YEAR()?)?, &v_rho.clone())?, &py::add(&Py::Float(1.0), &v_cover.clone())?)?;
    v_n = (if ({ let t3594 = v_g.clone(); Py::Bool({ let t3595 = Py::Int(0i64); if py::eq(&t3594, &t3595) { true } else { false } }) }).truthy() { py::truediv(&v_capacity.clone(), &v_V0.clone())? } else { py::truediv(&py::math1("log", &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_capacity.clone(), &v_g.clone())?, &v_V0.clone())?)?)?, &py::math1("log", &py::add(&Py::Float(1.0), &v_g.clone())?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("V_first_year"), v_V0.clone()), (Py::str("life_years"), v_n.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_landfill_volume(mut v_m: Py, mut v_rho: Py, mut v_cover: Py, mut v_capacity: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_Vw: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the waste mass rate m and the compacted density rho"), Py::tuple(vec![v_m.clone(), v_rho.clone()]))?;
    v_c = (if ({ let t3481 = v_cover.clone(); Py::Bool({ let t3482 = Py::None; if py::is(&t3481, &t3482) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_cover.clone() });
    let _ = f__fin(Py::tuple(vec![v_c.clone()]))?;
    let _ = f__need({ let t3486 = Py::Float(0.0); Py::Bool({ let t3487 = v_c.clone(); if py::le(&t3486, &t3487)? { { let t3488 = Py::Float(1.0); if py::le(&t3487, &t3488)? { true } else { false } } } else { false } }) }, Py::str("the cover ratio must lie between 0 and 1 (cover soil per volume of compacted waste)"))?;
    v_Vw = py::truediv(&v_m.clone(), &v_rho.clone())?;
    v_V = py::mul(&v_Vw.clone(), &py::add(&Py::Float(1.0), &v_c.clone())?)?;
    v_out = Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_m3_per_day"), py::mul(&v_V.clone(), &c_DAY()?)?), (Py::str("V_m3_per_year"), py::mul(&py::mul(&v_V.clone(), &c_DAY()?)?, &Py::Float(365.0))?), (Py::str("V_waste_m3_per_day"), py::mul(&v_Vw.clone(), &c_DAY()?)?), (Py::str("V_cover_m3_per_day"), py::mul(&py::mul(&v_Vw.clone(), &v_c.clone())?, &c_DAY()?)?)]);
    if ({ let t3489 = v_capacity.clone(); Py::Bool({ let t3490 = Py::None; if !py::is(&t3489, &t3490) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the landfill capacity"), Py::tuple(vec![v_capacity.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("life_years"), py::truediv(&v_capacity.clone(), &py::mul(&py::mul(&v_V.clone(), &c_DAY()?)?, &Py::Float(365.0))?)?)?;
        py::setitem(&v_out.clone(), &Py::str("life_days"), py::truediv(&v_capacity.clone(), &py::mul(&v_V.clone(), &c_DAY()?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_langmuir_dose(mut v_C0: Py, mut v_Ce: Py, mut v_qmax: Py, mut v_b: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_dose: Py = Py::None;
    let mut v_qe: Py = Py::None;
    let _ = f__pos(Py::str("C0, Ce, qmax and b"), Py::tuple(vec![v_C0.clone(), v_Ce.clone(), v_qmax.clone(), v_b.clone()]))?;
    let _ = f__need({ let t3640 = v_Ce.clone(); Py::Bool({ let t3641 = v_C0.clone(); if py::lt(&t3640, &t3641)? { true } else { false } }) }, Py::str("the target concentration must be below the initial concentration"))?;
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
    let t3700 = (f__linfit(Py::list((v_Ce.clone()).items()?), { let mut out = vec![]; for t3698 in py::iter(&(py::zip(&[v_Ce.clone(), v_qe.clone()])?))? { let mut v_c: Py; let mut v_q: Py; 
let t3699 = (t3698).items()?;
if t3699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t3699[0].clone();
v_q = t3699[1].clone();
out.push(py::truediv(&v_c.clone(), &v_q.clone())?); } Py::list(out) })?).items()?;
    if t3700.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3700[0].clone();
    v_i = t3700[1].clone();
    v_r2 = t3700[2].clone();
    let _ = f__need({ let t3705 = { let t3701 = v_s.clone(); Py::Bool({ let t3702 = Py::Int(0i64); if py::gt(&t3701, &t3702)? { true } else { false } }) }; if !t3705.truthy() { t3705 } else { { let t3703 = v_i.clone(); Py::Bool({ let t3704 = Py::Int(0i64); if py::gt(&t3703, &t3704)? { true } else { false } }) } } }, Py::str("the data do not follow a Langmuir isotherm (the linearised slope and intercept must be positive)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("qmax"), py::truediv(&Py::Float(1.0), &v_s.clone())?), (Py::str("b"), py::truediv(&v_s.clone(), &v_i.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lapse_stability(mut v_T1: Py, mut v_z1: Py, mut v_T2: Py, mut v_z2: Py) -> R {
    let mut v_elr: Py = Py::None;
    let mut v_km: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the temperatures (absolute)"), Py::tuple(vec![v_T1.clone(), v_T2.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_z1.clone(), v_z2.clone()]))?;
    let _ = f__need({ let t3753 = v_z2.clone(); Py::Bool({ let t3754 = v_z1.clone(); if py::gt(&t3753, &t3754)? { true } else { false } }) }, Py::str("the second height z2 must be above the first z1"))?;
    v_elr = py::truediv(&py::neg(&py::sub(&v_T2.clone(), &v_T1.clone())?)?, &py::sub(&v_z2.clone(), &v_z1.clone())?)?;
    v_km = py::mul(&v_elr.clone(), &Py::Float(1000.0))?;
    if ({ let t3755 = py::abs(&py::sub(&v_km.clone(), &c_GAMMA_DRY()?)?)?; Py::Bool({ let t3756 = Py::Float(1e-06); if py::le(&t3755, &t3756)? { true } else { false } }) }).truthy() {
        v_s = Py::str("neutral");
    } else {
        if ({ let t3757 = v_km.clone(); Py::Bool({ let t3758 = c_GAMMA_DRY()?; if py::gt(&t3757, &t3758)? { true } else { false } }) }).truthy() {
            v_s = Py::str("superadiabatic (unstable)");
        } else {
            if ({ let t3759 = v_km.clone(); Py::Bool({ let t3760 = Py::Float(0.0); if py::ge(&t3759, &t3760)? { true } else { false } }) }).truthy() {
                v_s = Py::str("subadiabatic (stable)");
            } else {
                v_s = Py::str("inversion (very stable)");
            }
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ELR"), v_elr.clone()), (Py::str("ELR_C_per_km"), v_km.clone()), (Py::str("stability"), v_s.clone())]))?);
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
    let _ = f__need({ let t1552 = py::len(&v_L.clone())?; Py::Bool({ let t1553 = Py::Int(24i64); if py::eq(&t1552, &t1553) { true } else { false } }) }, Py::str("give 24 hourly levels, starting with the hour from midnight to 01:00"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    let _ = f__need({ let t1559 = v_metric.clone(); Py::Bool({ let t1560 = Py::tuple(vec![Py::str("ldn"), Py::str("lden")]); if py::contains(&t1560, &t1559)? { true } else { false } }) }, Py::str("metric: ldn or lden"))?;
    v_s = Py::Float(0.0);
    for t1561 in py::iter(&(py::enumerate(&v_L.clone(), 0)?))? {
        let t1576 = (t1561).items()?;
        if t1576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h = t1576[0].clone();
        v_v = t1576[1].clone();
        if ({ let t1562 = v_metric.clone(); Py::Bool({ let t1563 = Py::str("ldn"); if py::eq(&t1562, &t1563) { true } else { false } }) }).truthy() {
            v_pen = (if ({ let t1568 = { let t1564 = v_h.clone(); Py::Bool({ let t1565 = Py::Int(7i64); if py::lt(&t1564, &t1565)? { true } else { false } }) }; if t1568.truthy() { t1568 } else { { let t1566 = v_h.clone(); Py::Bool({ let t1567 = Py::Int(22i64); if py::ge(&t1566, &t1567)? { true } else { false } }) } } }).truthy() { Py::Float(10.0) } else { Py::Float(0.0) });
        } else {
            v_pen = (if ({ let t1573 = { let t1569 = v_h.clone(); Py::Bool({ let t1570 = Py::Int(7i64); if py::lt(&t1569, &t1570)? { true } else { false } }) }; if t1573.truthy() { t1573 } else { { let t1571 = v_h.clone(); Py::Bool({ let t1572 = Py::Int(23i64); if py::ge(&t1571, &t1572)? { true } else { false } }) } } }).truthy() { Py::Float(10.0) } else { (if ({ let t1574 = v_h.clone(); Py::Bool({ let t1575 = Py::Int(19i64); if py::ge(&t1574, &t1575)? { true } else { false } }) }).truthy() { Py::Float(5.0) } else { Py::Float(0.0) }) });
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
    let _ = f__need({ let t1867 = { let t1863 = py::len(&v_L.clone())?; Py::Bool({ let t1864 = py::len(&v_t.clone())?; if py::eq(&t1863, &t1864) { true } else { false } }) }; if !t1867.truthy() { t1867 } else { { let t1865 = py::len(&v_L.clone())?; Py::Bool({ let t1866 = Py::Int(1i64); if py::ge(&t1865, &t1866)? { true } else { false } }) } } }, Py::str("give one duration per level"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    for t1873 in py::iter(&(v_t.clone()))? {
        v_v = t1873;
        let _ = f__need({ let t1876 = py::math1("isfinite", &v_v.clone())?; if !t1876.truthy() { t1876 } else { { let t1874 = v_v.clone(); Py::Bool({ let t1875 = Py::Int(0i64); if py::ge(&t1874, &t1875)? { true } else { false } }) } } }, Py::str("durations must be zero or positive"))?;
    }
    v_T = py::sum(&v_t.clone(), None)?;
    let _ = f__need({ let t1877 = v_T.clone(); Py::Bool({ let t1878 = Py::Int(0i64); if py::gt(&t1877, &t1878)? { true } else { false } }) }, Py::str("the total duration must be positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Leq"), py::mul(&Py::Float(10.0), &py::math1("log10", &py::truediv(&py::sum(&{ let mut out = vec![]; for t1891 in py::iter(&(py::zip(&[v_L.clone(), v_t.clone()])?))? { let mut v_ti: Py; let mut v_v: Py; 
let t1892 = (t1891).items()?;
if t1892.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1892[0].clone();
v_ti = t1892[1].clone();
out.push(py::mul(&v_ti.clone(), &py::pow(&Py::Float(10.0), &py::truediv(&v_v.clone(), &Py::Float(10.0))?)?)?); } Py::list(out) }, None)?, &v_T.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_removal(mut v_N0: Py, mut v_N: Py) -> R {
    let _ = f__pos(Py::str("N0 and N"), Py::tuple(vec![v_N0.clone(), v_N.clone()]))?;
    let _ = f__need({ let t3805 = v_N.clone(); Py::Bool({ let t3806 = v_N0.clone(); if py::lt(&t3805, &t3806)? { true } else { false } }) }, Py::str("the final count must be below the initial count"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("log_removal"), py::math1("log10", &py::truediv(&v_N0.clone(), &v_N.clone())?)?), (Py::str("percent_removal"), py::mul(&py::sub(&Py::Float(1.0), &py::truediv(&v_N.clone(), &v_N0.clone())?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning(mut v_n: Py, mut v_S: Py, mut v_W: Py, mut v_y: Py, mut v_D: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("Manning's n"), Py::tuple(vec![v_n.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_S.clone()]))?;
    let _ = f__need({ let t3854 = Py::Float(0.0); Py::Bool({ let t3855 = v_S.clone(); if py::lt(&t3854, &t3855)? { { let t3856 = Py::Float(1.0); if py::lt(&t3855, &t3856)? { true } else { false } } } else { false } }) }, Py::str("the slope S must be between 0 and 1 (a fraction, or written with %)"))?;
    if ({ let t3857 = v_D.clone(); Py::Bool({ let t3858 = Py::None; if !py::is(&t3857, &t3858) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3863 = { let t3859 = v_W.clone(); Py::Bool({ let t3860 = Py::None; if py::is(&t3859, &t3860) { true } else { false } }) }; if !t3863.truthy() { t3863 } else { { let t3861 = v_y.clone(); Py::Bool({ let t3862 = Py::None; if py::is(&t3861, &t3862) { true } else { false } }) } } }, Py::str("give a rectangular channel (W, y) or a full circular pipe (D), not both"))?;
        let _ = f__pos(Py::str("the diameter D"), Py::tuple(vec![v_D.clone()]))?;
        v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Float(4.0))?;
        v_P = py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?;
    } else {
        let _ = f__need({ let t3868 = { let t3864 = v_W.clone(); Py::Bool({ let t3865 = Py::None; if !py::is(&t3864, &t3865) { true } else { false } }) }; if !t3868.truthy() { t3868 } else { { let t3866 = v_y.clone(); Py::Bool({ let t3867 = Py::None; if !py::is(&t3866, &t3867) { true } else { false } }) } } }, Py::str("give the channel width W and the flow depth y, or a pipe diameter D"))?;
        let _ = f__pos(Py::str("the width W and the depth y"), Py::tuple(vec![v_W.clone(), v_y.clone()]))?;
        v_A = py::mul(&v_W.clone(), &v_y.clone())?;
        v_P = py::add(&v_W.clone(), &py::mul(&Py::Float(2.0), &v_y.clone())?)?;
    }
    v_R = py::truediv(&v_A.clone(), &v_P.clone())?;
    v_v = py::truediv(&py::mul(&py::pow(&v_R.clone(), &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?, &py::math1("sqrt", &v_S.clone())?)?, &v_n.clone())?;
    v_Q = py::mul(&v_v.clone(), &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("Q_L_per_s"), py::mul(&v_Q.clone(), &Py::Float(1000.0))?), (Py::str("R"), v_R.clone()), (Py::str("A"), v_A.clone()), (Py::str("P"), v_P.clone())]))?);
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
    let t3963 = (f__briggs_coef(v_cls.clone(), v_terrain.clone())?).items()?;
    if t3963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cy = t3963[0].clone();
    v_cz = t3963[1].clone();
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
    v_xs = { let mut out = vec![]; for t3966 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3966;
out.push(py::mul(&Py::Float(100.0), &py::pow(&Py::Float(100.0), &py::truediv(&v_i.clone(), &v_n.clone())?)?)?); } Py::list(out) };
    let _ = f__need({ let t3967 = py::call(&v_g.clone(), &[py::getitem(&v_xs.clone(), &Py::Int(0i64))?])?; Py::Bool({ let t3968 = Py::Int(0i64); if py::gt(&t3967, &t3968)? { true } else { false } }) }, Py::str("the maximum lies closer than 100 m: outside the range of the Briggs coefficients"))?;
    v_lo = Py::None;
    for t3969 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3969;
        if ({ let t3970 = py::call(&v_g.clone(), &[py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?; Py::Bool({ let t3971 = Py::Int(0i64); if py::le(&t3970, &t3971)? { true } else { false } }) }).truthy() {
            v_lo = v_i.clone();
            break;
        }
    }
    let _ = f__need({ let t3972 = v_lo.clone(); Py::Bool({ let t3973 = Py::None; if !py::is(&t3972, &t3973) { true } else { false } }) }, Py::str("the maximum lies beyond 10 km: outside the range of the Briggs coefficients"))?;
    v_x = f__bisect(v_g.clone(), py::getitem(&v_xs.clone(), &v_lo.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_lo.clone(), &Py::Int(1i64))?)?, Py::Int(500i64))?;
    let t4000 = (Py::tuple(vec![f__sig_eval(v_cy.clone(), v_x.clone())?, f__sig_eval(v_cz.clone(), v_x.clone())?])).items()?;
    if t4000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t4000[0].clone();
    v_sz = t4000[1].clone();
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
fn f_mcrt_masses(mut v_M: Py, mut v_Mw: Py, mut v_Me: Py) -> R {
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("the solids mass in the basin M"), Py::tuple(vec![v_M.clone()]))?;
    let _ = f__nonneg(Py::str("the solids wasted and lost per day"), Py::tuple(vec![v_Mw.clone(), v_Me.clone()]))?;
    let _ = f__need({ let t4095 = py::add(&v_Mw.clone(), &v_Me.clone())?; Py::Bool({ let t4096 = Py::Int(0i64); if py::gt(&t4095, &t4096)? { true } else { false } }) }, Py::str("some solids must leave the system (wasted or in the effluent)"))?;
    v_th = py::truediv(&v_M.clone(), &py::add(&v_Mw.clone(), &v_Me.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_c"), v_th.clone()), (Py::str("theta_c_days"), py::truediv(&v_th.clone(), &c_DAY()?)?)]))?);
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
    let _ = f__need({ let t4206 = v_Qp.clone(); Py::Bool({ let t4207 = v_Qf.clone(); if py::lt(&t4206, &t4207)? { true } else { false } }) }, Py::str("the permeate flow must be below the feed flow"))?;
    let _ = f__need({ let t4208 = v_Cp.clone(); Py::Bool({ let t4209 = v_Cf.clone(); if py::lt(&t4208, &t4209)? { true } else { false } }) }, Py::str("the permeate must be cleaner than the feed"))?;
    v_Qc = py::sub(&v_Qf.clone(), &v_Qp.clone())?;
    v_Cc = py::truediv(&py::sub(&py::mul(&v_Qf.clone(), &v_Cf.clone())?, &py::mul(&v_Qp.clone(), &v_Cp.clone())?)?, &v_Qc.clone())?;
    v_J = py::truediv(&v_Qp.clone(), &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("recovery"), py::truediv(&v_Qp.clone(), &v_Qf.clone())?), (Py::str("rejection"), py::sub(&Py::Float(1.0), &py::truediv(&v_Cp.clone(), &v_Cf.clone())?)?), (Py::str("J"), v_J.clone()), (Py::str("J_LMH"), py::mul(&v_J.clone(), &Py::Float(3600000.0))?), (Py::str("Qc"), v_Qc.clone()), (Py::str("Cc"), v_Cc.clone()), (Py::str("Cc_mgL"), py::mul(&v_Cc.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_flux(mut v_Qp: Py, mut v_A: Py, mut v_J: Py) -> R {
    let mut v_known: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_known = Py::Int(0i64);
    for t4135 in py::iter(&(Py::tuple(vec![v_Qp.clone(), v_A.clone(), v_J.clone()])))? {
        v_v = t4135;
        if ({ let t4136 = v_v.clone(); Py::Bool({ let t4137 = Py::None; if !py::is(&t4136, &t4137) { true } else { false } }) }).truthy() {
            v_known = py::add(&v_known.clone(), &Py::Int(1i64))?;
        }
    }
    let _ = f__need({ let t4138 = v_known.clone(); Py::Bool({ let t4139 = Py::Int(2i64); if py::eq(&t4138, &t4139) { true } else { false } }) }, Py::str("give exactly two of the permeate flow Qp, the membrane area A and the flux J"))?;
    if ({ let t4140 = v_Qp.clone(); Py::Bool({ let t4141 = Py::None; if !py::is(&t4140, &t4141) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the permeate flow Qp"), Py::tuple(vec![v_Qp.clone()]))?;
    }
    if ({ let t4148 = v_A.clone(); Py::Bool({ let t4149 = Py::None; if !py::is(&t4148, &t4149) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the membrane area A"), Py::tuple(vec![v_A.clone()]))?;
    }
    if ({ let t4150 = v_J.clone(); Py::Bool({ let t4151 = Py::None; if !py::is(&t4150, &t4151) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the flux J"), Py::tuple(vec![v_J.clone()]))?;
    }
    if ({ let t4152 = v_J.clone(); Py::Bool({ let t4153 = Py::None; if py::is(&t4152, &t4153) { true } else { false } }) }).truthy() {
        v_J = py::truediv(&v_Qp.clone(), &v_A.clone())?;
    } else {
        if ({ let t4154 = v_A.clone(); Py::Bool({ let t4155 = Py::None; if py::is(&t4154, &t4155) { true } else { false } }) }).truthy() {
            v_A = py::truediv(&v_Qp.clone(), &v_J.clone())?;
        } else {
            v_Qp = py::mul(&v_J.clone(), &v_A.clone())?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("J"), v_J.clone()), (Py::str("J_LMH"), py::mul(&v_J.clone(), &Py::Float(3600000.0))?), (Py::str("J_m_per_day"), py::mul(&v_J.clone(), &c_DAY()?)?), (Py::str("A"), v_A.clone()), (Py::str("Qp"), v_Qp.clone()), (Py::str("Qp_m3_per_day"), py::mul(&v_Qp.clone(), &c_DAY()?)?)]))?);
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
    let _ = f__need({ let t4260 = v_inv.clone(); Py::Bool({ let t4261 = Py::Int(0i64); if py::gt(&t4260, &t4261)? { true } else { false } }) }, Py::str("the organisms cannot grow on this substrate (Y k S0/(Ks + S0) <= kd)"))?;
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
    let _ = f__need({ let t4348 = { let t4344 = py::len(&v_Q.clone())?; Py::Bool({ let t4345 = py::len(&v_C.clone())?; if py::eq(&t4344, &t4345) { true } else { false } }) }; if !t4348.truthy() { t4348 } else { { let t4346 = py::len(&v_Q.clone())?; Py::Bool({ let t4347 = Py::Int(2i64); if py::ge(&t4346, &t4347)? { true } else { false } }) } } }, Py::str("give at least two flows and as many concentrations"))?;
    for t4349 in py::iter(&(v_Q.clone()))? {
        v_q = t4349;
        let _ = f__need({ let t4352 = py::math1("isfinite", &v_q.clone())?; if !t4352.truthy() { t4352 } else { { let t4350 = v_q.clone(); Py::Bool({ let t4351 = Py::Int(0i64); if py::gt(&t4350, &t4351)? { true } else { false } }) } } }, Py::str("every flow must be positive"))?;
    }
    for t4353 in py::iter(&(v_C.clone()))? {
        v_c = t4353;
        let _ = f__need({ let t4356 = py::math1("isfinite", &v_c.clone())?; if !t4356.truthy() { t4356 } else { { let t4354 = v_c.clone(); Py::Bool({ let t4355 = Py::Int(0i64); if py::ge(&t4354, &t4355)? { true } else { false } }) } } }, Py::str("every concentration must be zero or positive"))?;
    }
    let t4357 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t4357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tq = t4357[0].clone();
    v_m = t4357[1].clone();
    for t4358 in py::iter(&(py::zip(&[v_Q.clone(), v_C.clone()])?))? {
        let t4359 = (t4358).items()?;
        if t4359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q = t4359[0].clone();
        v_c = t4359[1].clone();
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
    if ({ let t4454 = v_kd.clone(); Py::Bool({ let t4455 = Py::None; if !py::is(&t4454, &t4455) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t4608 = { let t4604 = py::len(&v_L.clone())?; Py::Bool({ let t4605 = py::len(&v_hours.clone())?; if py::eq(&t4604, &t4605) { true } else { false } }) }; if !t4608.truthy() { t4608 } else { { let t4606 = py::len(&v_L.clone())?; Py::Bool({ let t4607 = Py::Int(1i64); if py::ge(&t4606, &t4607)? { true } else { false } }) } } }, Py::str("give one exposure time per level"))?;
    let _ = f__db_list(v_L.clone(), Py::str("every level"))?;
    for t4614 in py::iter(&(v_hours.clone()))? {
        v_v = t4614;
        let _ = f__need({ let t4617 = py::math1("isfinite", &v_v.clone())?; if !t4617.truthy() { t4617 } else { { let t4615 = v_v.clone(); Py::Bool({ let t4616 = Py::Int(0i64); if py::ge(&t4615, &t4616)? { true } else { false } }) } } }, Py::str("exposure times must be zero or positive"))?;
    }
    v_D = Py::Float(0.0);
    for t4618 in py::iter(&(py::zip(&[v_L.clone(), v_hours.clone()])?))? {
        let t4621 = (t4618).items()?;
        if t4621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t4621[0].clone();
        v_h = t4621[1].clone();
        if ({ let t4619 = v_v.clone(); Py::Bool({ let t4620 = Py::Float(80.0); if py::ge(&t4619, &t4620)? { true } else { false } }) }).truthy() {
            v_D = py::iadd(&v_D, &py::truediv(&v_h.clone(), &py::truediv(&Py::Float(8.0), &py::pow(&Py::Float(2.0), &py::truediv(&py::sub(&v_v.clone(), &Py::Float(90.0))?, &Py::Float(5.0))?)?)?)?)?;
        }
    }
    v_D = py::mul(&v_D, &Py::Float(100.0))?;
    let _ = f__need({ let t4622 = v_D.clone(); Py::Bool({ let t4623 = Py::Int(0i64); if py::gt(&t4622, &t4623)? { true } else { false } }) }, Py::str("no exposure at or above 80 dBA: the dose is zero"))?;
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
fn f_ph_poh(mut v_H: Py, mut v_OH: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_other: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__need({ let t4806 = { let t4808 = v_H.clone(); Py::Bool({ let t4809 = Py::None; if py::is(&t4808, &t4809) { true } else { false } }) }; Py::Bool({ let t4807 = { let t4810 = v_OH.clone(); Py::Bool({ let t4811 = Py::None; if py::is(&t4810, &t4811) { true } else { false } }) }; if py::ne(&t4806, &t4807) { true } else { false } }) }, Py::str("give either the hydrogen ion concentration H or the hydroxide concentration OH, not both"))?;
    v_c = (if ({ let t4812 = v_H.clone(); Py::Bool({ let t4813 = Py::None; if !py::is(&t4812, &t4813) { true } else { false } }) }).truthy() { v_H.clone() } else { v_OH.clone() });
    let _ = f__pos(Py::str("the ion concentration"), Py::tuple(vec![v_c.clone()]))?;
    v_m = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    let _ = f__need({ let t4820 = py::mul(&Py::Float(1e-14), &py::sub(&Py::Float(1.0), &Py::Float(1e-09))?)?; Py::Bool({ let t4821 = v_m.clone(); if py::le(&t4820, &t4821)? { { let t4822 = py::mul(&Py::Float(1.0), &py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?; if py::le(&t4821, &t4822)? { true } else { false } } } else { false } }) }, Py::str("the ion concentration must lie between 1e-14 and 1 mol/L (dilute solution at 25 degC)"))?;
    v_p = py::neg(&py::math1("log10", &v_m.clone())?)?;
    v_other = py::truediv(&c_KW_25C()?, &v_m.clone())?;
    if ({ let t4823 = v_H.clone(); Py::Bool({ let t4824 = Py::None; if !py::is(&t4823, &t4824) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("pH"), v_p.clone()), (Py::str("pOH"), py::sub(&Py::Float(14.0), &v_p.clone())?), (Py::str("H_molL"), v_m.clone()), (Py::str("OH_molL"), v_other.clone())]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("pH"), py::sub(&Py::Float(14.0), &v_p.clone())?), (Py::str("pOH"), v_p.clone()), (Py::str("H_molL"), v_other.clone()), (Py::str("OH_molL"), v_m.clone())]))?);
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
    let _ = f__need({ let t6270 = v_cls.clone(); Py::Bool({ let t6271 = Py::tuple(vec![Py::str("a"), Py::str("b"), Py::str("c"), Py::str("d")]); if py::contains(&t6271, &t6270)? { true } else { false } }) }, Py::str("the neutral/unstable plume-rise formula needs a class A to D"))?;
    v_r = f_briggs_rise(v_vs.clone(), v_d.clone(), v_Ts.clone(), v_Ta.clone(), v_u.clone(), v_hs.clone())?;
    v_H = py::add(&v_hs.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(1.6), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?, &py::pow(&(if ({ let t6300 = py::getitem(&v_r.clone(), &Py::str("F"))?; Py::Bool({ let t6301 = Py::Float(55.0); if py::lt(&t6300, &t6301)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(49.0), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &Py::Float(0.625))?)? } else { py::mul(&Py::Float(119.0), &py::pow(&py::getitem(&v_r.clone(), &Py::str("F"))?, &Py::Float(0.4))?)? }), &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?, &v_u.clone())?)?;
    let t6313 = (f__briggs(v_x.clone(), v_cls.clone(), v_terrain.clone())?).items()?;
    if t6313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t6313[0].clone();
    v_sz = t6313[1].clone();
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
    let _ = f__need({ let t4947 = { let t4943 = v_H.clone(); Py::Bool({ let t4944 = v_Lm.clone(); if py::le(&t4943, &t4944)? { true } else { false } }) }; if !t4947.truthy() { t4947 } else { { let t4945 = v_z.clone(); Py::Bool({ let t4946 = v_Lm.clone(); if py::le(&t4945, &t4946)? { true } else { false } }) } } }, Py::str("the source and the receptor must lie below the mixing height"))?;
    let _ = f__need({ let t4948 = v_sigma_z.clone(); Py::Bool({ let t4949 = py::mul(&Py::Float(10.0), &v_Lm.clone())?; if py::le(&t4948, &t4949)? { true } else { false } }) }, Py::str("sigma_z may be at most ten mixing heights (use the uniformly mixed limit beyond)"))?;
    v_tot = Py::Float(0.0);
    v_nmax = py::add(&py::int(&py::mul(&py::truediv(&v_sigma_z.clone(), &v_Lm.clone())?, &Py::Float(4.0))?)?, &Py::Int(6i64))?;
    for t4950 in py::range_iter(&[(py::neg(&v_nmax.clone())?).i()?, (py::add(&v_nmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t4950;
        for t4951 in py::iter(&(Py::tuple(vec![py::add(&py::sub(&v_z.clone(), &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_n.clone())?, &v_Lm.clone())?)?, py::add(&py::add(&v_z.clone(), &v_H.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_n.clone())?, &v_Lm.clone())?)?])))? {
            v_a = t4951;
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
    let t4889 = (f__briggs(v_x.clone(), v_cls.clone(), v_terrain.clone())?).items()?;
    if t4889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t4889[0].clone();
    v_sz = t4889[1].clone();
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
fn f_population_logistic(mut v_t0: Py, mut v_n: Py, mut v_P0: Py, mut v_P1: Py, mut v_P2: Py, mut v_t: Py) -> R {
    let mut v_Ps: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t0.clone(), v_n.clone(), v_P0.clone(), v_P1.clone(), v_P2.clone(), v_t.clone()]))?;
    let _ = f__need({ let t5043 = Py::Int(0i64); Py::Bool({ let t5044 = v_n.clone(); if py::lt(&t5043, &t5044)? { { let t5045 = Py::Int(100i64); if py::le(&t5044, &t5045)? { true } else { false } } } else { false } }) }, Py::str("the census interval must be between 0 and 100 years"))?;
    let _ = f__pos(Py::str("the census populations"), Py::tuple(vec![v_P0.clone(), v_P1.clone(), v_P2.clone()]))?;
    let _ = f__need({ let t5052 = v_P0.clone(); Py::Bool({ let t5053 = v_P1.clone(); if py::lt(&t5052, &t5053)? { { let t5054 = v_P2.clone(); if py::lt(&t5053, &t5054)? { true } else { false } } } else { false } }) }, Py::str("the three censuses must increase (P0 < P1 < P2)"))?;
    let _ = f__need({ let t5055 = py::mul(&v_P1.clone(), &v_P1.clone())?; Py::Bool({ let t5056 = py::mul(&v_P0.clone(), &v_P2.clone())?; if py::gt(&t5055, &t5056)? { true } else { false } }) }, Py::str("the growth is not slowing (P1^2 <= P0 P2): there is no saturation population; use the geometric method"))?;
    let _ = f__need({ let t5057 = py::abs(&py::sub(&v_t.clone(), &v_t0.clone())?)?; Py::Bool({ let t5058 = Py::Int(500i64); if py::le(&t5057, &t5058)? { true } else { false } }) }, Py::str("the projection year must lie within 500 years of the first census"))?;
    v_Ps = py::add(&v_P2.clone(), &py::truediv(&py::mul(&py::mul(&v_P0.clone(), &py::sub(&v_P2.clone(), &v_P1.clone())?)?, &py::sub(&v_P2.clone(), &v_P1.clone())?)?, &py::sub(&py::mul(&v_P1.clone(), &v_P1.clone())?, &py::mul(&v_P0.clone(), &v_P2.clone())?)?)?)?;
    let _ = f__need({ let t5059 = v_Ps.clone(); Py::Bool({ let t5060 = py::mul(&Py::Int(100i64), &v_P2.clone())?; if py::le(&t5059, &t5060)? { true } else { false } }) }, Py::str("the saturation population exceeds 100 times the last census: the data show no clear saturation"))?;
    v_m = py::truediv(&py::sub(&v_Ps.clone(), &v_P0.clone())?, &v_P0.clone())?;
    v_k = py::truediv(&py::math1("log", &py::truediv(&py::mul(&v_P1.clone(), &py::sub(&v_Ps.clone(), &v_P0.clone())?)?, &py::mul(&v_P0.clone(), &py::sub(&v_Ps.clone(), &v_P1.clone())?)?)?)?, &v_n.clone())?;
    let _ = f__need({ let t5061 = py::mul(&v_k.clone(), &py::abs(&py::sub(&v_t.clone(), &v_t0.clone())?)?)?; Py::Bool({ let t5062 = Py::Int(700i64); if py::le(&t5061, &t5062)? { true } else { false } }) }, Py::str("the projection year is too far from the censuses for this growth rate"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_sat"), v_Ps.clone()), (Py::str("m"), v_m.clone()), (Py::str("k"), v_k.clone()), (Py::str("P_t"), py::truediv(&v_Ps.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_m.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_t.clone(), &v_t0.clone())?)?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_population_projection(mut v_t1: Py, mut v_P1: Py, mut v_t2: Py, mut v_P2: Py, mut v_t: Py) -> R {
    let mut v_Pa: Py = Py::None;
    let mut v_ka: Py = Py::None;
    let mut v_kg: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t1.clone(), v_P1.clone(), v_t2.clone(), v_P2.clone(), v_t.clone()]))?;
    let _ = f__need({ let t5104 = v_t2.clone(); Py::Bool({ let t5105 = v_t1.clone(); if py::gt(&t5104, &t5105)? { true } else { false } }) }, Py::str("the second census must be later than the first"))?;
    let _ = f__need({ let t5110 = { let t5106 = py::sub(&v_t2.clone(), &v_t1.clone())?; Py::Bool({ let t5107 = Py::Int(200i64); if py::le(&t5106, &t5107)? { true } else { false } }) }; if !t5110.truthy() { t5110 } else { { let t5108 = py::abs(&py::sub(&v_t.clone(), &v_t2.clone())?)?; Py::Bool({ let t5109 = Py::Int(200i64); if py::le(&t5108, &t5109)? { true } else { false } }) } } }, Py::str("census interval and projection span must each be at most 200 years"))?;
    let _ = f__pos(Py::str("the census populations"), Py::tuple(vec![v_P1.clone(), v_P2.clone()]))?;
    v_ka = py::truediv(&py::sub(&v_P2.clone(), &v_P1.clone())?, &py::sub(&v_t2.clone(), &v_t1.clone())?)?;
    v_Pa = py::add(&v_P2.clone(), &py::mul(&v_ka.clone(), &py::sub(&v_t.clone(), &v_t2.clone())?)?)?;
    let _ = f__need({ let t5117 = v_Pa.clone(); Py::Bool({ let t5118 = Py::Int(0i64); if py::gt(&t5117, &t5118)? { true } else { false } }) }, Py::str("the arithmetic projection falls to zero or below: the decline is too fast for this method"))?;
    v_kg = py::truediv(&py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?, &py::sub(&v_t2.clone(), &v_t1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k_a"), v_ka.clone()), (Py::str("P_arith"), v_Pa.clone()), (Py::str("k_g"), v_kg.clone()), (Py::str("r_g"), py::math1("expm1", &v_kg.clone())?), (Py::str("P_geom"), py::mul(&v_P2.clone(), &py::math1("exp", &py::mul(&v_kg.clone(), &py::sub(&v_t.clone(), &v_t2.clone())?)?)?)?)]))?);
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
fn f_pump_power(mut v_Q: Py, mut v_H: Py, mut v_eta: Py, mut v_rho: Py, mut v_hours: Py, mut v_price: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Pw: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the flow Q and the head H"), Py::tuple(vec![v_Q.clone(), v_H.clone()]))?;
    let _ = f__frac(Py::str("the pump efficiency eta"), v_eta.clone(), Py::Bool(true), Py::Bool(false))?;
    v_r = (if ({ let t5219 = v_rho.clone(); Py::Bool({ let t5220 = Py::None; if py::is(&t5219, &t5220) { true } else { false } }) }).truthy() { c_RHO_WATER()? } else { v_rho.clone() });
    let _ = f__pos(Py::str("the density rho"), Py::tuple(vec![v_r.clone()]))?;
    v_Pw = py::mul(&py::mul(&py::mul(&v_r.clone(), &c_G0()?)?, &v_Q.clone())?, &v_H.clone())?;
    v_P = py::truediv(&v_Pw.clone(), &v_eta.clone())?;
    v_out = Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("P_kW"), py::truediv(&v_P.clone(), &Py::Float(1000.0))?), (Py::str("P_water"), v_Pw.clone()), (Py::str("P_water_kW"), py::truediv(&v_Pw.clone(), &Py::Float(1000.0))?)]);
    if ({ let t5221 = v_price.clone(); Py::Bool({ let t5222 = Py::None; if !py::is(&t5221, &t5222) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5223 = v_hours.clone(); Py::Bool({ let t5224 = Py::None; if !py::is(&t5223, &t5224) { true } else { false } }) }, Py::str("give the operating hours with the electricity price"))?;
    }
    if ({ let t5225 = v_hours.clone(); Py::Bool({ let t5226 = Py::None; if !py::is(&t5225, &t5226) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the operating time"), Py::tuple(vec![v_hours.clone()]))?;
        v_E = py::truediv(&py::mul(&py::truediv(&v_P.clone(), &Py::Float(1000.0))?, &v_hours.clone())?, &c_HOUR()?)?;
        py::setitem(&v_out.clone(), &Py::str("energy_kWh"), v_E.clone())?;
        if ({ let t5227 = v_price.clone(); Py::Bool({ let t5228 = Py::None; if !py::is(&t5227, &t5228) { true } else { false } }) }).truthy() {
            let _ = f__nonneg(Py::str("the electricity price"), Py::tuple(vec![v_price.clone()]))?;
            py::setitem(&v_out.clone(), &Py::str("cost"), py::mul(&v_E.clone(), &v_price.clone())?)?;
        }
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_method(mut v_C: Py, mut v_i: Py, mut v_A: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__frac(Py::str("the runoff coefficient C"), v_C.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__pos(Py::str("the rainfall intensity i and the area A"), Py::tuple(vec![v_i.clone(), v_A.clone()]))?;
    v_Q = py::mul(&py::mul(&v_C.clone(), &v_i.clone())?, &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("Q_L_per_s"), py::mul(&v_Q.clone(), &Py::Float(1000.0))?), (Py::str("Q_m3_per_h"), py::mul(&v_Q.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaeration(mut v_method: Py, mut v_u: Py, mut v_H: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_kday: Py = Py::None;
    let _ = f__need({ let t5380 = v_method.clone(); Py::Bool({ let t5381 = c__REAER()?; if py::contains(&t5381, &t5380)? { true } else { false } }) }, Py::str("method: o_connor_dobbins, churchill or owens_gibbs"))?;
    let _ = f__pos(Py::str("u and H"), Py::tuple(vec![v_u.clone(), v_H.clone()]))?;
    let t5388 = (py::getitem(&c__REAER()?, &v_method.clone())?).items()?;
    if t5388.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5388[0].clone();
    v_a = t5388[1].clone();
    v_b = t5388[2].clone();
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
fn f_retardation(mut v_rho_b: Py, mut v_n: Py, mut v_Kd: Py, mut v_Koc: Py, mut v_foc: Py, mut v_R: Py, mut v_v: Py) -> R {
    let mut v_kd: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_out = Py::dict(vec![]);
    if ({ let t5427 = v_R.clone(); Py::Bool({ let t5428 = Py::None; if !py::is(&t5427, &t5428) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5442 = { let t5429 = v_rho_b.clone(); Py::Bool({ let t5430 = Py::None; if py::is(&t5429, &t5430) { true } else { false } }) }; if !t5442.truthy() { t5442 } else { { let t5441 = { let t5431 = v_n.clone(); Py::Bool({ let t5432 = Py::None; if py::is(&t5431, &t5432) { true } else { false } }) }; if !t5441.truthy() { t5441 } else { { let t5440 = { let t5433 = v_Kd.clone(); Py::Bool({ let t5434 = Py::None; if py::is(&t5433, &t5434) { true } else { false } }) }; if !t5440.truthy() { t5440 } else { { let t5439 = { let t5435 = v_Koc.clone(); Py::Bool({ let t5436 = Py::None; if py::is(&t5435, &t5436) { true } else { false } }) }; if !t5439.truthy() { t5439 } else { { let t5437 = v_foc.clone(); Py::Bool({ let t5438 = Py::None; if py::is(&t5437, &t5438) { true } else { false } }) } } } } } } } } }, Py::str("give the retardation factor R directly or the soil properties to compute it, not both"))?;
        let _ = f__need({ let t5443 = v_v.clone(); Py::Bool({ let t5444 = Py::None; if !py::is(&t5443, &t5444) { true } else { false } }) }, Py::str("with R given, give the groundwater velocity v"))?;
        let _ = f__fin(Py::tuple(vec![v_R.clone()]))?;
        let _ = f__need({ let t5448 = v_R.clone(); Py::Bool({ let t5449 = Py::Float(1.0); if py::ge(&t5448, &t5449)? { true } else { false } }) }, Py::str("the retardation factor R must be at least 1"))?;
    } else {
        let _ = f__need({ let t5454 = { let t5450 = v_rho_b.clone(); Py::Bool({ let t5451 = Py::None; if !py::is(&t5450, &t5451) { true } else { false } }) }; if !t5454.truthy() { t5454 } else { { let t5452 = v_n.clone(); Py::Bool({ let t5453 = Py::None; if !py::is(&t5452, &t5453) { true } else { false } }) } } }, Py::str("give the bulk density rho_b and the porosity n (or R directly)"))?;
        let _ = f__pos(Py::str("the bulk density rho_b"), Py::tuple(vec![v_rho_b.clone()]))?;
        let _ = f__frac(Py::str("the porosity n"), v_n.clone(), Py::Bool(true), Py::Bool(true))?;
        if ({ let t5470 = v_Kd.clone(); Py::Bool({ let t5471 = Py::None; if !py::is(&t5470, &t5471) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t5476 = { let t5472 = v_Koc.clone(); Py::Bool({ let t5473 = Py::None; if py::is(&t5472, &t5473) { true } else { false } }) }; if !t5476.truthy() { t5476 } else { { let t5474 = v_foc.clone(); Py::Bool({ let t5475 = Py::None; if py::is(&t5474, &t5475) { true } else { false } }) } } }, Py::str("give Kd, or Koc with foc, not both"))?;
            let _ = f__nonneg(Py::str("Kd"), Py::tuple(vec![v_Kd.clone()]))?;
            v_kd = v_Kd.clone();
        } else {
            let _ = f__need({ let t5487 = { let t5483 = v_Koc.clone(); Py::Bool({ let t5484 = Py::None; if !py::is(&t5483, &t5484) { true } else { false } }) }; if !t5487.truthy() { t5487 } else { { let t5485 = v_foc.clone(); Py::Bool({ let t5486 = Py::None; if !py::is(&t5485, &t5486) { true } else { false } }) } } }, Py::str("give the distribution coefficient Kd, or Koc and foc"))?;
            let _ = f__nonneg(Py::str("Koc"), Py::tuple(vec![v_Koc.clone()]))?;
            let _ = f__frac(Py::str("the organic carbon fraction foc"), v_foc.clone(), Py::Bool(false), Py::Bool(false))?;
            v_kd = py::mul(&v_Koc.clone(), &v_foc.clone())?;
            py::setitem(&v_out.clone(), &Py::str("Kd"), v_kd.clone())?;
            py::setitem(&v_out.clone(), &Py::str("Kd_L_per_kg"), py::mul(&v_kd.clone(), &Py::Float(1000.0))?)?;
        }
        v_R = py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_rho_b.clone(), &v_kd.clone())?, &v_n.clone())?)?;
    }
    py::setitem(&v_out.clone(), &Py::str("R"), v_R.clone())?;
    if ({ let t5488 = v_v.clone(); Py::Bool({ let t5489 = Py::None; if !py::is(&t5488, &t5489) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the groundwater velocity v"), Py::tuple(vec![v_v.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("vc"), py::truediv(&v_v.clone(), &v_R.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("vc_m_per_day"), py::mul(&py::truediv(&v_v.clone(), &v_R.clone())?, &c_DAY()?)?)?;
        py::setitem(&v_out.clone(), &Py::str("v_m_per_day"), py::mul(&v_v.clone(), &c_DAY()?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_return_period(mut v_T: Py, mut v_n: Py, mut v_risk: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_none: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    if ({ let t5529 = v_n.clone(); Py::Bool({ let t5530 = Py::None; if !py::is(&t5529, &t5530) { true } else { false } }) }).truthy() {
        let _ = f__fin(Py::tuple(vec![v_n.clone()]))?;
        let _ = f__need({ let t5541 = { let t5534 = v_n.clone(); Py::Bool({ let t5535 = Py::Int(1i64); if py::ge(&t5534, &t5535)? { true } else { false } }) }; if !t5541.truthy() { t5541 } else { { let t5540 = { let t5536 = v_n.clone(); Py::Bool({ let t5537 = py::int(&v_n.clone())?; if py::eq(&t5536, &t5537) { true } else { false } }) }; if !t5540.truthy() { t5540 } else { { let t5538 = v_n.clone(); Py::Bool({ let t5539 = Py::Int(10000i64); if py::le(&t5538, &t5539)? { true } else { false } }) } } } } }, Py::str("the number of years n must be a whole number from 1 to 10000"))?;
    }
    if ({ let t5542 = v_T.clone(); Py::Bool({ let t5543 = Py::None; if !py::is(&t5542, &t5543) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5544 = v_risk.clone(); Py::Bool({ let t5545 = Py::None; if py::is(&t5544, &t5545) { true } else { false } }) }, Py::str("give the return period T or an acceptable risk, not both"))?;
        let _ = f__fin(Py::tuple(vec![v_T.clone()]))?;
        let _ = f__need({ let t5546 = v_T.clone(); Py::Bool({ let t5547 = Py::Float(1.0); if py::ge(&t5546, &t5547)? { true } else { false } }) }, Py::str("the return period T must be at least 1 year"))?;
        v_p = py::truediv(&Py::Float(1.0), &v_T.clone())?;
    } else {
        let _ = f__need({ let t5552 = { let t5548 = v_risk.clone(); Py::Bool({ let t5549 = Py::None; if !py::is(&t5548, &t5549) { true } else { false } }) }; if !t5552.truthy() { t5552 } else { { let t5550 = v_n.clone(); Py::Bool({ let t5551 = Py::None; if !py::is(&t5550, &t5551) { true } else { false } }) } } }, Py::str("give the return period T, or an acceptable risk with the design life n"))?;
        let _ = f__fin(Py::tuple(vec![v_risk.clone()]))?;
        let _ = f__need({ let t5553 = Py::Float(0.0); Py::Bool({ let t5554 = v_risk.clone(); if py::lt(&t5553, &t5554)? { { let t5555 = Py::Float(1.0); if py::lt(&t5554, &t5555)? { true } else { false } } } else { false } }) }, Py::str("the risk must be a probability between 0 and 1"))?;
        v_p = py::neg(&py::math1("expm1", &py::truediv(&py::math1("log1p", &py::neg(&v_risk.clone())?)?, &v_n.clone())?)?)?;
        v_T = py::truediv(&Py::Float(1.0), &v_p.clone())?;
    }
    v_out = Py::dict(vec![(Py::str("p"), v_p.clone()), (Py::str("p_percent"), py::mul(&v_p.clone(), &Py::Float(100.0))?), (Py::str("T"), v_T.clone())]);
    if ({ let t5556 = v_n.clone(); Py::Bool({ let t5557 = Py::None; if !py::is(&t5556, &t5557) { true } else { false } }) }).truthy() {
        v_none = (if ({ let t5558 = v_p.clone(); Py::Bool({ let t5559 = Py::Float(1.0); if py::lt(&t5558, &t5559)? { true } else { false } }) }).truthy() { py::math1("exp", &py::mul(&v_n.clone(), &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)? } else { Py::Float(0.0) });
        v_P = (if ({ let t5560 = v_p.clone(); Py::Bool({ let t5561 = Py::Float(1.0); if py::lt(&t5560, &t5561)? { true } else { false } }) }).truthy() { py::neg(&py::math1("expm1", &py::mul(&v_n.clone(), &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)? } else { Py::Float(1.0) });
        py::setitem(&v_out.clone(), &Py::str("P_n"), v_P.clone())?;
        py::setitem(&v_out.clone(), &Py::str("P_n_percent"), py::mul(&v_P.clone(), &Py::Float(100.0))?)?;
        py::setitem(&v_out.clone(), &Py::str("P_none"), v_none.clone())?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_return_ratio(mut v_X: Py, mut v_Xr: Py, mut v_Q: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("X, Xr and Q"), Py::tuple(vec![v_X.clone(), v_Xr.clone(), v_Q.clone()]))?;
    let _ = f__need({ let t5607 = v_Xr.clone(); Py::Bool({ let t5608 = v_X.clone(); if py::gt(&t5607, &t5608)? { true } else { false } }) }, Py::str("the return sludge must be more concentrated than the mixed liquor"))?;
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
fn f_series_efficiency(mut v_eta: Py, mut v_C_in: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pen: Py = Py::None;
    let _ = f__need({ let t1215 = py::len(&v_eta.clone())?; Py::Bool({ let t1216 = Py::Int(1i64); if py::ge(&t1215, &t1216)? { true } else { false } }) }, Py::str("give the efficiency of each device"))?;
    v_pen = Py::Float(1.0);
    for t1217 in py::iter(&(v_eta.clone()))? {
        v_e = t1217;
        let _ = f__fin(Py::tuple(vec![v_e.clone()]))?;
        let _ = f__need({ let t1221 = Py::Float(0.0); Py::Bool({ let t1222 = v_e.clone(); if py::le(&t1221, &t1222)? { { let t1223 = Py::Float(1.0); if py::le(&t1222, &t1223)? { true } else { false } } } else { false } }) }, Py::str("each efficiency must be a fraction from 0 to 1 (write 0.9 for 90%)"))?;
        v_pen = py::mul(&v_pen.clone(), &py::sub(&Py::Float(1.0), &v_e.clone())?)?;
    }
    v_out = Py::dict(vec![(Py::str("eta_total"), py::sub(&Py::Float(1.0), &v_pen.clone())?), (Py::str("eta_total_percent"), py::mul(&py::sub(&Py::Float(1.0), &v_pen.clone())?, &Py::Float(100.0))?), (Py::str("penetration"), v_pen.clone())]);
    if ({ let t1224 = v_C_in.clone(); Py::Bool({ let t1225 = Py::None; if !py::is(&t1224, &t1225) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the inlet concentration C_in"), Py::tuple(vec![v_C_in.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("C_out"), py::mul(&v_C_in.clone(), &v_pen.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("C_out_mgm3"), py::mul(&py::mul(&v_C_in.clone(), &v_pen.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_settling_chamber(mut v_dp: Py, mut v_rho_p: Py, mut v_rho_g: Py, mut v_mu: Py, mut v_L: Py, mut v_W: Py, mut v_Q: Py, mut v_N: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_vt: Py = Py::None;
    let _ = f__pos(Py::str("dp, rho_p, rho_g, mu, L, W and Q"), Py::tuple(vec![v_dp.clone(), v_rho_p.clone(), v_rho_g.clone(), v_mu.clone(), v_L.clone(), v_W.clone(), v_Q.clone()]))?;
    let _ = f__need({ let t5747 = v_rho_p.clone(); Py::Bool({ let t5748 = v_rho_g.clone(); if py::gt(&t5747, &t5748)? { true } else { false } }) }, Py::str("the particles must be denser than the gas"))?;
    let _ = f__need({ let t5751 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5751.truthy() { t5751 } else { { let t5749 = v_N.clone(); Py::Bool({ let t5750 = Py::Int(1i64); if py::ge(&t5749, &t5750)? { true } else { false } }) } } }, Py::str("the number of trays N must be a whole number of at least 1"))?;
    v_vt = f__stokes_v(v_dp.clone(), v_rho_p.clone(), v_rho_g.clone(), v_mu.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho_g.clone(), &v_vt.clone())?, &v_dp.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t5752 = v_Re.clone(); Py::Bool({ let t5753 = Py::Float(1.0); if py::lt(&t5752, &t5753)? { true } else { false } }) }, Py::str("Stokes' law needs a particle Reynolds number below 1"))?;
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
    let _ = f__need({ let t5804 = v_S.clone(); Py::Bool({ let t5805 = v_S0.clone(); if py::lt(&t5804, &t5805)? { true } else { false } }) }, Py::str("the effluent substrate must be below the influent"))?;
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
    if ({ let t6402 = py::sub(&py::mul(&v_kd.clone(), &v_L0.clone())?, &py::mul(&v_ka.clone(), &v_D0.clone())?)?; Py::Bool({ let t6403 = Py::Int(0i64); if py::le(&t6402, &t6403)? { true } else { false } }) }).truthy() {
        v_tc = Py::Float(0.0);
    } else {
        if ({ let t6404 = py::abs(&py::sub(&v_ka.clone(), &v_kd.clone())?)?; Py::Bool({ let t6405 = py::mul(&Py::Float(1e-09), &v_ka.clone())?; if py::le(&t6404, &t6405)? { true } else { false } }) }).truthy() {
            v_tc = py::truediv(&py::sub(&Py::Float(1.0), &py::truediv(&v_D0.clone(), &v_L0.clone())?)?, &v_kd.clone())?;
        } else {
            v_tc = py::truediv(&py::math1("log", &py::mul(&py::truediv(&v_ka.clone(), &v_kd.clone())?, &py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&v_D0.clone(), &py::sub(&v_ka.clone(), &v_kd.clone())?)?, &py::mul(&v_kd.clone(), &v_L0.clone())?)?)?)?)?, &py::sub(&v_ka.clone(), &v_kd.clone())?)?;
        }
    }
    v_Dc = (if ({ let t6406 = v_tc.clone(); Py::Bool({ let t6407 = Py::Float(0.0); if py::eq(&t6406, &t6407) { true } else { false } }) }).truthy() { v_D0.clone() } else { py::mul(&py::mul(&py::truediv(&v_kd.clone(), &v_ka.clone())?, &v_L0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_kd.clone())?, &v_tc.clone())?)?)? });
    let _ = f__need({ let t6408 = v_Dc.clone(); Py::Bool({ let t6409 = v_DOsat.clone(); if py::le(&t6408, &t6409)? { true } else { false } }) }, Py::str("the critical deficit exceeds saturation: the river turns anaerobic and the model no longer holds"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_c"), v_tc.clone()), (Py::str("t_c_days"), py::truediv(&v_tc.clone(), &c_DAY()?)?), (Py::str("D_c"), v_Dc.clone()), (Py::str("DO_min"), py::sub(&v_DOsat.clone(), &v_Dc.clone())?), (Py::str("DO_min_mgL"), py::mul(&py::sub(&v_DOsat.clone(), &v_Dc.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_srt(mut v_V: Py, mut v_X: Py, mut v_Qw: Py, mut v_Xw: Py, mut v_Qe: Py, mut v_Xe: Py) -> R {
    let mut v_qe: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_xe: Py = Py::None;
    let _ = f__pos(Py::str("V, X, Qw and Xw"), Py::tuple(vec![v_V.clone(), v_X.clone(), v_Qw.clone(), v_Xw.clone()]))?;
    v_xe = (if ({ let t6113 = v_Xe.clone(); Py::Bool({ let t6114 = Py::None; if py::is(&t6113, &t6114) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_Xe.clone() });
    let _ = f__nonneg(Py::str("Xe"), Py::tuple(vec![v_xe.clone()]))?;
    if ({ let t6121 = v_Qe.clone(); Py::Bool({ let t6122 = Py::None; if py::is(&t6121, &t6122) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6123 = v_xe.clone(); Py::Bool({ let t6124 = Py::Int(0i64); if py::eq(&t6123, &t6124) { true } else { false } }) }, Py::str("give the effluent flow Qe with the effluent solids Xe"))?;
        v_qe = Py::Float(0.0);
    } else {
        v_qe = v_Qe.clone();
        let _ = f__nonneg(Py::str("Qe"), Py::tuple(vec![v_qe.clone()]))?;
    }
    v_th = py::truediv(&py::mul(&v_V.clone(), &v_X.clone())?, &py::add(&py::mul(&v_Qw.clone(), &v_Xw.clone())?, &py::mul(&v_qe.clone(), &v_xe.clone())?)?)?;
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
    let _ = f__need({ let t6218 = v_condition.clone(); Py::Bool({ let t6219 = c__STAB_ALL()?; if py::contains(&t6219, &t6218)? { true } else { false } }) }, Py::str("condition: strong, moderate or slight insolation, overcast, night_cloudy or night_clear"))?;
    if ({ let t6220 = v_condition.clone(); Py::Bool({ let t6221 = Py::str("overcast"); if py::eq(&t6220, &t6221) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("stability"), Py::str("D"))]))?);
    }
    if ({ let t6234 = v_u.clone(); Py::Bool({ let t6235 = Py::Float(2.0); if py::lt(&t6234, &t6235)? { true } else { false } }) }).truthy() {
        v_row = Py::tuple(vec![Py::str("A"), Py::str("A-B"), Py::str("B"), Py::str(""), Py::str("")]);
    } else {
        if ({ let t6236 = v_u.clone(); Py::Bool({ let t6237 = Py::Float(3.0); if py::lt(&t6236, &t6237)? { true } else { false } }) }).truthy() {
            v_row = Py::tuple(vec![Py::str("A-B"), Py::str("B"), Py::str("C"), Py::str("E"), Py::str("F")]);
        } else {
            if ({ let t6238 = v_u.clone(); Py::Bool({ let t6239 = Py::Float(5.0); if py::lt(&t6238, &t6239)? { true } else { false } }) }).truthy() {
                v_row = Py::tuple(vec![Py::str("B"), Py::str("B-C"), Py::str("C"), Py::str("D"), Py::str("E")]);
            } else {
                if ({ let t6240 = v_u.clone(); Py::Bool({ let t6241 = Py::Float(6.0); if py::lt(&t6240, &t6241)? { true } else { false } }) }).truthy() {
                    v_row = Py::tuple(vec![Py::str("C"), Py::str("C-D"), Py::str("D"), Py::str("D"), Py::str("D")]);
                } else {
                    v_row = Py::tuple(vec![Py::str("C"), Py::str("D"), Py::str("D"), Py::str("D"), Py::str("D")]);
                }
            }
        }
    }
    v_col = py::getitem(&Py::dict(vec![(Py::str("strong"), Py::Int(0i64)), (Py::str("moderate"), Py::Int(1i64)), (Py::str("slight"), Py::Int(2i64)), (Py::str("night_cloudy"), Py::Int(3i64)), (Py::str("night_clear"), Py::Int(4i64))]), &v_condition.clone())?;
    v_s = py::getitem(&v_row.clone(), &v_col.clone())?;
    let _ = f__need({ let t6242 = v_s.clone(); Py::Bool({ let t6243 = Py::str(""); if py::ne(&t6242, &t6243) { true } else { false } }) }, Py::str("the table gives no class for a night with a surface wind below 2 m/s"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("stability"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stokes(mut v_d: Py, mut v_rho_p: Py, mut v_rho_w: Py, mut v_mu: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__pos(Py::str("d, rho_p, rho_w and mu"), Py::tuple(vec![v_d.clone(), v_rho_p.clone(), v_rho_w.clone(), v_mu.clone()]))?;
    let _ = f__need({ let t6346 = v_rho_p.clone(); Py::Bool({ let t6347 = v_rho_w.clone(); if py::gt(&t6346, &t6347)? { true } else { false } }) }, Py::str("the particle must be denser than the water"))?;
    v_vs = f__stokes_v(v_d.clone(), v_rho_p.clone(), v_rho_w.clone(), v_mu.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho_w.clone(), &v_vs.clone())?, &v_d.clone())?, &v_mu.clone())?;
    let _ = f__need({ let t6348 = v_Re.clone(); Py::Bool({ let t6349 = Py::Float(1.0); if py::le(&t6348, &t6349)? { true } else { false } }) }, Py::str("Stokes' law needs a particle Reynolds number of at most 1 (use the general terminal velocity)"))?;
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
    let _ = f__need({ let t6464 = v_D.clone(); Py::Bool({ let t6465 = v_DOsat.clone(); if py::le(&t6464, &t6465)? { true } else { false } }) }, Py::str("the computed deficit exceeds saturation: the river turns anaerobic and the model no longer holds"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("DO"), py::sub(&v_DOsat.clone(), &v_D.clone())?), (Py::str("D_mgL"), py::mul(&v_D.clone(), &Py::Float(1000.0))?), (Py::str("DO_mgL"), py::mul(&py::sub(&v_DOsat.clone(), &v_D.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_svi(mut v_V30: Py, mut v_MLSS: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("V30 and MLSS"), Py::tuple(vec![v_V30.clone(), v_MLSS.clone()]))?;
    let _ = f__need({ let t5964 = v_V30.clone(); Py::Bool({ let t5965 = Py::Float(1000.0); if py::le(&t5964, &t5965)? { true } else { false } }) }, Py::str("the settled volume cannot exceed 1000 mL/L"))?;
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
    let _ = f__need({ let t6510 = v_rho_p.clone(); Py::Bool({ let t6511 = v_rho_f.clone(); if py::gt(&t6510, &t6511)? { true } else { false } }) }, Py::str("the particle must be denser than the fluid"))?;
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
    let _ = f__need({ let t6538 = v_Re.clone(); Py::Bool({ let t6539 = Py::Float(200000.0); if py::le(&t6538, &t6539)? { true } else { false } }) }, Py::str("the drag law holds up to a Reynolds number of 2e5"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("vt"), v_v.clone()), (Py::str("Re"), v_Re.clone()), (Py::str("Cd"), f__rouse_cd(v_Re.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thickening(mut v_V1: Py, mut v_P1: Py, mut v_P2: Py) -> R {
    let _ = f__pos(Py::str("V1"), Py::tuple(vec![v_V1.clone()]))?;
    let _ = f__frac(Py::str("P1"), v_P1.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("P2"), v_P2.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t5862 = v_P2.clone(); Py::Bool({ let t5863 = v_P1.clone(); if py::gt(&t5862, &t5863)? { true } else { false } }) }, Py::str("thickening must raise the solids fraction (P2 > P1)"))?;
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
    let t6687 = (f__formula_counts(v_formula.clone())?).items()?;
    if t6687.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6687[0].clone();
    v_b = t6687[1].clone();
    v_c = t6687[2].clone();
    v_d = t6687[3].clone();
    let _ = f__need({ let t6688 = v_nitrification.clone(); Py::Bool({ let t6689 = Py::tuple(vec![Py::str("yes"), Py::str("no")]); if py::contains(&t6689, &t6688)? { true } else { false } }) }, Py::str("nitrification: yes or no"))?;
    let _ = f__need({ let t6690 = v_b.clone(); Py::Bool({ let t6691 = py::mul(&Py::Int(3i64), &v_d.clone())?; if py::ge(&t6690, &t6691)? { true } else { false } }) }, Py::str("too little hydrogen to form ammonia from the nitrogen"))?;
    v_n = py::add(&py::sub(&py::sub(&py::add(&v_a.clone(), &py::truediv(&v_b.clone(), &Py::Float(4.0))?)?, &py::truediv(&v_c.clone(), &Py::Float(2.0))?)?, &py::mul(&Py::Float(0.75), &v_d.clone())?)?, &(if ({ let t6692 = v_nitrification.clone(); Py::Bool({ let t6693 = Py::str("yes"); if py::eq(&t6692, &t6693) { true } else { false } }) }).truthy() { py::mul(&Py::Float(2.0), &v_d.clone())? } else { Py::Float(0.0) }))?;
    let _ = f__need({ let t6694 = v_n.clone(); Py::Bool({ let t6695 = Py::Int(0i64); if py::gt(&t6694, &t6695)? { true } else { false } }) }, Py::str("the compound needs no oxygen to be oxidised"))?;
    v_M = py::add(&py::add(&py::add(&py::mul(&v_a.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_b.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?, &py::mul(&v_c.clone(), &py::getitem(&c_AW()?, &Py::str("O"))?)?)?, &py::mul(&v_d.clone(), &py::getitem(&c_AW()?, &Py::str("N"))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("thod"), py::truediv(&py::mul(&v_n.clone(), &c_MW_O2()?)?, &v_M.clone())?), (Py::str("o2_per_mol"), v_n.clone()), (Py::str("molar_mass"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thod_solution(mut v_formula: Py, mut v_C: Py, mut v_nitrification: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_nit: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_nit = (if ({ let t6652 = v_nitrification.clone(); Py::Bool({ let t6653 = Py::None; if py::is(&t6652, &t6653) { true } else { false } }) }).truthy() { Py::str("no") } else { v_nitrification.clone() });
    let _ = f__pos(Py::str("the concentration C"), Py::tuple(vec![v_C.clone()]))?;
    v_r = f_thod(v_formula.clone(), v_nit.clone())?;
    v_g = py::getitem(&v_r.clone(), &Py::str("thod"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ThOD"), py::mul(&v_g.clone(), &v_C.clone())?), (Py::str("ThOD_mgL"), py::mul(&py::mul(&v_g.clone(), &v_C.clone())?, &Py::Float(1000.0))?), (Py::str("thod_ratio"), v_g.clone()), (Py::str("o2_per_mol"), py::getitem(&v_r.clone(), &Py::str("o2_per_mol"))?), (Py::str("molar_mass"), py::getitem(&v_r.clone(), &Py::str("molar_mass"))?), (Py::str("C_molL"), py::truediv(&v_C.clone(), &py::getitem(&v_r.clone(), &Py::str("molar_mass"))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transmembrane_pressure(mut v_P_feed: Py, mut v_P_conc: Py, mut v_P_perm: Py) -> R {
    let mut v_tmp: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_P_feed.clone(), v_P_conc.clone(), v_P_perm.clone()]))?;
    let _ = f__need({ let t6828 = v_P_conc.clone(); Py::Bool({ let t6829 = v_P_feed.clone(); if py::le(&t6828, &t6829)? { true } else { false } }) }, Py::str("the concentrate pressure cannot exceed the feed pressure"))?;
    v_tmp = py::sub(&py::truediv(&py::add(&v_P_feed.clone(), &v_P_conc.clone())?, &Py::Float(2.0))?, &v_P_perm.clone())?;
    let _ = f__need({ let t6830 = v_tmp.clone(); Py::Bool({ let t6831 = Py::Int(0i64); if py::gt(&t6830, &t6831)? { true } else { false } }) }, Py::str("the transmembrane pressure must be positive (the permeate pressure is too high)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("TMP"), v_tmp.clone()), (Py::str("TMP_kPa"), py::truediv(&v_tmp.clone(), &Py::Float(1000.0))?), (Py::str("TMP_bar"), py::truediv(&v_tmp.clone(), &Py::Float(100000.0))?), (Py::str("dP"), py::sub(&v_P_feed.clone(), &v_P_conc.clone())?), (Py::str("dP_kPa"), py::truediv(&py::sub(&v_P_feed.clone(), &v_P_conc.clone())?, &Py::Float(1000.0))?)]))?);
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
    if ({ let t6970 = v_t.clone(); Py::Bool({ let t6971 = Py::None; if py::is(&t6970, &t6971) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone())]))?);
    }
    let _ = f__pos(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("Gt"), py::mul(&v_G.clone(), &v_t.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vs_reduction(mut v_VS_in: Py, mut v_VS_out: Py) -> R {
    let mut v_vsr: Py = Py::None;
    let _ = f__frac(Py::str("the volatile fraction of the feed VS_in"), v_VS_in.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("the volatile fraction of the digested solids VS_out"), v_VS_out.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t7022 = v_VS_out.clone(); Py::Bool({ let t7023 = v_VS_in.clone(); if py::le(&t7022, &t7023)? { true } else { false } }) }, Py::str("the digested solids cannot be more volatile than the feed"))?;
    v_vsr = py::truediv(&py::sub(&v_VS_in.clone(), &v_VS_out.clone())?, &py::sub(&v_VS_in.clone(), &py::mul(&v_VS_in.clone(), &v_VS_out.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("VSR"), v_vsr.clone()), (Py::str("VSR_percent"), py::mul(&v_vsr.clone(), &Py::Float(100.0))?), (Py::str("TS_remaining"), py::truediv(&py::sub(&Py::Float(1.0), &v_VS_in.clone())?, &py::sub(&Py::Float(1.0), &v_VS_out.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_waste_generation(mut v_pop: Py, mut v_rate: Py, mut v_rho: Py, mut v_Vt: Py) -> R {
    let mut v_m_day: Py = Py::None;
    let mut v_vol_day: Py = Py::None;
    let _ = f__need({ let t6062 = Py::Bool(matches!(v_pop.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6062.truthy() { t6062 } else { { let t6060 = v_pop.clone(); Py::Bool({ let t6061 = Py::Int(1i64); if py::ge(&t6060, &t6061)? { true } else { false } }) } } }, Py::str("the population must be a whole number of at least 1"))?;
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
    let _ = f__need({ let t7066 = { let t7062 = py::len(&v_m.clone())?; Py::Bool({ let t7063 = py::len(&v_w.clone())?; if py::eq(&t7062, &t7063) { true } else { false } }) }; if !t7066.truthy() { t7066 } else { { let t7064 = py::len(&v_m.clone())?; Py::Bool({ let t7065 = Py::Int(1i64); if py::ge(&t7064, &t7065)? { true } else { false } }) } } }, Py::str("give one moisture fraction per component mass"))?;
    for t7067 in py::iter(&(v_m.clone()))? {
        v_v = t7067;
        let _ = f__need({ let t7070 = py::math1("isfinite", &v_v.clone())?; if !t7070.truthy() { t7070 } else { { let t7068 = v_v.clone(); Py::Bool({ let t7069 = Py::Int(0i64); if py::ge(&t7068, &t7069)? { true } else { false } }) } } }, Py::str("every mass must be zero or positive"))?;
    }
    for t7071 in py::iter(&(v_w.clone()))? {
        v_v = t7071;
        let _ = f__need({ let t7075 = py::math1("isfinite", &v_v.clone())?; if !t7075.truthy() { t7075 } else { { let t7072 = Py::Int(0i64); Py::Bool({ let t7073 = v_v.clone(); if py::le(&t7072, &t7073)? { { let t7074 = Py::Int(1i64); if py::lt(&t7073, &t7074)? { true } else { false } } } else { false } }) } } }, Py::str("every moisture fraction must lie between 0 and 1"))?;
    }
    v_tot = py::sum(&v_m.clone(), None)?;
    let _ = f__need({ let t7076 = v_tot.clone(); Py::Bool({ let t7077 = Py::Int(0i64); if py::gt(&t7076, &t7077)? { true } else { false } }) }, Py::str("the total mass must be positive"))?;
    v_water = py::sum(&{ let mut out = vec![]; for t7078 in py::iter(&(py::zip(&[v_m.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t7079 = (t7078).items()?;
if t7079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t7079[0].clone();
v_b = t7079[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_dry = py::sub(&v_tot.clone(), &v_water.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("moisture_wet"), py::truediv(&v_water.clone(), &v_tot.clone())?), (Py::str("dry_mass"), v_dry.clone()), (Py::str("moisture_dry"), py::truediv(&v_water.clone(), &v_dry.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_water_demand(mut v_pop: Py, mut v_q: Py, mut v_f_md: Py, mut v_f_ph: Py, mut v_r_ww: Py) -> R {
    let mut v_Qd: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pf: Py = Py::None;
    let _ = f__need({ let t7121 = Py::Bool(matches!(v_pop.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t7121.truthy() { t7121 } else { { let t7120 = py::not(&Py::Bool(matches!(v_pop.clone(), Py::Bool(_)))); if !t7120.truthy() { t7120 } else { { let t7118 = v_pop.clone(); Py::Bool({ let t7119 = Py::Int(1i64); if py::ge(&t7118, &t7119)? { true } else { false } }) } } } } }, Py::str("the population must be a whole number of at least 1"))?;
    let _ = f__pos(Py::str("the per-capita demand"), Py::tuple(vec![v_q.clone()]))?;
    let _ = f__need({ let t7128 = Py::Int(20i64); Py::Bool({ let t7129 = v_q.clone(); if py::le(&t7128, &t7129)? { { let t7130 = Py::Int(2000i64); if py::le(&t7129, &t7130)? { true } else { false } } } else { false } }) }, Py::str("the per-capita demand must be between 20 and 2000 L per person per day"))?;
    let _ = f__fin(Py::tuple(vec![v_f_md.clone(), v_f_ph.clone()]))?;
    let _ = f__need({ let t7134 = Py::Int(1i64); Py::Bool({ let t7135 = v_f_md.clone(); if py::le(&t7134, &t7135)? { { let t7136 = Py::Int(4i64); if py::le(&t7135, &t7136)? { true } else { false } } } else { false } }) }, Py::str("the maximum-day factor must lie between 1 and 4"))?;
    let _ = f__need({ let t7137 = v_f_md.clone(); Py::Bool({ let t7138 = v_f_ph.clone(); if py::le(&t7137, &t7138)? { { let t7139 = Py::Int(8i64); if py::le(&t7138, &t7139)? { true } else { false } } } else { false } }) }, Py::str("the peak-hour factor must lie between the maximum-day factor and 8"))?;
    v_Qd = py::truediv(&py::mul(&v_pop.clone(), &v_q.clone())?, &Py::Float(1000.0))?;
    v_out = Py::dict(vec![(Py::str("Q_avg"), py::truediv(&v_Qd.clone(), &c_DAY()?)?), (Py::str("Q_avg_m3_per_day"), v_Qd.clone()), (Py::str("Q_maxday"), py::truediv(&py::mul(&v_f_md.clone(), &v_Qd.clone())?, &c_DAY()?)?), (Py::str("Q_maxday_m3_per_day"), py::mul(&v_f_md.clone(), &v_Qd.clone())?), (Py::str("Q_peakhour"), py::truediv(&py::mul(&v_f_ph.clone(), &v_Qd.clone())?, &c_DAY()?)?), (Py::str("Q_peakhour_m3_per_day"), py::mul(&v_f_ph.clone(), &v_Qd.clone())?), (Py::str("Q_peakhour_L_per_s"), py::mul(&py::truediv(&py::mul(&v_f_ph.clone(), &v_Qd.clone())?, &c_DAY()?)?, &Py::Float(1000.0))?)]);
    if ({ let t7140 = v_r_ww.clone(); Py::Bool({ let t7141 = Py::None; if !py::is(&t7140, &t7141) { true } else { false } }) }).truthy() {
        let _ = f__frac(Py::str("the wastewater return fraction"), v_r_ww.clone(), Py::Bool(true), Py::Bool(false))?;
        v_pf = py::add(&Py::Float(1.0), &py::truediv(&Py::Float(14.0), &py::add(&Py::Float(4.0), &py::math1("sqrt", &py::truediv(&v_pop.clone(), &Py::Float(1000.0))?)?)?)?)?;
        py::setitem(&v_out.clone(), &Py::str("Q_ww_avg_m3_per_day"), py::mul(&v_r_ww.clone(), &v_Qd.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("harmon_peak_factor"), v_pf.clone())?;
        py::setitem(&v_out.clone(), &Py::str("Q_ww_peak_m3_per_day"), py::mul(&py::mul(&v_pf.clone(), &v_r_ww.clone())?, &v_Qd.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
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
    let _ = f__need({ let t7290 = Py::Float(0.0); Py::Bool({ let t7291 = v_p.clone(); if py::lt(&t7290, &t7291)? { { let t7292 = Py::Float(1.0); if py::lt(&t7291, &t7292)? { true } else { false } } } else { false } }) }, Py::str("the exponent p must lie between 0 and 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("u2"), py::mul(&v_u1.clone(), &py::pow(&py::truediv(&v_z2.clone(), &v_z1.clone())?, &v_p.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1059: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1058 = v_inputs.clone(); f_chlorine_dose(py::getitem(&t1058, &Py::str("Q"))?, py::getitem(&t1058, &Py::str("demand"))?, py::getitem(&t1058, &Py::str("residual"))?, py::getitem(&t1058, &Py::str("purity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1059 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1060 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1082 = (t1060).items()?;
        if t1082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1082[0].clone();
        v_v = t1082[1].clone();
        if ({ let t1061 = v_k.clone(); Py::Bool({ let t1062 = v_outputs.clone(); if !py::contains(&t1062, &t1061)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1125(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1127: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1126 = v_inputs.clone(); f_chronic_intake(py::getitem(&t1126, &Py::str("C"))?, py::getitem(&t1126, &Py::str("IR"))?, py::getitem(&t1126, &Py::str("BW"))?, py::method(&t1126, "get", &[Py::str("EF"), Py::None])?, py::method(&t1126, "get", &[Py::str("ED"), Py::None])?, py::method(&t1126, "get", &[Py::str("AT"), Py::None])?, py::method(&t1126, "get", &[Py::str("RfD"), Py::None])?, py::method(&t1126, "get", &[Py::str("SF"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1127 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1128 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1150 = (t1128).items()?;
        if t1150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1150[0].clone();
        v_v = t1150[1].clone();
        if ({ let t1129 = v_k.clone(); Py::Bool({ let t1130 = v_outputs.clone(); if !py::contains(&t1130, &t1129)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1189(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1191: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1190 = v_inputs.clone(); f_cmfr_series(py::getitem(&t1190, &Py::str("Cin"))?, py::getitem(&t1190, &Py::str("k"))?, py::method(&t1190, "get", &[Py::str("theta"), Py::None])?, py::method(&t1190, "get", &[Py::str("N"), Py::None])?, py::method(&t1190, "get", &[Py::str("V"), Py::None])?, py::method(&t1190, "get", &[Py::str("Q"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1191 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1192 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1214 = (t1192).items()?;
        if t1214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1214[0].clone();
        v_v = t1214[1].clone();
        if ({ let t1193 = v_k.clone(); Py::Bool({ let t1194 = v_outputs.clone(); if !py::contains(&t1194, &t1193)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1245(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1247: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1246 = v_inputs.clone(); f_series_efficiency(py::getitem(&t1246, &Py::str("eta"))?, py::method(&t1246, "get", &[Py::str("C_in"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1247 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1248 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1270 = (t1248).items()?;
        if t1270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1270[0].clone();
        v_v = t1270[1].clone();
        if ({ let t1249 = v_k.clone(); Py::Bool({ let t1250 = v_outputs.clone(); if !py::contains(&t1250, &t1249)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1321: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1320 = v_inputs.clone(); f_combustion_oxygen(py::getitem(&t1320, &Py::str("formula"))?, py::method(&t1320, "get", &[Py::str("m"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1321 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1322 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1344 = (t1322).items()?;
        if t1344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1344[0].clone();
        v_v = t1344[1].clone();
        if ({ let t1323 = v_k.clone(); Py::Bool({ let t1324 = v_outputs.clone(); if !py::contains(&t1324, &t1323)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1365: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1364 = v_inputs.clone(); f_ct_contact_time(py::getitem(&t1364, &Py::str("CT"))?, py::getitem(&t1364, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1365 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1366 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1388 = (t1366).items()?;
        if t1388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1388[0].clone();
        v_v = t1388[1].clone();
        if ({ let t1367 = v_k.clone(); Py::Bool({ let t1368 = v_outputs.clone(); if !py::contains(&t1368, &t1367)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1408 = v_inputs.clone(); f_ct_inactivation(py::getitem(&t1408, &Py::str("C"))?, py::getitem(&t1408, &Py::str("t10"))?, py::getitem(&t1408, &Py::str("CT999"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1410 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1432 = (t1410).items()?;
        if t1432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1432[0].clone();
        v_v = t1432[1].clone();
        if ({ let t1411 = v_k.clone(); Py::Bool({ let t1412 = v_outputs.clone(); if !py::contains(&t1412, &t1411)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1453(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1455: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1454 = v_inputs.clone(); f_cyclone_lapple(py::getitem(&t1454, &Py::str("mu"))?, py::getitem(&t1454, &Py::str("W"))?, py::getitem(&t1454, &Py::str("Ne"))?, py::getitem(&t1454, &Py::str("Vi"))?, py::getitem(&t1454, &Py::str("rho_p"))?, py::getitem(&t1454, &Py::str("rho_g"))?, py::getitem(&t1454, &Py::str("dp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1455 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1456 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1478 = (t1456).items()?;
        if t1478.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1478[0].clone();
        v_v = t1478[1].clone();
        if ({ let t1457 = v_k.clone(); Py::Bool({ let t1458 = v_outputs.clone(); if !py::contains(&t1458, &t1457)? { true } else { false } }) }).truthy() {
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
fn s_1526(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1527 = v_inputs.clone(); f_darcy(py::getitem(&t1527, &Py::str("K"))?, py::method(&t1527, "get", &[Py::str("i"), Py::None])?, py::method(&t1527, "get", &[Py::str("dh"), Py::None])?, py::method(&t1527, "get", &[Py::str("L"), Py::None])?, py::method(&t1527, "get", &[Py::str("A"), Py::None])?, py::method(&t1527, "get", &[Py::str("n_e"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1529 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1551 = (t1529).items()?;
        if t1551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1551[0].clone();
        v_v = t1551[1].clone();
        if ({ let t1530 = v_k.clone(); Py::Bool({ let t1531 = v_outputs.clone(); if !py::contains(&t1531, &t1530)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1589(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1591: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1590 = v_inputs.clone(); f_ldn(py::getitem(&t1590, &Py::str("L"))?, py::getitem(&t1590, &Py::str("metric"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1591 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1592 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1614 = (t1592).items()?;
        if t1614.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1614[0].clone();
        v_v = t1614[1].clone();
        if ({ let t1593 = v_k.clone(); Py::Bool({ let t1594 = v_outputs.clone(); if !py::contains(&t1594, &t1593)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1635(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1637: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1636 = v_inputs.clone(); f_db_add(py::getitem(&t1636, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1637 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1638 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1660 = (t1638).items()?;
        if t1660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1660[0].clone();
        v_v = t1660[1].clone();
        if ({ let t1639 = v_k.clone(); Py::Bool({ let t1640 = v_outputs.clone(); if !py::contains(&t1640, &t1639)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1681: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1680 = v_inputs.clone(); f_detention_time(py::getitem(&t1680, &Py::str("V"))?, py::getitem(&t1680, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1681 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1682 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1704 = (t1682).items()?;
        if t1704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1704[0].clone();
        v_v = t1704[1].clone();
        if ({ let t1683 = v_k.clone(); Py::Bool({ let t1684 = v_outputs.clone(); if !py::contains(&t1684, &t1683)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1735(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1737: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1736 = v_inputs.clone(); f_ct_value(py::getitem(&t1736, &Py::str("C"))?, py::getitem(&t1736, &Py::str("V"))?, py::getitem(&t1736, &Py::str("Q"))?, py::getitem(&t1736, &Py::str("BF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1737 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1738 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1760 = (t1738).items()?;
        if t1760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1760[0].clone();
        v_v = t1760[1].clone();
        if ({ let t1739 = v_k.clone(); Py::Bool({ let t1740 = v_outputs.clone(); if !py::contains(&t1740, &t1739)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1779(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1781: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1780 = v_inputs.clone(); f_do_saturation(py::getitem(&t1780, &Py::str("T"))?, py::getitem(&t1780, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1781 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1782 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1804 = (t1782).items()?;
        if t1804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1804[0].clone();
        v_v = t1804[1].clone();
        if ({ let t1783 = v_k.clone(); Py::Bool({ let t1784 = v_outputs.clone(); if !py::contains(&t1784, &t1783)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1837(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1839: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1838 = v_inputs.clone(); f_dulong(py::getitem(&t1838, &Py::str("C"))?, py::getitem(&t1838, &Py::str("H"))?, py::getitem(&t1838, &Py::str("O"))?, py::getitem(&t1838, &Py::str("S"))?, py::getitem(&t1838, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1839 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1840 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1862 = (t1840).items()?;
        if t1862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1862[0].clone();
        v_v = t1862[1].clone();
        if ({ let t1841 = v_k.clone(); Py::Bool({ let t1842 = v_outputs.clone(); if !py::contains(&t1842, &t1841)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1894 = v_inputs.clone(); f_leq(py::getitem(&t1894, &Py::str("L"))?, py::getitem(&t1894, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1896 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1918 = (t1896).items()?;
        if t1918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1918[0].clone();
        v_v = t1918[1].clone();
        if ({ let t1897 = v_k.clone(); Py::Bool({ let t1898 = v_outputs.clone(); if !py::contains(&t1898, &t1897)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1949(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1951: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1950 = v_inputs.clone(); f_esp_area(py::getitem(&t1950, &Py::str("w"))?, py::getitem(&t1950, &Py::str("Q"))?, py::getitem(&t1950, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1951 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1952 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1974 = (t1952).items()?;
        if t1974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1974[0].clone();
        v_v = t1974[1].clone();
        if ({ let t1953 = v_k.clone(); Py::Bool({ let t1954 = v_outputs.clone(); if !py::contains(&t1954, &t1953)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1993(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1995: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1994 = v_inputs.clone(); f_esp_efficiency(py::getitem(&t1994, &Py::str("w"))?, py::getitem(&t1994, &Py::str("A"))?, py::getitem(&t1994, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1995 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1996 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2018 = (t1996).items()?;
        if t2018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2018[0].clone();
        v_v = t2018[1].clone();
        if ({ let t1997 = v_k.clone(); Py::Bool({ let t1998 = v_outputs.clone(); if !py::contains(&t1998, &t1997)? { true } else { false } }) }).truthy() {
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
fn s_2047(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2049: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2048 = v_inputs.clone(); f_field_oxygen_transfer(py::getitem(&t2048, &Py::str("SOTR"))?, py::getitem(&t2048, &Py::str("alpha"))?, py::getitem(&t2048, &Py::str("beta"))?, py::getitem(&t2048, &Py::str("theta"))?, py::getitem(&t2048, &Py::str("CsT"))?, py::getitem(&t2048, &Py::str("Cs20"))?, py::getitem(&t2048, &Py::str("CL"))?, py::getitem(&t2048, &Py::str("T"))?, py::method(&t2048, "get", &[Py::str("Fp"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2049 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2050 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2072 = (t2050).items()?;
        if t2072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2072[0].clone();
        v_v = t2072[1].clone();
        if ({ let t2051 = v_k.clone(); Py::Bool({ let t2052 = v_outputs.clone(); if !py::contains(&t2052, &t2051)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2104 = v_inputs.clone(); f_ergun_headloss(py::getitem(&t2104, &Py::str("d"))?, py::getitem(&t2104, &Py::str("eps"))?, py::getitem(&t2104, &Py::str("psi"))?, py::getitem(&t2104, &Py::str("L"))?, py::getitem(&t2104, &Py::str("v"))?, py::getitem(&t2104, &Py::str("rho"))?, py::getitem(&t2104, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2128 = (t2106).items()?;
        if t2128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2128[0].clone();
        v_v = t2128[1].clone();
        if ({ let t2107 = v_k.clone(); Py::Bool({ let t2108 = v_outputs.clone(); if !py::contains(&t2108, &t2107)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2147(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2149: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2148 = v_inputs.clone(); f_filter_flow(py::getitem(&t2148, &Py::str("v"))?, py::getitem(&t2148, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2149 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2150 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2172 = (t2150).items()?;
        if t2172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2172[0].clone();
        v_v = t2172[1].clone();
        if ({ let t2151 = v_k.clone(); Py::Bool({ let t2152 = v_outputs.clone(); if !py::contains(&t2152, &t2151)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2194(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2196: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2195 = v_inputs.clone(); f_filter_rate(py::getitem(&t2195, &Py::str("Q"))?, py::getitem(&t2195, &Py::str("A"))?, py::getitem(&t2195, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2196 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2197 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2219 = (t2197).items()?;
        if t2219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2219[0].clone();
        v_v = t2219[1].clone();
        if ({ let t2198 = v_k.clone(); Py::Bool({ let t2199 = v_outputs.clone(); if !py::contains(&t2199, &t2198)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2275(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2277: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2276 = v_inputs.clone(); f_first_order_decay(py::method(&t2276, "get", &[Py::str("C0"), Py::None])?, py::method(&t2276, "get", &[Py::str("C"), Py::None])?, py::method(&t2276, "get", &[Py::str("k"), Py::None])?, py::method(&t2276, "get", &[Py::str("t"), Py::None])?, py::method(&t2276, "get", &[Py::str("t_half"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2277 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2278 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2300 = (t2278).items()?;
        if t2300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2300[0].clone();
        v_v = t2300[1].clone();
        if ({ let t2279 = v_k.clone(); Py::Bool({ let t2280 = v_outputs.clone(); if !py::contains(&t2280, &t2279)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2321: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2320 = v_inputs.clone(); f_fm_ratio(py::getitem(&t2320, &Py::str("Q"))?, py::getitem(&t2320, &Py::str("S0"))?, py::getitem(&t2320, &Py::str("V"))?, py::getitem(&t2320, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2321 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2322 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2344 = (t2322).items()?;
        if t2344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2344[0].clone();
        v_v = t2344[1].clone();
        if ({ let t2323 = v_k.clone(); Py::Bool({ let t2324 = v_outputs.clone(); if !py::contains(&t2324, &t2323)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2365(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2366 = v_inputs.clone(); f_freundlich_dose(py::getitem(&t2366, &Py::str("C0"))?, py::getitem(&t2366, &Py::str("Ce"))?, py::getitem(&t2366, &Py::str("K"))?, py::getitem(&t2366, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2368 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2390 = (t2368).items()?;
        if t2390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2390[0].clone();
        v_v = t2390[1].clone();
        if ({ let t2369 = v_k.clone(); Py::Bool({ let t2370 = v_outputs.clone(); if !py::contains(&t2370, &t2369)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2427 = v_inputs.clone(); f_freundlich_fit(py::getitem(&t2427, &Py::str("Ce"))?, py::getitem(&t2427, &Py::str("qe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2429 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2451 = (t2429).items()?;
        if t2451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2451[0].clone();
        v_v = t2451[1].clone();
        if ({ let t2430 = v_k.clone(); Py::Bool({ let t2431 = v_outputs.clone(); if !py::contains(&t2431, &t2430)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2479(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2481: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2480 = v_inputs.clone(); f_plume_concentration(py::getitem(&t2480, &Py::str("Q"))?, py::getitem(&t2480, &Py::str("u"))?, py::getitem(&t2480, &Py::str("sigma_y"))?, py::getitem(&t2480, &Py::str("sigma_z"))?, py::getitem(&t2480, &Py::str("H"))?, py::getitem(&t2480, &Py::str("y"))?, py::getitem(&t2480, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2481 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2482 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2504 = (t2482).items()?;
        if t2504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2504[0].clone();
        v_v = t2504[1].clone();
        if ({ let t2483 = v_k.clone(); Py::Bool({ let t2484 = v_outputs.clone(); if !py::contains(&t2484, &t2483)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2527(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2529: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2528 = v_inputs.clone(); f_geometric_mean(py::getitem(&t2528, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2529 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2530 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2552 = (t2530).items()?;
        if t2552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2552[0].clone();
        v_v = t2552[1].clone();
        if ({ let t2531 = v_k.clone(); Py::Bool({ let t2532 = v_outputs.clone(); if !py::contains(&t2532, &t2531)? { true } else { false } }) }).truthy() {
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
fn s_2577(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2578 = v_inputs.clone(); f_ground_centerline(py::getitem(&t2578, &Py::str("Q"))?, py::getitem(&t2578, &Py::str("u"))?, py::getitem(&t2578, &Py::str("sigma_y"))?, py::getitem(&t2578, &Py::str("sigma_z"))?, py::getitem(&t2578, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2580 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2602 = (t2580).items()?;
        if t2602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2602[0].clone();
        v_v = t2602[1].clone();
        if ({ let t2581 = v_k.clone(); Py::Bool({ let t2582 = v_outputs.clone(); if !py::contains(&t2582, &t2581)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2640(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2642: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2641 = v_inputs.clone(); f_hazen_williams(py::getitem(&t2641, &Py::str("C"))?, py::getitem(&t2641, &Py::str("D"))?, py::method(&t2641, "get", &[Py::str("S"), Py::None])?, py::method(&t2641, "get", &[Py::str("Q"), Py::None])?, py::method(&t2641, "get", &[Py::str("L"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2642 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2643 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2665 = (t2643).items()?;
        if t2665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2665[0].clone();
        v_v = t2665[1].clone();
        if ({ let t2644 = v_k.clone(); Py::Bool({ let t2645 = v_outputs.clone(); if !py::contains(&t2645, &t2644)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2709(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2711: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2710 = v_inputs.clone(); f_hhv_dry_basis(py::getitem(&t2710, &Py::str("w"))?, py::method(&t2710, "get", &[Py::str("HHV_ar"), Py::None])?, py::method(&t2710, "get", &[Py::str("HHV_dry"), Py::None])?, py::method(&t2710, "get", &[Py::str("ash"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2711 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2712 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2734 = (t2712).items()?;
        if t2734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2734[0].clone();
        v_v = t2734[1].clone();
        if ({ let t2713 = v_k.clone(); Py::Bool({ let t2714 = v_outputs.clone(); if !py::contains(&t2714, &t2713)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2765(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2767: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2766 = v_inputs.clone(); f_henry_convert(py::getitem(&t2766, &Py::str("H"))?, py::getitem(&t2766, &Py::str("form"))?, py::getitem(&t2766, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2767 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2768 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2790 = (t2768).items()?;
        if t2790.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2790[0].clone();
        v_v = t2790[1].clone();
        if ({ let t2769 = v_k.clone(); Py::Bool({ let t2770 = v_outputs.clone(); if !py::contains(&t2770, &t2769)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2812(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2814: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2813 = v_inputs.clone(); f_henry_equilibrium(py::getitem(&t2813, &Py::str("p"))?, py::getitem(&t2813, &Py::str("H"))?, py::method(&t2813, "get", &[Py::str("M"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2814 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2815 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2837 = (t2815).items()?;
        if t2837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2837[0].clone();
        v_v = t2837[1].clone();
        if ({ let t2816 = v_k.clone(); Py::Bool({ let t2817 = v_outputs.clone(); if !py::contains(&t2817, &t2816)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2859(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2861: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2860 = v_inputs.clone(); f_henry_solubility(py::getitem(&t2860, &Py::str("KH"))?, py::getitem(&t2860, &Py::str("P"))?, py::method(&t2860, "get", &[Py::str("M"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2861 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2862 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2884 = (t2862).items()?;
        if t2884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2884[0].clone();
        v_v = t2884[1].clone();
        if ({ let t2863 = v_k.clone(); Py::Bool({ let t2864 = v_outputs.clone(); if !py::contains(&t2864, &t2863)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2906(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2908: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2907 = v_inputs.clone(); f_henry_vant_hoff(py::getitem(&t2907, &Py::str("Hcp"))?, py::getitem(&t2907, &Py::str("B"))?, py::getitem(&t2907, &Py::str("T"))?, py::getitem(&t2907, &Py::str("Tref"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2908 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2909 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2931 = (t2909).items()?;
        if t2931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2931[0].clone();
        v_v = t2931[1].clone();
        if ({ let t2910 = v_k.clone(); Py::Bool({ let t2911 = v_outputs.clone(); if !py::contains(&t2911, &t2910)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2958(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2959 = v_inputs.clone(); f_holland_rise(py::getitem(&t2959, &Py::str("vs"))?, py::getitem(&t2959, &Py::str("d"))?, py::getitem(&t2959, &Py::str("u"))?, py::getitem(&t2959, &Py::str("p"))?, py::getitem(&t2959, &Py::str("Ts"))?, py::getitem(&t2959, &Py::str("Ta"))?, py::getitem(&t2959, &Py::str("hs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2961 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2983 = (t2961).items()?;
        if t2983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2983[0].clone();
        v_v = t2983[1].clone();
        if ({ let t2962 = v_k.clone(); Py::Bool({ let t2963 = v_outputs.clone(); if !py::contains(&t2963, &t2962)? { true } else { false } }) }).truthy() {
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
fn s_3002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3004: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3003 = v_inputs.clone(); f_hrt_olr(py::getitem(&t3003, &Py::str("Q"))?, py::getitem(&t3003, &Py::str("S0"))?, py::getitem(&t3003, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3004 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3005 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3027 = (t3005).items()?;
        if t3027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3027[0].clone();
        v_v = t3027[1].clone();
        if ({ let t3006 = v_k.clone(); Py::Bool({ let t3007 = v_outputs.clone(); if !py::contains(&t3007, &t3006)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3059: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3058 = v_inputs.clone(); f_hocl_fraction(py::getitem(&t3058, &Py::str("pH"))?, py::method(&t3058, "get", &[Py::str("pKa"), Py::None])?, py::method(&t3058, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3059 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3060 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3082 = (t3060).items()?;
        if t3082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3082[0].clone();
        v_v = t3082[1].clone();
        if ({ let t3061 = v_k.clone(); Py::Bool({ let t3062 = v_outputs.clone(); if !py::contains(&t3062, &t3061)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3101(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3103: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3102 = v_inputs.clone(); f_removal_fraction(py::getitem(&t3102, &Py::str("vs"))?, py::getitem(&t3102, &Py::str("Q"))?, py::getitem(&t3102, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3103 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3104 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3126 = (t3104).items()?;
        if t3126.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3126[0].clone();
        v_v = t3126[1].clone();
        if ({ let t3105 = v_k.clone(); Py::Bool({ let t3106 = v_outputs.clone(); if !py::contains(&t3106, &t3105)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3158 = v_inputs.clone(); f_indoor_air(py::getitem(&t3158, &Py::str("S"))?, py::getitem(&t3158, &Py::str("Qv"))?, py::getitem(&t3158, &Py::str("V"))?, py::method(&t3158, "get", &[Py::str("Camb"), Py::None])?, py::method(&t3158, "get", &[Py::str("k"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3160 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3182 = (t3160).items()?;
        if t3182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3182[0].clone();
        v_v = t3182[1].clone();
        if ({ let t3161 = v_k.clone(); Py::Bool({ let t3162 = v_outputs.clone(); if !py::contains(&t3162, &t3161)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3226(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3228: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3227 = v_inputs.clone(); f_kla_fit(py::getitem(&t3227, &Py::str("t"))?, py::getitem(&t3227, &Py::str("C"))?, py::getitem(&t3227, &Py::str("Cs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3228 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3229 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3251 = (t3229).items()?;
        if t3251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3251[0].clone();
        v_v = t3251[1].clone();
        if ({ let t3230 = v_k.clone(); Py::Bool({ let t3231 = v_outputs.clone(); if !py::contains(&t3231, &t3230)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3281(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3283: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3282 = v_inputs.clone(); f_kla_two_point(py::getitem(&t3282, &Py::str("Cs"))?, py::getitem(&t3282, &Py::str("C1"))?, py::getitem(&t3282, &Py::str("C2"))?, py::getitem(&t3282, &Py::str("t1"))?, py::getitem(&t3282, &Py::str("t2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3283 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3284 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3306 = (t3284).items()?;
        if t3306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3306[0].clone();
        v_v = t3306[1].clone();
        if ({ let t3285 = v_k.clone(); Py::Bool({ let t3286 = v_outputs.clone(); if !py::contains(&t3286, &t3285)? { true } else { false } }) }).truthy() {
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
fn s_3331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3333: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3332 = v_inputs.clone(); f_lake_steady(py::getitem(&t3332, &Py::str("W"))?, py::getitem(&t3332, &Py::str("Q"))?, py::getitem(&t3332, &Py::str("k"))?, py::getitem(&t3332, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3333 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3334 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3356 = (t3334).items()?;
        if t3356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3356[0].clone();
        v_v = t3356[1].clone();
        if ({ let t3335 = v_k.clone(); Py::Bool({ let t3336 = v_outputs.clone(); if !py::contains(&t3336, &t3335)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3381(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3383: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3382 = v_inputs.clone(); f_lake_transient(py::getitem(&t3382, &Py::str("W"))?, py::getitem(&t3382, &Py::str("Q"))?, py::getitem(&t3382, &Py::str("k"))?, py::getitem(&t3382, &Py::str("V"))?, py::getitem(&t3382, &Py::str("C0"))?, py::getitem(&t3382, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3383 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3384 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3406 = (t3384).items()?;
        if t3406.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3406[0].clone();
        v_v = t3406[1].clone();
        if ({ let t3385 = v_k.clone(); Py::Bool({ let t3386 = v_outputs.clone(); if !py::contains(&t3386, &t3385)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3449(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3451: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3450 = v_inputs.clone(); f_compaction_ratio(py::method(&t3450, "get", &[Py::str("rho_i"), Py::None])?, py::method(&t3450, "get", &[Py::str("rho_c"), Py::None])?, py::method(&t3450, "get", &[Py::str("V_i"), Py::None])?, py::method(&t3450, "get", &[Py::str("V_c"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3451 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3452 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3474 = (t3452).items()?;
        if t3474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3474[0].clone();
        v_v = t3474[1].clone();
        if ({ let t3453 = v_k.clone(); Py::Bool({ let t3454 = v_outputs.clone(); if !py::contains(&t3454, &t3453)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3504(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3506: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3505 = v_inputs.clone(); f_landfill_volume(py::getitem(&t3505, &Py::str("m"))?, py::getitem(&t3505, &Py::str("rho"))?, py::method(&t3505, "get", &[Py::str("cover"), Py::None])?, py::method(&t3505, "get", &[Py::str("capacity"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3506 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3507 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3529 = (t3507).items()?;
        if t3529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3529[0].clone();
        v_v = t3529[1].clone();
        if ({ let t3508 = v_k.clone(); Py::Bool({ let t3509 = v_outputs.clone(); if !py::contains(&t3509, &t3508)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3556(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3558: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3557 = v_inputs.clone(); f_landfill_gas(py::getitem(&t3557, &Py::str("M"))?, py::getitem(&t3557, &Py::str("L0"))?, py::getitem(&t3557, &Py::str("k"))?, py::getitem(&t3557, &Py::str("tc"))?, py::getitem(&t3557, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3558 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3559 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3581 = (t3559).items()?;
        if t3581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3581[0].clone();
        v_v = t3581[1].clone();
        if ({ let t3560 = v_k.clone(); Py::Bool({ let t3561 = v_outputs.clone(); if !py::contains(&t3561, &t3560)? { true } else { false } }) }).truthy() {
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
fn s_3608(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3610: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3609 = v_inputs.clone(); f_landfill_life(py::getitem(&t3609, &Py::str("capacity"))?, py::getitem(&t3609, &Py::str("M"))?, py::getitem(&t3609, &Py::str("rho"))?, py::getitem(&t3609, &Py::str("cover"))?, py::getitem(&t3609, &Py::str("g"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3610 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3611 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3633 = (t3611).items()?;
        if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3633[0].clone();
        v_v = t3633[1].clone();
        if ({ let t3612 = v_k.clone(); Py::Bool({ let t3613 = v_outputs.clone(); if !py::contains(&t3613, &t3612)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3654(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3656: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3655 = v_inputs.clone(); f_langmuir_dose(py::getitem(&t3655, &Py::str("C0"))?, py::getitem(&t3655, &Py::str("Ce"))?, py::getitem(&t3655, &Py::str("qmax"))?, py::getitem(&t3655, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3656 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3657 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3679 = (t3657).items()?;
        if t3679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3679[0].clone();
        v_v = t3679[1].clone();
        if ({ let t3658 = v_k.clone(); Py::Bool({ let t3659 = v_outputs.clone(); if !py::contains(&t3659, &t3658)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3718(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3720: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3719 = v_inputs.clone(); f_langmuir_fit(py::getitem(&t3719, &Py::str("Ce"))?, py::getitem(&t3719, &Py::str("qe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3720 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3721 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3743 = (t3721).items()?;
        if t3743.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3743[0].clone();
        v_v = t3743[1].clone();
        if ({ let t3722 = v_k.clone(); Py::Bool({ let t3723 = v_outputs.clone(); if !py::contains(&t3723, &t3722)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3773(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3775: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3774 = v_inputs.clone(); f_lapse_stability(py::getitem(&t3774, &Py::str("T1"))?, py::getitem(&t3774, &Py::str("z1"))?, py::getitem(&t3774, &Py::str("T2"))?, py::getitem(&t3774, &Py::str("z2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3775 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3776 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3798 = (t3776).items()?;
        if t3798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3798[0].clone();
        v_v = t3798[1].clone();
        if ({ let t3777 = v_k.clone(); Py::Bool({ let t3778 = v_outputs.clone(); if !py::contains(&t3778, &t3777)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3819(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3821: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3820 = v_inputs.clone(); f_log_removal(py::getitem(&t3820, &Py::str("N0"))?, py::getitem(&t3820, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3821 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3822 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3844 = (t3822).items()?;
        if t3844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3844[0].clone();
        v_v = t3844[1].clone();
        if ({ let t3823 = v_k.clone(); Py::Bool({ let t3824 = v_outputs.clone(); if !py::contains(&t3824, &t3823)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3881(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3883: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3882 = v_inputs.clone(); f_manning(py::getitem(&t3882, &Py::str("n"))?, py::getitem(&t3882, &Py::str("S"))?, py::method(&t3882, "get", &[Py::str("W"), Py::None])?, py::method(&t3882, "get", &[Py::str("y"), Py::None])?, py::method(&t3882, "get", &[Py::str("D"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3883 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3884 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3906 = (t3884).items()?;
        if t3906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3906[0].clone();
        v_v = t3906[1].clone();
        if ({ let t3885 = v_k.clone(); Py::Bool({ let t3886 = v_outputs.clone(); if !py::contains(&t3886, &t3885)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3925(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3927: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3926 = v_inputs.clone(); f_mass_to_ppm(py::getitem(&t3926, &Py::str("C"))?, py::getitem(&t3926, &Py::str("M"))?, py::getitem(&t3926, &Py::str("T"))?, py::getitem(&t3926, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3927 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3928 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3950 = (t3928).items()?;
        if t3950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3950[0].clone();
        v_v = t3950[1].clone();
        if ({ let t3929 = v_k.clone(); Py::Bool({ let t3930 = v_outputs.clone(); if !py::contains(&t3930, &t3929)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4015: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4014 = v_inputs.clone(); f_max_ground_briggs(py::getitem(&t4014, &Py::str("Q"))?, py::getitem(&t4014, &Py::str("u"))?, py::getitem(&t4014, &Py::str("H"))?, py::getitem(&t4014, &Py::str("cls"))?, py::getitem(&t4014, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4015 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4016 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4038 = (t4016).items()?;
        if t4038.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4038[0].clone();
        v_v = t4038[1].clone();
        if ({ let t4017 = v_k.clone(); Py::Bool({ let t4018 = v_outputs.clone(); if !py::contains(&t4018, &t4017)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4059: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4058 = v_inputs.clone(); f_max_ground_turner(py::getitem(&t4058, &Py::str("Q"))?, py::getitem(&t4058, &Py::str("u"))?, py::getitem(&t4058, &Py::str("H"))?, py::getitem(&t4058, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4059 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4060 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4082 = (t4060).items()?;
        if t4082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4082[0].clone();
        v_v = t4082[1].clone();
        if ({ let t4061 = v_k.clone(); Py::Bool({ let t4062 = v_outputs.clone(); if !py::contains(&t4062, &t4061)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4111: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4110 = v_inputs.clone(); f_mcrt_masses(py::getitem(&t4110, &Py::str("M"))?, py::getitem(&t4110, &Py::str("Mw"))?, py::getitem(&t4110, &Py::str("Me"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4111 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4112 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4134 = (t4112).items()?;
        if t4134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4134[0].clone();
        v_v = t4134[1].clone();
        if ({ let t4113 = v_k.clone(); Py::Bool({ let t4114 = v_outputs.clone(); if !py::contains(&t4114, &t4113)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4169 = v_inputs.clone(); f_membrane_flux(py::method(&t4169, "get", &[Py::str("Qp"), Py::None])?, py::method(&t4169, "get", &[Py::str("A"), Py::None])?, py::method(&t4169, "get", &[Py::str("J"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4171 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4193 = (t4171).items()?;
        if t4193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4193[0].clone();
        v_v = t4193[1].clone();
        if ({ let t4172 = v_k.clone(); Py::Bool({ let t4173 = v_outputs.clone(); if !py::contains(&t4173, &t4172)? { true } else { false } }) }).truthy() {
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
fn s_4222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4223 = v_inputs.clone(); f_membrane_balance(py::getitem(&t4223, &Py::str("Qf"))?, py::getitem(&t4223, &Py::str("Qp"))?, py::getitem(&t4223, &Py::str("Cf"))?, py::getitem(&t4223, &Py::str("Cp"))?, py::getitem(&t4223, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4247 = (t4225).items()?;
        if t4247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4247[0].clone();
        v_v = t4247[1].clone();
        if ({ let t4226 = v_k.clone(); Py::Bool({ let t4227 = v_outputs.clone(); if !py::contains(&t4227, &t4226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4276: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4275 = v_inputs.clone(); f_min_srt(py::getitem(&t4275, &Py::str("Y"))?, py::getitem(&t4275, &Py::str("k"))?, py::getitem(&t4275, &Py::str("Ks"))?, py::getitem(&t4275, &Py::str("kd"))?, py::getitem(&t4275, &Py::str("S0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4276 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4277 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4299 = (t4277).items()?;
        if t4299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4299[0].clone();
        v_v = t4299[1].clone();
        if ({ let t4278 = v_k.clone(); Py::Bool({ let t4279 = v_outputs.clone(); if !py::contains(&t4279, &t4278)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4320: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4319 = v_inputs.clone(); f_mixing_power(py::getitem(&t4319, &Py::str("G"))?, py::getitem(&t4319, &Py::str("mu"))?, py::getitem(&t4319, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4320 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4321 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4343 = (t4321).items()?;
        if t4343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4343[0].clone();
        v_v = t4343[1].clone();
        if ({ let t4322 = v_k.clone(); Py::Bool({ let t4323 = v_outputs.clone(); if !py::contains(&t4323, &t4322)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4372(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4374: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4373 = v_inputs.clone(); f_mix_many(py::getitem(&t4373, &Py::str("Q"))?, py::getitem(&t4373, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4374 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4375 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4397 = (t4375).items()?;
        if t4397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4397[0].clone();
        v_v = t4397[1].clone();
        if ({ let t4376 = v_k.clone(); Py::Bool({ let t4377 = v_outputs.clone(); if !py::contains(&t4377, &t4376)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4422(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4424: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4423 = v_inputs.clone(); f_mix_two(py::getitem(&t4423, &Py::str("Qw"))?, py::getitem(&t4423, &Py::str("Cw"))?, py::getitem(&t4423, &Py::str("Qr"))?, py::getitem(&t4423, &Py::str("Cr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4424 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4425 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4447 = (t4425).items()?;
        if t4447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4447[0].clone();
        v_v = t4447[1].clone();
        if ({ let t4426 = v_k.clone(); Py::Bool({ let t4427 = v_outputs.clone(); if !py::contains(&t4427, &t4426)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4476 = v_inputs.clone(); f_monod(py::getitem(&t4476, &Py::str("mu_max"))?, py::getitem(&t4476, &Py::str("Ks"))?, py::getitem(&t4476, &Py::str("S"))?, py::method(&t4476, "get", &[Py::str("kd"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4500 = (t4478).items()?;
        if t4500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4500[0].clone();
        v_v = t4500[1].clone();
        if ({ let t4479 = v_k.clone(); Py::Bool({ let t4480 = v_outputs.clone(); if !py::contains(&t4480, &t4479)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4528(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4530: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4529 = v_inputs.clone(); f_distance_attenuation(py::getitem(&t4529, &Py::str("L1"))?, py::getitem(&t4529, &Py::str("r1"))?, py::getitem(&t4529, &Py::str("r2"))?, py::getitem(&t4529, &Py::str("source"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4530 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4531 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4553 = (t4531).items()?;
        if t4553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4553[0].clone();
        v_v = t4553[1].clone();
        if ({ let t4532 = v_k.clone(); Py::Bool({ let t4533 = v_outputs.clone(); if !py::contains(&t4533, &t4532)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4578(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4580: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4579 = v_inputs.clone(); f_nrc_trickling(py::getitem(&t4579, &Py::str("W"))?, py::getitem(&t4579, &Py::str("V"))?, py::getitem(&t4579, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4580 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4581 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4603 = (t4581).items()?;
        if t4603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4603[0].clone();
        v_v = t4603[1].clone();
        if ({ let t4582 = v_k.clone(); Py::Bool({ let t4583 = v_outputs.clone(); if !py::contains(&t4583, &t4582)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4636(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4638: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4637 = v_inputs.clone(); f_noise_dose(py::getitem(&t4637, &Py::str("L"))?, py::getitem(&t4637, &Py::str("hours"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4638 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4639 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4661 = (t4639).items()?;
        if t4661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4661[0].clone();
        v_v = t4661[1].clone();
        if ({ let t4640 = v_k.clone(); Py::Bool({ let t4641 = v_outputs.clone(); if !py::contains(&t4641, &t4640)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4680(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4682: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4681 = v_inputs.clone(); f_osmotic_pressure(py::getitem(&t4681, &Py::str("C"))?, py::getitem(&t4681, &Py::str("M"))?, py::getitem(&t4681, &Py::str("i"))?, py::getitem(&t4681, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4682 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4683 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4705 = (t4683).items()?;
        if t4705.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4705[0].clone();
        v_v = t4705[1].clone();
        if ({ let t4684 = v_k.clone(); Py::Bool({ let t4685 = v_outputs.clone(); if !py::contains(&t4685, &t4684)? { true } else { false } }) }).truthy() {
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
fn s_4724(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4725 = v_inputs.clone(); f_overflow_rate(py::getitem(&t4725, &Py::str("Q"))?, py::getitem(&t4725, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4727 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4749 = (t4727).items()?;
        if t4749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4749[0].clone();
        v_v = t4749[1].clone();
        if ({ let t4728 = v_k.clone(); Py::Bool({ let t4729 = v_outputs.clone(); if !py::contains(&t4729, &t4728)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4780(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4782: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4781 = v_inputs.clone(); f_sag_distance(py::getitem(&t4781, &Py::str("kd"))?, py::getitem(&t4781, &Py::str("ka"))?, py::getitem(&t4781, &Py::str("L0"))?, py::getitem(&t4781, &Py::str("D0"))?, py::getitem(&t4781, &Py::str("x"))?, py::getitem(&t4781, &Py::str("v"))?, py::getitem(&t4781, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4782 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4783 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4805 = (t4783).items()?;
        if t4805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4805[0].clone();
        v_v = t4805[1].clone();
        if ({ let t4784 = v_k.clone(); Py::Bool({ let t4785 = v_outputs.clone(); if !py::contains(&t4785, &t4784)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4837(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4839: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4838 = v_inputs.clone(); f_ph_poh(py::method(&t4838, "get", &[Py::str("H"), Py::None])?, py::method(&t4838, "get", &[Py::str("OH"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4839 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4840 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4862 = (t4840).items()?;
        if t4862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4862[0].clone();
        v_v = t4862[1].clone();
        if ({ let t4841 = v_k.clone(); Py::Bool({ let t4842 = v_outputs.clone(); if !py::contains(&t4842, &t4841)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4902(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4904: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4903 = v_inputs.clone(); f_plume_with_class(py::getitem(&t4903, &Py::str("Q"))?, py::getitem(&t4903, &Py::str("u"))?, py::getitem(&t4903, &Py::str("H"))?, py::getitem(&t4903, &Py::str("x"))?, py::getitem(&t4903, &Py::str("y"))?, py::getitem(&t4903, &Py::str("z"))?, py::getitem(&t4903, &Py::str("cls"))?, py::getitem(&t4903, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4904 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4905 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4927 = (t4905).items()?;
        if t4927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4927[0].clone();
        v_v = t4927[1].clone();
        if ({ let t4906 = v_k.clone(); Py::Bool({ let t4907 = v_outputs.clone(); if !py::contains(&t4907, &t4906)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4966: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4965 = v_inputs.clone(); f_plume_lid(py::getitem(&t4965, &Py::str("Q"))?, py::getitem(&t4965, &Py::str("u"))?, py::getitem(&t4965, &Py::str("sigma_y"))?, py::getitem(&t4965, &Py::str("sigma_z"))?, py::getitem(&t4965, &Py::str("H"))?, py::getitem(&t4965, &Py::str("Lm"))?, py::getitem(&t4965, &Py::str("y"))?, py::getitem(&t4965, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4966 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4967 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4989 = (t4967).items()?;
        if t4989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4989[0].clone();
        v_v = t4989[1].clone();
        if ({ let t4968 = v_k.clone(); Py::Bool({ let t4969 = v_outputs.clone(); if !py::contains(&t4969, &t4968)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5014(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5016: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5015 = v_inputs.clone(); f_pollutant_load(py::getitem(&t5015, &Py::str("Q"))?, py::getitem(&t5015, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5016 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5017 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5039 = (t5017).items()?;
        if t5039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5039[0].clone();
        v_v = t5039[1].clone();
        if ({ let t5018 = v_k.clone(); Py::Bool({ let t5019 = v_outputs.clone(); if !py::contains(&t5019, &t5018)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5075(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5077: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5076 = v_inputs.clone(); f_population_logistic(py::getitem(&t5076, &Py::str("t0"))?, py::getitem(&t5076, &Py::str("n"))?, py::getitem(&t5076, &Py::str("P0"))?, py::getitem(&t5076, &Py::str("P1"))?, py::getitem(&t5076, &Py::str("P2"))?, py::getitem(&t5076, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5077 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5078 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5100 = (t5078).items()?;
        if t5100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5100[0].clone();
        v_v = t5100[1].clone();
        if ({ let t5079 = v_k.clone(); Py::Bool({ let t5080 = v_outputs.clone(); if !py::contains(&t5080, &t5079)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5131(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5133: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5132 = v_inputs.clone(); f_population_projection(py::getitem(&t5132, &Py::str("t1"))?, py::getitem(&t5132, &Py::str("P1"))?, py::getitem(&t5132, &Py::str("t2"))?, py::getitem(&t5132, &Py::str("P2"))?, py::getitem(&t5132, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5133 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5134 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5156 = (t5134).items()?;
        if t5156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5156[0].clone();
        v_v = t5156[1].clone();
        if ({ let t5135 = v_k.clone(); Py::Bool({ let t5136 = v_outputs.clone(); if !py::contains(&t5136, &t5135)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5176 = v_inputs.clone(); f_ppm_to_mass(py::getitem(&t5176, &Py::str("ppm"))?, py::getitem(&t5176, &Py::str("M"))?, py::getitem(&t5176, &Py::str("T"))?, py::getitem(&t5176, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5178 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5200 = (t5178).items()?;
        if t5200.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5200[0].clone();
        v_v = t5200[1].clone();
        if ({ let t5179 = v_k.clone(); Py::Bool({ let t5180 = v_outputs.clone(); if !py::contains(&t5180, &t5179)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5248(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5250: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5249 = v_inputs.clone(); f_pump_power(py::getitem(&t5249, &Py::str("Q"))?, py::getitem(&t5249, &Py::str("H"))?, py::getitem(&t5249, &Py::str("eta"))?, py::method(&t5249, "get", &[Py::str("rho"), Py::None])?, py::method(&t5249, "get", &[Py::str("hours"), Py::None])?, py::method(&t5249, "get", &[Py::str("price"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5250 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5251 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5273 = (t5251).items()?;
        if t5273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5273[0].clone();
        v_v = t5273[1].clone();
        if ({ let t5252 = v_k.clone(); Py::Bool({ let t5253 = v_outputs.clone(); if !py::contains(&t5253, &t5252)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5298(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5300: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5299 = v_inputs.clone(); f_k_temperature(py::getitem(&t5299, &Py::str("k20"))?, py::getitem(&t5299, &Py::str("theta"))?, py::getitem(&t5299, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5300 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5301 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5323 = (t5301).items()?;
        if t5323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5323[0].clone();
        v_v = t5323[1].clone();
        if ({ let t5302 = v_k.clone(); Py::Bool({ let t5303 = v_outputs.clone(); if !py::contains(&t5303, &t5302)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5354(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5356: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5355 = v_inputs.clone(); f_rational_method(py::getitem(&t5355, &Py::str("C"))?, py::getitem(&t5355, &Py::str("i"))?, py::getitem(&t5355, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5356 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5357 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5379 = (t5357).items()?;
        if t5379.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5379[0].clone();
        v_v = t5379[1].clone();
        if ({ let t5358 = v_k.clone(); Py::Bool({ let t5359 = v_outputs.clone(); if !py::contains(&t5359, &t5358)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5401(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5403: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5402 = v_inputs.clone(); f_reaeration(py::getitem(&t5402, &Py::str("method"))?, py::getitem(&t5402, &Py::str("u"))?, py::getitem(&t5402, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5403 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5404 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5426 = (t5404).items()?;
        if t5426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5426[0].clone();
        v_v = t5426[1].clone();
        if ({ let t5405 = v_k.clone(); Py::Bool({ let t5406 = v_outputs.clone(); if !py::contains(&t5406, &t5405)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5503(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5504 = v_inputs.clone(); f_retardation(py::method(&t5504, "get", &[Py::str("rho_b"), Py::None])?, py::method(&t5504, "get", &[Py::str("n"), Py::None])?, py::method(&t5504, "get", &[Py::str("Kd"), Py::None])?, py::method(&t5504, "get", &[Py::str("Koc"), Py::None])?, py::method(&t5504, "get", &[Py::str("foc"), Py::None])?, py::method(&t5504, "get", &[Py::str("R"), Py::None])?, py::method(&t5504, "get", &[Py::str("v"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5506 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5528 = (t5506).items()?;
        if t5528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5528[0].clone();
        v_v = t5528[1].clone();
        if ({ let t5507 = v_k.clone(); Py::Bool({ let t5508 = v_outputs.clone(); if !py::contains(&t5508, &t5507)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5575(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5577: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5576 = v_inputs.clone(); f_return_period(py::method(&t5576, "get", &[Py::str("T"), Py::None])?, py::method(&t5576, "get", &[Py::str("n"), Py::None])?, py::method(&t5576, "get", &[Py::str("risk"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5577 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5578 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5600 = (t5578).items()?;
        if t5600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5600[0].clone();
        v_v = t5600[1].clone();
        if ({ let t5579 = v_k.clone(); Py::Bool({ let t5580 = v_outputs.clone(); if !py::contains(&t5580, &t5579)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5622 = v_inputs.clone(); f_return_ratio(py::getitem(&t5622, &Py::str("X"))?, py::getitem(&t5622, &Py::str("Xr"))?, py::getitem(&t5622, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5646 = (t5624).items()?;
        if t5646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5646[0].clone();
        v_v = t5646[1].clone();
        if ({ let t5625 = v_k.clone(); Py::Bool({ let t5626 = v_outputs.clone(); if !py::contains(&t5626, &t5625)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5671(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5673: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5672 = v_inputs.clone(); f_river_decay(py::getitem(&t5672, &Py::str("C0"))?, py::getitem(&t5672, &Py::str("k"))?, py::getitem(&t5672, &Py::str("x"))?, py::getitem(&t5672, &Py::str("u"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5673 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5674 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5696 = (t5674).items()?;
        if t5696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5696[0].clone();
        v_v = t5696[1].clone();
        if ({ let t5675 = v_k.clone(); Py::Bool({ let t5676 = v_outputs.clone(); if !py::contains(&t5676, &t5675)? { true } else { false } }) }).truthy() {
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
fn s_5715(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5717: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5716 = v_inputs.clone(); f_sed_basin(py::getitem(&t5716, &Py::str("Q"))?, py::getitem(&t5716, &Py::str("v0"))?, py::getitem(&t5716, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5717 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5718 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5740 = (t5718).items()?;
        if t5740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5740[0].clone();
        v_v = t5740[1].clone();
        if ({ let t5719 = v_k.clone(); Py::Bool({ let t5720 = v_outputs.clone(); if !py::contains(&t5720, &t5719)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5766(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5768: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5767 = v_inputs.clone(); f_settling_chamber(py::getitem(&t5767, &Py::str("dp"))?, py::getitem(&t5767, &Py::str("rho_p"))?, py::getitem(&t5767, &Py::str("rho_g"))?, py::getitem(&t5767, &Py::str("mu"))?, py::getitem(&t5767, &Py::str("L"))?, py::getitem(&t5767, &Py::str("W"))?, py::getitem(&t5767, &Py::str("Q"))?, py::getitem(&t5767, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5768 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5769 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5791 = (t5769).items()?;
        if t5791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5791[0].clone();
        v_v = t5791[1].clone();
        if ({ let t5770 = v_k.clone(); Py::Bool({ let t5771 = v_outputs.clone(); if !py::contains(&t5771, &t5770)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5818(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5820: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5819 = v_inputs.clone(); f_sludge_production(py::getitem(&t5819, &Py::str("Y"))?, py::getitem(&t5819, &Py::str("kd"))?, py::getitem(&t5819, &Py::str("SRT"))?, py::getitem(&t5819, &Py::str("Q"))?, py::getitem(&t5819, &Py::str("S0"))?, py::getitem(&t5819, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5820 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5821 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5843 = (t5821).items()?;
        if t5843.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5843[0].clone();
        v_v = t5843[1].clone();
        if ({ let t5822 = v_k.clone(); Py::Bool({ let t5823 = v_outputs.clone(); if !py::contains(&t5823, &t5822)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5876(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5878: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5877 = v_inputs.clone(); f_thickening(py::getitem(&t5877, &Py::str("V1"))?, py::getitem(&t5877, &Py::str("P1"))?, py::getitem(&t5877, &Py::str("P2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5878 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5879 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5901 = (t5879).items()?;
        if t5901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5901[0].clone();
        v_v = t5901[1].clone();
        if ({ let t5880 = v_k.clone(); Py::Bool({ let t5881 = v_outputs.clone(); if !py::contains(&t5881, &t5880)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5932(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5934: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5933 = v_inputs.clone(); f_sludge_volume(py::getitem(&t5933, &Py::str("Ms"))?, py::getitem(&t5933, &Py::str("Ps"))?, py::getitem(&t5933, &Py::str("Ssl"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5934 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5935 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5957 = (t5935).items()?;
        if t5957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5957[0].clone();
        v_v = t5957[1].clone();
        if ({ let t5936 = v_k.clone(); Py::Bool({ let t5937 = v_outputs.clone(); if !py::contains(&t5937, &t5936)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5978(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5980: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5979 = v_inputs.clone(); f_svi(py::getitem(&t5979, &Py::str("V30"))?, py::getitem(&t5979, &Py::str("MLSS"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5980 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5981 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6003 = (t5981).items()?;
        if t6003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6003[0].clone();
        v_v = t6003[1].clone();
        if ({ let t5982 = v_k.clone(); Py::Bool({ let t5983 = v_outputs.clone(); if !py::contains(&t5983, &t5982)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6034(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6036: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6035 = v_inputs.clone(); f_so2_emission(py::getitem(&t6035, &Py::str("m_fuel"))?, py::getitem(&t6035, &Py::str("S"))?, py::getitem(&t6035, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6036 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6037 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6059 = (t6037).items()?;
        if t6059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6059[0].clone();
        v_v = t6059[1].clone();
        if ({ let t6038 = v_k.clone(); Py::Bool({ let t6039 = v_outputs.clone(); if !py::contains(&t6039, &t6038)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6082 = v_inputs.clone(); f_waste_generation(py::getitem(&t6082, &Py::str("pop"))?, py::getitem(&t6082, &Py::str("rate"))?, py::getitem(&t6082, &Py::str("rho"))?, py::getitem(&t6082, &Py::str("Vt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6084 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6106 = (t6084).items()?;
        if t6106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6106[0].clone();
        v_v = t6106[1].clone();
        if ({ let t6085 = v_k.clone(); Py::Bool({ let t6086 = v_outputs.clone(); if !py::contains(&t6086, &t6085)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6137(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6139: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6138 = v_inputs.clone(); f_srt(py::getitem(&t6138, &Py::str("V"))?, py::getitem(&t6138, &Py::str("X"))?, py::getitem(&t6138, &Py::str("Qw"))?, py::getitem(&t6138, &Py::str("Xw"))?, py::method(&t6138, "get", &[Py::str("Qe"), Py::None])?, py::method(&t6138, "get", &[Py::str("Xe"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6139 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6140 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6162 = (t6140).items()?;
        if t6162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6162[0].clone();
        v_v = t6162[1].clone();
        if ({ let t6141 = v_k.clone(); Py::Bool({ let t6142 = v_outputs.clone(); if !py::contains(&t6142, &t6141)? { true } else { false } }) }).truthy() {
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
fn s_6186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6188: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6187 = v_inputs.clone(); f_sound_power_to_pressure(py::getitem(&t6187, &Py::str("Lw"))?, py::getitem(&t6187, &Py::str("r"))?, py::getitem(&t6187, &Py::str("Qd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6188 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6189 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6211 = (t6189).items()?;
        if t6211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6211[0].clone();
        v_v = t6211[1].clone();
        if ({ let t6190 = v_k.clone(); Py::Bool({ let t6191 = v_outputs.clone(); if !py::contains(&t6191, &t6190)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6244(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6246: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6245 = v_inputs.clone(); f_stability_class(py::getitem(&t6245, &Py::str("u"))?, py::getitem(&t6245, &Py::str("condition"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6246 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6247 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6269 = (t6247).items()?;
        if t6269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6269[0].clone();
        v_v = t6269[1].clone();
        if ({ let t6248 = v_k.clone(); Py::Bool({ let t6249 = v_outputs.clone(); if !py::contains(&t6249, &t6248)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6316: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6315 = v_inputs.clone(); f_plume_chain(py::getitem(&t6315, &Py::str("Q"))?, py::getitem(&t6315, &Py::str("u"))?, py::getitem(&t6315, &Py::str("vs"))?, py::getitem(&t6315, &Py::str("d"))?, py::getitem(&t6315, &Py::str("Ts"))?, py::getitem(&t6315, &Py::str("Ta"))?, py::getitem(&t6315, &Py::str("hs"))?, py::getitem(&t6315, &Py::str("x"))?, py::getitem(&t6315, &Py::str("cls"))?, py::getitem(&t6315, &Py::str("terrain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6316 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6317 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6339 = (t6317).items()?;
        if t6339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6339[0].clone();
        v_v = t6339[1].clone();
        if ({ let t6318 = v_k.clone(); Py::Bool({ let t6319 = v_outputs.clone(); if !py::contains(&t6319, &t6318)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6362(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6364: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6363 = v_inputs.clone(); f_stokes(py::getitem(&t6363, &Py::str("d"))?, py::getitem(&t6363, &Py::str("rho_p"))?, py::getitem(&t6363, &Py::str("rho_w"))?, py::getitem(&t6363, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6364 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6365 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6387 = (t6365).items()?;
        if t6387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6387[0].clone();
        v_v = t6387[1].clone();
        if ({ let t6366 = v_k.clone(); Py::Bool({ let t6367 = v_outputs.clone(); if !py::contains(&t6367, &t6366)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6422(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6424: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6423 = v_inputs.clone(); f_sp_critical(py::getitem(&t6423, &Py::str("kd"))?, py::getitem(&t6423, &Py::str("ka"))?, py::getitem(&t6423, &Py::str("L0"))?, py::getitem(&t6423, &Py::str("D0"))?, py::getitem(&t6423, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6424 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6425 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6447 = (t6425).items()?;
        if t6447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6447[0].clone();
        v_v = t6447[1].clone();
        if ({ let t6426 = v_k.clone(); Py::Bool({ let t6427 = v_outputs.clone(); if !py::contains(&t6427, &t6426)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6478(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6480: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6479 = v_inputs.clone(); f_streeter_phelps(py::getitem(&t6479, &Py::str("kd"))?, py::getitem(&t6479, &Py::str("ka"))?, py::getitem(&t6479, &Py::str("L0"))?, py::getitem(&t6479, &Py::str("D0"))?, py::getitem(&t6479, &Py::str("t"))?, py::getitem(&t6479, &Py::str("DOsat"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6480 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6481 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6503 = (t6481).items()?;
        if t6503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6503[0].clone();
        v_v = t6503[1].clone();
        if ({ let t6482 = v_k.clone(); Py::Bool({ let t6483 = v_outputs.clone(); if !py::contains(&t6483, &t6482)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6552(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6553 = v_inputs.clone(); f_terminal_velocity(py::getitem(&t6553, &Py::str("d"))?, py::getitem(&t6553, &Py::str("rho_p"))?, py::getitem(&t6553, &Py::str("rho_f"))?, py::getitem(&t6553, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6555 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6577 = (t6555).items()?;
        if t6577.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6577[0].clone();
        v_v = t6577[1].clone();
        if ({ let t6556 = v_k.clone(); Py::Bool({ let t6557 = v_outputs.clone(); if !py::contains(&t6557, &t6556)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6626(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6628: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6627 = v_inputs.clone(); f_thod(py::getitem(&t6627, &Py::str("formula"))?, py::getitem(&t6627, &Py::str("nitrification"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6628 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6629 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6651 = (t6629).items()?;
        if t6651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6651[0].clone();
        v_v = t6651[1].clone();
        if ({ let t6630 = v_k.clone(); Py::Bool({ let t6631 = v_outputs.clone(); if !py::contains(&t6631, &t6630)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6708(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6710: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6709 = v_inputs.clone(); f_thod_solution(py::getitem(&t6709, &Py::str("formula"))?, py::getitem(&t6709, &Py::str("C"))?, py::method(&t6709, "get", &[Py::str("nitrification"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6710 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6711 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6733 = (t6711).items()?;
        if t6733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6733[0].clone();
        v_v = t6733[1].clone();
        if ({ let t6712 = v_k.clone(); Py::Bool({ let t6713 = v_outputs.clone(); if !py::contains(&t6713, &t6712)? { true } else { false } }) }).truthy() {
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
fn s_6799(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6801: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6800 = v_inputs.clone(); f_alkalinity(py::method(&t6800, "get", &[Py::str("HCO3"), Py::None])?, py::method(&t6800, "get", &[Py::str("CO3"), Py::None])?, py::method(&t6800, "get", &[Py::str("OH"), Py::None])?, py::method(&t6800, "get", &[Py::str("H"), Py::None])?, py::method(&t6800, "get", &[Py::str("HCO3_M"), Py::None])?, py::method(&t6800, "get", &[Py::str("CO3_M"), Py::None])?, py::method(&t6800, "get", &[Py::str("OH_M"), Py::None])?, py::method(&t6800, "get", &[Py::str("H_M"), Py::None])?, py::method(&t6800, "get", &[Py::str("pH"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6801 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6802 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6824 = (t6802).items()?;
        if t6824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6824[0].clone();
        v_v = t6824[1].clone();
        if ({ let t6803 = v_k.clone(); Py::Bool({ let t6804 = v_outputs.clone(); if !py::contains(&t6804, &t6803)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6844(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6846: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6845 = v_inputs.clone(); f_transmembrane_pressure(py::getitem(&t6845, &Py::str("P_feed"))?, py::getitem(&t6845, &Py::str("P_conc"))?, py::getitem(&t6845, &Py::str("P_perm"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6846 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6847 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6869 = (t6847).items()?;
        if t6869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6869[0].clone();
        v_v = t6869[1].clone();
        if ({ let t6848 = v_k.clone(); Py::Bool({ let t6849 = v_outputs.clone(); if !py::contains(&t6849, &t6848)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6894(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6896: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6895 = v_inputs.clone(); f_two_film(py::getitem(&t6895, &Py::str("kl"))?, py::getitem(&t6895, &Py::str("kg"))?, py::getitem(&t6895, &Py::str("H"))?, py::getitem(&t6895, &Py::str("C"))?, py::getitem(&t6895, &Py::str("Cg"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6896 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6897 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6919 = (t6897).items()?;
        if t6919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6919[0].clone();
        v_v = t6919[1].clone();
        if ({ let t6898 = v_k.clone(); Py::Bool({ let t6899 = v_outputs.clone(); if !py::contains(&t6899, &t6898)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6938(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6940: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6939 = v_inputs.clone(); f_ultimate_bod(py::getitem(&t6939, &Py::str("BOD_t"))?, py::getitem(&t6939, &Py::str("k"))?, py::getitem(&t6939, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6940 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6941 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6963 = (t6941).items()?;
        if t6963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6963[0].clone();
        v_v = t6963[1].clone();
        if ({ let t6942 = v_k.clone(); Py::Bool({ let t6943 = v_outputs.clone(); if !py::contains(&t6943, &t6942)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6984(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6986: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6985 = v_inputs.clone(); f_velocity_gradient(py::getitem(&t6985, &Py::str("P"))?, py::getitem(&t6985, &Py::str("mu"))?, py::getitem(&t6985, &Py::str("V"))?, py::method(&t6985, "get", &[Py::str("t"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6986 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6987 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7009 = (t6987).items()?;
        if t7009.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7009[0].clone();
        v_v = t7009[1].clone();
        if ({ let t6988 = v_k.clone(); Py::Bool({ let t6989 = v_outputs.clone(); if !py::contains(&t6989, &t6988)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7036(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7038: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7037 = v_inputs.clone(); f_vs_reduction(py::getitem(&t7037, &Py::str("VS_in"))?, py::getitem(&t7037, &Py::str("VS_out"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7038 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7039 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7061 = (t7039).items()?;
        if t7061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7061[0].clone();
        v_v = t7061[1].clone();
        if ({ let t7040 = v_k.clone(); Py::Bool({ let t7041 = v_outputs.clone(); if !py::contains(&t7041, &t7040)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7092(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7094: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7093 = v_inputs.clone(); f_waste_moisture(py::getitem(&t7093, &Py::str("m"))?, py::getitem(&t7093, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7094 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7095 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7117 = (t7095).items()?;
        if t7117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7117[0].clone();
        v_v = t7117[1].clone();
        if ({ let t7096 = v_k.clone(); Py::Bool({ let t7097 = v_outputs.clone(); if !py::contains(&t7097, &t7096)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7164(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7166: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7165 = v_inputs.clone(); f_water_demand(py::getitem(&t7165, &Py::str("pop"))?, py::getitem(&t7165, &Py::str("q"))?, py::getitem(&t7165, &Py::str("f_md"))?, py::getitem(&t7165, &Py::str("f_ph"))?, py::method(&t7165, "get", &[Py::str("r_ww"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7166 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7167 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7189 = (t7167).items()?;
        if t7189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7189[0].clone();
        v_v = t7189[1].clone();
        if ({ let t7168 = v_k.clone(); Py::Bool({ let t7169 = v_outputs.clone(); if !py::contains(&t7169, &t7168)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7214(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7216: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7215 = v_inputs.clone(); f_hardness(py::getitem(&t7215, &Py::str("Ca"))?, py::getitem(&t7215, &Py::str("Mg"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7216 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7217 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7239 = (t7217).items()?;
        if t7239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7239[0].clone();
        v_v = t7239[1].clone();
        if ({ let t7218 = v_k.clone(); Py::Bool({ let t7219 = v_outputs.clone(); if !py::contains(&t7219, &t7218)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7258(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7260: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7259 = v_inputs.clone(); f_weir_loading(py::getitem(&t7259, &Py::str("Q"))?, py::getitem(&t7259, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7260 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7261 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7283 = (t7261).items()?;
        if t7283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7283[0].clone();
        v_v = t7283[1].clone();
        if ({ let t7262 = v_k.clone(); Py::Bool({ let t7263 = v_outputs.clone(); if !py::contains(&t7263, &t7262)? { true } else { false } }) }).truthy() {
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
fn s_7305(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7307: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7306 = v_inputs.clone(); f_wind_power_law(py::getitem(&t7306, &Py::str("u1"))?, py::getitem(&t7306, &Py::str("z1"))?, py::getitem(&t7306, &Py::str("z2"))?, py::getitem(&t7306, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7307 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7308 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7330 = (t7308).items()?;
        if t7330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7330[0].clone();
        v_v = t7330[1].clone();
        if ({ let t7309 = v_k.clone(); Py::Bool({ let t7310 = v_outputs.clone(); if !py::contains(&t7310, &t7309)? { true } else { false } }) }).truthy() {
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
fn s_832(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t834: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t833 = v_inputs.clone(); f_channel_velocity(py::getitem(&t833, &Py::str("W"))?, py::getitem(&t833, &Py::str("d"))?, py::method(&t833, "get", &[Py::str("Q"), Py::None])?, py::method(&t833, "get", &[Py::str("v"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t834 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t835 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t857 = (t835).items()?;
        if t857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t857[0].clone();
        v_v = t857[1].clone();
        if ({ let t836 = v_k.clone(); Py::Bool({ let t837 = v_outputs.clone(); if !py::contains(&t837, &t836)? { true } else { false } }) }).truthy() {
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
fn s_890(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t892: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t891 = v_inputs.clone(); f_chemical_feed(py::getitem(&t891, &Py::str("Q"))?, py::getitem(&t891, &Py::str("dose"))?, py::method(&t891, "get", &[Py::str("purity"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t892 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t893 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t915 = (t893).items()?;
        if t915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t915[0].clone();
        v_v = t915[1].clone();
        if ({ let t894 = v_k.clone(); Py::Bool({ let t895 = v_outputs.clone(); if !py::contains(&t895, &t894)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_940(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t942: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t941 = v_inputs.clone(); f_chick_first_order(py::getitem(&t941, &Py::str("N0"))?, py::getitem(&t941, &Py::str("k"))?, py::getitem(&t941, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t942 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t965 = (t943).items()?;
        if t965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t965[0].clone();
        v_v = t965[1].clone();
        if ({ let t944 = v_k.clone(); Py::Bool({ let t945 = v_outputs.clone(); if !py::contains(&t945, &t944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_993(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t995: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t994 = v_inputs.clone(); f_chick_watson(py::getitem(&t994, &Py::str("k"))?, py::getitem(&t994, &Py::str("n"))?, py::getitem(&t994, &Py::str("C"))?, py::getitem(&t994, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t995 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t996 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1018 = (t996).items()?;
        if t1018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1018[0].clone();
        v_v = t1018[1].clone();
        if ({ let t997 = v_k.clone(); Py::Bool({ let t998 = v_outputs.clone(); if !py::contains(&t998, &t997)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
