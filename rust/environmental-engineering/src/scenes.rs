//! Real-world scenes (1.6, owner: "replace graph-heavy figures with real-world visuals", in the spirit of MachineBuild):
//! the plant, river, stack or tower the calculation is about, drawn for the visitor's own inputs. Every number shown is
//! an input, an output, or this model solved again at another point (Solved::with / solve_other); a shape the model
//! does not compute is not drawn to a value (it is a sketch and the caption says so). Colours are see-through fills
//! that read on light and dark pages (water blue, soil brown, plume grey; the site's green and pink for good / poor).

use ax_engine::figkit::{q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

const WATER: &str = "fill=\"#2f8fd8\" fill-opacity=\"0.22\"";
const SOIL: &str = "fill=\"#9a7b4f\" fill-opacity=\"0.30\"";
const PLUME: &str = "fill=\"#8a8794\" fill-opacity=\"0.22\"";
const STEEL: &str = "fill=\"#8a8794\" fill-opacity=\"0.45\"";

fn rect(c: &mut Canvas, x: f64, y: f64, w: f64, h: f64, fill: &str) {
    c.raw(&format!("<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" {fill}/>", w.max(0.5), h.max(0.5)));
}

fn poly(c: &mut Canvas, pts: &[(f64, f64)], fill: &str) {
    let d: Vec<String> = pts.iter().map(|p| format!("{:.1},{:.1}", p.0, p.1)).collect();
    c.raw(&format!("<polygon points=\"{}\" {fill}/>", d.join(" ")));
}

fn arrow(c: &mut Canvas, a: (f64, f64), b: (f64, f64)) {
    c.line(a, b);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l = (dx * dx + dy * dy).sqrt().max(1e-9);
    let (ux, uy) = (dx / l, dy / l);
    let p1 = (b.0 - 8.0 * ux + 4.5 * uy, b.1 - 8.0 * uy - 4.5 * ux);
    let p2 = (b.0 - 8.0 * ux - 4.5 * uy, b.1 - 8.0 * uy + 4.5 * ux);
    c.raw(&format!("<polygon class=\"axp-kd\" points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\"/>", b.0, b.1, p1.0, p1.1, p2.0, p2.1));
}

/// A colour between the site's pink (0) and green (1).
fn good(t: f64) -> String {
    let t = t.clamp(0.0, 1.0);
    let (a, b) = ((0xe0, 0x30, 0x6f), (0x18, 0xa8, 0x74));
    let m = |x: i32, y: i32| (x as f64 + (y - x) as f64 * t).round() as i32;
    format!("#{:02x}{:02x}{:02x}", m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

fn mgl(x: f64) -> String {
    format!("{} mg/L", ax_engine::figkit::num(x * 1000.0))
}

/// The scenes of one solved request.
pub fn scenes(s: &Solved) -> Vec<Figure> {
    let f = match s.key {
        "enveng_briggs_plume_rise" | "enveng_briggs_stable_plume_rise" | "enveng_holland_plume_rise"
        | "enveng_stack_plume_ground_concentration" | "enveng_plume_concentration_stability"
        | "enveng_gaussian_plume_concentration" | "enveng_ground_centerline_concentration"
        | "enveng_max_ground_concentration_briggs" => plume(s),
        "enveng_streeter_phelps_critical_point" | "enveng_streeter_phelps_deficit" | "enveng_oxygen_sag_distance" => river(s),
        "enveng_overflow_rate_circular" | "enveng_weir_loading_rate" | "enveng_sedimentation_basin_sizing" => clarifier(s),
        "enveng_food_to_microorganism_ratio" | "enveng_return_sludge_ratio" | "enveng_solids_retention_time"
        | "enveng_hrt_organic_loading" | "enveng_activated_sludge_cstr_design" | "enveng_sludge_production"
        | "enveng_aeration_oxygen_requirement" => flowsheet(s),
        "enveng_noise_distance_attenuation" | "enveng_sound_power_to_pressure" => noise(s),
        "enveng_air_stripping_tower" => tower(s),
        _ => None,
    };
    f.into_iter().collect()
}

// ------------------------------------------------------------------------------------------------ stack and plume
fn plume(s: &Solved) -> Option<Figure> {
    let hs = s.get("hs");
    let big_h = s.get("H")?;
    let dh = s.get("dh").or_else(|| hs.map(|h| big_h - h)).filter(|d| *d >= 0.0);
    let hs = hs.or_else(|| dh.map(|d| big_h - d)).unwrap_or(big_h);
    let x_r = s.get("x_max").or_else(|| s.get("x")).filter(|x| *x > 0.0);
    let x_end = x_r.map(|x| x * 1.25).unwrap_or(big_h * 12.0).max(big_h * 3.0);
    // plume half-width: the model's own sigma_z along the wind when the stability class is known; else only at x
    let cls = s.txt("cls");
    let terrain = s.txt("terrain").unwrap_or_else(|| "rural".into());
    let mut env: Vec<(f64, f64)> = vec![];
    if let Some(cl) = &cls {
        for i in 1..=24 {
            let x = x_end * i as f64 / 24.0;
            if let Some(a) = s.solve_other("enveng_briggs_dispersion_coefficients",
                                           &[("x", Input::Float(x)), ("cls", Input::Text(cl.clone())),
                                             ("terrain", Input::Text(terrain.clone()))]) {
                if let Some(sz) = ax_engine::figkit::num_of(&a, "sigma_z") {
                    env.push((x, 2.0 * sz));
                }
            }
        }
    }
    let top = (big_h + env.iter().map(|e| e.1).fold(0.0, f64::max).min(big_h * 1.5)).max(big_h * 1.35);
    let (x0, x1, y0, y1) = (70.0, 500.0, 300.0, 40.0);
    let px = |x: f64| x0 + (x1 - x0) * x / x_end;
    let py = |z: f64| y0 - (y0 - y1) * z / top;
    let mut c = Canvas::new(340.0);
    rect(&mut c, 0.0, y0, 520.0, 40.0, SOIL);
    c.line((0.0, y0), (520.0, y0));
    // plume envelope (two sigma_z either side of the centreline), then the centreline
    let rise_x = x_end * 0.08;
    let zc = |x: f64| if x < rise_x { hs + (big_h - hs) * (x / rise_x).powf(2.0 / 3.0) } else { big_h };
    if env.len() >= 3 {
        let mut up: Vec<(f64, f64)> = vec![(px(0.0), py(hs))];
        let mut dn: Vec<(f64, f64)> = vec![(px(0.0), py(hs))];
        for (x, w) in &env {
            up.push((px(*x), py(zc(*x) + w)));
            dn.push((px(*x), py((zc(*x) - w).max(0.0))));
        }
        dn.reverse();
        up.extend(dn);
        poly(&mut c, &up, PLUME);
    }
    let center: Vec<(f64, f64)> = (0..=40).map(|i| { let x = x_end * i as f64 / 40.0; (px(x), py(zc(x))) }).collect();
    c.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"6 4\" d=\"M{}\"/>",
                   center.iter().map(|p| format!("{:.1},{:.1}", p.0, p.1)).collect::<Vec<_>>().join(" L")));
    // the stack
    let sw = 12.0;
    rect(&mut c, px(0.0) - sw / 2.0, py(hs), sw, y0 - py(hs), STEEL);
    c.wire(&[(px(0.0) - sw / 2.0, y0), (px(0.0) - sw / 2.0, py(hs)), (px(0.0) + sw / 2.0, py(hs)), (px(0.0) + sw / 2.0, y0)]);
    c.tag(px(0.0) - 12.0, (py(hs) + y0) / 2.0, "end", "stack", &q(hs, "m"));
    // effective height
    c.dashed((px(0.0) + 30.0, py(big_h)), (x1, py(big_h)));
    c.tag(x1, py(big_h) - 8.0, "end", "effective height H", &q(big_h, "m"));
    if let Some(d) = dh {
        c.tag(px(rise_x) + 8.0, (py(hs) + py(big_h)) / 2.0 + 4.0, "start", "plume rise", &q(d, "m"));
    }
    // receptor
    if let Some(xr) = x_r {
        arrow(&mut c, (px(xr), y0 - 46.0), (px(xr), y0 - 4.0));
        let what = if s.key == "enveng_max_ground_concentration_briggs" { "maximum here" } else { "receptor" };
        let conc = s.get("C_max_ugm3").or_else(|| s.get("C_ugm3")).map(|v| format!("{} µg/m³", ax_engine::figkit::num(v)));
        c.tag(px(xr) + 6.0, y0 - 30.0, "start", what, &conc.unwrap_or_else(|| q(xr, "m")));
        c.name(px(xr), y0 + 16.0, "middle", &q(xr, "m"));
    }
    c.name(x1, y0 + 30.0, "end", "downwind →");
    let title = format!("The stack and its plume: {}", s.p.procedure(s.key).map(|p| p.name.as_str()).unwrap_or(""));
    let cap = format!("Side view. The stack ({}) and the effective height H = {} that the model computes{}; the dashed line \
                       is the plume centreline (its rise is sketched to H). {}Heights and distances are each to scale, but \
                       not to the same scale.",
                      q(hs, "m"), q(big_h, "m"), dh.map(|d| format!(" (rise {})", q(d, "m"))).unwrap_or_default(),
                      if env.len() >= 3 { format!("The grey band is ±2 sigma_z, the model's Briggs coefficient for class {} \
                                                  ({terrain}) at 24 distances. ", cls.unwrap_or_default()) } else { String::new() });
    Some(s.figure("scene", &title, &cap, &c.s, 340.0))
}

// ------------------------------------------------------------------------------------------------ river oxygen sag
fn river(s: &Solved) -> Option<Figure> {
    let (kd, ka, l0, d0, dosat) = (s.get("kd")?, s.get("ka")?, s.get("L0")?, s.get("D0")?, s.get("DOsat")?);
    let base = [("kd", Input::Float(kd)), ("ka", Input::Float(ka)), ("L0", Input::Float(l0)), ("D0", Input::Float(d0)),
                ("DOsat", Input::Float(dosat))];
    let crit = s.solve_other("enveng_streeter_phelps_critical_point", &base);
    let tc = crit.as_ref().and_then(|a| ax_engine::figkit::num_of(a, "t_c")).filter(|t| *t > 0.0);
    let given_t = s.get("t").filter(|t| *t > 0.0);
    let t_end = tc.map(|t| t * 3.0).or(given_t.map(|t| t * 2.0)).unwrap_or(10.0 * 86400.0);
    let v = s.get("v").filter(|v| *v > 0.0);
    let mut pts: Vec<(f64, f64)> = vec![];
    for i in 0..=40 {
        let t = t_end * i as f64 / 40.0;
        let mut set: Vec<(&str, Input)> = base.iter().cloned().collect();
        set.push(("t", Input::Float(t)));
        if let Some(a) = s.solve_other("enveng_streeter_phelps_deficit", &set) {
            if let Some(d) = ax_engine::figkit::num_of(&a, "DO") {
                pts.push((t, d));
            }
        }
    }
    if pts.len() < 5 {
        return None;
    }
    let (x0, x1) = (40.0, 500.0);
    let px = |t: f64| x0 + (x1 - x0) * t / t_end;
    let do_max = dosat.max(1e-9);
    let (gy0, gy1) = (190.0, 50.0);
    let py = |d: f64| gy0 - (gy0 - gy1) * (d / do_max).clamp(0.0, 1.0);
    let mut c = Canvas::new(340.0);
    // the river (top view): banks, water coloured by its dissolved oxygen
    rect(&mut c, 0.0, 220.0, 520.0, 18.0, SOIL);
    rect(&mut c, 0.0, 292.0, 520.0, 18.0, SOIL);
    for w in pts.windows(2) {
        let (a, b) = (w[0], w[1]);
        let col = good(((a.1 + b.1) / 2.0 / do_max - 0.3) / 0.65);
        c.raw(&format!("<rect x=\"{:.1}\" y=\"238\" width=\"{:.1}\" height=\"54\" fill=\"{col}\" fill-opacity=\"0.45\"/>",
                       px(a.0), (px(b.0) - px(a.0) + 0.6).max(0.5)));
    }
    c.line((0.0, 238.0), (520.0, 238.0));
    c.line((0.0, 292.0), (520.0, 292.0));
    arrow(&mut c, (px(0.0), 205.0), (px(0.0), 240.0));
    c.name(px(0.0) + 4.0, 214.0, "start", "outfall");
    arrow(&mut c, (x1 - 60.0, 265.0), (x1 - 10.0, 265.0));
    c.name(x1 - 64.0, 269.0, "end", "flow");
    // DO along the river, above it
    c.dashed((x0, py(dosat)), (x1, py(dosat)));
    c.name(x1, py(dosat) - 6.0, "end", &format!("saturation {}", mgl(dosat)));
    let path: Vec<String> = pts.iter().map(|p| format!("{:.1},{:.1}", px(p.0), py(p.1))).collect();
    c.raw(&format!("<path class=\"axp-c\" d=\"M{}\"/>", path.join(" L")));
    c.name(x0, 40.0, "start", "dissolved oxygen along the river");
    if let (Some(t), Some(a)) = (tc, crit.as_ref()) {
        if let Some(dmin) = ax_engine::figkit::num_of(a, "DO_min") {
            c.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", px(t), py(dmin)));
            let at = v.map(|v| q(v * t, "m")).unwrap_or_else(|| format!("{} d", ax_engine::figkit::num(t / 86400.0)));
            c.tag(px(t) + 8.0, py(dmin) + 18.0, "start", "lowest oxygen", &format!("{} at {at}", mgl(dmin)));
        }
    }
    if let Some(t) = given_t {
        if let Some(d) = s.get("DO") {
            c.raw(&format!("<circle class=\"axp-d2\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", px(t), py(d)));
            c.value(px(t), py(d) - 10.0, "middle", &mgl(d));
        }
    }
    let axis = v.map(|v| format!("distance downstream to {}", q(v * t_end, "m")))
        .unwrap_or_else(|| format!("travel time to {} days", ax_engine::figkit::num(t_end / 86400.0)));
    c.name(260.0, 330.0, "middle", &axis);
    let cap = format!("The river below the outfall, seen from above, coloured by its dissolved oxygen (green near \
                       saturation, pink where it is lowest), with the oxygen curve above it: 41 points along the river, each \
                       the model's Streeter-Phelps deficit solved again (kd = {}/d, ka = {}/d, L0 = {}, D0 = {}).",
                      ax_engine::figkit::num(kd * 86400.0), ax_engine::figkit::num(ka * 86400.0), mgl(l0), mgl(d0));
    Some(s.figure("scene", "The river's oxygen sag below the outfall", &cap, &c.s, 340.0))
}

// ------------------------------------------------------------------------------------------------ clarifier
fn clarifier(s: &Solved) -> Option<Figure> {
    let d = s.get("D").filter(|d| *d > 0.0)?;
    let mut c = Canvas::new(340.0);
    let (cx, cy, r) = (170.0, 175.0, 130.0);
    c.raw(&format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" {WATER}/>"));
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\"/>"));
    c.raw(&format!("<circle class=\"axp-c2\" cx=\"{cx}\" cy=\"{cy}\" r=\"{}\" stroke-dasharray=\"3 3\"/>", r - 8.0));
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{cx}\" cy=\"{cy}\" r=\"{}\"/>", r * 0.16));
    for k in 0..8 {
        let a = std::f64::consts::PI * 2.0 * k as f64 / 8.0;
        let (ux, uy) = (a.cos(), a.sin());
        arrow(&mut c, (cx + ux * r * 0.22, cy + uy * r * 0.22), (cx + ux * r * 0.78, cy + uy * r * 0.78));
    }
    c.name(cx, cy + 4.0, "middle", "feed");
    c.line((cx - r, cy + r + 18.0), (cx + r, cy + r + 18.0));
    c.value(cx, cy + r + 14.0, "middle", &format!("D = {}", q(d, "m")));
    let mut y = 70.0;
    let mut row = |c: &mut Canvas, name: &str, v: String| {
        c.tag(330.0, y, "start", name, &v);
        y += 44.0;
    };
    if let Some(qf) = s.get("Q") {
        row(&mut c, "flow", format!("{} m³/d", ax_engine::figkit::num(qf * 86400.0)));
    }
    if let Some(a) = s.get("A") {
        row(&mut c, "surface area", q(a, "m²"));
    }
    if let Some(v0) = s.get("v0_m_per_day").or_else(|| s.get("v0").map(|v| v * 86400.0)) {
        row(&mut c, "overflow rate", format!("{} m/d", ax_engine::figkit::num(v0)));
    }
    if let Some(wl) = s.get("WL_m3_per_m_day") {
        row(&mut c, "weir loading", format!("{} m³/(m·d)", ax_engine::figkit::num(wl)));
    }
    if let Some(dep) = s.get("depth") {
        row(&mut c, "side water depth", q(dep, "m"));
    }
    if let Some(l) = s.get("weir_length") {
        row(&mut c, "weir length (green)", q(l, "m"));
    }
    let cap = "A circular clarifier from above: water enters at the centre feed well, flows outward (arrows) as particles \
               settle, and leaves over the peripheral weir (green). The circle is the diameter given or computed; the \
               numbers beside it are the model's.".to_string();
    Some(s.figure("scene", "The clarifier, seen from above", &cap, &c.s, 340.0))
}

