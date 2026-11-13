"""Environmental engineering procedures: deterministic capabilities with typed inputs and several outputs (domain
code, loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: air pollution (Pasquill stability classes, Briggs dispersion coefficients, the Gaussian plume with ground
reflection and with a mixing lid, maximum ground-level concentrations, Briggs and Holland plume rise, wind profiles,
ppm and mass concentration, the box model, settling chambers, cyclones, electrostatic precipitators, sulfur dioxide
emissions); water quality (BOD kinetics, temperature correction, the dilution test, a least-squares rate fit,
theoretical oxygen demand, the Streeter-Phelps oxygen sag and its critical point, reaeration formulas, dissolved
oxygen saturation, mixing of streams, lakes as stirred tanks, first-order decay in rivers, pollutant loads);
water treatment (detention time, overflow and weir loading, basin sizing, Stokes and general settling, ideal-basin
removal, filtration rate, Ergun clean-bed head loss, backwash expansion, velocity gradient, disinfection CT, Chick and
Chick-Watson inactivation, chlorine dosing, hypochlorous acid fraction, hardness, adsorption isotherms and their
fits, membranes and osmotic pressure); activated sludge (F/M, SRT, HRT and organic loading, Monod kinetics, the
Lawrence-McCarty stirred-tank design, minimum SRT, sludge production, oxygen demand, return ratio, SVI, sludge
volume and thickening, methane yield, the NRC trickling filter, field oxygen transfer, KLa from tests and fits);
gas transfer (Henry's constant conversions, Henry's law, van 't Hoff temperature dependence, two-film theory, air
stripping towers); noise (decibel addition and subtraction, distance attenuation, Leq, Ldn/Lden, sound power,
noise dose); solid waste (generation, landfill life, landfill gas generation, Dulong heating value, moisture).
Batch 1 (2026-11-11): pH and pOH, Henry's law in its solubility form, channel (grit chamber) velocity, landfill
compaction ratio, retardation factor and contaminant velocity, Darcy's law and seepage velocity, chronic daily intake
with hazard quotient and cancer risk, mean cell residence time from solids masses, contact time from a required CT,
filter flow from the loading rate, stoichiometric combustion oxygen, stability from the environmental lapse rate.
Batch 2 (2026-11-11): total alkalinity as CaCO3, first-order stirred tanks in series with the plug-flow limit, Manning
and Hazen-Williams (SI), pump power and energy cost, the rational method, the indoor-air box model, collection devices
in series, the dry-basis heating value, return period and risk, the geometric mean of counts, volatile solids
reduction (Van Kleeck), transmembrane pressure, the theoretical oxygen demand of a solution in mg/L.
Batch 3 (2026-11-11): first-order decay in a batch reactor (C, C0, k or t; half-life), the chemical feed rate (kg/d and
lb/d, flows in MGD read), the membrane permeate flux (LMH) or area, the landfill volume per day and year with cover and
its life; one CMFR from V and Q, the SRT with the effluent solids optional, exam forms of the ultimate BOD and of the
pump's input (brake, motor) power.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported
inside the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run,
verify, parse) is plain Python (math, re).

How the references differ from the shipped code:
- normal densities (scipy.stats.norm) for every Gaussian plume; the Poisson-summed (Jacobi theta, mpmath) form of the
  image series under a mixing lid; sympy derivatives plus scipy brentq for maxima and critical points;
- ODE integration (scipy DOP853) for every first-order decay, BOD exertion, the Streeter-Phelps pair, lakes, box
  models, reaeration curves, Chick-Watson inactivation, the electrostatic precipitator and van 't Hoff;
- fluids (v_terminal for Stokes and the Rouse drag law, Ergun for clean-bed head loss); chemicals (ideal-gas molar
  volume, atomic weights and formula parsing, Henry_converter); scipy logsumexp for decibel sums; scipy linregress for
  isotherm and KLa fits; scipy least_squares for the BOD rate fit; scipy quad for the stripping-tower transfer units
  and landfill gas; numpy linear solves of mass balances; scipy brentq for steady states and inverse problems.
- weaker references (the same published formula evaluated a different way) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- SI internally (concentrations kg/m^3 = g/L, rate constants 1/s, times s); companion outputs give the customary
  units (mg/L, 1/d, h, d, ug/m^3). Temperatures are read in K or degC and used in K; "T - 20" means T - 293.15 K.
- g = 9.80665 m/s^2, R = 8.31446261815324 J/(mol K), 1 atm = 101325 Pa, water molar density 55344.59 mol/m^3 (the
  chemicals library's value), atomic weights of the chemicals library (C 12.0107, H 1.00794, O 15.9994, N 14.0067,
  S 32.065, Ca 40.078, Mg 24.305).
- Gaussian plume: steady state, flat terrain, full ground reflection; y crosswind, z height; Briggs (1973) open-country
  and urban dispersion coefficients for 100 m to 10 km; Pasquill-Turner stability table (surface wind at 10 m).
- Briggs plume rise: final rise, neutral/unstable 2/3 law at x_f = 49 F^(5/8) (F < 55 m^4/s^3) or 119 F^(2/5);
  stable 2.6 (F/(u s))^(1/3). Holland: pressure in mbar inside the formula.
- BOD: first order, base e; Streeter-Phelps with ka = kd handled by its limit; deficits never above saturation.
- Lists carry no unit: the unit of each list is stated in its input (days and mg/L for BOD data, minutes and mg/L for
  KLa tests, mg/L and mg/g for isotherms, dB for levels, hours for exposure).
- Results built from exp, log or roots are rounded to 12 significant digits and exact zeros are written as 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 20 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L`).
Physical values carry a unit; fractions may be written with %; levels with dB; lists in brackets; choices as words.
The shared unit reader has no "d" for days: it is read here as a day. Any other number, an unknown name or a name
given twice is refused, never guessed.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ constants
G0 = 9.80665                      # m/s^2
R_GAS = 8.31446261815324          # J/(mol K)
P_ATM = 101325.0                  # Pa
T0C = 273.15                      # K
RHO_W_MOLAR = 55344.59            # mol/m^3, water (chemicals' convention)
DAY = 86400.0
HOUR = 3600.0
YEAR = 365.25 * 86400.0           # julian year, s (the unit reader's "yr")
MG = 1e-3                         # 1 mg/L in kg/m^3
AW = {"C": 12.0107, "H": 1.00794, "O": 15.9994, "N": 14.0067, "S": 32.065, "Ca": 40.078, "Mg": 24.305}
MW_O2 = 2 * AW["O"]               # g/mol


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(*vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _frac(name, v, lo_open=False, hi_open=False):
    _fin(v)
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1),
          f"{name} must be a fraction between 0 and 1")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _bisect(f, a, b, n=500):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _ivp(fun, t_end, y0, rtol=1e-13, atol=None):
    """Reference helper: scipy DOP853 integration from 0 to t_end; returns the final state."""
    from scipy.integrate import solve_ivp
    if atol is None:
        atol = [1e-18 * max(1.0, abs(v)) for v in y0]
    sol = solve_ivp(fun, (0.0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol)
    if not sol.success:
        raise RuntimeError(sol.message)
    return [float(v) for v in sol.y[:, -1]]


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _log10(x):
    return math.log10(x)


# ================================================================================================ units
_KU = {"mass": "kg", "mass_flow_rate": "kg/s", "temperature": "K", "temperature_difference": "K", "pressure": "Pa",
       "volume": "m^3", "flow_rate": "m^3/s", "time": "s", "length": "m", "area": "m^2", "speed": "m/s",
       "density": "kg/m^3", "dynamic_viscosity": "Pa*s", "mass_concentration": "kg/m^3",
       "first_order_rate_constant": "1/s", "molar_mass": "kg/mol", "molar_concentration": "mol/m^3",
       "mass_flux": "kg/(m^2*s)", "concentration_time_integral": "kg*s/m^3", "temperature_gradient": "K/m",
       "power": "W", "henry_constant": "mol/(m^3*Pa)", "henry_volatility": "Pa*m^3/mol",
       "specific_volume": "m^3/kg", "specific_energy": "J/kg", "volume_transport_per_width": "m^2/s",
       "velocity_gradient": "1/s", "volumetric_loading": "kg/(m^3*s)", "food_to_microorganism_ratio": "1/s",
       "volumetric_transfer_rate": "kg/(m^3*s)",
       "number": ""}

# how a value of a kind is written in templates and questions (unit, factor from SI); SI when absent
_SHOW = {"mass_concentration": ("mg/L", 1e3), "first_order_rate_constant": ("1/d", DAY),
         "concentration_time_integral": ("mg*min/L", 1.0 / 0.06), "dynamic_viscosity": ("Pa*s", 1.0),
         "food_to_microorganism_ratio": ("1/d", DAY), "volumetric_loading": ("kg/(m^3*d)", DAY)}

_FAHRENHEIT = ("°F", "degF", "deg F", "F°")
_DAYRX = re.compile(r"(?<![A-Za-z])d(?![A-Za-z])")
_TONNE = re.compile(r"(?<![A-Za-z])tonnes?(?![A-Za-z])")
US_GAL = 0.003785411784           # m^3 per US gallon (exact)
# flows written in US gallons: (m^3 per unit written, the unit it is rewritten to)
_GALLON_FLOW = {"MGD": (3785.411784, "m^3/day"), "mgd": (3785.411784, "m^3/day"), "Mgal/d": (3785.411784, "m^3/day"),
                "Mgal/day": (3785.411784, "m^3/day"), "gpd": (US_GAL, "m^3/day"), "gal/d": (US_GAL, "m^3/day"),
                "gal/day": (US_GAL, "m^3/day"), "gpm": (US_GAL, "m^3/min"), "gal/min": (US_GAL, "m^3/min")}


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime: 'd' (day) and 'tonne' are rewritten here for the shared unit reader."""
    v = float(num)
    u = unit.strip()
    if kind in ("temperature", "temperature_difference"):
        if u in _FAHRENHEIT:
            k = (v - 32.0) * 5.0 / 9.0 + T0C if kind == "temperature" else v * 5.0 / 9.0
            return f"{k!r} K"
        if u == "R" or u == "°R":
            return None
    if kind == "area" and u in ("ha", "hectare", "hectares", "acre", "acres"):     # not in the shared unit reader
        return f"{v * (1e4 if u.startswith('h') else 4046.8564224)!r} m^2"
    if kind == "flow_rate" and u in _GALLON_FLOW:                                  # US gallons: not in the shared reader
        return f"{v * _GALLON_FLOW[u][0]!r} {_GALLON_FLOW[u][1]}"
    if kind == "speed" and u in ("LMH", "lmh"):                                    # membrane flux, L/(m^2 h)
        return f"{num} L/(m^2*h)"
    u = _DAYRX.sub("day", u)
    u = _TONNE.sub("t", u)
    return f"{num} {u}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard is built in: every stated number is bound to a named input (any other text or number refuses
    the whole question); the capability phrase must open the question and be followed by the colon; a physical value
    without its unit, or a unit on a pure number, is refused; values are passed with their units so the runtime
    converts each exactly once."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") == "choice":
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                    continue
                if key is not None and by[key].get("type") == "text":
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip()
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "choice", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                w = _written_unit(num, unit, kind)
                if w is None:
                    return None
                out[key] = w
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                if unit in spec.get("words", ()):
                    out[key] = float(num)
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    return f"{x:.10g}"


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        unit, f = spec.get("show") or _SHOW.get(kind) or (_KU[kind], 1.0)
        if unit == "degC":
            return f"{_fmt(float(v) - T0C)} degC"
        return f"{_fmt(float(v) * f)} {unit}"
    if spec.get("words"):
        return f"{_fmt(float(v))} {spec['words'][0]}"
    return _fmt(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, show=None):
    _KU[kind]                                   # every physical kind has its SI unit here
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if show:
        d["show"] = show
    return d


def _numin(sym, meaning, required=True, pct=False, words=None):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    if words:
        d["words"] = list(words)
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _textin(sym, meaning, required=True):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": outputs or {}}


def _norm_pdf(x, mu, s):
    from scipy.stats import norm
    return float(norm.pdf(x, loc=mu, scale=s))


# ================================================================================================ AIR: stability
_STAB_DAY = ("strong", "moderate", "slight")
_STAB_ALL = ("strong", "moderate", "slight", "overcast", "night_cloudy", "night_clear")


def stability_class(u, condition):
    _pos("the surface wind speed u", u)
    _need(condition in _STAB_ALL, "condition: strong, moderate or slight insolation, overcast, night_cloudy or night_clear")
    if condition == "overcast":
        return _out(stability="D")
    if u < 2.0:
        row = ("A", "A-B", "B", "", "")
    elif u < 3.0:
        row = ("A-B", "B", "C", "E", "F")
    elif u < 5.0:
        row = ("B", "B-C", "C", "D", "E")
    elif u < 6.0:
        row = ("C", "C-D", "D", "D", "D")
    else:
        row = ("C", "D", "D", "D", "D")
    col = {"strong": 0, "moderate": 1, "slight": 2, "night_cloudy": 3, "night_clear": 4}[condition]
    s = row[col]
    _need(s != "", "the table gives no class for a night with a surface wind below 2 m/s")
    return _out(stability=s)


def _ref_stability_class(u, condition):
    import bisect as _bs
    if condition == "overcast":
        return {"stability": "D"}
    table = {"strong": ["A", "A-B", "B", "C", "C"], "moderate": ["A-B", "B", "B-C", "C-D", "D"],
             "slight": ["B", "C", "C", "D", "D"], "night_cloudy": [None, "E", "D", "D", "D"],
             "night_clear": [None, "F", "E", "D", "D"]}
    i = _bs.bisect_right([2.0, 3.0, 5.0, 6.0], u)
    return {"stability": table[condition][i]}


def _wrong_stability_class(u, condition):
    swap = {"strong": "slight", "slight": "strong", "night_cloudy": "night_clear", "night_clear": "night_cloudy"}
    return stability_class(u, swap.get(condition, condition))


def _rand_stability(rng):
    return {"u": _sig(rng.uniform(0.5, 7.0), 3), "condition": rng.choice(["strong", "slight", "night_cloudy", "night_clear",
                                                                          "strong", "slight", "moderate"])}


_add("enveng_stability_class",
     name="Pasquill-Turner atmospheric stability class", topic="air pollution / atmospheric stability",
     purpose="The Pasquill stability class (A very unstable to F stable) from the surface wind speed and the daytime insolation or night-time cloud cover.",
     inputs=[_phys("u", "speed", "surface wind speed at 10 m"),
             _choice("condition", _STAB_ALL, "daytime insolation (strong, moderate, slight), overcast (day or night), night_cloudy (at least 4/8 low cloud) or night_clear (at most 3/8 cloud)")],
     outputs=[_o("stability", "number", "Pasquill stability class (A-F; A-B etc. for intermediate cases)")],
     primary="stability", conditions=["u > 0", "no class is defined at night with u below 2 m/s"],
     method="Turner's table: wind-speed rows < 2, 2-3, 3-5, 5-6, > 6 m/s against insolation or cloud-cover columns; heavy overcast is D",
     latex=r"\text{class} = T(u,\ \text{insolation or cloud cover})",
     run=stability_class, reference=_ref_stability_class, wrong=_wrong_stability_class, rand=_rand_stability,
     phrases=("Pasquill stability class", "Atmospheric stability class", "Stability class"),
     examples=[_ex({"u": 2.5, "condition": "strong"}, {"stability": "A-B"}),
               _ex({"u": 4.0, "condition": "night_clear"}, {"stability": "E"})],
     level="beginner",
     extra_q=[{"q": "Stability class: u = 1.5 m/s, condition = night_clear", "outcome": "INVALID_INPUT"},
              {"q": "Atmospheric stability class: u = 9 km/h, condition = strong", "like": 0}],
     weak="an independent encoding of the same published table (column lists and bisect)")


# ================================================================================================ AIR: Briggs sigmas
# sigma = a x (1 + b x)^p, x in metres (Briggs 1973, as tabulated by Gifford and Hanna)
_BRIGGS_RURAL = {"a": ((0.22, 1e-4, -0.5), (0.20, 0.0, 1.0)), "b": ((0.16, 1e-4, -0.5), (0.12, 0.0, 1.0)),
                 "c": ((0.11, 1e-4, -0.5), (0.08, 2e-4, -0.5)), "d": ((0.08, 1e-4, -0.5), (0.06, 1.5e-3, -0.5)),
                 "e": ((0.06, 1e-4, -0.5), (0.03, 3e-4, -1.0)), "f": ((0.04, 1e-4, -0.5), (0.016, 3e-4, -1.0))}
_BRIGGS_URBAN = {"a": ((0.32, 4e-4, -0.5), (0.24, 1e-3, 0.5)), "b": ((0.32, 4e-4, -0.5), (0.24, 1e-3, 0.5)),
                 "c": ((0.22, 4e-4, -0.5), (0.20, 0.0, 1.0)), "d": ((0.16, 4e-4, -0.5), (0.14, 3e-4, -0.5)),
                 "e": ((0.11, 4e-4, -0.5), (0.08, 1.5e-3, -0.5)), "f": ((0.11, 4e-4, -0.5), (0.08, 1.5e-3, -0.5))}
_CLASSES = ("a", "b", "c", "d", "e", "f")


def _briggs_coef(cls, terrain):
    _need(cls in _CLASSES, "the stability class must be one of A to F")
    _need(terrain in ("rural", "urban"), "terrain must be rural or urban")
    return (_BRIGGS_RURAL if terrain == "rural" else _BRIGGS_URBAN)[cls]


def _sig_eval(c, x):
    a, b, p = c
    return a * x * (1.0 + b * x) ** p


def _briggs(x, cls, terrain):
    _pos("the downwind distance x", x)
    _need(100.0 <= x <= 10000.0, "the Briggs coefficients hold for x between 100 m and 10 km")
    cy, cz = _briggs_coef(cls, terrain)
    return _sig_eval(cy, x), _sig_eval(cz, x)


def briggs_sigmas(x, cls, terrain):
    sy, sz = _briggs(x, cls, terrain)
    return _out(sigma_y=sy, sigma_z=sz)


def _ref_briggs(x, cls, terrain):
    import numpy as np
    rural = {"a": [0.22, 1e-4, 0.20, 0.0, 0], "b": [0.16, 1e-4, 0.12, 0.0, 0], "c": [0.11, 1e-4, 0.08, 2e-4, 1],
             "d": [0.08, 1e-4, 0.06, 1.5e-3, 1], "e": [0.06, 1e-4, 0.03, 3e-4, 2], "f": [0.04, 1e-4, 0.016, 3e-4, 2]}
    urban = {"a": [0.32, 4e-4, 0.24, 1e-3, 3], "b": [0.32, 4e-4, 0.24, 1e-3, 3], "c": [0.22, 4e-4, 0.20, 0.0, 0],
             "d": [0.16, 4e-4, 0.14, 3e-4, 1], "e": [0.11, 4e-4, 0.08, 1.5e-3, 1], "f": [0.11, 4e-4, 0.08, 1.5e-3, 1]}
    ay, by, az, bz, form = (rural if terrain == "rural" else urban)[cls]
    sy = ay * x / np.sqrt(1.0 + by * x)
    sz = {0: az * x, 1: az * x / np.sqrt(1.0 + bz * x), 2: az * x / (1.0 + bz * x), 3: az * x * np.sqrt(1.0 + bz * x)}[form]
    return {"sigma_y": float(sy), "sigma_z": float(sz)}


def _rand_briggs(rng):
    return {"x": _sig(_lu(rng, 100.0, 10000.0), 4), "cls": rng.choice(_CLASSES), "terrain": rng.choice(["rural", "urban"])}


_add("enveng_briggs_dispersion_coefficients",
     name="Briggs dispersion coefficients sigma_y and sigma_z", topic="air pollution / dispersion coefficients",
     purpose="The horizontal and vertical plume spreads (Pasquill-Gifford curves in Briggs' formulas) at a downwind distance for a stability class, open-country or urban.",
     inputs=[_phys("x", "length", "downwind distance from the source"),
             _choice("cls", _CLASSES, "Pasquill stability class A-F (urban: A and B share A-B, E and F share E-F)"),
             _choice("terrain", ("rural", "urban"), "rural (open country) or urban coefficients")],
     outputs=[_o("sigma_y", "length", "horizontal (crosswind) dispersion coefficient"),
              _o("sigma_z", "length", "vertical dispersion coefficient")],
     primary="sigma_y", conditions=["100 m <= x <= 10 km"],
     method="Briggs (1973) interpolation formulas sigma = a x (1 + b x)^p for each class and terrain",
     latex=r"\sigma_y = \frac{a_y x}{\sqrt{1 + b_y x}},\quad \sigma_z = a_z x\,(1 + b_z x)^{p_z}",
     run=briggs_sigmas, reference=_ref_briggs,
     wrong=lambda x, cls, terrain: (lambda c: _out(sigma_y=c[0][0] * x, sigma_z=c[1][0] * x))(_briggs_coef(cls, terrain)),
     rand=_rand_briggs,
     phrases=("Briggs dispersion coefficients", "Pasquill-Gifford dispersion coefficients", "Dispersion coefficients"),
     aliases={"class": "cls", "stability": "cls"},
     examples=[_ex({"x": 1000.0, "cls": "d", "terrain": "rural"}), _ex({"x": 2500.0, "cls": "b", "terrain": "urban"})],
     level="intermediate",
     extra_q=[{"q": "Dispersion coefficients: x = 1 km, class = D, terrain = rural", "like": 0},
              {"q": "Briggs dispersion coefficients: x = 50 m, cls = d, terrain = rural", "outcome": "INVALID_INPUT"}],
     weak="the same published coefficients evaluated in numpy from a separately written table")


# ================================================================================================ AIR: Gaussian plume
def _plume(Q, u, sy, sz, H, y, z):
    return (Q / (2.0 * math.pi * u * sy * sz) * math.exp(-y * y / (2.0 * sy * sy))
            * (math.exp(-(z - H) ** 2 / (2.0 * sz * sz)) + math.exp(-(z + H) ** 2 / (2.0 * sz * sz))))


def plume_concentration(Q, u, sigma_y, sigma_z, H, y, z):
    _pos("Q, u, sigma_y and sigma_z", Q, u, sigma_y, sigma_z)
    _nonneg("the effective stack height H and the receptor height z", H, z)
    _fin(y)
    C = _plume(Q, u, sigma_y, sigma_z, H, y, z)
    return _out(C=C, C_ugm3=C * 1e9)


def _ref_plume(Q, u, sigma_y, sigma_z, H, y, z):
    C = Q / u * _norm_pdf(y, 0.0, sigma_y) * (_norm_pdf(z, H, sigma_z) + _norm_pdf(z, -H, sigma_z))
    return {"C": C, "C_ugm3": C * 1e9}


def _rand_plume(rng):
    sz = _sig(_lu(rng, 10.0, 300.0), 3)
    return {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3), "sigma_y": _sig(sz * rng.uniform(1.0, 3.0), 3),
            "sigma_z": sz, "H": _sig(rng.uniform(0.0, 2.0) * sz, 3), "y": _sig(rng.uniform(-1.5, 1.5) * sz, 3),
            "z": _sig(rng.uniform(0.0, 1.5) * sz, 3)}


_add("enveng_gaussian_plume_concentration",
     name="Gaussian plume concentration at a receptor (ground reflection)", topic="air pollution / Gaussian plume",
     purpose="The steady concentration downwind of a continuous point source at crosswind distance y and height z, given the dispersion coefficients at the receptor's distance and the effective stack height, with full reflection at the ground.",
     inputs=[_phys("Q", "mass_flow_rate", "pollutant emission rate", show=("g/s", 1e3)),
             _phys("u", "speed", "mean wind speed at the stack height"),
             _phys("sigma_y", "length", "horizontal dispersion coefficient at the receptor distance"),
             _phys("sigma_z", "length", "vertical dispersion coefficient at the receptor distance"),
             _phys("H", "length", "effective stack height (stack height plus plume rise)"),
             _phys("y", "length", "crosswind distance from the plume centreline"),
             _phys("z", "length", "receptor height above the ground")],
     outputs=[_o("C", "mass_concentration", "concentration at the receptor"),
              _o("C_ugm3", "number", "concentration in micrograms per cubic metre")],
     primary="C", conditions=["Q, u, sigma_y, sigma_z > 0", "H, z >= 0", "steady state, flat terrain"],
     method="C = Q/(2 pi u sy sz) exp(-y^2/2sy^2) [exp(-(z-H)^2/2sz^2) + exp(-(z+H)^2/2sz^2)]",
     latex=r"C = \frac{Q}{2\pi u\sigma_y\sigma_z} e^{-\frac{y^2}{2\sigma_y^2}}\left[e^{-\frac{(z-H)^2}{2\sigma_z^2}} + e^{-\frac{(z+H)^2}{2\sigma_z^2}}\right]",
     run=plume_concentration, reference=_ref_plume,
     wrong=lambda Q, u, sigma_y, sigma_z, H, y, z: (lambda C: _out(C=C, C_ugm3=C * 1e9))(
         Q / (2.0 * math.pi * u * sigma_y * sigma_z) * math.exp(-y * y / (2.0 * sigma_y ** 2)) * math.exp(-(z - H) ** 2 / (2.0 * sigma_z ** 2))),
     rand=_rand_plume,
     phrases=("Gaussian plume concentration", "Plume concentration at a receptor"),
     examples=[_ex({"Q": 0.1, "u": 4.0, "sigma_y": 60.0, "sigma_z": 30.0, "H": 50.0, "y": 20.0, "z": 0.0}),
               _ex({"Q": 0.5, "u": 6.0, "sigma_y": 150.0, "sigma_z": 80.0, "H": 100.0, "y": 0.0, "z": 30.0})],
     level="intermediate",
     extra_q=[{"q": "Gaussian plume concentration: Q = 360 kg/h, u = 4 m/s, sigma_y = 60 m, sigma_z = 30 m, H = 50 m, y = 20 m, z = 0 m", "like": 0}])


def ground_centerline(Q, u, sigma_y, sigma_z, H):
    _pos("Q, u, sigma_y and sigma_z", Q, u, sigma_y, sigma_z)
    _nonneg("the effective stack height H", H)
    C = Q / (math.pi * u * sigma_y * sigma_z) * math.exp(-H * H / (2.0 * sigma_z * sigma_z))
    return _out(C=C, C_ugm3=C * 1e9)


def _ref_ground_centerline(Q, u, sigma_y, sigma_z, H):
    C = 2.0 * Q / u * _norm_pdf(0.0, 0.0, sigma_y) * _norm_pdf(H, 0.0, sigma_z)
    return {"C": C, "C_ugm3": C * 1e9}


_add("enveng_ground_centerline_concentration",
     name="Ground-level centreline concentration of a plume", topic="air pollution / Gaussian plume",
     purpose="The ground-level concentration directly below the plume axis (y = 0, z = 0) from the emission rate, wind speed, dispersion coefficients and effective stack height (H = 0 for a ground-level source).",
     inputs=[_phys("Q", "mass_flow_rate", "pollutant emission rate", show=("g/s", 1e3)),
             _phys("u", "speed", "mean wind speed"),
             _phys("sigma_y", "length", "horizontal dispersion coefficient"),
             _phys("sigma_z", "length", "vertical dispersion coefficient"),
             _phys("H", "length", "effective stack height (0 for a ground-level source)")],
     outputs=[_o("C", "mass_concentration", "ground-level centreline concentration"),
              _o("C_ugm3", "number", "concentration in micrograms per cubic metre")],
     primary="C", conditions=["Q, u, sigma_y, sigma_z > 0", "H >= 0"],
     method="C = Q/(pi u sy sz) exp(-H^2/(2 sz^2))",
     latex=r"C(x,0,0) = \frac{Q}{\pi u\sigma_y\sigma_z}\exp\!\left(-\frac{H^2}{2\sigma_z^2}\right)",
     run=ground_centerline, reference=_ref_ground_centerline,
     wrong=lambda Q, u, sigma_y, sigma_z, H: (lambda C: _out(C=C, C_ugm3=C * 1e9))(
         Q / (2.0 * math.pi * u * sigma_y * sigma_z) * math.exp(-H * H / (2.0 * sigma_z * sigma_z))),
     rand=lambda rng: (lambda sz: {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3),
                                   "sigma_y": _sig(sz * rng.uniform(1.0, 3.0), 3), "sigma_z": sz,
                                   "H": _sig(rng.uniform(0.0, 2.5) * sz, 3)})(_sig(_lu(rng, 10.0, 300.0), 3)),
     phrases=("Ground-level centreline concentration", "Ground-level centerline concentration", "Centreline concentration"),
     examples=[_ex({"Q": 0.15, "u": 5.0, "sigma_y": 80.0, "sigma_z": 40.0, "H": 60.0}),
               _ex({"Q": 0.002, "u": 3.0, "sigma_y": 20.0, "sigma_z": 12.0, "H": 0.0})],
     level="beginner")


def plume_with_class(Q, u, H, x, y, z, cls, terrain):
    _pos("Q and u", Q, u)
    _nonneg("H and z", H, z)
    _fin(y)
    sy, sz = _briggs(x, cls, terrain)
    C = _plume(Q, u, sy, sz, H, y, z)
    return _out(C=C, C_ugm3=C * 1e9, sigma_y=sy, sigma_z=sz)


def _ref_plume_with_class(Q, u, H, x, y, z, cls, terrain):
    s = _ref_briggs(x, cls, terrain)
    r = _ref_plume(Q, u, s["sigma_y"], s["sigma_z"], H, y, z)
    return {"C": r["C"], "C_ugm3": r["C_ugm3"], "sigma_y": s["sigma_y"], "sigma_z": s["sigma_z"]}


def _wrong_plume_with_class(Q, u, H, x, y, z, cls, terrain):
    sy, sz = _briggs(x, cls, terrain)
    C = _plume(Q, u, sz, sy, H, y, z)              # the two spreads exchanged
    return _out(C=C, C_ugm3=C * 1e9, sigma_y=sy, sigma_z=sz)


def _rand_plume_class(rng):
    x = _sig(_lu(rng, 300.0, 8000.0), 3)
    cls = rng.choice(_CLASSES)
    terrain = rng.choice(["rural", "urban"])
    sy, sz = _briggs(x, cls, terrain)
    return {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3), "H": _sig(rng.uniform(0.2, 1.5) * sz, 3),
            "x": x, "y": _sig(rng.uniform(-1.0, 1.0) * sy, 3), "z": _sig(rng.uniform(0.0, 1.0) * sz, 3), "cls": cls, "terrain": terrain}


_add("enveng_plume_concentration_stability",
     name="Gaussian plume concentration from the stability class", topic="air pollution / Gaussian plume",
     purpose="The concentration at a receptor (x, y, z) downwind of an elevated point source, with the dispersion coefficients taken from Briggs' formulas for the stability class and terrain.",
     inputs=[_phys("Q", "mass_flow_rate", "pollutant emission rate", show=("g/s", 1e3)),
             _phys("u", "speed", "mean wind speed at the stack height"),
             _phys("H", "length", "effective stack height"),
             _phys("x", "length", "downwind distance"), _phys("y", "length", "crosswind distance from the centreline"),
             _phys("z", "length", "receptor height"),
             _choice("cls", _CLASSES, "Pasquill stability class A-F"),
             _choice("terrain", ("rural", "urban"), "rural or urban dispersion coefficients")],
     outputs=[_o("C", "mass_concentration", "concentration at the receptor"),
              _o("C_ugm3", "number", "concentration in micrograms per cubic metre"),
              _o("sigma_y", "length", "horizontal dispersion coefficient at x"),
              _o("sigma_z", "length", "vertical dispersion coefficient at x")],
     primary="C", conditions=["100 m <= x <= 10 km", "Q, u > 0", "H, z >= 0"],
     method="Briggs sigma_y(x), sigma_z(x) for the class, then the reflected Gaussian plume",
     latex=r"C = \frac{Q}{2\pi u\sigma_y(x)\sigma_z(x)} e^{-\frac{y^2}{2\sigma_y^2}}\left[e^{-\frac{(z-H)^2}{2\sigma_z^2}} + e^{-\frac{(z+H)^2}{2\sigma_z^2}}\right]",
     run=plume_with_class, reference=_ref_plume_with_class, wrong=_wrong_plume_with_class, rand=_rand_plume_class,
     phrases=("Plume concentration with stability class", "Gaussian plume with stability class"),
     aliases={"class": "cls", "stability": "cls"},
     examples=[_ex({"Q": 0.2, "u": 5.0, "H": 80.0, "x": 2000.0, "y": 0.0, "z": 0.0, "cls": "c", "terrain": "rural"}),
               _ex({"Q": 0.05, "u": 3.0, "H": 40.0, "x": 800.0, "y": 30.0, "z": 2.0, "cls": "d", "terrain": "urban"})],
     level="advanced")


def max_ground_turner(Q, u, H, k):
    _pos("Q, u, H and the ratio k = sigma_y/sigma_z", Q, u, H, k)
    C = 2.0 * Q / (math.pi * math.e * u * H * H * k)
    return _out(C_max=C, C_max_ugm3=C * 1e9, sigma_z_max=H / math.sqrt(2.0))


def _ref_max_ground_turner(Q, u, H, k):
    import sympy as sp
    s = sp.symbols("s", positive=True)
    C = Q / (sp.pi * u * k * s ** 2) * sp.exp(-H ** 2 / (2 * s ** 2))
    d = sp.lambdify(s, sp.diff(sp.log(C), s), "math")
    sz = _brentq(d, H / 20.0, 20.0 * H)
    Cv = float(C.subs(s, sz))
    return {"C_max": Cv, "C_max_ugm3": Cv * 1e9, "sigma_z_max": sz}


_add("enveng_max_ground_concentration_ratio",
     name="Maximum ground-level concentration (constant sigma_y/sigma_z)", topic="air pollution / maximum concentration",
     purpose="The largest ground-level centreline concentration of an elevated source when sigma_y/sigma_z is taken constant along the plume, and the vertical spread at which it occurs (sigma_z = H/sqrt 2).",
     inputs=[_phys("Q", "mass_flow_rate", "emission rate", show=("g/s", 1e3)), _phys("u", "speed", "mean wind speed"),
             _phys("H", "length", "effective stack height"), _numin("k", "ratio sigma_y/sigma_z (constant along the plume)")],
     outputs=[_o("C_max", "mass_concentration", "maximum ground-level concentration"),
              _o("C_max_ugm3", "number", "maximum concentration in micrograms per cubic metre"),
              _o("sigma_z_max", "length", "vertical dispersion coefficient at the maximum")],
     primary="C_max", conditions=["Q, u, H, k > 0", "sigma_y/sigma_z constant along the plume"],
     method="maximise C = Q/(pi u k sz^2) exp(-H^2/2sz^2) over sz: sz = H/sqrt2, Cmax = 2Q/(pi e u H^2 k)",
     latex=r"C_{max} = \frac{2Q}{\pi e\,u H^2}\frac{\sigma_z}{\sigma_y},\quad \sigma_z = \frac{H}{\sqrt 2}",
     run=max_ground_turner, reference=_ref_max_ground_turner,
     wrong=lambda Q, u, H, k: (lambda C: _out(C_max=C, C_max_ugm3=C * 1e9, sigma_z_max=H))(Q / (math.pi * u * H * H * k)),
     rand=lambda rng: {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3), "H": _sig(rng.uniform(20.0, 250.0), 3),
                       "k": _sig(rng.uniform(1.0, 4.0), 3)},
     phrases=("Maximum ground-level concentration with constant sigma ratio", "Maximum ground-level concentration"),
     examples=[_ex({"Q": 0.1, "u": 5.0, "H": 100.0, "k": 2.0}), _ex({"Q": 1.2, "u": 3.5, "H": 150.0, "k": 1.6})],
     level="expert")


def _lsig(c, x):
    a, b, p = c
    return 1.0 / x + p * b / (1.0 + b * x)


def max_ground_briggs(Q, u, H, cls, terrain):
    _pos("Q, u and H", Q, u, H)
    cy, cz = _briggs_coef(cls, terrain)

    def g(x):
        sz = _sig_eval(cz, x)
        return -_lsig(cy, x) + _lsig(cz, x) * (H * H / (sz * sz) - 1.0)
    n = 400
    xs = [100.0 * 100.0 ** (i / n) for i in range(n + 1)]
    _need(g(xs[0]) > 0, "the maximum lies closer than 100 m: outside the range of the Briggs coefficients")
    lo = None
    for i in range(n):
        if g(xs[i + 1]) <= 0:
            lo = i
            break
    _need(lo is not None, "the maximum lies beyond 10 km: outside the range of the Briggs coefficients")
    x = _bisect(g, xs[lo], xs[lo + 1])
    sy, sz = _sig_eval(cy, x), _sig_eval(cz, x)
    C = Q / (math.pi * u * sy * sz) * math.exp(-H * H / (2.0 * sz * sz))
    return _out(x_max=x, C_max=C, C_max_ugm3=C * 1e9, sigma_y=sy, sigma_z=sz)


def _ref_max_ground_briggs(Q, u, H, cls, terrain):
    import numpy as np
    import sympy as sp
    x = sp.symbols("x", positive=True)
    s = _ref_briggs  # coefficients re-derived from the reference table through two evaluations
    rural = {"a": (0.22, 1e-4, -0.5, 0.20, 0.0, 1.0), "b": (0.16, 1e-4, -0.5, 0.12, 0.0, 1.0),
             "c": (0.11, 1e-4, -0.5, 0.08, 2e-4, -0.5), "d": (0.08, 1e-4, -0.5, 0.06, 1.5e-3, -0.5),
             "e": (0.06, 1e-4, -0.5, 0.03, 3e-4, -1.0), "f": (0.04, 1e-4, -0.5, 0.016, 3e-4, -1.0)}
    urban = {"a": (0.32, 4e-4, -0.5, 0.24, 1e-3, 0.5), "b": (0.32, 4e-4, -0.5, 0.24, 1e-3, 0.5),
             "c": (0.22, 4e-4, -0.5, 0.20, 0.0, 1.0), "d": (0.16, 4e-4, -0.5, 0.14, 3e-4, -0.5),
             "e": (0.11, 4e-4, -0.5, 0.08, 1.5e-3, -0.5), "f": (0.11, 4e-4, -0.5, 0.08, 1.5e-3, -0.5)}
    ay, by, py, az, bz, pz = (rural if terrain == "rural" else urban)[cls]
    sy = ay * x * (1 + by * x) ** sp.Float(py)
    sz = az * x * (1 + bz * x) ** sp.Float(pz)
    lnC = -sp.log(sy) - sp.log(sz) - sp.Float(H) ** 2 / (2 * sz ** 2)
    d = sp.lambdify(x, sp.diff(lnC, x), "math")
    grid = np.geomspace(100.0, 10000.0, 3001)
    vals = [d(float(v)) for v in grid]
    i = next(k for k in range(len(grid) - 1) if vals[k] > 0 >= vals[k + 1])
    xm = _brentq(d, float(grid[i]), float(grid[i + 1]))
    sig = s(xm, cls, terrain)
    C = 2.0 * Q / u * _norm_pdf(0.0, 0.0, sig["sigma_y"]) * _norm_pdf(H, 0.0, sig["sigma_z"])
    return {"x_max": xm, "C_max": C, "C_max_ugm3": C * 1e9, "sigma_y": sig["sigma_y"], "sigma_z": sig["sigma_z"]}


def _wrong_max_ground_briggs(Q, u, H, cls, terrain):
    r = max_ground_briggs(Q, u, H, cls, terrain)
    cy, cz = _briggs_coef(cls, terrain)
    # the constant-ratio shortcut: sigma_z = H/sqrt2 located on the sigma_z curve
    x = _bisect(lambda v: _sig_eval(cz, v) - H / math.sqrt(2.0), 1.0, 1e6)
    sy, sz = _sig_eval(cy, x), _sig_eval(cz, x)
    C = Q / (math.pi * u * sy * sz) * math.exp(-H * H / (2.0 * sz * sz))
    return dict(r, x_max=_r(x), C_max=_r(C), C_max_ugm3=_r(C * 1e9))


def _rand_max_briggs(rng):
    return {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3), "H": _sig(rng.uniform(15.0, 150.0), 3),
            "cls": rng.choice(_CLASSES), "terrain": rng.choice(["rural", "urban"])}


_add("enveng_max_ground_concentration_briggs",
     name="Maximum ground-level concentration and its distance (Briggs coefficients)", topic="air pollution / maximum concentration",
     purpose="The distance of the largest ground-level centreline concentration of an elevated source and that concentration, found exactly on Briggs' dispersion curves for the stability class (no constant-ratio shortcut).",
     inputs=[_phys("Q", "mass_flow_rate", "emission rate", show=("g/s", 1e3)), _phys("u", "speed", "mean wind speed"),
             _phys("H", "length", "effective stack height"), _choice("cls", _CLASSES, "Pasquill stability class A-F"),
             _choice("terrain", ("rural", "urban"), "rural or urban dispersion coefficients")],
     outputs=[_o("x_max", "length", "downwind distance of the maximum"),
              _o("C_max", "mass_concentration", "maximum ground-level concentration"),
              _o("C_max_ugm3", "number", "maximum concentration in micrograms per cubic metre"),
              _o("sigma_y", "length", "sigma_y at the maximum"), _o("sigma_z", "length", "sigma_z at the maximum")],
     primary="C_max", conditions=["the maximum lies between 100 m and 10 km"],
     method="root of d ln C/dx = -sy'/sy + (sz'/sz)(H^2/sz^2 - 1) by a log-spaced scan and bisection",
     latex=r"\frac{d\ln C}{dx} = -\frac{\sigma_y'}{\sigma_y} + \frac{\sigma_z'}{\sigma_z}\left(\frac{H^2}{\sigma_z^2} - 1\right) = 0",
     run=max_ground_briggs, reference=_ref_max_ground_briggs, wrong=_wrong_max_ground_briggs, rand=_rand_max_briggs,
     phrases=("Maximum ground-level concentration with Briggs coefficients", "Distance of maximum ground-level concentration"),
     aliases={"class": "cls", "stability": "cls"},
     examples=[_ex({"Q": 0.1, "u": 5.0, "H": 50.0, "cls": "c", "terrain": "rural"}),
               _ex({"Q": 0.3, "u": 4.0, "H": 80.0, "cls": "d", "terrain": "urban"})],
     level="expert")


# ================================================================================================ AIR: plume rise
def _buoyancy_flux(vs, d, Ts, Ta):
    _pos("vs, d, Ts and Ta", vs, d, Ts, Ta)
    _need(Ts > Ta, "the stack gas must be warmer than the air (Ts > Ta) for a buoyant plume")
    return G0 * vs * d * d * (Ts - Ta) / (4.0 * Ts)


def briggs_rise(vs, d, Ts, Ta, u, hs):
    F = _buoyancy_flux(vs, d, Ts, Ta)
    _pos("u", u)
    _nonneg("the stack height hs", hs)
    xf = 49.0 * F ** 0.625 if F < 55.0 else 119.0 * F ** 0.4
    dh = 1.6 * F ** (1.0 / 3.0) * xf ** (2.0 / 3.0) / u
    return _out(F=F, x_f=xf, dh=dh, H=hs + dh)


def _ref_briggs_rise(vs, d, Ts, Ta, u, hs):
    import numpy as np
    F = 9.80665 * vs * d ** 2 * (1.0 - Ta / Ts) / 4.0
    if F < 55.0:
        dh, xf = 1.6 * np.power(49.0, 2.0 / 3.0) * np.power(F, 0.75) / u, 49.0 * np.power(F, 0.625)
    else:
        dh, xf = 1.6 * np.power(119.0, 2.0 / 3.0) * np.power(F, 0.6) / u, 119.0 * np.power(F, 0.4)
    return {"F": float(F), "x_f": float(xf), "dh": float(dh), "H": float(hs + dh)}


def _wrong_briggs_rise(vs, d, Ts, Ta, u, hs):
    F = G0 * vs * d * d * (Ts - Ta) / (4.0 * Ta)
    xf = 49.0 * F ** 0.625 if F < 55.0 else 119.0 * F ** 0.4
    dh = 1.6 * F ** (1.0 / 3.0) * xf ** (2.0 / 3.0) / u
    return _out(F=F, x_f=xf, dh=dh, H=hs + dh)


def _rand_stack(rng):
    Ta = _sig(rng.uniform(260.0, 310.0), 4)
    return {"vs": _sig(rng.uniform(5.0, 25.0), 3), "d": _sig(rng.uniform(0.5, 6.0), 3), "Ts": _sig(Ta + rng.uniform(30.0, 250.0), 4),
            "Ta": Ta, "u": _sig(rng.uniform(1.5, 12.0), 3), "hs": _sig(rng.uniform(10.0, 200.0), 3)}


_STACK_IN = [_phys("vs", "speed", "stack gas exit velocity"), _phys("d", "length", "stack inside diameter at the exit"),
             _phys("Ts", "temperature", "stack gas temperature"), _phys("Ta", "temperature", "ambient air temperature")]

_add("enveng_briggs_plume_rise",
     name="Briggs plume rise and effective stack height (neutral or unstable air)", topic="air pollution / plume rise",
     purpose="The buoyancy flux, the final plume rise by Briggs' 2/3 law (neutral or unstable air) and the effective stack height.",
     inputs=_STACK_IN + [_phys("u", "speed", "wind speed at the stack top"), _phys("hs", "length", "physical stack height")],
     outputs=[_o("F", "number", "buoyancy flux, m^4/s^3"), _o("x_f", "length", "distance to final rise"),
              _o("dh", "length", "plume rise"), _o("H", "length", "effective stack height hs + dh")],
     primary="dh", conditions=["Ts > Ta", "neutral or unstable air (classes A-D)", "buoyancy-dominated plume"],
     method="F = g vs d^2 (Ts - Ta)/(4 Ts); x_f = 49 F^(5/8) (F < 55) or 119 F^(2/5); dh = 1.6 F^(1/3) x_f^(2/3)/u",
     latex=r"F = \frac{g v_s d^2 (T_s - T_a)}{4T_s},\quad \Delta h = \frac{1.6F^{1/3}x_f^{2/3}}{u}",
     run=briggs_rise, reference=_ref_briggs_rise, wrong=_wrong_briggs_rise, rand=_rand_stack,
     phrases=("Briggs plume rise", "Plume rise and effective stack height"),
     examples=[_ex({"vs": 15.0, "d": 3.0, "Ts": 420.0, "Ta": 288.0, "u": 5.0, "hs": 100.0}),
               _ex({"vs": 8.0, "d": 1.0, "Ts": 400.0, "Ta": 293.15, "u": 3.0, "hs": 30.0})],
     level="intermediate",
     extra_q=[{"q": "Briggs plume rise: vs = 15 m/s, d = 3 m, Ts = 146.85 degC, Ta = 14.85 degC, u = 5 m/s, hs = 100 m", "like": 0}],
     weak="the same published constants evaluated through the algebraically reduced powers F^(3/4), F^(3/5)")


def briggs_stable_rise(vs, d, Ts, Ta, u, dtheta, hs):
    F = _buoyancy_flux(vs, d, Ts, Ta)
    _pos("u and the potential-temperature gradient dtheta", u, dtheta)
    _nonneg("the stack height hs", hs)
    s = G0 / Ta * dtheta
    dh = 2.6 * (F / (u * s)) ** (1.0 / 3.0)
    return _out(F=F, s=s, dh=dh, H=hs + dh)


def _ref_briggs_stable_rise(vs, d, Ts, Ta, u, dtheta, hs):
    import numpy as np
    F = 9.80665 * vs * d ** 2 * (1.0 - Ta / Ts) / 4.0
    s = 9.80665 * dtheta / Ta
    dh = 2.6 * np.cbrt(F) * np.cbrt(1.0 / (u * s))
    return {"F": float(F), "s": s, "dh": float(dh), "H": float(hs + dh)}


_add("enveng_briggs_stable_plume_rise",
     name="Briggs plume rise in stable air", topic="air pollution / plume rise",
     purpose="The final rise of a buoyant plume in stable air (classes E, F) from the buoyancy flux, wind speed and potential-temperature gradient, and the effective stack height.",
     inputs=_STACK_IN + [_phys("u", "speed", "wind speed at the stack top"),
                         _phys("dtheta", "temperature_gradient", "potential temperature gradient d(theta)/dz"),
                         _phys("hs", "length", "physical stack height")],
     outputs=[_o("F", "number", "buoyancy flux, m^4/s^3"), _o("s", "number", "stability parameter g/Ta dtheta/dz, 1/s^2"),
              _o("dh", "length", "plume rise"), _o("H", "length", "effective stack height")],
     primary="dh", conditions=["Ts > Ta", "stable air: dtheta/dz > 0"],
     method="s = (g/Ta) dtheta/dz; dh = 2.6 (F/(u s))^(1/3)",
     latex=r"\Delta h = 2.6\left(\frac{F}{u s}\right)^{1/3},\quad s = \frac{g}{T_a}\frac{\partial\theta}{\partial z}",
     run=briggs_stable_rise, reference=_ref_briggs_stable_rise,
     wrong=lambda vs, d, Ts, Ta, u, dtheta, hs: (lambda F, s: _out(F=F, s=s, dh=2.6 * (F / (u * s)) ** (1.0 / 3.0), H=hs + 2.6 * (F / (u * s)) ** (1.0 / 3.0)))(
         _buoyancy_flux(vs, d, Ts, Ta), G0 / Ts * dtheta),
     rand=lambda rng: dict(_rand_stack(rng), dtheta=_sig(rng.uniform(0.005, 0.05), 3)),
     phrases=("Briggs stable plume rise", "Plume rise in stable air"),
     examples=[_ex({"vs": 15.0, "d": 3.0, "Ts": 420.0, "Ta": 288.0, "u": 4.0, "dtheta": 0.02, "hs": 100.0}),
               _ex({"vs": 10.0, "d": 1.5, "Ts": 450.0, "Ta": 275.0, "u": 2.5, "dtheta": 0.035, "hs": 60.0})],
     level="advanced",
     weak="the same published formula evaluated with numpy cube roots")


def holland_rise(vs, d, u, p, Ts, Ta, hs):
    _pos("vs, d, u, p, Ts and Ta", vs, d, u, p, Ts, Ta)
    _need(Ts > Ta, "the stack gas must be warmer than the air (Ts > Ta)")
    _nonneg("the stack height hs", hs)
    dh = vs * d / u * (1.5 + 2.68e-3 * (p / 100.0) * d * (Ts - Ta) / Ts)
    return _out(dh=dh, H=hs + dh)


def _ref_holland_rise(vs, d, u, p, Ts, Ta, hs):
    momentum = 1.5 * vs * d / u
    buoyancy = 2.68e-5 * p * vs * d * d * (1.0 - Ta / Ts) / u
    return {"dh": momentum + buoyancy, "H": hs + momentum + buoyancy}


_add("enveng_holland_plume_rise",
     name="Holland plume rise", topic="air pollution / plume rise",
     purpose="Plume rise by Holland's formula (momentum term plus a buoyancy term with the pressure in millibars) and the effective stack height.",
     inputs=[_phys("vs", "speed", "stack gas exit velocity"), _phys("d", "length", "stack inside diameter"),
             _phys("u", "speed", "wind speed at the stack top"), _phys("p", "pressure", "atmospheric pressure", show=("kPa", 1e-3)),
             _phys("Ts", "temperature", "stack gas temperature"), _phys("Ta", "temperature", "ambient air temperature"),
             _phys("hs", "length", "physical stack height")],
     outputs=[_o("dh", "length", "plume rise"), _o("H", "length", "effective stack height")],
     primary="dh", conditions=["Ts > Ta"],
     method="dh = vs d/u (1.5 + 2.68e-3 p[mbar] d (Ts - Ta)/Ts)",
     latex=r"\Delta h = \frac{v_s d}{u}\left(1.5 + 2.68\times10^{-3}\,p\,d\,\frac{T_s - T_a}{T_s}\right)",
     run=holland_rise, reference=_ref_holland_rise,
     wrong=lambda vs, d, u, p, Ts, Ta, hs: (lambda dh: _out(dh=dh, H=hs + dh))(vs * d / u * (2.68e-3 * (p / 100.0) * d * (Ts - Ta) / Ts)),
     rand=lambda rng: dict(_rand_stack(rng), p=_sig(rng.uniform(85000.0, 103000.0), 4)),
     phrases=("Holland plume rise", "Holland formula plume rise"),
     examples=[_ex({"vs": 10.0, "d": 2.0, "u": 4.0, "p": 101325.0, "Ts": 400.0, "Ta": 290.0, "hs": 50.0}),
               _ex({"vs": 20.0, "d": 4.0, "u": 6.0, "p": 95000.0, "Ts": 450.0, "Ta": 280.0, "hs": 150.0})],
     level="intermediate",
     weak="the same published formula split into its momentum and buoyancy terms")


def wind_power_law(u1, z1, z2, p):
    _pos("u1, z1 and z2", u1, z1, z2)
    _need(0.0 < p < 1.0, "the exponent p must lie between 0 and 1")
    return _out(u2=u1 * (z2 / z1) ** p)


def _ref_wind_power_law(u1, z1, z2, p):
    import mpmath as mp
    with mp.workdps(30):
        return {"u2": float(mp.mpf(u1) * mp.exp(mp.mpf(p) * (mp.log(z2) - mp.log(z1))))}


_add("enveng_wind_power_law",
     name="Wind speed at height (power law)", topic="air pollution / wind profile",
     purpose="The wind speed at another height from a measured speed with the power-law profile u2 = u1 (z2/z1)^p (p from the stability class and terrain, e.g. 0.15 rural neutral).",
     inputs=[_phys("u1", "speed", "measured wind speed"), _phys("z1", "length", "measurement height"),
             _phys("z2", "length", "height of interest (e.g. the stack top)"), _numin("p", "power-law exponent (0 < p < 1)")],
     outputs=[_o("u2", "speed", "wind speed at z2")],
     primary="u2", conditions=["0 < p < 1"], method="u2 = u1 (z2/z1)^p", latex=r"u_2 = u_1\left(\frac{z_2}{z_1}\right)^p",
     run=wind_power_law, reference=_ref_wind_power_law,
     wrong=lambda u1, z1, z2, p: _out(u2=u1 * (z1 / z2) ** p),
     rand=lambda rng: {"u1": _sig(rng.uniform(1.0, 10.0), 3), "z1": _sig(rng.uniform(2.0, 20.0), 3),
                       "z2": _sig(rng.uniform(30.0, 300.0), 3), "p": _sig(rng.uniform(0.07, 0.55), 3)},
     phrases=("Wind speed at height", "Wind power law"),
     examples=[_ex({"u1": 4.0, "z1": 10.0, "z2": 100.0, "p": 0.15}), _ex({"u1": 2.5, "z1": 10.0, "z2": 60.0, "p": 0.35})],
     level="beginner", weak="the same power law evaluated in 30-digit mpmath arithmetic")


# ================================================================================================ AIR: concentrations
def ppm_to_mass(ppm, M, T, P):
    _pos("ppm, M, T and P", ppm, M, T, P)
    C = ppm * 1e-6 * P * M / (R_GAS * T)
    return _out(C=C, C_ugm3=C * 1e9, C_mgm3=C * 1e6)


def _ref_ppm_to_mass(ppm, M, T, P):
    from chemicals import ideal_gas
    C = ppm * 1e-6 * M / ideal_gas(T, P)
    return {"C": C, "C_ugm3": C * 1e9, "C_mgm3": C * 1e6}


_add("enveng_ppm_to_mass_concentration",
     name="Gas concentration: ppm (by volume) to mass concentration", topic="air pollution / concentration units",
     purpose="The mass concentration of a gaseous pollutant from its volume mixing ratio in ppm, its molar mass and the air temperature and pressure (ideal gas).",
     inputs=[_numin("ppm", "volume mixing ratio in parts per million", words=("ppm",)),
             _phys("M", "molar_mass", "molar mass of the pollutant", show=("g/mol", 1e3)),
             _phys("T", "temperature", "air temperature"), _phys("P", "pressure", "air pressure", show=("kPa", 1e-3))],
     outputs=[_o("C", "mass_concentration", "mass concentration"), _o("C_ugm3", "number", "in micrograms per cubic metre"),
              _o("C_mgm3", "number", "in milligrams per cubic metre")],
     primary="C", conditions=["ideal gas"], method="C = ppm 1e-6 P M/(R T)",
     latex=r"C = \text{ppm}\times10^{-6}\,\frac{P M}{R T}",
     run=ppm_to_mass, reference=_ref_ppm_to_mass,
     wrong=lambda ppm, M, T, P: (lambda C: _out(C=C, C_ugm3=C * 1e9, C_mgm3=C * 1e6))(ppm * 1e-6 * M / 0.022414),
     rand=lambda rng: {"ppm": _sig(_lu(rng, 0.01, 500.0), 3), "M": _sig(rng.uniform(16.0, 160.0), 4) * 1e-3,
                       "T": _sig(rng.uniform(283.0, 318.0), 4), "P": _sig(rng.uniform(80000.0, 104000.0), 4)},
     phrases=("Convert ppm to mass concentration", "ppm to mass concentration"),
     examples=[_ex({"ppm": 0.14, "M": 0.064066, "T": 298.15, "P": 101325.0}), _ex({"ppm": 9.0, "M": 0.02801, "T": 293.15, "P": 95000.0})],
     level="beginner",
     extra_q=[{"q": "Convert ppm to mass concentration: ppm = 0.14 ppm, M = 64.066 g/mol, T = 25 degC, P = 1 atm", "like": 0}])


def mass_to_ppm(C, M, T, P):
    _pos("C, M, T and P", C, M, T, P)
    ppm = C * R_GAS * T / (P * M) * 1e6
    return _out(ppm=ppm, ppb=ppm * 1e3)


def _ref_mass_to_ppm(C, M, T, P):
    from chemicals import ideal_gas
    ppm = C / M * ideal_gas(T, P) * 1e6
    return {"ppm": ppm, "ppb": ppm * 1e3}


_add("enveng_mass_concentration_to_ppm",
     name="Gas concentration: mass concentration to ppm (by volume)", topic="air pollution / concentration units",
     purpose="The volume mixing ratio (ppm and ppb) of a gaseous pollutant from its mass concentration, molar mass, temperature and pressure (ideal gas).",
     inputs=[_phys("C", "mass_concentration", "mass concentration", show=("ug/m^3", 1e9)),
             _phys("M", "molar_mass", "molar mass", show=("g/mol", 1e3)),
             _phys("T", "temperature", "air temperature"), _phys("P", "pressure", "air pressure", show=("kPa", 1e-3))],
     outputs=[_o("ppm", "number", "volume mixing ratio in ppm"), _o("ppb", "number", "volume mixing ratio in ppb")],
     primary="ppm", conditions=["ideal gas"], method="ppm = C R T/(P M) 1e6",
     latex=r"\text{ppm} = \frac{C R T}{P M}\times10^{6}",
     run=mass_to_ppm, reference=_ref_mass_to_ppm,
     wrong=lambda C, M, T, P: (lambda v: _out(ppm=v, ppb=v * 1e3))(C * 0.022414 / M * 1e6),
     rand=lambda rng: {"C": _sig(_lu(rng, 1.0, 5000.0), 3) * 1e-9, "M": _sig(rng.uniform(16.0, 160.0), 4) * 1e-3,
                       "T": _sig(rng.uniform(283.0, 318.0), 4), "P": _sig(rng.uniform(80000.0, 104000.0), 4)},
     phrases=("Convert mass concentration to ppm", "Mass concentration to ppm"),
     examples=[_ex({"C": 100e-9, "M": 0.046006, "T": 298.15, "P": 101325.0}), _ex({"C": 10000e-9, "M": 0.02801, "T": 288.15, "P": 90000.0})],
     level="intermediate")


def box_model(q, L, u, Hm, Cb, C0, t):
    _pos("L, u and the mixing height Hm", L, u, Hm)
    _nonneg("q, Cb, C0 and t", q, Cb, C0, t)
    Css = Cb + q * L / (u * Hm)
    C = Css + (C0 - Css) * math.exp(-u * t / L)
    return _out(C=C, C_ss=Css, C_ugm3=C * 1e9)


def _ref_box_model(q, L, u, Hm, Cb, C0, t):
    Css = Cb + q * L / (u * Hm)
    if t == 0:
        return {"C": C0, "C_ss": Css, "C_ugm3": C0 * 1e9}
    C = _ivp(lambda _t, y: [q / Hm - u * (y[0] - Cb) / L], t, [C0], atol=[1e-22])[0]
    return {"C": C, "C_ss": Css, "C_ugm3": C * 1e9}


_add("enveng_box_model",
     name="Box model of an air basin (well-mixed, time-dependent)", topic="air pollution / box model",
     purpose="The concentration over a city treated as a well-mixed box: emission flux per area, wind through a box of length L under a mixing height, background air; the concentration after time t and at steady state.",
     inputs=[_phys("q", "mass_flux", "emission rate per unit ground area", show=("ug/(m^2*s)", 1e9)),
             _phys("L", "length", "box length along the wind"), _phys("u", "speed", "wind speed"),
             _phys("Hm", "length", "mixing height"), _phys("Cb", "mass_concentration", "background (upwind) concentration", show=("ug/m^3", 1e9)),
             _phys("C0", "mass_concentration", "concentration at t = 0", show=("ug/m^3", 1e9)),
             _phys("t", "time", "elapsed time", show=("h", 1.0 / HOUR))],
     outputs=[_o("C", "mass_concentration", "concentration at time t"), _o("C_ss", "mass_concentration", "steady-state concentration"),
              _o("C_ugm3", "number", "concentration at t in micrograms per cubic metre")],
     primary="C", conditions=["well-mixed box", "constant wind, mixing height and emissions"],
     method="dC/dt = q/Hm - u (C - Cb)/L, so C = Css + (C0 - Css) e^(-u t/L) with Css = Cb + q L/(u Hm)",
     latex=r"C(t) = C_{ss} + (C_0 - C_{ss})e^{-ut/L},\quad C_{ss} = C_b + \frac{qL}{uH_m}",
     run=box_model, reference=_ref_box_model,
     wrong=lambda q, L, u, Hm, Cb, C0, t: (lambda Css: (lambda C: _out(C=C, C_ss=Css, C_ugm3=C * 1e9))(Css + (C0 - Css) * math.exp(-u * t / L)))(q * L / (u * Hm)),
     rand=lambda rng: {"q": _sig(_lu(rng, 0.5, 50.0), 3) * 1e-9, "L": _sig(rng.uniform(5000.0, 40000.0), 3), "u": _sig(rng.uniform(1.0, 6.0), 3),
                       "Hm": _sig(rng.uniform(200.0, 1500.0), 3), "Cb": _sig(rng.uniform(5.0, 60.0), 3) * 1e-9,
                       "C0": _sig(rng.uniform(0.0, 100.0), 3) * 1e-9, "t": _sig(rng.uniform(0.2, 6.0), 3) * HOUR},
     phrases=("Air basin box model", "Box model concentration"),
     examples=[_ex({"q": 4e-9, "L": 15000.0, "u": 3.0, "Hm": 600.0, "Cb": 20e-9, "C0": 20e-9, "t": 2 * HOUR}),
               _ex({"q": 1e-8, "L": 20000.0, "u": 2.0, "Hm": 400.0, "Cb": 10e-9, "C0": 50e-9, "t": 1.5 * HOUR})],
     level="advanced")


# ================================================================================================ AIR: control devices
def _stokes_v(dp, rho_p, rho_f, mu):
    return G0 * (rho_p - rho_f) * dp * dp / (18.0 * mu)


def settling_chamber(dp, rho_p, rho_g, mu, L, W, Q, N):
    _pos("dp, rho_p, rho_g, mu, L, W and Q", dp, rho_p, rho_g, mu, L, W, Q)
    _need(rho_p > rho_g, "the particles must be denser than the gas")
    _need(isinstance(N, int) and N >= 1, "the number of trays N must be a whole number of at least 1")
    vt = _stokes_v(dp, rho_p, rho_g, mu)
    Re = rho_g * vt * dp / mu
    _need(Re < 1.0, "Stokes' law needs a particle Reynolds number below 1")
    eta = min(1.0, vt * L * W * N / Q)
    return _out(vt=vt, eta=eta, Re=Re)


def _ref_settling_chamber(dp, rho_p, rho_g, mu, L, W, Q, N):
    from fluids import v_terminal
    vt = v_terminal(dp, rho_p, rho_g, mu, Method="Stokes")
    return {"vt": vt, "eta": min(1.0, vt * L * W * N / Q), "Re": rho_g * vt * dp / mu}


_add("enveng_settling_chamber_efficiency",
     name="Gravity settling chamber collection efficiency", topic="air pollution / particulate control",
     purpose="The Stokes settling velocity of a particle in a gas and the collection efficiency of a horizontal-flow settling chamber with N trays (laminar plug-flow model).",
     inputs=[_phys("dp", "length", "particle diameter", show=("um", 1e6)), _phys("rho_p", "density", "particle density"),
             _phys("rho_g", "density", "gas density"), _phys("mu", "dynamic_viscosity", "gas viscosity"),
             _phys("L", "length", "chamber length"), _phys("W", "length", "chamber width"),
             _phys("Q", "flow_rate", "gas flow rate"), _intin("N", "number of trays (1 for a single chamber)", 1, 100)],
     outputs=[_o("vt", "speed", "Stokes terminal settling velocity"), _o("eta", "number", "collection efficiency (fraction, at most 1)"),
              _o("Re", "number", "particle Reynolds number")],
     primary="eta", conditions=["Stokes regime Re < 1", "laminar plug flow"],
     method="vt = g (rho_p - rho_g) dp^2/(18 mu); eta = min(1, vt L W N/Q)",
     latex=r"\eta = \frac{v_t L W N}{Q},\quad v_t = \frac{g(\rho_p - \rho_g)d_p^2}{18\mu}",
     run=settling_chamber, reference=_ref_settling_chamber,
     wrong=lambda dp, rho_p, rho_g, mu, L, W, Q, N: (lambda vt: _out(vt=vt, eta=min(1.0, vt * L * W * N / Q), Re=rho_g * vt * dp / mu))(
         G0 * (rho_p - rho_g) * dp * dp / (9.0 * mu)),
     rand=lambda rng: {"dp": _sig(rng.uniform(10.0, 60.0), 3) * 1e-6, "rho_p": _sig(rng.uniform(1000.0, 3000.0), 3), "rho_g": 1.2,
                       "mu": 1.8e-5, "L": _sig(rng.uniform(2.0, 10.0), 3), "W": _sig(rng.uniform(1.0, 4.0), 3),
                       "Q": _sig(rng.uniform(5.0, 40.0), 3), "N": rng.randint(1, 3)},
     phrases=("Settling chamber efficiency", "Gravity settling chamber"),
     examples=[_ex({"dp": 50e-6, "rho_p": 2000.0, "rho_g": 1.2, "mu": 1.8e-5, "L": 5.0, "W": 2.0, "Q": 10.0, "N": 1}),
               _ex({"dp": 30e-6, "rho_p": 2500.0, "rho_g": 1.2, "mu": 1.85e-5, "L": 6.0, "W": 3.0, "Q": 20.0, "N": 2})],
     level="advanced")


def esp_efficiency(w, A, Q):
    _pos("w, A and Q", w, A, Q)
    eta = 1.0 - math.exp(-w * A / Q)
    return _out(eta=eta, penetration=1.0 - eta)


def _ref_esp_efficiency(w, A, Q):
    c = _ivp(lambda a, y: [-w / Q * y[0]], A, [1.0])[0]
    return {"eta": 1.0 - c, "penetration": c}


_add("enveng_esp_efficiency",
     name="Electrostatic precipitator efficiency (Deutsch-Anderson)", topic="air pollution / particulate control",
     purpose="The collection efficiency and penetration of an electrostatic precipitator from the drift velocity, collecting area and gas flow.",
     inputs=[_phys("w", "speed", "effective drift (migration) velocity"), _phys("A", "area", "collecting plate area"),
             _phys("Q", "flow_rate", "gas flow rate")],
     outputs=[_o("eta", "number", "collection efficiency (fraction)"), _o("penetration", "number", "fraction passing (1 - eta)")],
     primary="eta", conditions=["w, A, Q > 0"], method="eta = 1 - exp(-w A/Q)", latex=r"\eta = 1 - e^{-wA/Q}",
     run=esp_efficiency, reference=_ref_esp_efficiency,
     wrong=lambda w, A, Q: _out(eta=w * A / (Q + w * A), penetration=Q / (Q + w * A)),
     rand=lambda rng: {"w": _sig(rng.uniform(0.02, 0.2), 3), "A": _sig(rng.uniform(500.0, 8000.0), 3), "Q": _sig(rng.uniform(50.0, 300.0), 3)},
     phrases=("Electrostatic precipitator efficiency", "ESP efficiency"),
     examples=[_ex({"w": 0.1, "A": 5000.0, "Q": 200.0}), _ex({"w": 0.06, "A": 2000.0, "Q": 100.0})],
     level="beginner")


def esp_area(w, Q, eta):
    _pos("w and Q", w, Q)
    _frac("the efficiency eta", eta, lo_open=True, hi_open=True)
    return _out(A=-Q * math.log(1.0 - eta) / w)


def _ref_esp_area(w, Q, eta):
    return {"A": _brentq(lambda a: 1.0 - math.exp(-w * a / Q) - eta, 0.0, -Q * math.log(1.0 - eta) / w * 10.0 + 1.0)}


_add("enveng_esp_area",
     name="Electrostatic precipitator plate area for a target efficiency", topic="air pollution / particulate control",
     purpose="The collecting plate area an electrostatic precipitator needs to reach a stated efficiency (Deutsch-Anderson).",
     inputs=[_phys("w", "speed", "effective drift velocity"), _phys("Q", "flow_rate", "gas flow rate"),
             _numin("eta", "required collection efficiency (fraction or %)", pct=True)],
     outputs=[_o("A", "area", "required collecting area")],
     primary="A", conditions=["0 < eta < 1"], method="A = -Q ln(1 - eta)/w", latex=r"A = -\frac{Q}{w}\ln(1-\eta)",
     run=esp_area, reference=_ref_esp_area,
     wrong=lambda w, Q, eta: _out(A=-Q * math.log10(1.0 - eta) / w),
     rand=lambda rng: {"w": _sig(rng.uniform(0.02, 0.2), 3), "Q": _sig(rng.uniform(50.0, 300.0), 3), "eta": _sig(rng.uniform(0.8, 0.999), 4)},
     phrases=("Electrostatic precipitator area", "ESP plate area"),
     examples=[_ex({"w": 0.1, "Q": 200.0, "eta": 0.99}), _ex({"w": 0.08, "Q": 150.0, "eta": 0.95})],
     level="intermediate",
     extra_q=[{"q": "ESP plate area: w = 0.1 m/s, Q = 200 m^3/s, eta = 99%", "like": 0}])


def cyclone_lapple(mu, W, Ne, Vi, rho_p, rho_g, dp):
    _pos("mu, W, Ne, Vi, rho_p, rho_g and dp", mu, W, Ne, Vi, rho_p, rho_g, dp)
    _need(rho_p > rho_g, "the particles must be denser than the gas")
    d50 = math.sqrt(9.0 * mu * W / (2.0 * math.pi * Ne * Vi * (rho_p - rho_g)))
    return _out(d50=d50, eta=1.0 / (1.0 + (d50 / dp) ** 2))


def _ref_cyclone_lapple(mu, W, Ne, Vi, rho_p, rho_g, dp):
    d50sq = 9.0 * mu * W / (2.0 * math.pi * Ne * Vi * (rho_p - rho_g))
    return {"d50": math.sqrt(d50sq), "eta": dp * dp / (dp * dp + d50sq)}


_add("enveng_cyclone_lapple",
     name="Cyclone cut diameter and fractional efficiency (Lapple)", topic="air pollution / particulate control",
     purpose="The cut diameter (50 % efficiency) of a cyclone from the gas viscosity, inlet width, effective number of turns, inlet velocity and densities, and the collection efficiency of a particle size by Lapple's curve.",
     inputs=[_phys("mu", "dynamic_viscosity", "gas viscosity"), _phys("W", "length", "inlet width"),
             _numin("Ne", "effective number of turns"), _phys("Vi", "speed", "inlet gas velocity"),
             _phys("rho_p", "density", "particle density"), _phys("rho_g", "density", "gas density"),
             _phys("dp", "length", "particle diameter", show=("um", 1e6))],
     outputs=[_o("d50", "length", "cut diameter"), _o("eta", "number", "collection efficiency of particles of diameter dp")],
     primary="eta", conditions=["rho_p > rho_g"],
     method="d50 = sqrt(9 mu W/(2 pi Ne Vi (rho_p - rho_g))); eta = 1/(1 + (d50/dp)^2)",
     latex=r"d_{50} = \sqrt{\frac{9\mu W}{2\pi N_e V_i(\rho_p - \rho_g)}},\quad \eta = \frac{1}{1 + (d_{50}/d_p)^2}",
     run=cyclone_lapple, reference=_ref_cyclone_lapple,
     wrong=lambda mu, W, Ne, Vi, rho_p, rho_g, dp: (lambda d50: _out(d50=d50, eta=1.0 / (1.0 + (d50 / dp) ** 2)))(
         math.sqrt(9.0 * mu * W / (Ne * Vi * (rho_p - rho_g)))),
     rand=lambda rng: {"mu": 1.8e-5, "W": _sig(rng.uniform(0.1, 0.5), 3), "Ne": _sig(rng.uniform(4.0, 8.0), 2),
                       "Vi": _sig(rng.uniform(10.0, 25.0), 3), "rho_p": _sig(rng.uniform(1000.0, 3000.0), 3), "rho_g": 1.2,
                       "dp": _sig(rng.uniform(2.0, 20.0), 3) * 1e-6},
     phrases=("Cyclone efficiency Lapple", "Cyclone cut diameter"),
     examples=[_ex({"mu": 1.8e-5, "W": 0.25, "Ne": 6.0, "Vi": 15.0, "rho_p": 1500.0, "rho_g": 1.2, "dp": 10e-6}),
               _ex({"mu": 1.85e-5, "W": 0.4, "Ne": 5.0, "Vi": 20.0, "rho_p": 2400.0, "rho_g": 1.1, "dp": 5e-6})],
     level="advanced", weak="the same published formula rearranged (efficiency as dp^2/(dp^2 + d50^2))")


def so2_emission(m_fuel, S, eta):
    _pos("the fuel rate m_fuel", m_fuel)
    _frac("the sulfur fraction S", S, lo_open=True)
    _frac("the control efficiency eta", eta, hi_open=True)
    ratio = (AW["S"] + 2 * AW["O"]) / AW["S"]
    E0 = m_fuel * S * ratio
    return _out(E=E0 * (1.0 - eta), E_uncontrolled=E0, E_kg_per_day=E0 * (1.0 - eta) * DAY)


def _ref_so2_emission(m_fuel, S, eta):
    from chemicals import molecular_weight, simple_formula_parser
    r = molecular_weight(simple_formula_parser("SO2")) / molecular_weight(simple_formula_parser("S"))
    E0 = m_fuel * S * r
    return {"E": E0 * (1.0 - eta), "E_uncontrolled": E0, "E_kg_per_day": E0 * (1.0 - eta) * 86400.0}


_add("enveng_so2_emission",
     name="Sulfur dioxide emission from fuel sulfur", topic="air pollution / emissions",
     purpose="The SO2 emission rate from a fuel burning rate and its sulfur mass fraction (all sulfur to SO2), before and after a control device.",
     inputs=[_phys("m_fuel", "mass_flow_rate", "fuel burning rate", show=("t/h", 3.6)),
             _numin("S", "sulfur mass fraction of the fuel (fraction or %)", pct=True),
             _numin("eta", "SO2 control (removal) efficiency (fraction or %; 0 if uncontrolled)", pct=True)],
     outputs=[_o("E", "mass_flow_rate", "controlled SO2 emission rate"), _o("E_uncontrolled", "mass_flow_rate", "uncontrolled SO2 emission rate"),
              _o("E_kg_per_day", "number", "controlled emission in kg per day")],
     primary="E", conditions=["all fuel sulfur oxidised to SO2"], method="E = m_fuel S (M_SO2/M_S)(1 - eta)",
     latex=r"E = \dot m_{fuel}\,S\,\frac{M_{SO_2}}{M_S}(1-\eta)",
     run=so2_emission, reference=_ref_so2_emission,
     wrong=lambda m_fuel, S, eta: (lambda E0: _out(E=E0 * (1.0 - eta), E_uncontrolled=E0, E_kg_per_day=E0 * (1.0 - eta) * DAY))(m_fuel * S),
     rand=lambda rng: {"m_fuel": _sig(rng.uniform(1.0, 100.0), 3), "S": _sig(rng.uniform(0.002, 0.04), 3), "eta": _sig(rng.uniform(0.0, 0.95), 3)},
     phrases=("SO2 emission from fuel sulfur", "Sulfur dioxide emission"),
     examples=[_ex({"m_fuel": 10.0, "S": 0.02, "eta": 0.9}), _ex({"m_fuel": 2.5, "S": 0.008, "eta": 0.0})],
     level="beginner",
     extra_q=[{"q": "SO2 emission from fuel sulfur: m_fuel = 36 t/h, S = 2%, eta = 90%", "like": 0}])


def plume_lid(Q, u, sigma_y, sigma_z, H, Lm, y, z):
    _pos("Q, u, sigma_y, sigma_z and the mixing height Lm", Q, u, sigma_y, sigma_z, Lm)
    _nonneg("H and z", H, z)
    _fin(y)
    _need(H <= Lm and z <= Lm, "the source and the receptor must lie below the mixing height")
    _need(sigma_z <= 10.0 * Lm, "sigma_z may be at most ten mixing heights (use the uniformly mixed limit beyond)")
    tot = 0.0
    nmax = int(sigma_z / Lm * 4.0) + 6
    for n in range(-nmax, nmax + 1):
        for a in (z - H + 2.0 * n * Lm, z + H + 2.0 * n * Lm):
            tot += math.exp(-a * a / (2.0 * sigma_z * sigma_z))
    C = Q / (2.0 * math.pi * u * sigma_y * sigma_z) * math.exp(-y * y / (2.0 * sigma_y * sigma_y)) * tot
    Cmix = Q / (math.sqrt(2.0 * math.pi) * u * sigma_y * Lm) * math.exp(-y * y / (2.0 * sigma_y * sigma_y))
    return _out(C=C, C_ugm3=C * 1e9, C_mixed=Cmix)


def _ref_plume_lid(Q, u, sigma_y, sigma_z, H, Lm, y, z):
    import mpmath as mp
    with mp.workdps(30):
        q = mp.exp(-mp.pi ** 2 * mp.mpf(sigma_z) ** 2 / (2 * mp.mpf(Lm) ** 2))
        s = 0
        for a in (z - H, z + H):                  # Poisson summation of the image series: a Jacobi theta function
            s += mp.sqrt(2 * mp.pi) * sigma_z / (2 * Lm) * mp.jtheta(3, mp.pi * a / (2 * Lm), q)
        C = Q / u * mp.npdf(y, 0, sigma_y) * s / (mp.sqrt(2 * mp.pi) * sigma_z)
        Cmix = Q / u * mp.npdf(y, 0, sigma_y) / Lm
    return {"C": float(C), "C_ugm3": float(C) * 1e9, "C_mixed": float(Cmix)}


def _rand_lid(rng):
    Lm = _sig(rng.uniform(300.0, 1500.0), 3)
    sz = _sig(Lm * rng.uniform(0.3, 4.0), 3)
    return {"Q": _sig(_lu(rng, 1.0, 500.0), 3), "u": _sig(rng.uniform(1.0, 10.0), 3), "sigma_y": _sig(sz * rng.uniform(1.0, 3.0), 3),
            "sigma_z": sz, "H": _sig(Lm * rng.uniform(0.05, 0.9), 3), "Lm": Lm, "y": _sig(rng.uniform(-1.0, 1.0) * sz, 3),
            "z": _sig(Lm * rng.uniform(0.0, 0.8), 3)}


_add("enveng_plume_mixing_lid",
     name="Gaussian plume trapped below an inversion lid (multiple reflections)", topic="air pollution / Gaussian plume",
     purpose="The concentration of a plume reflected both at the ground and at the base of an elevated inversion (mixing height), by the image-source series, with the fully mixed limit for comparison.",
     inputs=[_phys("Q", "mass_flow_rate", "emission rate", show=("g/s", 1e3)), _phys("u", "speed", "mean wind speed"),
             _phys("sigma_y", "length", "horizontal dispersion coefficient"), _phys("sigma_z", "length", "vertical dispersion coefficient"),
             _phys("H", "length", "effective stack height"), _phys("Lm", "length", "mixing height (inversion base)"),
             _phys("y", "length", "crosswind distance"), _phys("z", "length", "receptor height")],
     outputs=[_o("C", "mass_concentration", "concentration with ground and lid reflections"),
              _o("C_ugm3", "number", "in micrograms per cubic metre"),
              _o("C_mixed", "mass_concentration", "uniformly mixed limit Q/(sqrt(2 pi) u sy Lm) e^(-y^2/2sy^2)")],
     primary="C", conditions=["H, z <= Lm", "sigma_z <= 10 Lm"],
     method="image sources at z = +-H + 2 n Lm summed until the terms vanish",
     latex=r"C = \frac{Q e^{-y^2/2\sigma_y^2}}{2\pi u\sigma_y\sigma_z}\sum_{n=-\infty}^{\infty}\left[e^{-\frac{(z-H+2nL)^2}{2\sigma_z^2}} + e^{-\frac{(z+H+2nL)^2}{2\sigma_z^2}}\right]",
     run=plume_lid, reference=_ref_plume_lid,
     wrong=lambda Q, u, sigma_y, sigma_z, H, Lm, y, z: dict(plume_lid(Q, u, sigma_y, sigma_z, H, Lm, y, z),
                                                          C=_r(_plume(Q, u, sigma_y, sigma_z, H, y, z)), C_ugm3=_r(_plume(Q, u, sigma_y, sigma_z, H, y, z) * 1e9)),
     rand=_rand_lid,
     phrases=("Plume below an inversion lid", "Gaussian plume with mixing height"),
     examples=[_ex({"Q": 0.1, "u": 4.0, "sigma_y": 900.0, "sigma_z": 600.0, "H": 80.0, "Lm": 500.0, "y": 0.0, "z": 0.0}),
               _ex({"Q": 0.2, "u": 3.0, "sigma_y": 400.0, "sigma_z": 250.0, "H": 120.0, "Lm": 800.0, "y": 100.0, "z": 10.0})],
     level="expert")


def plume_chain(Q, u, vs, d, Ts, Ta, hs, x, cls, terrain):
    _need(cls in ("a", "b", "c", "d"), "the neutral/unstable plume-rise formula needs a class A to D")
    r = briggs_rise(vs, d, Ts, Ta, u, hs)
    H = hs + 1.6 * r["F"] ** (1.0 / 3.0) * (49.0 * r["F"] ** 0.625 if r["F"] < 55.0 else 119.0 * r["F"] ** 0.4) ** (2.0 / 3.0) / u
    sy, sz = _briggs(x, cls, terrain)
    C = Q / (math.pi * u * sy * sz) * math.exp(-H * H / (2.0 * sz * sz))
    return _out(H=H, sigma_y=sy, sigma_z=sz, C=C, C_ugm3=C * 1e9)


def _ref_plume_chain(Q, u, vs, d, Ts, Ta, hs, x, cls, terrain):
    r = _ref_briggs_rise(vs, d, Ts, Ta, u, hs)
    s = _ref_briggs(x, cls, terrain)
    c = _ref_ground_centerline(Q, u, s["sigma_y"], s["sigma_z"], r["H"])
    return {"H": r["H"], "sigma_y": s["sigma_y"], "sigma_z": s["sigma_z"], "C": c["C"], "C_ugm3": c["C_ugm3"]}


def _wrong_plume_chain(Q, u, vs, d, Ts, Ta, hs, x, cls, terrain):
    sy, sz = _briggs(x, cls, terrain)
    C = Q / (math.pi * u * sy * sz) * math.exp(-hs * hs / (2.0 * sz * sz))       # plume rise forgotten
    return _out(H=hs, sigma_y=sy, sigma_z=sz, C=C, C_ugm3=C * 1e9)


_add("enveng_stack_plume_ground_concentration",
     name="Stack plume: effective height, dispersion and ground-level concentration", topic="air pollution / Gaussian plume",
     purpose="The whole chain for a buoyant stack in neutral or unstable air: Briggs plume rise to the effective height, Briggs dispersion coefficients at the receptor distance, then the ground-level centreline concentration.",
     inputs=[_phys("Q", "mass_flow_rate", "emission rate", show=("g/s", 1e3)), _phys("u", "speed", "wind speed at the stack top")]
     + _STACK_IN + [_phys("hs", "length", "physical stack height"), _phys("x", "length", "downwind distance of the receptor"),
                    _choice("cls", ("a", "b", "c", "d"), "Pasquill stability class A-D"),
                    _choice("terrain", ("rural", "urban"), "rural or urban dispersion coefficients")],
     outputs=[_o("H", "length", "effective stack height"), _o("sigma_y", "length", "sigma_y at x"), _o("sigma_z", "length", "sigma_z at x"),
              _o("C", "mass_concentration", "ground-level centreline concentration at x"), _o("C_ugm3", "number", "in micrograms per cubic metre")],
     primary="C", conditions=["Ts > Ta", "class A-D", "100 m <= x <= 10 km"],
     method="Briggs final rise (2/3 law) -> H; Briggs sigma_y, sigma_z at x; C = Q/(pi u sy sz) exp(-H^2/2sz^2)",
     latex=r"C = \frac{Q}{\pi u\sigma_y\sigma_z}\exp\!\left(-\frac{(h_s + \Delta h)^2}{2\sigma_z^2}\right)",
     run=plume_chain, reference=_ref_plume_chain, wrong=_wrong_plume_chain,
     rand=lambda rng: dict(_rand_stack(rng), Q=_sig(_lu(rng, 1.0, 500.0), 3), x=_sig(_lu(rng, 500.0, 10000.0), 3),
                           cls=rng.choice(["a", "b", "c", "d"]), terrain=rng.choice(["rural", "urban"]), hs=_sig(rng.uniform(10.0, 80.0), 3),
                           d=_sig(rng.uniform(0.5, 2.0), 3)),
     phrases=("Stack plume ground-level concentration", "Stack plume concentration"),
     aliases={"class": "cls", "stability": "cls"},
     examples=[_ex({"Q": 0.1, "u": 6.0, "vs": 10.0, "d": 1.5, "Ts": 400.0, "Ta": 290.0, "hs": 50.0, "x": 3000.0, "cls": "c", "terrain": "rural"}),
               _ex({"Q": 0.05, "u": 4.0, "vs": 8.0, "d": 1.0, "Ts": 380.0, "Ta": 285.0, "hs": 30.0, "x": 2000.0, "cls": "d", "terrain": "urban"})],
     level="expert")


# ================================================================================================ WATER QUALITY: BOD
_K_IN = _phys("k", "first_order_rate_constant", "BOD rate constant (base e)")


def bod_exerted(L0, k, t):
    _pos("L0 and k", L0, k)
    _nonneg("the time t", t)
    rem = L0 * math.exp(-k * t)
    return _out(BOD_t=L0 - rem, L_t=rem, BOD_t_mgL=(L0 - rem) * 1e3)


def _ref_bod_exerted(L0, k, t):
    L = _ivp(lambda _t, y: [-k * y[0]], t, [L0])[0] if t > 0 else L0
    return {"BOD_t": L0 - L, "L_t": L, "BOD_t_mgL": (L0 - L) * 1e3}


def _rand_bod(rng):
    return {"L0": _sig(rng.uniform(5.0, 400.0), 3) * MG, "k": _sig(rng.uniform(0.08, 0.45), 3) / DAY, "t": _sig(rng.uniform(1.0, 20.0), 3) * DAY}


_add("enveng_bod_exerted",
     name="BOD exerted after a time (first-order)", topic="water quality / BOD kinetics",
     purpose="The BOD exerted after time t and the oxygen demand remaining, from the ultimate BOD and the first-order rate constant.",
     inputs=[_phys("L0", "mass_concentration", "ultimate (carbonaceous) BOD"), _K_IN, _phys("t", "time", "incubation or travel time", show=("d", 1.0 / DAY))],
     outputs=[_o("BOD_t", "mass_concentration", "BOD exerted after t"), _o("L_t", "mass_concentration", "BOD remaining after t"),
              _o("BOD_t_mgL", "number", "BOD exerted in mg/L")],
     primary="BOD_t", conditions=["first-order kinetics"], method="BOD_t = L0 (1 - e^(-k t)), L_t = L0 e^(-k t)",
     latex=r"BOD_t = L_0\left(1 - e^{-kt}\right)",
     run=bod_exerted, reference=_ref_bod_exerted,
     wrong=lambda L0, k, t: (lambda e: _out(BOD_t=L0 * e, L_t=L0 - L0 * e, BOD_t_mgL=L0 * e * 1e3))(math.exp(-k * t)),
     rand=_rand_bod,
     phrases=("BOD exerted", "BOD after time"),
     examples=[_ex({"L0": 0.25, "k": 0.23 / DAY, "t": 5 * DAY}), _ex({"L0": 0.03, "k": 0.1 / DAY, "t": 3 * DAY})],
     level="beginner",
     extra_q=[{"q": "BOD exerted: L0 = 250 mg/L, k = 0.23 1/day, t = 120 h", "like": 0}])


def ultimate_bod(BOD_t, k, t):
    _pos("BOD_t, k and t", BOD_t, k, t)
    L0 = BOD_t / (1.0 - math.exp(-k * t))
    return _out(L0=L0, L0_mgL=L0 * 1e3)


def _ref_ultimate_bod(BOD_t, k, t):
    frac = 1.0 - _ivp(lambda _t, y: [-k * y[0]], t, [1.0])[0]
    return {"L0": BOD_t / frac, "L0_mgL": BOD_t / frac * 1e3}


_add("enveng_ultimate_bod",
     name="Ultimate BOD from a BOD test", topic="water quality / BOD kinetics",
     purpose="The ultimate carbonaceous BOD from a BOD measured after t days (e.g. BOD5) and the rate constant.",
     inputs=[_phys("BOD_t", "mass_concentration", "BOD measured after time t (e.g. BOD5)"), _K_IN,
             _phys("t", "time", "incubation time", show=("d", 1.0 / DAY))],
     outputs=[_o("L0", "mass_concentration", "ultimate BOD"), _o("L0_mgL", "number", "ultimate BOD in mg/L")],
     primary="L0", conditions=["first-order kinetics"], method="L0 = BOD_t/(1 - e^(-k t))", latex=r"L_0 = \frac{BOD_t}{1 - e^{-kt}}",
     run=ultimate_bod, reference=_ref_ultimate_bod,
     wrong=lambda BOD_t, k, t: (lambda L0: _out(L0=L0, L0_mgL=L0 * 1e3))(BOD_t / (1.0 - 10.0 ** (-k * t))),
     rand=lambda rng: {"BOD_t": _sig(rng.uniform(5.0, 300.0), 3) * MG, "k": _sig(rng.uniform(0.08, 0.45), 3) / DAY, "t": _sig(rng.uniform(1.0, 10.0), 3) * DAY},
     phrases=("Ultimate BOD", "Ultimate BOD from BOD test", "What is the ultimate BOD", "Ultimate carbonaceous BOD",
              "Ultimate BOD from BOD5", "Ultimate BOD from a measured BOD"),
     aliases={"BOD5": "BOD_t", "BODt": "BOD_t"},
     examples=[_ex({"BOD_t": 0.2, "k": 0.23 / DAY, "t": 5 * DAY}), _ex({"BOD_t": 0.012, "k": 0.15 / DAY, "t": 5 * DAY})],
     level="beginner",
     # batch 3 (2026-11-11): exam forms of the same calculation
     extra_q=[{"q": "What is the ultimate BOD: BOD5 = 200 mg/L, k = 0.23 1/d, t = 5 d", "like": 0},
              {"q": "Ultimate carbonaceous BOD: BOD_t = 12 mg/L, k = 0.15 1/day, t = 120 h", "like": 1},
              {"q": "Ultimate BOD from BOD5: BOD5 = 200 mg/L, k = 0.23 1/d, t = 5 d", "like": 0},
              {"q": "Ultimate BOD: BOD_t = 200 mg/L, k = 0.23 1/d, t = 0 d", "outcome": "INVALID_INPUT"}])


def k_temperature(k20, theta, T):
    _pos("k20 and theta", k20, theta)
    _need(0.9 < theta < 1.2, "theta must lie between 0.9 and 1.2 (typical 1.024 to 1.135)")
    _need(T0C - 5.0 <= T <= T0C + 60.0, "the temperature must lie between -5 and 60 degC")
    kT = k20 * theta ** (T - (T0C + 20.0))
    return _out(kT=kT, kT_per_day=kT * DAY)


def _ref_k_temperature(k20, theta, T):
    import mpmath as mp
    with mp.workdps(30):
        kT = mp.mpf(k20) * mp.exp((mp.mpf(T) - mp.mpf("293.15")) * mp.log(theta))
    return {"kT": float(kT), "kT_per_day": float(kT * 86400)}


_add("enveng_rate_constant_temperature",
     name="Temperature correction of a rate constant (theta model)", topic="water quality / BOD kinetics",
     purpose="A first-order rate constant (BOD, reaeration, decay) at temperature T from its 20 degC value and the temperature coefficient theta.",
     inputs=[_phys("k20", "first_order_rate_constant", "rate constant at 20 degC"),
             _numin("theta", "temperature coefficient (1.047 BOD, 1.024 reaeration)"),
             _phys("T", "temperature", "water temperature", show=("degC", None))],
     outputs=[_o("kT", "first_order_rate_constant", "rate constant at T"), _o("kT_per_day", "number", "rate constant in 1/d")],
     primary="kT", conditions=["0.9 < theta < 1.2", "-5 degC <= T <= 60 degC"], method="kT = k20 theta^(T - 20)",
     latex=r"k_T = k_{20}\,\theta^{\,T - 20}",
     run=k_temperature, reference=_ref_k_temperature,
     wrong=lambda k20, theta, T: (lambda kT: _out(kT=kT, kT_per_day=kT * DAY))(k20 * theta ** ((T0C + 20.0) - T)),
     rand=lambda rng: {"k20": _sig(rng.uniform(0.08, 0.5), 3) / DAY, "theta": rng.choice([1.024, 1.047, 1.056, 1.072, 1.135]),
                       "T": _sig(T0C + rng.choice([rng.uniform(0.0, 17.0), rng.uniform(23.0, 35.0)]), 4)},
     phrases=("Rate constant temperature correction", "Temperature correction of rate constant"),
     examples=[_ex({"k20": 0.23 / DAY, "theta": 1.047, "T": T0C + 25.0}), _ex({"k20": 0.4 / DAY, "theta": 1.024, "T": T0C + 10.0})],
     level="intermediate", weak="the same theta model evaluated as an exponential in 30-digit mpmath")


def bod_dilution(D1, D2, P, B1=None, B2=None, f=None):
    _nonneg("D1 and D2", D1, D2)
    _frac("the sample fraction P", P, lo_open=True)
    _need(D1 > D2, "the oxygen must fall during incubation (D1 > D2)")
    seeded = [v is not None for v in (B1, B2, f)]
    _need(all(seeded) or not any(seeded), "a seeded test needs B1, B2 and the seed fraction f together")
    seed = 0.0
    if all(seeded):
        _nonneg("B1 and B2", B1, B2)
        _frac("the seed fraction f", f)
        seed = (B1 - B2) * f
    BOD = (D1 - D2 - seed) / P
    _need(BOD > 0, "the seed correction exceeds the oxygen used: no BOD can be stated")
    return _out(BOD=BOD, BOD_mgL=BOD * 1e3)


def _ref_bod_dilution(D1, D2, P, B1=None, B2=None, f=None):
    from fractions import Fraction as Fr
    used = Fr(D1) - Fr(D2)
    if B1 is not None:
        used -= (Fr(B1) - Fr(B2)) * Fr(f)
    v = float(used / Fr(P))
    return {"BOD": v, "BOD_mgL": v * 1e3}


def _rand_dilution(rng):
    D1 = _sig(rng.uniform(7.5, 9.0), 3)
    d = {"D1": D1 * MG, "D2": _sig(D1 - rng.uniform(2.5, 6.0), 3) * MG, "P": _sig(rng.choice([0.01, 0.02, 0.05, 0.1, 0.2]), 2)}
    if rng.random() < 0.5:
        B1 = _sig(rng.uniform(8.0, 9.0), 3)
        d.update(B1=B1 * MG, B2=_sig(B1 - rng.uniform(0.1, 1.0), 3) * MG, f=_sig(rng.uniform(0.9, 1.0), 3))
    return d


_add("enveng_bod_dilution_test",
     name="BOD from a dilution test (with optional seed correction)", topic="water quality / BOD measurement",
     purpose="The BOD of a sample from the dissolved oxygen of the diluted sample before and after incubation and the sample fraction, with the seed correction when the dilution water is seeded.",
     inputs=[_phys("D1", "mass_concentration", "DO of the diluted sample immediately after preparation"),
             _phys("D2", "mass_concentration", "DO of the diluted sample after incubation"),
             _numin("P", "decimal fraction of sample in the bottle (sample volume / bottle volume)", pct=True),
             _phys("B1", "mass_concentration", "DO of the seed control before incubation", required=False),
             _phys("B2", "mass_concentration", "DO of the seed control after incubation", required=False),
             _numin("f", "ratio of seed in the sample bottle to seed in the control", required=False)],
     outputs=[_o("BOD", "mass_concentration", "BOD of the sample"), _o("BOD_mgL", "number", "BOD in mg/L")],
     primary="BOD", conditions=["D1 > D2", "B1, B2 and f given together or not at all"],
     method="BOD = ((D1 - D2) - (B1 - B2) f)/P (unseeded: BOD = (D1 - D2)/P)",
     latex=r"BOD = \frac{(D_1 - D_2) - (B_1 - B_2)f}{P}",
     run=bod_dilution, reference=_ref_bod_dilution,
     wrong=lambda D1, D2, P, B1=None, B2=None, f=None: _out(BOD=(D1 - D2) * P, BOD_mgL=(D1 - D2) * P * 1e3),
     rand=_rand_dilution,
     phrases=("BOD dilution test", "BOD from dilution test"),
     examples=[_ex({"D1": 8.5e-3, "D2": 4.0e-3, "P": 0.05}), _ex({"D1": 8.8e-3, "D2": 3.6e-3, "P": 0.02, "B1": 8.9e-3, "B2": 8.3e-3, "f": 0.98})],
     level="beginner",
     extra_q=[{"q": "BOD dilution test: D1 = 8.5 mg/L, D2 = 4 mg/L, P = 5%", "like": 0},
              {"q": "BOD dilution test: D1 = 8.5 mg/L, D2 = 4 mg/L, P = 0.05, B1 = 8.9 mg/L", "outcome": "INVALID_INPUT"}],
     weak="exact rational arithmetic of the same definition")


def _bod_L0(k, t, y):
    num, den = 0.0, 0.0
    for ti, yi in zip(t, y):
        fi = 1.0 - math.exp(-k * ti)
        num += yi * fi
        den += fi * fi
    return num / den


def _bod_h(k, t, y):
    L0 = _bod_L0(k, t, y)
    s = 0.0
    for ti, yi in zip(t, y):
        s += (yi - L0 * (1.0 - math.exp(-k * ti))) * ti * math.exp(-k * ti)
    return s


def bod_rate_fit(t, y):
    _need(len(t) == len(y) and len(t) >= 3, "give at least three times and as many BOD values")
    for ti in t:
        _need(math.isfinite(ti) and ti > 0, "every time must be positive (days)")
    for yi in y:
        _need(math.isfinite(yi) and yi > 0, "every BOD value must be positive (mg/L)")
    _need(len(set(t)) == len(t), "the times must be distinct")
    n = 400
    ks = [1e-3 * 2e4 ** (i / n) for i in range(n + 1)]
    hs = [_bod_h(k, t, y) for k in ks]
    roots = [i for i in range(n) if (hs[i] > 0) != (hs[i + 1] > 0)]
    _need(len(roots) == 1, "the data do not determine one first-order rate constant between 0.001 and 20 per day")
    kd = _bisect(lambda k: _bod_h(k, t, y), ks[roots[0]], ks[roots[0] + 1])
    L0 = _bod_L0(kd, t, y)
    rss = 0.0
    for ti, yi in zip(t, y):
        rss += (yi - L0 * (1.0 - math.exp(-kd * ti))) ** 2
    return _out(k=kd / DAY, k_per_day=kd, L0=L0 * MG, L0_mgL=L0, rss=rss)


def _ref_bod_rate_fit(t, y):
    import numpy as np
    from scipy.optimize import least_squares
    t, y = np.array(t, float), np.array(y, float)
    sol = least_squares(lambda p: p[1] * (1.0 - np.exp(-p[0] * t)) - y, x0=[0.2, float(y.max()) * 1.2], method="lm",
                        xtol=1e-15, ftol=1e-15, gtol=1e-15, max_nfev=20000)
    k, L0 = float(sol.x[0]), float(sol.x[1])
    for _ in range(8):                 # full Newton steps on the two-parameter gradient (exact Hessian) to polish
        e = np.exp(-k * t)
        r = L0 * (1.0 - e) - y
        J = np.column_stack([L0 * t * e, 1.0 - e])
        g = J.T @ r
        Hs = J.T @ J + np.array([[np.sum(r * (-L0 * t * t * e)), np.sum(r * t * e)], [np.sum(r * t * e), 0.0]])
        step = np.linalg.solve(Hs, g)
        k, L0 = k - float(step[0]), L0 - float(step[1])
    rss = float(np.sum((y - L0 * (1.0 - np.exp(-k * t))) ** 2))
    return {"k": k / 86400.0, "k_per_day": k, "L0": L0 * 1e-3, "L0_mgL": L0, "rss": rss}


def _rand_bod_fit(rng):
    k, L0 = rng.uniform(0.1, 0.5), rng.uniform(5.0, 300.0)
    n = rng.randint(5, 8)
    ts = sorted(rng.sample(range(1, 21), n))
    return {"t": [float(v) for v in ts], "y": [_sig(L0 * (1.0 - math.exp(-k * v)) * (1.0 + rng.uniform(-0.03, 0.03)), 4) for v in ts]}


_add("enveng_bod_rate_fit",
     name="BOD rate constant and ultimate BOD fitted to BOD data", topic="water quality / BOD kinetics",
     purpose="The first-order rate constant k and ultimate BOD L0 that best fit a series of BOD measurements (nonlinear least squares of BOD = L0 (1 - e^(-k t))).",
     inputs=[_vecin("t", "incubation times in days", 3, 50), _vecin("y", "measured BOD values in mg/L (same order)", 3, 50)],
     outputs=[_o("k", "first_order_rate_constant", "fitted rate constant"), _o("k_per_day", "number", "fitted rate constant in 1/d"),
              _o("L0", "mass_concentration", "fitted ultimate BOD"), _o("L0_mgL", "number", "fitted ultimate BOD in mg/L"),
              _o("rss", "number", "residual sum of squares in (mg/L)^2")],
     primary="k_per_day", conditions=["at least three distinct positive times", "a single rate constant between 0.001 and 20 per day fits"],
     method="for each k the best L0 is sum(y f)/sum(f^2) with f = 1 - e^(-k t); k is the root of the stationarity condition sum((y - L0 f) t e^(-k t)) = 0 (scan and bisection)",
     latex=r"\min_{k,L_0}\sum_i\left[y_i - L_0\left(1 - e^{-kt_i}\right)\right]^2",
     run=bod_rate_fit, reference=_ref_bod_rate_fit,
     wrong=lambda t, y: (lambda r: dict(r, k=_r(r["k"] / math.log(10.0)), k_per_day=_r(r["k_per_day"] / math.log(10.0))))(bod_rate_fit(t, y)),
     rand=_rand_bod_fit,
     phrases=("BOD rate constant fit", "Fit BOD rate constant"),
     examples=[_ex({"t": [1.0, 2.0, 3.0, 5.0, 7.0, 10.0], "y": [41.0, 74.0, 98.0, 132.0, 152.0, 168.0]}),
               _ex({"t": [2.0, 4.0, 6.0, 8.0, 10.0], "y": [11.0, 18.0, 22.0, 24.0, 26.0]})],
     level="expert")


_ELEM_RX = re.compile(r"([A-Z][a-z]?)(\d*)")


def _formula_counts(text):
    s = text.strip().replace(" ", "")
    _need(len(s) > 0, "give a molecular formula such as C6H12O6")
    # element symbol (capital + optional small letter) then an optional count, read left to right with no gaps
    # (the same as matching ([A-Z][a-z]?)(\d*) contiguously; a plain loop so the Rust translation reads it too)
    upper, lower, digits = "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz", "0123456789"
    pos, counts = 0, {}
    while pos < len(s):
        _need(s[pos] in upper, "the formula may contain only element symbols and counts (C, H, O, N)")
        el = s[pos]
        pos += 1
        if pos < len(s) and s[pos] in lower:
            el = el + s[pos]
            pos += 1
        start = pos
        while pos < len(s) and s[pos] in digits:
            pos += 1
        n = int(s[start:pos]) if pos > start else 1
        _need(el in ("C", "H", "O", "N"), "only C, H, O and N are allowed")
        counts[el] = counts.get(el, 0) + n
    _need(counts.get("C", 0) > 0 and counts.get("H", 0) > 0, "the compound must contain carbon and hydrogen")
    return counts.get("C", 0), counts.get("H", 0), counts.get("O", 0), counts.get("N", 0)


def thod(formula, nitrification):
    a, b, c, d = _formula_counts(formula)
    _need(nitrification in ("yes", "no"), "nitrification: yes or no")
    _need(b >= 3 * d, "too little hydrogen to form ammonia from the nitrogen")
    n = a + b / 4.0 - c / 2.0 - 0.75 * d + (2.0 * d if nitrification == "yes" else 0.0)
    _need(n > 0, "the compound needs no oxygen to be oxidised")
    M = a * AW["C"] + b * AW["H"] + c * AW["O"] + d * AW["N"]
    return _out(thod=n * MW_O2 / M, o2_per_mol=n, molar_mass=M)


def _ref_thod(formula, nitrification):
    import numpy as np
    from chemicals import molecular_weight, simple_formula_parser
    at = simple_formula_parser(formula)
    a, b, c, d = (at.get(e, 0) for e in ("C", "H", "O", "N"))
    # unknowns: O2, CO2, H2O, NH3 (or HNO3); element balances C, H, N, O
    if nitrification == "no":
        A = [[0, 1, 0, 0], [0, 0, 2, 3], [0, 0, 0, 1], [2, -2, -1, 0]]
    else:
        A = [[0, 1, 0, 0], [0, 0, 2, 1], [0, 0, 0, 1], [2, -2, -1, -3]]
    x = np.linalg.solve(np.array(A, float), np.array([a, b, d, -c], float))
    M = molecular_weight(at)
    return {"thod": float(x[0]) * molecular_weight({"O": 2}) / M, "o2_per_mol": float(x[0]), "molar_mass": M}


def _wrong_thod(formula, nitrification):
    a, b, c, d = _formula_counts(formula)
    n = a + b / 4.0 + c / 2.0 - 0.75 * d + (2.0 * d if nitrification == "yes" else 0.0)    # the oxygen of the compound added
    M = a * AW["C"] + b * AW["H"] + c * AW["O"] + d * AW["N"]
    return _out(thod=n * MW_O2 / M, o2_per_mol=n, molar_mass=M)


_THOD_LIST = ["C6H12O6", "C2H5OH", "CH3COOH", "C5H7O2N", "C6H6O", "C3H6O", "C2H6O2", "C12H22O11", "C7H6O2", "C4H9OH",
              "C3H7NO2", "C2H5NO2", "C6H5NO2", "C10H19O3N", "C18H36O2", "C3H8O3", "CH4O", "C8H10O", "C6H14O6", "C4H8O2"]

_add("enveng_theoretical_oxygen_demand",
     name="Theoretical oxygen demand of an organic compound", topic="water quality / oxygen demand",
     purpose="The theoretical oxygen demand (g O2 per g compound) of a compound CaHbOcNd oxidised to CO2 and water, with its nitrogen released as ammonia or, with nitrification, oxidised to nitrate.",
     inputs=[_textin("formula", "molecular formula with C, H, O, N only (e.g. C6H12O6)"),
             _choice("nitrification", ("yes", "no"), "include the oxygen to nitrify ammonia (yes) or not (no)")],
     outputs=[_o("thod", "number", "theoretical oxygen demand, g O2 per g compound"), _o("o2_per_mol", "number", "mol O2 per mol compound"),
              _o("molar_mass", "number", "molar mass, g/mol")],
     primary="thod", conditions=["elements C, H, O, N only", "b >= 3d"],
     method="CaHbOcNd + (a + b/4 - c/2 - 3d/4) O2 -> a CO2 + (b - 3d)/2 H2O + d NH3; nitrification adds 2d O2",
     latex=r"\text{ThOD} = \frac{\left(a + \frac b4 - \frac c2 - \frac{3d}{4}\right)M_{O_2}}{M}",
     run=thod, reference=_ref_thod,
     wrong=_wrong_thod,
     rand=lambda rng: {"formula": rng.choice(_THOD_LIST), "nitrification": rng.choice(["yes", "no"])},
     phrases=("Theoretical oxygen demand", "ThOD"),
     examples=[_ex({"formula": "C6H12O6", "nitrification": "no"}), _ex({"formula": "C5H7O2N", "nitrification": "yes"})],
     level="intermediate")


# ================================================================================================ WATER QUALITY: oxygen sag
def _sp_deficit(kd, ka, L0, D0, t):
    if abs(ka - kd) <= 1e-9 * ka:
        return (kd * L0 * t + D0) * math.exp(-kd * t)
    return kd * L0 / (ka - kd) * (math.exp(-kd * t) - math.exp(-ka * t)) + D0 * math.exp(-ka * t)


def _sp_checks(kd, ka, L0, D0, DOsat):
    _pos("kd, ka, L0 and DOsat", kd, ka, L0, DOsat)
    _nonneg("the initial deficit D0", D0)
    _need(D0 <= DOsat, "the initial deficit cannot exceed the saturation concentration")


def streeter_phelps(kd, ka, L0, D0, t, DOsat):
    _sp_checks(kd, ka, L0, D0, DOsat)
    _nonneg("the travel time t", t)
    D = _sp_deficit(kd, ka, L0, D0, t)
    _need(D <= DOsat, "the computed deficit exceeds saturation: the river turns anaerobic and the model no longer holds")
    return _out(D=D, DO=DOsat - D, D_mgL=D * 1e3, DO_mgL=(DOsat - D) * 1e3)


def _ref_sp(kd, ka, L0, D0, t, DOsat):
    D = _ivp(lambda _t, y: [-kd * y[0], kd * y[0] - ka * y[1]], t, [L0, D0], atol=[1e-20, 1e-20])[1] if t > 0 else D0
    return {"D": D, "DO": DOsat - D, "D_mgL": D * 1e3, "DO_mgL": (DOsat - D) * 1e3}


def _rand_sp(rng):
    kd = rng.uniform(0.1, 0.5)
    ka = kd * rng.uniform(1.15, 5.0) if rng.random() < 0.85 else kd * rng.uniform(0.3, 0.85)
    DOsat = _sig(rng.uniform(7.5, 11.0), 3)
    return {"kd": _sig(kd, 3) / DAY, "ka": _sig(ka, 3) / DAY, "L0": _sig(rng.uniform(3.0, 15.0), 3) * MG,
            "D0": _sig(rng.uniform(0.2, 2.0), 3) * MG, "t": _sig(rng.uniform(0.2, 6.0), 3) * DAY, "DOsat": DOsat * MG}


_SP_IN = [_phys("kd", "first_order_rate_constant", "deoxygenation rate constant"),
          _phys("ka", "first_order_rate_constant", "reaeration rate constant"),
          _phys("L0", "mass_concentration", "ultimate BOD of the mixed river water at the outfall"),
          _phys("D0", "mass_concentration", "oxygen deficit at the outfall (DOsat - DO)")]

_add("enveng_streeter_phelps_deficit",
     name="Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time", topic="water quality / oxygen sag",
     purpose="The dissolved-oxygen deficit and the dissolved oxygen of a river at a travel time downstream of a BOD discharge (Streeter-Phelps oxygen sag).",
     inputs=_SP_IN + [_phys("t", "time", "travel time from the outfall", show=("d", 1.0 / DAY)),
                      _phys("DOsat", "mass_concentration", "saturation dissolved oxygen")],
     outputs=[_o("D", "mass_concentration", "oxygen deficit at t"), _o("DO", "mass_concentration", "dissolved oxygen at t"),
              _o("D_mgL", "number", "deficit in mg/L"), _o("DO_mgL", "number", "dissolved oxygen in mg/L")],
     primary="DO", conditions=["first-order deoxygenation and reaeration", "plug flow", "deficit stays below saturation"],
     method="D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t)",
     latex=r"D = \frac{k_d L_0}{k_a - k_d}\left(e^{-k_d t} - e^{-k_a t}\right) + D_0 e^{-k_a t}",
     run=streeter_phelps, reference=_ref_sp,
     wrong=lambda kd, ka, L0, D0, t, DOsat: (lambda D: _out(D=D, DO=DOsat - D, D_mgL=D * 1e3, DO_mgL=(DOsat - D) * 1e3))(
         kd * L0 / (ka - kd) * (math.exp(-kd * t) - math.exp(-ka * t)) + D0 * math.exp(-kd * t)),
     rand=_rand_sp,
     phrases=("Streeter-Phelps oxygen deficit", "Oxygen sag at travel time"),
     examples=[_ex({"kd": 0.23 / DAY, "ka": 0.46 / DAY, "L0": 0.010, "D0": 0.001, "t": 2 * DAY, "DOsat": 0.0091}),
               _ex({"kd": 0.3 / DAY, "ka": 0.3 / DAY, "L0": 0.008, "D0": 0.0015, "t": 1.5 * DAY, "DOsat": 0.0084})],
     level="intermediate",
     extra_q=[{"q": "Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 48 h, DOsat = 9.1 mg/L", "like": 0}])


def sag_distance(kd, ka, L0, D0, x, v, DOsat):
    _pos("the river velocity v", v)
    _nonneg("the distance x", x)
    r = streeter_phelps(kd, ka, L0, D0, x / v, DOsat)
    r["t"] = _r(x / v)
    return r


def _ref_sag_distance(kd, ka, L0, D0, x, v, DOsat):
    # integrate along the river in distance: dL/dx = -kd L / v, dD/dx = (kd L - ka D)/v
    D = _ivp(lambda _x, y: [-kd * y[0] / v, (kd * y[0] - ka * y[1]) / v], x, [L0, D0], atol=[1e-20, 1e-20])[1] if x > 0 else D0
    return {"D": D, "DO": DOsat - D, "D_mgL": D * 1e3, "DO_mgL": (DOsat - D) * 1e3, "t": x / v}


_add("enveng_oxygen_sag_distance",
     name="Dissolved oxygen at a distance downstream (Streeter-Phelps)", topic="water quality / oxygen sag",
     purpose="The oxygen deficit and dissolved oxygen at a distance downstream of an outfall from the river velocity (travel time x/v) and the Streeter-Phelps sag.",
     inputs=_SP_IN + [_phys("x", "length", "distance downstream", show=("km", 1e-3)), _phys("v", "speed", "mean river velocity"),
                      _phys("DOsat", "mass_concentration", "saturation dissolved oxygen")],
     outputs=[_o("D", "mass_concentration", "oxygen deficit at x"), _o("DO", "mass_concentration", "dissolved oxygen at x"),
              _o("D_mgL", "number", "deficit in mg/L"), _o("DO_mgL", "number", "dissolved oxygen in mg/L"), _o("t", "time", "travel time")],
     primary="DO", conditions=["plug flow at constant velocity", "deficit stays below saturation"],
     method="t = x/v, then the Streeter-Phelps deficit",
     latex=r"D(x) = \frac{k_d L_0}{k_a - k_d}\left(e^{-k_d x/v} - e^{-k_a x/v}\right) + D_0 e^{-k_a x/v}",
     run=sag_distance, reference=_ref_sag_distance,
     wrong=lambda kd, ka, L0, D0, x, v, DOsat: dict(sag_distance(ka, kd, L0, D0, x, v, DOsat) if abs(ka - kd) > 1e-6 * ka else {}, t=_r(x / v)),
     rand=lambda rng: (lambda d: dict({k: d[k] for k in ("kd", "ka", "L0", "D0", "DOsat")}, v=_sig(rng.uniform(0.1, 0.8), 3),
                                      x=_sig(rng.uniform(2.0, 80.0), 3) * 1000.0))(_rand_sp(rng)),
     phrases=("Oxygen sag at a distance", "Dissolved oxygen downstream"),
     examples=[_ex({"kd": 0.23 / DAY, "ka": 0.46 / DAY, "L0": 0.010, "D0": 0.001, "x": 30000.0, "v": 0.3, "DOsat": 0.0091}),
               _ex({"kd": 0.35 / DAY, "ka": 0.7 / DAY, "L0": 0.012, "D0": 0.0005, "x": 15000.0, "v": 0.2, "DOsat": 0.0088})],
     level="advanced")


def sp_critical(kd, ka, L0, D0, DOsat):
    _sp_checks(kd, ka, L0, D0, DOsat)
    if kd * L0 - ka * D0 <= 0:
        tc = 0.0
    elif abs(ka - kd) <= 1e-9 * ka:
        tc = (1.0 - D0 / L0) / kd
    else:
        tc = math.log(ka / kd * (1.0 - D0 * (ka - kd) / (kd * L0))) / (ka - kd)
    Dc = D0 if tc == 0.0 else kd / ka * L0 * math.exp(-kd * tc)
    _need(Dc <= DOsat, "the critical deficit exceeds saturation: the river turns anaerobic and the model no longer holds")
    return _out(t_c=tc, t_c_days=tc / DAY, D_c=Dc, DO_min=DOsat - Dc, DO_min_mgL=(DOsat - Dc) * 1e3)


def _ref_sp_critical(kd, ka, L0, D0, DOsat):
    import sympy as sp
    t = sp.symbols("t", nonnegative=True)
    if kd * L0 - ka * D0 <= 0:
        tc, Dc = 0.0, D0
    else:
        if abs(ka - kd) <= 1e-9 * ka:
            Dexpr = (kd * L0 * t + D0) * sp.exp(-kd * t)
        else:
            Dexpr = kd * L0 / (ka - kd) * (sp.exp(-kd * t) - sp.exp(-ka * t)) + D0 * sp.exp(-ka * t)
        dD = sp.lambdify(t, sp.diff(Dexpr, t), "math")
        tc = _brentq(dD, 0.0, 40.0 / min(ka, kd))
        Dc = float(Dexpr.subs(t, tc))
    return {"t_c": tc, "t_c_days": tc / 86400.0, "D_c": Dc, "DO_min": DOsat - Dc, "DO_min_mgL": (DOsat - Dc) * 1e3}


def _wrong_sp_critical(kd, ka, L0, D0, DOsat):
    tc = math.log(ka / kd) / (ka - kd)
    Dc = kd / ka * L0 * math.exp(-kd * tc)
    return _out(t_c=tc, t_c_days=tc / DAY, D_c=Dc, DO_min=DOsat - Dc, DO_min_mgL=(DOsat - Dc) * 1e3)


_add("enveng_streeter_phelps_critical_point",
     name="Critical (minimum) dissolved oxygen of the oxygen sag", topic="water quality / oxygen sag",
     purpose="The critical time at which the oxygen deficit of a river is largest, the critical deficit and the minimum dissolved oxygen (Streeter-Phelps).",
     inputs=_SP_IN + [_phys("DOsat", "mass_concentration", "saturation dissolved oxygen")],
     outputs=[_o("t_c", "time", "critical travel time"), _o("t_c_days", "number", "critical time in days"),
              _o("D_c", "mass_concentration", "critical (largest) deficit"), _o("DO_min", "mass_concentration", "minimum dissolved oxygen"),
              _o("DO_min_mgL", "number", "minimum dissolved oxygen in mg/L")],
     primary="D_c", conditions=["the critical deficit stays below saturation", "if the deficit falls from the outfall on, t_c = 0 and D_c = D0"],
     method="t_c = ln[(ka/kd)(1 - D0 (ka - kd)/(kd L0))]/(ka - kd); D_c = (kd/ka) L0 e^(-kd t_c)",
     latex=r"t_c = \frac{1}{k_a - k_d}\ln\!\left[\frac{k_a}{k_d}\left(1 - \frac{D_0(k_a - k_d)}{k_d L_0}\right)\right],\quad D_c = \frac{k_d}{k_a}L_0e^{-k_d t_c}",
     run=sp_critical, reference=_ref_sp_critical, wrong=_wrong_sp_critical,
     rand=lambda rng: {k: v for k, v in _rand_sp(rng).items() if k != "t"},
     phrases=("Streeter-Phelps critical point", "Critical dissolved oxygen", "Minimum dissolved oxygen"),
     examples=[_ex({"kd": 0.23 / DAY, "ka": 0.46 / DAY, "L0": 0.010, "D0": 0.001, "DOsat": 0.0091}),
               _ex({"kd": 0.35 / DAY, "ka": 0.7 / DAY, "L0": 0.012, "D0": 0.002, "DOsat": 0.0088})],
     level="expert")


_REAER = {"o_connor_dobbins": (3.93, 0.5, 1.5), "churchill": (5.026, 1.0, 1.67), "owens_gibbs": (5.32, 0.67, 1.85)}


def reaeration(method, u, H):
    _need(method in _REAER, "method: o_connor_dobbins, churchill or owens_gibbs")
    _pos("u and H", u, H)
    c, a, b = _REAER[method]
    kday = c * u ** a / H ** b
    return _out(ka=kday / DAY, ka_per_day=kday)


def _ref_reaeration(method, u, H):
    import mpmath as mp
    c, a, b = {"o_connor_dobbins": ("3.93", "0.5", "1.5"), "churchill": ("5.026", "1", "1.67"), "owens_gibbs": ("5.32", "0.67", "1.85")}[method]
    with mp.workdps(30):
        k = mp.mpf(c) * mp.exp(mp.mpf(a) * mp.log(u) - mp.mpf(b) * mp.log(H))
    return {"ka": float(k / 86400), "ka_per_day": float(k)}


_add("enveng_reaeration_coefficient",
     name="Stream reaeration coefficient (O'Connor-Dobbins, Churchill, Owens-Gibbs)", topic="water quality / reaeration",
     purpose="The reaeration rate constant of a stream at 20 degC from its mean velocity and depth by a chosen empirical formula.",
     inputs=[_choice("method", tuple(_REAER), "o_connor_dobbins, churchill or owens_gibbs"),
             _phys("u", "speed", "mean stream velocity"), _phys("H", "length", "mean depth")],
     outputs=[_o("ka", "first_order_rate_constant", "reaeration rate constant at 20 degC"), _o("ka_per_day", "number", "in 1/d")],
     primary="ka", conditions=["u in m/s and H in m inside the formula constants"],
     method="ka = 3.93 u^0.5/H^1.5 (O'Connor-Dobbins), 5.026 u/H^1.67 (Churchill), 5.32 u^0.67/H^1.85 (Owens-Gibbs), per day",
     latex=r"k_a = \frac{3.93\,u^{0.5}}{H^{1.5}}\ \text{(O'Connor-Dobbins)}",
     run=reaeration, reference=_ref_reaeration,
     wrong=lambda method, u, H: (lambda c: _out(ka=c[0] * H ** c[1] / u ** c[2] / DAY, ka_per_day=c[0] * H ** c[1] / u ** c[2]))(_REAER[method]),
     rand=lambda rng: {"method": rng.choice(list(_REAER)), "u": _sig(rng.uniform(0.1, 1.5), 3), "H": _sig(rng.uniform(0.3, 5.0), 3)},
     phrases=("Reaeration coefficient", "Stream reaeration rate"),
     examples=[_ex({"method": "o_connor_dobbins", "u": 0.3, "H": 2.0}), _ex({"method": "churchill", "u": 0.8, "H": 1.5})],
     level="intermediate", weak="the same published formulas evaluated in 30-digit mpmath")


def do_saturation(T, S):
    _need(T0C <= T <= T0C + 40.0, "the formula holds between 0 and 40 degC")
    _need(0.0 <= S <= 40.0, "the salinity must lie between 0 and 40 g/kg")
    lnC = (-139.34411 + 1.575701e5 / T - 6.642308e7 / T ** 2 + 1.2438e10 / T ** 3 - 8.621949e11 / T ** 4
           - S * (1.7674e-2 - 1.0754e1 / T + 2.1407e3 / T ** 2))
    C = math.exp(lnC)
    return _out(DOsat=C * MG, DOsat_mgL=C)


def _ref_do_saturation(T, S):
    import numpy as np
    x = 1.0 / T
    fresh = np.polyval([-8.621949e11, 1.2438e10, -6.642308e7, 1.575701e5, -139.34411], x)
    salt = np.polyval([2.1407e3, -1.0754e1, 1.7674e-2], x)
    C = float(np.exp(fresh - S * salt))
    return {"DOsat": C * 1e-3, "DOsat_mgL": C}


_add("enveng_do_saturation",
     name="Dissolved oxygen saturation concentration (Benson-Krause)", topic="water quality / dissolved oxygen",
     purpose="The saturation concentration of dissolved oxygen in water in equilibrium with moist air at 1 atm, from the temperature and salinity (Benson and Krause equation as adopted in Standard Methods).",
     inputs=[_phys("T", "temperature", "water temperature", show=("degC", None)), _numin("S", "salinity in g/kg (0 for fresh water)", words=("g/kg", "ppt"))],
     outputs=[_o("DOsat", "mass_concentration", "saturation dissolved oxygen"), _o("DOsat_mgL", "number", "in mg/L")],
     primary="DOsat", conditions=["0-40 degC", "0-40 g/kg salinity", "1 atm moist air"],
     method="ln C = -139.34411 + 1.575701e5/T - 6.642308e7/T^2 + 1.2438e10/T^3 - 8.621949e11/T^4 - S(1.7674e-2 - 10.754/T + 2140.7/T^2)",
     latex=r"\ln C_s = -139.34411 + \frac{1.575701\times10^5}{T} - \frac{6.642308\times10^7}{T^2} + \frac{1.2438\times10^{10}}{T^3} - \frac{8.621949\times10^{11}}{T^4} - S(\dots)",
     run=do_saturation, reference=_ref_do_saturation,
     wrong=lambda T, S: do_saturation(T - 0.15, S),       # Celsius converted with 273 instead of 273.15
     rand=lambda rng: {"T": _sig(T0C + rng.uniform(0.0, 40.0), 5), "S": rng.choice([0.0, 0.0, _sig(rng.uniform(1.0, 38.0), 3)])},
     phrases=("Dissolved oxygen saturation", "Oxygen saturation concentration"),
     examples=[_ex({"T": T0C + 20.0, "S": 0.0}), _ex({"T": T0C + 10.0, "S": 35.0})],
     level="intermediate", weak="the same published fit evaluated as polynomials in 1/T (numpy polyval)")


# ================================================================================================ WATER QUALITY: mixing and loads
def mix_two(Qw, Cw, Qr, Cr):
    _pos("the flows Qw and Qr", Qw, Qr)
    _nonneg("the concentrations Cw and Cr", Cw, Cr)
    C = (Qw * Cw + Qr * Cr) / (Qw + Qr)
    return _out(C=C, C_mgL=C * 1e3, Q=Qw + Qr)


def _ref_mix_two(Qw, Cw, Qr, Cr):
    import numpy as np
    Q, M = np.linalg.solve(np.array([[1.0, 0.0], [0.0, 1.0]]), np.array([Qw + Qr, Qw * Cw + Qr * Cr]))
    return {"C": float(M / Q), "C_mgL": float(M / Q) * 1e3, "Q": float(Q)}


_add("enveng_mixing_two_streams",
     name="Concentration after mixing a discharge with a river", topic="water quality / mass balance",
     purpose="The fully mixed concentration (BOD, DO, a pollutant) and flow just below a wastewater outfall from the flows and concentrations of the discharge and the river.",
     inputs=[_phys("Qw", "flow_rate", "wastewater (discharge) flow"), _phys("Cw", "mass_concentration", "concentration in the wastewater"),
             _phys("Qr", "flow_rate", "river flow upstream"), _phys("Cr", "mass_concentration", "concentration in the river upstream")],
     outputs=[_o("C", "mass_concentration", "mixed concentration"), _o("C_mgL", "number", "in mg/L"), _o("Q", "flow_rate", "combined flow")],
     primary="C", conditions=["complete mixing", "conservative over the mixing zone"], method="C = (Qw Cw + Qr Cr)/(Qw + Qr)",
     latex=r"C = \frac{Q_w C_w + Q_r C_r}{Q_w + Q_r}",
     run=mix_two, reference=_ref_mix_two,
     wrong=lambda Qw, Cw, Qr, Cr: (lambda C: _out(C=C, C_mgL=C * 1e3, Q=Qw + Qr))((Cw + Cr) / 2.0),
     rand=lambda rng: {"Qw": _sig(rng.uniform(0.05, 2.0), 3), "Cw": _sig(rng.uniform(5.0, 250.0), 3) * MG,
                       "Qr": _sig(rng.uniform(2.0, 50.0), 3), "Cr": _sig(rng.uniform(0.5, 5.0), 3) * MG},
     phrases=("Mixing of discharge and river", "Mixed concentration below outfall"),
     examples=[_ex({"Qw": 0.5, "Cw": 0.2, "Qr": 5.0, "Cr": 0.002}), _ex({"Qw": 1.2, "Cw": 0.03, "Qr": 10.0, "Cr": 0.0015})],
     level="beginner",
     extra_q=[{"q": "Mixing of discharge and river: Qw = 1800 m^3/h, Cw = 200 mg/L, Qr = 5 m^3/s, Cr = 2 mg/L", "like": 0}])


def mix_many(Q, C):
    _need(len(Q) == len(C) and len(Q) >= 2, "give at least two flows and as many concentrations")
    for q in Q:
        _need(math.isfinite(q) and q > 0, "every flow must be positive")
    for c in C:
        _need(math.isfinite(c) and c >= 0, "every concentration must be zero or positive")
    tq, m = 0.0, 0.0
    for q, c in zip(Q, C):
        tq += q
        m += q * c
    return _out(C_mix=m / tq, Q_total=tq)


def _ref_mix_many(Q, C):
    import numpy as np
    return {"C_mix": float(np.average(C, weights=Q)), "Q_total": float(np.sum(Q))}


_add("enveng_mixing_several_streams",
     name="Flow-weighted mixing of several streams", topic="water quality / mass balance",
     purpose="The flow-weighted concentration (or temperature) of several streams combined, and the total flow; flows in any one unit, concentrations in any one unit.",
     inputs=[_vecin("Q", "flows of the streams (any one unit, e.g. m^3/s)", 2, 50), _vecin("C", "concentrations of the streams (any one unit, e.g. mg/L)", 2, 50)],
     outputs=[_o("C_mix", "number", "mixed concentration, in the unit of C"), _o("Q_total", "number", "total flow, in the unit of Q")],
     primary="C_mix", conditions=["complete mixing", "conservative substance"], method="C = sum(Q_i C_i)/sum(Q_i)",
     latex=r"C = \frac{\sum_i Q_i C_i}{\sum_i Q_i}",
     run=mix_many, reference=_ref_mix_many,
     wrong=lambda Q, C: _out(C_mix=sum(C) / len(C), Q_total=sum(Q)),
     rand=lambda rng: (lambda n: {"Q": [_sig(rng.uniform(0.1, 20.0), 3) for _ in range(n)], "C": [_sig(rng.uniform(0.0, 300.0), 3) for _ in range(n)]})(rng.randint(2, 6)),
     phrases=("Flow-weighted mixing", "Mixing of several streams"),
     examples=[_ex({"Q": [2.0, 0.5, 1.0], "C": [3.0, 120.0, 40.0]}), _ex({"Q": [10.0, 4.0], "C": [12.0, 25.0]})],
     level="intermediate")


def lake_steady(W, Q, k, V):
    _pos("W, Q and V", W, Q, V)
    _nonneg("the decay constant k", k)
    C = W / (Q + k * V)
    return _out(C=C, C_mgL=C * 1e3, tau=V / Q)


def _ref_lake_steady(W, Q, k, V):
    C = _brentq(lambda c: W - Q * c - k * c * V, 0.0, W / Q * 2.0)
    return {"C": C, "C_mgL": C * 1e3, "tau": V / Q}


_add("enveng_lake_steady_state",
     name="Steady-state concentration in a well-mixed lake (first-order decay)", topic="water quality / lakes",
     purpose="The steady concentration of a pollutant in a lake treated as a completely mixed reactor with inflow load, outflow and first-order decay, and the hydraulic residence time.",
     inputs=[_phys("W", "mass_flow_rate", "total pollutant input (load)", show=("kg/d", DAY)), _phys("Q", "flow_rate", "outflow rate"),
             _phys("k", "first_order_rate_constant", "first-order decay constant (0 if conservative)"), _phys("V", "volume", "lake volume")],
     outputs=[_o("C", "mass_concentration", "steady-state concentration"), _o("C_mgL", "number", "in mg/L"), _o("tau", "time", "hydraulic residence time V/Q")],
     primary="C", conditions=["completely mixed", "steady state"], method="0 = W - Q C - k C V, C = W/(Q + k V)",
     latex=r"C = \frac{W}{Q + kV}",
     run=lake_steady, reference=_ref_lake_steady,
     wrong=lambda W, Q, k, V: _out(C=W / Q, C_mgL=W / Q * 1e3, tau=V / Q),
     rand=lambda rng: {"W": _sig(rng.uniform(1.0, 500.0), 3) / DAY, "Q": _sig(rng.uniform(1.0, 100.0), 3),
                       "k": _sig(rng.uniform(0.01, 0.5), 3) / DAY, "V": _sig(_lu(rng, 1e6, 1e9), 3)},
     phrases=("Lake steady-state concentration", "Completely mixed lake"),
     examples=[_ex({"W": 100.0 / DAY, "Q": 10.0, "k": 0.2 / DAY, "V": 1e7}), _ex({"W": 20.0 / DAY, "Q": 3.0, "k": 0.05 / DAY, "V": 5e6})],
     level="intermediate", weak="the same mass balance solved numerically (brentq)")


def lake_transient(W, Q, k, V, C0, t):
    _pos("W, Q and V", W, Q, V)
    _nonneg("k, C0 and t", k, C0, t)
    Css = W / (Q + k * V)
    C = Css + (C0 - Css) * math.exp(-(Q / V + k) * t)
    return _out(C=C, C_mgL=C * 1e3, C_ss=Css)


def _ref_lake_transient(W, Q, k, V, C0, t):
    C = _ivp(lambda _t, y: [(W - Q * y[0]) / V - k * y[0]], t, [C0], atol=[1e-20])[0] if t > 0 else C0
    return {"C": C, "C_mgL": C * 1e3, "C_ss": W / (Q + k * V)}


_add("enveng_lake_transient",
     name="Concentration in a well-mixed lake over time", topic="water quality / lakes",
     purpose="The concentration in a completely mixed lake a time t after a change in load, from the initial concentration, the load, outflow, volume and first-order decay.",
     inputs=[_phys("W", "mass_flow_rate", "pollutant load after the change", show=("kg/d", DAY)), _phys("Q", "flow_rate", "outflow rate"),
             _phys("k", "first_order_rate_constant", "first-order decay constant"), _phys("V", "volume", "lake volume"),
             _phys("C0", "mass_concentration", "concentration at t = 0"), _phys("t", "time", "elapsed time", show=("d", 1.0 / DAY))],
     outputs=[_o("C", "mass_concentration", "concentration at t"), _o("C_mgL", "number", "in mg/L"), _o("C_ss", "mass_concentration", "new steady state")],
     primary="C", conditions=["completely mixed", "constant flow, volume and load"],
     method="V dC/dt = W - Q C - k C V, so C = Css + (C0 - Css) e^(-(Q/V + k) t)",
     latex=r"C(t) = C_{ss} + (C_0 - C_{ss})e^{-(Q/V + k)t}",
     run=lake_transient, reference=_ref_lake_transient,
     wrong=lambda W, Q, k, V, C0, t: (lambda Css: (lambda C: _out(C=C, C_mgL=C * 1e3, C_ss=Css))(Css + (C0 - Css) * math.exp(-(Q / V) * t)))(W / (Q + k * V)),
     rand=lambda rng: {"W": _sig(rng.uniform(1.0, 500.0), 3) / DAY, "Q": _sig(rng.uniform(1.0, 100.0), 3), "k": _sig(rng.uniform(0.02, 0.5), 3) / DAY,
                       "V": _sig(_lu(rng, 1e6, 5e7), 3), "C0": _sig(rng.uniform(0.0, 0.05), 3), "t": _sig(rng.uniform(1.0, 30.0), 3) * DAY},
     phrases=("Lake concentration over time", "Lake transient concentration"),
     examples=[_ex({"W": 100.0 / DAY, "Q": 10.0, "k": 0.2 / DAY, "V": 1e7, "C0": 0.0, "t": 5 * DAY}),
               _ex({"W": 10.0 / DAY, "Q": 5.0, "k": 0.1 / DAY, "V": 2e7, "C0": 0.01, "t": 20 * DAY})],
     level="advanced")


def river_decay(C0, k, x, u):
    _pos("C0, k and u", C0, k, u)
    _nonneg("the distance x", x)
    C = C0 * math.exp(-k * x / u)
    return _out(C=C, C_mgL=C * 1e3, t=x / u, fraction_remaining=C / C0)


def _ref_river_decay(C0, k, x, u):
    C = _ivp(lambda _x, y: [-k / u * y[0]], x, [C0], atol=[1e-20])[0] if x > 0 else C0
    return {"C": C, "C_mgL": C * 1e3, "t": x / u, "fraction_remaining": C / C0}


_add("enveng_river_first_order_decay",
     name="First-order decay of a pollutant downstream (plug flow)", topic="water quality / rivers",
     purpose="The concentration of a decaying pollutant (BOD, coliforms, a chemical) a distance downstream in a river in plug flow, and the travel time.",
     inputs=[_phys("C0", "mass_concentration", "concentration at the start (after mixing)"),
             _phys("k", "first_order_rate_constant", "first-order decay constant"),
             _phys("x", "length", "distance downstream", show=("km", 1e-3)), _phys("u", "speed", "mean velocity")],
     outputs=[_o("C", "mass_concentration", "concentration at x"), _o("C_mgL", "number", "in mg/L"), _o("t", "time", "travel time x/u"),
              _o("fraction_remaining", "number", "C/C0")],
     primary="C", conditions=["plug flow", "first-order decay"], method="C = C0 e^(-k x/u)", latex=r"C = C_0\,e^{-kx/u}",
     run=river_decay, reference=_ref_river_decay,
     wrong=lambda C0, k, x, u: (lambda C: _out(C=C, C_mgL=C * 1e3, t=x / u, fraction_remaining=C / C0))(C0 * 10.0 ** (-k * x / u)),
     rand=lambda rng: {"C0": _sig(rng.uniform(1.0, 100.0), 3) * MG, "k": _sig(rng.uniform(0.05, 2.0), 3) / DAY,
                       "x": _sig(rng.uniform(1.0, 100.0), 3) * 1000.0, "u": _sig(rng.uniform(0.1, 1.0), 3)},
     phrases=("River first-order decay", "Pollutant decay downstream"),
     examples=[_ex({"C0": 0.02, "k": 0.3 / DAY, "x": 40000.0, "u": 0.25}), _ex({"C0": 0.005, "k": 1.0 / DAY, "x": 10000.0, "u": 0.5})],
     level="intermediate")


def pollutant_load(Q, C):
    _pos("the flow Q", Q)
    _nonneg("the concentration C", C)
    W = Q * C
    return _out(W=W, W_kg_per_day=W * DAY, W_t_per_year=W * YEAR / 1000.0)


def _ref_pollutant_load(Q, C):
    from fractions import Fraction as Fr
    W = float(Fr(Q) * Fr(C))
    return {"W": W, "W_kg_per_day": float(Fr(Q) * Fr(C) * 86400), "W_t_per_year": float(Fr(Q) * Fr(C) * Fr(31557600) / 1000)}


_add("enveng_pollutant_load",
     name="Pollutant mass load of a flow", topic="water quality / loads",
     purpose="The mass load (rate) carried by a flow at a concentration, in kg/s, kg per day and tonnes per year.",
     inputs=[_phys("Q", "flow_rate", "flow rate"), _phys("C", "mass_concentration", "concentration")],
     outputs=[_o("W", "mass_flow_rate", "mass load"), _o("W_kg_per_day", "number", "load in kg per day"),
              _o("W_t_per_year", "number", "load in tonnes per (365.25-day) year")],
     primary="W", conditions=["Q > 0"], method="W = Q C", latex=r"W = Q\,C",
     run=pollutant_load, reference=_ref_pollutant_load,
     wrong=lambda Q, C: _out(W=Q * C, W_kg_per_day=Q * C * HOUR, W_t_per_year=Q * C * YEAR),
     rand=lambda rng: {"Q": _sig(rng.uniform(0.01, 20.0), 3), "C": _sig(rng.uniform(0.1, 500.0), 3) * MG},
     phrases=("Pollutant load", "Mass load of a flow"),
     examples=[_ex({"Q": 0.5, "C": 0.03}), _ex({"Q": 2.0, "C": 0.0004})],
     level="beginner",
     extra_q=[{"q": "Pollutant load: Q = 43200 m^3/d, C = 30 mg/L", "like": 0}], weak="exact rational arithmetic of the same definition")


# ================================================================================================ TREATMENT: hydraulics of basins
def detention_time(V, Q):
    _pos("V and Q", V, Q)
    return _out(theta=V / Q, theta_h=V / Q / HOUR)


def _ref_detention_time(V, Q):
    from fractions import Fraction as Fr
    return {"theta": float(Fr(V) / Fr(Q)), "theta_h": float(Fr(V) / Fr(Q) / 3600)}


_add("enveng_detention_time",
     name="Hydraulic detention time", topic="water treatment / basin hydraulics",
     purpose="The hydraulic detention (residence) time of a tank or basin from its volume and the flow.",
     inputs=[_phys("V", "volume", "tank volume"), _phys("Q", "flow_rate", "flow rate", show=("m^3/d", DAY))],
     outputs=[_o("theta", "time", "detention time V/Q"), _o("theta_h", "number", "detention time in hours")],
     primary="theta", conditions=["V, Q > 0"], method="theta = V/Q", latex=r"\theta = \frac{V}{Q}",
     run=detention_time, reference=_ref_detention_time,
     wrong=lambda V, Q: _out(theta=V / Q, theta_h=V / Q / 60.0),
     rand=lambda rng: {"V": _sig(rng.uniform(50.0, 20000.0), 3), "Q": _sig(rng.uniform(500.0, 100000.0), 3) / DAY},
     phrases=("Detention time", "Hydraulic detention time"),
     examples=[_ex({"V": 1000.0, "Q": 12000.0 / DAY}), _ex({"V": 250.0, "Q": 0.05})],
     level="beginner",
     extra_q=[{"q": "Detention time: V = 1000 m^3, Q = 500 m^3/h", "like": 0}], weak="exact rational arithmetic of the same definition")


def overflow_rate(Q, D):
    _pos("Q and D", Q, D)
    A = math.pi * D * D / 4.0
    return _out(A=A, v0=Q / A, v0_m_per_day=Q / A * DAY)


def _ref_overflow_rate(Q, D):
    import numpy as np
    A = np.pi * (D / 2.0) ** 2
    return {"A": float(A), "v0": float(Q / A), "v0_m_per_day": float(Q / A * 86400.0)}


_add("enveng_overflow_rate_circular",
     name="Surface overflow rate of a circular clarifier", topic="water treatment / sedimentation",
     purpose="The surface area and the surface overflow rate (surface loading) of a circular settling tank from the flow and the tank diameter.",
     inputs=[_phys("Q", "flow_rate", "flow rate", show=("m^3/d", DAY)), _phys("D", "length", "tank diameter")],
     outputs=[_o("A", "area", "surface area"), _o("v0", "speed", "overflow rate Q/A"), _o("v0_m_per_day", "number", "overflow rate in m^3/(m^2 d)")],
     primary="v0", conditions=["Q, D > 0"], method="A = pi D^2/4; v0 = Q/A", latex=r"v_0 = \frac{Q}{\pi D^2/4}",
     run=overflow_rate, reference=_ref_overflow_rate,
     wrong=lambda Q, D: (lambda A: _out(A=A, v0=Q / A, v0_m_per_day=Q / A * DAY))(math.pi * D * D),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 100000.0), 3) / DAY, "D": _sig(rng.uniform(5.0, 60.0), 3)},
     phrases=("Overflow rate of a circular clarifier", "Surface overflow rate"),
     examples=[_ex({"Q": 10000.0 / DAY, "D": 20.0}), _ex({"Q": 0.5, "D": 35.0})],
     level="beginner")


def sed_basin(Q, v0, theta):
    _pos("Q, v0 and theta", Q, v0, theta)
    A = Q / v0
    V = Q * theta
    return _out(A=A, V=V, depth=V / A, D=math.sqrt(4.0 * A / math.pi))


def _ref_sed_basin(Q, v0, theta):
    import numpy as np
    A = Q / v0
    return {"A": A, "V": Q * theta, "depth": float(v0 * theta), "D": float(2.0 * np.sqrt(A / np.pi))}


_add("enveng_sedimentation_basin_sizing",
     name="Sedimentation basin sizing from overflow rate and detention time", topic="water treatment / sedimentation",
     purpose="The surface area, volume, side-water depth and (circular) diameter of a settling basin from the design flow, the design overflow rate and the detention time.",
     inputs=[_phys("Q", "flow_rate", "design flow", show=("m^3/d", DAY)),
             _phys("v0", "speed", "design overflow rate (surface loading)", show=("m/d", DAY)),
             _phys("theta", "time", "design detention time", show=("h", 1.0 / HOUR))],
     outputs=[_o("A", "area", "surface area Q/v0"), _o("V", "volume", "volume Q theta"), _o("depth", "length", "depth V/A = v0 theta"),
              _o("D", "length", "diameter of one circular basin")],
     primary="A", conditions=["one basin"], method="A = Q/v0; V = Q theta; depth = V/A; D = sqrt(4A/pi)",
     latex=r"A = \frac{Q}{v_0},\ V = Q\theta,\ h = v_0\theta",
     run=sed_basin, reference=_ref_sed_basin,
     wrong=lambda Q, v0, theta: (lambda A, V: _out(A=A, V=V, depth=A / V, D=math.sqrt(4.0 * A / math.pi)))(Q / v0, Q * theta),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 100000.0), 3) / DAY, "v0": _sig(rng.uniform(20.0, 60.0), 3) / DAY,
                       "theta": _sig(rng.uniform(1.5, 4.0), 3) * HOUR},
     phrases=("Sedimentation basin sizing", "Settling basin design"),
     examples=[_ex({"Q": 15000.0 / DAY, "v0": 40.0 / DAY, "theta": 2.0 * HOUR}), _ex({"Q": 40000.0 / DAY, "v0": 30.0 / DAY, "theta": 3.0 * HOUR})],
     level="intermediate")


def weir_loading(Q, D):
    _pos("Q and D", Q, D)
    WL = Q / (math.pi * D)
    return _out(weir_length=math.pi * D, WL=WL, WL_m3_per_m_day=WL * DAY)


def _ref_weir_loading(Q, D):
    import numpy as np
    P = 2.0 * np.pi * (D / 2.0)
    return {"weir_length": float(P), "WL": float(Q / P), "WL_m3_per_m_day": float(Q / P * 86400.0)}


_add("enveng_weir_loading_rate",
     name="Weir loading rate of a circular clarifier", topic="water treatment / sedimentation",
     purpose="The weir length and weir loading rate of a circular clarifier with a peripheral weir.",
     inputs=[_phys("Q", "flow_rate", "flow rate", show=("m^3/d", DAY)), _phys("D", "length", "weir (tank) diameter")],
     outputs=[_o("weir_length", "length", "weir length pi D"), _o("WL", "volume_transport_per_width", "weir loading rate Q/(pi D)"),
              _o("WL_m3_per_m_day", "number", "weir loading in m^3/(m d)")],
     primary="WL", conditions=["peripheral weir"], method="WL = Q/(pi D)", latex=r"WL = \frac{Q}{\pi D}",
     run=weir_loading, reference=_ref_weir_loading,
     wrong=lambda Q, D: _out(weir_length=math.pi * D / 2.0, WL=Q / (math.pi * D / 2.0), WL_m3_per_m_day=Q / (math.pi * D / 2.0) * DAY),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 100000.0), 3) / DAY, "D": _sig(rng.uniform(5.0, 60.0), 3)},
     phrases=("Weir loading rate", "Weir overflow rate"),
     examples=[_ex({"Q": 10000.0 / DAY, "D": 20.0}), _ex({"Q": 30000.0 / DAY, "D": 32.0})],
     level="beginner")


# ================================================================================================ TREATMENT: settling
def stokes(d, rho_p, rho_w, mu):
    _pos("d, rho_p, rho_w and mu", d, rho_p, rho_w, mu)
    _need(rho_p > rho_w, "the particle must be denser than the water")
    vs = _stokes_v(d, rho_p, rho_w, mu)
    Re = rho_w * vs * d / mu
    _need(Re <= 1.0, "Stokes' law needs a particle Reynolds number of at most 1 (use the general terminal velocity)")
    return _out(vs=vs, Re=Re, vs_m_per_h=vs * HOUR)


def _ref_stokes(d, rho_p, rho_w, mu):
    from fluids import v_terminal
    v = v_terminal(d, rho_p, rho_w, mu, Method="Stokes")
    return {"vs": v, "Re": rho_w * v * d / mu, "vs_m_per_h": v * 3600.0}


_add("enveng_stokes_settling_velocity",
     name="Stokes settling velocity of a discrete particle", topic="water treatment / sedimentation",
     purpose="The terminal settling velocity of a small spherical particle in water by Stokes' law, with the particle Reynolds number that confirms laminar settling.",
     inputs=[_phys("d", "length", "particle diameter", show=("mm", 1e3)), _phys("rho_p", "density", "particle density"),
             _phys("rho_w", "density", "water density"), _phys("mu", "dynamic_viscosity", "water viscosity")],
     outputs=[_o("vs", "speed", "settling velocity"), _o("Re", "number", "particle Reynolds number"), _o("vs_m_per_h", "number", "in m/h")],
     primary="vs", conditions=["Re <= 1", "rho_p > rho_w"], method="vs = g (rho_p - rho_w) d^2/(18 mu)",
     latex=r"v_s = \frac{g(\rho_p - \rho_w)d^2}{18\mu}",
     run=stokes, reference=_ref_stokes,
     wrong=lambda d, rho_p, rho_w, mu: (lambda v: _out(vs=v, Re=rho_w * v * d / mu, vs_m_per_h=v * HOUR))(G0 * rho_p * d * d / (18.0 * mu)),
     rand=lambda rng: {"d": _sig(rng.uniform(0.005, 0.08), 3) * 1e-3, "rho_p": _sig(rng.uniform(1050.0, 2650.0), 4), "rho_w": 998.2,
                       "mu": _sig(rng.uniform(0.8e-3, 1.5e-3), 3)},
     phrases=("Stokes settling velocity", "Settling velocity by Stokes law"),
     examples=[_ex({"d": 0.05e-3, "rho_p": 2650.0, "rho_w": 998.2, "mu": 1.002e-3}), _ex({"d": 0.02e-3, "rho_p": 1200.0, "rho_w": 999.7, "mu": 1.307e-3})],
     level="beginner")


def _rouse_cd(Re):
    return 24.0 / Re + 3.0 / math.sqrt(Re) + 0.34


def terminal_velocity(d, rho_p, rho_f, mu):
    _pos("d, rho_p, rho_f and mu", d, rho_p, rho_f, mu)
    _need(rho_p > rho_f, "the particle must be denser than the fluid")
    rhs = 4.0 / 3.0 * G0 * d * (rho_p - rho_f) / rho_f
    vst = _stokes_v(d, rho_p, rho_f, mu)

    def f(v):
        return rhs - _rouse_cd(rho_f * v * d / mu) * v * v
    v = _bisect(f, vst * 1e-12, vst)
    Re = rho_f * v * d / mu
    _need(Re <= 2e5, "the drag law holds up to a Reynolds number of 2e5")
    return _out(vt=v, Re=Re, Cd=_rouse_cd(Re))


def _ref_terminal_velocity(d, rho_p, rho_f, mu):
    from fluids import v_terminal
    from fluids.drag import Rouse
    v = v_terminal(d, rho_p, rho_f, mu, Method="Rouse")
    Re = rho_f * v * d / mu
    return {"vt": v, "Re": Re, "Cd": Rouse(Re)}


_add("enveng_terminal_settling_velocity",
     name="Terminal settling velocity in any flow regime (drag coefficient iteration)", topic="water treatment / sedimentation",
     purpose="The terminal settling velocity of a sphere in a fluid for laminar, transitional or turbulent settling, solving the force balance with the drag law Cd = 24/Re + 3/sqrt(Re) + 0.34.",
     inputs=[_phys("d", "length", "particle diameter", show=("mm", 1e3)), _phys("rho_p", "density", "particle density"),
             _phys("rho_f", "density", "fluid density"), _phys("mu", "dynamic_viscosity", "fluid viscosity")],
     outputs=[_o("vt", "speed", "terminal settling velocity"), _o("Re", "number", "particle Reynolds number"), _o("Cd", "number", "drag coefficient")],
     primary="vt", conditions=["Re <= 2e5", "rho_p > rho_f"],
     method="vt^2 Cd(Re) = (4/3) g d (rho_p - rho_f)/rho_f with Cd = 24/Re + 3/sqrt(Re) + 0.34, solved by bisection below the Stokes velocity",
     latex=r"v_t = \sqrt{\frac{4g d(\rho_p - \rho_f)}{3C_D\rho_f}},\quad C_D = \frac{24}{Re} + \frac{3}{\sqrt{Re}} + 0.34",
     run=terminal_velocity, reference=_ref_terminal_velocity,
     wrong=lambda d, rho_p, rho_f, mu: (lambda v: _out(vt=v, Re=rho_f * v * d / mu, Cd=24.0 / (rho_f * v * d / mu)))(_stokes_v(d, rho_p, rho_f, mu)),
     rand=lambda rng: {"d": _sig(_lu(rng, 0.1, 3.0), 3) * 1e-3, "rho_p": _sig(rng.uniform(1300.0, 2700.0), 4), "rho_f": 998.2,
                       "mu": _sig(rng.uniform(0.8e-3, 1.4e-3), 3)},
     phrases=("Terminal settling velocity", "General settling velocity"),
     examples=[_ex({"d": 0.5e-3, "rho_p": 2650.0, "rho_f": 998.2, "mu": 1.002e-3}), _ex({"d": 2.0e-3, "rho_p": 1500.0, "rho_f": 998.2, "mu": 1.0e-3})],
     level="advanced")


def removal_fraction(vs, Q, A):
    _pos("vs, Q and A", vs, Q, A)
    v0 = Q / A
    return _out(v0=v0, R=min(1.0, vs / v0))


def _ref_removal_fraction(vs, Q, A):
    from fractions import Fraction as Fr
    r = Fr(vs) * Fr(A) / Fr(Q)
    return {"v0": float(Fr(Q) / Fr(A)), "R": float(min(Fr(1), r))}


_add("enveng_ideal_basin_removal",
     name="Fraction of discrete particles removed in an ideal settling basin", topic="water treatment / sedimentation",
     purpose="The overflow rate of an ideal (Camp) settling basin and the fraction of particles of one settling velocity it removes (all if vs >= v0, otherwise vs/v0).",
     inputs=[_phys("vs", "speed", "settling velocity of the particles", show=("m/h", HOUR)), _phys("Q", "flow_rate", "flow", show=("m^3/d", DAY)),
             _phys("A", "area", "basin surface area")],
     outputs=[_o("v0", "speed", "overflow rate Q/A"), _o("R", "number", "fraction removed")],
     primary="R", conditions=["ideal basin, discrete (type I) settling"], method="v0 = Q/A; R = min(1, vs/v0)", latex=r"R = \min\left(1, \frac{v_s}{v_0}\right)",
     run=removal_fraction, reference=_ref_removal_fraction,
     wrong=lambda vs, Q, A: _out(v0=Q / A, R=min(1.0, (Q / A) / vs)),
     rand=lambda rng: (lambda Q, A: {"Q": Q, "A": A, "vs": _sig(Q / A * rng.uniform(0.1, 0.9), 3)})(_sig(rng.uniform(1000.0, 50000.0), 3) / DAY, _sig(rng.uniform(50.0, 1500.0), 3)),
     phrases=("Ideal settling basin removal", "Fraction of particles removed"),
     examples=[_ex({"vs": 0.5 / HOUR, "Q": 10000.0 / DAY, "A": 400.0}), _ex({"vs": 2.0 / HOUR, "Q": 20000.0 / DAY, "A": 300.0})],
     level="intermediate", weak="exact rational arithmetic of the same definition")


# ================================================================================================ TREATMENT: filtration
def filter_rate(Q, A, N):
    _pos("Q and A", Q, A)
    _need(isinstance(N, int) and N >= 2, "give at least two filters (one may be out of service)")
    v = Q / (N * A)
    v1 = Q / ((N - 1) * A)
    return _out(v=v, v_m_per_h=v * HOUR, v_one_out=v1, v_one_out_m_per_h=v1 * HOUR)


def _ref_filter_rate(Q, A, N):
    from fractions import Fraction as Fr
    v, v1 = Fr(Q) / (N * Fr(A)), Fr(Q) / ((N - 1) * Fr(A))
    return {"v": float(v), "v_m_per_h": float(v * 3600), "v_one_out": float(v1), "v_one_out_m_per_h": float(v1 * 3600)}


_add("enveng_filtration_rate",
     name="Filtration (hydraulic loading) rate with one filter out of service", topic="water treatment / filtration",
     purpose="The filtration rate of a bank of identical filters, with all in service and with one out of service (for backwashing).",
     inputs=[_phys("Q", "flow_rate", "plant flow", show=("m^3/d", DAY)), _phys("A", "area", "area of one filter"),
             _intin("N", "number of filters", 2, 200)],
     outputs=[_o("v", "speed", "filtration rate with all filters"), _o("v_m_per_h", "number", "in m/h"),
              _o("v_one_out", "speed", "rate with one filter out of service"), _o("v_one_out_m_per_h", "number", "in m/h")],
     primary="v", conditions=["N >= 2 identical filters"], method="v = Q/(N A); v' = Q/((N - 1) A)", latex=r"v = \frac{Q}{NA},\quad v' = \frac{Q}{(N-1)A}",
     run=filter_rate, reference=_ref_filter_rate,
     wrong=lambda Q, A, N: _out(v=Q / (N * A), v_m_per_h=Q / (N * A) * HOUR, v_one_out=Q / (N * A), v_one_out_m_per_h=Q / (N * A) * HOUR),
     rand=lambda rng: {"Q": _sig(rng.uniform(5000.0, 200000.0), 3) / DAY, "A": _sig(rng.uniform(20.0, 150.0), 3), "N": rng.randint(2, 12)},
     phrases=("Filtration rate", "Filter loading rate"),
     examples=[_ex({"Q": 40000.0 / DAY, "A": 50.0, "N": 6}), _ex({"Q": 12000.0 / DAY, "A": 30.0, "N": 4})],
     level="beginner", weak="exact rational arithmetic of the same definition")


def ergun_headloss(d, eps, psi, L, v, rho, mu):
    _pos("d, L, v, rho and mu", d, L, v, rho, mu)
    _frac("the porosity eps", eps, lo_open=True, hi_open=True)
    _frac("the sphericity psi", psi, lo_open=True)
    dp = psi * d
    dPdL = 150.0 * mu * (1.0 - eps) ** 2 * v / (eps ** 3 * dp * dp) + 1.75 * rho * (1.0 - eps) * v * v / (eps ** 3 * dp)
    return _out(dP=dPdL * L, h=dPdL * L / (rho * G0), Re=rho * v * dp / (mu * (1.0 - eps)))


def _ref_ergun_headloss(d, eps, psi, L, v, rho, mu):
    from fluids import Ergun
    dP = Ergun(dp=psi * d, voidage=eps, vs=v, rho=rho, mu=mu, L=L)
    return {"dP": dP, "h": dP / (rho * 9.80665), "Re": rho * v * psi * d / (mu * (1.0 - eps))}


_add("enveng_filter_clean_bed_headloss",
     name="Clean-bed head loss through a granular filter (Ergun / Carman-Kozeny)", topic="water treatment / filtration",
     purpose="The head loss through a clean granular filter bed from the grain size, sphericity, porosity, depth and filtration rate (Ergun equation, the Carman-Kozeny form with friction factor 150/Re + 1.75).",
     inputs=[_phys("d", "length", "grain diameter", show=("mm", 1e3)), _numin("eps", "bed porosity (fraction)", pct=True),
             _numin("psi", "grain sphericity (fraction, 1 for spheres)"), _phys("L", "length", "bed depth"),
             _phys("v", "speed", "filtration (approach) velocity", show=("m/h", HOUR)), _phys("rho", "density", "water density"),
             _phys("mu", "dynamic_viscosity", "water viscosity")],
     outputs=[_o("dP", "pressure", "pressure drop"), _o("h", "length", "head loss"), _o("Re", "number", "bed Reynolds number rho v psi d/(mu (1 - eps))")],
     primary="h", conditions=["0 < eps < 1", "0 < psi <= 1", "clean bed"],
     method="dP/L = 150 mu (1-eps)^2 v/(eps^3 (psi d)^2) + 1.75 rho (1-eps) v^2/(eps^3 psi d); h = dP/(rho g)",
     latex=r"\frac{h}{L} = \frac{150\mu(1-\varepsilon)^2 v}{\rho g\varepsilon^3(\psi d)^2} + \frac{1.75(1-\varepsilon)v^2}{g\varepsilon^3\psi d}",
     run=ergun_headloss, reference=_ref_ergun_headloss,
     wrong=lambda d, eps, psi, L, v, rho, mu: (lambda dPdL: _out(dP=dPdL * L, h=dPdL * L / (rho * G0), Re=rho * v * psi * d / (mu * (1.0 - eps))))(
         150.0 * mu * (1.0 - eps) ** 2 * v / (eps ** 2 * (psi * d) ** 2) + 1.75 * rho * (1.0 - eps) * v * v / (eps ** 2 * psi * d)),
     rand=lambda rng: {"d": _sig(rng.uniform(0.4, 1.5), 3) * 1e-3, "eps": _sig(rng.uniform(0.38, 0.5), 3), "psi": _sig(rng.uniform(0.7, 1.0), 3),
                       "L": _sig(rng.uniform(0.5, 1.5), 3), "v": _sig(rng.uniform(5.0, 15.0), 3) / HOUR, "rho": 998.2, "mu": 1.002e-3},
     phrases=("Filter clean-bed head loss", "Clean bed head loss"),
     examples=[_ex({"d": 0.5e-3, "eps": 0.4, "psi": 0.85, "L": 0.7, "v": 8.0 / HOUR, "rho": 998.2, "mu": 1.002e-3}),
               _ex({"d": 1.0e-3, "eps": 0.45, "psi": 0.75, "L": 1.0, "v": 12.0 / HOUR, "rho": 999.7, "mu": 1.307e-3})],
     level="advanced")


def backwash_expansion(L, eps, vb, vs):
    _pos("L, vb and vs", L, vb, vs)
    _frac("the porosity eps", eps, lo_open=True, hi_open=True)
    eps_e = (vb / vs) ** 0.22
    _need(eps < eps_e < 1.0, "the backwash rate must fluidise the bed (eps < (vb/vs)^0.22 < 1)")
    Le = L * (1.0 - eps) / (1.0 - eps_e)
    return _out(eps_e=eps_e, Le=Le, expansion=Le / L - 1.0)


def _ref_backwash_expansion(L, eps, vb, vs):
    import numpy as np
    eps_e = float(np.exp(0.22 * (np.log(vb) - np.log(vs))))
    solids = L * (1.0 - eps)                    # the solids volume per area is conserved
    Le = solids / (1.0 - eps_e)
    return {"eps_e": eps_e, "Le": Le, "expansion": (Le - L) / L}


_add("enveng_backwash_bed_expansion",
     name="Filter bed expansion during backwash", topic="water treatment / filtration",
     purpose="The expanded porosity, expanded depth and percentage expansion of a filter bed fluidised by backwash, from the backwash rate and the grains' settling velocity (expanded porosity (vb/vs)^0.22).",
     inputs=[_phys("L", "length", "unexpanded bed depth"), _numin("eps", "unexpanded porosity (fraction)", pct=True),
             _phys("vb", "speed", "backwash rate", show=("m/h", HOUR)), _phys("vs", "speed", "settling velocity of the grains", show=("m/h", HOUR))],
     outputs=[_o("eps_e", "number", "expanded porosity"), _o("Le", "length", "expanded bed depth"), _o("expansion", "number", "fractional expansion Le/L - 1")],
     primary="Le", conditions=["the bed is fluidised: eps < (vb/vs)^0.22 < 1"],
     method="eps_e = (vb/vs)^0.22; Le = L (1 - eps)/(1 - eps_e)", latex=r"\varepsilon_e = \left(\frac{v_b}{v_s}\right)^{0.22},\quad L_e = \frac{L(1-\varepsilon)}{1-\varepsilon_e}",
     run=backwash_expansion, reference=_ref_backwash_expansion,
     wrong=lambda L, eps, vb, vs: (lambda e: _out(eps_e=e, Le=L * (1.0 - e) / (1.0 - eps), expansion=(1.0 - e) / (1.0 - eps) - 1.0))((vb / vs) ** 0.22),
     rand=lambda rng: (lambda vs, eps: {"vs": vs, "eps": eps, "L": _sig(rng.uniform(0.5, 1.2), 3),
                                        "vb": _sig(vs * rng.uniform(eps ** (1 / 0.22) * 1.3, 0.6), 3)})(_sig(rng.uniform(150.0, 500.0), 3) / HOUR, _sig(rng.uniform(0.38, 0.48), 3)),
     phrases=("Backwash bed expansion", "Filter backwash expansion"),
     examples=[_ex({"L": 0.7, "eps": 0.42, "vb": 50.0 / HOUR, "vs": 300.0 / HOUR}), _ex({"L": 1.0, "eps": 0.4, "vb": 30.0 / HOUR, "vs": 250.0 / HOUR})],
     level="advanced", weak="the same published correlation evaluated through logarithms and the conserved solids volume")


# ================================================================================================ TREATMENT: mixing
def velocity_gradient(P, mu, V, t=None):
    _pos("P, mu and V", P, mu, V)
    G = math.sqrt(P / (mu * V))
    if t is None:
        return _out(G=G)
    _pos("the time t", t)
    return _out(G=G, Gt=G * t)


def _ref_velocity_gradient(P, mu, V, t=None):
    import numpy as np
    G = float(np.sqrt(P) / np.sqrt(mu * V))
    return {"G": G} if t is None else {"G": G, "Gt": G * t}


_add("enveng_velocity_gradient",
     name="Mean velocity gradient G (rapid mixing and flocculation)", topic="water treatment / mixing and flocculation",
     purpose="The root-mean-square velocity gradient G of a mixing or flocculation tank from the power dissipated, the viscosity and the volume, and the Camp number G t when the detention time is given.",
     inputs=[_phys("P", "power", "power dissipated in the water"), _phys("mu", "dynamic_viscosity", "water viscosity"),
             _phys("V", "volume", "tank volume"), _phys("t", "time", "detention time (for G t)", required=False, show=("min", 1.0 / 60.0))],
     outputs=[_o("G", "velocity_gradient", "mean velocity gradient"), _o("Gt", "number", "Camp number G t (dimensionless)")],
     primary="G", conditions=["P, mu, V > 0"], method="G = sqrt(P/(mu V)); Camp number G t", latex=r"G = \sqrt{\frac{P}{\mu V}}",
     run=velocity_gradient, reference=_ref_velocity_gradient,
     wrong=lambda P, mu, V, t=None: (lambda G: _out(G=G) if t is None else _out(G=G, Gt=G * t))(P / (mu * V)),
     rand=lambda rng: dict({"P": _sig(rng.uniform(50.0, 20000.0), 3), "mu": _sig(rng.uniform(0.9e-3, 1.4e-3), 3), "V": _sig(rng.uniform(5.0, 1000.0), 3)},
                           **({"t": _sig(rng.uniform(1.0, 40.0), 3) * 60.0} if rng.random() < 0.5 else {})),
     phrases=("Velocity gradient", "Mean velocity gradient G"),
     examples=[_ex({"P": 2000.0, "mu": 1.002e-3, "V": 50.0}), _ex({"P": 500.0, "mu": 1.307e-3, "V": 400.0, "t": 1800.0})],
     level="intermediate", weak="the same definition evaluated with numpy square roots")


def mixing_power(G, mu, V):
    _pos("G, mu and V", G, mu, V)
    return _out(P=G * G * mu * V)


def _ref_mixing_power(G, mu, V):
    return {"P": _brentq(lambda p: math.sqrt(p / (mu * V)) - G, 0.0, G * G * mu * V * 4.0 + 1.0)}


_add("enveng_mixing_power",
     name="Power needed for a target velocity gradient", topic="water treatment / mixing and flocculation",
     purpose="The power that must be dissipated in a tank to give a target mean velocity gradient G.",
     inputs=[_phys("G", "velocity_gradient", "target velocity gradient"), _phys("mu", "dynamic_viscosity", "water viscosity"),
             _phys("V", "volume", "tank volume")],
     outputs=[_o("P", "power", "power required")],
     primary="P", conditions=["G, mu, V > 0"], method="P = G^2 mu V", latex=r"P = G^2\mu V",
     run=mixing_power, reference=_ref_mixing_power,
     wrong=lambda G, mu, V: _out(P=G * mu * V),
     rand=lambda rng: {"G": _sig(rng.uniform(20.0, 1000.0), 3), "mu": _sig(rng.uniform(0.9e-3, 1.4e-3), 3), "V": _sig(rng.uniform(5.0, 1000.0), 3)},
     phrases=("Mixing power", "Power for velocity gradient"),
     examples=[_ex({"G": 700.0, "mu": 1.002e-3, "V": 10.0}), _ex({"G": 40.0, "mu": 1.307e-3, "V": 500.0})],
     level="intermediate", weak="the defining equation G = sqrt(P/(mu V)) solved numerically for P (brentq)")


# ================================================================================================ TREATMENT: disinfection
def ct_value(C, V, Q, BF):
    _pos("C, V and Q", C, V, Q)
    _frac("the baffling factor BF", BF, lo_open=True)
    t10 = BF * V / Q
    return _out(t10=t10, CT=C * t10, CT_mg_min_L=C * 1e3 * t10 / 60.0)


def _ref_ct_value(C, V, Q, BF):
    from fractions import Fraction as Fr
    t10 = Fr(BF) * Fr(V) / Fr(Q)
    return {"t10": float(t10), "CT": float(Fr(C) * t10), "CT_mg_min_L": float(Fr(C) * 1000 * t10 / 60)}


_add("enveng_disinfection_ct",
     name="Disinfection CT value of a contact basin", topic="water treatment / disinfection",
     purpose="The effective contact time t10 of a basin (baffling factor times the theoretical detention time) and the CT value of a disinfectant residual.",
     inputs=[_phys("C", "mass_concentration", "disinfectant residual at the outlet"), _phys("V", "volume", "basin volume"),
             _phys("Q", "flow_rate", "peak flow", show=("m^3/d", DAY)), _numin("BF", "baffling factor t10/T (0.1 unbaffled to 1 plug flow)")],
     outputs=[_o("t10", "time", "effective contact time"), _o("CT", "concentration_time_integral", "CT value"),
              _o("CT_mg_min_L", "number", "CT in mg min/L")],
     primary="CT", conditions=["0 < BF <= 1"], method="t10 = BF V/Q; CT = C t10", latex=r"CT = C\,t_{10} = C\,BF\,\frac{V}{Q}",
     run=ct_value, reference=_ref_ct_value,
     wrong=lambda C, V, Q, BF: _out(t10=V / Q, CT=C * V / Q, CT_mg_min_L=C * 1e3 * V / Q / 60.0),
     rand=lambda rng: {"C": _sig(rng.uniform(0.2, 3.0), 3) * MG, "V": _sig(rng.uniform(100.0, 5000.0), 3), "Q": _sig(rng.uniform(2000.0, 100000.0), 3) / DAY,
                       "BF": rng.choice([0.1, 0.3, 0.5, 0.7])},
     phrases=("Disinfection CT", "CT value of a contact basin"),
     examples=[_ex({"C": 1.0e-3, "V": 1000.0, "Q": 20000.0 / DAY, "BF": 0.5}), _ex({"C": 0.8e-3, "V": 2500.0, "Q": 40000.0 / DAY, "BF": 0.7})],
     level="beginner", weak="exact rational arithmetic of the same definition")


def ct_inactivation(C, t10, CT999):
    _pos("C, t10 and CT999", C, t10, CT999)
    CT = C * t10
    ratio = CT / CT999
    return _out(CT=CT, ratio=ratio, log_inactivation=3.0 * ratio)


def _ref_ct_inactivation(C, t10, CT999):
    from fractions import Fraction as Fr
    r = Fr(C) * Fr(t10) / Fr(CT999)
    return {"CT": C * t10, "ratio": float(r), "log_inactivation": float(3 * r)}


_add("enveng_ct_log_inactivation",
     name="Log inactivation from the CT ratio", topic="water treatment / disinfection",
     purpose="The CT achieved, the ratio to the CT required for 3-log (99.9 %) inactivation from the regulatory tables, and the log inactivation credited (3 times the ratio).",
     inputs=[_phys("C", "mass_concentration", "disinfectant residual"), _phys("t10", "time", "effective contact time", show=("min", 1.0 / 60.0)),
             _phys("CT999", "concentration_time_integral", "CT required for 3-log inactivation (from the tables)")],
     outputs=[_o("CT", "concentration_time_integral", "CT achieved"), _o("ratio", "number", "CT achieved / CT for 3-log"),
              _o("log_inactivation", "number", "log inactivation credited")],
     primary="log_inactivation", conditions=["first-order (Chick-Watson) inactivation with n = 1"],
     method="log inactivation = 3 CT/CT_99.9", latex=r"\text{log inactivation} = 3\,\frac{C t_{10}}{CT_{99.9}}",
     run=ct_inactivation, reference=_ref_ct_inactivation,
     wrong=lambda C, t10, CT999: _out(CT=C * t10, ratio=C * t10 / CT999, log_inactivation=C * t10 / CT999),
     rand=lambda rng: {"C": _sig(rng.uniform(0.2, 3.0), 3) * MG, "t10": _sig(rng.uniform(5.0, 120.0), 3) * 60.0,
                       "CT999": _sig(rng.uniform(30.0, 200.0), 3) * 0.06},
     phrases=("CT log inactivation", "Log inactivation from CT"),
     examples=[_ex({"C": 1.2e-3, "t10": 40.0 * 60.0, "CT999": 104.0 * 0.06}), _ex({"C": 0.5e-3, "t10": 90.0 * 60.0, "CT999": 70.0 * 0.06})],
     level="intermediate", weak="exact rational arithmetic of the same definition")


def chick_first_order(N0, k, t):
    _pos("N0 and k", N0, k)
    _nonneg("the contact time t", t)
    s = math.exp(-k * t)
    return _out(N=N0 * s, survival=s, log_removal=k * t / math.log(10.0), percent_removal=(1.0 - s) * 100.0)


def _ref_chick_first_order(N0, k, t):
    s = _ivp(lambda _t, y: [-k * y[0]], t, [1.0], atol=[1e-22])[0] if t > 0 else 1.0
    return {"N": N0 * s, "survival": s, "log_removal": -math.log10(s), "percent_removal": (1.0 - s) * 100.0}


_add("enveng_chick_law",
     name="Microbial inactivation by Chick's law (first order)", topic="water treatment / disinfection",
     purpose="The surviving organisms, survival fraction, log removal and percentage removal after a contact time under first-order (Chick's law) inactivation or die-off.",
     inputs=[_numin("N0", "initial organism concentration (count per volume, e.g. per 100 mL)"),
             _phys("k", "first_order_rate_constant", "inactivation (die-off) rate constant", show=("1/min", 60.0)),
             _phys("t", "time", "contact time", show=("min", 1.0 / 60.0))],
     outputs=[_o("N", "number", "surviving organisms, same unit as N0"), _o("survival", "number", "N/N0"),
              _o("log_removal", "number", "log10 removal"), _o("percent_removal", "number", "percentage removed")],
     primary="survival", conditions=["first-order kinetics"], method="N = N0 e^(-k t); log removal = k t/ln 10", latex=r"\frac{N}{N_0} = e^{-kt}",
     run=chick_first_order, reference=_ref_chick_first_order,
     wrong=lambda N0, k, t: (lambda s: _out(N=N0 * s, survival=s, log_removal=k * t, percent_removal=(1.0 - s) * 100.0))(10.0 ** (-k * t)),
     rand=lambda rng: {"N0": _sig(_lu(rng, 1e2, 1e7), 3), "k": _sig(rng.uniform(0.02, 0.5), 3) / 60.0, "t": _sig(rng.uniform(2.0, 60.0), 3) * 60.0},
     phrases=("Chick's law inactivation", "Chick law inactivation"),
     examples=[_ex({"N0": 1e6, "k": 0.2 / 60.0, "t": 30.0 * 60.0}), _ex({"N0": 5000.0, "k": 0.05 / 60.0, "t": 20.0 * 60.0})],
     level="intermediate")


def chick_watson(k, n, C, t):
    _pos("k, n and C", k, n, C)
    _nonneg("the contact time t", t)
    _need(0.3 <= n <= 3.0, "the coefficient of dilution n must lie between 0.3 and 3")
    lnS = -k * (C * 1e3) ** n * (t / 60.0)
    return _out(survival=math.exp(lnS), log_inactivation=-lnS / math.log(10.0))


def _ref_chick_watson(k, n, C, t):
    rate = k * (C * 1000.0) ** n / 60.0
    s = _ivp(lambda _t, y: [-rate * y[0]], t, [1.0], atol=[1e-25])[0] if t > 0 else 1.0
    return {"survival": s, "log_inactivation": -math.log10(s)}


_add("enveng_chick_watson",
     name="Disinfection by the Chick-Watson law", topic="water treatment / disinfection",
     purpose="The survival fraction and log inactivation of organisms exposed to a disinfectant concentration C for time t under ln(N/N0) = -k C^n t (k in (L/mg)^n per minute, C in mg/L, t in minutes inside the law).",
     inputs=[_numin("k", "Chick-Watson coefficient in (L/mg)^n per minute"), _numin("n", "coefficient of dilution (1 for most disinfectants)"),
             _phys("C", "mass_concentration", "disinfectant concentration"), _phys("t", "time", "contact time", show=("min", 1.0 / 60.0))],
     outputs=[_o("survival", "number", "N/N0"), _o("log_inactivation", "number", "log10 inactivation")],
     primary="log_inactivation", conditions=["constant concentration", "0.3 <= n <= 3"],
     method="ln(N/N0) = -k C^n t with C in mg/L and t in minutes", latex=r"\ln\frac{N}{N_0} = -k\,C^n t",
     run=chick_watson, reference=_ref_chick_watson,
     wrong=lambda k, n, C, t: (lambda L: _out(survival=10.0 ** (-L), log_inactivation=L))(k * (C * 1e3) ** n * (t / 60.0)),
     rand=lambda rng: {"k": _sig(rng.uniform(0.02, 0.5), 3), "n": rng.choice([0.8, 1.0, 1.0, 1.2]), "C": _sig(rng.uniform(0.2, 3.0), 3) * MG,
                       "t": _sig(rng.uniform(1.0, 30.0), 3) * 60.0},
     phrases=("Chick-Watson inactivation", "Chick-Watson disinfection"),
     examples=[_ex({"k": 0.1, "n": 1.0, "C": 1.0e-3, "t": 30.0 * 60.0}), _ex({"k": 0.3, "n": 0.8, "C": 2.0e-3, "t": 10.0 * 60.0})],
     level="advanced")


def log_removal(N0, N):
    _pos("N0 and N", N0, N)
    _need(N < N0, "the final count must be below the initial count")
    return _out(log_removal=math.log10(N0 / N), percent_removal=(1.0 - N / N0) * 100.0)


def _ref_log_removal(N0, N):
    import mpmath as mp
    with mp.workdps(30):
        lr = mp.log10(mp.mpf(N0)) - mp.log10(mp.mpf(N))
        pr = (1 - mp.mpf(N) / mp.mpf(N0)) * 100
    return {"log_removal": float(lr), "percent_removal": float(pr)}


_add("enveng_log_removal",
     name="Log removal and percentage removal", topic="water treatment / disinfection",
     purpose="The log10 removal (log reduction value) and the percentage removal from the counts or concentrations before and after treatment.",
     inputs=[_numin("N0", "count or concentration before treatment"), _numin("N", "count or concentration after treatment (same unit)")],
     outputs=[_o("log_removal", "number", "log10 removal"), _o("percent_removal", "number", "percentage removed")],
     primary="log_removal", conditions=["0 < N < N0"], method="LR = log10(N0/N); percent = (1 - N/N0) 100", latex=r"LR = \log_{10}\frac{N_0}{N}",
     run=log_removal, reference=_ref_log_removal,
     wrong=lambda N0, N: _out(log_removal=math.log(N0 / N), percent_removal=(1.0 - N / N0) * 100.0),
     rand=lambda rng: (lambda N0: {"N0": N0, "N": _sig(N0 * 10.0 ** (-rng.uniform(0.5, 6.0)), 3)})(_sig(_lu(rng, 1e3, 1e8), 3)),
     phrases=("Log removal", "Log reduction value"),
     examples=[_ex({"N0": 1e6, "N": 100.0}), _ex({"N0": 2.5e5, "N": 4000.0})],
     level="beginner", weak="the same definition in 30-digit mpmath")


def chlorine_dose(Q, demand, residual, purity):
    _pos("the flow Q", Q)
    _nonneg("the demand and the residual", demand, residual)
    _frac("the available chlorine fraction purity", purity, lo_open=True)
    dose = demand + residual
    _need(dose > 0, "demand plus residual must be positive")
    feed = Q * dose
    return _out(dose=dose, dose_mgL=dose * 1e3, feed=feed, feed_kg_per_day=feed * DAY, product_kg_per_day=feed * DAY / purity)


def _ref_chlorine_dose(Q, demand, residual, purity):
    from fractions import Fraction as Fr
    dose = Fr(demand) + Fr(residual)
    feed = Fr(Q) * dose
    return {"dose": float(dose), "dose_mgL": float(dose * 1000), "feed": float(feed), "feed_kg_per_day": float(feed * 86400),
            "product_kg_per_day": float(feed * 86400 / Fr(purity))}


_add("enveng_chlorine_dose_feed",
     name="Chlorine dose and feed rate", topic="water treatment / disinfection",
     purpose="The chlorine dose (demand plus residual), the chlorine feed rate for a flow and the feed of a product with a stated available-chlorine fraction (e.g. 65 % calcium hypochlorite).",
     inputs=[_phys("Q", "flow_rate", "flow treated", show=("m^3/d", DAY)), _phys("demand", "mass_concentration", "chlorine demand"),
             _phys("residual", "mass_concentration", "required free chlorine residual"),
             _numin("purity", "available chlorine fraction of the product (1 for chlorine gas; fraction or %)", pct=True)],
     outputs=[_o("dose", "mass_concentration", "chlorine dose"), _o("dose_mgL", "number", "dose in mg/L"), _o("feed", "mass_flow_rate", "chlorine feed rate"),
              _o("feed_kg_per_day", "number", "chlorine feed in kg/d"), _o("product_kg_per_day", "number", "product feed in kg/d")],
     primary="feed", conditions=["0 < purity <= 1"], method="dose = demand + residual; feed = Q dose; product = feed/purity",
     latex=r"\dot m = Q\,(\text{demand} + \text{residual}),\quad \dot m_{product} = \frac{\dot m}{p}",
     run=chlorine_dose, reference=_ref_chlorine_dose,
     wrong=lambda Q, demand, residual, purity: (lambda f: _out(dose=demand + residual, dose_mgL=(demand + residual) * 1e3, feed=f, feed_kg_per_day=f * DAY,
                                                              product_kg_per_day=f * DAY * purity))(Q * (demand + residual)),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 100000.0), 3) / DAY, "demand": _sig(rng.uniform(0.5, 5.0), 3) * MG,
                       "residual": _sig(rng.uniform(0.2, 2.0), 3) * MG, "purity": rng.choice([0.65, 0.7, 0.125, 0.15, 0.1])},
     phrases=("Chlorine dose and feed rate", "Chlorine feed rate"),
     examples=[_ex({"Q": 20000.0 / DAY, "demand": 2.5e-3, "residual": 0.5e-3, "purity": 1.0}),
               _ex({"Q": 5000.0 / DAY, "demand": 1.8e-3, "residual": 0.7e-3, "purity": 0.65})],
     level="beginner",
     extra_q=[{"q": "Chlorine feed rate: Q = 5000 m^3/d, demand = 1.8 mg/L, residual = 0.7 mg/L, purity = 65%", "like": 1}],
     weak="exact rational arithmetic of the same definition")


def hocl_fraction(pH, pKa=None, T=None):
    _fin(pH)
    _need(0.0 <= pH <= 14.0, "pH must lie between 0 and 14")
    _need((pKa is None) != (T is None), "give either pKa or the temperature T (to compute pKa), not both")
    if pKa is None:
        _need(T0C <= T <= T0C + 35.0, "the pKa correlation holds between 0 and 35 degC")
        pKa = 3000.0 / T - 10.0686 + 0.0253 * T
    a = 1.0 / (1.0 + 10.0 ** (pH - pKa))
    return _out(alpha_HOCl=a, alpha_OCl=1.0 - a, pKa=pKa)


def _ref_hocl_fraction(pH, pKa=None, T=None):
    if pKa is None:
        pKa = (3000.0 - 10.0686 * T + 0.0253 * T * T) / T
    H, Ka = 10.0 ** (-pH), 10.0 ** (-pKa)
    a = _brentq(lambda x: x * (H + Ka) - H, 0.0, 1.0)         # [HOCl] = CT [H]/([H] + Ka)
    return {"alpha_HOCl": a, "alpha_OCl": 1.0 - a, "pKa": pKa}


_add("enveng_hypochlorous_acid_fraction",
     name="Fraction of free chlorine as hypochlorous acid (pH)", topic="water treatment / disinfection",
     purpose="The fractions of free chlorine present as HOCl and as OCl- at a pH, from the acid dissociation constant (given, or from the temperature by Morris' correlation).",
     inputs=[_numin("pH", "pH of the water"), _numin("pKa", "pKa of HOCl (7.54 at 25 degC)", required=False),
             _phys("T", "temperature", "water temperature (to compute pKa instead)", required=False, show=("degC", None))],
     outputs=[_o("alpha_HOCl", "number", "fraction as HOCl"), _o("alpha_OCl", "number", "fraction as OCl-"), _o("pKa", "number", "pKa used")],
     primary="alpha_HOCl", conditions=["either pKa or T", "0-35 degC for the correlation"],
     method="alpha_HOCl = 1/(1 + 10^(pH - pKa)); pKa = 3000/T - 10.0686 + 0.0253 T (Morris)",
     latex=r"\alpha_{HOCl} = \frac{1}{1 + 10^{\,pH - pK_a}}",
     run=hocl_fraction, reference=_ref_hocl_fraction,
     wrong=lambda pH, pKa=None, T=None: (lambda r: dict(r, alpha_HOCl=_r(1.0 - r["alpha_HOCl"]), alpha_OCl=_r(r["alpha_HOCl"])))(hocl_fraction(pH, pKa, T)),
     rand=lambda rng: dict({"pH": _sig(rng.choice([rng.uniform(5.5, 7.2), rng.uniform(7.8, 9.5)]), 3)},
                           **({"pKa": 7.54} if rng.random() < 0.5 else {"T": _sig(T0C + rng.uniform(0.0, 35.0), 4)})),
     phrases=("Hypochlorous acid fraction", "Fraction of HOCl"),
     examples=[_ex({"pH": 7.0, "pKa": 7.54}), _ex({"pH": 8.0, "T": T0C + 20.0})],
     level="advanced", weak="the same equilibrium solved numerically (brentq) with the correlation rearranged")


def hardness(Ca, Mg):
    _nonneg("Ca and Mg", Ca, Mg)
    eq_caco3 = (AW["Ca"] + AW["C"] + 3 * AW["O"]) / 2.0
    ca_h = Ca * 1e3 * eq_caco3 / (AW["Ca"] / 2.0)
    mg_h = Mg * 1e3 * eq_caco3 / (AW["Mg"] / 2.0)
    th = ca_h + mg_h
    cls = "soft" if th < 60 else ("moderately hard" if th < 120 else ("hard" if th < 180 else "very hard"))
    return _out(total_hardness=th, calcium_hardness=ca_h, magnesium_hardness=mg_h, classification=cls)


def _ref_hardness(Ca, Mg):
    from chemicals import molecular_weight, simple_formula_parser
    w = lambda f: molecular_weight(simple_formula_parser(f))  # noqa: E731
    ca_h = Ca * 1e3 * w("CaCO3") / w("Ca")
    mg_h = Mg * 1e3 * w("CaCO3") / w("Mg")
    th = ca_h + mg_h
    cls = "soft" if th < 60 else ("moderately hard" if th < 120 else ("hard" if th < 180 else "very hard"))
    return {"total_hardness": th, "calcium_hardness": ca_h, "magnesium_hardness": mg_h, "classification": cls}


_add("enveng_water_hardness",
     name="Water hardness as CaCO3", topic="water treatment / water chemistry",
     purpose="Calcium, magnesium and total hardness expressed as mg/L of CaCO3 from the calcium and magnesium concentrations, with the usual classification (soft to very hard).",
     inputs=[_phys("Ca", "mass_concentration", "calcium concentration (as Ca)"), _phys("Mg", "mass_concentration", "magnesium concentration (as Mg)")],
     outputs=[_o("total_hardness", "number", "total hardness, mg/L as CaCO3"), _o("calcium_hardness", "number", "calcium hardness, mg/L as CaCO3"),
              _o("magnesium_hardness", "number", "magnesium hardness, mg/L as CaCO3"),
              _o("classification", "number", "soft (< 60), moderately hard (60-120), hard (120-180), very hard (>= 180)")],
     primary="total_hardness", conditions=["only Ca and Mg contribute"],
     method="hardness = C (EW_CaCO3/EW_ion), EW_CaCO3 = 50.04, EW_Ca = 20.04, EW_Mg = 12.15",
     latex=r"TH = Ca\,\frac{50.04}{20.04} + Mg\,\frac{50.04}{12.15}",
     run=hardness, reference=_ref_hardness,
     wrong=lambda Ca, Mg: (lambda e: _out(total_hardness=Ca * 1e3 * e / AW["Ca"] + Mg * 1e3 * e / AW["Mg"], calcium_hardness=Ca * 1e3 * e / AW["Ca"],
                                          magnesium_hardness=Mg * 1e3 * e / AW["Mg"], classification="soft"))((AW["Ca"] + AW["C"] + 3 * AW["O"]) / 2.0),
     rand=lambda rng: {"Ca": _sig(rng.uniform(5.0, 150.0), 3) * MG, "Mg": _sig(rng.uniform(1.0, 50.0), 3) * MG},
     phrases=("Water hardness", "Total hardness as CaCO3"),
     examples=[_ex({"Ca": 0.06, "Mg": 0.015}), _ex({"Ca": 0.012, "Mg": 0.004})],
     level="beginner")


# ================================================================================================ TREATMENT: adsorption and membranes
def freundlich_dose(C0, Ce, K, n):
    _pos("C0, Ce, K and n", C0, Ce, K, n)
    _need(Ce < C0, "the target concentration must be below the initial concentration")
    qe = K * (Ce * 1e3) ** (1.0 / n)
    dose = (C0 - Ce) * 1e3 / qe            # (mg/L)/(mg/g) = g/L = kg/m^3
    return _out(qe=qe, dose=dose, dose_mgL=dose * 1e3)


def _ref_freundlich_dose(C0, Ce, K, n):
    qe = float(10.0 ** (math.log10(K) + math.log10(Ce * 1000.0) / n))
    return {"qe": qe, "dose": (C0 - Ce) * 1000.0 / qe, "dose_mgL": (C0 - Ce) * 1e6 / qe}


_add("enveng_freundlich_carbon_dose",
     name="Activated carbon dose from the Freundlich isotherm", topic="water treatment / adsorption",
     purpose="The equilibrium adsorbent loading from the Freundlich isotherm q = K Ce^(1/n) (q in mg/g, Ce in mg/L) and the batch carbon dose that lowers C0 to Ce.",
     inputs=[_phys("C0", "mass_concentration", "initial concentration"), _phys("Ce", "mass_concentration", "target equilibrium concentration"),
             _numin("K", "Freundlich capacity factor, (mg/g)(L/mg)^(1/n)"), _numin("n", "Freundlich exponent n (isotherm exponent 1/n)")],
     outputs=[_o("qe", "number", "equilibrium loading, mg/g"), _o("dose", "mass_concentration", "adsorbent dose"), _o("dose_mgL", "number", "dose in mg/L")],
     primary="dose", conditions=["Ce < C0", "batch equilibrium"], method="qe = K Ce^(1/n); dose = (C0 - Ce)/qe",
     latex=r"q_e = K C_e^{1/n},\quad \frac{m}{V} = \frac{C_0 - C_e}{q_e}",
     run=freundlich_dose, reference=_ref_freundlich_dose,
     wrong=lambda C0, Ce, K, n: (lambda q: _out(qe=q, dose=(C0 - Ce) * 1e3 / q, dose_mgL=(C0 - Ce) * 1e6 / q))(K * (Ce * 1e3) ** n),
     rand=lambda rng: (lambda C0: {"C0": C0, "Ce": _sig(C0 * rng.uniform(0.02, 0.5), 3), "K": _sig(rng.uniform(5.0, 200.0), 3),
                                   "n": _sig(rng.uniform(1.5, 4.0), 3)})(_sig(rng.uniform(0.5, 20.0), 3) * MG),
     phrases=("Freundlich carbon dose", "Carbon dose from Freundlich isotherm"),
     examples=[_ex({"C0": 0.01, "Ce": 0.001, "K": 50.0, "n": 2.0}), _ex({"C0": 0.005, "Ce": 0.0005, "K": 120.0, "n": 3.0})],
     level="intermediate", weak="the same isotherm evaluated through logarithms")


def langmuir_dose(C0, Ce, qmax, b):
    _pos("C0, Ce, qmax and b", C0, Ce, qmax, b)
    _need(Ce < C0, "the target concentration must be below the initial concentration")
    c = Ce * 1e3
    qe = qmax * b * c / (1.0 + b * c)
    dose = (C0 - Ce) * 1e3 / qe
    return _out(qe=qe, dose=dose, dose_mgL=dose * 1e3)


def _ref_langmuir_dose(C0, Ce, qmax, b):
    c = Ce * 1000.0
    qe = qmax / (1.0 + 1.0 / (b * c))
    return {"qe": qe, "dose": (C0 - Ce) * 1000.0 / qe, "dose_mgL": (C0 - Ce) * 1e6 / qe}


_add("enveng_langmuir_carbon_dose",
     name="Adsorbent dose from the Langmuir isotherm", topic="water treatment / adsorption",
     purpose="The equilibrium loading from the Langmuir isotherm q = qmax b Ce/(1 + b Ce) (q in mg/g, Ce in mg/L) and the batch adsorbent dose that lowers C0 to Ce.",
     inputs=[_phys("C0", "mass_concentration", "initial concentration"), _phys("Ce", "mass_concentration", "target equilibrium concentration"),
             _numin("qmax", "Langmuir monolayer capacity, mg/g"), _numin("b", "Langmuir constant, L/mg")],
     outputs=[_o("qe", "number", "equilibrium loading, mg/g"), _o("dose", "mass_concentration", "adsorbent dose"), _o("dose_mgL", "number", "dose in mg/L")],
     primary="dose", conditions=["Ce < C0"], method="qe = qmax b Ce/(1 + b Ce); dose = (C0 - Ce)/qe",
     latex=r"q_e = \frac{q_{max}bC_e}{1 + bC_e},\quad \frac{m}{V} = \frac{C_0 - C_e}{q_e}",
     run=langmuir_dose, reference=_ref_langmuir_dose,
     wrong=lambda C0, Ce, qmax, b: (lambda q: _out(qe=q, dose=(C0 - Ce) * 1e3 / q, dose_mgL=(C0 - Ce) * 1e6 / q))(qmax * b * Ce * 1e3),
     rand=lambda rng: (lambda C0: {"C0": C0, "Ce": _sig(C0 * rng.uniform(0.05, 0.5), 3), "qmax": _sig(rng.uniform(20.0, 400.0), 3),
                                   "b": _sig(rng.uniform(0.2, 5.0), 3)})(_sig(rng.uniform(0.5, 20.0), 3) * MG),
     phrases=("Langmuir adsorbent dose", "Adsorbent dose from Langmuir isotherm"),
     examples=[_ex({"C0": 0.01, "Ce": 0.001, "qmax": 150.0, "b": 0.8}), _ex({"C0": 0.02, "Ce": 0.004, "qmax": 300.0, "b": 0.3})],
     level="intermediate", weak="the same isotherm evaluated in a rearranged form")


def _linfit(x, y):
    n = len(x)
    mx = sum(x) / n
    my = sum(y) / n
    sxx = sum((a - mx) ** 2 for a in x)
    syy = sum((b - my) ** 2 for b in y)
    sxy = sum((a - mx) * (b - my) for a, b in zip(x, y))
    _need(sxx > 0 and syy > 0, "the data must vary")
    slope = sxy / sxx
    return slope, my - slope * mx, sxy * sxy / (sxx * syy)


def _iso_check(Ce, qe):
    _need(len(Ce) == len(qe) and len(Ce) >= 3, "give at least three points (Ce and qe of equal length)")
    for v in list(Ce) + list(qe):
        _need(math.isfinite(v) and v > 0, "every concentration and loading must be positive")


def freundlich_fit(Ce, qe):
    _iso_check(Ce, qe)
    s, i, r2 = _linfit([math.log10(v) for v in Ce], [math.log10(v) for v in qe])
    _need(s > 0, "the loading must rise with concentration")
    return _out(K=10.0 ** i, inv_n=s, n=1.0 / s, r2=r2)


def _ref_freundlich_fit(Ce, qe):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.log10(Ce), np.log10(qe))
    return {"K": float(10.0 ** r.intercept), "inv_n": float(r.slope), "n": float(1.0 / r.slope), "r2": float(r.rvalue ** 2)}


def _rand_iso(rng, kind):
    n = rng.randint(4, 8)
    Ce = sorted(_sig(_lu(rng, 0.1, 50.0), 3) for _ in range(n))
    if len(set(Ce)) < n:
        Ce = [float(i + 1) for i in range(n)]
    if kind == "f":
        K, inv = rng.uniform(5.0, 150.0), rng.uniform(0.25, 0.7)
        q = [_sig(K * c ** inv * (1.0 + rng.uniform(-0.05, 0.05)), 4) for c in Ce]
    else:
        qm, b = rng.uniform(30.0, 300.0), rng.uniform(0.05, 1.0)
        q = [_sig(qm * b * c / (1.0 + b * c) * (1.0 + rng.uniform(-0.03, 0.03)), 4) for c in Ce]
    return {"Ce": Ce, "qe": q}


_add("enveng_freundlich_fit",
     name="Freundlich isotherm constants fitted to adsorption data", topic="water treatment / adsorption",
     purpose="The Freundlich constants K and 1/n fitted by linear regression of log q against log Ce, with the coefficient of determination.",
     inputs=[_vecin("Ce", "equilibrium concentrations in mg/L", 3, 50), _vecin("qe", "equilibrium loadings in mg/g (same order)", 3, 50)],
     outputs=[_o("K", "number", "Freundlich K, (mg/g)(L/mg)^(1/n)"), _o("inv_n", "number", "isotherm exponent 1/n"), _o("n", "number", "Freundlich n"),
              _o("r2", "number", "coefficient of determination of the log-log fit")],
     primary="K", conditions=["at least three positive points"], method="least squares of log10 q = log10 K + (1/n) log10 Ce",
     latex=r"\log q_e = \log K + \frac1n\log C_e",
     run=freundlich_fit, reference=_ref_freundlich_fit,
     wrong=lambda Ce, qe: (lambda r: dict(r, K=_r(math.exp(math.log10(r["K"])))))(freundlich_fit(Ce, qe)),
     rand=lambda rng: _rand_iso(rng, "f"),
     phrases=("Freundlich isotherm fit", "Fit Freundlich isotherm"),
     examples=[_ex({"Ce": [0.5, 1.0, 2.0, 5.0, 10.0], "qe": [28.0, 40.0, 55.0, 86.0, 120.0]}), _ex({"Ce": [1.2, 3.5, 8.0, 20.0], "qe": [12.0, 19.5, 27.0, 41.0]})],
     level="advanced")


def langmuir_fit(Ce, qe):
    _iso_check(Ce, qe)
    s, i, r2 = _linfit(list(Ce), [c / q for c, q in zip(Ce, qe)])
    _need(s > 0 and i > 0, "the data do not follow a Langmuir isotherm (the linearised slope and intercept must be positive)")
    return _out(qmax=1.0 / s, b=s / i, r2=r2)


def _ref_langmuir_fit(Ce, qe):
    import numpy as np
    from scipy.stats import linregress
    Ce, qe = np.array(Ce, float), np.array(qe, float)
    r = linregress(Ce, Ce / qe)
    return {"qmax": float(1.0 / r.slope), "b": float(r.slope / r.intercept), "r2": float(r.rvalue ** 2)}


_add("enveng_langmuir_fit",
     name="Langmuir isotherm constants fitted to adsorption data", topic="water treatment / adsorption",
     purpose="The Langmuir capacity qmax and constant b fitted by linear regression of Ce/q against Ce (linearised Langmuir form), with the coefficient of determination.",
     inputs=[_vecin("Ce", "equilibrium concentrations in mg/L", 3, 50), _vecin("qe", "equilibrium loadings in mg/g (same order)", 3, 50)],
     outputs=[_o("qmax", "number", "monolayer capacity, mg/g"), _o("b", "number", "Langmuir constant, L/mg"),
              _o("r2", "number", "coefficient of determination of the linearised fit")],
     primary="qmax", conditions=["positive slope and intercept of Ce/q against Ce"], method="least squares of Ce/q = 1/(qmax b) + Ce/qmax",
     latex=r"\frac{C_e}{q_e} = \frac{1}{q_{max}b} + \frac{C_e}{q_{max}}",
     run=langmuir_fit, reference=_ref_langmuir_fit,
     wrong=lambda Ce, qe: (lambda r: dict(r, b=_r(1.0 / r["b"])))(langmuir_fit(Ce, qe)),
     rand=lambda rng: _rand_iso(rng, "l"),
     phrases=("Langmuir isotherm fit", "Fit Langmuir isotherm"),
     examples=[_ex({"Ce": [0.5, 1.0, 2.0, 5.0, 10.0], "qe": [40.0, 66.0, 97.0, 133.0, 152.0]}), _ex({"Ce": [2.0, 6.0, 15.0, 40.0], "qe": [20.0, 42.0, 66.0, 85.0]})],
     level="advanced")


def membrane_balance(Qf, Qp, Cf, Cp, A):
    _pos("Qf, Qp, Cf and A", Qf, Qp, Cf, A)
    _nonneg("the permeate concentration Cp", Cp)
    _need(Qp < Qf, "the permeate flow must be below the feed flow")
    _need(Cp < Cf, "the permeate must be cleaner than the feed")
    Qc = Qf - Qp
    Cc = (Qf * Cf - Qp * Cp) / Qc
    J = Qp / A
    return _out(recovery=Qp / Qf, rejection=1.0 - Cp / Cf, J=J, J_LMH=J * 3.6e6, Qc=Qc, Cc=Cc, Cc_mgL=Cc * 1e3)


def _ref_membrane_balance(Qf, Qp, Cf, Cp, A):
    import numpy as np
    Qc, M = np.linalg.solve(np.array([[1.0, 0.0], [0.0, 1.0]]), np.array([Qf - Qp, Qf * Cf - Qp * Cp]))
    Cc = float(M / Qc)
    return {"recovery": Qp / Qf, "rejection": (Cf - Cp) / Cf, "J": Qp / A, "J_LMH": Qp / A * 3.6e6, "Qc": float(Qc), "Cc": Cc, "Cc_mgL": Cc * 1e3}


_add("enveng_membrane_mass_balance",
     name="Membrane recovery, rejection, flux and concentrate", topic="water treatment / membranes",
     purpose="The water recovery, solute rejection, permeate flux (also in L/(m^2 h)) and the concentrate flow and concentration of a membrane unit from the feed and permeate flows and concentrations and the membrane area.",
     inputs=[_phys("Qf", "flow_rate", "feed flow", show=("m^3/d", DAY)), _phys("Qp", "flow_rate", "permeate flow", show=("m^3/d", DAY)),
             _phys("Cf", "mass_concentration", "feed concentration"), _phys("Cp", "mass_concentration", "permeate concentration"),
             _phys("A", "area", "membrane area")],
     outputs=[_o("recovery", "number", "Qp/Qf"), _o("rejection", "number", "1 - Cp/Cf"), _o("J", "speed", "permeate flux"),
              _o("J_LMH", "number", "flux in L/(m^2 h)"), _o("Qc", "flow_rate", "concentrate flow"),
              _o("Cc", "mass_concentration", "concentrate concentration"), _o("Cc_mgL", "number", "concentrate concentration in mg/L")],
     primary="recovery", conditions=["Qp < Qf", "Cp < Cf", "steady state"],
     method="r = Qp/Qf; R = 1 - Cp/Cf; J = Qp/A; Qc = Qf - Qp; Cc = (Qf Cf - Qp Cp)/Qc",
     latex=r"r = \frac{Q_p}{Q_f},\ R = 1 - \frac{C_p}{C_f},\ C_c = \frac{Q_fC_f - Q_pC_p}{Q_f - Q_p}",
     run=membrane_balance, reference=_ref_membrane_balance,
     wrong=lambda Qf, Qp, Cf, Cp, A: (lambda r: dict(r, rejection=_r(Cp / Cf), Cc=_r(Cf), Cc_mgL=_r(Cf * 1e3)))(membrane_balance(Qf, Qp, Cf, Cp, A)),
     rand=lambda rng: (lambda Qf, Cf: {"Qf": Qf, "Qp": _sig(Qf * rng.uniform(0.5, 0.9), 3), "Cf": Cf, "Cp": _sig(Cf * rng.uniform(0.005, 0.1), 3),
                                       "A": _sig(rng.uniform(100.0, 5000.0), 3)})(_sig(rng.uniform(500.0, 20000.0), 3) / DAY, _sig(rng.uniform(500.0, 35000.0), 3) * MG),
     phrases=("Membrane mass balance", "Membrane recovery and rejection"),
     examples=[_ex({"Qf": 1000.0 / DAY, "Qp": 750.0 / DAY, "Cf": 2.0, "Cp": 0.05, "A": 1500.0}),
               _ex({"Qf": 5000.0 / DAY, "Qp": 2250.0 / DAY, "Cf": 35.0, "Cp": 0.2, "A": 6000.0})],
     level="intermediate")


def osmotic_pressure(C, M, i, T):
    _pos("C, M, i and T", C, M, i, T)
    pi = i * C / M * R_GAS * T
    return _out(pi=pi, pi_bar=pi / 1e5)


def _ref_osmotic_pressure(C, M, i, T):
    from chemicals import ideal_gas
    # van 't Hoff as an ideal-gas analogy: pi = (i C/M) / (molar volume at P = 1 Pa) ... i.e. pi = n R T / V
    pi = i * C / M * ideal_gas(T, 1.0)
    return {"pi": pi, "pi_bar": pi / 1e5}


_add("enveng_osmotic_pressure",
     name="Osmotic pressure of a dilute solution (van 't Hoff)", topic="water treatment / membranes",
     purpose="The osmotic pressure of a dilute salt solution from its mass concentration, molar mass, van 't Hoff factor and temperature.",
     inputs=[_phys("C", "mass_concentration", "solute mass concentration", show=("g/L", 1.0)), _phys("M", "molar_mass", "solute molar mass", show=("g/mol", 1e3)),
             _numin("i", "van 't Hoff factor (ions per formula unit, 2 for NaCl)"), _phys("T", "temperature", "temperature")],
     outputs=[_o("pi", "pressure", "osmotic pressure"), _o("pi_bar", "number", "osmotic pressure in bar")],
     primary="pi", conditions=["dilute (ideal) solution"], method="pi = i (C/M) R T", latex=r"\pi = i\,\frac{C}{M}\,RT",
     run=osmotic_pressure, reference=_ref_osmotic_pressure,
     wrong=lambda C, M, i, T: (lambda p: _out(pi=p, pi_bar=p / 1e5))(C / M * R_GAS * T),
     rand=lambda rng: {"C": _sig(rng.uniform(0.5, 40.0), 3), "M": rng.choice([0.05844, 0.11098, 0.14204, 0.09521]), "i": rng.choice([2.0, 3.0]),
                       "T": _sig(T0C + rng.uniform(5.0, 35.0), 4)},
     phrases=("Osmotic pressure", "Van 't Hoff osmotic pressure"),
     examples=[_ex({"C": 35.0, "M": 0.05844, "i": 2.0, "T": 298.15}), _ex({"C": 2.0, "M": 0.11098, "i": 3.0, "T": 293.15})],
     level="intermediate", weak="the same ideal law through the chemicals library's ideal-gas molar volume")


# ================================================================================================ WASTEWATER: activated sludge
def fm_ratio(Q, S0, V, X):
    _pos("Q, S0, V and X", Q, S0, V, X)
    fm = Q * S0 / (V * X)
    return _out(FM=fm, FM_per_day=fm * DAY)


def _ref_fm_ratio(Q, S0, V, X):
    from fractions import Fraction as Fr
    fm = Fr(Q) * Fr(S0) / (Fr(V) * Fr(X))
    return {"FM": float(fm), "FM_per_day": float(fm * 86400)}


_add("enveng_food_to_microorganism_ratio",
     name="Food-to-microorganism ratio (F/M)", topic="wastewater / activated sludge",
     purpose="The food-to-microorganism ratio of an aeration tank from the flow, influent BOD, tank volume and mixed-liquor volatile suspended solids.",
     inputs=[_phys("Q", "flow_rate", "influent flow", show=("m^3/d", DAY)), _phys("S0", "mass_concentration", "influent BOD (or COD)"),
             _phys("V", "volume", "aeration tank volume"), _phys("X", "mass_concentration", "mixed liquor volatile suspended solids (MLVSS)")],
     outputs=[_o("FM", "food_to_microorganism_ratio", "food-to-microorganism (F/M) ratio"), _o("FM_per_day", "number", "in kg BOD/(kg MLVSS d)")],
     primary="FM", conditions=["Q, S0, V, X > 0"], method="F/M = Q S0/(V X)", latex=r"\frac FM = \frac{QS_0}{VX}",
     run=fm_ratio, reference=_ref_fm_ratio,
     wrong=lambda Q, S0, V, X: _out(FM=Q * S0 / V, FM_per_day=Q * S0 / V * DAY),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 50000.0), 3) / DAY, "S0": _sig(rng.uniform(100.0, 400.0), 3) * MG,
                       "V": _sig(rng.uniform(500.0, 20000.0), 3), "X": _sig(rng.uniform(1500.0, 4000.0), 3) * MG},
     phrases=("Food-to-microorganism ratio", "F/M ratio"),
     examples=[_ex({"Q": 10000.0 / DAY, "S0": 0.2, "V": 3000.0, "X": 2.5}), _ex({"Q": 4000.0 / DAY, "S0": 0.15, "V": 1500.0, "X": 2.0})],
     level="beginner", weak="exact rational arithmetic of the same definition")


def srt(V, X, Qw, Xw, Qe=None, Xe=None):
    _pos("V, X, Qw and Xw", V, X, Qw, Xw)
    xe = 0.0 if Xe is None else Xe
    _nonneg("Xe", xe)
    if Qe is None:
        _need(xe == 0, "give the effluent flow Qe with the effluent solids Xe")
        qe = 0.0
    else:
        qe = Qe
        _nonneg("Qe", qe)
    th = V * X / (Qw * Xw + qe * xe)
    return _out(SRT=th, SRT_days=th / DAY)


def _ref_srt(V, X, Qw, Xw, Qe=None, Xe=None):
    from fractions import Fraction as Fr
    lost = Fr(0) if Xe is None or Qe is None else Fr(Qe) * Fr(Xe)   # effluent solids neglected unless given
    th = Fr(V) * Fr(X) / (Fr(Qw) * Fr(Xw) + lost)
    return {"SRT": float(th), "SRT_days": float(th / 86400)}


def _rand_srt(rng):
    x = {"V": _sig(rng.uniform(500.0, 20000.0), 3), "X": _sig(rng.uniform(1500.0, 4000.0), 3) * MG,
         "Qw": _sig(rng.uniform(20.0, 400.0), 3) / DAY, "Xw": _sig(rng.uniform(6000.0, 12000.0), 3) * MG}
    if rng.random() < 0.75:
        x["Qe"] = _sig(rng.uniform(1000.0, 50000.0), 3) / DAY
        x["Xe"] = _sig(rng.uniform(5.0, 30.0), 3) * MG
    elif rng.random() < 0.3:
        x["Qe"] = _sig(rng.uniform(1000.0, 50000.0), 3) / DAY
    return x


_add("enveng_solids_retention_time",
     name="Solids retention time (mean cell residence time)", topic="wastewater / activated sludge",
     purpose="The solids retention time (sludge age, mean cell residence time) of an activated-sludge system from the biomass in the tank and the biomass leaving in the waste sludge and the effluent; the effluent solids are neglected (Xe = 0) when not given.",
     inputs=[_phys("V", "volume", "aeration tank volume"), _phys("X", "mass_concentration", "MLVSS in the tank"),
             _phys("Qw", "flow_rate", "waste sludge flow", show=("m^3/d", DAY)), _phys("Xw", "mass_concentration", "VSS in the waste sludge"),
             _phys("Qe", "flow_rate", "effluent flow (needed only with Xe)", required=False, show=("m^3/d", DAY)),
             _phys("Xe", "mass_concentration", "VSS in the effluent (default 0: effluent solids neglected)", required=False)],
     outputs=[_o("SRT", "time", "solids retention time"), _o("SRT_days", "number", "SRT in days")],
     primary="SRT", conditions=["steady state", "Xe = 0 (effluent solids neglected) unless given; Xe > 0 needs Qe"],
     method="SRT = V X/(Qw Xw + Qe Xe)", latex=r"\theta_c = \frac{VX}{Q_wX_w + Q_eX_e}",
     run=srt, reference=_ref_srt,
     wrong=lambda V, X, Qw, Xw, Qe=None, Xe=None: _out(SRT=V * X / (Qw * Xw), SRT_days=V * X / (Qw * Xw) / DAY),
     rand=_rand_srt,
     phrases=("Solids retention time", "Mean cell residence time", "Sludge age"),
     examples=[_ex({"V": 3000.0, "X": 2.5, "Qw": 100.0 / DAY, "Xw": 10.0, "Qe": 9900.0 / DAY, "Xe": 0.015}),
               _ex({"V": 1200.0, "X": 3.0, "Qw": 60.0 / DAY, "Xw": 8.0, "Qe": 3940.0 / DAY, "Xe": 0.01}),
               _ex({"V": 3000.0, "X": 2.5, "Qw": 100.0 / DAY, "Xw": 10.0})],
     level="intermediate",
     # batch 3 (2026-11-11): the effluent solids optional
     extra_q=[{"q": "Sludge age: V = 3000 m^3, X = 2500 mg/L, Qw = 100 m^3/d, Xw = 10000 mg/L", "like": 2},
              {"q": "Mean cell residence time: V = 3000 m^3, X = 2500 mg/L, Qw = 100 m^3/d, Xw = 10000 mg/L, Xe = 0 mg/L", "like": 2},
              {"q": "Solids retention time: V = 3000 m^3, X = 2500 mg/L, Qw = 100 m^3/d, Xw = 10000 mg/L, Xe = 15 mg/L", "outcome": "INVALID_INPUT"}],
     weak="exact rational arithmetic of the same definition")


def hrt_olr(Q, S0, V):
    _pos("Q, S0 and V", Q, S0, V)
    return _out(HRT=V / Q, HRT_h=V / Q / HOUR, OLR=Q * S0 / V, OLR_kg_per_m3_day=Q * S0 / V * DAY)


def _ref_hrt_olr(Q, S0, V):
    from fractions import Fraction as Fr
    h = Fr(V) / Fr(Q)
    return {"HRT": float(h), "HRT_h": float(h / 3600), "OLR": float(Fr(S0) / h), "OLR_kg_per_m3_day": float(Fr(S0) / h * 86400)}


_add("enveng_hrt_organic_loading",
     name="Hydraulic retention time and volumetric organic loading", topic="wastewater / activated sludge",
     purpose="The hydraulic retention time of a reactor and its volumetric organic (BOD) loading rate from the flow, influent BOD and volume.",
     inputs=[_phys("Q", "flow_rate", "influent flow", show=("m^3/d", DAY)), _phys("S0", "mass_concentration", "influent BOD"),
             _phys("V", "volume", "reactor volume")],
     outputs=[_o("HRT", "time", "hydraulic retention time"), _o("HRT_h", "number", "HRT in hours"),
              _o("OLR", "volumetric_loading", "volumetric organic loading rate"), _o("OLR_kg_per_m3_day", "number", "loading in kg BOD/(m^3 d)")],
     primary="OLR", conditions=["Q, S0, V > 0"], method="HRT = V/Q; OLR = Q S0/V", latex=r"\theta = \frac VQ,\quad L_v = \frac{QS_0}{V}",
     run=hrt_olr, reference=_ref_hrt_olr,
     wrong=lambda Q, S0, V: _out(HRT=V / Q, HRT_h=V / Q / HOUR, OLR=S0 * V / Q, OLR_kg_per_m3_day=S0 * V / Q * DAY),
     rand=lambda rng: {"Q": _sig(rng.uniform(1000.0, 50000.0), 3) / DAY, "S0": _sig(rng.uniform(100.0, 400.0), 3) * MG, "V": _sig(rng.uniform(500.0, 20000.0), 3)},
     phrases=("Hydraulic retention time and organic loading", "Volumetric organic loading"),
     examples=[_ex({"Q": 10000.0 / DAY, "S0": 0.2, "V": 3000.0}), _ex({"Q": 2500.0 / DAY, "S0": 0.35, "V": 800.0})],
     level="intermediate", weak="exact rational arithmetic of the same definition")


def monod(mu_max, Ks, S, kd=None):
    _pos("mu_max, Ks and S", mu_max, Ks, S)
    mu = mu_max * S / (Ks + S)
    out = {"mu": mu, "mu_per_day": mu * DAY}
    if kd is not None:
        _nonneg("the decay coefficient kd", kd)
        out["mu_net"] = mu - kd
        out["mu_net_per_day"] = (mu - kd) * DAY
    return _out(**out)


def _ref_monod(mu_max, Ks, S, kd=None):
    mu = mu_max / (1.0 + Ks / S)
    out = {"mu": mu, "mu_per_day": mu * 86400.0}
    if kd is not None:
        out["mu_net"] = mu - kd
        out["mu_net_per_day"] = (mu - kd) * 86400.0
    return out


_add("enveng_monod_growth_rate",
     name="Monod specific growth rate", topic="wastewater / microbial kinetics",
     purpose="The specific growth rate of microorganisms at a substrate concentration by the Monod equation, and the net rate when the endogenous decay coefficient is given.",
     inputs=[_phys("mu_max", "first_order_rate_constant", "maximum specific growth rate"), _phys("Ks", "mass_concentration", "half-saturation constant"),
             _phys("S", "mass_concentration", "substrate concentration"), _phys("kd", "first_order_rate_constant", "endogenous decay coefficient", required=False)],
     outputs=[_o("mu", "first_order_rate_constant", "specific growth rate"), _o("mu_per_day", "number", "in 1/d"),
              _o("mu_net", "first_order_rate_constant", "net specific growth rate mu - kd"), _o("mu_net_per_day", "number", "net rate in 1/d")],
     primary="mu", conditions=["Monod kinetics"], method="mu = mu_max S/(Ks + S); mu_net = mu - kd", latex=r"\mu = \frac{\mu_{max}S}{K_s + S}",
     run=monod, reference=_ref_monod,
     wrong=lambda mu_max, Ks, S, kd=None: (lambda m: _out(**({"mu": m, "mu_per_day": m * DAY} if kd is None else
                                                             {"mu": m, "mu_per_day": m * DAY, "mu_net": m - kd, "mu_net_per_day": (m - kd) * DAY})))(mu_max * Ks / (Ks + S)),
     rand=lambda rng: dict({"mu_max": _sig(rng.uniform(1.0, 6.0), 3) / DAY, "Ks": _sig(rng.uniform(20.0, 120.0), 3) * MG,
                            "S": _sig(rng.choice([rng.uniform(2.0, 15.0), rng.uniform(150.0, 500.0)]), 3) * MG},
                           **({"kd": _sig(rng.uniform(0.04, 0.15), 3) / DAY} if rng.random() < 0.5 else {})),
     phrases=("Monod growth rate", "Monod specific growth rate"),
     examples=[_ex({"mu_max": 3.0 / DAY, "Ks": 0.06, "S": 0.01}), _ex({"mu_max": 5.0 / DAY, "Ks": 0.04, "S": 0.1, "kd": 0.1 / DAY})],
     level="intermediate", weak="the same equation evaluated in a rearranged form")


def _lm_S(Y, k, Ks, kd, SRT):
    den = SRT * (Y * k - kd) - 1.0
    _need(den > 0, "the SRT is below the washout limit (SRT (Y k - kd) must exceed 1)")
    return Ks * (1.0 + kd * SRT) / den


def lawrence_mccarty(Y, k, Ks, kd, SRT, HRT, S0):
    _pos("Y, k, Ks, SRT, HRT and S0", Y, k, Ks, SRT, HRT, S0)
    _nonneg("kd", kd)
    _need(Y < 1.0, "the yield Y must be below 1")
    _need(SRT >= HRT, "the SRT cannot be shorter than the HRT in a system with sludge recycle or wasting")
    S = _lm_S(Y, k, Ks, kd, SRT)
    _need(S < S0, "the effluent substrate would exceed the influent: washout")
    X = SRT * Y * (S0 - S) / (HRT * (1.0 + kd * SRT))
    return _out(S=S, S_mgL=S * 1e3, X=X, X_mgL=X * 1e3, Y_obs=Y / (1.0 + kd * SRT))


def _ref_lawrence_mccarty(Y, k, Ks, kd, SRT, HRT, S0):
    S = _brentq(lambda s: Y * k * s / (Ks + s) - kd - 1.0 / SRT, 1e-30, S0 * 10.0 + Ks * 1e6)     # biomass balance
    X = (S0 - S) * (Ks + S) / (HRT * k * S)                                                    # substrate balance
    return {"S": S, "S_mgL": S * 1e3, "X": X, "X_mgL": X * 1e3, "Y_obs": Y / (1.0 + kd * SRT)}


def _rand_lm(rng):
    return {"Y": _sig(rng.uniform(0.4, 0.7), 3), "k": _sig(rng.uniform(3.0, 10.0), 3) / DAY, "Ks": _sig(rng.uniform(20.0, 100.0), 3) * MG,
            "kd": _sig(rng.uniform(0.03, 0.12), 3) / DAY, "SRT": _sig(rng.uniform(3.0, 20.0), 3) * DAY,
            "HRT": _sig(rng.uniform(3.0, 12.0), 3) * HOUR, "S0": _sig(rng.uniform(150.0, 400.0), 3) * MG}


_KIN_IN = [_numin("Y", "true yield coefficient, kg VSS per kg BOD"), _phys("k", "first_order_rate_constant", "maximum specific substrate utilisation rate"),
           _phys("Ks", "mass_concentration", "half-saturation constant"), _phys("kd", "first_order_rate_constant", "endogenous decay coefficient")]

_add("enveng_activated_sludge_cstr_design",
     name="Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)", topic="wastewater / activated sludge",
     purpose="The effluent soluble substrate, the mixed-liquor biomass and the observed yield of a complete-mix activated-sludge reactor with recycle, from the kinetic coefficients, the SRT and HRT and the influent substrate.",
     inputs=_KIN_IN + [_phys("SRT", "time", "solids retention time", show=("d", 1.0 / DAY)), _phys("HRT", "time", "hydraulic retention time", show=("h", 1.0 / HOUR)),
                       _phys("S0", "mass_concentration", "influent substrate (BOD)")],
     outputs=[_o("S", "mass_concentration", "effluent soluble substrate"), _o("S_mgL", "number", "in mg/L"),
              _o("X", "mass_concentration", "biomass concentration in the reactor"), _o("X_mgL", "number", "in mg/L"),
              _o("Y_obs", "number", "observed yield Y/(1 + kd SRT)")],
     primary="S", conditions=["steady state, complete mix", "SRT above the washout limit", "S < S0"],
     method="S = Ks (1 + kd SRT)/(SRT (Y k - kd) - 1); X = (SRT/HRT) Y (S0 - S)/(1 + kd SRT)",
     latex=r"S = \frac{K_s(1 + k_d\theta_c)}{\theta_c(Yk - k_d) - 1},\quad X = \frac{\theta_c Y(S_0 - S)}{\theta(1 + k_d\theta_c)}",
     run=lawrence_mccarty, reference=_ref_lawrence_mccarty,
     wrong=lambda Y, k, Ks, kd, SRT, HRT, S0: (lambda S: _out(S=S, S_mgL=S * 1e3, X=SRT * Y * (S0 - S) / HRT, X_mgL=SRT * Y * (S0 - S) / HRT * 1e3, Y_obs=Y))(_lm_S(Y, k, Ks, kd, SRT)),
     rand=_rand_lm,
     phrases=("Activated sludge complete-mix design", "Lawrence-McCarty design"),
     examples=[_ex({"Y": 0.5, "k": 5.0 / DAY, "Ks": 0.06, "kd": 0.06 / DAY, "SRT": 10 * DAY, "HRT": 6 * HOUR, "S0": 0.25}),
               _ex({"Y": 0.6, "k": 8.0 / DAY, "Ks": 0.04, "kd": 0.08 / DAY, "SRT": 6 * DAY, "HRT": 4 * HOUR, "S0": 0.18})],
     level="expert")


def min_srt(Y, k, Ks, kd, S0):
    _pos("Y, k, Ks and S0", Y, k, Ks, S0)
    _nonneg("kd", kd)
    inv = Y * k * S0 / (Ks + S0) - kd
    _need(inv > 0, "the organisms cannot grow on this substrate (Y k S0/(Ks + S0) <= kd)")
    return _out(SRT_min=1.0 / inv, SRT_min_days=1.0 / inv / DAY)


def _ref_min_srt(Y, k, Ks, kd, S0):
    lo = 1.0 / (Y * k - kd) * (1.0 + 1e-12)
    th = _brentq(lambda t: Ks * (1.0 + kd * t) / (t * (Y * k - kd) - 1.0) - S0, lo, lo * 1e6)   # effluent S equals S0
    return {"SRT_min": th, "SRT_min_days": th / 86400.0}


_add("enveng_minimum_srt",
     name="Minimum (washout) solids retention time", topic="wastewater / activated sludge",
     purpose="The washout SRT of a complete-mix biological reactor: below it the organisms are lost faster than they grow on the influent substrate.",
     inputs=_KIN_IN + [_phys("S0", "mass_concentration", "influent substrate")],
     outputs=[_o("SRT_min", "time", "minimum SRT"), _o("SRT_min_days", "number", "minimum SRT in days")],
     primary="SRT_min", conditions=["Y k S0/(Ks + S0) > kd"], method="1/SRT_min = Y k S0/(Ks + S0) - kd",
     latex=r"\frac{1}{\theta_{c,min}} = \frac{YkS_0}{K_s + S_0} - k_d",
     run=min_srt, reference=_ref_min_srt,
     wrong=lambda Y, k, Ks, kd, S0: (lambda v: _out(SRT_min=1.0 / v, SRT_min_days=1.0 / v / DAY))(Y * k - kd),
     rand=lambda rng: {k: v for k, v in _rand_lm(rng).items() if k in ("Y", "k", "Ks", "kd", "S0")},
     phrases=("Minimum solids retention time", "Washout SRT"),
     examples=[_ex({"Y": 0.5, "k": 5.0 / DAY, "Ks": 0.06, "kd": 0.06 / DAY, "S0": 0.25}), _ex({"Y": 0.6, "k": 8.0 / DAY, "Ks": 0.04, "kd": 0.08 / DAY, "S0": 0.18})],
     level="expert")


def sludge_production(Y, kd, SRT, Q, S0, S):
    _pos("Y, SRT, Q and S0", Y, SRT, Q, S0)
    _nonneg("kd and S", kd, S)
    _need(S < S0, "the effluent substrate must be below the influent")
    yo = Y / (1.0 + kd * SRT)
    Px = yo * Q * (S0 - S)
    return _out(Y_obs=yo, Px=Px, Px_kg_per_day=Px * DAY)


def _ref_sludge_production(Y, kd, SRT, Q, S0, S):
    from fractions import Fraction as Fr
    yo = Fr(Y) / (1 + Fr(kd) * Fr(SRT))
    Px = yo * Fr(Q) * (Fr(S0) - Fr(S))
    return {"Y_obs": float(yo), "Px": float(Px), "Px_kg_per_day": float(Px * 86400)}


_add("enveng_sludge_production",
     name="Waste biomass production (observed yield)", topic="wastewater / activated sludge",
     purpose="The observed yield and the daily mass of biological solids produced (to be wasted) from the kinetic yield, decay coefficient, SRT, flow and substrate removed.",
     inputs=[_numin("Y", "true yield, kg VSS per kg BOD"), _phys("kd", "first_order_rate_constant", "endogenous decay coefficient"),
             _phys("SRT", "time", "solids retention time", show=("d", 1.0 / DAY)), _phys("Q", "flow_rate", "flow", show=("m^3/d", DAY)),
             _phys("S0", "mass_concentration", "influent substrate"), _phys("S", "mass_concentration", "effluent substrate")],
     outputs=[_o("Y_obs", "number", "observed yield"), _o("Px", "mass_flow_rate", "biomass production"), _o("Px_kg_per_day", "number", "in kg VSS/d")],
     primary="Px", conditions=["S < S0"], method="Y_obs = Y/(1 + kd SRT); Px = Y_obs Q (S0 - S)", latex=r"P_x = \frac{Y}{1 + k_d\theta_c}Q(S_0 - S)",
     run=sludge_production, reference=_ref_sludge_production,
     wrong=lambda Y, kd, SRT, Q, S0, S: _out(Y_obs=Y, Px=Y * Q * (S0 - S), Px_kg_per_day=Y * Q * (S0 - S) * DAY),
     rand=lambda rng: {"Y": _sig(rng.uniform(0.4, 0.7), 3), "kd": _sig(rng.uniform(0.03, 0.12), 3) / DAY, "SRT": _sig(rng.uniform(3.0, 20.0), 3) * DAY,
                       "Q": _sig(rng.uniform(1000.0, 50000.0), 3) / DAY, "S0": _sig(rng.uniform(150.0, 400.0), 3) * MG, "S": _sig(rng.uniform(2.0, 20.0), 3) * MG},
     phrases=("Sludge production", "Waste biomass production"),
     examples=[_ex({"Y": 0.5, "kd": 0.06 / DAY, "SRT": 10 * DAY, "Q": 10000.0 / DAY, "S0": 0.25, "S": 0.005}),
               _ex({"Y": 0.6, "kd": 0.05 / DAY, "SRT": 8 * DAY, "Q": 4000.0 / DAY, "S0": 0.2, "S": 0.01})],
     level="intermediate", weak="exact rational arithmetic of the same definition")


def oxygen_demand(Q, S0, S, f, Px):
    _pos("Q and S0", Q, S0)
    _nonneg("S and Px", S, Px)
    _frac("the BOD5/BODu ratio f", f, lo_open=True)
    _need(S < S0, "the effluent substrate must be below the influent")
    O2 = Q * (S0 - S) / f - 1.42 * Px
    _need(O2 > 0, "the biomass wasted exceeds the oxygen equivalent of the BOD removed")
    return _out(O2=O2, O2_kg_per_day=O2 * DAY)


def _ref_oxygen_demand(Q, S0, S, f, Px):
    from fractions import Fraction as Fr
    O2 = Fr(Q) * (Fr(S0) - Fr(S)) / Fr(f) - Fr(142, 100) * Fr(Px)
    return {"O2": float(O2), "O2_kg_per_day": float(O2 * 86400)}


_add("enveng_aeration_oxygen_requirement",
     name="Oxygen requirement of an activated-sludge process", topic="wastewater / activated sludge",
     purpose="The carbonaceous oxygen requirement: ultimate BOD removed (BOD5 removed divided by the BOD5/BODu ratio) less 1.42 times the biomass wasted.",
     inputs=[_phys("Q", "flow_rate", "flow", show=("m^3/d", DAY)), _phys("S0", "mass_concentration", "influent BOD5"),
             _phys("S", "mass_concentration", "effluent soluble BOD5"), _numin("f", "BOD5/BODu ratio (e.g. 0.68)"),
             _phys("Px", "mass_flow_rate", "biomass wasted (VSS)", show=("kg/d", DAY))],
     outputs=[_o("O2", "mass_flow_rate", "oxygen required"), _o("O2_kg_per_day", "number", "in kg O2/d")],
     primary="O2", conditions=["carbonaceous demand only (no nitrification)"], method="O2 = Q (S0 - S)/f - 1.42 Px",
     latex=r"\dot m_{O_2} = \frac{Q(S_0 - S)}{f} - 1.42P_x",
     run=oxygen_demand, reference=_ref_oxygen_demand,
     wrong=lambda Q, S0, S, f, Px: _out(O2=Q * (S0 - S) - 1.42 * Px, O2_kg_per_day=(Q * (S0 - S) - 1.42 * Px) * DAY),
     rand=lambda rng: (lambda Q, S0, S: {"Q": Q, "S0": S0, "S": S, "f": rng.choice([0.68, 0.7, 0.65]),
                                         "Px": _sig(Q * (S0 - S) * rng.uniform(0.2, 0.4), 3)})(
         _sig(rng.uniform(1000.0, 50000.0), 3) / DAY, _sig(rng.uniform(150.0, 400.0), 3) * MG, _sig(rng.uniform(2.0, 20.0), 3) * MG),
     phrases=("Aeration oxygen requirement", "Oxygen requirement of activated sludge"),
     examples=[_ex({"Q": 10000.0 / DAY, "S0": 0.25, "S": 0.005, "f": 0.68, "Px": 800.0 / DAY}),
               _ex({"Q": 4000.0 / DAY, "S0": 0.2, "S": 0.01, "f": 0.7, "Px": 250.0 / DAY})],
     level="advanced", weak="exact rational arithmetic of the same definition")


def return_ratio(X, Xr, Q):
    _pos("X, Xr and Q", X, Xr, Q)
    _need(Xr > X, "the return sludge must be more concentrated than the mixed liquor")
    R = X / (Xr - X)
    return _out(R=R, Qr=R * Q)


def _ref_return_ratio(X, Xr, Q):
    import numpy as np
    # balance around the clarifier inlet: (Q + Qr) X = Qr Xr
    Qr = float(np.linalg.solve(np.array([[Xr - X]]), np.array([Q * X]))[0])
    return {"R": Qr / Q, "Qr": Qr}


_add("enveng_return_sludge_ratio",
     name="Return activated sludge ratio", topic="wastewater / activated sludge",
     purpose="The return sludge ratio and flow needed to hold the mixed-liquor concentration, from a solids balance around the aeration tank (influent solids neglected).",
     inputs=[_phys("X", "mass_concentration", "mixed liquor suspended solids"), _phys("Xr", "mass_concentration", "return sludge concentration"),
             _phys("Q", "flow_rate", "influent flow", show=("m^3/d", DAY))],
     outputs=[_o("R", "number", "return ratio Qr/Q"), _o("Qr", "flow_rate", "return sludge flow")],
     primary="R", conditions=["Xr > X", "influent solids negligible"], method="(Q + Qr) X = Qr Xr, so R = X/(Xr - X)", latex=r"R = \frac{Q_r}{Q} = \frac{X}{X_r - X}",
     run=return_ratio, reference=_ref_return_ratio,
     wrong=lambda X, Xr, Q: _out(R=X / Xr, Qr=X / Xr * Q),
     rand=lambda rng: {"X": _sig(rng.uniform(1500.0, 4000.0), 3) * MG, "Xr": _sig(rng.uniform(6000.0, 12000.0), 3) * MG, "Q": _sig(rng.uniform(1000.0, 50000.0), 3) / DAY},
     phrases=("Return sludge ratio", "Return activated sludge ratio"),
     examples=[_ex({"X": 3.0, "Xr": 10.0, "Q": 10000.0 / DAY}), _ex({"X": 2.5, "Xr": 8.0, "Q": 5000.0 / DAY})],
     level="intermediate")


def svi(V30, MLSS):
    _pos("V30 and MLSS", V30, MLSS)
    _need(V30 <= 1000.0, "the settled volume cannot exceed 1000 mL/L")
    s = V30 / MLSS                         # (mL/L)/(g/L) = mL/g
    return _out(SVI=s, Xr_estimate=1.0 / s * 1e3, Xr_estimate_mgL=1e6 / s)


def _ref_svi(V30, MLSS):
    from fractions import Fraction as Fr
    s = Fr(V30) * 1000 / (Fr(MLSS) * 1000)
    return {"SVI": float(s), "Xr_estimate": float(1000 / s), "Xr_estimate_mgL": float(10 ** 6 / s)}


_add("enveng_sludge_volume_index",
     name="Sludge volume index (SVI)", topic="wastewater / activated sludge",
     purpose="The sludge volume index from the 30-minute settled volume and the mixed-liquor suspended solids, with the customary estimate of the return sludge concentration (10^6/SVI mg/L).",
     inputs=[_numin("V30", "settled sludge volume after 30 minutes, mL/L", words=("mL/L", "ml/L")),
             _phys("MLSS", "mass_concentration", "mixed liquor suspended solids")],
     outputs=[_o("SVI", "number", "sludge volume index, mL/g"), _o("Xr_estimate", "mass_concentration", "estimated maximum return sludge concentration"),
              _o("Xr_estimate_mgL", "number", "estimate in mg/L")],
     primary="SVI", conditions=["V30 <= 1000 mL/L"], method="SVI = V30 (mL/L)/MLSS (g/L)", latex=r"SVI = \frac{V_{30}}{MLSS}",
     run=svi, reference=_ref_svi,
     wrong=lambda V30, MLSS: (lambda s: _out(SVI=s, Xr_estimate=1.0 / s * 1e3, Xr_estimate_mgL=1e6 / s))(V30 / (MLSS * 1e3)),
     rand=lambda rng: {"V30": _sig(rng.uniform(150.0, 900.0), 3), "MLSS": _sig(rng.uniform(1500.0, 5000.0), 3) * MG},
     phrases=("Sludge volume index", "SVI"),
     examples=[_ex({"V30": 300.0, "MLSS": 2.5}), _ex({"V30": 450.0, "MLSS": 3.2})],
     level="beginner", weak="exact rational arithmetic of the same definition")


def sludge_volume(Ms, Ps, Ssl):
    _pos("Ms and Ssl", Ms, Ssl)
    _frac("the solids fraction Ps", Ps, lo_open=True)
    V = Ms / (1000.0 * Ssl * Ps)
    return _out(V=V, V_m3_per_day=V * DAY)


def _ref_sludge_volume(Ms, Ps, Ssl):
    from fractions import Fraction as Fr
    wet_mass = Fr(Ms) / Fr(Ps)
    V = wet_mass / (Fr(Ssl) * 1000)
    return {"V": float(V), "V_m3_per_day": float(V * 86400)}


_add("enveng_sludge_volume",
     name="Volume of wet sludge from dry solids", topic="wastewater / sludge handling",
     purpose="The volume (flow) of wet sludge from the dry-solids mass rate, the solids fraction and the specific gravity of the sludge (water 1000 kg/m^3).",
     inputs=[_phys("Ms", "mass_flow_rate", "dry solids mass rate", show=("kg/d", DAY)), _numin("Ps", "solids fraction of the wet sludge (fraction or %)", pct=True),
             _numin("Ssl", "specific gravity of the wet sludge")],
     outputs=[_o("V", "flow_rate", "wet sludge volume rate"), _o("V_m3_per_day", "number", "in m^3/d")],
     primary="V", conditions=["0 < Ps <= 1"], method="V = Ms/(rho_w Ssl Ps)", latex=r"V = \frac{M_s}{\rho_w S_{sl} P_s}",
     run=sludge_volume, reference=_ref_sludge_volume,
     wrong=lambda Ms, Ps, Ssl: _out(V=Ms * Ps / (1000.0 * Ssl), V_m3_per_day=Ms * Ps / (1000.0 * Ssl) * DAY),
     rand=lambda rng: {"Ms": _sig(rng.uniform(100.0, 10000.0), 3) / DAY, "Ps": _sig(rng.uniform(0.01, 0.3), 3), "Ssl": _sig(rng.uniform(1.0, 1.06), 4)},
     phrases=("Wet sludge volume", "Sludge volume from dry solids"),
     examples=[_ex({"Ms": 1000.0 / DAY, "Ps": 0.04, "Ssl": 1.02}), _ex({"Ms": 3500.0 / DAY, "Ps": 0.22, "Ssl": 1.05})],
     level="intermediate", weak="exact rational arithmetic of the same definition")


def thickening(V1, P1, P2):
    _pos("V1", V1)
    _frac("P1", P1, lo_open=True)
    _frac("P2", P2, lo_open=True)
    _need(P2 > P1, "thickening must raise the solids fraction (P2 > P1)")
    return _out(V2=V1 * P1 / P2, reduction=1.0 - P1 / P2)


def _ref_thickening(V1, P1, P2):
    from fractions import Fraction as Fr
    V2 = Fr(V1) * Fr(P1) / Fr(P2)
    return {"V2": float(V2), "reduction": float(1 - V2 / Fr(V1))}


_add("enveng_sludge_thickening",
     name="Sludge volume after thickening or dewatering", topic="wastewater / sludge handling",
     purpose="The sludge volume after thickening or dewatering from one solids fraction to another (same specific gravity), and the fractional volume reduction.",
     inputs=[_phys("V1", "flow_rate", "sludge volume rate before", show=("m^3/d", DAY)), _numin("P1", "solids fraction before (fraction or %)", pct=True),
             _numin("P2", "solids fraction after (fraction or %)", pct=True)],
     outputs=[_o("V2", "flow_rate", "sludge volume rate after"), _o("reduction", "number", "fractional volume reduction")],
     primary="V2", conditions=["P2 > P1", "same specific gravity"], method="V2 = V1 P1/P2", latex=r"V_2 = V_1\frac{P_1}{P_2}",
     run=thickening, reference=_ref_thickening,
     wrong=lambda V1, P1, P2: _out(V2=V1 * P2 / P1, reduction=1.0 - P2 / P1),
     rand=lambda rng: (lambda P1: {"V1": _sig(rng.uniform(10.0, 1000.0), 3) / DAY, "P1": P1, "P2": _sig(P1 * rng.uniform(1.5, 8.0), 3)})(_sig(rng.uniform(0.005, 0.04), 3)),
     phrases=("Sludge thickening", "Sludge volume after thickening"),
     examples=[_ex({"V1": 100.0 / DAY, "P1": 0.01, "P2": 0.05}), _ex({"V1": 40.0 / DAY, "P1": 0.04, "P2": 0.2})],
     level="intermediate",
     extra_q=[{"q": "Sludge thickening: V1 = 100 m^3/d, P1 = 1%, P2 = 5%", "like": 0}], weak="exact rational arithmetic of the same definition")


def methane_yield(COD, Px, T, P):
    _pos("COD, T and P", COD, T, P)
    _nonneg("Px", Px)
    net = COD - 1.42 * Px
    _need(net > 0, "the biomass produced exceeds the COD removed")
    n = net / (2.0 * MW_O2 * 1e-3)                  # mol CH4/s: 1 mol CH4 takes 2 mol O2
    Q = n * R_GAS * T / P
    Qstp = n * R_GAS * T0C / P_ATM
    return _out(Q_CH4=Q, Q_CH4_m3_per_day=Q * DAY, Q_CH4_stp_m3_per_day=Qstp * DAY)


def _ref_methane_yield(COD, Px, T, P):
    from chemicals import ideal_gas, molecular_weight
    n = (COD - 1.42 * Px) / (2.0 * molecular_weight({"O": 2}) * 1e-3)
    return {"Q_CH4": n * ideal_gas(T, P), "Q_CH4_m3_per_day": n * ideal_gas(T, P) * 86400.0,
            "Q_CH4_stp_m3_per_day": n * ideal_gas(273.15, 101325.0) * 86400.0}


_add("enveng_anaerobic_methane_yield",
     name="Methane production from COD removed (anaerobic digestion)", topic="wastewater / anaerobic treatment",
     purpose="The methane gas flow from an anaerobic process from the COD removed less the COD converted to biomass (1.42 kg COD per kg VSS), at a stated temperature and pressure (64 g COD per mole of methane).",
     inputs=[_phys("COD", "mass_flow_rate", "COD removed", show=("kg/d", DAY)), _phys("Px", "mass_flow_rate", "biomass (VSS) produced", show=("kg/d", DAY)),
             _phys("T", "temperature", "gas temperature"), _phys("P", "pressure", "gas pressure", show=("kPa", 1e-3))],
     outputs=[_o("Q_CH4", "flow_rate", "methane flow at T and P"), _o("Q_CH4_m3_per_day", "number", "in m^3/d at T and P"),
              _o("Q_CH4_stp_m3_per_day", "number", "in m^3/d at 0 degC and 1 atm")],
     primary="Q_CH4", conditions=["ideal gas", "COD balance"], method="n = (COD - 1.42 Px)/(64 g/mol); Q = n R T/P",
     latex=r"Q_{CH_4} = \frac{\mathrm{COD} - 1.42P_x}{64\ \mathrm{g/mol}}\,\frac{RT}{P}",
     run=methane_yield, reference=_ref_methane_yield,
     wrong=lambda COD, Px, T, P: (lambda q: _out(Q_CH4=q, Q_CH4_m3_per_day=q * DAY, Q_CH4_stp_m3_per_day=q * DAY))(0.35 * (COD - 1.42 * Px)),
     rand=lambda rng: (lambda COD: {"COD": COD, "Px": _sig(COD * rng.uniform(0.02, 0.1), 3), "T": _sig(T0C + rng.uniform(25.0, 40.0), 4),
                                    "P": _sig(rng.uniform(95000.0, 105000.0), 4)})(_sig(rng.uniform(100.0, 20000.0), 3) / DAY),
     phrases=("Methane production from COD", "Anaerobic methane yield"),
     examples=[_ex({"COD": 5000.0 / DAY, "Px": 300.0 / DAY, "T": 308.15, "P": 101325.0}), _ex({"COD": 1200.0 / DAY, "Px": 60.0 / DAY, "T": 298.15, "P": 100000.0})],
     level="advanced")


def nrc_trickling(W, V, R):
    _pos("W and V", W, V)
    _nonneg("the recirculation ratio R", R)
    F = (1.0 + R) / (1.0 + R / 10.0) ** 2
    E = 1.0 / (1.0 + 0.4432 * math.sqrt(W * DAY / (V * F)))
    return _out(F=F, E=E, E_percent=E * 100.0)


def _ref_nrc_trickling(W, V, R):
    F = (1.0 + R) * 100.0 / (10.0 + R) ** 2
    load = W * 86400.0 / V                        # kg BOD/(m^3 d)
    E = 100.0 / (100.0 + 44.32 * (load / F) ** 0.5)
    return {"F": F, "E": E, "E_percent": E * 100.0}


_add("enveng_nrc_trickling_filter",
     name="Trickling filter BOD removal efficiency (NRC equation)", topic="wastewater / attached growth",
     purpose="The BOD removal efficiency of a single-stage stone-media trickling filter (with its settling tank) by the NRC equation, from the BOD load, the filter volume and the recirculation ratio.",
     inputs=[_phys("W", "mass_flow_rate", "BOD load to the filter (excluding recycle)", show=("kg/d", DAY)), _phys("V", "volume", "filter media volume"),
             _numin("R", "recirculation ratio Qr/Q")],
     outputs=[_o("F", "number", "recirculation factor (1 + R)/(1 + R/10)^2"), _o("E", "number", "BOD removal efficiency (fraction)"),
              _o("E_percent", "number", "efficiency in per cent")],
     primary="E", conditions=["first-stage stone filter at 20 degC", "W in kg/d and V in m^3 inside the formula constant"],
     method="F = (1 + R)/(1 + R/10)^2; E = 1/(1 + 0.4432 sqrt(W/(V F)))", latex=r"E = \frac{100}{1 + 0.4432\sqrt{W/(VF)}},\quad F = \frac{1+R}{(1 + R/10)^2}",
     run=nrc_trickling, reference=_ref_nrc_trickling,
     wrong=lambda W, V, R: (lambda F: (lambda E: _out(F=F, E=E, E_percent=E * 100.0))(1.0 / (1.0 + 0.4432 * math.sqrt(W * DAY / (V * F)))))(1.0 + R),
     rand=lambda rng: {"W": _sig(rng.uniform(100.0, 3000.0), 3) / DAY, "V": _sig(rng.uniform(200.0, 5000.0), 3), "R": _sig(rng.uniform(0.5, 4.0), 3)},
     phrases=("NRC trickling filter efficiency", "Trickling filter efficiency"),
     examples=[_ex({"W": 800.0 / DAY, "V": 1000.0, "R": 2.0}), _ex({"W": 1500.0 / DAY, "V": 1200.0, "R": 1.0})],
     level="advanced", weak="the same published equation evaluated in a rearranged form")


def field_oxygen_transfer(SOTR, alpha, beta, theta, CsT, Cs20, CL, T, Fp=None):
    _pos("SOTR, alpha, beta, theta, CsT and Cs20", SOTR, alpha, beta, theta, CsT, Cs20)
    _nonneg("CL", CL)
    F = 1.0 if Fp is None else Fp
    _pos("the pressure correction Fp", F)
    drive = beta * CsT * F - CL
    _need(drive > 0, "the oxygen level is at or above the field saturation: no transfer")
    ratio = alpha * theta ** (T - (T0C + 20.0)) * drive / Cs20
    return _out(AOTR=SOTR * ratio, AOTR_kg_per_h=SOTR * ratio * HOUR, ratio=ratio)


def _ref_field_oxygen_transfer(SOTR, alpha, beta, theta, CsT, Cs20, CL, T, Fp=None):
    import mpmath as mp
    F = 1.0 if Fp is None else Fp
    with mp.workdps(30):
        ratio = mp.mpf(alpha) * mp.exp((mp.mpf(T) - mp.mpf("293.15")) * mp.log(theta)) * (mp.mpf(beta) * CsT * F - CL) / Cs20
    return {"AOTR": float(SOTR * ratio), "AOTR_kg_per_h": float(SOTR * ratio * 3600), "ratio": float(ratio)}


_add("enveng_field_oxygen_transfer",
     name="Field (actual) oxygen transfer rate of an aeration system", topic="wastewater / aeration",
     purpose="The actual oxygen transfer rate in the process from the standard (clean water, 20 degC, zero DO) rate, corrected by alpha, beta, the temperature coefficient, the site saturation and the operating DO.",
     inputs=[_phys("SOTR", "mass_flow_rate", "standard oxygen transfer rate", show=("kg/h", HOUR)), _numin("alpha", "alpha factor (process/clean water KLa)"),
             _numin("beta", "beta factor (process/clean water saturation)"), _numin("theta", "temperature coefficient (1.024)"),
             _phys("CsT", "mass_concentration", "clean-water saturation DO at the process temperature"),
             _phys("Cs20", "mass_concentration", "clean-water saturation DO at 20 degC"), _phys("CL", "mass_concentration", "operating DO"),
             _phys("T", "temperature", "process temperature", show=("degC", None)), _numin("Fp", "pressure (elevation) correction of saturation", required=False)],
     outputs=[_o("AOTR", "mass_flow_rate", "actual oxygen transfer rate"), _o("AOTR_kg_per_h", "number", "in kg O2/h"), _o("ratio", "number", "AOTR/SOTR")],
     primary="AOTR", conditions=["beta CsT Fp > CL"], method="AOTR = SOTR alpha theta^(T - 20) (beta CsT Fp - CL)/Cs20",
     latex=r"AOTR = SOTR\,\alpha\,\theta^{T-20}\,\frac{\beta C_{s,T}F - C_L}{C_{s,20}}",
     run=field_oxygen_transfer, reference=_ref_field_oxygen_transfer,
     wrong=lambda SOTR, alpha, beta, theta, CsT, Cs20, CL, T, Fp=None: (lambda r: _out(AOTR=SOTR * r, AOTR_kg_per_h=SOTR * r * HOUR, ratio=r))(
         alpha * theta ** ((T0C + 20.0) - T) * (beta * CsT * (1.0 if Fp is None else Fp) - CL) / Cs20),
     rand=lambda rng: dict({"SOTR": _sig(rng.uniform(10.0, 500.0), 3) / HOUR, "alpha": _sig(rng.uniform(0.4, 0.9), 2), "beta": _sig(rng.uniform(0.9, 0.99), 2),
                            "theta": 1.024, "CsT": _sig(rng.uniform(7.5, 10.5), 3) * MG, "Cs20": 9.09 * MG, "CL": _sig(rng.uniform(0.5, 3.0), 3) * MG,
                            "T": _sig(T0C + rng.choice([rng.uniform(8.0, 17.0), rng.uniform(23.0, 32.0)]), 4)},
                           **({"Fp": _sig(rng.uniform(0.85, 1.0), 3)} if rng.random() < 0.5 else {})),
     phrases=("Field oxygen transfer rate", "Actual oxygen transfer rate"),
     examples=[_ex({"SOTR": 100.0 / HOUR, "alpha": 0.6, "beta": 0.95, "theta": 1.024, "CsT": 8.26e-3, "Cs20": 9.09e-3, "CL": 2.0e-3, "T": T0C + 25.0}),
               _ex({"SOTR": 250.0 / HOUR, "alpha": 0.5, "beta": 0.98, "theta": 1.024, "CsT": 10.08e-3, "Cs20": 9.09e-3, "CL": 1.5e-3, "T": T0C + 12.0, "Fp": 0.9})],
     level="advanced", weak="the same published equation evaluated in 30-digit mpmath")


def kla_two_point(Cs, C1, C2, t1, t2):
    _pos("Cs", Cs)
    _nonneg("C1, t1 and t2", C1, t1, t2)
    _need(C1 < C2 < Cs, "the concentrations must rise toward saturation: C1 < C2 < Cs")
    _need(t2 > t1, "t2 must be later than t1")
    K = math.log((Cs - C1) / (Cs - C2)) / (t2 - t1)
    return _out(KLa=K, KLa_per_h=K * HOUR)


def _ref_kla_two_point(Cs, C1, C2, t1, t2):
    f = lambda K: Cs - (Cs - C1) * math.exp(-K * (t2 - t1)) - C2  # noqa: E731
    hi = 1.0 / (t2 - t1)
    while f(hi) < 0:
        hi *= 2.0
    K = _brentq(f, 0.0, hi)
    return {"KLa": K, "KLa_per_h": K * 3600.0}


_add("enveng_kla_two_point",
     name="Overall oxygen transfer coefficient KLa from two DO readings", topic="wastewater / aeration",
     purpose="The volumetric mass-transfer coefficient KLa from two dissolved-oxygen readings during reaeration toward a known saturation concentration.",
     inputs=[_phys("Cs", "mass_concentration", "saturation DO"), _phys("C1", "mass_concentration", "DO at the first time"),
             _phys("C2", "mass_concentration", "DO at the second time"), _phys("t1", "time", "first time", show=("min", 1.0 / 60.0)),
             _phys("t2", "time", "second time", show=("min", 1.0 / 60.0))],
     outputs=[_o("KLa", "first_order_rate_constant", "KLa"), _o("KLa_per_h", "number", "KLa in 1/h")],
     primary="KLa", conditions=["C1 < C2 < Cs", "t2 > t1"], method="KLa = ln((Cs - C1)/(Cs - C2))/(t2 - t1)", latex=r"K_La = \frac{\ln\frac{C_s - C_1}{C_s - C_2}}{t_2 - t_1}",
     run=kla_two_point, reference=_ref_kla_two_point,
     wrong=lambda Cs, C1, C2, t1, t2: (lambda K: _out(KLa=K, KLa_per_h=K * HOUR))(math.log10((Cs - C1) / (Cs - C2)) / (t2 - t1)),
     rand=lambda rng: (lambda Cs, C1: {"Cs": Cs, "C1": C1, "C2": _sig(C1 + (Cs - C1) * rng.uniform(0.2, 0.8), 3), "t1": _sig(rng.uniform(0.0, 5.0), 3) * 60.0,
                                       "t2": _sig(rng.uniform(8.0, 30.0), 3) * 60.0})(_sig(rng.uniform(8.0, 10.0), 3) * MG, _sig(rng.uniform(0.2, 3.0), 3) * MG),
     phrases=("KLa from two DO readings", "Oxygen transfer coefficient KLa"),
     examples=[_ex({"Cs": 9.1e-3, "C1": 2.0e-3, "C2": 6.0e-3, "t1": 120.0, "t2": 600.0}), _ex({"Cs": 8.5e-3, "C1": 1.0e-3, "C2": 5.5e-3, "t1": 0.0, "t2": 900.0})],
     level="intermediate")


def kla_fit(t, C, Cs):
    _need(len(t) == len(C) and len(t) >= 3, "give at least three times and as many DO readings")
    _pos("Cs", Cs)
    cs = Cs * 1e3
    for v in C:
        _need(math.isfinite(v) and 0 <= v < cs, "every DO reading must be below the saturation concentration")
    _need(len(set(t)) == len(t), "the times must be distinct")
    s, i, r2 = _linfit(list(t), [math.log(cs - v) for v in C])
    _need(s < 0, "the deficit must fall with time")
    k = -s
    return _out(KLa_per_min=k, KLa=k / 60.0, KLa_per_h=k * 60.0, r2=r2)


def _ref_kla_fit(t, C, Cs):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.array(t, float), np.log(Cs * 1000.0 - np.array(C, float)))
    k = -float(r.slope)
    return {"KLa_per_min": k, "KLa": k / 60.0, "KLa_per_h": k * 60.0, "r2": float(r.rvalue ** 2)}


def _rand_kla_fit(rng):
    cs, c0, k = rng.uniform(8.0, 10.0), rng.uniform(0.0, 2.0), rng.uniform(0.05, 0.4)
    n = rng.randint(5, 9)
    ts = sorted(rng.sample(range(1, 31), n))
    C = [_sig(min(cs * 0.97, cs - (cs - c0) * math.exp(-k * v) + rng.uniform(-0.05, 0.05)), 3) for v in ts]
    return {"t": [float(v) for v in ts], "C": [max(0.0, v) for v in C], "Cs": _sig(cs, 3) * MG}


_add("enveng_kla_regression",
     name="KLa fitted to a reaeration test", topic="wastewater / aeration",
     purpose="The volumetric oxygen-transfer coefficient KLa from a series of dissolved-oxygen readings during a reaeration test, by linear regression of ln(Cs - C) against time.",
     inputs=[_vecin("t", "times in minutes", 3, 60), _vecin("C", "DO readings in mg/L (same order)", 3, 60), _phys("Cs", "mass_concentration", "saturation DO")],
     outputs=[_o("KLa_per_min", "number", "KLa in 1/min"), _o("KLa", "first_order_rate_constant", "KLa"), _o("KLa_per_h", "number", "KLa in 1/h"),
              _o("r2", "number", "coefficient of determination")],
     primary="KLa_per_min", conditions=["every reading below Cs", "at least three distinct times"], method="least squares of ln(Cs - C) = ln(Cs - C0) - KLa t",
     latex=r"\ln(C_s - C) = \ln(C_s - C_0) - K_La\,t",
     run=kla_fit, reference=_ref_kla_fit,
     wrong=lambda t, C, Cs: (lambda r: dict(r, KLa_per_min=_r(r["KLa_per_min"] / math.log(10.0)), KLa=_r(r["KLa"] / math.log(10.0)),
                                           KLa_per_h=_r(r["KLa_per_h"] / math.log(10.0))))(kla_fit(t, C, Cs)),
     rand=_rand_kla_fit,
     phrases=("KLa regression", "KLa from reaeration test"),
     examples=[_ex({"t": [2.0, 4.0, 6.0, 8.0, 10.0, 12.0], "C": [2.5, 4.1, 5.4, 6.3, 7.0, 7.6], "Cs": 9.1e-3}),
               _ex({"t": [1.0, 3.0, 5.0, 9.0], "C": [1.2, 3.0, 4.3, 6.1], "Cs": 8.8e-3})],
     level="advanced")


def aeration_curve(Cs, C0, KLa, t):
    _pos("Cs and KLa", Cs, KLa)
    _nonneg("C0 and t", C0, t)
    _need(C0 <= Cs, "the starting DO cannot exceed saturation")
    C = Cs - (Cs - C0) * math.exp(-KLa * t)
    return _out(C=C, C_mgL=C * 1e3, OTR=KLa * (Cs - C), OTR_mg_per_L_h=KLa * (Cs - C) * 1e3 * HOUR)


def _ref_aeration_curve(Cs, C0, KLa, t):
    C = _ivp(lambda _t, y: [KLa * (Cs - y[0])], t, [C0], atol=[1e-20])[0] if t > 0 else C0
    return {"C": C, "C_mgL": C * 1e3, "OTR": KLa * (Cs - C), "OTR_mg_per_L_h": KLa * (Cs - C) * 3.6e6}


_add("enveng_aeration_do_curve",
     name="Dissolved oxygen during aeration and the transfer rate", topic="wastewater / aeration",
     purpose="The dissolved oxygen after an aeration time from the starting DO, the saturation DO and KLa, and the oxygen transfer rate KLa (Cs - C) at that moment.",
     inputs=[_phys("Cs", "mass_concentration", "saturation DO"), _phys("C0", "mass_concentration", "starting DO"),
             _phys("KLa", "first_order_rate_constant", "volumetric transfer coefficient", show=("1/h", HOUR)), _phys("t", "time", "aeration time", show=("min", 1.0 / 60.0))],
     outputs=[_o("C", "mass_concentration", "DO at t"), _o("C_mgL", "number", "in mg/L"), _o("OTR", "volumetric_transfer_rate", "oxygen transfer rate at t"),
              _o("OTR_mg_per_L_h", "number", "transfer rate in mg/(L h)")],
     primary="C", conditions=["C0 <= Cs", "no oxygen uptake"], method="dC/dt = KLa (Cs - C), C = Cs - (Cs - C0) e^(-KLa t)", latex=r"C = C_s - (C_s - C_0)e^{-K_La\,t}",
     run=aeration_curve, reference=_ref_aeration_curve,
     wrong=lambda Cs, C0, KLa, t: (lambda C: _out(C=C, C_mgL=C * 1e3, OTR=KLa * (Cs - C), OTR_mg_per_L_h=KLa * (Cs - C) * 1e3 * HOUR))(Cs * (1.0 - math.exp(-KLa * t))),
     rand=lambda rng: {"Cs": _sig(rng.uniform(8.0, 10.0), 3) * MG, "C0": _sig(rng.uniform(0.5, 4.0), 3) * MG, "KLa": _sig(rng.uniform(2.0, 20.0), 3) / HOUR,
                       "t": _sig(rng.uniform(1.0, 30.0), 3) * 60.0},
     phrases=("Aeration DO curve", "Dissolved oxygen during aeration"),
     examples=[_ex({"Cs": 9.1e-3, "C0": 1.0e-3, "KLa": 10.0 / HOUR, "t": 600.0}), _ex({"Cs": 8.5e-3, "C0": 2.0e-3, "KLa": 4.0 / HOUR, "t": 1200.0})],
     level="intermediate")


# ================================================================================================ GAS TRANSFER: Henry's law
_HFORMS = ("pa_m3_per_mol", "atm_m3_per_mol", "dimensionless", "atm", "mol_per_m3_pa", "m_per_atm")


def henry_convert(H, form, T):
    _pos("H and T", H, T)
    _need(form in _HFORMS, "form: " + ", ".join(_HFORMS))
    if form == "pa_m3_per_mol":
        hpc = H
    elif form == "atm_m3_per_mol":
        hpc = H * P_ATM
    elif form == "dimensionless":
        hpc = H * R_GAS * T
    elif form == "atm":
        hpc = H * P_ATM / RHO_W_MOLAR
    elif form == "mol_per_m3_pa":
        hpc = 1.0 / H
    else:
        hpc = P_ATM / (H * 1000.0)
    return _out(H_pa_m3_per_mol=hpc, H_atm_m3_per_mol=hpc / P_ATM, H_dimensionless=hpc / (R_GAS * T),
                H_atm=hpc * RHO_W_MOLAR / P_ATM, H_mol_per_m3_pa=1.0 / hpc, H_m_per_atm=P_ATM / (hpc * 1000.0))


def _ref_henry_convert(H, form, T):
    from chemicals import Henry_converter as hc
    kw = {"rhom": 55344.59, "MW": 18.01528}
    scale = {"pa_m3_per_mol": "m^3*Pa/mol", "atm_m3_per_mol": "m^3*atm/mol", "atm": "atm", "mol_per_m3_pa": "mol/(m^3*Pa)", "m_per_atm": "M/atm"}
    if form == "dimensionless":
        hpc = hc(H * T / 298.15, "KHcc", "m^3*Pa/mol", **kw)        # the library's dimensionless scale is fixed at 298.15 K
    else:
        hpc = hc(H, scale[form], "m^3*Pa/mol", **kw)
    return {"H_pa_m3_per_mol": hpc, "H_atm_m3_per_mol": hc(hpc, "m^3*Pa/mol", "m^3*atm/mol", **kw),
            "H_dimensionless": hc(hpc, "m^3*Pa/mol", "KHcc", **kw) * 298.15 / T, "H_atm": hc(hpc, "m^3*Pa/mol", "atm", **kw),
            "H_mol_per_m3_pa": hc(hpc, "m^3*Pa/mol", "mol/(m^3*Pa)", **kw), "H_m_per_atm": hc(hpc, "m^3*Pa/mol", "M/atm", **kw)}


_add("enveng_henry_constant_conversion",
     name="Henry's law constant conversion between forms", topic="gas transfer / Henry's law",
     purpose="A Henry's law constant converted between its common forms: volatility (Pa m^3/mol, atm m^3/mol), dimensionless gas/liquid ratio at temperature T, mole-fraction form (atm) and solubility forms (mol/(m^3 Pa), mol/(L atm)).",
     inputs=[_numin("H", "the Henry's constant value in the stated form"),
             _choice("form", _HFORMS, "form of H: pa_m3_per_mol, atm_m3_per_mol, dimensionless (Cg/Cl), atm (p = H x), mol_per_m3_pa or m_per_atm (mol/(L atm))"),
             _phys("T", "temperature", "temperature (for the dimensionless form)")],
     outputs=[_o("H_pa_m3_per_mol", "number", "volatility constant, Pa m^3/mol"), _o("H_atm_m3_per_mol", "number", "volatility constant, atm m^3/mol"),
              _o("H_dimensionless", "number", "dimensionless gas/liquid ratio at T"), _o("H_atm", "number", "mole-fraction form, atm"),
              _o("H_mol_per_m3_pa", "number", "solubility constant, mol/(m^3 Pa)"), _o("H_m_per_atm", "number", "solubility constant, mol/(L atm)")],
     primary="H_dimensionless", conditions=["dilute aqueous solution (water 55344.59 mol/m^3)"],
     method="all forms through the volatility constant Hpc: Hcc = Hpc/(R T), Hpx = Hpc rho_w, Hcp = 1/Hpc",
     latex=r"H_{cc} = \frac{H_{pc}}{RT},\quad H_{px} = H_{pc}\,c_w,\quad H_{cp} = \frac{1}{H_{pc}}",
     run=henry_convert, reference=_ref_henry_convert,
     wrong=lambda H, form, T: (lambda r: dict(r, H_dimensionless=_r(r["H_pa_m3_per_mol"] / (R_GAS * T0C))))(henry_convert(H, form, T)),
     rand=lambda rng: (lambda hpc, form, T: {"form": form, "T": T, "H": _sig({"pa_m3_per_mol": hpc, "atm_m3_per_mol": hpc / P_ATM, "dimensionless": hpc / (R_GAS * T),
                                                                            "atm": hpc * RHO_W_MOLAR / P_ATM, "mol_per_m3_pa": 1.0 / hpc,
                                                                            "m_per_atm": P_ATM / (hpc * 1000.0)}[form], 4)})(
         _lu(rng, 1.0, 5000.0), rng.choice(_HFORMS), _sig(T0C + rng.choice([rng.uniform(5.0, 20.0), rng.uniform(25.0, 40.0)]), 4)),
     phrases=("Henry's constant conversion", "Convert Henry's law constant"),
     examples=[_ex({"H": 0.0055, "form": "atm_m3_per_mol", "T": 298.15}), _ex({"H": 0.25, "form": "dimensionless", "T": 288.15})],
     level="advanced")


def henry_equilibrium(p, H, M=None):
    _pos("p and H", p, H)
    C = p / H
    out = {"C": C}
    if M is not None:
        _pos("the molar mass M", M)
        out["C_mass"] = C * M
        out["C_mgL"] = C * M * 1e3
    return _out(**out)


def _ref_henry_equilibrium(p, H, M=None):
    from chemicals import Henry_converter as hc
    hcp = hc(H, "m^3*Pa/mol", "mol/(m^3*Pa)", rhom=55344.59, MW=18.01528)
    C = hcp * p
    out = {"C": C}
    if M is not None:
        out["C_mass"] = C * M
        out["C_mgL"] = C * M * 1e3
    return out


_add("enveng_henry_law_equilibrium",
     name="Dissolved concentration in equilibrium with a gas (Henry's law)", topic="gas transfer / Henry's law",
     purpose="The equilibrium molar (and, with the molar mass, mass) concentration of a gas dissolved in water from its partial pressure and the Henry's volatility constant.",
     inputs=[_phys("p", "pressure", "partial pressure of the gas", show=("atm", 1.0 / P_ATM)),
             _phys("H", "henry_volatility", "Henry's volatility constant", show=("atm*m^3/mol", 1.0 / P_ATM)),
             _phys("M", "molar_mass", "molar mass (for the mass concentration)", required=False, show=("g/mol", 1e3))],
     outputs=[_o("C", "molar_concentration", "dissolved molar concentration"), _o("C_mass", "mass_concentration", "dissolved mass concentration"),
              _o("C_mgL", "number", "in mg/L")],
     primary="C", conditions=["dilute solution, equilibrium"], method="C = p/H", latex=r"C = \frac{p}{H}",
     run=henry_equilibrium, reference=_ref_henry_equilibrium,
     wrong=lambda p, H, M=None: (lambda C: _out(**({"C": C} if M is None else {"C": C, "C_mass": C * M, "C_mgL": C * M * 1e3})))(p * H / 1e8),
     rand=lambda rng: dict({"p": _sig(_lu(rng, 10.0, 1e5), 3), "H": _sig(_lu(rng, 1.0, 1e5), 3)},
                           **({"M": _sig(rng.uniform(16.0, 160.0), 4) * 1e-3} if rng.random() < 0.5 else {})),
     phrases=("Henry's law equilibrium", "Dissolved gas concentration by Henry's law"),
     examples=[_ex({"p": 0.21 * P_ATM, "H": 0.79 * P_ATM, "M": 0.0319988}), _ex({"p": 101.325, "H": 0.0055 * P_ATM})],
     level="beginner", weak="the volatility constant converted to its solubility form by the chemicals library (Henry_converter)")


def henry_vant_hoff(Hcp, B, T, Tref):
    _pos("Hcp, T and Tref", Hcp, T, Tref)
    _fin(B)
    h = Hcp * math.exp(B * (1.0 / T - 1.0 / Tref))
    return _out(Hcp_T=h, Hpc_T=1.0 / h)


def _ref_henry_vant_hoff(Hcp, B, T, Tref):
    # d ln H/dT = -B/T^2 integrated from Tref to T (parametrised on s in [0, 1])
    lnh = _ivp(lambda s, y: [-B / (Tref + s * (T - Tref)) ** 2 * (T - Tref)], 1.0, [math.log(Hcp)], atol=[1e-16])[0]
    h = math.exp(lnh)
    return {"Hcp_T": h, "Hpc_T": 1.0 / h}


_add("enveng_henry_temperature_dependence",
     name="Temperature dependence of Henry's constant (van 't Hoff)", topic="gas transfer / Henry's law",
     purpose="The Henry's solubility constant at a temperature T from its value at a reference temperature and the van 't Hoff coefficient B = -d ln Hcp / d(1/T) (the form tabulated in solubility compilations).",
     inputs=[_phys("Hcp", "henry_constant", "Henry's solubility constant at Tref"), _numin("B", "temperature coefficient d ln Hcp/d(1/T), in K", words=("K",)),
             _phys("T", "temperature", "temperature of interest"), _phys("Tref", "temperature", "reference temperature (usually 298.15 K)")],
     outputs=[_o("Hcp_T", "henry_constant", "solubility constant at T"), _o("Hpc_T", "henry_volatility", "volatility constant 1/Hcp at T")],
     primary="Hcp_T", conditions=["constant enthalpy of solution over the range"], method="Hcp(T) = Hcp(Tref) exp(B (1/T - 1/Tref))",
     latex=r"H^{cp}(T) = H^{cp}(T_{ref})\exp\!\left[B\left(\frac1T - \frac{1}{T_{ref}}\right)\right]",
     run=henry_vant_hoff, reference=_ref_henry_vant_hoff,
     wrong=lambda Hcp, B, T, Tref: (lambda h: _out(Hcp_T=h, Hpc_T=1.0 / h))(Hcp * math.exp(B * (1.0 / Tref - 1.0 / T))),
     rand=lambda rng: {"Hcp": _sig(_lu(rng, 1e-5, 1.0), 3), "B": _sig(rng.uniform(1000.0, 5000.0), 3), "T": _sig(T0C + rng.choice([rng.uniform(0.0, 20.0), rng.uniform(30.0, 50.0)]), 4),
                       "Tref": 298.15},
     phrases=("Henry's constant temperature dependence", "Van 't Hoff Henry's constant"),
     examples=[_ex({"Hcp": 1.3e-5, "B": 1500.0, "T": 283.15, "Tref": 298.15}), _ex({"Hcp": 0.0018, "B": 3200.0, "T": 308.15, "Tref": 298.15})],
     level="advanced")


def two_film(kl, kg, H, C, Cg):
    _pos("kl, kg and H", kl, kg, H)
    _nonneg("C and Cg", C, Cg)
    KL = 1.0 / (1.0 / kl + 1.0 / (H * kg))
    return _out(KL=KL, flux=KL * (C - Cg / H), liquid_resistance_fraction=KL / kl)


def _ref_two_film(kl, kg, H, C, Cg):
    rl, rg = 1.0 / kl, 1.0 / (H * kg)
    KL = 1.0 / (rl + rg)
    Cstar = Cg / H
    return {"KL": KL, "flux": (C - Cstar) / (rl + rg), "liquid_resistance_fraction": rl / (rl + rg)}


_add("enveng_two_film_transfer",
     name="Overall liquid-phase transfer coefficient and flux (two-film theory)", topic="gas transfer / two-film theory",
     purpose="The overall liquid-side mass-transfer coefficient from the liquid- and gas-film coefficients and the dimensionless Henry's constant, the share of resistance in the liquid film, and the flux from water to air (negative when the gas dissolves).",
     inputs=[_phys("kl", "speed", "liquid-film coefficient"), _phys("kg", "speed", "gas-film coefficient"), _numin("H", "dimensionless Henry's constant (Cg/Cl)"),
             _phys("C", "mass_concentration", "bulk liquid concentration"), _phys("Cg", "mass_concentration", "bulk gas concentration", show=("ug/m^3", 1e9))],
     outputs=[_o("KL", "speed", "overall liquid-phase coefficient"), _o("flux", "mass_flux", "flux from liquid to gas"),
              _o("liquid_resistance_fraction", "number", "fraction of the resistance in the liquid film")],
     primary="KL", conditions=["linear equilibrium"], method="1/KL = 1/kl + 1/(H kg); flux = KL (C - Cg/H)",
     latex=r"\frac{1}{K_L} = \frac{1}{k_l} + \frac{1}{Hk_g},\quad N = K_L\left(C - \frac{C_g}{H}\right)",
     run=two_film, reference=_ref_two_film,
     wrong=lambda kl, kg, H, C, Cg: (lambda KL: _out(KL=KL, flux=KL * (C - Cg / H), liquid_resistance_fraction=KL / kl))(1.0 / (1.0 / kl + H / kg)),
     rand=lambda rng: {"kl": _sig(_lu(rng, 1e-6, 1e-4), 3), "kg": _sig(_lu(rng, 1e-4, 1e-2), 3), "H": _sig(_lu(rng, 0.001, 2.0), 3),
                       "C": _sig(rng.uniform(0.01, 5.0), 3) * MG, "Cg": _sig(rng.uniform(0.0, 50.0), 3) * 1e-9},
     phrases=("Two-film mass transfer", "Two-film theory transfer"),
     examples=[_ex({"kl": 2e-5, "kg": 5e-3, "H": 0.2, "C": 1e-3, "Cg": 0.0}), _ex({"kl": 5e-5, "kg": 2e-3, "H": 0.005, "C": 0.5e-3, "Cg": 20e-9})],
     level="advanced", weak="the same series-resistance relations evaluated resistance by resistance")


def air_stripping(Qw, Qa, H, Cin, Cout, HTU):
    _pos("Qw, Qa, H, Cin, Cout and HTU", Qw, Qa, H, Cin, Cout, HTU)
    _need(Cout < Cin, "the effluent concentration must be below the influent")
    ratio_min = (Cin - Cout) / (H * Cin)
    _need(Qa / Qw > ratio_min, "the air-to-water ratio is below the minimum: the removal cannot be reached")
    R = H * Qa / Qw
    if abs(R - 1.0) <= 1e-9:
        ntu = Cin / Cout - 1.0
    else:
        ntu = R / (R - 1.0) * math.log(((Cin / Cout) * (R - 1.0) + 1.0) / R)
    return _out(R=R, NTU=ntu, Z=HTU * ntu, air_to_water_min=ratio_min)


def _ref_air_stripping(Qw, Qa, H, Cin, Cout, HTU):
    R = H * Qa / Qw
    ntu = _quad(lambda c: 1.0 / (c - (Qw / Qa) * (c - Cout) / H), Cout, Cin)     # clean air enters at the bottom
    return {"R": R, "NTU": ntu, "Z": HTU * ntu, "air_to_water_min": (Cin - Cout) / (H * Cin)}


def _rand_stripping(rng):
    Cin = _sig(rng.uniform(50.0, 2000.0), 3) * 1e-6
    H = _sig(rng.uniform(0.1, 1.0), 3)
    Cout = _sig(Cin * rng.uniform(0.005, 0.2), 3)
    Qw = _sig(rng.uniform(0.01, 0.2), 3)
    Qa = _sig(Qw * (Cin - Cout) / (H * Cin) * rng.uniform(1.3, 6.0), 3)
    return {"Qw": Qw, "Qa": Qa, "H": H, "Cin": Cin, "Cout": Cout, "HTU": _sig(rng.uniform(0.5, 2.0), 3)}


_add("enveng_air_stripping_tower",
     name="Packed-tower air stripping: stripping factor, transfer units and height", topic="gas transfer / air stripping",
     purpose="The stripping factor, number of transfer units, packed height and minimum air-to-water ratio of a countercurrent packed tower stripping a volatile compound with clean air.",
     inputs=[_phys("Qw", "flow_rate", "water flow"), _phys("Qa", "flow_rate", "air flow"), _numin("H", "dimensionless Henry's constant (Cg/Cl)"),
             _phys("Cin", "mass_concentration", "influent concentration", show=("ug/L", 1e6)), _phys("Cout", "mass_concentration", "required effluent concentration", show=("ug/L", 1e6)),
             _phys("HTU", "length", "height of a transfer unit")],
     outputs=[_o("R", "number", "stripping factor H Qa/Qw"), _o("NTU", "number", "number of transfer units"), _o("Z", "length", "packed height HTU NTU"),
              _o("air_to_water_min", "number", "minimum air-to-water ratio (Cin - Cout)/(H Cin)")],
     primary="Z", conditions=["Qa/Qw above the minimum", "clean inlet air", "dilute, linear equilibrium"],
     method="R = H Qa/Qw; NTU = R/(R - 1) ln[((Cin/Cout)(R - 1) + 1)/R] (Cin/Cout - 1 when R = 1); Z = HTU NTU",
     latex=r"NTU = \frac{R}{R-1}\ln\!\left[\frac{(C_{in}/C_{out})(R-1) + 1}{R}\right],\quad Z = HTU\cdot NTU",
     run=air_stripping, reference=_ref_air_stripping,
     wrong=lambda Qw, Qa, H, Cin, Cout, HTU: (lambda R: (lambda n: _out(R=R, NTU=n, Z=HTU * n, air_to_water_min=(Cin - Cout) / (H * Cin)))(
         math.log(((Cin / Cout) * (R - 1.0) + 1.0) / R)))(H * Qa / Qw),
     rand=_rand_stripping,
     phrases=("Air stripping tower", "Packed tower air stripping"),
     examples=[_ex({"Qw": 0.05, "Qa": 1.5, "H": 0.2, "Cin": 200e-6, "Cout": 5e-6, "HTU": 1.0}), _ex({"Qw": 0.1, "Qa": 2.0, "H": 0.5, "Cin": 1000e-6, "Cout": 10e-6, "HTU": 0.8})],
     level="expert")


# ================================================================================================ NOISE
_DB = ("dB", "dBA", "dB(A)")


def _db_list(L, name="every level"):
    for v in L:
        _need(math.isfinite(v) and -50.0 <= v <= 250.0, f"{name} must be a sound level in dB between -50 and 250")


def db_add(L):
    _need(len(L) >= 2, "give at least two levels")
    _db_list(L)
    return _out(L_total=10.0 * math.log10(sum(10.0 ** (v / 10.0) for v in L)))


def _ref_db_add(L):
    import numpy as np
    from scipy.special import logsumexp
    return {"L_total": float(10.0 / np.log(10.0) * logsumexp(np.array(L) * np.log(10.0) / 10.0))}


_add("enveng_decibel_addition",
     name="Adding sound levels in decibels", topic="noise / decibel arithmetic",
     purpose="The combined sound pressure level of several incoherent sources, added on an energy basis.",
     inputs=[_vecin("L", "sound levels in dB", 2, 100)],
     outputs=[_o("L_total", "number", "combined level, dB")],
     primary="L_total", conditions=["incoherent sources"], method="L = 10 log10(sum 10^(Li/10))", latex=r"L = 10\log_{10}\sum_i 10^{L_i/10}",
     run=db_add, reference=_ref_db_add,
     wrong=lambda L: _out(L_total=20.0 * math.log10(sum(10.0 ** (v / 20.0) for v in L))),
     rand=lambda rng: {"L": [_sig(rng.uniform(40.0, 110.0), 3) for _ in range(rng.randint(2, 6))]},
     phrases=("Add sound levels", "Decibel addition"),
     examples=[_ex({"L": [85.0, 85.0]}), _ex({"L": [90.0, 87.0, 82.0, 75.0]})],
     level="beginner")


def db_subtract(Lt, Lb):
    _db_list([Lt, Lb], "Lt and Lb")
    _need(Lt > Lb, "the total level must exceed the background level")
    return _out(L_source=10.0 * math.log10(10.0 ** (Lt / 10.0) - 10.0 ** (Lb / 10.0)))


def _ref_db_subtract(Lt, Lb):
    import numpy as np
    from scipy.special import logsumexp
    return {"L_source": float(10.0 / np.log(10.0) * logsumexp([Lt * np.log(10.0) / 10.0, Lb * np.log(10.0) / 10.0], b=[1.0, -1.0]))}


_add("enveng_background_noise_correction",
     name="Source level after removing background noise", topic="noise / decibel arithmetic",
     purpose="The sound level of a source alone from the total measured level and the background level (energy subtraction).",
     inputs=[_numin("Lt", "total measured level, dB", words=_DB), _numin("Lb", "background level, dB", words=_DB)],
     outputs=[_o("L_source", "number", "level of the source alone, dB")],
     primary="L_source", conditions=["Lt > Lb"], method="Ls = 10 log10(10^(Lt/10) - 10^(Lb/10))", latex=r"L_s = 10\log_{10}\left(10^{L_t/10} - 10^{L_b/10}\right)",
     run=db_subtract, reference=_ref_db_subtract,
     wrong=lambda Lt, Lb: _out(L_source=Lt - Lb),
     rand=lambda rng: (lambda Lt: {"Lt": Lt, "Lb": _sig(Lt - rng.uniform(1.0, 10.0), 3)})(_sig(rng.uniform(50.0, 110.0), 3)),
     phrases=("Background noise correction", "Subtract background noise"),
     examples=[_ex({"Lt": 85.0, "Lb": 80.0}), _ex({"Lt": 72.0, "Lb": 68.0})],
     level="beginner")


def distance_attenuation(L1, r1, r2, source):
    _db_list([L1], "L1")
    _pos("r1 and r2", r1, r2)
    _need(source in ("point", "line"), "source: point or line")
    f = 20.0 if source == "point" else 10.0
    return _out(L2=L1 - f * math.log10(r2 / r1))


def _ref_distance_attenuation(L1, r1, r2, source):
    import numpy as np
    # intensity ratio: spherical spreading I ~ 1/r^2, cylindrical I ~ 1/r
    ratio = (r1 / r2) ** 2 if source == "point" else r1 / r2
    return {"L2": float(L1 + 10.0 * np.log10(ratio))}


_add("enveng_noise_distance_attenuation",
     name="Sound level at another distance (point or line source)", topic="noise / propagation",
     purpose="The sound level at a second distance from a point source (6 dB per doubling) or a line source such as a busy road (3 dB per doubling), free field.",
     inputs=[_numin("L1", "level at distance r1, dB", words=_DB), _phys("r1", "length", "first distance"), _phys("r2", "length", "second distance"),
             _choice("source", ("point", "line"), "point or line source")],
     outputs=[_o("L2", "number", "level at distance r2, dB")],
     primary="L2", conditions=["free field, no absorption"], method="L2 = L1 - 20 log10(r2/r1) (point), L1 - 10 log10(r2/r1) (line)",
     latex=r"L_2 = L_1 - 20\log_{10}\frac{r_2}{r_1}\ \text{(point)}",
     run=distance_attenuation, reference=_ref_distance_attenuation,
     wrong=lambda L1, r1, r2, source: _out(L2=L1 - (10.0 if source == "point" else 20.0) * math.log10(r2 / r1)),
     rand=lambda rng: {"L1": _sig(rng.uniform(60.0, 110.0), 3), "r1": _sig(rng.uniform(1.0, 20.0), 3), "r2": _sig(rng.uniform(30.0, 1000.0), 3),
                       "source": rng.choice(["point", "line"])},
     phrases=("Noise distance attenuation", "Sound level at distance"),
     examples=[_ex({"L1": 90.0, "r1": 10.0, "r2": 80.0, "source": "point"}), _ex({"L1": 75.0, "r1": 15.0, "r2": 120.0, "source": "line"})],
     level="intermediate", weak="the same spreading laws written as intensity ratios")


def leq(L, t):
    _need(len(L) == len(t) and len(L) >= 1, "give one duration per level")
    _db_list(L)
    for v in t:
        _need(math.isfinite(v) and v >= 0, "durations must be zero or positive")
    T = sum(t)
    _need(T > 0, "the total duration must be positive")
    return _out(Leq=10.0 * math.log10(sum(ti * 10.0 ** (v / 10.0) for v, ti in zip(L, t)) / T))


def _ref_leq(L, t):
    import numpy as np
    from scipy.special import logsumexp
    t = np.array(t, float)
    return {"Leq": float(10.0 / np.log(10.0) * logsumexp(np.array(L) * np.log(10.0) / 10.0, b=t / t.sum()))}


_add("enveng_equivalent_sound_level",
     name="Equivalent continuous sound level Leq", topic="noise / exposure",
     purpose="The equivalent continuous sound level of a period made of intervals at different levels (energy average weighted by duration).",
     inputs=[_vecin("L", "levels of the intervals in dB", 1, 200), _vecin("t", "durations of the intervals (any one unit)", 1, 200)],
     outputs=[_o("Leq", "number", "equivalent continuous level, dB")],
     primary="Leq", conditions=["one duration per level"], method="Leq = 10 log10(sum t_i 10^(L_i/10)/sum t_i)", latex=r"L_{eq} = 10\log_{10}\frac{\sum_i t_i10^{L_i/10}}{\sum_i t_i}",
     run=leq, reference=_ref_leq,
     wrong=lambda L, t: _out(Leq=sum(v * ti for v, ti in zip(L, t)) / sum(t)),
     rand=lambda rng: (lambda n: {"L": [_sig(rng.uniform(45.0, 100.0), 3) for _ in range(n)], "t": [_sig(rng.uniform(0.5, 8.0), 2) for _ in range(n)]})(rng.randint(2, 6)),
     phrases=("Equivalent sound level", "Leq"),
     examples=[_ex({"L": [70.0, 85.0, 60.0], "t": [4.0, 1.0, 3.0]}), _ex({"L": [92.0, 78.0], "t": [2.0, 6.0]})],
     level="intermediate")


def ldn(L, metric):
    _need(len(L) == 24, "give 24 hourly levels, starting with the hour from midnight to 01:00")
    _db_list(L)
    _need(metric in ("ldn", "lden"), "metric: ldn or lden")
    s = 0.0
    for h, v in enumerate(L):
        if metric == "ldn":
            pen = 10.0 if (h < 7 or h >= 22) else 0.0
        else:
            pen = 10.0 if (h < 7 or h >= 23) else (5.0 if h >= 19 else 0.0)
        s += 10.0 ** ((v + pen) / 10.0)
    return _out(L=10.0 * math.log10(s / 24.0))


def _ref_ldn(L, metric):
    import numpy as np
    from scipy.special import logsumexp
    hours = np.arange(24)
    if metric == "ldn":
        pen = np.where((hours < 7) | (hours >= 22), 10.0, 0.0)
    else:
        pen = np.select([(hours < 7) | (hours >= 23), hours >= 19], [10.0, 5.0], 0.0)
    return {"L": float(10.0 / np.log(10.0) * logsumexp((np.array(L) + pen) * np.log(10.0) / 10.0) - 10.0 * np.log10(24.0))}


_add("enveng_day_night_level",
     name="Day-night (Ldn) and day-evening-night (Lden) sound levels", topic="noise / community noise",
     purpose="The day-night average level (10 dB night penalty, 22:00-07:00) or the day-evening-night level (5 dB evening 19:00-23:00, 10 dB night 23:00-07:00) from 24 hourly equivalent levels.",
     inputs=[_vecin("L", "24 hourly Leq values in dB, the first for 00:00-01:00", 24, 24), _choice("metric", ("ldn", "lden"), "ldn or lden")],
     outputs=[_o("L", "number", "Ldn or Lden, dB")],
     primary="L", conditions=["exactly 24 hourly levels"], method="L = 10 log10((1/24) sum 10^((L_h + penalty_h)/10))",
     latex=r"L_{dn} = 10\log_{10}\frac{1}{24}\left(\sum_{day}10^{L_h/10} + \sum_{night}10^{(L_h+10)/10}\right)",
     run=ldn, reference=_ref_ldn,
     wrong=lambda L, metric: _out(L=10.0 * math.log10(sum(10.0 ** (v / 10.0) for v in L) / 24.0)),
     rand=lambda rng: {"L": [_sig(rng.uniform(45.0, 75.0), 3) for _ in range(24)], "metric": rng.choice(["ldn", "lden"])},
     phrases=("Day-night sound level", "Ldn and Lden"),
     examples=[_ex({"L": [50.0] * 7 + [65.0] * 12 + [60.0] * 3 + [55.0] * 2, "metric": "ldn"}),
               _ex({"L": [48.0] * 7 + [66.0] * 12 + [62.0] * 4 + [52.0], "metric": "lden"})],
     level="advanced")


def sound_power_to_pressure(Lw, r, Qd):
    _db_list([Lw], "Lw")
    _pos("r and the directivity Qd", r, Qd)
    return _out(Lp=Lw + 10.0 * math.log10(Qd / (4.0 * math.pi * r * r)))


def _ref_sound_power_to_pressure(Lw, r, Qd):
    import numpy as np
    W = 1e-12 * 10.0 ** (Lw / 10.0)           # sound power, W
    I = Qd * W / (4.0 * np.pi * r ** 2)       # intensity at r, W/m^2 (reference 1e-12)
    return {"Lp": float(10.0 * np.log10(I / 1e-12))}


_add("enveng_sound_power_to_pressure",
     name="Sound pressure level from sound power level", topic="noise / propagation",
     purpose="The sound pressure level at a distance from a source of given sound power level and directivity factor (1 free field, 2 on a hard floor, 4 at a wall-floor edge, 8 in a corner).",
     inputs=[_numin("Lw", "sound power level, dB re 1 pW", words=_DB), _phys("r", "length", "distance from the source"),
             _numin("Qd", "directivity factor (1, 2, 4 or 8)")],
     outputs=[_o("Lp", "number", "sound pressure level, dB")],
     primary="Lp", conditions=["far field, no absorption", "rho c taken as 400 rayl (Lp = LI)"], method="Lp = Lw + 10 log10(Q/(4 pi r^2))",
     latex=r"L_p = L_W + 10\log_{10}\frac{Q}{4\pi r^2}",
     run=sound_power_to_pressure, reference=_ref_sound_power_to_pressure,
     wrong=lambda Lw, r, Qd: _out(Lp=Lw - 20.0 * math.log10(r) - 11.0),
     rand=lambda rng: {"Lw": _sig(rng.uniform(70.0, 130.0), 3), "r": _sig(rng.uniform(1.0, 200.0), 3), "Qd": rng.choice([2.0, 4.0, 8.0])},
     phrases=("Sound power to pressure level", "Sound pressure from sound power"),
     examples=[_ex({"Lw": 110.0, "r": 20.0, "Qd": 2.0}), _ex({"Lw": 95.0, "r": 5.0, "Qd": 1.0})],
     level="intermediate", weak="the same relation through sound power and intensity in SI")


def noise_dose(L, hours):
    _need(len(L) == len(hours) and len(L) >= 1, "give one exposure time per level")
    _db_list(L)
    for v in hours:
        _need(math.isfinite(v) and v >= 0, "exposure times must be zero or positive")
    D = 0.0
    for v, h in zip(L, hours):
        if v >= 80.0:
            D += h / (8.0 / 2.0 ** ((v - 90.0) / 5.0))
    D *= 100.0
    _need(D > 0, "no exposure at or above 80 dBA: the dose is zero")
    return _out(dose_percent=D, TWA=16.61 * math.log10(D / 100.0) + 90.0)


def _ref_noise_dose(L, hours):
    import numpy as np
    L, h = np.array(L, float), np.array(hours, float)
    T = 8.0 * np.power(2.0, -(L - 90.0) / 5.0)
    D = float(100.0 * np.sum(np.where(L >= 80.0, h / T, 0.0)))
    return {"dose_percent": D, "TWA": float(90.0 + 16.61 * np.log10(D) - 16.61 * 2.0)}


_add("enveng_occupational_noise_dose",
     name="Occupational noise dose and 8-hour TWA (OSHA, 5 dB exchange)", topic="noise / occupational exposure",
     purpose="The daily noise dose (per cent of the permissible exposure) and the 8-hour time-weighted average from levels and exposure times, with the 90 dBA criterion, 5 dB exchange rate and 80 dBA threshold.",
     inputs=[_vecin("L", "A-weighted levels in dBA", 1, 50), _vecin("hours", "exposure time at each level in hours", 1, 50)],
     outputs=[_o("dose_percent", "number", "noise dose, per cent"), _o("TWA", "number", "8-hour time-weighted average, dBA")],
     primary="dose_percent", conditions=["levels below 80 dBA excluded"], method="T = 8/2^((L - 90)/5); D = 100 sum C/T; TWA = 16.61 log10(D/100) + 90",
     latex=r"D = 100\sum_i\frac{C_i}{T_i},\ T_i = \frac{8}{2^{(L_i-90)/5}},\ TWA = 16.61\log_{10}\frac{D}{100} + 90",
     run=noise_dose, reference=_ref_noise_dose,
     wrong=lambda L, hours: (lambda D: _out(dose_percent=D, TWA=16.61 * math.log10(D / 100.0) + 90.0))(
         100.0 * sum(h / (8.0 / 2.0 ** ((v - 90.0) / 3.0)) for v, h in zip(L, hours) if v >= 80.0)),
     rand=lambda rng: (lambda n: {"L": [_sig(rng.uniform(82.0, 105.0), 3) for _ in range(n)], "hours": [_sig(rng.uniform(0.25, 4.0), 2) for _ in range(n)]})(rng.randint(1, 5)),
     phrases=("Occupational noise dose", "OSHA noise dose"),
     examples=[_ex({"L": [90.0, 95.0, 85.0], "hours": [4.0, 2.0, 2.0]}), _ex({"L": [100.0, 88.0], "hours": [1.0, 7.0]})],
     level="advanced", weak="the same published rule evaluated with numpy arrays")


# ================================================================================================ SOLID WASTE
def waste_generation(pop, rate, rho, Vt):
    _need(isinstance(pop, int) and pop >= 1, "the population must be a whole number of at least 1")
    _pos("rate, rho and Vt", rate, rho, Vt)
    m_day = pop * rate                        # kg/d
    vol_day = m_day / rho
    return _out(m=m_day / DAY, m_kg_per_day=m_day, m_t_per_year=m_day * 365.0 / 1000.0, V_m3_per_day=vol_day,
                trucks_per_day=math.ceil(vol_day / Vt - 1e-12))


def _ref_waste_generation(pop, rate, rho, Vt):
    from fractions import Fraction as Fr
    m = Fr(pop) * Fr(rate)
    v = m / Fr(rho)
    q = v / Fr(Vt)
    n = q.numerator // q.denominator + (1 if q.numerator % q.denominator else 0)
    return {"m": float(m / 86400), "m_kg_per_day": float(m), "m_t_per_year": float(m * 365 / 1000), "V_m3_per_day": float(v), "trucks_per_day": int(n)}


_add("enveng_solid_waste_generation",
     name="Municipal solid waste generation and collection trucks", topic="solid waste / generation",
     purpose="The daily and yearly mass of municipal solid waste of a population, its compacted volume and the number of truck loads per day.",
     inputs=[_intin("pop", "population served", 1, 100000000), _numin("rate", "generation rate in kg per person per day", words=("kg/cap/d", "kg/(cap*d)")),
             _phys("rho", "density", "compacted density in the truck"), _phys("Vt", "volume", "truck body volume")],
     outputs=[_o("m", "mass_flow_rate", "waste generation"), _o("m_kg_per_day", "number", "in kg/d"), _o("m_t_per_year", "number", "in t per 365-day year"),
              _o("V_m3_per_day", "number", "compacted volume per day, m^3"), _o("trucks_per_day", "number", "truck loads per day (rounded up)")],
     primary="m", conditions=["full truck loads at the stated density"], method="m = P r; V = m/rho; trucks = ceil(V/Vt)",
     latex=r"\dot m = P\,r,\quad n = \left\lceil\frac{\dot m}{\rho V_t}\right\rceil",
     run=waste_generation, reference=_ref_waste_generation,
     wrong=lambda pop, rate, rho, Vt: (lambda r: dict(r, trucks_per_day=int(r["V_m3_per_day"] // Vt)))(waste_generation(pop, rate, rho, Vt)),
     rand=lambda rng: {"pop": rng.randint(1000, 500000), "rate": _sig(rng.uniform(0.5, 2.5), 3), "rho": _sig(rng.uniform(250.0, 500.0), 3),
                       "Vt": _sig(rng.uniform(10.0, 30.0), 3)},
     phrases=("Solid waste generation", "Municipal solid waste generation"),
     examples=[_ex({"pop": 50000, "rate": 1.5, "rho": 400.0, "Vt": 15.0}), _ex({"pop": 120000, "rate": 2.0, "rho": 350.0, "Vt": 20.0})],
     level="beginner", weak="exact rational arithmetic of the same definition")


def landfill_life(capacity, M, rho, cover, g):
    _pos("capacity, M and rho", capacity, M, rho)
    _need(0.0 <= cover <= 1.0, "the cover ratio must lie between 0 and 1")
    _need(0.0 <= g <= 0.2, "the growth rate must lie between 0 and 20 % a year")
    V0 = M * YEAR / rho * (1.0 + cover)       # m^3 in the first year
    n = capacity / V0 if g == 0 else math.log(1.0 + capacity * g / V0) / math.log(1.0 + g)
    return _out(V_first_year=V0, life_years=n)


def _ref_landfill_life(capacity, M, rho, cover, g):
    V0 = M * 31557600.0 / rho * (1.0 + cover)
    if g == 0:
        return {"V_first_year": V0, "life_years": capacity / V0}
    f = lambda n: V0 * ((1.0 + g) ** n - 1.0) / g - capacity  # noqa: E731
    hi = 1.0
    while f(hi) < 0:
        hi *= 2.0
    return {"V_first_year": V0, "life_years": _brentq(f, 0.0, hi)}


_add("enveng_landfill_life",
     name="Landfill life with daily cover and waste growth", topic="solid waste / landfills",
     purpose="The landfill volume used in the first year (waste at its in-place density plus cover soil) and the life of a landfill of given capacity when the waste grows at a constant yearly rate.",
     inputs=[_phys("capacity", "volume", "available landfill volume"), _phys("M", "mass_flow_rate", "waste delivered in the first year", show=("t/yr", YEAR / 1000.0)),
             _phys("rho", "density", "in-place (compacted) waste density"), _numin("cover", "cover soil volume per volume of waste (e.g. 0.25)"),
             _numin("g", "yearly growth of the waste quantity (fraction or %; 0 for none)", pct=True)],
     outputs=[_o("V_first_year", "number", "volume used in the first year, m^3"), _o("life_years", "number", "landfill life, years")],
     primary="life_years", conditions=["growth compounded yearly, life read continuously"],
     method="V0 = M (1 + cover)/rho per year; capacity = V0 ((1 + g)^n - 1)/g, so n = ln(1 + capacity g/V0)/ln(1 + g)",
     latex=r"n = \frac{\ln\left(1 + \frac{C g}{V_0}\right)}{\ln(1+g)},\quad V_0 = \frac{M(1 + c)}{\rho}",
     run=landfill_life, reference=_ref_landfill_life,
     wrong=lambda capacity, M, rho, cover, g: landfill_life(capacity, M, rho, 0.0, g),
     rand=lambda rng: {"capacity": _sig(_lu(rng, 1e5, 1e7), 3), "M": _sig(rng.uniform(10000.0, 300000.0), 3) * 1000.0 / YEAR, "rho": _sig(rng.uniform(500.0, 900.0), 3),
                       "cover": _sig(rng.uniform(0.15, 0.3), 2), "g": rng.choice([0.0, _sig(rng.uniform(0.01, 0.05), 2)])},
     phrases=("Landfill life", "Landfill design life"),
     examples=[_ex({"capacity": 2.0e6, "M": 50000.0 * 1000.0 / YEAR, "rho": 600.0, "cover": 0.25, "g": 0.0}),
               _ex({"capacity": 5.0e6, "M": 120000.0 * 1000.0 / YEAR, "rho": 700.0, "cover": 0.2, "g": 0.03})],
     level="advanced")


def landfill_gas(M, L0, k, tc, t):
    _pos("M, L0, k and tc", M, L0, k, tc)
    _nonneg("the time t", t)
    tm = min(t, tc)
    Q = M * L0 * (math.exp(-k * (t - tm)) - math.exp(-k * t))
    if t <= tc:
        Vc = M * L0 * (t - (1.0 - math.exp(-k * t)) / k)
    else:
        Vc = M * L0 * (tc - (math.exp(-k * (t - tc)) - math.exp(-k * t)) / k)
    return _out(Q_CH4=Q, Q_CH4_m3_per_year=Q * YEAR, V_CH4=Vc, Q_LFG=2.0 * Q)


def _ref_landfill_gas(M, L0, k, tc, t):
    def q(s):                                  # generation rate: every deposit decays at first order
        if s <= 0:
            return 0.0
        return _quad(lambda tau: M * k * L0 * math.exp(-k * (s - tau)), 0.0, min(s, tc))
    Q = q(t)
    pts = [0.0, min(t, tc), t]
    Vc = sum(_quad(q, a, b) for a, b in zip(pts[:-1], pts[1:]) if b > a)
    return {"Q_CH4": Q, "Q_CH4_m3_per_year": Q * 31557600.0, "V_CH4": Vc, "Q_LFG": 2.0 * Q}


_add("enveng_landfill_gas_generation",
     name="Landfill methane generation (first-order decay, constant acceptance)", topic="solid waste / landfill gas",
     purpose="The methane generation rate and the cumulative methane of a landfill receiving waste at a constant rate for its operating life, each deposit decaying at first order (LandGEM-type model in continuous form), and the landfill gas flow at 50 % methane.",
     inputs=[_phys("M", "mass_flow_rate", "waste acceptance rate", show=("t/yr", YEAR / 1000.0)),
             _phys("L0", "specific_volume", "methane generation potential", show=("m^3/t", 1000.0)),
             _phys("k", "first_order_rate_constant", "methane generation rate constant", show=("1/yr", YEAR)),
             _phys("tc", "time", "operating (filling) period", show=("yr", 1.0 / YEAR)), _phys("t", "time", "time since opening", show=("yr", 1.0 / YEAR))],
     outputs=[_o("Q_CH4", "flow_rate", "methane generation rate at t"), _o("Q_CH4_m3_per_year", "number", "in m^3 per year"),
              _o("V_CH4", "volume", "cumulative methane generated up to t"), _o("Q_LFG", "flow_rate", "landfill gas rate at 50 % methane")],
     primary="Q_CH4", conditions=["constant acceptance from opening to tc", "first-order decay of every deposit"],
     method="Q(t) = M L0 (e^(-k (t - min(t, tc))) - e^(-k t)); cumulative volume by integrating Q",
     latex=r"Q(t) = \int_0^{\min(t,t_c)} kL_0M\,e^{-k(t-\tau)}d\tau = ML_0\left(e^{-k(t-\min(t,t_c))} - e^{-kt}\right)",
     run=landfill_gas, reference=_ref_landfill_gas,
     wrong=lambda M, L0, k, tc, t: (lambda Q: _out(Q_CH4=Q, Q_CH4_m3_per_year=Q * YEAR, V_CH4=M * tc * L0 * (1.0 - math.exp(-k * t)), Q_LFG=2.0 * Q))(k * L0 * M * tc * math.exp(-k * t)),
     rand=lambda rng: {"M": _sig(rng.uniform(10000.0, 300000.0), 3) * 1000.0 / YEAR, "L0": _sig(rng.uniform(50.0, 170.0), 3) / 1000.0,
                       "k": _sig(rng.uniform(0.02, 0.2), 3) / YEAR, "tc": _sig(rng.uniform(5.0, 30.0), 3) * YEAR, "t": _sig(rng.uniform(1.0, 60.0), 3) * YEAR},
     phrases=("Landfill gas generation", "Landfill methane generation"),
     examples=[_ex({"M": 100000.0 * 1000.0 / YEAR, "L0": 0.1, "k": 0.05 / YEAR, "tc": 20 * YEAR, "t": 25 * YEAR}),
               _ex({"M": 50000.0 * 1000.0 / YEAR, "L0": 0.17, "k": 0.04 / YEAR, "tc": 15 * YEAR, "t": 10 * YEAR})],
     level="expert")


def dulong(C, H, O, S, N):
    for nm, v in (("C", C), ("H", H), ("O", O), ("S", S), ("N", N)):
        _frac(nm, v)
    _need(C + H + O + S + N <= 1.0 + 1e-12, "the mass fractions cannot add to more than 1")
    _need(C > 0, "the waste must contain carbon")
    kJ = 337.0 * C * 100 + 1419.0 * (H * 100 - O * 100 / 8.0) + 93.0 * S * 100 + 23.0 * N * 100
    _need(kJ > 0, "the composition gives no positive heating value")
    return _out(HHV=kJ * 1e3, HHV_kJ_per_kg=kJ, HHV_MJ_per_kg=kJ / 1e3)


def _ref_dulong(C, H, O, S, N):
    from fractions import Fraction as Fr
    kJ = 100 * (337 * Fr(C) + 1419 * Fr(H) - Fr(1419, 8) * Fr(O) + 93 * Fr(S) + 23 * Fr(N))
    return {"HHV": float(kJ * 1000), "HHV_kJ_per_kg": float(kJ), "HHV_MJ_per_kg": float(kJ / 1000)}


_add("enveng_dulong_heating_value",
     name="Heating value of waste from its elemental composition (modified Dulong)", topic="solid waste / energy recovery",
     purpose="The higher heating value of a waste or fuel from its dry mass fractions of carbon, hydrogen, oxygen, sulfur and nitrogen by the modified Dulong formula.",
     inputs=[_numin("C", "carbon mass fraction (fraction or %)", pct=True), _numin("H", "hydrogen mass fraction", pct=True),
             _numin("O", "oxygen mass fraction", pct=True), _numin("S", "sulfur mass fraction", pct=True), _numin("N", "nitrogen mass fraction", pct=True)],
     outputs=[_o("HHV", "specific_energy", "higher heating value"), _o("HHV_kJ_per_kg", "number", "in kJ/kg"), _o("HHV_MJ_per_kg", "number", "in MJ/kg")],
     primary="HHV", conditions=["fractions on the same (dry) basis, sum at most 1"], method="HHV (kJ/kg) = 337 C + 1419 (H - O/8) + 93 S + 23 N, mass per cent",
     latex=r"HHV = 337C + 1419\left(H - \frac O8\right) + 93S + 23N\ \ [\mathrm{kJ/kg},\ \%]",
     run=dulong, reference=_ref_dulong,
     wrong=lambda C, H, O, S, N: (lambda kJ: _out(HHV=kJ * 1e3, HHV_kJ_per_kg=kJ, HHV_MJ_per_kg=kJ / 1e3))(
         337.0 * C * 100 + 1419.0 * (H * 100 + O * 100 / 8.0) + 93.0 * S * 100 + 23.0 * N * 100),
     rand=lambda rng: {"C": _sig(rng.uniform(0.3, 0.6), 3), "H": _sig(rng.uniform(0.04, 0.08), 3), "O": _sig(rng.uniform(0.2, 0.4), 3),
                       "S": _sig(rng.uniform(0.0, 0.01), 2), "N": _sig(rng.uniform(0.0, 0.02), 2)},
     phrases=("Dulong heating value", "Heating value from elemental composition"),
     examples=[_ex({"C": 0.48, "H": 0.064, "O": 0.376, "S": 0.004, "N": 0.008}), _ex({"C": 0.6, "H": 0.072, "O": 0.228, "S": 0.002, "N": 0.012})],
     level="intermediate",
     extra_q=[{"q": "Dulong heating value: C = 48%, H = 6.4%, O = 37.6%, S = 0.4%, N = 0.8%", "like": 0}],
     weak="exact rational arithmetic of the same published formula")


def waste_moisture(m, w):
    _need(len(m) == len(w) and len(m) >= 1, "give one moisture fraction per component mass")
    for v in m:
        _need(math.isfinite(v) and v >= 0, "every mass must be zero or positive")
    for v in w:
        _need(math.isfinite(v) and 0 <= v < 1, "every moisture fraction must lie between 0 and 1")
    tot = sum(m)
    _need(tot > 0, "the total mass must be positive")
    water = sum(a * b for a, b in zip(m, w))
    dry = tot - water
    return _out(moisture_wet=water / tot, dry_mass=dry, moisture_dry=water / dry)


def _ref_waste_moisture(m, w):
    import numpy as np
    m, w = np.array(m, float), np.array(w, float)
    water = float(np.dot(m, w))
    dry = float(np.dot(m, 1.0 - w))
    return {"moisture_wet": water / (water + dry), "dry_mass": dry, "moisture_dry": water / dry}


_add("enveng_waste_moisture_content",
     name="Moisture content of mixed solid waste", topic="solid waste / properties",
     purpose="The moisture content (wet and dry basis) and the dry mass of a waste mixture from the masses of its components and their moisture fractions.",
     inputs=[_vecin("m", "wet masses of the components (any one unit)", 1, 50), _vecin("w", "moisture fractions of the components, wet basis", 1, 50)],
     outputs=[_o("moisture_wet", "number", "moisture content, wet basis"), _o("dry_mass", "number", "dry mass, in the unit of m"),
              _o("moisture_dry", "number", "moisture content, dry basis")],
     primary="moisture_wet", conditions=["moisture fractions below 1"], method="water = sum m_i w_i; M_wet = water/sum m; M_dry = water/(sum m - water)",
     latex=r"M_{wet} = \frac{\sum m_iw_i}{\sum m_i}",
     run=waste_moisture, reference=_ref_waste_moisture,
     wrong=lambda m, w: (lambda mw: _out(moisture_wet=mw, dry_mass=sum(m) * (1.0 - mw), moisture_dry=mw / (1.0 - mw)))(sum(w) / len(w)),
     rand=lambda rng: (lambda n: {"m": [_sig(rng.uniform(1.0, 40.0), 3) for _ in range(n)], "w": [_sig(rng.uniform(0.02, 0.75), 2) for _ in range(n)]})(rng.randint(2, 7)),
     phrases=("Waste moisture content", "Moisture content of mixed waste"),
     examples=[_ex({"m": [40.0, 25.0, 15.0, 20.0], "w": [0.7, 0.06, 0.02, 0.2]}), _ex({"m": [10.0, 30.0], "w": [0.6, 0.1]})],
     level="intermediate")


# ================================================================================================ BATCH 1 (2026-11-11)
# pH and pOH, Henry's law (solubility form), channel velocity, landfill compaction, retardation, Darcy's law, chronic
# daily intake and risk, mean cell residence time from solids masses, contact time from a required CT, filter flow
# from the loading rate, stoichiometric combustion oxygen, atmospheric stability from the lapse rate.
KW_25C = 1e-14                    # ion product of water at 25 degC, (mol/L)^2
GAMMA_DRY = 9.8                   # dry adiabatic lapse rate, degC/km


def ph_poh(H=None, OH=None):
    _need((H is None) != (OH is None), "give either the hydrogen ion concentration H or the hydroxide concentration OH, not both")
    c = H if H is not None else OH
    _pos("the ion concentration", c)
    m = c / 1000.0                                          # mol/m^3 -> mol/L
    _need(1e-14 * (1.0 - 1e-9) <= m <= 1.0 * (1.0 + 1e-12), "the ion concentration must lie between 1e-14 and 1 mol/L (dilute solution at 25 degC)")
    p = -math.log10(m)
    other = KW_25C / m
    if H is not None:
        return _out(pH=p, pOH=14.0 - p, H_molL=m, OH_molL=other)
    return _out(pH=14.0 - p, pOH=p, H_molL=other, OH_molL=m)


def _ref_ph_poh(H=None, OH=None):
    import mpmath as mp
    mp.mp.dps = 40
    c = mp.mpf(H if H is not None else OH) / 1000
    o = mp.mpf("1e-14") / c                                 # the other ion from Kw, its p-value from its own logarithm
    pc, po = -mp.log10(c), -mp.log10(o)
    if H is not None:
        return {"pH": float(pc), "pOH": float(po), "H_molL": float(c), "OH_molL": float(o)}
    return {"pH": float(po), "pOH": float(pc), "H_molL": float(o), "OH_molL": float(c)}


def _wrong_ph_poh(H=None, OH=None):
    c = (H if H is not None else OH) / 1000.0
    p = -math.log(c)                                        # natural logarithm instead of log10
    return _out(pH=p, pOH=14.0 - p, H_molL=c, OH_molL=KW_25C / c) if H is not None else _out(pH=14.0 - p, pOH=p, H_molL=KW_25C / c, OH_molL=c)


def _rand_ph(rng):
    v = 10.0 ** (-rng.uniform(0.3, 13.7)) * 1000.0
    return {"H": _sig(v, 3)} if rng.random() < 0.5 else {"OH": _sig(v, 3)}


_add("enveng_ph_poh",
     name="pH and pOH from the hydrogen or hydroxide ion concentration", topic="water quality / acids and bases",
     purpose="The pH, the pOH and both ion concentrations of a water sample at 25 degC from either its hydrogen ion concentration [H+] or its hydroxide ion concentration [OH-] (pH = -log10[H+], pOH = -log10[OH-], pH + pOH = 14).",
     inputs=[_phys("H", "molar_concentration", "hydrogen ion concentration [H+]", required=False, show=("mol/L", 1e-3)),
             _phys("OH", "molar_concentration", "hydroxide ion concentration [OH-]", required=False, show=("mol/L", 1e-3))],
     outputs=[_o("pH", "number", "pH = -log10[H+]"), _o("pOH", "number", "pOH = -log10[OH-] = 14 - pH"),
              _o("H_molL", "number", "hydrogen ion concentration, mol/L"), _o("OH_molL", "number", "hydroxide ion concentration, mol/L")],
     primary="pH", conditions=["25 degC: Kw = 1e-14, pH + pOH = 14", "exactly one of H, OH", "1e-14 <= concentration <= 1 mol/L (dilute: activity = concentration)"],
     method="pH = -log10[H+]; [OH-] = Kw/[H+]; pOH = 14 - pH (or from [OH-] the other way round)",
     latex=r"\mathrm{pH} = -\log_{10}[\mathrm{H^+}],\quad \mathrm{pOH} = -\log_{10}[\mathrm{OH^-}],\quad \mathrm{pH} + \mathrm{pOH} = 14",
     run=ph_poh, reference=_ref_ph_poh, wrong=_wrong_ph_poh, rand=_rand_ph,
     phrases=("pH and pOH", "pH of a sample", "pH from ion concentration", "Calculate the pH"),
     aliases={"H3O": "H", "Hplus": "H", "OHminus": "OH"},
     examples=[_ex({"OH": 1.0e-5 * 1000.0}), _ex({"H": 2.5e-4 * 1000.0})],
     level="beginner",
     extra_q=[{"q": "pH of a sample: OH = 1.0e-5 mol/L", "like": 0},
              {"q": "pH and pOH: H = 2 mol/L", "outcome": "INVALID_INPUT"},
              {"q": "pH and pOH: H = 1e-7 mol/L, OH = 1e-7 mol/L", "outcome": "INVALID_INPUT"},
              {"q": "pH and pOH: H = -0.001 mol/L", "outcome": "INVALID_INPUT"}])


def henry_solubility(KH, P, M=None):
    _pos("KH and P", KH, P)
    C = KH * P
    out = {"C": C, "C_molL": C / 1000.0}
    if M is not None:
        _pos("the molar mass M", M)
        out["C_mass"] = C * M
        out["C_mgL"] = C * M * 1e3
    return _out(**out)


def _ref_henry_solubility(KH, P, M=None):
    from chemicals import Henry_converter as hc
    # the solubility constant converted to the volatility form by the library, then C = P/Hpc
    hpc = hc(KH * P_ATM / 1000.0, "M/atm", "m^3*Pa/mol", rhom=55344.59, MW=18.01528)
    C = P / hpc
    out = {"C": C, "C_molL": C / 1000.0}
    if M is not None:
        out["C_mass"] = C * M
        out["C_mgL"] = C * M * 1e3
    return out


_add("enveng_henry_law_solubility",
     name="Dissolved gas concentration from Henry's solubility constant (C = KH P)", topic="gas transfer / Henry's law",
     purpose="The equilibrium concentration of a gas dissolved in water from its partial pressure and Henry's solubility constant KH in mol/(L atm), as mol/L and, with the gas molar mass, as mg/L.",
     inputs=[_phys("KH", "henry_constant", "Henry's solubility constant", show=("mol/(L*atm)", P_ATM / 1e3)),
             _phys("P", "pressure", "partial pressure of the gas above the water", show=("atm", 1.0 / P_ATM)),
             _phys("M", "molar_mass", "molar mass of the gas (for mg/L)", required=False, show=("g/mol", 1e3))],
     outputs=[_o("C", "molar_concentration", "dissolved concentration KH P"), _o("C_molL", "number", "in mol/L"),
              _o("C_mass", "mass_concentration", "dissolved mass concentration"), _o("C_mgL", "number", "in mg/L")],
     primary="C", conditions=["KH, P > 0", "dilute solution at equilibrium, KH at the water temperature"],
     method="C = KH P; mass concentration C M", latex=r"C = K_H\,P,\qquad C_m = K_H\,P\,M",
     run=henry_solubility, reference=_ref_henry_solubility,
     wrong=lambda KH, P, M=None: (lambda C: _out(**({"C": C, "C_molL": C / 1000.0} if M is None else {"C": C, "C_molL": C / 1000.0, "C_mass": C * M, "C_mgL": C * M * 1e3})))(P / KH),
     rand=lambda rng: dict({"KH": _sig(_lu(rng, 1e-4, 1.0), 3) * 1e3 / P_ATM, "P": _sig(_lu(rng, 1e-4, 5.0), 3) * P_ATM},
                           **({"M": _sig(rng.uniform(2.0, 160.0), 4) * 1e-3} if rng.random() < 0.6 else {})),
     phrases=("Henry's law solubility", "Dissolved gas concentration from KH", "Henry's law C = KH P", "Henry's law"),
     aliases={"Kh": "KH", "p": "P"},
     examples=[_ex({"KH": 1.3e-3 * 1e3 / P_ATM, "P": 0.21 * P_ATM, "M": 0.032}), _ex({"KH": 3.4e-2 * 1e3 / P_ATM, "P": 4.0e-4 * P_ATM})],
     level="beginner",
     extra_q=[{"q": "Henry's law: KH = 0.0013 mol/(L*atm), P = 0.21 atm, M = 32 g/mol", "like": 0},
              {"q": "Henry's law solubility: KH = 0.0013 mol/(L*atm), P = -0.21 atm", "outcome": "INVALID_INPUT"}])


def channel_velocity(W, d, Q=None, v=None):
    _need((Q is None) != (v is None), "give either the flow Q or the velocity v, not both")
    _pos("the width W and the depth d", W, d)
    A = W * d
    if Q is not None:
        _pos("the flow Q", Q)
        return _out(A=A, v=Q / A, Q=Q)
    _pos("the velocity v", v)
    return _out(A=A, v=v, Q=v * A)


def _ref_channel_velocity(W, d, Q=None, v=None):
    from scipy.integrate import quad
    A = _quad(lambda z: W, 0.0, d)                          # the wetted section integrated over the depth
    if Q is not None:
        vv = _brentq(lambda u: quad(lambda z: u * W, 0.0, d)[0] - Q, 0.0, 2.0 * Q / A + 1.0)   # uniform velocity carrying Q
        return {"A": A, "v": vv, "Q": Q}
    return {"A": A, "v": v, "Q": quad(lambda z: v * W, 0.0, d, epsrel=1e-13)[0]}


_add("enveng_channel_flow_velocity",
     name="Flow velocity in a rectangular channel (continuity, grit chamber)", topic="water treatment / channel hydraulics",
     purpose="The mean (horizontal) velocity of flow in a rectangular channel or grit chamber from the flow, the width and the water depth (v = Q/(W d)), or the flow from the velocity (Q = A v).",
     inputs=[_phys("W", "length", "channel width"), _phys("d", "length", "water depth"),
             _phys("Q", "flow_rate", "flow rate", required=False), _phys("v", "speed", "mean flow velocity", required=False)],
     outputs=[_o("A", "area", "flow cross-section W d"), _o("v", "speed", "mean (horizontal) velocity Q/A"), _o("Q", "flow_rate", "flow rate A v")],
     primary="v", conditions=["W, d > 0", "exactly one of Q, v, positive", "rectangular section, uniform mean velocity"],
     method="A = W d; v = Q/A (or Q = A v)", latex=r"v = \frac{Q}{A} = \frac{Q}{W\,d},\qquad Q = A\,v",
     run=channel_velocity, reference=_ref_channel_velocity,
     wrong=lambda W, d, Q=None, v=None: (lambda A: _out(A=A, v=Q / A, Q=Q) if Q is not None else _out(A=A, v=v, Q=v * A))(W * d / 2.0),
     rand=lambda rng: dict({"W": _sig(rng.uniform(0.3, 6.0), 3), "d": _sig(rng.uniform(0.2, 3.0), 3)},
                           **({"Q": _sig(_lu(rng, 0.01, 5.0), 3)} if rng.random() < 0.6 else {"v": _sig(rng.uniform(0.1, 1.5), 3)})),
     phrases=("Flow velocity in a channel", "Horizontal velocity of a grit chamber", "Grit chamber velocity", "Channel flow continuity"),
     examples=[_ex({"Q": 0.3, "W": 1.0, "d": 1.0}), _ex({"v": 0.25, "W": 2.0, "d": 1.5})],
     level="beginner",
     extra_q=[{"q": "Grit chamber velocity: Q = 1080 m^3/h, W = 1 m, d = 1 m", "like": 0},
              {"q": "Flow velocity in a channel: Q = 0.3 m^3/s, W = 0 m, d = 1 m", "outcome": "INVALID_INPUT"},
              {"q": "Flow velocity in a channel: Q = 0.3 m^3/s, v = 0.3 m/s, W = 1 m, d = 1 m", "outcome": "INVALID_INPUT"}])


def compaction_ratio(rho_i=None, rho_c=None, V_i=None, V_c=None):
    dens = rho_i is not None or rho_c is not None
    vols = V_i is not None or V_c is not None
    _need(dens != vols, "give either the two densities (rho_i, rho_c) or the two volumes (V_i, V_c)")
    if dens:
        _need(rho_i is not None and rho_c is not None, "give both densities, before (rho_i) and after (rho_c) compaction")
        _pos("the densities", rho_i, rho_c)
        CR = rho_c / rho_i
    else:
        _need(V_i is not None and V_c is not None, "give both volumes, before (V_i) and after (V_c) compaction")
        _pos("the volumes", V_i, V_c)
        CR = V_i / V_c
    _need(CR >= 1.0, "compaction makes the waste denser: the compacted density must be at least the initial one")
    red = 1.0 - 1.0 / CR
    return _out(CR=CR, volume_reduction=red, volume_reduction_percent=red * 100.0)


def _ref_compaction_ratio(rho_i=None, rho_c=None, V_i=None, V_c=None):
    import numpy as np
    if rho_i is not None:                                   # one kilogram of waste before and after: specific volumes
        v = np.array([1.0 / rho_i, 1.0 / rho_c])
    else:
        v = np.array([V_i, V_c], float)
    CR = float(v[0] / v[1])
    red = float((v[0] - v[1]) / v[0])
    return {"CR": CR, "volume_reduction": red, "volume_reduction_percent": red * 100.0}


_add("enveng_landfill_compaction_ratio",
     name="Compaction ratio of solid waste", topic="solid waste / landfill compaction",
     purpose="The compaction ratio of solid waste (compacted over initial density, or initial over compacted volume) and the volume reduction it gives.",
     inputs=[_phys("rho_i", "density", "density as delivered (loose)", required=False), _phys("rho_c", "density", "density after compaction", required=False),
             _phys("V_i", "volume", "volume before compaction", required=False), _phys("V_c", "volume", "volume after compaction", required=False)],
     outputs=[_o("CR", "number", "compaction ratio"), _o("volume_reduction", "number", "fraction of the volume removed 1 - 1/CR"),
              _o("volume_reduction_percent", "number", "volume reduction in per cent")],
     primary="CR", conditions=["two densities or two volumes", "the same mass of waste before and after", "CR >= 1"],
     method="CR = rho_c/rho_i = V_i/V_c (mass conserved); reduction = 1 - V_c/V_i",
     latex=r"CR = \frac{\rho_c}{\rho_i} = \frac{V_i}{V_c},\qquad \Delta V = 1 - \frac{1}{CR}",
     run=compaction_ratio, reference=_ref_compaction_ratio,
     wrong=lambda rho_i=None, rho_c=None, V_i=None, V_c=None: (lambda CR: _out(CR=CR, volume_reduction=1.0 - CR, volume_reduction_percent=(1.0 - CR) * 100.0))(
         rho_i / rho_c if rho_i is not None else V_c / V_i),
     rand=lambda rng: (lambda a, cr: {"rho_i": a, "rho_c": _sig(a * cr, 3)} if rng.random() < 0.5 else {"V_i": _sig(a * 10.0, 3), "V_c": _sig(a * 10.0 / cr, 3)})(
         _sig(rng.uniform(80.0, 300.0), 3), rng.uniform(1.5, 8.0)),
     phrases=("Compaction ratio", "Landfill compaction ratio", "Waste compaction ratio"),
     examples=[_ex({"rho_i": 120.0, "rho_c": 600.0}), _ex({"V_i": 1000.0, "V_c": 250.0})],
     level="beginner",
     extra_q=[{"q": "Compaction ratio: rho_i = 600 kg/m^3, rho_c = 120 kg/m^3", "outcome": "INVALID_INPUT"},
              {"q": "Compaction ratio: rho_i = 120 kg/m^3, V_c = 250 m^3", "outcome": "INVALID_INPUT"}])


def retardation(rho_b=None, n=None, Kd=None, Koc=None, foc=None, R=None, v=None):
    out = {}
    if R is not None:
        _need(rho_b is None and n is None and Kd is None and Koc is None and foc is None,
              "give the retardation factor R directly or the soil properties to compute it, not both")
        _need(v is not None, "with R given, give the groundwater velocity v")
        _fin(R)
        _need(R >= 1.0, "the retardation factor R must be at least 1")
    else:
        _need(rho_b is not None and n is not None, "give the bulk density rho_b and the porosity n (or R directly)")
        _pos("the bulk density rho_b", rho_b)
        _frac("the porosity n", n, lo_open=True, hi_open=True)
        if Kd is not None:
            _need(Koc is None and foc is None, "give Kd, or Koc with foc, not both")
            _nonneg("Kd", Kd)
            kd = Kd
        else:
            _need(Koc is not None and foc is not None, "give the distribution coefficient Kd, or Koc and foc")
            _nonneg("Koc", Koc)
            _frac("the organic carbon fraction foc", foc)
            kd = Koc * foc
            out["Kd"] = kd
            out["Kd_L_per_kg"] = kd * 1e3
        R = 1.0 + rho_b * kd / n
    out["R"] = R
    if v is not None:
        _pos("the groundwater velocity v", v)
        out["vc"] = v / R
        out["vc_m_per_day"] = v / R * DAY
        out["v_m_per_day"] = v * DAY
    return _out(**out)


def _ref_retardation(rho_b=None, n=None, Kd=None, Koc=None, foc=None, R=None, v=None):
    import numpy as np
    out = {}
    if R is None:
        kd = float(np.prod([Koc, foc])) if Kd is None else Kd
        if Kd is None:
            out["Kd"] = kd
            out["Kd_L_per_kg"] = kd * 1e3
        # per unit volume of aquifer at unit dissolved concentration: dissolved n, sorbed rho_b Kd; R = total / dissolved
        dissolved, sorbed = n, rho_b * kd
        R = float(np.sum([dissolved, sorbed]) / dissolved)
        frac_mobile = dissolved / (dissolved + sorbed)
    else:
        frac_mobile = 1.0 / R
    out["R"] = R
    if v is not None:
        out["vc"] = v * frac_mobile                         # the contaminant moves only while dissolved
        out["vc_m_per_day"] = out["vc"] * 86400.0
        out["v_m_per_day"] = v * 86400.0
    return out


def _wrong_retardation(rho_b=None, n=None, Kd=None, Koc=None, foc=None, R=None, v=None):
    out = {}
    if R is None:
        kd = Kd if Kd is not None else Koc * foc
        R = 1.0 + rho_b * kd * n                            # porosity multiplied instead of divided
        if Kd is None:
            out.update(Kd=kd, Kd_L_per_kg=kd * 1e3)
    out["R"] = R
    if v is not None:
        out.update(vc=v * R, vc_m_per_day=v * R * DAY, v_m_per_day=v * DAY)   # multiplied by R instead of divided
    return _out(**out)


def _rand_retardation(rng):
    x = {}
    if rng.random() < 0.2:
        x["R"] = _sig(rng.uniform(1.0, 30.0), 3)
        x["v"] = _sig(rng.uniform(0.01, 2.0), 3) / DAY
        return x
    x["rho_b"] = _sig(rng.uniform(1.3, 2.1), 3) * 1000.0
    x["n"] = _sig(rng.uniform(0.15, 0.5), 2)
    if rng.random() < 0.5:
        x["Kd"] = _sig(_lu(rng, 0.05, 50.0), 3) * 1e-3
    else:
        x["Koc"] = _sig(_lu(rng, 10.0, 5000.0), 3) * 1e-3
        x["foc"] = _sig(_lu(rng, 0.0005, 0.05), 2)
    if rng.random() < 0.7:
        x["v"] = _sig(rng.uniform(0.01, 2.0), 3) / DAY
    return x


_add("enveng_retardation_factor",
     name="Retardation factor and contaminant velocity in groundwater", topic="groundwater / contaminant transport",
     purpose="The retardation factor of a sorbing contaminant in an aquifer, R = 1 + rho_b Kd / n (Kd given or Koc foc), and the velocity at which the contaminant migrates, v_c = v / R (R may also be given directly).",
     inputs=[_phys("rho_b", "density", "dry bulk density of the aquifer material", required=False, show=("g/cm^3", 1e-3)),
             _numin("n", "porosity (fraction or %)", required=False, pct=True),
             _phys("Kd", "specific_volume", "distribution coefficient", required=False, show=("L/kg", 1e3)),
             _phys("Koc", "specific_volume", "organic-carbon partition coefficient", required=False, show=("L/kg", 1e3)),
             _numin("foc", "fraction of organic carbon (fraction or %)", required=False, pct=True),
             _numin("R", "retardation factor, if already known", required=False),
             _phys("v", "speed", "groundwater (seepage) velocity", required=False, show=("m/d", DAY))],
     outputs=[_o("R", "number", "retardation factor"), _o("vc", "speed", "contaminant migration velocity v/R"),
              _o("vc_m_per_day", "number", "contaminant velocity, m/d"), _o("v_m_per_day", "number", "groundwater velocity, m/d"),
              _o("Kd", "specific_volume", "distribution coefficient Koc foc"), _o("Kd_L_per_kg", "number", "Kd in L/kg")],
     primary="R", conditions=["linear, reversible, instantaneous sorption", "0 < n < 1", "R >= 1"],
     method="Kd = Koc foc; R = 1 + rho_b Kd / n; v_c = v / R",
     latex=r"R = 1 + \frac{\rho_b K_d}{n},\quad K_d = K_{oc}f_{oc},\quad v_c = \frac{v}{R}",
     run=retardation, reference=_ref_retardation, wrong=_wrong_retardation, rand=_rand_retardation,
     phrases=("Retardation factor", "Contaminant migration velocity", "Migration velocity of the contaminant", "Contaminant velocity in groundwater"),
     examples=[_ex({"rho_b": 1600.0, "n": 0.3, "Koc": 0.1, "foc": 0.01, "v": 0.5 / DAY}), _ex({"R": 4.0, "v": 1.2 / DAY})],
     level="intermediate",
     extra_q=[{"q": "Retardation factor: rho_b = 1.8 g/cm^3, n = 0.35, Kd = 0.5 L/kg",
               "outputs": {"R": 1.0 + 1800.0 * 0.0005 / 0.35}},
              {"q": "Migration velocity of the contaminant: rho_b = 1.6 g/cm^3, n = 30%, Koc = 100 L/kg, foc = 1%, v = 0.5 m/d", "like": 0},
              {"q": "Retardation factor: rho_b = 1.6 g/cm^3, n = 120%, Kd = 1 L/kg", "outcome": "INVALID_INPUT"},
              {"q": "Retardation factor: R = 0.5, v = 1 m/d", "outcome": "INVALID_INPUT"}])


def darcy(K, i=None, dh=None, L=None, A=None, n_e=None):
    _pos("the hydraulic conductivity K", K)
    if i is None:
        _need(dh is not None and L is not None, "give the hydraulic gradient i, or the head difference dh and the flow length L")
        _pos("dh and L", dh, L)
        i = dh / L
    else:
        _need(dh is None and L is None, "give the gradient i or dh and L, not both")
        _pos("the hydraulic gradient i", i)
    q = K * i
    out = {"i": i, "q": q, "q_m_per_day": q * DAY}
    if A is not None:
        _pos("the flow area A", A)
        out["Q"] = q * A
        out["Q_m3_per_day"] = q * A * DAY
    if n_e is not None:
        _frac("the effective porosity n_e", n_e, lo_open=True, hi_open=True)
        out["v_s"] = q / n_e
        out["v_s_m_per_day"] = q / n_e * DAY
    return _out(**out)


def _ref_darcy(K, i=None, dh=None, L=None, A=None, n_e=None):
    # head along the flow path, dh/dx = -q/K, integrated for a unit flux; the flux that gives the stated head drop
    Lx = L if i is None else 1.0
    drop = dh if i is None else i
    h_end = _ivp(lambda x, y: [-1.0 / K], Lx, [0.0], atol=[1e-20])[0]
    q = drop / -h_end
    out = {"i": drop / Lx, "q": q, "q_m_per_day": q * 86400.0}
    if A is not None:
        out["Q"] = q * A
        out["Q_m3_per_day"] = q * A * 86400.0
    if n_e is not None:
        out["v_s"] = q / n_e
        out["v_s_m_per_day"] = q / n_e * 86400.0
    return out


def _wrong_darcy(K, i=None, dh=None, L=None, A=None, n_e=None):
    i = i if i is not None else dh / L
    q = K / i
    out = {"i": i, "q": q, "q_m_per_day": q * DAY}
    if A is not None:
        out.update(Q=q * A, Q_m3_per_day=q * A * DAY)
    if n_e is not None:
        out.update(v_s=q * n_e, v_s_m_per_day=q * n_e * DAY)
    return _out(**out)


def _rand_darcy(rng):
    x = {"K": _sig(_lu(rng, 1e-3, 100.0), 3) / DAY}
    if rng.random() < 0.5:
        x["i"] = _sig(_lu(rng, 1e-4, 0.05), 3)
    else:
        x["dh"] = _sig(rng.uniform(0.1, 20.0), 3)
        x["L"] = _sig(rng.uniform(10.0, 2000.0), 3)
    if rng.random() < 0.6:
        x["A"] = _sig(_lu(rng, 1.0, 5000.0), 3)
    if rng.random() < 0.7:
        x["n_e"] = _sig(rng.uniform(0.05, 0.45), 2)
    return x


_add("enveng_darcy_seepage_velocity",
     name="Darcy's law: groundwater flow and seepage velocity", topic="groundwater / Darcy's law",
     purpose="Groundwater flow through an aquifer by Darcy's law: the Darcy flux q = K i, the flow Q = K i A through an area and the seepage (average linear) velocity v_s = K i / n_e; the gradient may be given as a head difference over a length.",
     inputs=[_phys("K", "speed", "hydraulic conductivity", show=("m/d", DAY)),
             _numin("i", "hydraulic gradient dh/L (dimensionless)", required=False),
             _phys("dh", "length", "head difference along the flow path", required=False),
             _phys("L", "length", "length of the flow path", required=False),
             _phys("A", "area", "cross-sectional area of flow", required=False),
             _numin("n_e", "effective porosity (fraction or %)", required=False, pct=True)],
     outputs=[_o("i", "number", "hydraulic gradient"), _o("q", "speed", "Darcy flux (specific discharge) K i"), _o("q_m_per_day", "number", "Darcy flux, m/d"),
              _o("Q", "flow_rate", "flow K i A"), _o("Q_m3_per_day", "number", "flow, m^3/d"),
              _o("v_s", "speed", "seepage (average linear) velocity K i / n_e"), _o("v_s_m_per_day", "number", "seepage velocity, m/d")],
     primary="q", conditions=["laminar flow (Darcy's law)", "K, i > 0", "0 < n_e < 1"],
     method="i = dh/L; q = K i; Q = q A; v_s = q/n_e",
     latex=r"Q = K\,i\,A,\qquad v_s = \frac{K\,i}{n_e},\qquad i = \frac{\Delta h}{L}",
     run=darcy, reference=_ref_darcy, wrong=_wrong_darcy, rand=_rand_darcy,
     phrases=("Darcy's law", "Darcy flow", "Seepage velocity", "Groundwater seepage velocity"),
     examples=[_ex({"K": 10.0 / DAY, "i": 0.002, "A": 500.0, "n_e": 0.25}), _ex({"K": 1e-4, "dh": 2.0, "L": 400.0, "n_e": 0.3})],
     level="beginner",
     extra_q=[{"q": "Seepage velocity: K = 1e-4 m/s, dh = 2 m, L = 400 m, n_e = 30%", "like": 1},
              {"q": "Darcy's law: K = 10 m/d, i = 0.002, n_e = 150%", "outcome": "INVALID_INPUT"},
              {"q": "Darcy's law: K = 10 m/d, i = 0.002, dh = 2 m, L = 400 m", "outcome": "INVALID_INPUT"}])


_DOSE = ("mg/kg/d", "mg/(kg*d)", "mg/kg-day", "mg/kg/day", "mg/(kg*day)", "mg/kg-d")
_SLOPE = ("(mg/kg/d)^-1", "(mg/kg-day)^-1", "(mg/(kg*d))^-1", "kg*d/mg", "kg*day/mg", "(mg/kg/day)^-1", "(mg/kg-d)^-1")


def chronic_intake(C, IR, BW, EF=None, ED=None, AT=None, RfD=None, SF=None):
    _pos("C, IR and BW", C, IR, BW)
    given = (EF is not None) + (ED is not None) + (AT is not None)
    _need(given == 0 or given == 3, "give EF, ED and AT together, or none of them (continuous lifetime exposure)")
    c = C * 1e3                                             # mg/L
    ir = IR * 1e3 * DAY                                     # L/d
    f = 1.0
    if given == 3:
        _pos("EF, ED and AT", EF, ED, AT)
        _need(EF <= 365.0, "the exposure frequency EF cannot exceed 365 days per year")
        _need(EF * ED <= AT * (1.0 + 1e-12), "the exposure days EF x ED cannot exceed the averaging time AT")
        f = EF * ED / AT
    cdi = c * ir * f / BW
    out = {"CDI": cdi}
    if RfD is not None:
        _pos("the reference dose RfD", RfD)
        out["HQ"] = cdi / RfD
    if SF is not None:
        _pos("the slope factor SF", SF)
        risk = cdi * SF
        _need(risk <= 0.01, "the linear low-dose risk model holds only for a risk up to 0.01")
        out["risk"] = risk
    return _out(**out)


def _ref_chronic_intake(C, IR, BW, EF=None, ED=None, AT=None, RfD=None, SF=None):
    from scipy.integrate import quad
    dose_rate = (C * 1e3) * (IR * 1e3 * 86400.0) / BW       # mg/(kg d) while exposed
    if EF is None:
        exposed, at = 1.0, 1.0
    else:
        exposed, at = EF * ED, AT                           # exposure days within the averaging period (days)
    # the dose accumulated over the averaging period, divided by its length
    total = quad(lambda t: dose_rate, 0.0, exposed, epsabs=0.0, epsrel=1e-13)[0]
    cdi = total / at
    out = {"CDI": cdi}
    if RfD is not None:
        out["HQ"] = cdi / RfD
    if SF is not None:
        out["risk"] = cdi * SF
    return out


def _wrong_chronic_intake(C, IR, BW, EF=None, ED=None, AT=None, RfD=None, SF=None):
    f = 1.0 if EF is None else EF * ED / AT
    cdi = C * 1e3 * IR * 1e3 * DAY * f                      # body weight forgotten
    out = {"CDI": cdi}
    if RfD is not None:
        out["HQ"] = cdi / RfD
    if SF is not None:
        out["risk"] = cdi * SF
    return _out(**out)


def _rand_cdi(rng):
    x = {"C": _sig(_lu(rng, 1e-4, 1.0), 3) * MG, "IR": _sig(rng.uniform(0.5, 3.0), 2) * 1e-3 / DAY, "BW": _sig(rng.uniform(10.0, 90.0), 3)}
    if rng.random() < 0.6:
        x["ED"] = float(rng.randint(1, 70))
        x["EF"] = float(rng.choice([350, 365, 250, 180]))
        x["AT"] = rng.choice([70.0 * 365.0, x["ED"] * 365.0])
    if rng.random() < 0.6:
        x["RfD"] = _sig(_lu(rng, 1e-4, 0.5), 2)
    if rng.random() < 0.5:
        x["SF"] = _sig(_lu(rng, 1e-3, 0.5), 2)
    return x


_add("enveng_chronic_daily_intake_risk",
     name="Chronic daily intake, hazard quotient and cancer risk", topic="risk assessment / exposure",
     purpose="The chronic daily intake of a contaminant in drinking water, CDI = C IR EF ED / (BW AT) (a continuous lifetime exposure, CDI = C IR / BW, when EF, ED and AT are not given), with the hazard quotient CDI/RfD and the lifetime cancer risk CDI SF when the reference dose or slope factor is given.",
     inputs=[_phys("C", "mass_concentration", "concentration in the water"),
             _phys("IR", "flow_rate", "intake (ingestion) rate", show=("L/d", 1e3 * DAY)),
             _phys("BW", "mass", "body weight"),
             _numin("EF", "exposure frequency, days per year", required=False, words=("d/yr", "days/yr", "days/year", "d/y", "day/yr", "days/y")),
             _numin("ED", "exposure duration, years", required=False, words=("yr", "years", "year", "y")),
             _numin("AT", "averaging time, days", required=False, words=("d", "days", "day")),
             _numin("RfD", "reference dose, mg/(kg d)", required=False, words=_DOSE),
             _numin("SF", "cancer slope factor, (mg/(kg d))^-1", required=False, words=_SLOPE)],
     outputs=[_o("CDI", "number", "chronic daily intake, mg/(kg d)"), _o("HQ", "number", "hazard quotient CDI/RfD"),
              _o("risk", "number", "lifetime excess cancer risk CDI SF")],
     primary="CDI", conditions=["EF, ED and AT together or none (continuous lifetime)", "EF <= 365 d/yr, EF ED <= AT",
                                "ED in years, AT in days (70 yr x 365 = 25,550 d)", "linear low-dose risk up to 0.01"],
     method="CDI = C IR EF ED / (BW AT) [mg/L, L/d, d/yr, yr, kg, d]; HQ = CDI/RfD; risk = CDI SF",
     latex=r"CDI = \frac{C\cdot IR\cdot EF\cdot ED}{BW\cdot AT},\quad HQ = \frac{CDI}{RfD},\quad \text{risk} = CDI\cdot SF",
     run=chronic_intake, reference=_ref_chronic_intake, wrong=_wrong_chronic_intake, rand=_rand_cdi,
     phrases=("Chronic daily intake", "CDI", "Hazard quotient", "Cancer risk from drinking water", "Lifetime cancer risk"),
     examples=[_ex({"C": 0.05 * MG, "IR": 2.0e-3 / DAY, "BW": 70.0, "RfD": 0.02}),
               _ex({"C": 0.01 * MG, "IR": 2.0e-3 / DAY, "BW": 70.0, "EF": 350.0, "ED": 30.0, "AT": 25550.0, "SF": 1.5})],
     level="intermediate",
     extra_q=[{"q": "Hazard quotient: C = 0.05 mg/L, IR = 2 L/d, BW = 70 kg, RfD = 0.02 mg/kg/d", "like": 0},
              {"q": "Chronic daily intake: C = 0.01 mg/L, IR = 2 L/d, BW = 70 kg, EF = 350 d/yr, ED = 30 yr", "outcome": "INVALID_INPUT"},
              {"q": "Chronic daily intake: C = 0.01 mg/L, IR = 2 L/d, BW = 70 kg, EF = 400 d/yr, ED = 30 yr, AT = 25550 d", "outcome": "INVALID_INPUT"}])


def mcrt_masses(M, Mw, Me):
    _pos("the solids mass in the basin M", M)
    _nonneg("the solids wasted and lost per day", Mw, Me)
    _need(Mw + Me > 0, "some solids must leave the system (wasted or in the effluent)")
    th = M / (Mw + Me)
    return _out(theta_c=th, theta_c_days=th / DAY)


def _ref_mcrt_masses(M, Mw, Me):
    import math as _m
    # mean residence time of a solids particle in a mixed tank: the integral of the fraction still present
    k = (Mw + Me) / M
    th = _quad(lambda s: _m.exp(-s), 0.0, 60.0) / k + _quad(lambda s: _m.exp(-s), 60.0, 800.0) / k
    return {"theta_c": th, "theta_c_days": th / 86400.0}


_add("enveng_mean_cell_residence_time",
     name="Mean cell residence time (sludge age) from the solids masses", topic="wastewater / activated sludge",
     purpose="The mean cell residence time (solids retention time, sludge age) of an activated-sludge system as the mass of solids held in the basin over the mass leaving per day in the waste sludge and the effluent.",
     inputs=[_phys("M", "mass", "mass of solids (MLVSS or MLSS) in the aeration basin"),
             _phys("Mw", "mass_flow_rate", "mass of solids wasted per day", show=("kg/d", DAY)),
             _phys("Me", "mass_flow_rate", "mass of solids lost in the effluent per day", show=("kg/d", DAY))],
     outputs=[_o("theta_c", "time", "mean cell residence time"), _o("theta_c_days", "number", "in days")],
     primary="theta_c", conditions=["steady state", "M > 0, Mw + Me > 0"],
     method="theta_c = M / (Mw + Me)", latex=r"\theta_c = \frac{M_{basin}}{\dot M_{w} + \dot M_{e}}",
     run=mcrt_masses, reference=_ref_mcrt_masses,
     wrong=lambda M, Mw, Me: _out(theta_c=M / Mw, theta_c_days=M / Mw / DAY),
     rand=lambda rng: {"M": _sig(_lu(rng, 200.0, 50000.0), 3), "Mw": _sig(_lu(rng, 20.0, 5000.0), 3) / DAY, "Me": _sig(_lu(rng, 1.0, 500.0), 3) / DAY},
     phrases=("Mean cell residence time", "Sludge age from solids masses", "MCRT", "Mean cell residence time from solids masses"),
     examples=[_ex({"M": 3000.0, "Mw": 300.0 / DAY, "Me": 20.0 / DAY}), _ex({"M": 12000.0, "Mw": 1500.0 / DAY, "Me": 100.0 / DAY})],
     level="beginner",
     extra_q=[{"q": "MCRT: M = 3 t, Mw = 300 kg/d, Me = 20 kg/d", "like": 0},
              {"q": "Mean cell residence time: M = 3000 kg, Mw = 0 kg/d, Me = 0 kg/d", "outcome": "INVALID_INPUT"}])


def ct_contact_time(CT, C):
    _pos("the required CT and the residual C", CT, C)
    t = CT / C
    return _out(t=t, t_min=t / 60.0, t_h=t / HOUR)


def _ref_ct_contact_time(CT, C):
    from scipy.integrate import quad
    # the time at which the accumulated product of residual and time reaches the required CT
    t = _brentq(lambda s: quad(lambda u: C, 0.0, s, epsrel=1e-14)[0] - CT, 0.0, 2.0 * CT / C + 1.0)
    return {"t": t, "t_min": t / 60.0, "t_h": t / 3600.0}


_add("enveng_contact_time_from_ct",
     name="Minimum disinfection contact time from a required CT", topic="water treatment / disinfection",
     purpose="The minimum contact time a disinfectant residual needs to reach a required CT value (t = CT / C).",
     inputs=[_phys("CT", "concentration_time_integral", "required CT value"), _phys("C", "mass_concentration", "disinfectant residual")],
     outputs=[_o("t", "time", "minimum contact time CT/C"), _o("t_min", "number", "in minutes"), _o("t_h", "number", "in hours")],
     primary="t", conditions=["CT, C > 0", "constant residual over the contact time"],
     method="t = CT_required / C", latex=r"t = \frac{CT_{req}}{C}",
     run=ct_contact_time, reference=_ref_ct_contact_time,
     wrong=lambda CT, C: _out(t=CT * C, t_min=CT * C / 60.0, t_h=CT * C / HOUR),
     rand=lambda rng: {"CT": _sig(_lu(rng, 2.0, 300.0), 3) * 0.06, "C": _sig(rng.uniform(0.2, 4.0), 3) * MG},
     phrases=("Contact time from CT", "Minimum contact time", "Required contact time", "Disinfection contact time"),
     examples=[_ex({"CT": 60.0 * 0.06, "C": 1.5 * MG}), _ex({"CT": 104.0 * 0.06, "C": 0.8 * MG})],
     level="beginner",
     extra_q=[{"q": "Minimum contact time: CT = 60 mg*min/L, C = 1.5 mg/L", "like": 0},
              {"q": "Minimum contact time: CT = 60 mg*min/L, C = 0 mg/L", "outcome": "INVALID_INPUT"}])


def filter_flow(v, A):
    _pos("the loading rate v and the area A", v, A)
    Q = v * A
    return _out(Q=Q, Q_m3_per_h=Q * HOUR, Q_m3_per_day=Q * DAY)


def _ref_filter_flow(v, A):
    from scipy.integrate import dblquad
    s = A ** 0.5                                            # the bed as a square of the same area
    Q = dblquad(lambda y, x: v, 0.0, s, 0.0, s, epsabs=0.0, epsrel=1e-13)[0]
    return {"Q": Q, "Q_m3_per_h": Q * 3600.0, "Q_m3_per_day": Q * 86400.0}


_add("enveng_filter_flow_from_loading",
     name="Flow through a filter from its loading rate and bed area", topic="water treatment / filtration",
     purpose="The flow a filter treats from its hydraulic (surface) loading rate and the bed area, Q = v A, in m^3/h and m^3/d.",
     inputs=[_phys("v", "speed", "filtration (hydraulic loading) rate", show=("m/d", DAY)), _phys("A", "area", "filter bed area")],
     outputs=[_o("Q", "flow_rate", "flow v A"), _o("Q_m3_per_h", "number", "in m^3/h"), _o("Q_m3_per_day", "number", "in m^3/d")],
     primary="Q", conditions=["v, A > 0"], method="Q = v A", latex=r"Q = v\,A",
     run=filter_flow, reference=_ref_filter_flow,
     wrong=lambda v, A: _out(Q=v / A, Q_m3_per_h=v / A * HOUR, Q_m3_per_day=v / A * DAY),
     rand=lambda rng: {"v": _sig(rng.uniform(60.0, 600.0), 3) / DAY, "A": _sig(_lu(rng, 5.0, 300.0), 3)},
     phrases=("Filter flow from loading rate", "Flow through a filter", "Filter flow"),
     examples=[_ex({"v": 120.0 / DAY, "A": 50.0}), _ex({"v": 240.0 / DAY, "A": 80.0})],
     level="beginner",
     extra_q=[{"q": "Flow through a filter: v = 5 m/h, A = 50 m^2", "like": 0},
              {"q": "Filter flow: v = 120 m/d, A = -50 m^2", "outcome": "INVALID_INPUT"}])


def _cho_counts(text):
    s = text.strip().replace(" ", "")
    _need(len(s) > 0, "give a fuel formula such as CH4, C2H5OH or C")
    upper, lower, digits = "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz", "0123456789"
    pos, counts = 0, {}
    while pos < len(s):
        _need(s[pos] in upper, "the formula may contain only element symbols and counts (C, H, O)")
        el = s[pos]
        pos += 1
        if pos < len(s) and s[pos] in lower:
            el = el + s[pos]
            pos += 1
        start = pos
        while pos < len(s) and s[pos] in digits:
            pos += 1
        n = int(s[start:pos]) if pos > start else 1
        _need(el in ("C", "H", "O"), "only C, H and O are allowed")
        counts[el] = counts.get(el, 0) + n
    _need(counts.get("C", 0) + counts.get("H", 0) > 0, "the fuel must contain carbon or hydrogen")
    return counts.get("C", 0), counts.get("H", 0), counts.get("O", 0)


def combustion_oxygen(formula, m=None):
    x, y, z = _cho_counts(formula)
    n = x + y / 4.0 - z / 2.0
    _need(n > 0, "the fuel needs no oxygen to burn")
    M = x * AW["C"] + y * AW["H"] + z * AW["O"]
    o2 = n * MW_O2 / M
    out = {"o2_per_mol": n, "molar_mass": M, "O2_per_kg": o2,
           "CO2_per_kg": x * (AW["C"] + 2 * AW["O"]) / M, "H2O_per_kg": y / 2.0 * (2 * AW["H"] + AW["O"]) / M}
    if m is not None:
        _pos("the fuel mass m", m)
        out["O2_mass"] = o2 * m
    return _out(**out)


def _ref_combustion_oxygen(formula, m=None):
    import numpy as np
    from chemicals import molecular_weight, simple_formula_parser
    at = simple_formula_parser(formula)
    x, y, z = (at.get(e, 0) for e in ("C", "H", "O"))
    # unknowns O2, CO2, H2O per mol fuel; element balances C, H, O
    A = np.array([[0.0, 1.0, 0.0], [0.0, 0.0, 2.0], [2.0, -2.0, -1.0]])
    o2, co2, h2o = np.linalg.solve(A, np.array([x, y, -z], float))
    M = molecular_weight(at)
    out = {"o2_per_mol": float(o2), "molar_mass": M, "O2_per_kg": float(o2) * molecular_weight({"O": 2}) / M,
           "CO2_per_kg": float(co2) * molecular_weight({"C": 1, "O": 2}) / M, "H2O_per_kg": float(h2o) * molecular_weight({"H": 2, "O": 1}) / M}
    if m is not None:
        out["O2_mass"] = out["O2_per_kg"] * m
    return out


def _wrong_combustion_oxygen(formula, m=None):
    x, y, z = _cho_counts(formula)
    n = x + y / 2.0 - z / 2.0                               # hydrogen burned as if to H2O2
    M = x * AW["C"] + y * AW["H"] + z * AW["O"]
    out = {"o2_per_mol": n, "molar_mass": M, "O2_per_kg": n * MW_O2 / M,
           "CO2_per_kg": x * (AW["C"] + 2 * AW["O"]) / M, "H2O_per_kg": y / 2.0 * (2 * AW["H"] + AW["O"]) / M}
    if m is not None:
        out["O2_mass"] = n * MW_O2 / M * m
    return _out(**out)


_FUELS = ["CH4", "C2H6", "C3H8", "C4H10", "C8H18", "C2H5OH", "CH3OH", "C6H12O6", "C7H16", "C2H4", "C6H6", "C10H22",
          "C", "C16H34", "C3H6O", "H2", "C12H22O11", "C6H10O5"]

_add("enveng_combustion_oxygen",
     name="Stoichiometric oxygen for complete combustion of a fuel", topic="air pollution / combustion",
     purpose="The oxygen needed to burn a fuel CxHyOz (or pure carbon) completely to CO2 and water, per mole and per kilogram of fuel, the CO2 and water formed per kilogram, and the total oxygen for a given mass of fuel.",
     inputs=[_textin("formula", "fuel formula with C, H, O only (e.g. CH4, C2H5OH, C)"),
             _phys("m", "mass", "mass of fuel burned (for the total oxygen)", required=False)],
     outputs=[_o("O2_per_kg", "number", "kg O2 per kg fuel"), _o("o2_per_mol", "number", "mol O2 per mol fuel x + y/4 - z/2"),
              _o("molar_mass", "number", "molar mass of the fuel, g/mol"), _o("CO2_per_kg", "number", "kg CO2 formed per kg fuel"),
              _o("H2O_per_kg", "number", "kg water formed per kg fuel"), _o("O2_mass", "mass", "oxygen for the given mass of fuel")],
     primary="O2_per_kg", conditions=["complete combustion to CO2 and H2O", "elements C, H, O only"],
     method="CxHyOz + (x + y/4 - z/2) O2 -> x CO2 + y/2 H2O; O2/fuel = (x + y/4 - z/2) M_O2 / M_fuel",
     latex=r"\mathrm{C_xH_yO_z} + \left(x + \frac y4 - \frac z2\right)\mathrm{O_2} \rightarrow x\,\mathrm{CO_2} + \frac y2\,\mathrm{H_2O}",
     run=combustion_oxygen, reference=_ref_combustion_oxygen, wrong=_wrong_combustion_oxygen,
     rand=lambda rng: dict({"formula": rng.choice(_FUELS)}, **({"m": _sig(_lu(rng, 0.1, 1e5), 3)} if rng.random() < 0.5 else {})),
     phrases=("Stoichiometric oxygen for combustion", "Oxygen for complete combustion", "Combustion oxygen requirement"),
     examples=[_ex({"formula": "C", "m": 1000.0}), _ex({"formula": "CH4"})],
     level="intermediate",
     extra_q=[{"q": "Oxygen for complete combustion: formula = C2H5OH",
               "outputs": {"o2_per_mol": 3.0}},
              {"q": "Oxygen for complete combustion: formula = CH4N", "outcome": "INVALID_INPUT"},
              {"q": "Oxygen for complete combustion: formula = CO2", "outcome": "INVALID_INPUT"}])


def lapse_stability(T1, z1, T2, z2):
    _pos("the temperatures (absolute)", T1, T2)
    _fin(z1, z2)
    _need(z2 > z1, "the second height z2 must be above the first z1")
    elr = -(T2 - T1) / (z2 - z1)                            # K/m, positive when the air cools with height
    km = elr * 1000.0
    if abs(km - GAMMA_DRY) <= 1e-6:
        s = "neutral"
    elif km > GAMMA_DRY:
        s = "superadiabatic (unstable)"
    elif km >= 0.0:
        s = "subadiabatic (stable)"
    else:
        s = "inversion (very stable)"
    return _out(ELR=elr, ELR_C_per_km=km, stability=s)


def _ref_lapse_stability(T1, z1, T2, z2):
    import bisect as _bs
    from scipy.stats import linregress
    slope = float(linregress([z1, z2], [T1, T2]).slope)     # K/m
    elr = -slope
    km = elr * 1000.0
    if abs(km - 9.8) <= 1e-6:
        s = "neutral"
    else:
        s = ["inversion (very stable)", "subadiabatic (stable)", "superadiabatic (unstable)"][_bs.bisect_right([0.0, 9.8], km)]
    return {"ELR": elr, "ELR_C_per_km": km, "stability": s}


def _rand_lapse(rng):
    T1 = _sig(T0C + rng.uniform(-10.0, 35.0), 5)
    z1 = rng.choice([0.0, 0.0, 2.0, 10.0, 100.0])
    dz = _sig(rng.uniform(50.0, 2000.0), 3)
    km = rng.choice([rng.uniform(-15.0, 0.0), rng.uniform(0.0, 9.7), rng.uniform(9.9, 30.0)])
    return {"T1": T1, "z1": z1, "T2": _sig(T1 - km * dz / 1000.0, 6), "z2": z1 + dz}


_add("enveng_lapse_rate_stability",
     name="Atmospheric stability from the environmental lapse rate", topic="air pollution / atmospheric stability",
     purpose="The environmental lapse rate from the air temperature at two heights and the stability it gives against the dry adiabatic lapse rate (9.8 degC/km): superadiabatic (unstable), neutral, subadiabatic (stable) or an inversion.",
     inputs=[_phys("T1", "temperature", "air temperature at the lower height", show=("degC", None)), _phys("z1", "length", "lower height"),
             _phys("T2", "temperature", "air temperature at the upper height", show=("degC", None)), _phys("z2", "length", "upper height")],
     outputs=[_o("ELR", "temperature_gradient", "environmental lapse rate -dT/dz"), _o("ELR_C_per_km", "number", "lapse rate, degC/km"),
              _o("stability", "number", "stability: superadiabatic (unstable), neutral, subadiabatic (stable) or inversion (very stable)")],
     primary="ELR", conditions=["dry air (dry adiabatic lapse rate 9.8 degC/km)", "z2 > z1", "linear temperature profile between the two heights"],
     method="ELR = -(T2 - T1)/(z2 - z1); ELR > 9.8 degC/km superadiabatic, = 9.8 neutral, 0 to 9.8 subadiabatic, < 0 inversion",
     latex=r"\Gamma_{env} = -\frac{T_2 - T_1}{z_2 - z_1}\ \text{vs}\ \Gamma_d = 9.8\ ^\circ\mathrm{C/km}",
     run=lapse_stability, reference=_ref_lapse_stability,
     wrong=lambda T1, z1, T2, z2: lapse_stability(T2, z1, T1, z2),
     rand=_rand_lapse,
     phrases=("Atmospheric stability from lapse rate", "Environmental lapse rate", "Lapse rate stability"),
     examples=[_ex({"T1": T0C + 20.0, "z1": 0.0, "T2": T0C + 14.0, "z2": 500.0}), _ex({"T1": T0C + 15.0, "z1": 0.0, "T2": T0C + 17.0, "z2": 300.0})],
     level="beginner",
     extra_q=[{"q": "Environmental lapse rate: T1 = 20 degC, z1 = 0 m, T2 = 15.1 degC, z2 = 500 m",
               "outputs": {"stability": "neutral"}},
              {"q": "Environmental lapse rate: T1 = 20 degC, z1 = 0 m, T2 = 17 degC, z2 = 500 m",
               "outputs": {"stability": "subadiabatic (stable)"}},
              {"q": "Lapse rate stability: T1 = 20 degC, z1 = 500 m, T2 = 14 degC, z2 = 0 m", "outcome": "INVALID_INPUT"}])


# ================================================================================================ BATCH 2 (2026-11-11)
# total alkalinity, stirred tanks in series (and the plug-flow limit), Manning, Hazen-Williams, pump power, the rational
# method, the indoor-air box model, collection devices in series, the dry-basis heating value, return period and risk,
# geometric mean of counts, volatile solids reduction, transmembrane pressure, ThOD of a solution.
EQ_CACO3 = 50000.0                # mg of CaCO3 per equivalent (50 mg/meq, the customary rounded value)
MW_HCO3 = AW["H"] + AW["C"] + 3 * AW["O"]       # 61.01684 g/mol
MW_CO3 = AW["C"] + 3 * AW["O"]                  # 60.0089 g/mol
MW_OH = AW["O"] + AW["H"]                       # 17.00734 g/mol
RHO_WATER = 1000.0                # kg/m^3, default water density for pump power


def _species(mass, molar, M, name):
    """One ion's molar concentration in mol/L from a mass (kg/m^3 = g/L) or a molar (mol/m^3) concentration."""
    if mass is None and molar is None:
        return None
    _need(mass is None or molar is None, f"give {name} once, as a mass or as a molar concentration")
    if mass is not None:
        _nonneg(f"{name} concentration", mass)
        return mass / M
    _nonneg(f"{name} concentration", molar)
    return molar / 1000.0


def alkalinity(HCO3=None, CO3=None, OH=None, H=None, HCO3_M=None, CO3_M=None, OH_M=None, H_M=None, pH=None):
    hco3 = _species(HCO3, HCO3_M, MW_HCO3, "the bicarbonate")
    co3 = _species(CO3, CO3_M, MW_CO3, "the carbonate")
    oh = _species(OH, OH_M, MW_OH, "the hydroxide")
    h = _species(H, H_M, AW["H"], "the hydrogen ion")
    if pH is not None:
        _need(h is None, "give the hydrogen ion as pH or as a concentration, not both")
        _fin(pH)
        _need(0.0 <= pH <= 14.0, "the pH must lie between 0 and 14")
        h = 10.0 ** (-pH)
    _need(hco3 is not None or co3 is not None or oh is not None,
          "give at least one of the bicarbonate (HCO3), carbonate (CO3) or hydroxide (OH) concentrations")
    if hco3 is None:
        hco3 = 0.0
    if co3 is None:
        co3 = 0.0
    if h is None and oh is None:
        h = 0.0
        oh = 0.0
    elif oh is None:
        oh = KW_25C / h if h > 0 else 0.0
    elif h is None:
        h = KW_25C / oh if oh > 0 else 0.0
    eq = hco3 + 2.0 * co3 + oh - h
    return _out(alkalinity=eq * EQ_CACO3, alk_meq_L=eq * 1000.0, alk_eq_L=eq,
                HCO3_caco3=hco3 * EQ_CACO3, CO3_caco3=2.0 * co3 * EQ_CACO3, OH_caco3=oh * EQ_CACO3, H_caco3=h * EQ_CACO3,
                H_molL=h, OH_molL=oh)


def _ref_alkalinity(HCO3=None, CO3=None, OH=None, H=None, HCO3_M=None, CO3_M=None, OH_M=None, H_M=None, pH=None):
    import mpmath as mp
    from chemicals import molecular_weight, simple_formula_parser
    mp.mp.dps = 40
    w = lambda f: molecular_weight(simple_formula_parser(f))  # noqa: E731

    def mol(mass, molar, f):
        if mass is not None:
            return mp.mpf(mass) / mp.mpf(w(f))               # g/L over g/mol
        if molar is not None:
            return mp.mpf(molar) / 1000
        return None
    c = [mol(HCO3, HCO3_M, "HCO3"), mol(CO3, CO3_M, "CO3"), mol(OH, OH_M, "OH"), mol(H, H_M, "H")]
    if pH is not None:
        c[3] = mp.power(10, -mp.mpf(pH))
    kw = mp.mpf("1e-14")
    if c[2] is None and c[3] is None:
        c[2] = c[3] = mp.mpf(0)
    elif c[2] is None:
        c[2] = kw / c[3] if c[3] > 0 else mp.mpf(0)
    elif c[3] is None:
        c[3] = kw / c[2] if c[2] > 0 else mp.mpf(0)
    c = [mp.mpf(0) if v is None else v for v in c]
    charge = [1, 2, 1, -1]                                  # equivalents per mole, the acid side negative
    eq = float(mp.fsum([z * v for z, v in zip(charge, c)]))
    return {"alkalinity": eq * 50000.0, "alk_meq_L": eq * 1000.0, "alk_eq_L": eq,
            "HCO3_caco3": float(c[0]) * 50000.0, "CO3_caco3": float(2 * c[1]) * 50000.0, "OH_caco3": float(c[2]) * 50000.0,
            "H_caco3": float(c[3]) * 50000.0, "H_molL": float(c[3]), "OH_molL": float(c[2])}


def _wrong_alkalinity(**kw):
    r = alkalinity(**kw)
    eq = r["alk_eq_L"] - r["CO3_caco3"] / 2.0 / EQ_CACO3   # carbonate counted once (its charge forgotten)
    return _out(**dict(r, alkalinity=eq * EQ_CACO3, alk_meq_L=eq * 1000.0, alk_eq_L=eq))


def _rand_alk(rng):
    x = {}
    pick = rng.random()
    if rng.random() < 0.5:
        x["HCO3"] = _sig(rng.uniform(5.0, 400.0), 3) * MG
    else:
        x["HCO3_M"] = _sig(rng.uniform(1e-4, 6e-3), 3) * 1000.0
    if rng.random() < 0.85:
        if rng.random() < 0.5:
            x["CO3"] = _sig(rng.uniform(1.0, 120.0), 3) * MG
        else:
            x["CO3_M"] = _sig(rng.uniform(1e-5, 2e-3), 3) * 1000.0
    if pick < 0.45:
        x["pH"] = _sig(rng.uniform(6.0, 11.5), 3)
    elif pick < 0.65:
        x["OH"] = _sig(rng.uniform(0.01, 30.0), 3) * MG
    elif pick < 0.8:
        x["H_M"] = _sig(10.0 ** -rng.uniform(6.0, 11.0), 3) * 1000.0
    elif pick < 0.9:
        x["OH_M"] = _sig(10.0 ** -rng.uniform(3.0, 7.0), 3) * 1000.0
    return x


_add("enveng_total_alkalinity",
     name="Total alkalinity as CaCO3 from the carbonate species", topic="water quality / alkalinity",
     purpose="The total alkalinity of a water, [HCO3-] + 2[CO3 2-] + [OH-] - [H+] in equivalents per litre, expressed as mg/L of CaCO3 (50,000 mg/eq), from the ion concentrations in mg/L or mol/L or the pH; with one of [H+], [OH-] given, the other follows from Kw = 1e-14 (25 degC).",
     inputs=[_phys("HCO3", "mass_concentration", "bicarbonate [HCO3-] as mass concentration", required=False),
             _phys("CO3", "mass_concentration", "carbonate [CO3 2-] as mass concentration", required=False),
             _phys("OH", "mass_concentration", "hydroxide [OH-] as mass concentration", required=False),
             _phys("H", "mass_concentration", "hydrogen ion [H+] as mass concentration", required=False),
             _phys("HCO3_M", "molar_concentration", "bicarbonate [HCO3-] as molar concentration", required=False, show=("mol/L", 1e-3)),
             _phys("CO3_M", "molar_concentration", "carbonate [CO3 2-] as molar concentration", required=False, show=("mol/L", 1e-3)),
             _phys("OH_M", "molar_concentration", "hydroxide [OH-] as molar concentration", required=False, show=("mol/L", 1e-3)),
             _phys("H_M", "molar_concentration", "hydrogen ion [H+] as molar concentration", required=False, show=("mol/L", 1e-3)),
             _numin("pH", "pH of the water (gives [H+])", required=False)],
     outputs=[_o("alkalinity", "number", "total alkalinity, mg/L as CaCO3"), _o("alk_meq_L", "number", "total alkalinity, meq/L"),
              _o("alk_eq_L", "number", "total alkalinity, eq/L"), _o("HCO3_caco3", "number", "bicarbonate alkalinity, mg/L as CaCO3"),
              _o("CO3_caco3", "number", "carbonate alkalinity, mg/L as CaCO3"), _o("OH_caco3", "number", "hydroxide alkalinity, mg/L as CaCO3"),
              _o("H_caco3", "number", "hydrogen ion (subtracted), mg/L as CaCO3"), _o("H_molL", "number", "[H+] used, mol/L"),
              _o("OH_molL", "number", "[OH-] used, mol/L")],
     primary="alkalinity",
     conditions=["at least one of HCO3, CO3, OH", "each ion once (mass or molar form; H also as pH)",
                 "neither H nor OH given: both neglected; one given: the other from Kw = 1e-14 (25 degC)",
                 "as CaCO3 with 50,000 mg/eq", "molar masses HCO3 61.017, CO3 60.009, OH 17.007, H 1.008 g/mol"],
     method="Alk (eq/L) = [HCO3-] + 2[CO3 2-] + [OH-] - [H+] (mol/L); Alk (mg/L as CaCO3) = 50,000 x Alk (eq/L)",
     latex=r"\text{Alk} = \left([\mathrm{HCO_3^-}] + 2[\mathrm{CO_3^{2-}}] + [\mathrm{OH^-}] - [\mathrm{H^+}]\right)\times 50\,000\ \mathrm{mg\,CaCO_3/eq}",
     run=alkalinity, reference=_ref_alkalinity, wrong=_wrong_alkalinity, rand=_rand_alk,
     phrases=("Total alkalinity", "Alkalinity as CaCO3", "Alkalinity of water", "Alkalinity"),
     aliases={"bicarbonate": "HCO3", "carbonate": "CO3", "hydroxide": "OH"},
     examples=[_ex({"HCO3": 0.122, "CO3": 0.030, "pH": 10.0}), _ex({"HCO3_M": 2.0e-3 * 1000.0, "CO3_M": 1.0e-4 * 1000.0})],
     level="beginner",
     extra_q=[{"q": "Alkalinity as CaCO3: bicarbonate = 122 mg/L, carbonate = 30 mg/L, pH = 10", "like": 0},
              {"q": "Total alkalinity: HCO3 = 122 mg/L, HCO3_M = 0.002 mol/L", "outcome": "INVALID_INPUT"},
              {"q": "Total alkalinity: pH = 7", "outcome": "INVALID_INPUT"},
              {"q": "Total alkalinity: HCO3 = -10 mg/L", "outcome": "INVALID_INPUT"},
              {"q": "Total alkalinity: HCO3 = 100 mg/L, H_M = 1e-7 mol/L, pH = 7", "outcome": "INVALID_INPUT"}])


def cmfr_series(Cin, k, theta=None, N=None, V=None, Q=None):
    _pos("Cin and k", Cin, k)
    if theta is None:
        _need(V is not None and Q is not None, "give the residence time theta, or the reactor volume V and the flow Q")
        _pos("the volume V and the flow Q", V, Q)
        theta = V / Q
    else:
        _need(V is None and Q is None, "give the residence time theta, or V and Q, not both")
        _pos("theta", theta)
    n = 1 if N is None else N
    _need(n == int(n) and 1 <= n <= 1000, "the number of reactors N must be a whole number from 1 to 1000")
    n = int(n)
    kt = k * theta
    Cout = Cin / (1.0 + kt / n) ** n
    Cone = Cin / (1.0 + kt)
    Cpfr = Cin * math.exp(-kt)
    return _out(Cout=Cout, Cout_mgL=Cout * 1e3, removal=1.0 - Cout / Cin, removal_percent=(1.0 - Cout / Cin) * 100.0,
                Cout_one=Cone, Cout_one_mgL=Cone * 1e3, Cout_pfr=Cpfr, Cout_pfr_mgL=Cpfr * 1e3,
                removal_pfr=1.0 - Cpfr / Cin, theta_each=theta / n, theta_each_h=theta / n / HOUR)


def _ref_cmfr_series(Cin, k, theta=None, N=None, V=None, Q=None):
    import numpy as np
    if theta is None:
        theta = _brentq(lambda s: Q * s - V, 0.0, 2.0 * V / Q)          # the time the flow takes to fill the volume
    n = 1 if N is None else int(N)
    th = theta / n
    # steady mass balances of the n tanks: (1 + k th) C_i - C_(i-1) = 0, C_0 = Cin
    A = np.eye(n) * (1.0 + k * th) - np.eye(n, k=-1)
    b = np.zeros(n)
    b[0] = Cin
    C = np.linalg.solve(A, b)
    Cout = float(C[-1])
    Cone = _brentq(lambda c: Cin - c - k * theta * c, 0.0, Cin)
    Cpfr = _ivp(lambda t, y: [-k * y[0]], theta, [Cin], atol=[1e-30])[0]
    return {"Cout": Cout, "Cout_mgL": Cout * 1e3, "removal": 1.0 - Cout / Cin, "removal_percent": (1.0 - Cout / Cin) * 100.0,
            "Cout_one": Cone, "Cout_one_mgL": Cone * 1e3, "Cout_pfr": Cpfr, "Cout_pfr_mgL": Cpfr * 1e3,
            "removal_pfr": 1.0 - Cpfr / Cin, "theta_each": th, "theta_each_h": th / 3600.0}


def _wrong_cmfr_series(Cin, k, theta=None, N=None, V=None, Q=None):
    r = cmfr_series(Cin, k, theta, N, V, Q)
    th = r["theta_each"] * (1 if N is None else int(N))
    n = 1 if N is None else int(N)
    if n == 1:
        Cout = Cin * math.exp(-k * th)                      # one stirred tank treated as plug flow
    else:
        Cout = Cin / (1.0 + k * th) ** n                    # every tank given the whole residence time
    return _out(**dict(r, Cout=Cout, Cout_mgL=Cout * 1e3, removal=1.0 - Cout / Cin, removal_percent=(1.0 - Cout / Cin) * 100.0))


def _rand_cmfr(rng):
    x = {"Cin": _sig(rng.uniform(5.0, 500.0), 3) * MG, "k": _sig(_lu(rng, 0.05, 20.0), 3) / DAY,
         "theta": _sig(_lu(rng, 0.5, 72.0), 3) * HOUR}
    if rng.random() < 0.9:
        x["N"] = rng.choice([1, 2, 3, 4, 5, 6, 8, 10, 20])
    if rng.random() < 0.35:                                 # the residence time as volume over flow
        q = _sig(_lu(rng, 50.0, 1e5), 3) / DAY
        x["V"] = _sig(x.pop("theta") * q, 3)
        x["Q"] = q
    return x


_add("enveng_cmfr_series",
     name="First-order reaction in N stirred tanks in series and in plug flow", topic="reactors / ideal reactors",
     purpose="The effluent concentration of a first-order reaction through N identical completely mixed flow reactors (CMFRs) in series sharing a total residence time, Cout = Cin/(1 + k theta/N)^N, with a single CMFR at steady state (N = 1, the default: Cout = Cin/(1 + k theta)) and the plug-flow limit Cin exp(-k theta) for comparison; theta given directly or as the volume over the flow, V/Q; with the fraction and per cent removed.",
     inputs=[_phys("Cin", "mass_concentration", "influent concentration"),
             _phys("k", "first_order_rate_constant", "first-order rate constant"),
             _phys("theta", "time", "total hydraulic residence time of all the tanks (or give V and Q)", required=False, show=("h", 1.0 / HOUR)),
             _intin("N", "number of identical tanks in series (default 1)", lo=1, hi=1000, required=False),
             _phys("V", "volume", "total reactor volume (with Q, in place of theta)", required=False),
             _phys("Q", "flow_rate", "flow through the reactor (with V, in place of theta)", required=False, show=("m^3/d", DAY))],
     outputs=[_o("Cout", "mass_concentration", "effluent of the N tanks Cin/(1 + k theta/N)^N"), _o("Cout_mgL", "number", "in mg/L"),
              _o("removal", "number", "fraction removed by the N tanks"), _o("removal_percent", "number", "per cent removed"),
              _o("Cout_one", "mass_concentration", "effluent of one CMFR of the same total volume"), _o("Cout_one_mgL", "number", "in mg/L"),
              _o("Cout_pfr", "mass_concentration", "effluent of a plug-flow reactor Cin exp(-k theta)"), _o("Cout_pfr_mgL", "number", "in mg/L"),
              _o("removal_pfr", "number", "fraction removed in plug flow"), _o("theta_each", "time", "residence time of each tank theta/N"),
              _o("theta_each_h", "number", "residence time of each tank, h")],
     primary="Cout", conditions=["steady state, first-order decay", "identical tanks sharing the total residence time theta", "1 <= N <= 1000",
                                 "theta, or the volume V and the flow Q (theta = V/Q), not both"],
     method="theta = V/Q when V and Q are given; Cout = Cin/(1 + k theta/N)^N; one CMFR: Cin/(1 + k theta); plug flow: Cin exp(-k theta)",
     latex=r"C_{out} = \frac{C_{in}}{\left(1 + k\theta/N\right)^N},\qquad \theta = \frac VQ,\qquad C_{PFR} = C_{in}\,e^{-k\theta}",
     run=cmfr_series, reference=_ref_cmfr_series, wrong=_wrong_cmfr_series, rand=_rand_cmfr,
     phrases=("CMFRs in series", "Reactors in series", "Tanks in series", "CSTRs in series", "First-order reactor",
              "Completely mixed flow reactor", "Completely mixed reactor", "Single CMFR", "CMFR", "CSTR",
              "First-order CMFR", "CMFR at steady state"),
     aliases={"tau": "theta", "HRT": "theta"},
     examples=[_ex({"Cin": 0.1, "k": 0.2 / HOUR, "theta": 10.0 * HOUR, "N": 4}), _ex({"Cin": 0.05, "k": 1.0 / DAY, "theta": DAY}),
               _ex({"Cin": 0.2, "k": 0.4 / HOUR, "V": 500.0, "Q": 100.0 / HOUR})],
     level="intermediate",
     extra_q=[{"q": "CSTRs in series: Cin = 100 mg/L, k = 4.8 1/d, theta = 10 h, N = 4", "like": 0},
              {"q": "Reactors in series: Cin = 100 mg/L, k = 0.2 1/h, theta = 10 h, N = 0", "outcome": "INVALID_INPUT"},
              {"q": "Reactors in series: Cin = 100 mg/L, k = -0.2 1/h, theta = 10 h, N = 2", "outcome": "INVALID_INPUT"},
              # batch 3 (2026-11-11): one completely mixed reactor with V and Q
              {"q": "CMFR: Cin = 200 mg/L, k = 0.4 1/h, V = 500 m^3, Q = 2400 m^3/d", "like": 2},
              {"q": "Completely mixed flow reactor: Cin = 200 mg/L, k = 9.6 1/d, theta = 5 h", "like": 2},
              {"q": "CSTR: Cin = 200 mg/L, k = 0.4 1/h, HRT = 5 h, N = 1", "like": 2},
              {"q": "CMFR: Cin = 200 mg/L, k = 0.4 1/h, theta = 5 h, V = 500 m^3, Q = 2400 m^3/d", "outcome": "INVALID_INPUT"},
              {"q": "CMFR: Cin = 200 mg/L, k = 0.4 1/h, V = 500 m^3", "outcome": "INVALID_INPUT"}])


def manning(n, S, W=None, y=None, D=None):
    _pos("Manning's n", n)
    _fin(S)
    _need(0.0 < S < 1.0, "the slope S must be between 0 and 1 (a fraction, or written with %)")
    if D is not None:
        _need(W is None and y is None, "give a rectangular channel (W, y) or a full circular pipe (D), not both")
        _pos("the diameter D", D)
        A = math.pi * D * D / 4.0
        P = math.pi * D
    else:
        _need(W is not None and y is not None, "give the channel width W and the flow depth y, or a pipe diameter D")
        _pos("the width W and the depth y", W, y)
        A = W * y
        P = W + 2.0 * y
    R = A / P
    v = R ** (2.0 / 3.0) * math.sqrt(S) / n
    Q = v * A
    return _out(v=v, Q=Q, Q_L_per_s=Q * 1e3, R=R, A=A, P=P)


def _ref_manning(n, S, W=None, y=None, D=None):
    import math as _m
    from fluids.open_flow import V_Manning
    if D is not None:
        r = D / 2.0
        A = 2.0 * _quad(lambda z: _m.sqrt(max(r * r - z * z, 0.0)), -r, r)     # chord widths over the diameter
        P = _quad(lambda t: r, 0.0, 2.0 * _m.pi)                              # arc length of the wetted circle
    else:
        A = _quad(lambda z: W, 0.0, y)
        P = W + _quad(lambda z: 2.0, 0.0, y)
    R = A / P
    v = float(V_Manning(R, S, n))
    return {"v": v, "Q": v * A, "Q_L_per_s": v * A * 1000.0, "R": R, "A": A, "P": P}


def _wrong_manning(n, S, W=None, y=None, D=None):
    r = manning(n, S, W, y, D)
    v = 1.49 * r["v"]                                       # the US customary 1.49 kept with SI units
    return _out(**dict(r, v=v, Q=v * r["A"], Q_L_per_s=v * r["A"] * 1e3))


def _rand_manning(rng):
    x = {"n": _sig(rng.uniform(0.009, 0.05), 2), "S": _sig(_lu(rng, 1e-4, 0.05), 3)}
    if rng.random() < 0.5:
        x["D"] = _sig(rng.uniform(0.15, 3.0), 3)
    else:
        x["W"] = _sig(rng.uniform(0.3, 10.0), 3)
        x["y"] = _sig(rng.uniform(0.1, 3.0), 3)
    return x


_add("enveng_manning_flow",
     name="Manning's equation: velocity and flow in a channel or a full pipe (SI)", topic="hydraulics / open channels and sewers",
     purpose="The mean velocity and flow by Manning's equation in SI units, v = (1/n) R^(2/3) S^(1/2), for a rectangular channel (width and flow depth) or a circular pipe flowing full (diameter), with the hydraulic radius, area and wetted perimeter.",
     inputs=[_numin("n", "Manning roughness coefficient"),
             _numin("S", "slope of the channel or energy line (fraction or %)", pct=True),
             _phys("W", "length", "channel width (rectangular channel)", required=False),
             _phys("y", "length", "flow depth (rectangular channel)", required=False),
             _phys("D", "length", "pipe diameter (circular pipe flowing full)", required=False)],
     outputs=[_o("v", "speed", "mean velocity (1/n) R^(2/3) S^(1/2)"), _o("Q", "flow_rate", "flow v A"), _o("Q_L_per_s", "number", "flow, L/s"),
              _o("R", "length", "hydraulic radius A/P"), _o("A", "area", "flow area"), _o("P", "length", "wetted perimeter")],
     primary="Q", conditions=["uniform flow, SI units (factor 1)", "0 < S < 1", "rectangular channel (W, y) or a full circular pipe (D)"],
     method="R = A/P (rectangle: A = W y, P = W + 2y; full pipe: R = D/4); v = R^(2/3) S^(1/2)/n; Q = v A",
     latex=r"v = \frac{1}{n}R^{2/3}S^{1/2},\qquad Q = vA,\qquad R = \frac{A}{P}",
     run=manning, reference=_ref_manning, wrong=_wrong_manning, rand=_rand_manning,
     phrases=("Manning's equation", "Manning equation", "Flow by Manning's equation", "Manning flow"),
     examples=[_ex({"n": 0.013, "S": 0.001, "D": 0.6}), _ex({"n": 0.015, "S": 0.0005, "W": 3.0, "y": 1.2})],
     level="intermediate",
     extra_q=[{"q": "Manning equation: n = 0.013, S = 0.1%, D = 600 mm", "like": 0},
              {"q": "Manning's equation: n = 0.013, S = 0.001, D = 0.6 m, W = 3 m, y = 1 m", "outcome": "INVALID_INPUT"},
              {"q": "Manning's equation: n = 0.013, S = 0.001, W = 3 m", "outcome": "INVALID_INPUT"},
              {"q": "Manning's equation: n = 0.013, S = 0, D = 0.6 m", "outcome": "INVALID_INPUT"}])


def hazen_williams(C, D, S=None, Q=None, L=None):
    _pos("the Hazen-Williams coefficient C and the diameter D", C, D)
    _need((S is None) != (Q is None), "give either the slope of the energy line S or the flow Q, not both")
    R = D / 4.0
    A = math.pi * D * D / 4.0
    k = 0.849 * C * R ** 0.63
    if S is not None:
        _fin(S)
        _need(0.0 < S < 1.0, "the slope S must be between 0 and 1 (a fraction, or written with %)")
        v = k * S ** 0.54
        Q = v * A
    else:
        _pos("the flow Q", Q)
        v = Q / A
        S = (v / k) ** (1.0 / 0.54)
        _need(S < 1.0, "the head loss gradient would exceed 1: outside the range of the Hazen-Williams formula")
    out = {"v": v, "Q": Q, "S": S, "R": R, "A": A}
    if L is not None:
        _pos("the pipe length L", L)
        out["hf"] = S * L
    return _out(**out)


def _ref_hazen_williams(C, D, S=None, Q=None, L=None):
    import mpmath as mp
    mp.mp.dps = 40
    R = mp.mpf(D) / 4
    A = mp.pi * mp.mpf(D) ** 2 / 4

    def vel(s):                                            # the formula in logarithms
        return mp.exp(mp.log(mp.mpf("0.849")) + mp.log(C) + mp.mpf("0.63") * mp.log(R) + mp.mpf("0.54") * mp.log(s))
    if S is not None:
        s = mp.mpf(S)
    else:
        s = mp.findroot(lambda t: vel(mp.exp(t)) * A - Q, mp.mpf(-5))       # log of the slope that carries Q
        s = mp.exp(s)
    v = vel(s)
    out = {"v": float(v), "Q": float(v * A), "S": float(s), "R": float(R), "A": float(A)}
    if L is not None:
        out["hf"] = float(s * L)
    return out


def _wrong_hazen_williams(C, D, S=None, Q=None, L=None):
    A = math.pi * D * D / 4.0
    k = 0.849 * C * D ** 0.63                               # the diameter in place of the hydraulic radius
    if S is not None:
        v = k * S ** 0.54
        Q = v * A
    else:
        v = Q / A
        S = (v / k) ** (1.0 / 0.54)
    out = {"v": v, "Q": Q, "S": S, "R": D / 4.0, "A": A}
    if L is not None:
        out["hf"] = S * L
    return _out(**out)


def _rand_hw(rng):
    x = {"C": float(rng.choice([80, 100, 110, 120, 130, 140, 150])), "D": _sig(rng.uniform(0.05, 1.5), 3)}
    if rng.random() < 0.5:
        x["S"] = _sig(_lu(rng, 1e-4, 0.05), 3)
    else:
        A = math.pi * x["D"] ** 2 / 4.0
        x["Q"] = _sig(A * rng.uniform(0.3, 3.0), 3)
    if rng.random() < 0.6:
        x["L"] = _sig(rng.uniform(10.0, 5000.0), 3)
    return x


_add("enveng_hazen_williams",
     name="Hazen-Williams equation for a full pipe (SI): velocity, flow and head loss", topic="hydraulics / pipe flow",
     purpose="Water flow in a full circular pipe by the Hazen-Williams formula in SI units, v = 0.849 C R^0.63 S^0.54 (R = D/4): the velocity and flow from the slope of the energy line, or the slope from the flow, and the head loss over a pipe length.",
     inputs=[_numin("C", "Hazen-Williams roughness coefficient"), _phys("D", "length", "pipe diameter"),
             _numin("S", "slope of the energy line hf/L (fraction or %)", required=False, pct=True),
             _phys("Q", "flow_rate", "flow rate", required=False), _phys("L", "length", "pipe length (for the head loss)", required=False)],
     outputs=[_o("v", "speed", "mean velocity"), _o("Q", "flow_rate", "flow"), _o("S", "number", "slope of the energy line hf/L"),
              _o("hf", "length", "head loss over L"), _o("R", "length", "hydraulic radius D/4"), _o("A", "area", "pipe area")],
     primary="v", conditions=["water, turbulent flow, full circular pipe", "SI form (0.849)", "exactly one of S, Q", "S < 1"],
     method="R = D/4; v = 0.849 C R^0.63 S^0.54; Q = v A; from Q: S = (v/(0.849 C R^0.63))^(1/0.54); hf = S L",
     latex=r"v = 0.849\,C\,R^{0.63}S^{0.54},\qquad Q = vA,\qquad h_f = S\,L",
     run=hazen_williams, reference=_ref_hazen_williams, wrong=_wrong_hazen_williams, rand=_rand_hw,
     phrases=("Hazen-Williams equation", "Hazen-Williams", "Hazen Williams", "Head loss by Hazen-Williams"),
     examples=[_ex({"C": 100.0, "D": 0.3, "S": 0.002}), _ex({"C": 120.0, "D": 0.2, "Q": 0.03, "L": 1000.0})],
     level="intermediate",
     extra_q=[{"q": "Head loss by Hazen-Williams: C = 120, D = 200 mm, Q = 30 L/s, L = 1 km", "like": 1},
              {"q": "Hazen-Williams: C = 100, D = 0.3 m, S = 0.002, Q = 0.1 m^3/s", "outcome": "INVALID_INPUT"},
              {"q": "Hazen-Williams: C = 100, D = 0.3 m", "outcome": "INVALID_INPUT"}])


_PRICE = ("$/kWh", "USD/kWh", "/kWh", "per kWh", "$ per kWh")


def pump_power(Q, H, eta, rho=None, hours=None, price=None):
    _pos("the flow Q and the head H", Q, H)
    _frac("the pump efficiency eta", eta, lo_open=True)
    r = RHO_WATER if rho is None else rho
    _pos("the density rho", r)
    Pw = r * G0 * Q * H
    P = Pw / eta
    out = {"P": P, "P_kW": P / 1e3, "P_water": Pw, "P_water_kW": Pw / 1e3}
    if price is not None:
        _need(hours is not None, "give the operating hours with the electricity price")
    if hours is not None:
        _pos("the operating time", hours)
        E = P / 1e3 * hours / HOUR
        out["energy_kWh"] = E
        if price is not None:
            _nonneg("the electricity price", price)
            out["cost"] = E * price
    return _out(**out)


def _ref_pump_power(Q, H, eta, rho=None, hours=None, price=None):
    import numpy as np
    from scipy.integrate import quad
    Pw = float(np.prod([1000.0 if rho is None else rho, 9.80665, Q, H]))
    P = Pw / eta
    out = {"P": P, "P_kW": P / 1e3, "P_water": Pw, "P_water_kW": Pw / 1e3}
    if hours is not None:
        J = quad(lambda t: P, 0.0, hours, epsabs=0.0, epsrel=1e-13)[0]        # energy drawn over the operating time
        out["energy_kWh"] = J / 3.6e6
        if price is not None:
            out["cost"] = J / 3.6e6 * price
    return out


def _wrong_pump_power(Q, H, eta, rho=None, hours=None, price=None):
    r = pump_power(Q, H, eta, rho, hours, price)
    P = r["P_water"] * eta                                  # efficiency multiplied instead of divided
    out = dict(r, P=P, P_kW=P / 1e3)
    if hours is not None:
        out["energy_kWh"] = P / 1e3 * hours / HOUR
        if price is not None:
            out["cost"] = out["energy_kWh"] * price
    return _out(**out)


def _rand_pump(rng):
    x = {"Q": _sig(_lu(rng, 0.005, 2.0), 3), "H": _sig(rng.uniform(3.0, 120.0), 3), "eta": _sig(rng.uniform(0.4, 0.92), 2)}
    if rng.random() < 0.3:
        x["rho"] = _sig(rng.uniform(990.0, 1100.0), 4)
    if rng.random() < 0.6:
        x["hours"] = _sig(rng.uniform(1.0, 8760.0), 3) * HOUR
        if rng.random() < 0.7:
            x["price"] = _sig(rng.uniform(0.05, 0.4), 2)
    return x


_add("enveng_pump_power",
     name="Pump power from flow and head, with energy and electricity cost", topic="hydraulics / pumping",
     purpose="The power a pump draws to lift a flow Q through a total head H with efficiency eta, P = rho g Q H / eta (water 1000 kg/m^3 unless a density is given), in kW, and the energy and electricity cost over a number of operating hours at a price per kWh.",
     inputs=[_phys("Q", "flow_rate", "flow pumped"), _phys("H", "length", "total dynamic head"),
             _numin("eta", "pump (wire-to-water) efficiency (fraction or %)", pct=True),
             _phys("rho", "density", "density of the liquid (default water, 1000 kg/m^3)", required=False),
             _phys("hours", "time", "operating time", required=False, show=("h", 1.0 / HOUR)),
             _numin("price", "electricity price per kWh", required=False, words=_PRICE)],
     outputs=[_o("P", "power", "power drawn to lift the water: the input (brake, shaft or motor) power the pump requires, rho g Q H / eta"),
              _o("P_kW", "number", "power drawn, kW"),
              _o("P_water", "power", "water (hydraulic) power, the useful output power imparted to the water, rho g Q H"),
              _o("P_water_kW", "number", "water power, kW"),
              _o("energy_kWh", "number", "energy over the operating time, kWh"), _o("cost", "number", "electricity cost, in the price's currency")],
     primary="P", conditions=["g = 9.80665 m/s^2", "0 < eta <= 1", "rho = 1000 kg/m^3 unless given", "a price needs the operating hours"],
     method="P = rho g Q H / eta; E = P t; cost = E x price",
     latex=r"P = \frac{\rho\,g\,Q\,H}{\eta},\qquad E = P\,t,\qquad \text{cost} = E\cdot\text{price}",
     run=pump_power, reference=_ref_pump_power, wrong=_wrong_pump_power, rand=_rand_pump,
     phrases=("Pump power", "Power required by a pump", "Pumping power", "Pump power and energy cost",
              "Brake power of a pump", "Input power of a pump", "Motor power of a pump", "Pump brake power",
              "Pump input power", "Pump motor power", "Brake power", "Shaft power of a pump"),
     examples=[_ex({"Q": 0.1, "H": 30.0, "eta": 0.75}), _ex({"Q": 0.05, "H": 20.0, "eta": 0.8, "hours": 24.0 * HOUR, "price": 0.12})],
     level="beginner",
     extra_q=[{"q": "Power required by a pump: Q = 360 m^3/h, H = 30 m, eta = 75%", "like": 0},
              # batch 3 (2026-11-11): exam forms asking for the power drawn (input, brake or motor power), P = rho g Q H/eta
              {"q": "Brake power of a pump: Q = 0.1 m^3/s, H = 30 m, eta = 75%", "like": 0},
              {"q": "Input power of a pump: Q = 360 m^3/h, H = 30 m, eta = 0.75", "like": 0},
              {"q": "Motor power of a pump: Q = 8640 m^3/d, H = 30 m, eta = 75%", "like": 0},
              {"q": "Pump brake power: Q = 0.05 m^3/s, H = 20 m, eta = 80%, hours = 24 h, price = 0.12 $/kWh", "like": 1},
              {"q": "Pump power: Q = 0.1 m^3/s, H = 30 m, eta = 150%", "outcome": "INVALID_INPUT"},
              {"q": "Pump power: Q = 0.1 m^3/s, H = 30 m, eta = 0.75, price = 0.1 $/kWh", "outcome": "INVALID_INPUT"}])


def rational_method(C, i, A):
    _frac("the runoff coefficient C", C, lo_open=True)
    _pos("the rainfall intensity i and the area A", i, A)
    Q = C * i * A
    return _out(Q=Q, Q_L_per_s=Q * 1e3, Q_m3_per_h=Q * HOUR)


def _ref_rational_method(C, i, A):
    i_mmh = i * 3.6e6                                       # mm/h
    A_ha = A / 1e4                                          # hectares
    Q = C * i_mmh * A_ha / 360.0                            # the customary metric form, m^3/s
    return {"Q": Q, "Q_L_per_s": Q * 1000.0, "Q_m3_per_h": Q * 3600.0}


_add("enveng_rational_method",
     name="Rational method: peak runoff Q = C i A", topic="hydrology / stormwater",
     purpose="The peak storm runoff from a small catchment by the rational method, Q = C i A, from the runoff coefficient, the rainfall intensity for the time of concentration (mm/h) and the drainage area (m^2, km^2 or ha), in m^3/s.",
     inputs=[_numin("C", "runoff coefficient (fraction or %)", pct=True), _phys("i", "speed", "rainfall intensity", show=("mm/h", 3.6e6)),
             _phys("A", "area", "drainage area", show=("km^2", 1e-6))],
     outputs=[_o("Q", "flow_rate", "peak runoff C i A"), _o("Q_L_per_s", "number", "peak runoff, L/s"), _o("Q_m3_per_h", "number", "peak runoff, m^3/h")],
     primary="Q", conditions=["0 < C <= 1", "i for a duration equal to the time of concentration", "small catchment"],
     method="Q = C i A (SI); with i in mm/h and A in ha, Q = C i A / 360 m^3/s", latex=r"Q = C\,i\,A",
     run=rational_method, reference=_ref_rational_method,
     wrong=lambda C, i, A: _out(Q=C * i * A / 3.6, Q_L_per_s=C * i * A / 3.6 * 1e3, Q_m3_per_h=C * i * A / 3.6 * HOUR),
     rand=lambda rng: {"C": _sig(rng.uniform(0.1, 0.95), 2), "i": _sig(rng.uniform(5.0, 200.0), 3) / 3.6e6, "A": _sig(_lu(rng, 0.1, 500.0), 3) * 1e4},
     phrases=("Rational method", "Peak runoff by the rational method", "Peak runoff", "Rational formula"),
     examples=[_ex({"C": 0.6, "i": 50.0 / 3.6e6, "A": 0.25e6}), _ex({"C": 0.35, "i": 80.0 / 3.6e6, "A": 1.2e6})],
     level="beginner",
     extra_q=[{"q": "Rational method: C = 0.6, i = 50 mm/h, A = 25 ha", "like": 0},
              {"q": "Peak runoff: C = 0.6, i = 50 mm/h, A = 250000 m^2", "like": 0},
              {"q": "Rational method: C = 140%, i = 50 mm/h, A = 25 ha", "outcome": "INVALID_INPUT"},
              {"q": "Rational method: C = 0.6, i = 0 mm/h, A = 25 ha", "outcome": "INVALID_INPUT"}])


def indoor_air(S, Qv, V, Camb=None, k=None):
    _nonneg("the indoor emission rate S", S)
    _pos("the ventilation rate Qv and the room volume V", Qv, V)
    ca = 0.0 if Camb is None else Camb
    kk = 0.0 if k is None else k
    _nonneg("the outdoor concentration Camb", ca)
    _nonneg("the decay rate k", kk)
    _need(S + Qv * ca > 0, "nothing enters the room: give an emission rate S or an outdoor concentration Camb")
    C = (Qv * ca + S) / (Qv + kk * V)
    return _out(C=C, C_ugm3=C * 1e9, C_mgm3=C * 1e6, ach=Qv / V * HOUR, tau_h=V / (Qv + kk * V) / HOUR)


def _ref_indoor_air(S, Qv, V, Camb=None, k=None):
    ca = 0.0 if Camb is None else Camb
    kk = 0.0 if k is None else k
    # the room's mass balance V dC/dt = S + Qv Camb - Qv C - k V C, at its root
    hi = (S + Qv * ca) / Qv * 2.0
    C = _brentq(lambda c: S + Qv * ca - Qv * c - kk * V * c, 0.0, hi)
    return {"C": C, "C_ugm3": C * 1e9, "C_mgm3": C * 1e6, "ach": Qv * 3600.0 / V, "tau_h": V / (Qv + kk * V) / 3600.0}


def _rand_indoor(rng):
    V = _sig(rng.uniform(20.0, 2000.0), 3)
    x = {"S": _sig(_lu(rng, 0.1, 500.0), 3) * 1e-6 / HOUR, "Qv": _sig(V * rng.uniform(0.2, 6.0), 3) / HOUR, "V": V}
    if rng.random() < 0.85:
        x["Camb"] = _sig(rng.uniform(1.0, 200.0), 3) * 1e-9
    if rng.random() < 0.6:
        x["k"] = _sig(rng.uniform(0.05, 3.0), 3) / HOUR
    return x


_add("enveng_indoor_air_steady_state",
     name="Steady indoor air concentration (well-mixed room box model)", topic="air pollution / indoor air quality",
     purpose="The steady concentration of a pollutant in a well-mixed room from its indoor emission rate, the ventilation rate, the outdoor concentration and a first-order decay rate, C = (Qv Camb + S)/(Qv + k V), with the air changes per hour.",
     inputs=[_phys("S", "mass_flow_rate", "indoor emission rate", show=("mg/h", 1e6 * HOUR)),
             _phys("Qv", "flow_rate", "ventilation (outdoor air) rate", show=("m^3/h", HOUR)),
             _phys("V", "volume", "room volume"),
             _phys("Camb", "mass_concentration", "outdoor (ambient) concentration (default 0)", required=False, show=("ug/m^3", 1e9)),
             _phys("k", "first_order_rate_constant", "first-order decay (reaction, deposition) rate (default 0)", required=False, show=("1/h", HOUR))],
     outputs=[_o("C", "mass_concentration", "steady indoor concentration"), _o("C_ugm3", "number", "in ug/m^3"), _o("C_mgm3", "number", "in mg/m^3"),
              _o("ach", "number", "air changes per hour Qv/V"), _o("tau_h", "number", "time constant V/(Qv + k V), h")],
     primary="C", conditions=["well-mixed room at steady state", "Camb = 0 and k = 0 unless given", "something enters the room"],
     method="C = (Qv Camb + S)/(Qv + k V); ach = Qv/V",
     latex=r"C_{ss} = \frac{Q_v C_{amb} + S}{Q_v + k V}",
     run=indoor_air, reference=_ref_indoor_air,
     wrong=lambda S, Qv, V, Camb=None, k=None: (lambda ca, kk: _out(C=(ca + S) / (Qv + kk * V), C_ugm3=(ca + S) / (Qv + kk * V) * 1e9, C_mgm3=(ca + S) / (Qv + kk * V) * 1e6,
                                                                   ach=Qv / V * HOUR, tau_h=V / (Qv + kk * V) / HOUR))(0.0 if Camb is None else Camb, 0.0 if k is None else k),
     rand=_rand_indoor,
     phrases=("Indoor air concentration", "Indoor air box model", "Steady indoor concentration", "Indoor air quality box model"),
     examples=[_ex({"S": 100.0 / (1e6 * HOUR), "Qv": 100.0 / HOUR, "V": 200.0, "Camb": 10.0e-9, "k": 0.5 / HOUR}),
               _ex({"S": 50.0 / (1e6 * HOUR), "Qv": 250.0 / HOUR, "V": 500.0})],
     level="intermediate",
     extra_q=[{"q": "Indoor air box model: S = 100 mg/h, Qv = 100 m^3/h, V = 200 m^3, Camb = 10 ug/m^3, k = 0.5 1/h", "like": 0},
              {"q": "Indoor air concentration: S = 0 mg/h, Qv = 100 m^3/h, V = 200 m^3", "outcome": "INVALID_INPUT"},
              {"q": "Indoor air concentration: S = 100 mg/h, Qv = 0 m^3/h, V = 200 m^3", "outcome": "INVALID_INPUT"}])


def series_efficiency(eta, C_in=None):
    _need(len(eta) >= 1, "give the efficiency of each device")
    pen = 1.0
    for e in eta:
        _fin(e)
        _need(0.0 <= e <= 1.0, "each efficiency must be a fraction from 0 to 1 (write 0.9 for 90%)")
        pen = pen * (1.0 - e)
    out = {"eta_total": 1.0 - pen, "eta_total_percent": (1.0 - pen) * 100.0, "penetration": pen}
    if C_in is not None:
        _pos("the inlet concentration C_in", C_in)
        out["C_out"] = C_in * pen
        out["C_out_mgm3"] = C_in * pen * 1e6
    return _out(**out)


def _ref_series_efficiency(eta, C_in=None):
    import numpy as np
    with np.errstate(divide="ignore"):
        pen = float(np.exp(np.sum(np.log1p(-np.array(eta, float)))))   # penetrations multiplied as a sum of logs
    out = {"eta_total": 1.0 - pen, "eta_total_percent": (1.0 - pen) * 100.0, "penetration": pen}
    if C_in is not None:
        out["C_out"] = C_in * pen
        out["C_out_mgm3"] = C_in * pen * 1e6
    return out


def _wrong_series_efficiency(eta, C_in=None):
    p = 1.0
    for e in eta:
        p = p * e
    pen = 1.0 - p                                           # efficiencies multiplied instead of penetrations
    out = {"eta_total": p, "eta_total_percent": p * 100.0, "penetration": pen}
    if C_in is not None:
        out.update(C_out=C_in * pen, C_out_mgm3=C_in * pen * 1e6)
    return _out(**out)


_add("enveng_collection_efficiency_series",
     name="Overall efficiency of collection devices in series", topic="air pollution / particulate control",
     purpose="The overall collection efficiency of air pollution control devices in series (a cyclone before a baghouse or an ESP), 1 - prod(1 - eta_i), the fraction that penetrates, and the outlet concentration for a given inlet concentration.",
     inputs=[_vecin("eta", "efficiency of each device as a fraction (0.9 for 90%)", 1, 20),
             _phys("C_in", "mass_concentration", "inlet concentration (optional)", required=False, show=("mg/m^3", 1e6))],
     outputs=[_o("eta_total", "number", "overall efficiency 1 - prod(1 - eta_i)"), _o("eta_total_percent", "number", "overall efficiency, %"),
              _o("penetration", "number", "fraction passing all devices prod(1 - eta_i)"),
              _o("C_out", "mass_concentration", "outlet concentration"), _o("C_out_mgm3", "number", "outlet concentration, mg/m^3")],
     primary="eta_total", conditions=["devices in series, each efficiency on what reaches it", "0 <= eta_i <= 1 as fractions"],
     method="P = prod(1 - eta_i); eta = 1 - P; C_out = C_in P",
     latex=r"\eta_{overall} = 1 - \prod_i\left(1 - \eta_i\right)",
     run=series_efficiency, reference=_ref_series_efficiency, wrong=_wrong_series_efficiency,
     rand=lambda rng: dict({"eta": [_sig(rng.uniform(0.3, 0.995), 3) for _ in range(rng.randint(2, 4))]},
                           **({"C_in": _sig(_lu(rng, 10.0, 20000.0), 3) * 1e-6} if rng.random() < 0.5 else {})),
     phrases=("Overall collection efficiency", "Collection devices in series", "Overall efficiency in series", "Efficiency of devices in series"),
     examples=[_ex({"eta": [0.8, 0.99]}), _ex({"eta": [0.9, 0.95, 0.5], "C_in": 5000.0e-6})],
     level="beginner",
     extra_q=[{"q": "Collection devices in series: eta = [0.80, 0.99]", "like": 0},
              {"q": "Overall collection efficiency: eta = [80, 99]", "outcome": "INVALID_INPUT"}])


def hhv_dry_basis(w, HHV_ar=None, HHV_dry=None, ash=None):
    _frac("the moisture fraction w", w, hi_open=True)
    _need((HHV_ar is None) != (HHV_dry is None), "give the heating value as received (HHV_ar) or on a dry basis (HHV_dry), not both")
    if HHV_ar is not None:
        _pos("the heating value", HHV_ar)
        ar = HHV_ar
        dry = ar / (1.0 - w)
    else:
        _pos("the heating value", HHV_dry)
        dry = HHV_dry
        ar = dry * (1.0 - w)
    out = {"HHV_dry": dry, "HHV_ar": ar, "HHV_dry_MJ_per_kg": dry / 1e6, "HHV_ar_MJ_per_kg": ar / 1e6}
    if ash is not None:
        _frac("the ash fraction (as received)", ash)
        _need(w + ash < 1.0, "the moisture and ash fractions must add up to less than 1")
        out["HHV_daf"] = ar / (1.0 - w - ash)
        out["HHV_daf_MJ_per_kg"] = ar / (1.0 - w - ash) / 1e6
    return _out(**out)


def _ref_hhv_dry(w, HHV_ar=None, HHV_dry=None, ash=None):
    from fractions import Fraction as F
    # one kilogram as received: its energy and its dry (and ash-free) mass
    wq = F(w)
    if HHV_ar is not None:
        energy = F(HHV_ar)
    else:
        energy = F(HHV_dry) * (1 - wq)
    dry_mass = 1 - wq
    out = {"HHV_dry": float(energy / dry_mass), "HHV_ar": float(energy), "HHV_dry_MJ_per_kg": float(energy / dry_mass / 10 ** 6),
           "HHV_ar_MJ_per_kg": float(energy / 10 ** 6)}
    if ash is not None:
        daf = energy / (dry_mass - F(ash))
        out["HHV_daf"] = float(daf)
        out["HHV_daf_MJ_per_kg"] = float(daf / 10 ** 6)
    return out


def _wrong_hhv_dry(w, HHV_ar=None, HHV_dry=None, ash=None):
    ar = HHV_ar if HHV_ar is not None else HHV_dry / (1.0 - w)
    dry = ar * (1.0 - w)                                    # moisture removed from the energy instead of the mass
    out = {"HHV_dry": dry, "HHV_ar": ar, "HHV_dry_MJ_per_kg": dry / 1e6, "HHV_ar_MJ_per_kg": ar / 1e6}
    if ash is not None:
        out.update(HHV_daf=ar * (1.0 - w - ash), HHV_daf_MJ_per_kg=ar * (1.0 - w - ash) / 1e6)
    return _out(**out)


def _rand_hhv(rng):
    x = {"w": _sig(rng.uniform(0.02, 0.6), 2)}
    if rng.random() < 0.6:
        x["HHV_ar"] = _sig(rng.uniform(5.0, 30.0), 3) * 1e6
    else:
        x["HHV_dry"] = _sig(rng.uniform(10.0, 35.0), 3) * 1e6
    if rng.random() < 0.4:
        x["ash"] = _sig(rng.uniform(0.01, 0.3), 2)
    return x


_add("enveng_heating_value_dry_basis",
     name="Heating value on a dry basis from the as-received value and the moisture", topic="solid waste / energy recovery",
     purpose="The higher heating value of a fuel or waste on a dry basis from its as-received value and moisture fraction, HHV_dry = HHV_ar/(1 - w) (or the reverse), and on a dry ash-free basis when the ash fraction is given.",
     inputs=[_numin("w", "moisture fraction as received, wet basis (fraction or %)", pct=True),
             _phys("HHV_ar", "specific_energy", "heating value as received", required=False, show=("kJ/kg", 1e-3)),
             _phys("HHV_dry", "specific_energy", "heating value on a dry basis", required=False, show=("kJ/kg", 1e-3)),
             _numin("ash", "ash fraction as received (fraction or %)", required=False, pct=True)],
     outputs=[_o("HHV_dry", "specific_energy", "dry-basis heating value HHV_ar/(1 - w)"), _o("HHV_ar", "specific_energy", "as-received heating value"),
              _o("HHV_dry_MJ_per_kg", "number", "dry basis, MJ/kg"), _o("HHV_ar_MJ_per_kg", "number", "as received, MJ/kg"),
              _o("HHV_daf", "specific_energy", "dry ash-free heating value HHV_ar/(1 - w - ash)"), _o("HHV_daf_MJ_per_kg", "number", "dry ash-free, MJ/kg")],
     primary="HHV_dry", conditions=["0 <= w < 1 (wet-basis moisture)", "exactly one of HHV_ar, HHV_dry", "w + ash < 1"],
     method="HHV_dry = HHV_ar/(1 - w); HHV_daf = HHV_ar/(1 - w - ash)",
     latex=r"HHV_{dry} = \frac{HHV_{ar}}{1 - w},\qquad HHV_{daf} = \frac{HHV_{ar}}{1 - w - a}",
     run=hhv_dry_basis, reference=_ref_hhv_dry, wrong=_wrong_hhv_dry, rand=_rand_hhv,
     phrases=("Heating value on a dry basis", "Dry basis heating value", "Dry-basis heating value", "HHV dry basis"),
     examples=[_ex({"w": 0.25, "HHV_ar": 12000e3}), _ex({"w": 0.2, "HHV_dry": 18000e3, "ash": 0.1})],
     level="beginner",
     extra_q=[{"q": "Dry basis heating value: w = 25%, HHV_ar = 12 MJ/kg", "like": 0},
              {"q": "Heating value on a dry basis: w = 1, HHV_ar = 12000 kJ/kg", "outcome": "INVALID_INPUT"},
              {"q": "Heating value on a dry basis: w = 0.25, HHV_ar = 12000 kJ/kg, HHV_dry = 16000 kJ/kg", "outcome": "INVALID_INPUT"}])


_YEARS = ("yr", "years", "year", "y")


def return_period(T=None, n=None, risk=None):
    if n is not None:
        _fin(n)
        _need(n >= 1 and n == int(n) and n <= 10000, "the number of years n must be a whole number from 1 to 10000")
    if T is not None:
        _need(risk is None, "give the return period T or an acceptable risk, not both")
        _fin(T)
        _need(T >= 1.0, "the return period T must be at least 1 year")
        p = 1.0 / T
    else:
        _need(risk is not None and n is not None, "give the return period T, or an acceptable risk with the design life n")
        _fin(risk)
        _need(0.0 < risk < 1.0, "the risk must be a probability between 0 and 1")
        p = -math.expm1(math.log1p(-risk) / n)
        T = 1.0 / p
    out = {"p": p, "p_percent": p * 100.0, "T": T}
    if n is not None:
        none = math.exp(n * math.log1p(-p)) if p < 1.0 else 0.0
        P = -math.expm1(n * math.log1p(-p)) if p < 1.0 else 1.0
        out["P_n"] = P
        out["P_n_percent"] = P * 100.0
        out["P_none"] = none
    return _out(**out)


def _ref_return_period(T=None, n=None, risk=None):
    from scipy.stats import binom
    if T is not None:
        p = 1.0 / T
    else:
        p = _brentq(lambda q: float(binom.sf(0, int(n), q)) - risk, 1e-300, 1.0)   # the yearly probability giving that risk
    out = {"p": p, "p_percent": p * 100.0, "T": 1.0 / p}
    if n is not None:
        out["P_n"] = float(binom.sf(0, int(n), p))                # at least one exceedance in n independent years
        out["P_n_percent"] = out["P_n"] * 100.0
        out["P_none"] = float(binom.pmf(0, int(n), p))
    return out


def _wrong_return_period(T=None, n=None, risk=None):
    p = 1.0 / T if T is not None else risk / n
    out = {"p": p, "p_percent": p * 100.0, "T": 1.0 / p}
    if n is not None:
        out.update(P_n=n * p, P_n_percent=n * p * 100.0, P_none=1.0 - n * p)    # the linear (wrong) risk
    return _out(**out)


def _rand_rp(rng):
    u = rng.random()
    if u < 0.8:
        x = {"T": float(rng.choice([2, 5, 10, 25, 50, 100, 200, 500, 1000]))}
        if u < 0.7:
            x["n"] = float(rng.randint(2, 100))
        return x
    return {"risk": _sig(rng.uniform(0.01, 0.6), 2), "n": float(rng.randint(2, 100))}


_add("enveng_return_period_risk",
     name="Return period, exceedance probability and risk over a design life", topic="hydrology / frequency analysis",
     purpose="The annual exceedance probability of an event with return period T, p = 1/T, and the probability that it is equalled or exceeded at least once in n years, 1 - (1 - 1/T)^n; or the return period needed for an acceptable risk over a design life.",
     inputs=[_numin("T", "return period, years", required=False, words=_YEARS),
             _numin("n", "number of years (design life)", required=False, words=_YEARS),
             _numin("risk", "acceptable probability of at least one exceedance in n years (fraction or %)", required=False, pct=True)],
     outputs=[_o("p", "number", "annual exceedance probability 1/T"), _o("p_percent", "number", "annual exceedance probability, %"),
              _o("T", "number", "return period, years"), _o("P_n", "number", "probability of at least one exceedance in n years"),
              _o("P_n_percent", "number", "that probability, %"), _o("P_none", "number", "probability of no exceedance in n years")],
     primary="p", conditions=["independent years", "T >= 1 yr", "n a whole number of years", "T, or a risk with n"],
     method="p = 1/T; P(at least one in n years) = 1 - (1 - p)^n; for a risk R: p = 1 - (1 - R)^(1/n), T = 1/p",
     latex=r"p = \frac{1}{T},\qquad P_n = 1 - \left(1 - \frac{1}{T}\right)^n",
     run=return_period, reference=_ref_return_period, wrong=_wrong_return_period, rand=_rand_rp,
     phrases=("Return period", "Exceedance probability", "Probability of exceedance", "Return period risk"),
     examples=[_ex({"T": 100.0, "n": 30.0}), _ex({"risk": 0.1, "n": 25.0})],
     level="beginner",
     extra_q=[{"q": "Probability of exceedance: T = 100 yr, n = 30 years", "like": 0},
              {"q": "Return period: T = 50 yr", "outputs": {"p": 0.02}},
              {"q": "Return period: T = 0.5 yr", "outcome": "INVALID_INPUT"},
              {"q": "Return period: T = 100 yr, n = 2.5 yr", "outcome": "INVALID_INPUT"},
              {"q": "Return period: risk = 10%", "outcome": "INVALID_INPUT"}])


def geometric_mean(x):
    _need(len(x) >= 2, "give at least two values")
    s = 0.0
    t = 0.0
    for v in x:
        _pos("every value (a count of zero has no logarithm)", v)
        s = s + math.log(v)
        t = t + v
    m = len(x)
    mean = t / m
    d = 0.0
    for v in x:
        d = d + (v - mean) ** 2
    gm = math.exp(s / m)
    return _out(geometric_mean=gm, mean=mean, sd=math.sqrt(d / (m - 1)), mean_log10=s / m / math.log(10.0), count=float(m))


def _ref_geometric_mean(x):
    import numpy as np
    from scipy.stats import gmean
    a = np.array(x, float)
    return {"geometric_mean": float(gmean(a)), "mean": float(np.mean(a)), "sd": float(np.std(a, ddof=1)),
            "mean_log10": float(np.mean(np.log10(a))), "count": float(a.size)}


_add("enveng_geometric_mean_counts",
     name="Geometric mean of bacterial counts (with mean and standard deviation)", topic="water quality / microbiology",
     purpose="The geometric mean of a set of positive values such as coliform counts (MPN or CFU per 100 mL), the n-th root of their product, with the arithmetic mean, the sample standard deviation and the mean of the base-10 logarithms.",
     inputs=[_vecin("x", "positive values, e.g. coliform counts per 100 mL", 2, 500)],
     outputs=[_o("geometric_mean", "number", "geometric mean (prod x)^(1/n)"), _o("mean", "number", "arithmetic mean"),
              _o("sd", "number", "sample standard deviation (n - 1)"), _o("mean_log10", "number", "mean of log10 x"), _o("count", "number", "number of values")],
     primary="geometric_mean", conditions=["every value > 0", "at least two values"],
     method="GM = exp(mean(ln x)); mean = sum x / n; s = sqrt(sum (x - mean)^2/(n - 1))",
     latex=r"GM = \left(\prod_{i=1}^n x_i\right)^{1/n},\qquad s = \sqrt{\frac{\sum (x_i - \bar x)^2}{n - 1}}",
     run=geometric_mean, reference=_ref_geometric_mean,
     wrong=lambda x: (lambda r: _out(**dict(r, sd=r["sd"] * math.sqrt((len(x) - 1) / len(x)))))(geometric_mean(x)),
     rand=lambda rng: {"x": [_sig(_lu(rng, 1.0, 5000.0), 2) for _ in range(rng.randint(3, 12))]},
     phrases=("Geometric mean", "Geometric mean of coliform counts", "Geometric mean of counts", "Coliform geometric mean"),
     examples=[_ex({"x": [10.0, 100.0, 1000.0]}), _ex({"x": [20.0, 40.0, 35.0, 120.0, 8.0]})],
     level="beginner",
     extra_q=[{"q": "Geometric mean of coliform counts: x = [10, 100, 1000]", "like": 0},
              {"q": "Geometric mean: x = [10, 0, 1000]", "outcome": "INVALID_INPUT"}])


def vs_reduction(VS_in, VS_out):
    _frac("the volatile fraction of the feed VS_in", VS_in, lo_open=True)
    _frac("the volatile fraction of the digested solids VS_out", VS_out, hi_open=True)
    _need(VS_out <= VS_in, "the digested solids cannot be more volatile than the feed")
    vsr = (VS_in - VS_out) / (VS_in - VS_in * VS_out)
    return _out(VSR=vsr, VSR_percent=vsr * 100.0, TS_remaining=(1.0 - VS_in) / (1.0 - VS_out))


def _ref_vs_reduction(VS_in, VS_out):
    from fractions import Fraction as F
    a, b = F(VS_in), F(VS_out)
    fixed = 1 - a                                           # per unit of feed solids: fixed solids pass unchanged
    ts_out = fixed / (1 - b)
    vs_left = ts_out * b
    vsr = (a - vs_left) / a
    return {"VSR": float(vsr), "VSR_percent": float(vsr * 100), "TS_remaining": float(ts_out)}


_add("enveng_volatile_solids_reduction",
     name="Volatile solids reduction in digestion (Van Kleeck)", topic="biosolids / digestion",
     purpose="The fraction of the volatile solids destroyed by sludge digestion from the volatile fractions of the feed and the digested sludge by the Van Kleeck equation, VSR = (VS_in - VS_out)/(VS_in - VS_in VS_out), which assumes the fixed solids pass unchanged.",
     inputs=[_numin("VS_in", "volatile fraction of the feed solids (fraction or %)", pct=True),
             _numin("VS_out", "volatile fraction of the digested solids (fraction or %)", pct=True)],
     outputs=[_o("VSR", "number", "volatile solids reduction (fraction)"), _o("VSR_percent", "number", "volatile solids reduction, %"),
              _o("TS_remaining", "number", "total solids left per unit of feed solids")],
     primary="VSR", conditions=["fixed (inorganic) solids conserved", "0 < VS_out <= VS_in, VS_out < 1"],
     method="VSR = (VS_in - VS_out)/(VS_in - VS_in VS_out)",
     latex=r"VSR = \frac{VS_{in} - VS_{out}}{VS_{in} - VS_{in}\,VS_{out}}",
     run=vs_reduction, reference=_ref_vs_reduction,
     wrong=lambda VS_in, VS_out: _out(VSR=(VS_in - VS_out) / VS_in, VSR_percent=(VS_in - VS_out) / VS_in * 100.0,
                                      TS_remaining=(1.0 - VS_in) / (1.0 - VS_out)),
     rand=lambda rng: (lambda a: {"VS_in": a, "VS_out": _sig(a * rng.uniform(0.4, 0.95), 2)})(_sig(rng.uniform(0.55, 0.85), 2)),
     phrases=("Volatile solids reduction", "Van Kleeck", "VS reduction", "Volatile solids destruction"),
     aliases={"VSin": "VS_in", "VSout": "VS_out"},
     examples=[_ex({"VS_in": 0.75, "VS_out": 0.55}), _ex({"VS_in": 0.7, "VS_out": 0.5})],
     level="intermediate",
     extra_q=[{"q": "Volatile solids reduction: VS_in = 75%, VS_out = 55%", "like": 0},
              {"q": "Volatile solids reduction: VS_in = 0.5, VS_out = 0.7", "outcome": "INVALID_INPUT"}])


def transmembrane_pressure(P_feed, P_conc, P_perm):
    _fin(P_feed, P_conc, P_perm)
    _need(P_conc <= P_feed, "the concentrate pressure cannot exceed the feed pressure")
    tmp = (P_feed + P_conc) / 2.0 - P_perm
    _need(tmp > 0, "the transmembrane pressure must be positive (the permeate pressure is too high)")
    return _out(TMP=tmp, TMP_kPa=tmp / 1e3, TMP_bar=tmp / 1e5, dP=P_feed - P_conc, dP_kPa=(P_feed - P_conc) / 1e3)


def _ref_tmp(P_feed, P_conc, P_perm):
    # the feed-side pressure falls linearly along the channel: its mean over the length, less the permeate side
    mean_feed = _quad(lambda s: P_feed + (P_conc - P_feed) * s, 0.0, 1.0)
    t = mean_feed - P_perm
    return {"TMP": t, "TMP_kPa": t / 1000.0, "TMP_bar": t / 100000.0, "dP": P_feed - P_conc, "dP_kPa": (P_feed - P_conc) / 1000.0}


_add("enveng_transmembrane_pressure",
     name="Transmembrane pressure of a crossflow membrane", topic="water treatment / membranes",
     purpose="The transmembrane pressure of a crossflow membrane element, TMP = (P_feed + P_concentrate)/2 - P_permeate, and the feed-channel pressure drop.",
     inputs=[_phys("P_feed", "pressure", "feed pressure", show=("kPa", 1e-3)), _phys("P_conc", "pressure", "concentrate (retentate) pressure", show=("kPa", 1e-3)),
             _phys("P_perm", "pressure", "permeate pressure", show=("kPa", 1e-3))],
     outputs=[_o("TMP", "pressure", "transmembrane pressure"), _o("TMP_kPa", "number", "in kPa"), _o("TMP_bar", "number", "in bar"),
              _o("dP", "pressure", "feed-channel pressure drop P_feed - P_conc"), _o("dP_kPa", "number", "pressure drop, kPa")],
     primary="TMP", conditions=["the same pressure basis for all three (gauge or absolute)", "P_conc <= P_feed", "TMP > 0"],
     method="TMP = (P_feed + P_conc)/2 - P_perm", latex=r"TMP = \frac{P_f + P_c}{2} - P_p",
     run=transmembrane_pressure, reference=_ref_tmp,
     wrong=lambda P_feed, P_conc, P_perm: (lambda t: _out(TMP=t, TMP_kPa=t / 1e3, TMP_bar=t / 1e5, dP=P_feed - P_conc, dP_kPa=(P_feed - P_conc) / 1e3))((P_feed - P_conc) / 2.0 - P_perm),
     rand=lambda rng: (lambda pf: {"P_feed": pf, "P_conc": _sig(pf * rng.uniform(0.6, 0.98), 3), "P_perm": _sig(pf * rng.uniform(0.0, 0.3), 3)})(_sig(_lu(rng, 50.0, 6000.0), 3) * 1e3),
     phrases=("Transmembrane pressure", "TMP", "Membrane transmembrane pressure"),
     aliases={"Pf": "P_feed", "Pc": "P_conc", "Pp": "P_perm"},
     examples=[_ex({"P_feed": 800e3, "P_conc": 700e3, "P_perm": 50e3}), _ex({"P_feed": 200e3, "P_conc": 180e3, "P_perm": 0.0})],
     level="beginner",
     extra_q=[{"q": "TMP: P_feed = 8 bar, P_conc = 7 bar, P_perm = 0.5 bar", "like": 0},
              {"q": "Transmembrane pressure: P_feed = 700 kPa, P_conc = 800 kPa, P_perm = 50 kPa", "outcome": "INVALID_INPUT"},
              {"q": "Transmembrane pressure: P_feed = 200 kPa, P_conc = 180 kPa, P_perm = 300 kPa", "outcome": "INVALID_INPUT"}])


def thod_solution(formula, C, nitrification=None):
    nit = "no" if nitrification is None else nitrification
    _pos("the concentration C", C)
    r = thod(formula, nit)
    g = r["thod"]
    return _out(ThOD=g * C, ThOD_mgL=g * C * 1e3, thod_ratio=g, o2_per_mol=r["o2_per_mol"], molar_mass=r["molar_mass"],
                C_molL=C / r["molar_mass"])


def _ref_thod_solution(formula, C, nitrification=None):
    r = _ref_thod(formula, "no" if nitrification is None else nitrification)
    return {"ThOD": r["thod"] * C, "ThOD_mgL": r["thod"] * C * 1000.0, "thod_ratio": r["thod"], "o2_per_mol": r["o2_per_mol"],
            "molar_mass": r["molar_mass"], "C_molL": C / r["molar_mass"]}


def _wrong_thod_solution(formula, C, nitrification=None):
    r = _wrong_thod(formula, "no" if nitrification is None else nitrification)
    return _out(ThOD=r["thod"] * C, ThOD_mgL=r["thod"] * C * 1e3, thod_ratio=r["thod"], o2_per_mol=r["o2_per_mol"],
                molar_mass=r["molar_mass"], C_molL=C / r["molar_mass"])


_add("enveng_thod_solution",
     name="Theoretical oxygen demand of a solution (mg/L)", topic="water quality / oxygen demand",
     purpose="The theoretical oxygen demand in mg/L of a solution of an organic compound CaHbOcNd at a given concentration, its complete oxidation to CO2 and water (nitrogen to ammonia, or to nitrate with nitrification): ThOD = C x (g O2 per g compound).",
     inputs=[_textin("formula", "molecular formula with C, H, O, N only (e.g. C6H12O6)"),
             _phys("C", "mass_concentration", "concentration of the compound"),
             _choice("nitrification", ("yes", "no"), "include the oxygen to nitrify ammonia (default no)", required=False)],
     outputs=[_o("ThOD", "mass_concentration", "theoretical oxygen demand of the solution"), _o("ThOD_mgL", "number", "ThOD, mg/L"),
              _o("thod_ratio", "number", "g O2 per g compound"), _o("o2_per_mol", "number", "mol O2 per mol compound"),
              _o("molar_mass", "number", "molar mass, g/mol"), _o("C_molL", "number", "molar concentration of the compound, mol/L")],
     primary="ThOD", conditions=["elements C, H, O, N only, b >= 3d", "complete oxidation; nitrogen to NH3 unless nitrification = yes"],
     method="ThOD = C (a + b/4 - c/2 - 3d/4 [+ 2d]) M_O2 / M",
     latex=r"\text{ThOD} = C\,\frac{\left(a + \frac b4 - \frac c2 - \frac{3d}{4}\right)M_{O_2}}{M}",
     run=thod_solution, reference=_ref_thod_solution, wrong=_wrong_thod_solution,
     rand=lambda rng: dict({"formula": rng.choice(_THOD_LIST), "C": _sig(_lu(rng, 1.0, 2000.0), 3) * MG},
                           **({"nitrification": rng.choice(["yes", "no"])} if rng.random() < 0.6 else {})),
     phrases=("ThOD of a solution", "Theoretical oxygen demand of a solution", "Theoretical oxygen demand in mg/L", "ThOD in mg/L"),
     examples=[_ex({"formula": "C6H12O6", "C": 0.1}), _ex({"formula": "C5H7O2N", "C": 0.05, "nitrification": "yes"})],
     level="beginner",
     extra_q=[{"q": "Theoretical oxygen demand of a solution: formula = C6H12O6, C = 100 mg/L, nitrification = no", "like": 0},
              {"q": "ThOD of a solution: formula = C6H12O6, C = 0 mg/L", "outcome": "INVALID_INPUT"},
              {"q": "ThOD of a solution: formula = C6H12Cl, C = 100 mg/L", "outcome": "INVALID_INPUT"}])


# ================================================================================================ BATCH 3 (2026-11-11)
# first-order decay in a batch reactor (any one of C, C0, k, t), the chemical feed rate, the membrane permeate flux,
# the landfill volume per day and year; extended above: one CMFR with V and Q (enveng_cmfr_series), the effluent solids
# optional in the SRT (enveng_solids_retention_time), exam forms of the ultimate BOD and of the pump's input power.
LN2 = math.log(2.0)
LB = 0.45359237                   # kg per pound (exact)
MGD_SI = 3785.411784 / DAY        # m^3/s per million US gallons a day


def first_order_decay(C0=None, C=None, k=None, t=None, t_half=None):
    if t_half is not None:
        _need(k is None, "give the rate constant k or the half-life t_half, not both")
        _pos("the half-life t_half", t_half)
        k = LN2 / t_half
    known = 0
    for v in (C0, C, k, t):
        if v is not None:
            known = known + 1
    _need(known == 3, "give exactly three of C0, C, k (or t_half) and t: the fourth is computed")
    if C0 is not None:
        _pos("the initial concentration C0", C0)
    if C is not None:
        _pos("the concentration C", C)
    if k is not None:
        _pos("the rate constant k", k)
    if t is not None:
        _pos("the time t", t)
    if C0 is not None and C is not None:
        _need(C < C0, "a first-order decay needs C below C0")
    if k is not None and t is not None:
        _need(k * t <= 700.0, "k t is too large (above 700): nothing measurable is left")
    if C is None:
        C = C0 * math.exp(-k * t)
    elif C0 is None:
        C0 = C * math.exp(k * t)
    elif k is None:
        k = math.log(C0 / C) / t
    else:
        t = math.log(C0 / C) / k
    f = C / C0
    th = LN2 / k
    return _out(C=C, C_mgL=C * 1e3, C0=C0, C0_mgL=C0 * 1e3, k=k, k_per_day=k * DAY, k_per_h=k * HOUR,
                t=t, t_days=t / DAY, t_h=t / HOUR, t_half=th, t_half_days=th / DAY, t_half_h=th / HOUR,
                fraction_remaining=f, removal_percent=(1.0 - f) * 100.0)


def _ref_first_order_decay(C0=None, C=None, k=None, t=None, t_half=None):
    import mpmath as mp
    mp.mp.dps = 40
    if t_half is not None:                                  # the rate that halves the amount in t_half
        k = float(mp.findroot(lambda u: mp.exp(-u) - mp.mpf(1) / 2, mp.mpf("0.5")) / t_half)    # u = k t_half

    def surv(kk, tt):                                       # dC/dt = -k C integrated from 1
        return _ivp(lambda _s, y: [-kk * y[0]], tt, [1.0], atol=[1e-30])[0]

    def root(f, x0):                                        # f decreasing from f(0) > 0: bracket by doubling
        hi = x0
        while f(hi) > 0:
            hi = hi * 2.0
        return _brentq(f, 0.0, hi)
    if C is None:
        C = C0 * surv(k, t)
    elif C0 is None:
        C0 = C / surv(k, t)
    elif k is None:
        k = root(lambda kk: math.exp(-kk * t) - C / C0, 1.0 / t)
    else:
        t = root(lambda tt: math.exp(-k * tt) - C / C0, 1.0 / k)
    th = float(mp.log(2)) / k
    return {"C": C, "C_mgL": C * 1000.0, "C0": C0, "C0_mgL": C0 * 1000.0, "k": k, "k_per_day": k * 86400.0, "k_per_h": k * 3600.0,
            "t": t, "t_days": t / 86400.0, "t_h": t / 3600.0, "t_half": th, "t_half_days": th / 86400.0, "t_half_h": th / 3600.0,
            "fraction_remaining": C / C0, "removal_percent": (1.0 - C / C0) * 100.0}


def _wrong_first_order_decay(C0=None, C=None, k=None, t=None, t_half=None):
    if t_half is not None:
        k = LN2 / t_half
    if C is None:                                           # base 10 in place of base e
        C = C0 * 10.0 ** (-k * t)
    elif C0 is None:
        C0 = C * 10.0 ** (k * t)
    elif k is None:
        k = math.log10(C0 / C) / t
    else:
        t = math.log10(C0 / C) / k
    f = C / C0
    th = LN2 / k
    return _out(C=C, C_mgL=C * 1e3, C0=C0, C0_mgL=C0 * 1e3, k=k, k_per_day=k * DAY, k_per_h=k * HOUR,
                t=t, t_days=t / DAY, t_h=t / HOUR, t_half=th, t_half_days=th / DAY, t_half_h=th / HOUR,
                fraction_remaining=f, removal_percent=(1.0 - f) * 100.0)


def _rand_decay(rng):
    unit = rng.choice([DAY, HOUR])
    x = {"C0": _sig(rng.uniform(5.0, 500.0), 3) * MG, "k": _sig(_lu(rng, 0.01, 3.0), 3) / unit}
    x["t"] = _sig(_lu(rng, 0.05, 6.0) / (x["k"] * unit), 3) * unit
    x["C"] = _sig(x["C0"] * math.exp(-x["k"] * x["t"]), 3)
    del x[rng.choice(["C0", "C", "k", "t"])]
    if "k" in x and rng.random() < 0.3:
        x["t_half"] = _sig(LN2 / x.pop("k"), 3)
    return x


_add("enveng_first_order_decay",
     name="First-order decay in a batch reactor: C = C0 exp(-k t) and the half-life", topic="reactors / batch reactor kinetics",
     purpose="First-order decay in a batch (or plug-flow) reactor, C = C0 exp(-k t): the concentration after a time, the initial concentration, the rate constant or the time, whichever one of the four is not given, with the half-life t1/2 = ln 2 / k (the rate may be given as a half-life instead of k), the fraction remaining and the per cent removed.",
     inputs=[_phys("C0", "mass_concentration", "initial concentration", required=False),
             _phys("C", "mass_concentration", "concentration after the time t", required=False),
             _phys("k", "first_order_rate_constant", "first-order rate constant (base e)", required=False),
             _phys("t", "time", "reaction time", required=False, show=("h", 1.0 / HOUR)),
             _phys("t_half", "time", "half-life (in place of k)", required=False, show=("h", 1.0 / HOUR))],
     outputs=[_o("C", "mass_concentration", "concentration after the time t, C0 exp(-k t)"), _o("C_mgL", "number", "in mg/L"),
              _o("C0", "mass_concentration", "initial concentration"), _o("C0_mgL", "number", "initial concentration, mg/L"),
              _o("k", "first_order_rate_constant", "first-order rate constant ln(C0/C)/t"), _o("k_per_day", "number", "rate constant, 1/d"),
              _o("k_per_h", "number", "rate constant, 1/h"), _o("t", "time", "reaction time ln(C0/C)/k"), _o("t_days", "number", "reaction time, days"),
              _o("t_h", "number", "reaction time, hours"), _o("t_half", "time", "half-life ln 2/k"), _o("t_half_days", "number", "half-life, days"),
              _o("t_half_h", "number", "half-life, hours"), _o("fraction_remaining", "number", "fraction remaining C/C0"),
              _o("removal_percent", "number", "per cent removed (1 - C/C0) x 100")],
     primary="C", conditions=["first-order kinetics, constant k (batch reactor or plug flow)", "exactly three of C0, C, k (or t_half), t",
                              "0 < C < C0 when both are given", "k t <= 700"],
     method="C = C0 e^(-k t); C0 = C e^(k t); k = ln(C0/C)/t; t = ln(C0/C)/k; t1/2 = ln 2/k (k = ln 2/t1/2 when the half-life is given)",
     latex=r"C = C_0\,e^{-kt},\qquad k = \frac{\ln(C_0/C)}{t},\qquad t_{1/2} = \frac{\ln 2}{k}",
     run=first_order_decay, reference=_ref_first_order_decay, wrong=_wrong_first_order_decay, rand=_rand_decay,
     phrases=("First-order decay", "Batch reactor first-order decay", "First-order batch reactor", "Batch reactor",
              "Half-life", "Half life", "Exponential decay", "First-order reaction in a batch reactor"),
     aliases={"Co": "C0", "Ct": "C", "t12": "t_half", "half_life": "t_half"},
     examples=[_ex({"C0": 0.1, "k": 0.2 / DAY, "t": 5.0 * DAY}), _ex({"C0": 0.08, "C": 0.02, "t": 4.0 * HOUR})],
     level="beginner",
     extra_q=[{"q": "Batch reactor: C0 = 100 mg/L, k = 0.2 1/d, t = 120 h", "like": 0},
              {"q": "Half-life: C0 = 80 mg/L, C = 20 mg/L, t = 240 min", "like": 1},
              {"q": "First-order decay: C0 = 100 mg/L, C = 25 mg/L, t_half = 2 h", "outputs": {"t": 14400.0, "t_h": 4.0, "k_per_h": 0.34657359028}},
              {"q": "First-order decay: C = 36.7879441171 mg/L, k = 0.2 1/d, t = 5 d", "outputs": {"C0_mgL": 100.0}},
              {"q": "First-order decay: C0 = 100 mg/L, C = 50 mg/L, k = 0.1 1/d", "outputs": {"t_days": 6.93147180560, "t_half_days": 6.93147180560}},
              {"q": "First-order decay: C0 = 100 mg/L, C = 50 mg/L, k = 0.1 1/d, t = 5 d", "outcome": "INVALID_INPUT"},
              {"q": "First-order decay: C0 = 20 mg/L, C = 50 mg/L, t = 5 d", "outcome": "INVALID_INPUT"},
              {"q": "First-order decay: C0 = 100 mg/L, k = 0.2 1/d", "outcome": "INVALID_INPUT"},
              {"q": "First-order decay: C0 = 100 mg/L, k = 0.2 1/d, t_half = 3 d, t = 5 d", "outcome": "INVALID_INPUT"}])


def chemical_feed(Q, dose, purity=None):
    _pos("the flow Q and the dose", Q, dose)
    p = 1.0 if purity is None else purity
    _frac("the purity (active fraction of the product)", p, lo_open=True)
    active = Q * dose
    feed = active / p
    return _out(feed=feed, feed_kg_per_day=feed * DAY, feed_lb_per_day=feed * DAY / LB, active_kg_per_day=active * DAY,
                active_lb_per_day=active * DAY / LB, Q_MGD=Q / MGD_SI)


def _ref_chemical_feed(Q, dose, purity=None):
    from fractions import Fraction as F
    # the US "pounds formula": lb/d = MGD x mg/L x 8.3454 (the weight of a million gallons of water in pounds, per mg/L)
    mgd = F(Q) * 86400 / F("3785.411784")
    lb_active = mgd * F(dose) * 1000 * F("3.785411784") / F("0.45359237")
    lb = lb_active / (F(1) if purity is None else F(purity))
    kg = lb * F("0.45359237")
    return {"feed": float(kg / 86400), "feed_kg_per_day": float(kg), "feed_lb_per_day": float(lb),
            "active_kg_per_day": float(lb_active * F("0.45359237")), "active_lb_per_day": float(lb_active), "Q_MGD": float(mgd)}


def _wrong_chemical_feed(Q, dose, purity=None):
    p = 1.0 if purity is None else purity
    feed = Q * dose * p                                     # the purity multiplied instead of divided
    return _out(feed=feed, feed_kg_per_day=feed * DAY, feed_lb_per_day=feed * DAY / LB, active_kg_per_day=Q * dose * DAY,
                active_lb_per_day=Q * dose * DAY / LB, Q_MGD=Q / MGD_SI)


def _rand_feed(rng):
    if rng.random() < 0.5:
        q = _sig(_lu(rng, 100.0, 500000.0), 3) / DAY
    else:
        q = _sig(_lu(rng, 0.05, 100.0), 3) * MGD_SI
    x = {"Q": q, "dose": _sig(_lu(rng, 0.2, 200.0), 3) * MG}
    if rng.random() < 0.85:
        x["purity"] = _sig(rng.uniform(0.05, 0.98), 2)
    return x


_add("enveng_chemical_feed_rate",
     name="Chemical feed rate: mass per day = Q x dose / purity (kg/d and lb/d)", topic="water treatment / chemical dosing",
     purpose="The feed rate of a treatment chemical (coagulant, chlorine product, lime, polymer, ...) for a flow and a dose, mass per day = Q x dose / purity, in kg/d and lb/d, with the active chemical alone; the flow in m^3/d or MGD, the dose in mg/L, the purity (active fraction of the product) as a fraction or %, 1 when not given.",
     inputs=[_phys("Q", "flow_rate", "flow treated (m^3/d, or MGD)", show=("m^3/d", DAY)),
             _phys("dose", "mass_concentration", "dose of the active chemical"),
             _numin("purity", "active fraction of the product (fraction or %; default 1)", required=False, pct=True)],
     outputs=[_o("feed", "mass_flow_rate", "feed rate of the product Q dose/purity"), _o("feed_kg_per_day", "number", "product feed, kg/d"),
              _o("feed_lb_per_day", "number", "product feed, lb/d"), _o("active_kg_per_day", "number", "active chemical fed, kg/d"),
              _o("active_lb_per_day", "number", "active chemical fed, lb/d"), _o("Q_MGD", "number", "flow, million US gallons a day")],
     primary="feed", conditions=["0 < purity <= 1 (1 unless given)", "dose as the active chemical", "1 lb = 0.45359237 kg, 1 US gal = 3.785411784 L"],
     method="feed = Q x dose / purity; lb/d = kg/d / 0.45359237 (= 8.3454 x MGD x mg/L / purity)",
     latex=r"\dot m = \frac{Q\cdot\text{dose}}{p},\qquad \dot m\,[\mathrm{lb/d}] = \frac{8.3454\,Q[\mathrm{MGD}]\,\text{dose}[\mathrm{mg/L}]}{p}",
     run=chemical_feed, reference=_ref_chemical_feed, wrong=_wrong_chemical_feed, rand=_rand_feed,
     phrases=("Chemical feed rate", "Chemical feed", "Chemical dosage feed rate", "Chemical dose feed rate", "Feed rate of a chemical",
              "Coagulant feed rate", "Pounds formula", "Chemical feed rate in lb/d", "Chemical feed rate in kg/d"),
     aliases={"dosage": "dose", "D": "dose", "flow": "Q"},
     examples=[_ex({"Q": 10000.0 / DAY, "dose": 0.025}), _ex({"Q": 2.0 * MGD_SI, "dose": 0.01, "purity": 0.48})],
     level="beginner",
     extra_q=[{"q": "Chemical feed rate: Q = 2 MGD, dose = 10 mg/L, purity = 48%", "like": 1},
              {"q": "Pounds formula: Q = 2 mgd, dose = 10 mg/L, purity = 0.48", "like": 1},
              {"q": "Coagulant feed rate: Q = 10000 m^3/day, dose = 25 mg/L, purity = 100%", "like": 0},
              {"q": "Chemical feed rate: Q = 10000 m^3/d, dose = 25 mg/L, purity = 0%", "outcome": "INVALID_INPUT"},
              {"q": "Chemical feed rate: Q = 10000 m^3/d, dose = 25 mg/L, purity = 120%", "outcome": "INVALID_INPUT"},
              {"q": "Chemical feed rate: Q = 10000 m^3/d, dose = 0 mg/L", "outcome": "INVALID_INPUT"}])


def membrane_flux(Qp=None, A=None, J=None):
    known = 0
    for v in (Qp, A, J):
        if v is not None:
            known = known + 1
    _need(known == 2, "give exactly two of the permeate flow Qp, the membrane area A and the flux J")
    if Qp is not None:
        _pos("the permeate flow Qp", Qp)
    if A is not None:
        _pos("the membrane area A", A)
    if J is not None:
        _pos("the flux J", J)
    if J is None:
        J = Qp / A
    elif A is None:
        A = Qp / J
    else:
        Qp = J * A
    return _out(J=J, J_LMH=J * 3.6e6, J_m_per_day=J * DAY, A=A, Qp=Qp, Qp_m3_per_day=Qp * DAY)


def _ref_membrane_flux(Qp=None, A=None, J=None):
    from fractions import Fraction as F
    # in litres per hour and square metres: LMH = (L/h)/m^2
    if J is None:
        lmh = F(Qp) * 1000 * 3600 / F(A)
        a = F(A)
    else:
        lmh = F(J) * 1000 * 3600
        a = F(A) if A is not None else F(Qp) * 1000 * 3600 / lmh
    lph = lmh * a
    return {"J": float(lmh / 3600000), "J_LMH": float(lmh), "J_m_per_day": float(lmh * 24 / 1000), "A": float(a),
            "Qp": float(lph / 3600000), "Qp_m3_per_day": float(lph * 24 / 1000)}


def _wrong_membrane_flux(Qp=None, A=None, J=None):
    r = membrane_flux(Qp, A, J)
    return _out(**dict(r, J_LMH=r["J"] * 3600.0))           # m^3 taken as litres: the factor 1000 forgotten


_add("enveng_membrane_flux",
     name="Membrane permeate flux J = Qp/A (LMH and m/d), or the area for a flux", topic="water treatment / membranes",
     purpose="The permeate flux of a membrane, J = Q_permeate / A, in L/(m^2 h) (LMH) and m/d, from the permeate flow and the membrane area; or the membrane area needed for a design flux, or the permeate flow a flux gives (any two of the three).",
     inputs=[_phys("Qp", "flow_rate", "permeate flow", required=False, show=("m^3/d", DAY)),
             _phys("A", "area", "membrane area", required=False),
             _phys("J", "speed", "permeate flux (L/(m^2 h) or LMH, m/d)", required=False, show=("L/(m^2*h)", 3.6e6))],
     outputs=[_o("J", "speed", "permeate flux Qp/A"), _o("J_LMH", "number", "flux, L/(m^2 h) (LMH)"), _o("J_m_per_day", "number", "flux, m/d"),
              _o("A", "area", "membrane area Qp/J"), _o("Qp", "flow_rate", "permeate flow J A"), _o("Qp_m3_per_day", "number", "permeate flow, m^3/d")],
     primary="J", conditions=["exactly two of Qp, A, J", "the flux of the permeate (product water) over the active membrane area"],
     method="J = Qp/A; A = Qp/J; Qp = J A; 1 m/s = 3.6e6 L/(m^2 h)",
     latex=r"J = \frac{Q_p}{A}",
     run=membrane_flux, reference=_ref_membrane_flux, wrong=_wrong_membrane_flux,
     rand=lambda rng: (lambda q, a, j: [{"Qp": q, "A": a}, {"Qp": q, "J": j}, {"A": a, "J": j}][rng.randint(0, 2)])(
         _sig(_lu(rng, 10.0, 100000.0), 3) / DAY, _sig(_lu(rng, 10.0, 50000.0), 3), _sig(rng.uniform(5.0, 80.0), 3) / 3.6e6),
     phrases=("Membrane flux", "Permeate flux", "Membrane permeate flux", "Flux of a membrane", "Membrane area for a flux",
              "Required membrane area"),
     aliases={"Q": "Qp", "flux": "J", "area": "A"},
     examples=[_ex({"Qp": 750.0 / DAY, "A": 1500.0}), _ex({"Qp": 2400.0 / DAY, "J": 25.0 / 3.6e6})],
     level="beginner",
     weak="exact rational arithmetic of the same definition, in litres per hour and square metres",
     extra_q=[{"q": "Permeate flux: Qp = 750 m^3/day, A = 1500 m^2", "like": 0},
              {"q": "Required membrane area: Qp = 100 m^3/h, J = 25 LMH", "like": 1},
              {"q": "Membrane flux: Qp = 2400 m^3/d, J = 0.6 m/d", "outputs": {"A": 4000.0, "J_LMH": 25.0}},
              {"q": "Membrane flux: Qp = 750 m^3/d, A = 1500 m^2, J = 20 LMH", "outcome": "INVALID_INPUT"},
              {"q": "Membrane flux: Qp = 750 m^3/d", "outcome": "INVALID_INPUT"},
              {"q": "Membrane flux: Qp = 750 m^3/d, A = 0 m^2", "outcome": "INVALID_INPUT"}])


def landfill_volume(m, rho, cover=None, capacity=None):
    _pos("the waste mass rate m and the compacted density rho", m, rho)
    c = 0.0 if cover is None else cover
    _fin(c)
    _need(0.0 <= c <= 1.0, "the cover ratio must lie between 0 and 1 (cover soil per volume of compacted waste)")
    Vw = m / rho
    V = Vw * (1.0 + c)
    out = {"V": V, "V_m3_per_day": V * DAY, "V_m3_per_year": V * DAY * 365.0, "V_waste_m3_per_day": Vw * DAY,
           "V_cover_m3_per_day": Vw * c * DAY}
    if capacity is not None:
        _pos("the landfill capacity", capacity)
        out["life_years"] = capacity / (V * DAY * 365.0)
        out["life_days"] = capacity / (V * DAY)
    return _out(**out)


def _ref_landfill_volume(m, rho, cover=None, capacity=None):
    c = 0.0 if cover is None else cover

    def rate(s):                                            # waste and cover soil placed per second
        return m / rho + c * (m / rho)
    day = _quad(rate, 0.0, 86400.0)
    year = _quad(rate, 0.0, 365.0 * 86400.0)
    out = {"V": day / 86400.0, "V_m3_per_day": day, "V_m3_per_year": year, "V_waste_m3_per_day": _quad(lambda s: m / rho, 0.0, 86400.0),
           "V_cover_m3_per_day": day - _quad(lambda s: m / rho, 0.0, 86400.0)}
    if capacity is not None:
        n = _brentq(lambda yrs: yrs * year - capacity, 0.0, 2.0 * capacity / year + 1.0)    # years until it is full
        out["life_years"] = n
        out["life_days"] = n * 365.0
    return out


def _wrong_landfill_volume(m, rho, cover=None, capacity=None):
    c = 0.0 if cover is None else cover
    V = m / (rho * (1.0 + c))                               # the cover put on the density instead of the volume
    out = {"V": V, "V_m3_per_day": V * DAY, "V_m3_per_year": V * DAY * 365.0, "V_waste_m3_per_day": m / rho * DAY,
           "V_cover_m3_per_day": m / rho * c * DAY}
    if capacity is not None:
        out["life_years"] = capacity / (V * DAY * 365.0)
        out["life_days"] = capacity / (V * DAY)
    return _out(**out)


def _rand_landfill_volume(rng):
    x = {"m": _sig(_lu(rng, 5.0, 5000.0), 3) * 1000.0 / DAY, "rho": _sig(rng.uniform(300.0, 1000.0), 3)}
    if rng.random() < 0.85:
        x["cover"] = _sig(rng.uniform(0.1, 0.5), 2)
    if rng.random() < 0.6:
        x["capacity"] = _sig(_lu(rng, 1e4, 2e7), 3)
    return x


_add("enveng_landfill_daily_volume",
     name="Landfill volume per day and per year with cover soil, and the landfill life", topic="solid waste / landfills",
     purpose="The landfill volume used each day and each year by waste placed at its compacted density with daily cover soil, V = mass / compacted density x (1 + cover ratio), and, for a given capacity, the life of the landfill (constant waste rate).",
     inputs=[_phys("m", "mass_flow_rate", "waste placed per day", show=("t/d", DAY / 1000.0)),
             _phys("rho", "density", "compacted (in-place) density of the waste"),
             _numin("cover", "cover soil volume per volume of compacted waste (fraction or %; default 0)", required=False, pct=True),
             _phys("capacity", "volume", "available landfill volume (for the life)", required=False)],
     outputs=[_o("V", "flow_rate", "landfill volume used, waste plus cover"), _o("V_m3_per_day", "number", "landfill volume used per day, m^3"),
              _o("V_m3_per_year", "number", "landfill volume used per 365-day year, m^3"),
              _o("V_waste_m3_per_day", "number", "compacted waste alone per day, m^3"), _o("V_cover_m3_per_day", "number", "cover soil per day, m^3"),
              _o("life_years", "number", "landfill life, years"), _o("life_days", "number", "landfill life, days")],
     primary="V", conditions=["constant waste rate, every day of a 365-day year", "cover as a volume ratio to the compacted waste, 0 to 1",
                              "no settlement or decomposition"],
     method="V = m/rho (1 + cover); per year x 365; life = capacity / yearly volume",
     latex=r"V = \frac{\dot m}{\rho_c}\,(1 + c),\qquad n = \frac{V_{cap}}{365\,V_{day}}",
     run=landfill_volume, reference=_ref_landfill_volume, wrong=_wrong_landfill_volume, rand=_rand_landfill_volume,
     phrases=("Landfill daily volume", "Landfill volume per day", "Daily landfill volume", "Landfill volume required",
              "Landfill volume with cover", "Landfill volume per year", "Landfill volume and life"),
     aliases={"mass": "m", "M": "m", "density": "rho", "cover_ratio": "cover", "V_capacity": "capacity"},
     examples=[_ex({"m": 200.0 * 1000.0 / DAY, "rho": 600.0, "cover": 0.25}),
               _ex({"m": 500.0 * 1000.0 / DAY, "rho": 800.0, "cover": 0.2, "capacity": 2.0e6})],
     level="beginner",
     extra_q=[{"q": "Landfill volume per year: m = 200 tonne/day, rho = 600 kg/m^3, cover = 25%", "like": 0},
              {"q": "Landfill volume and life: m = 500 t/d, rho = 800 kg/m^3, cover = 0.2, capacity = 2000000 m^3", "like": 1},
              {"q": "Landfill daily volume: m = 200 t/d, rho = 600 kg/m^3", "outputs": {"V_m3_per_day": 333.333333333}},
              {"q": "Landfill daily volume: m = 200 t/d, rho = 600 kg/m^3, cover = 150%", "outcome": "INVALID_INPUT"},
              {"q": "Landfill daily volume: m = 200 t/d, rho = 0 kg/m^3, cover = 0.2", "outcome": "INVALID_INPUT"}])


# the worked examples' outputs (computed by their references on the build machine): filled in below
_EXV: dict = {
    'enveng_stability_class': [{'stability': 'A-B'}, {'stability': 'E'}],
    'enveng_briggs_dispersion_coefficients': [{'sigma_y': 76.2770071396, 'sigma_z': 37.947331922}, {'sigma_y': 565.685424949, 'sigma_z': 1122.49721603}],
    'enveng_gaussian_plume_concentration': [{'C': 1.04280565887e-06, 'C_ugm3': 1042.80565887}, {'C': 1.04886709087e-06, 'C_ugm3': 1048.86709087}],
    'enveng_ground_centerline_concentration': [{'C': 9.68813343132e-07, 'C_ugm3': 968.813343132}, {'C': 8.84194128288e-07, 'C_ugm3': 884.194128288}],
    'enveng_plume_concentration_stability': [{'C': 3.93568708537e-07, 'C_ugm3': 393.568708537, 'sigma_y': 200.831604419, 'sigma_z': 135.224680757}, {'C': 4.21799828485e-07, 'C_ugm3': 421.799828485, 'sigma_y': 111.409699812, 'sigma_z': 100.578969135}],
    'enveng_max_ground_concentration_ratio': [{'C_max': 2.34199326097e-07, 'C_max_ugm3': 234.199326097, 'sigma_z_max': 70.7106781187}, {'C_max': 2.23046977236e-06, 'C_max_ugm3': 2230.46977236, 'sigma_z_max': 106.066017178}],
    'enveng_max_ground_concentration_briggs': [{'x_max': 459.39307575, 'C_max': 1.33356818615e-06, 'C_max_ugm3': 1333.56818615, 'sigma_y': 49.4110278085, 'sigma_z': 35.1712053156}, {'x_max': 431.271622094, 'C_max': 2.44678825472e-06, 'C_max_ugm3': 2446.78825472, 'sigma_y': 63.7254392202, 'sigma_z': 56.8144210912}],
    'enveng_briggs_plume_rise': [{'F': 104.0205375, 'x_f': 762.771747653, 'dh': 125.637535501, 'H': 225.637535501}, {'F': 5.2392027625, 'x_f': 137.954805754, 'dh': 24.7311903107, 'H': 54.7311903107}],
    'enveng_briggs_stable_plume_rise': [{'F': 104.0205375, 's': 0.000681017361111, 'dh': 87.5535286851, 'H': 187.553528685}, {'F': 21.452046875, 's': 0.00124811909091, 'dh': 49.4383819899, 'H': 109.43838199}],
    'enveng_holland_plume_rise': [{'dh': 14.9676525, 'H': 64.9676525}, {'dh': 71.2971851852, 'H': 221.297185185}],
    'enveng_wind_power_law': [{'u2': 5.65015017849}, {'u2': 4.68050750881}],
    'enveng_ppm_to_mass_concentration': [{'C': 3.66609115103e-07, 'C_ugm3': 366.609115103, 'C_mgm3': 0.366609115103}, {'C': 9.82551085915e-06, 'C_ugm3': 9825.51085915, 'C_mgm3': 9.82551085915}],
    'enveng_mass_concentration_to_ppm': [{'ppm': 0.0531787238557, 'ppb': 53.1787238557}, {'ppm': 9.50379786354, 'ppb': 9503.79786354}],
    'enveng_box_model': [{'C': 4.54357413773e-08, 'C_ss': 5.33333333333e-08, 'C_ugm3': 45.4357413773}, {'C': 1.37622867001e-07, 'C_ss': 2.6e-07, 'C_ugm3': 137.622867001}],
    'enveng_settling_chamber_efficiency': [{'vt': 0.151246389043, 'eta': 0.151246389043, 'Re': 0.504154630144}, {'vt': 0.0662293432973, 'eta': 0.119212817935, 'Re': 0.128878722092}],
    'enveng_esp_efficiency': [{'eta': 0.917915001376, 'penetration': 0.0820849986239}, {'eta': 0.698805788088, 'penetration': 0.301194211912}],
    'enveng_esp_area': [{'A': 9210.34037198}, {'A': 5616.99801291}],
    'enveng_cyclone_lapple': [{'d50': 6.9126486021e-06, 'eta': 0.676659984169}, {'d50': 6.64723615568e-06, 'eta': 0.361346094226}],
    'enveng_so2_emission': [{'E': 0.0399587088726, 'E_uncontrolled': 0.399587088726, 'E_kg_per_day': 3452.43244659}, {'E': 0.0399587088726, 'E_uncontrolled': 0.0399587088726, 'E_kg_per_day': 3452.43244659}],
    'enveng_plume_mixing_lid': [{'C': 2.2195314983e-08, 'C_ugm3': 22.195314983, 'C_mixed': 2.21634600223e-08}, {'C': 1.83184642955e-07, 'C_ugm3': 183.184642955, 'C_mixed': 8.05558576673e-08}],
    'enveng_stack_plume_ground_concentration': [{'H': 77.4471996331, 'sigma_y': 289.429146371, 'sigma_z': 189.73665961, 'C': 8.88844532169e-08, 'C_ugm3': 88.8844532169}, {'H': 47.6492147684, 'sigma_y': 238.5139176, 'sigma_z': 221.359436212, 'C': 7.36354105169e-08, 'C_ugm3': 73.6354105169}],
    'enveng_bod_exerted': [{'BOD_t': 0.170840807655, 'L_t': 0.0791591923448, 'BOD_t_mgL': 170.840807655}, {'BOD_t': 0.00777545337955, 'L_t': 0.0222245466205, 'BOD_t_mgL': 7.77545337955}],
    'enveng_ultimate_bod': [{'L0': 0.292670121889, 'L0_mgL': 292.670121889}, {'L0': 0.0227430616128, 'L0_mgL': 22.7430616128}],
    'enveng_rate_constant_temperature': [{'kT': 3.34924950558e-06, 'kT_per_day': 0.289375157283}, {'kT': 3.65213382047e-06, 'kT_per_day': 0.315544362088}],
    'enveng_bod_dilution_test': [{'BOD': 0.09, 'BOD_mgL': 90.0}, {'BOD': 0.2306, 'BOD_mgL': 230.6}],
    'enveng_bod_rate_fit': [{'k': 3.00244020212e-06, 'k_per_day': 0.259410833463, 'L0': 0.181571752714, 'L0_mgL': 181.571752714, 'rss': 0.532713769663}, {'k': 2.91963756763e-06, 'k_per_day': 0.252256685844, 'L0': 0.02807538814, 'L0_mgL': 28.07538814, 'rss': 0.201668959591}],
    'enveng_theoretical_oxygen_demand': [{'thod': 1.0657037672, 'o2_per_mol': 6.0, 'molar_mass': 180.15588}, {'thod': 1.98021864202, 'o2_per_mol': 7.0, 'molar_mass': 113.11458}],
    'enveng_streeter_phelps_deficit': [{'D': 0.00272616508531, 'DO': 0.00637383491469, 'D_mgL': 2.72616508531, 'DO_mgL': 6.37383491469}, {'D': 0.00325190357327, 'DO': 0.00514809642673, 'D_mgL': 3.25190357327, 'DO_mgL': 5.14809642673}],
    'enveng_oxygen_sag_distance': [{'D': 0.00237812321105, 'DO': 0.00672187678895, 'D_mgL': 2.37812321105, 'DO_mgL': 6.72187678895, 't': 100000.0}, {'D': 0.0025926233295, 'DO': 0.0062073766705, 'D_mgL': 2.5926233295, 'DO_mgL': 6.2073766705, 't': 75000.0}],
    'enveng_streeter_phelps_critical_point': [{'t_c': 220803.338468, 't_c_days': 2.55559419523, 'D_c': 0.00277777777778, 'DO_min': 0.00632222222222, 'DO_min_mgL': 6.32222222222}, {'t_c': 126100.953981, 't_c_days': 1.45950178219, 'D_c': 0.0036, 'DO_min': 0.0052, 'DO_min_mgL': 5.2}],
    'enveng_reaeration_coefficient': [{'ka': 8.80834754085e-06, 'ka_per_day': 0.76104122753}, {'ka': 2.36442926549e-05, 'ka_per_day': 2.04286688538}],
    'enveng_do_saturation': [{'DOsat': 0.00909242604289, 'DOsat_mgL': 9.09242604289}, {'DOsat': 0.00902425916979, 'DOsat_mgL': 9.02425916979}],
    'enveng_mixing_two_streams': [{'C': 0.02, 'C_mgL': 20.0, 'Q': 5.5}, {'C': 0.00455357142857, 'C_mgL': 4.55357142857, 'Q': 11.2}],
    'enveng_mixing_several_streams': [{'C_mix': 30.2857142857, 'Q_total': 3.5}, {'C_mix': 15.7142857143, 'Q_total': 14.0}],
    'enveng_lake_steady_state': [{'C': 3.49162011173e-05, 'C_mgL': 0.0349162011173, 'tau': 1000000.0}, {'C': 3.92772977219e-05, 'C_mgL': 0.0392772977219, 'tau': 1666666.66667}],
    'enveng_lake_transient': [{'C': 2.65771374756e-05, 'C_mgL': 0.0265771374756, 'C_ss': 3.49162011173e-05}, {'C': 0.000882359920549, 'C_mgL': 0.882359920549, 'C_ss': 4.11184210526e-06}],
    'enveng_river_first_order_decay': [{'C': 0.0114750684147, 'C_mgL': 11.4750684147, 't': 160000.0, 'fraction_remaining': 0.573753420737}, {'C': 0.00396678693585, 'C_mgL': 3.96678693585, 't': 20000.0, 'fraction_remaining': 0.793357387171}],
    'enveng_pollutant_load': [{'W': 0.015, 'W_kg_per_day': 1296.0, 'W_t_per_year': 473.364}, {'W': 0.0008, 'W_kg_per_day': 69.12, 'W_t_per_year': 25.24608}],
    'enveng_detention_time': [{'theta': 7200.0, 'theta_h': 2.0}, {'theta': 5000.0, 'theta_h': 1.38888888889}],
    'enveng_overflow_rate_circular': [{'A': 314.159265359, 'v0': 0.00036841422012, 'v0_m_per_day': 31.8309886184}, {'A': 962.112750162, 'v0': 0.000519689610096, 'v0_m_per_day': 44.9011823123}],
    'enveng_sedimentation_basin_sizing': [{'A': 375.0, 'V': 1250.0, 'depth': 3.33333333333, 'D': 21.8509686118}, {'A': 1333.33333333, 'V': 5000.0, 'depth': 3.75, 'D': 41.2025815491}],
    'enveng_weir_loading_rate': [{'weir_length': 62.8318530718, 'WL': 0.0018420711006, 'WL_m3_per_m_day': 159.154943092}, {'weir_length': 100.530964915, 'WL': 0.00345388331363, 'WL_m3_per_m_day': 298.415518297}],
    'enveng_stokes_settling_velocity': [{'vs': 0.00224531831753, 'Re': 0.111840156914, 'vs_m_per_h': 8.08314594311}, {'vs': 3.33974665476e-05, 'Re': 0.000510902024601, 'vs_m_per_h': 0.120230879572}],
    'enveng_terminal_settling_velocity': [{'vt': 0.0905805628428, 'Re': 45.1185218711, 'Cd': 1.3185581361}, {'vt': 0.148638452472, 'Re': 296.741806514, 'Cd': 0.59503176301}],
    'enveng_ideal_basin_removal': [{'v0': 0.000289351851852, 'R': 0.48}, {'v0': 0.000771604938272, 'R': 0.72}],
    'enveng_filtration_rate': [{'v': 0.00154320987654, 'v_m_per_h': 5.55555555556, 'v_one_out': 0.00185185185185, 'v_one_out_m_per_h': 6.66666666667}, {'v': 0.00115740740741, 'v_m_per_h': 4.16666666667, 'v_one_out': 0.00154320987654, 'v_one_out_m_per_h': 5.55555555556}],
    'enveng_filter_clean_bed_headloss': [{'dP': 7414.17092785, 'h': 0.757398347116, 'Re': 1.56810453168}, {'dP': 4013.08926485, 'h': 0.409344021956, 'Re': 3.47673367184}],
    'enveng_backwash_bed_expansion': [{'eps_e': 0.674227907706, 'Le': 1.24627004462, 'expansion': 0.780385778031}, {'eps_e': 0.627219965337, 'Le': 1.60952825851, 'expansion': 0.609528258513}],
    'enveng_velocity_gradient': [{'G': 199.800299501}, {'G': 30.9255343747, 'Gt': 55665.9618744}],
    'enveng_mixing_power': [{'P': 4909.8}, {'P': 1045.6}],
    'enveng_disinfection_ct': [{'t10': 2160.0, 'CT': 2.16, 'CT_mg_min_L': 36.0}, {'t10': 3780.0, 'CT': 3.024, 'CT_mg_min_L': 50.4}],
    'enveng_ct_log_inactivation': [{'CT': 2.88, 'ratio': 0.461538461538, 'log_inactivation': 1.38461538462}, {'CT': 2.7, 'ratio': 0.642857142857, 'log_inactivation': 1.92857142857}],
    'enveng_chick_law': [{'N': 2478.75217667, 'survival': 0.00247875217667, 'log_removal': 2.60576689142, 'percent_removal': 99.7521247823}, {'N': 1839.39720586, 'survival': 0.367879441171, 'log_removal': 0.434294481903, 'percent_removal': 63.2120558829}],
    'enveng_chick_watson': [{'survival': 0.0497870683679, 'log_inactivation': 1.30288344571}, {'survival': 0.00538949613555, 'log_inactivation': 2.26845183514}],
    'enveng_log_removal': [{'log_removal': 4.0, 'percent_removal': 99.99}, {'log_removal': 1.79588001734, 'percent_removal': 98.4}],
    'enveng_chlorine_dose_feed': [{'dose': 0.003, 'dose_mgL': 3.0, 'feed': 0.000694444444444, 'feed_kg_per_day': 60.0, 'product_kg_per_day': 60.0}, {'dose': 0.0025, 'dose_mgL': 2.5, 'feed': 0.000144675925926, 'feed_kg_per_day': 12.5, 'product_kg_per_day': 19.2307692308}],
    'enveng_hypochlorous_acid_fraction': [{'alpha_HOCl': 0.776154575342, 'alpha_OCl': 0.223845424658, 'pKa': 7.54}, {'alpha_HOCl': 0.276273046828, 'alpha_OCl': 0.723726953172, 'pKa': 7.58176377025}],
    'enveng_water_hardness': [{'total_hardness': 211.607492876, 'calcium_hardness': 149.838165577, 'magnesium_hardness': 61.7693272989, 'classification': 'very hard'}, {'total_hardness': 46.4394537285, 'calcium_hardness': 29.9676331154, 'magnesium_hardness': 16.471820613, 'classification': 'soft'}],
    'enveng_freundlich_carbon_dose': [{'qe': 50.0, 'dose': 0.18, 'dose_mgL': 180.0}, {'qe': 95.2440631181, 'dose': 0.0472470393711, 'dose_mgL': 47.2470393711}],
    'enveng_langmuir_carbon_dose': [{'qe': 66.6666666667, 'dose': 0.135, 'dose_mgL': 135.0}, {'qe': 163.636363636, 'dose': 0.0977777777778, 'dose_mgL': 97.7777777778}],
    'enveng_freundlich_fit': [{'K': 39.4762342855, 'inv_n': 0.483576413036, 'n': 2.06792550886, 'r2': 0.999809339847}, {'K': 11.1445806716, 'inv_n': 0.433303339928, 'n': 2.30785204694, 'r2': 0.999278484569}],
    'enveng_langmuir_fit': [{'qmax': 177.900483764, 'b': 0.589675737336, 'r2': 0.999972823331}, {'qmax': 102.998151779, 'b': 0.118021164379, 'r2': 0.999894329783}],
    'enveng_membrane_mass_balance': [{'recovery': 0.75, 'rejection': 0.975, 'J': 5.78703703704e-06, 'J_LMH': 20.8333333333, 'Qc': 0.00289351851852, 'Cc': 7.85, 'Cc_mgL': 7850.0}, {'recovery': 0.45, 'rejection': 0.994285714286, 'J': 4.34027777778e-06, 'J_LMH': 15.625, 'Qc': 0.0318287037037, 'Cc': 63.4727272727, 'Cc_mgL': 63472.7272727}],
    'enveng_osmotic_pressure': [{'pi': 2969318.82396, 'pi_bar': 29.6931882396}, {'pi': 131774.268328, 'pi_bar': 1.31774268328}],
    'enveng_food_to_microorganism_ratio': [{'FM': 3.08641975309e-06, 'FM_per_day': 0.266666666667}, {'FM': 2.31481481481e-06, 'FM_per_day': 0.2}],
    'enveng_solids_retention_time': [{'SRT': 564214.192425, 'SRT_days': 6.53025685677}, {'SRT': 598844.820947, 'SRT_days': 6.93107431652},
                                     {'SRT': 648000.0, 'SRT_days': 7.5}],      # batch 3: 3000 x 2500/(100 x 10000) = 7.5 d, Xe = 0
    'enveng_hrt_organic_loading': [{'HRT': 25920.0, 'HRT_h': 7.2, 'OLR': 7.71604938272e-06, 'OLR_kg_per_m3_day': 0.666666666667}, {'HRT': 27648.0, 'HRT_h': 7.68, 'OLR': 1.26591435185e-05, 'OLR_kg_per_m3_day': 1.09375}],
    'enveng_monod_growth_rate': [{'mu': 4.96031746032e-06, 'mu_per_day': 0.428571428571}, {'mu': 4.1335978836e-05, 'mu_per_day': 3.57142857143, 'mu_net': 4.01785714286e-05, 'mu_net_per_day': 3.47142857143}],
    'enveng_activated_sludge_cstr_design': [{'S': 0.00410256410256, 'S_mgL': 4.10256410256, 'X': 3.07371794872, 'X_mgL': 3073.71794872, 'Y_obs': 0.3125}, {'S': 0.00216691068814, 'S_mgL': 2.16691068814, 'X': 2.59540184401, 'X_mgL': 2595.40184401, 'Y_obs': 0.405405405405}],
    'enveng_minimum_srt': [{'SRT_min': 44168.8654354, 'SRT_min_days': 0.511213720317}, {'SRT_min': 22457.4669187, 'SRT_min_days': 0.259924385633}],
    'enveng_sludge_production': [{'Y_obs': 0.3125, 'Px': 0.00886140046296, 'Px_kg_per_day': 765.625}, {'Y_obs': 0.428571428571, 'Px': 0.00376984126984, 'Px_kg_per_day': 325.714285714}],
    'enveng_aeration_oxygen_requirement': [{'O2': 0.0285525599129, 'O2_kg_per_day': 2466.94117647}, {'O2': 0.00845734126984, 'O2_kg_per_day': 730.714285714}],
    'enveng_return_sludge_ratio': [{'R': 0.428571428571, 'Qr': 0.0496031746032}, {'R': 0.454545454545, 'Qr': 0.0263047138047}],
    'enveng_sludge_volume_index': [{'SVI': 120.0, 'Xr_estimate': 8.33333333333, 'Xr_estimate_mgL': 8333.33333333}, {'SVI': 140.625, 'Xr_estimate': 7.11111111111, 'Xr_estimate_mgL': 7111.11111111}],
    'enveng_sludge_volume': [{'V': 0.000283678286129, 'V_m3_per_day': 24.5098039216}, {'V': 0.000175364758698, 'V_m3_per_day': 15.1515151515}],
    'enveng_sludge_thickening': [{'V2': 0.000231481481481, 'reduction': 0.8}, {'V2': 9.25925925926e-05, 'reduction': 0.8}],
    'enveng_anaerobic_methane_yield': [{'Q_CH4': 0.0209169556161, 'Q_CH4_m3_per_day': 1807.22496523, 'Q_CH4_stp_m3_per_day': 1601.95845936}, {'Q_CH4': 0.00499791112067, 'Q_CH4_m3_per_day': 431.819520826, 'Q_CH4_stp_m3_per_day': 390.437973436}],
    'enveng_nrc_trickling_filter': [{'F': 2.08333333333, 'E': 0.784534628877, 'E_percent': 78.4534628877}, {'F': 1.65289256198, 'E': 0.721803607573, 'E_percent': 72.1803607573}],
    'enveng_field_oxygen_transfer': [{'AOTR': 0.0120702910805, 'AOTR_kg_per_h': 43.4530478898, 'ratio': 0.434530478898}, {'AOTR': 0.0233518516529, 'AOTR_kg_per_h': 84.0666659504, 'ratio': 0.336266663801}],
    'enveng_kla_two_point': [{'KLa': 0.00172644306783, 'KLa_per_h': 6.21519504417}, {'KLa': 0.00101810081319, 'KLa_per_h': 3.6651629275}],
    'enveng_kla_regression': [{'KLa_per_min': 0.146989111574, 'KLa': 0.00244981852624, 'KLa_per_h': 8.81934669446, 'r2': 0.999180696125}, {'KLa_per_min': 0.128960973283, 'KLa': 0.00214934955472, 'KLa_per_h': 7.737658397, 'r2': 0.999846613245}],
    'enveng_aeration_do_curve': [{'C': 0.00757010761702, 'C_mgL': 7.57010761702, 'OTR': 4.24970106385e-06, 'OTR_mg_per_L_h': 15.2989238298}, {'C': 0.00678661860225, 'C_mgL': 6.78661860225, 'OTR': 1.90375710861e-06, 'OTR_mg_per_L_h': 6.85352559101}],
    'enveng_henry_constant_conversion': [{'H_pa_m3_per_mol': 557.2875, 'H_atm_m3_per_mol': 0.0055, 'H_dimensionless': 0.22480724488, 'H_atm': 304.395245, 'H_mol_per_m3_pa': 0.00179440593948, 'H_m_per_atm': 0.181818181818}, {'H_pa_m3_per_mol': 598.953100855, 'H_atm_m3_per_mol': 0.00591120750906, 'H_dimensionless': 0.25, 'H_atm': 327.153355994, 'H_mol_per_m3_pa': 0.00166957980278, 'H_m_per_atm': 0.169170173517}],
    'enveng_henry_law_equilibrium': [{'C': 0.26582278481, 'C_mass': 0.00850601012658, 'C_mgL': 8.50601012658}, {'C': 0.181818181818}],
    'enveng_henry_temperature_dependence': [{'Hcp_T': 1.69703919571e-05, 'Hpc_T': 58926.1581304}, {'Hcp_T': 0.00127059723203, 'Hpc_T': 787.031464255}],
    'enveng_two_film_transfer': [{'KL': 1.96078431373e-05, 'flux': 1.96078431373e-08, 'liquid_resistance_fraction': 0.980392156863}, {'KL': 8.33333333333e-06, 'flux': 4.13333333333e-09, 'liquid_resistance_fraction': 0.166666666667}],
    'enveng_air_stripping_tower': [{'R': 6.0, 'NTU': 4.2138545266, 'Z': 4.2138545266, 'air_to_water_min': 4.875}, {'R': 10.0, 'NTU': 5.00102240513, 'Z': 4.0008179241, 'air_to_water_min': 1.98}],
    'enveng_decibel_addition': [{'L_total': 88.0102999566}, {'L_total': 92.2822047669}],
    'enveng_background_noise_correction': [{'L_source': 83.3491146137}, {'L_source': 69.7951916946}],
    'enveng_noise_distance_attenuation': [{'L2': 71.9382002602}, {'L2': 65.9691001301}],
    'enveng_equivalent_sound_level': [{'Leq': 76.522799105}, {'Leq': 86.4693778478}],
    'enveng_day_night_level': [{'L': 63.5436368222}, {'L': 64.8797423829}],
    'enveng_sound_power_to_pressure': [{'Lp': 75.9976014031}, {'Lp': 70.0285012731}],
    'enveng_occupational_noise_dose': [{'dose_percent': 112.5, 'TWA': 90.8496433979}, {'dose_percent': 116.312599785, 'TWA': 91.0900605344}],
    'enveng_solid_waste_generation': [{'m': 0.868055555556, 'm_kg_per_day': 75000.0, 'm_t_per_year': 27375.0, 'V_m3_per_day': 187.5, 'trucks_per_day': 13}, {'m': 2.77777777778, 'm_kg_per_day': 240000.0, 'm_t_per_year': 87600.0, 'V_m3_per_day': 685.714285714, 'trucks_per_day': 35}],
    'enveng_landfill_life': [{'V_first_year': 104166.666667, 'life_years': 19.2}, {'V_first_year': 205714.285714, 'life_years': 18.5271240835}],
    'enveng_landfill_gas_generation': [{'Q_CH4': 0.155999184416, 'Q_CH4_m3_per_year': 4922959.86211, 'V_CH4': 101540802.758, 'Q_LFG': 0.311998368831}, {'Q_CH4': 0.0887988823199, 'Q_CH4_m3_per_year': 2802279.6087, 'V_CH4': 14943009.7826, 'Q_LFG': 0.17759776464}],
    'enveng_dulong_heating_value': [{'HHV': 18643900.0, 'HHV_kJ_per_kg': 18643.9, 'HHV_MJ_per_kg': 18.6439}, {'HHV': 26438850.0, 'HHV_kJ_per_kg': 26438.85, 'HHV_MJ_per_kg': 26.43885}],
    'enveng_waste_moisture_content': [{'moisture_wet': 0.338, 'dry_mass': 66.2, 'moisture_dry': 0.510574018127}, {'moisture_wet': 0.225, 'dry_mass': 31.0, 'moisture_dry': 0.290322580645}],
    # batch 1 (2026-11-11), each checked by hand: pH 14 - 5 = 9; -log10(2.5e-4) = 3.60206; 1.3e-3 x 0.21 = 2.73e-4 mol/L
    # x 32 g/mol = 8.736 mg/L; 0.3/(1 x 1) m/s; 600/120 = 5; R = 1 + 1.6 x 1/0.3 = 6.333, 0.5/6.333 = 0.07895 m/d;
    # 10 x 0.002 = 0.02 m/d, x 500 = 10 m^3/d, /0.25 = 0.08 m/d; 0.05 x 2/70 = 1.4286e-3, /0.02 = 0.0714;
    # 0.01 x 2 x 350 x 30/(70 x 25550) = 1.1742e-4, x 1.5 = 1.761e-4; 3000/320 = 9.375 d; 60/1.5 = 40 min;
    # 120 x 50 = 6000 m^3/d = 250 m^3/h; 31.9988/12.0107 = 2.6642, 2 x 31.9988/16.04246 = 3.9893; 6 degC/0.5 km = 12 degC/km
    'enveng_ph_poh': [{'pH': 9.0, 'pOH': 5.0, 'H_molL': 1e-09, 'OH_molL': 1e-05}, {'pH': 3.60205999133, 'pOH': 10.3979400087, 'H_molL': 0.00025, 'OH_molL': 4e-11}],
    'enveng_henry_law_solubility': [{'C': 0.273, 'C_molL': 0.000273, 'C_mass': 0.008736, 'C_mgL': 8.736}, {'C': 0.0136, 'C_molL': 1.36e-05}],
    'enveng_channel_flow_velocity': [{'A': 1.0, 'v': 0.3, 'Q': 0.3}, {'A': 3.0, 'v': 0.25, 'Q': 0.75}],
    'enveng_landfill_compaction_ratio': [{'CR': 5.0, 'volume_reduction': 0.8, 'volume_reduction_percent': 80.0}, {'CR': 4.0, 'volume_reduction': 0.75, 'volume_reduction_percent': 75.0}],
    'enveng_retardation_factor': [{'Kd': 0.001, 'Kd_L_per_kg': 1.0, 'R': 6.33333333333, 'vc': 9.13742690058e-07, 'vc_m_per_day': 0.0789473684211, 'v_m_per_day': 0.5}, {'R': 4.0, 'vc': 3.47222222222e-06, 'vc_m_per_day': 0.3, 'v_m_per_day': 1.2}],
    'enveng_darcy_seepage_velocity': [{'i': 0.002, 'q': 2.31481481481e-07, 'q_m_per_day': 0.02, 'Q': 0.000115740740741, 'Q_m3_per_day': 10.0, 'v_s': 9.25925925926e-07, 'v_s_m_per_day': 0.08}, {'i': 0.005, 'q': 5e-07, 'q_m_per_day': 0.0432, 'v_s': 1.66666666667e-06, 'v_s_m_per_day': 0.144}],
    'enveng_chronic_daily_intake_risk': [{'CDI': 0.00142857142857, 'HQ': 0.0714285714286}, {'CDI': 0.000117416829746, 'risk': 0.000176125244618}],
    'enveng_mean_cell_residence_time': [{'theta_c': 810000.0, 'theta_c_days': 9.375}, {'theta_c': 648000.0, 'theta_c_days': 7.5}],
    'enveng_contact_time_from_ct': [{'t': 2400.0, 't_min': 40.0, 't_h': 0.666666666667}, {'t': 7800.0, 't_min': 130.0, 't_h': 2.16666666667}],
    'enveng_filter_flow_from_loading': [{'Q': 0.0694444444444, 'Q_m3_per_h': 250.0, 'Q_m3_per_day': 6000.0}, {'Q': 0.222222222222, 'Q_m3_per_h': 800.0, 'Q_m3_per_day': 19200.0}],
    'enveng_combustion_oxygen': [{'o2_per_mol': 1.0, 'molar_mass': 12.0107, 'O2_per_kg': 2.66419109627, 'CO2_per_kg': 3.66419109627, 'H2O_per_kg': 0.0, 'O2_mass': 2664.19109627}, {'o2_per_mol': 2.0, 'molar_mass': 16.04246, 'O2_per_kg': 3.98926349201, 'CO2_per_kg': 2.74331368132, 'H2O_per_kg': 2.24594981069}],
    'enveng_lapse_rate_stability': [{'ELR': 0.012, 'ELR_C_per_km': 12.0, 'stability': 'superadiabatic (unstable)'}, {'ELR': -0.00666666666667, 'ELR_C_per_km': -6.66666666667, 'stability': 'inversion (very stable)'}],
    # batch 2 (2026-11-11), each checked by hand: 122/61.017 x 50 = 99.97, 30/60.009 x 100 = 49.99, pH 10 OH 1e-4 x
    # 5e4 = 5.0, total 154.96; 2e-3 x 5e4 + 2 x 1e-4 x 5e4 = 110; k theta = 2: 100/1.5^4 = 19.753, 100/3, 100 e^-2 =
    # 13.534; R = 0.15, 0.15^(2/3) 0.001^0.5/0.013 = 0.6867 m/s x 0.2827 m^2; R = 3.6/5.4; 0.849 x 100 x 0.075^0.63 x
    # 0.002^0.54 = 0.5791; 1000 x 9.80665 x 0.1 x 30/0.75 = 39.23 kW; 12.258 kW x 24 h = 294.2 kWh x 0.12 = 35.30;
    # 0.6 x 50 x 25/360 = 2.083 m^3/s; (100 x 10e-3 + 100)/(100 + 0.5 x 200) = 0.505 mg/m^3; 1 - 0.2 x 0.01 = 0.998;
    # 12/0.75 = 16 MJ/kg, 14.4/0.7 = 20.57; 1 - 0.99^30 = 0.2603; 1 - 0.9^(1/25) = 0.0042056; GM(10,100,1000) = 100,
    # s = sqrt(599400/2) = 547.4; (0.75 - 0.55)/(0.75 - 0.4125) = 0.5926; (800 + 700)/2 - 50 = 700 kPa;
    # 6 x 31.9988/180.156 = 1.0657 x 100 = 106.57 mg/L; C5H7O2N nitrified 7 mol O2: 1.9802 x 50 = 99.01 mg/L
    'enveng_total_alkalinity': [{'alkalinity': 154.964980495, 'alk_meq_L': 3.0992996099, 'alk_eq_L': 0.0030992996099, 'HCO3_caco3': 99.9724010617, 'CO3_caco3': 49.9925844333, 'OH_caco3': 5.0, 'H_caco3': 5e-06, 'H_molL': 1e-10, 'OH_molL': 0.0001}, {'alkalinity': 110.0, 'alk_meq_L': 2.2, 'alk_eq_L': 0.0022, 'HCO3_caco3': 100.0, 'CO3_caco3': 10.0, 'OH_caco3': 0.0, 'H_caco3': 0.0, 'H_molL': 0.0, 'OH_molL': 0.0}],
    'enveng_cmfr_series': [{'Cout': 0.0197530864198, 'Cout_mgL': 19.7530864198, 'removal': 0.802469135802, 'removal_percent': 80.2469135802, 'Cout_one': 0.0333333333333, 'Cout_one_mgL': 33.3333333333, 'Cout_pfr': 0.0135335283237, 'Cout_pfr_mgL': 13.5335283237, 'removal_pfr': 0.864664716763, 'theta_each': 9000.0, 'theta_each_h': 2.5}, {'Cout': 0.025, 'Cout_mgL': 25.0, 'removal': 0.5, 'removal_percent': 50.0, 'Cout_one': 0.025, 'Cout_one_mgL': 25.0, 'Cout_pfr': 0.0183939720586, 'Cout_pfr_mgL': 18.3939720586, 'removal_pfr': 0.632120558829, 'theta_each': 86400.0, 'theta_each_h': 24.0},
                           # batch 3: V/Q = 500 m^3 / 100 m^3/h = 5 h, k theta = 2: 200/3 = 66.667 mg/L, 200 e^-2 = 27.067
                           {'Cout': 0.0666666666667, 'Cout_mgL': 66.6666666667, 'removal': 0.666666666667, 'removal_percent': 66.6666666667, 'Cout_one': 0.0666666666667, 'Cout_one_mgL': 66.6666666667, 'Cout_pfr': 0.0270670566473, 'Cout_pfr_mgL': 27.0670566473, 'removal_pfr': 0.864664716763, 'theta_each': 18000.0, 'theta_each_h': 5.0}],
    'enveng_manning_flow': [{'v': 0.686727048818, 'Q': 0.194167498643, 'Q_L_per_s': 194.167498643, 'R': 0.15, 'A': 0.282743338823, 'P': 1.88495559215}, {'v': 1.13762616052, 'Q': 4.09545417786, 'Q_L_per_s': 4095.45417786, 'R': 0.666666666667, 'A': 3.6, 'P': 5.4}],
    'enveng_hazen_williams': [{'v': 0.579098558442, 'Q': 0.0409340649804, 'S': 0.002, 'R': 0.075, 'A': 0.0706858347058}, {'v': 0.954929658551, 'Q': 0.03, 'S': 0.0057822262322, 'R': 0.05, 'A': 0.0314159265359, 'hf': 5.7822262322}],
    'enveng_pump_power': [{'P': 39226.6, 'P_kW': 39.2266, 'P_water': 29419.95, 'P_water_kW': 29.41995}, {'P': 12258.3125, 'P_kW': 12.2583125, 'P_water': 9806.65, 'P_water_kW': 9.80665, 'energy_kWh': 294.1995, 'cost': 35.30394}],
    'enveng_rational_method': [{'Q': 2.08333333333, 'Q_L_per_s': 2083.33333333, 'Q_m3_per_h': 7500.0}, {'Q': 9.33333333333, 'Q_L_per_s': 9333.33333333, 'Q_m3_per_h': 33600.0}],
    'enveng_indoor_air_steady_state': [{'C': 5.05e-07, 'C_ugm3': 505.0, 'C_mgm3': 0.505, 'ach': 0.5, 'tau_h': 1.0}, {'C': 2e-07, 'C_ugm3': 200.0, 'C_mgm3': 0.2, 'ach': 0.5, 'tau_h': 2.0}],
    'enveng_collection_efficiency_series': [{'eta_total': 0.998, 'eta_total_percent': 99.8, 'penetration': 0.002}, {'eta_total': 0.9975, 'eta_total_percent': 99.75, 'penetration': 0.0025, 'C_out': 1.25e-05, 'C_out_mgm3': 12.5}],
    'enveng_heating_value_dry_basis': [{'HHV_dry': 16000000.0, 'HHV_ar': 12000000.0, 'HHV_dry_MJ_per_kg': 16.0, 'HHV_ar_MJ_per_kg': 12.0}, {'HHV_dry': 18000000.0, 'HHV_ar': 14400000.0, 'HHV_dry_MJ_per_kg': 18.0, 'HHV_ar_MJ_per_kg': 14.4, 'HHV_daf': 20571428.5714, 'HHV_daf_MJ_per_kg': 20.5714285714}],
    'enveng_return_period_risk': [{'p': 0.01, 'p_percent': 1.0, 'T': 100.0, 'P_n': 0.260299626612, 'P_n_percent': 26.0299626612, 'P_none': 0.739700373388}, {'p': 0.0042055524182, 'p_percent': 0.42055524182, 'T': 237.780890727, 'P_n': 0.1, 'P_n_percent': 10.0, 'P_none': 0.9}],
    'enveng_geometric_mean_counts': [{'geometric_mean': 100.0, 'mean': 370.0, 'sd': 547.448627727, 'mean_log10': 2.0, 'count': 3.0}, {'geometric_mean': 30.6115875298, 'mean': 44.6, 'sd': 43.997727214, 'mean_log10': 1.48588585288, 'count': 5.0}],
    'enveng_volatile_solids_reduction': [{'VSR': 0.592592592593, 'VSR_percent': 59.2592592593, 'TS_remaining': 0.555555555556}, {'VSR': 0.571428571429, 'VSR_percent': 57.1428571429, 'TS_remaining': 0.6}],
    'enveng_transmembrane_pressure': [{'TMP': 700000.0, 'TMP_kPa': 700.0, 'TMP_bar': 7.0, 'dP': 100000.0, 'dP_kPa': 100.0}, {'TMP': 190000.0, 'TMP_kPa': 190.0, 'TMP_bar': 1.9, 'dP': 20000.0, 'dP_kPa': 20.0}],
    'enveng_thod_solution': [{'ThOD': 0.10657037672, 'ThOD_mgL': 106.57037672, 'thod_ratio': 1.0657037672, 'o2_per_mol': 6.0, 'molar_mass': 180.15588, 'C_molL': 0.000555074860726}, {'ThOD': 0.099010932101, 'ThOD_mgL': 99.010932101, 'thod_ratio': 1.98021864202, 'o2_per_mol': 7.0, 'molar_mass': 113.11458, 'C_molL': 0.000442029665849}],
    # batch 3 (2026-11-11), each checked by hand: 100 e^-1 = 36.788 mg/L, t1/2 = 0.69315/0.2 = 3.4657 d; ln(80/20)/4 h =
    # 0.34657 1/h = 8.3178 1/d, t1/2 = 2 h; 10,000 m^3/d x 25 g/m^3 = 250 kg/d = 551.16 lb/d (/0.45359237), 2.6417 MGD;
    # 2 MGD x 10 mg/L x 8.3454 = 166.91 lb/d, /0.48 = 347.73 lb/d = 157.73 kg/d; 750/1500 = 0.5 m/d = 20.833 L/(m^2 h);
    # 2400 m^3/d = 100,000 L/h / 25 = 4000 m^2; 200,000/600 = 333.33 m^3/d x 1.25 = 416.67, x 365 = 152,083 m^3;
    # 500,000/800 x 1.2 = 750 m^3/d, x 365 = 273,750 m^3/yr, 2e6/273,750 = 7.306 yr
    'enveng_first_order_decay': [{'C': 0.0367879441171, 'C_mgL': 36.7879441171, 'C0': 0.1, 'C0_mgL': 100.0, 'k': 2.31481481481e-06, 'k_per_day': 0.2, 'k_per_h': 0.00833333333333, 't': 432000.0, 't_days': 5.0, 't_h': 120.0, 't_half': 299439.582002, 't_half_days': 3.4657359028, 't_half_h': 83.1776616672, 'fraction_remaining': 0.367879441171, 'removal_percent': 63.2120558829}, {'C': 0.02, 'C_mgL': 20.0, 'C0': 0.08, 'C0_mgL': 80.0, 'k': 9.62704417444e-05, 'k_per_day': 8.31776616672, 'k_per_h': 0.34657359028, 't': 14400.0, 't_days': 0.166666666667, 't_h': 4.0, 't_half': 7200.0, 't_half_days': 0.0833333333333, 't_half_h': 2.0, 'fraction_remaining': 0.25, 'removal_percent': 75.0}],
    'enveng_chemical_feed_rate': [{'feed': 0.00289351851852, 'feed_kg_per_day': 250.0, 'feed_lb_per_day': 551.155655462, 'active_kg_per_day': 250.0, 'active_lb_per_day': 551.155655462, 'Q_MGD': 2.64172052358}, {'feed': 0.0018255265162, 'feed_kg_per_day': 157.725491, 'feed_lb_per_day': 347.725185501, 'active_kg_per_day': 75.70823568, 'active_lb_per_day': 166.90808904, 'Q_MGD': 2.0}],
    'enveng_membrane_flux': [{'J': 5.78703703704e-06, 'J_LMH': 20.8333333333, 'J_m_per_day': 0.5, 'A': 1500.0, 'Qp': 0.00868055555556, 'Qp_m3_per_day': 750.0}, {'J': 6.94444444444e-06, 'J_LMH': 25.0, 'J_m_per_day': 0.6, 'A': 4000.0, 'Qp': 0.0277777777778, 'Qp_m3_per_day': 2400.0}],
    'enveng_landfill_daily_volume': [{'V': 0.0048225308642, 'V_m3_per_day': 416.666666667, 'V_m3_per_year': 152083.333333, 'V_waste_m3_per_day': 333.333333333, 'V_cover_m3_per_day': 83.3333333333}, {'V': 0.00868055555556, 'V_m3_per_day': 750.0, 'V_m3_per_year': 273750.0, 'V_waste_m3_per_day': 625.0, 'V_cover_m3_per_day': 125.0, 'life_years': 7.30593607306, 'life_days': 2666.66666667}],
}

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)


# ================================================================================================ FUNDAMENTALS: population and demand
# (2026-11-12, core curriculum gaps) Population projection from census data by the arithmetic and geometric methods
# and by the logistic (three-census) method; water and wastewater demand with peaking factors. Exponential and
# logistic growth at a GIVEN rate are differential_equations' and cell_molecular_biology's capabilities; these fit the
# methods to census data. The example outputs were worked by hand.

def population_projection(t1, P1, t2, P2, t):
    _fin(t1, P1, t2, P2, t)
    _need(t2 > t1, "the second census must be later than the first")
    _need(t2 - t1 <= 200 and abs(t - t2) <= 200, "census interval and projection span must each be at most 200 years")
    _pos("the census populations", P1, P2)
    ka = (P2 - P1) / (t2 - t1)
    Pa = P2 + ka * (t - t2)
    _need(Pa > 0, "the arithmetic projection falls to zero or below: the decline is too fast for this method")
    kg = math.log(P2 / P1) / (t2 - t1)
    return _out(k_a=ka, P_arith=Pa, k_g=kg, r_g=math.expm1(kg), P_geom=P2 * math.exp(kg * (t - t2)))


def _ref_population_projection(t1, P1, t2, P2, t):
    # straight lines through the two censuses (numpy polyfit, years centred on the second census) for the population
    # and for its logarithm; the compound rate as the (t2 - t1)-th root of the growth ratio
    import numpy as np
    x = np.array([t1 - t2, 0.0])
    lin = np.polyfit(x, np.array([P1, P2], float), 1)
    lg = np.polyfit(x, np.log(np.array([P1, P2], float)), 1)
    return {"k_a": float(lin[0]), "P_arith": float(np.polyval(lin, t - t2)), "k_g": float(lg[0]),
            "r_g": (P2 / P1) ** (1.0 / (t2 - t1)) - 1.0, "P_geom": float(np.exp(np.polyval(lg, t - t2)))}


def _wrong_population_projection(t1, P1, t2, P2, t):
    # the increase added to the first census instead of the last, and a base-10 logarithm for the rate
    ka = (P2 - P1) / (t2 - t1)
    kg = math.log10(P2 / P1) / (t2 - t1)
    return _out(k_a=ka, P_arith=P1 + ka * (t - t2), k_g=kg, r_g=math.expm1(kg), P_geom=P2 * math.exp(kg * (t - t2)))


def _rand_population_projection(rng):
    t1 = rng.randint(1950, 2000)
    t2 = t1 + rng.choice([10, 20, 30])
    P1 = rng.randint(5000, 500000)
    return {"t1": t1, "P1": P1, "t2": t2, "P2": int(round(P1 * rng.uniform(0.9, 1.8))), "t": t2 + rng.randint(5, 40)}


_add("enveng_population_projection",
     name="Population projection by the arithmetic and geometric methods", topic="fundamentals / population and demand",
     purpose="The population projected to a year from two censuses by the arithmetic method (constant increase per year) and by the geometric method (constant percentage growth: the continuous rate and the equivalent compound annual rate).",
     inputs=[_numin("t1", "year of the first census"), _numin("P1", "population at the first census"),
             _numin("t2", "year of the second census"), _numin("P2", "population at the second census"),
             _numin("t", "year of the projection")],
     outputs=[_o("k_a", "number", "arithmetic increase, persons per year"), _o("P_arith", "number", "arithmetic projection"),
              _o("k_g", "number", "geometric (continuous) growth rate, per year"), _o("r_g", "number", "equivalent compound growth rate per year"),
              _o("P_geom", "number", "geometric projection")],
     primary="P_geom", conditions=["t2 > t1", "positive census populations", "interval and projection span at most 200 years",
                                   "the arithmetic projection must stay positive"],
     method="k_a = (P2 - P1)/(t2 - t1), P = P2 + k_a (t - t2); k_g = ln(P2/P1)/(t2 - t1), P = P2 exp(k_g (t - t2))",
     latex=r"P_t = P_2 + \frac{P_2 - P_1}{t_2 - t_1}(t - t_2),\qquad P_t = P_2\,e^{k_g (t - t_2)},\ k_g = \frac{\ln(P_2/P_1)}{t_2 - t_1}",
     run=population_projection, reference=_ref_population_projection, wrong=_wrong_population_projection,
     rand=_rand_population_projection,
     phrases=("Population projection", "Population forecast", "Arithmetic and geometric population projection"),
     aliases={"year1": "t1", "year2": "t2", "pop1": "P1", "pop2": "P2", "year": "t", "target_year": "t"},
     # worked by hand: 20 000 people in 20 years -> 1000 per year; growth ratio 1.4 over 20 years, so 2040 = 70000 x 1.4
     examples=[_ex({"t1": 2000.0, "P1": 50000.0, "t2": 2020.0, "P2": 70000.0, "t": 2040.0},
                   {"k_a": 1000.0, "P_arith": 90000.0, "k_g": 0.0168236118310606, "r_g": 0.0169659257465, "P_geom": 98000.0}),
               _ex({"t1": 1990.0, "P1": 120000.0, "t2": 2010.0, "P2": 150000.0, "t": 2030.0},
                   {"k_a": 1500.0, "P_arith": 180000.0, "k_g": 0.0111571775657105, "r_g": 0.0112196509975149, "P_geom": 187500.0})],
     level="beginner", weak="numpy straight-line fits through the same two censuses",
     extra_q=[{"q": "Population forecast: t1 = 2000, P1 = 50000, t2 = 2020, P2 = 70000, t = 2040", "like": 0},
              {"q": "Population projection: t1 = 2020, P1 = 50000, t2 = 2000, P2 = 70000, t = 2040", "outcome": "INVALID_INPUT"}])


def population_logistic(t0, n, P0, P1, P2, t):
    _fin(t0, n, P0, P1, P2, t)
    _need(0 < n <= 100, "the census interval must be between 0 and 100 years")
    _pos("the census populations", P0, P1, P2)
    _need(P0 < P1 < P2, "the three censuses must increase (P0 < P1 < P2)")
    _need(P1 * P1 > P0 * P2, "the growth is not slowing (P1^2 <= P0 P2): there is no saturation population; use the geometric method")
    _need(abs(t - t0) <= 500, "the projection year must lie within 500 years of the first census")
    Ps = P2 + P0 * (P2 - P1) * (P2 - P1) / (P1 * P1 - P0 * P2)
    _need(Ps <= 100 * P2, "the saturation population exceeds 100 times the last census: the data show no clear saturation")
    m = (Ps - P0) / P0
    k = math.log(P1 * (Ps - P0) / (P0 * (Ps - P1))) / n
    _need(k * abs(t - t0) <= 700, "the projection year is too far from the censuses for this growth rate")
    return _out(P_sat=Ps, m=m, k=k, P_t=Ps / (1.0 + m * math.exp(-k * (t - t0))))


def _ref_population_logistic(t0, n, P0, P1, P2, t):
    # the saturation population found numerically (scipy brentq) as the one whose logistic curve through the first
    # two censuses also passes through the third
    def curve(S, tau):
        mm = (S - P0) / P0
        kk = math.log(P1 * (S - P0) / (P0 * (S - P1))) / n
        return S / (1.0 + mm * math.exp(-kk * tau))
    Ps = _brentq(lambda S: curve(S, 2.0 * n) - P2, P2, 200.0 * P2)
    m = (Ps - P0) / P0
    k = math.log(P1 * (Ps - P0) / (P0 * (Ps - P1))) / n
    return {"P_sat": Ps, "m": m, "k": k, "P_t": curve(Ps, t - t0)}


def _wrong_population_logistic(t0, n, P0, P1, P2, t):
    # the constant m divided by the saturation population instead of the first census
    Ps = P2 + P0 * (P2 - P1) * (P2 - P1) / (P1 * P1 - P0 * P2)
    m = (Ps - P0) / Ps
    k = math.log(P1 * (Ps - P0) / (P0 * (Ps - P1))) / n
    return _out(P_sat=Ps, m=m, k=k, P_t=Ps / (1.0 + m * math.exp(-k * (t - t0))))


def _rand_population_logistic(rng):
    P0 = rng.randint(5000, 500000)
    S = P0 * rng.uniform(2.0, 6.0)
    m = (S - P0) / P0
    k = rng.uniform(0.02, 0.08)
    n = rng.choice([10, 20])
    t0 = rng.randint(1900, 1990)
    P1 = int(round(S / (1.0 + m * math.exp(-k * n))))
    P2 = int(round(S / (1.0 + m * math.exp(-2.0 * k * n))))
    return {"t0": t0, "n": n, "P0": P0, "P1": P1, "P2": P2, "t": t0 + 2 * n + rng.randint(0, 40)}


_add("enveng_population_logistic",
     name="Population projection by the logistic method (three censuses)", topic="fundamentals / population and demand",
     purpose="The saturation population and the logistic curve P = Ps/(1 + m exp(-k (t - t0))) through three equally spaced censuses P0, P1, P2 (interval n years), and the population it projects for a year.",
     inputs=[_numin("t0", "year of the first census"), _numin("n", "interval between the censuses in years"),
             _numin("P0", "population at the first census"), _numin("P1", "population one interval later"),
             _numin("P2", "population two intervals later"), _numin("t", "year of the projection")],
     outputs=[_o("P_sat", "number", "saturation population"), _o("m", "number", "logistic constant (Ps - P0)/P0"),
              _o("k", "number", "logistic rate constant, per year"), _o("P_t", "number", "projected population")],
     primary="P_t", conditions=["three equally spaced censuses with P0 < P1 < P2", "growth slowing: P1^2 > P0 P2",
                                "saturation population at most 100 times the last census"],
     method="Ps = (2 P0 P1 P2 - P1^2 (P0 + P2))/(P0 P2 - P1^2); m = (Ps - P0)/P0; k = ln(P1 (Ps - P0)/(P0 (Ps - P1)))/n",
     latex=r"P_s = \frac{2P_0P_1P_2 - P_1^2(P_0 + P_2)}{P_0P_2 - P_1^2},\quad P_t = \frac{P_s}{1 + m\,e^{-k(t - t_0)}}",
     run=population_logistic, reference=_ref_population_logistic, wrong=_wrong_population_logistic,
     rand=_rand_population_logistic,
     phrases=("Logistic population projection", "Population projection by the logistic method", "Logistic population forecast"),
     aliases={"interval": "n", "year0": "t0", "year": "t", "target_year": "t"},
     # worked by hand: Ps = 75000 + 40000 x 15000^2/(60000^2 - 40000 x 75000) = 90000, m = 1.25, e^(10 k) = 2.5;
     # 2030: 90000/(1 + 1.25/2.5^3) = 90000/1.08; 2025: 90000/(1 + 0.2/sqrt(2.5))
     examples=[_ex({"t0": 2000.0, "n": 10.0, "P0": 40000.0, "P1": 60000.0, "P2": 75000.0, "t": 2030.0},
                   {"P_sat": 90000.0, "m": 1.25, "k": 0.0916290731874155, "P_t": 83333.3333333333}),
               _ex({"t0": 2000.0, "n": 10.0, "P0": 40000.0, "P1": 60000.0, "P2": 75000.0, "t": 2025.0},
                   {"P_sat": 90000.0, "m": 1.25, "k": 0.0916290731874155, "P_t": 79894.1061213})],
     level="intermediate",
     extra_q=[{"q": "Logistic population forecast: t0 = 2000, n = 10, P0 = 40000, P1 = 60000, P2 = 75000, t = 2030", "like": 0},
              {"q": "Logistic population projection: t0 = 2000, n = 10, P0 = 40000, P1 = 60000, P2 = 100000, t = 2030", "outcome": "INVALID_INPUT"}])


def water_demand(pop, q, f_md, f_ph, r_ww=None):
    _need(isinstance(pop, int) and not isinstance(pop, bool) and pop >= 1, "the population must be a whole number of at least 1")
    _pos("the per-capita demand", q)
    _need(20 <= q <= 2000, "the per-capita demand must be between 20 and 2000 L per person per day")
    _fin(f_md, f_ph)
    _need(1 <= f_md <= 4, "the maximum-day factor must lie between 1 and 4")
    _need(f_md <= f_ph <= 8, "the peak-hour factor must lie between the maximum-day factor and 8")
    Qd = pop * q / 1000.0                                     # m^3/d
    out = {"Q_avg": Qd / DAY, "Q_avg_m3_per_day": Qd, "Q_maxday": f_md * Qd / DAY, "Q_maxday_m3_per_day": f_md * Qd,
           "Q_peakhour": f_ph * Qd / DAY, "Q_peakhour_m3_per_day": f_ph * Qd, "Q_peakhour_L_per_s": f_ph * Qd / DAY * 1000.0}
    if r_ww is not None:
        _frac("the wastewater return fraction", r_ww, lo_open=True)
        pf = 1.0 + 14.0 / (4.0 + math.sqrt(pop / 1000.0))
        out["Q_ww_avg_m3_per_day"] = r_ww * Qd
        out["harmon_peak_factor"] = pf
        out["Q_ww_peak_m3_per_day"] = pf * r_ww * Qd
    return _out(**out)


def _ref_water_demand(pop, q, f_md, f_ph, r_ww=None):
    # exact rational arithmetic of the demands from the inputs as written (a definition: a restated reference);
    # Harmon's factor in its rationalized form 1 + 14 (sqrt(P) - 4)/(P - 16), P in thousands (14/8 at P = 16)
    from fractions import Fraction as Fr
    Qd = Fr(pop) * Fr(repr(q)) / 1000
    out = {"Q_avg": float(Qd / 86400), "Q_avg_m3_per_day": float(Qd), "Q_maxday": float(Fr(repr(f_md)) * Qd / 86400),
           "Q_maxday_m3_per_day": float(Fr(repr(f_md)) * Qd), "Q_peakhour": float(Fr(repr(f_ph)) * Qd / 86400),
           "Q_peakhour_m3_per_day": float(Fr(repr(f_ph)) * Qd), "Q_peakhour_L_per_s": float(Fr(repr(f_ph)) * Qd / 86400 * 1000)}
    if r_ww is not None:
        Pk = pop / 1000.0
        pf = 1.0 + (14.0 * (math.sqrt(Pk) - 4.0) / (Pk - 16.0) if abs(Pk - 16.0) > 1e-6 else 14.0 / 8.0)
        out["Q_ww_avg_m3_per_day"] = float(Fr(repr(r_ww)) * Qd)
        out["harmon_peak_factor"] = pf
        out["Q_ww_peak_m3_per_day"] = pf * float(Fr(repr(r_ww)) * Qd)
    return out


def _wrong_water_demand(pop, q, f_md, f_ph, r_ww=None):
    # litres taken as cubic metres (the 1000 forgotten)
    out = dict(water_demand(pop, q, f_md, f_ph, r_ww))
    for key in ("Q_avg", "Q_avg_m3_per_day", "Q_maxday", "Q_maxday_m3_per_day", "Q_peakhour", "Q_peakhour_m3_per_day",
                "Q_peakhour_L_per_s", "Q_ww_avg_m3_per_day", "Q_ww_peak_m3_per_day"):
        if key in out:
            out[key] = _r(out[key] * 1000.0)
    return out


def _rand_water_demand(rng):
    f_md = _sig(rng.uniform(1.2, 2.5), 2)
    x = {"pop": rng.randint(500, 2000000), "q": _sig(rng.uniform(80.0, 600.0), 3), "f_md": f_md,
         "f_ph": _sig(f_md * rng.uniform(1.2, 2.2), 2)}
    if rng.random() < 0.5:
        x["r_ww"] = _sig(rng.uniform(0.6, 0.9), 2)
    return x


_add("enveng_water_demand",
     name="Water and wastewater demand with peaking factors", topic="fundamentals / population and demand",
     purpose="The average-day, maximum-day and peak-hour water demand of a population from the per-capita demand and the peaking factors (both relative to the average day) and, with the fraction of water returned as wastewater, the average wastewater flow and its peak by Harmon's factor 1 + 14/(4 + sqrt(P/1000)).",
     inputs=[_intin("pop", "population served", 1, 100000000),
             _numin("q", "average per-capita demand in L per person per day", words=("L/cap/d", "L/(cap*d)", "Lpcd", "lpcd", "L/person/d")),
             _numin("f_md", "maximum-day factor (maximum day / average day)"),
             _numin("f_ph", "peak-hour factor (peak hour / average day)"),
             _numin("r_ww", "fraction of the water returned as wastewater (optional)", required=False, pct=True)],
     outputs=[_o("Q_avg", "flow_rate", "average-day demand"), _o("Q_avg_m3_per_day", "number", "average-day demand, m^3/d"),
              _o("Q_maxday", "flow_rate", "maximum-day demand"), _o("Q_maxday_m3_per_day", "number", "maximum-day demand, m^3/d"),
              _o("Q_peakhour", "flow_rate", "peak-hour demand"), _o("Q_peakhour_m3_per_day", "number", "peak-hour demand as a daily rate, m^3/d"),
              _o("Q_peakhour_L_per_s", "number", "peak-hour demand, L/s"),
              _o("Q_ww_avg_m3_per_day", "number", "average wastewater flow, m^3/d"), _o("harmon_peak_factor", "number", "Harmon peaking factor"),
              _o("Q_ww_peak_m3_per_day", "number", "peak wastewater flow, m^3/d")],
     primary="Q_avg", conditions=["pop a whole number >= 1", "20 <= q <= 2000 L per person per day", "1 <= f_md <= 4",
                                  "f_md <= f_ph <= 8 (both relative to the average day)", "0 < r_ww <= 1"],
     method="Q_avg = P q; Q_md = f_md Q_avg; Q_ph = f_ph Q_avg; Q_ww = r Q_avg; PF = 1 + 14/(4 + sqrt(P/1000))",
     latex=r"Q_{avg} = P\,q,\quad Q_{md} = f_{md}Q_{avg},\quad Q_{ph} = f_{ph}Q_{avg},\quad PF = 1 + \frac{14}{4 + \sqrt{P/1000}}",
     run=water_demand, reference=_ref_water_demand, wrong=_wrong_water_demand, rand=_rand_water_demand,
     phrases=("Water demand", "Water and wastewater demand", "Design water demand"),
     aliases={"population": "pop", "per_capita": "q", "max_day_factor": "f_md", "peak_hour_factor": "f_ph", "return_fraction": "r_ww"},
     # worked by hand: 50000 x 300 L = 15000 m^3/d; 20000 x 250 L = 5000 m^3/d, PF = 1 + 3.5 (sqrt 20 - 4)
     examples=[_ex({"pop": 50000, "q": 300.0, "f_md": 1.8, "f_ph": 3.0},
                   {"Q_avg": 0.173611111111, "Q_avg_m3_per_day": 15000.0, "Q_maxday": 0.3125, "Q_maxday_m3_per_day": 27000.0,
                    "Q_peakhour": 0.520833333333, "Q_peakhour_m3_per_day": 45000.0, "Q_peakhour_L_per_s": 520.833333333}),
               _ex({"pop": 20000, "q": 250.0, "f_md": 1.5, "f_ph": 2.5, "r_ww": 0.8},
                   {"Q_avg": 0.0578703703704, "Q_avg_m3_per_day": 5000.0, "Q_maxday": 0.0868055555556, "Q_maxday_m3_per_day": 7500.0,
                    "Q_peakhour": 0.144675925926, "Q_peakhour_m3_per_day": 12500.0, "Q_peakhour_L_per_s": 144.675925926,
                    "Q_ww_avg_m3_per_day": 4000.0, "harmon_peak_factor": 2.65247584249853, "Q_ww_peak_m3_per_day": 10609.9033699941})],
     level="beginner", weak="exact rational arithmetic of the same definitions; Harmon's factor in its rationalized form",
     extra_q=[{"q": "Design water demand: pop = 50000, q = 300 Lpcd, f_md = 1.8, f_ph = 3", "like": 0},
              {"q": "Water demand: pop = 50000, q = 300 L/cap/d, f_md = 1.8, f_ph = 1.2", "outcome": "INVALID_INPUT"}])
