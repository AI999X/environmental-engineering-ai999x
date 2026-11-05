//! Environmental Engineering's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the
//! model computes: values over two inputs, single responses, fitted test data, the stability verdict along the wind
//! speed and the worked request as a table (docs/FIGURE_PARITY.md). Helpers: figs_common.rs, figs_more.rs.

use crate::figs_common as c;
use crate::figs_more::{self as m, dec, fx};
use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // a value over two inputs
        "enveng_detention_time" => vec![m::field_half(s, "field", "V", "Q", "theta")],
        "enveng_henry_constant_conversion" => vec![m::field_half(s, "field", "H", "T", "H_dimensionless")],
        "enveng_hypochlorous_acid_fraction" => vec![hocl_field(s)],
        "enveng_log_removal" => vec![log_field(s)],
        "enveng_overflow_rate_circular" => vec![m::field_half(s, "field", "D", "Q", "v0")],
        "enveng_pollutant_load" => vec![m::field_half(s, "field", "Q", "C", "W")],
        "enveng_reaeration_coefficient" => vec![m::field_half(s, "field", "H", "u", "ka")],
        "enveng_sludge_volume_index" => vec![m::field_half(s, "field", "MLSS", "V30", "SVI")],
        "enveng_weir_loading_rate" => vec![m::field_half(s, "field", "Q", "D", "WL")],
        // one response against one input
        "enveng_activated_sludge_cstr_design" => vec![resp(s, "s-vs-y", "Y", "S")],
        "enveng_air_stripping_tower" => vec![resp(s, "z-vs-qw", "Qw", "Z")],
        "enveng_briggs_plume_rise" => vec![resp(s, "dh-vs-vs", "vs", "dh")],
        "enveng_streeter_phelps_deficit" => vec![resp(s, "do-vs-kd", "kd", "DO")],
        // test data and what was fitted
        "enveng_bod_rate_fit" => vec![data_fig(s, "t", "y", &["k_per_day", "L0_mgL", "rss"], "incubation time t (d)",
                                                "BOD y (mg/L)", None)],
        "enveng_freundlich_fit" => vec![data_fig(s, "Ce", "qe", &["K", "n", "r2"], "equilibrium concentration Ce (mg/L)",
                                                  "loading qe (mg/g)", None)],
        "enveng_langmuir_fit" => vec![data_fig(s, "Ce", "qe", &["qmax", "b", "r2"], "equilibrium concentration Ce (mg/L)",
                                                "loading qe (mg/g)", None)],
        "enveng_kla_regression" => vec![data_fig(s, "t", "C", &["KLa_per_min", "KLa_per_h", "r2"], "time t (min)",
                                                  "dissolved oxygen C (mg/L)", Some(("Cs", 1000.0)))],
        // a verdict along one input, the request as a table
        "enveng_stability_class" => vec![stability(s)],
        "enveng_theoretical_oxygen_demand" => vec![m::io_table(s, "io")],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

/// `y` against `x` from half to one and a half times the given value.
fn resp(s: &Solved, kind: &str, x: &str, y: &str) -> Option<Figure> {
    let x0 = c::inp(s, x)?;
    m::curve1(s, kind, x, fx(x0, 0.5, 1.5), y)
}

/// The HOCl fraction over pKa (near the given one) and pH (half to one and a half times it).
fn hocl_field(s: &Solved) -> Option<Figure> {
    let (pka, ph) = (c::inp(s, "pKa")?, c::inp(s, "pH")?);
    m::field_xy(s, "field", "pKa", fx(pka, 0.8, 1.2), "pH", fx(ph, 0.5, 1.5), "alpha_HOCl")
}

/// Log removal over the counts before and after treatment, a tenth to ten times each (logarithmic axes).
fn log_field(s: &Solved) -> Option<Figure> {
    let (n0, n) = (c::inp(s, "N0")?, c::inp(s, "N")?);
    if !(n0 > 0.0 && n > 0.0) {
        return None;
    }
    m::field_xy(s, "field", "N0", dec(n0), "N", dec(n), "log_removal")
}

/// The data of a fit as given (dots), the returned constants stated over the plot; `level`: an input drawn as a
/// level line (its value times a factor into the plot's unit, e.g. kg/m³ to mg/L).
fn data_fig(s: &Solved, xs: &str, ys: &str, outs: &[&str], xlab: &str, ylab: &str, level: Option<(&str, f64)>)
            -> Option<Figure> {
    let (x, y) = (m::list(s, xs)?, m::list(s, ys)?);
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let mut series = vec![("data (as given)".to_string(), m::dots(&pts))];
    let mut lv_note = String::new();
    if let Some((sym, f)) = level {
        if let Some(v) = c::inp(s, sym) {
            let (lo, hi) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a, b| (a.0.min(*b), a.1.max(*b)));
            series.push((format!("{sym} = {} (input)", ax_engine::figkit::num(v * f)), vec![(lo, v * f), (hi, v * f)]));
            lv_note = format!(" The flat line is the given {sym}.");
        }
    }
    let got: Vec<String> = outs.iter().filter_map(|o| s.out(o).map(|v| format!("{o} = {}", ax_engine::figkit::num(v)))).collect();
    if got.is_empty() {
        return None;
    }
    let header = got.join(", ");
    let title = format!("{}: the data and what was returned", c::name(s));
    let cap = format!("The data {ys} ({}) against {xs} ({}) as given ({} points: {}). Returned by the model: {}.{lv_note}",
                      c::meaning(s, ys), c::meaning(s, xs), pts.len(),
                      pts.iter().map(|p| format!("({}, {})", ax_engine::figkit::num(p.0), ax_engine::figkit::num(p.1)))
                          .collect::<Vec<_>>().join(", "), header);
    // the heading holds what fits beside the plot (about 45 characters); the caption states every value
    let mut head = String::new();
    for g in &got {
        if head.chars().count() + g.chars().count() + 2 > 44 {
            break;
        }
        if !head.is_empty() {
            head.push_str(", ");
        }
        head.push_str(g);
    }
    // a margin on either side so no dot sits on the frame (points without a value draw nothing)
    let (lo, hi) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a, b| (a.0.min(*b), a.1.max(*b)));
    let pad = (hi - lo).max(hi.abs() * 0.1).max(1e-12) * 0.06;
    series[0].1.push((lo - pad, f64::NAN));
    series[0].1.push((hi + pad, f64::NAN));
    let mut f = m::chart(s, "xy", &title, &cap, series, xlab, ylab, &head, None, "", false)?;
    f.svg = f.svg.replace("\" r=\"3\"/>", "\" r=\"4.5\"/>");
    Some(f)
}

/// The stability class along the wind speed, a tenth to ten times the given speed.
fn stability(s: &Solved) -> Option<Figure> {
    let u = c::inp(s, "u")?;
    if !(u > 0.0) {
        return None;
    }
    m::bands_1d(s, "decision", "u", dec(u), "stability")
}