// ------------------------------------------------------------------------------------------------ activated sludge
fn flowsheet(s: &Solved) -> Option<Figure> {
    let mut c = Canvas::new(300.0);
    // influent -> aeration tank -> clarifier -> effluent; return sludge back; waste sludge out
    rect(&mut c, 120.0, 70.0, 150.0, 90.0, WATER);
    c.wire(&[(120.0, 70.0), (120.0, 160.0), (270.0, 160.0), (270.0, 70.0)]);
    for k in 0..6 {
        let x = 135.0 + k as f64 * 24.0;
        c.raw(&format!("<circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"150\" r=\"2.5\"/><circle class=\"axp-k\" cx=\"{:.1}\" \
                        cy=\"{:.1}\" r=\"2\"/>", x + 6.0, 128.0 - (k % 3) as f64 * 14.0));
    }
    c.bold(195.0, 62.0, "middle", "aeration tank");
    c.raw(&format!("<circle cx=\"380\" cy=\"115\" r=\"42\" {WATER}/>"));
    c.raw("<circle class=\"axp-k\" cx=\"380\" cy=\"115\" r=\"42\"/>");
    c.bold(380.0, 62.0, "middle", "clarifier");
    arrow(&mut c, (20.0, 115.0), (118.0, 115.0));
    arrow(&mut c, (272.0, 115.0), (336.0, 115.0));
    arrow(&mut c, (424.0, 115.0), (510.0, 115.0));
    c.wire(&[(380.0, 158.0), (380.0, 220.0), (195.0, 220.0)]);
    arrow(&mut c, (195.0, 220.0), (195.0, 163.0));
    arrow(&mut c, (380.0, 220.0), (380.0, 270.0));
    c.name(20.0, 100.0, "start", "influent");
    c.name(510.0, 100.0, "end", "effluent");
    c.name(205.0, 236.0, "start", "return sludge");
    c.name(388.0, 262.0, "start", "waste sludge");
    let f = |k: &str| s.get(k);
    let per_day = |x: f64| format!("{} m³/d", ax_engine::figkit::num(x * 86400.0));
    if let Some(v) = f("Q") { c.value(20.0, 134.0, "start", &per_day(v)); }
    if let Some(v) = f("S0") { c.value(20.0, 150.0, "start", &format!("BOD {}", mgl(v))); }
    if let Some(v) = f("V") { c.value(195.0, 186.0, "middle", &format!("V = {}", q(v, "m³"))); }
    if let Some(v) = f("X") { c.value(195.0, 202.0, "middle", &format!("MLSS {}", mgl(v))); }
    if let Some(v) = f("S").or_else(|| f("S_mgL").map(|x| x / 1000.0)) { c.value(510.0, 134.0, "end", &format!("BOD {}", mgl(v))); }
    if let Some(v) = f("Qr") { c.value(205.0, 252.0, "start", &per_day(v)); }
    if let Some(v) = f("R") { c.value(205.0, 268.0, "start", &format!("R = {}", ax_engine::figkit::num(v))); }
    if let Some(v) = f("Qw") { c.value(388.0, 278.0, "start", &per_day(v)); }
    if let Some(v) = f("Xr") { c.value(388.0, 294.0, "start", &format!("{}", mgl(v))); }
    let mut tags: Vec<String> = vec![];
    if let Some(v) = f("FM_per_day") { tags.push(format!("F/M = {} /d", ax_engine::figkit::num(v))); }
    if let Some(v) = f("SRT_days").or_else(|| f("SRT").map(|x| x / 86400.0)) { tags.push(format!("SRT = {} d", ax_engine::figkit::num(v))); }
    if let Some(v) = f("HRT_h").or_else(|| f("HRT").map(|x| x / 3600.0)) { tags.push(format!("HRT = {} h", ax_engine::figkit::num(v))); }
    if let Some(v) = f("Px_kg_per_day") { tags.push(format!("sludge {} kg/d", ax_engine::figkit::num(v))); }
    if let Some(v) = f("O2_kg_per_day") { tags.push(format!("oxygen {} kg/d", ax_engine::figkit::num(v))); }
    for (i, t) in tags.iter().enumerate() {
        c.value(20.0, 22.0 + i as f64 * 16.0, "start", t);
    }
    let cap = "The activated-sludge process as a flow sheet: influent into the aeration tank, mixed liquor to the clarifier, \
               clear effluent out, settled sludge returned to the tank or wasted. The numbers are the inputs and results of \
               this calculation, written where they belong; the drawing itself is not to scale.".to_string();
    Some(s.figure("scene", "Where these numbers sit in the plant", &cap, &c.s, 300.0))
}

// ------------------------------------------------------------------------------------------------ noise
fn noise(s: &Solved) -> Option<Figure> {
    let (dist, lvl, out_sym) = if s.key == "enveng_noise_distance_attenuation" {
        (s.get("r2")?, s.get("L2")?, "L2")
    } else {
        (s.get("r")?, s.get("Lp")?, "Lp")
    };
    let rsym = if out_sym == "L2" { "r2" } else { "r" };
    let r_max = dist * 1.6;
    let mut c = Canvas::new(320.0);
    let (sx, sy) = (40.0, 300.0);
    let scale = 440.0 / r_max;
    rect(&mut c, 0.0, sy, 520.0, 20.0, SOIL);
    let mut rings: Vec<(f64, f64)> = vec![];
    for k in 1..=5 {
        let r = r_max * k as f64 / 5.0;
        if let Some(a) = s.with(&[(rsym, r)]) {
            if let Some(l) = ax_engine::figkit::num_of(&a, out_sym) {
                rings.push((r, l));
            }
        }
    }
    let lmax = rings.iter().map(|x| x.1).fold(lvl, f64::max);
    let lmin = rings.iter().map(|x| x.1).fold(lvl, f64::min);
    for (r, l) in &rings {
        let t = if lmax > lmin { (l - lmin) / (lmax - lmin) } else { 0.5 };
        let col = good(1.0 - t);
        c.raw(&format!("<path d=\"M{:.1},{sy} A{:.1},{:.1} 0 0 1 {:.1},{:.1}\" fill=\"none\" stroke=\"{col}\" stroke-width=\"2.2\"/>",
                       sx + r * scale, r * scale, r * scale, sx, sy - r * scale));
        c.value(sx + r * scale * 0.72, sy - r * scale * 0.72 - 4.0, "start", &format!("{} dB", ax_engine::figkit::num(*l)));
    }
    c.raw(&format!("<circle class=\"axp-kd\" cx=\"{sx}\" cy=\"{sy}\" r=\"7\"/>"));
    c.name(sx + 10.0, sy - 10.0, "start", "source");
    arrow(&mut c, (sx + dist * scale, sy - 40.0), (sx + dist * scale, sy - 4.0));
    c.tag(sx + dist * scale + 6.0, sy - 24.0, "start", "you are here", &format!("{} dB at {}", ax_engine::figkit::num(lvl), q(dist, "m")));
    let cap = format!("The sound level around the source, as rings at five distances up to {}: each ring's level is the model \
                       solved again at that distance (pink louder, green quieter); the arrow is the point asked about.",
                      q(r_max, "m"));
    Some(s.figure("scene", "How loud it is around the source", &cap, &c.s, 320.0))
}

// ------------------------------------------------------------------------------------------------ stripping tower
fn tower(s: &Solved) -> Option<Figure> {
    let z = s.get("Z").filter(|z| *z > 0.0)?;
    let person = 1.8;
    let top = (z * 1.12).max(person * 1.3);
    let (gy, ty) = (300.0, 30.0);
    let py = |h: f64| gy - (gy - ty) * h / top;
    let mut c = Canvas::new(340.0);
    rect(&mut c, 0.0, gy, 520.0, 30.0, SOIL);
    c.line((0.0, gy), (520.0, gy));
    let (tx, tw) = (180.0, 70.0);
    rect(&mut c, tx, py(z), tw, gy - py(z), WATER);
    c.wire(&[(tx, gy), (tx, py(z)), (tx + tw, py(z)), (tx + tw, gy)]);
    let mut k = py(z) + 10.0;
    while k < gy - 6.0 {
        c.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{k:.1} L{:.1},{:.1}\"/>", tx + 4.0, tx + tw - 4.0, (k + 8.0).min(gy - 2.0)));
        k += 12.0;
    }
    arrow(&mut c, (tx - 70.0, py(z) + 14.0), (tx - 2.0, py(z) + 14.0));
    c.tag(tx - 74.0, py(z) + 8.0, "end", "water in", &s.get("Cin").map(|v| format!("{} µg/L", ax_engine::figkit::num(v * 1e6))).unwrap_or_default());
    arrow(&mut c, (tx + 2.0, gy - 14.0), (tx - 70.0, gy - 14.0));
    c.tag(tx - 74.0, gy - 20.0, "end", "water out", &s.get("Cout").map(|v| format!("{} µg/L", ax_engine::figkit::num(v * 1e6))).unwrap_or_default());
    arrow(&mut c, (tx + tw + 70.0, gy - 14.0), (tx + tw + 2.0, gy - 14.0));
    c.name(tx + tw + 74.0, gy - 10.0, "start", "air in");
    arrow(&mut c, (tx + tw / 2.0, py(z) - 2.0), (tx + tw / 2.0, (py(z) - 26.0).max(ty - 10.0)));
    c.name(tx + tw / 2.0 + 8.0, (py(z) - 14.0).max(18.0), "start", "air out");
    // a person for scale
    let (hx, ph) = (400.0, py(0.0) - py(person));
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{hx}\" cy=\"{:.1}\" r=\"{:.1}\"/>", gy - ph + ph * 0.09, (ph * 0.09).max(1.5)));
    c.wire(&[(hx, gy - ph * 0.82), (hx, gy - ph * 0.42)]);
    c.wire(&[(hx - ph * 0.12, gy), (hx, gy - ph * 0.42), (hx + ph * 0.12, gy)]);
    c.wire(&[(hx - ph * 0.16, gy - ph * 0.66), (hx, gy - ph * 0.74), (hx + ph * 0.16, gy - ph * 0.66)]);
    c.name(hx, gy + 16.0, "middle", "1.8 m person");
    c.tag(tx + tw + 12.0, (py(z) + gy) / 2.0, "start", "packing height Z", &q(z, "m"));
    let mut info = vec![];
    if let Some(r) = s.get("R") { info.push(format!("stripping factor R = {}", ax_engine::figkit::num(r))); }
    if let Some(n) = s.get("NTU") { info.push(format!("transfer units NTU = {}", ax_engine::figkit::num(n))); }
    for (i, t) in info.iter().enumerate() {
        c.value(10.0, 20.0 + i as f64 * 16.0, "start", t);
    }
    let cap = format!("The packed tower to scale beside a 1.8 m person: water flows down through the packing, air is blown \
                       up through it and carries the volatile compound away. Packing height Z = {} as computed.", q(z, "m"));
    Some(s.figure("scene", "The stripping tower, to scale", &cap, &c.s, 340.0))
}
