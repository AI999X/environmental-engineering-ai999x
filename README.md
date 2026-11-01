# environmental-engineering-ai999x — offline, verified environmental engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/environmental-engineering-ai999x)](https://pypi.org/project/environmental-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/environmental-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/environmental-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/environmental-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/environmental-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/environmental-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

environmental-engineering-ai999x is an offline, deterministic Python solver that answers environmental engineering questions with verified results from 99 certified capabilities, gives 613 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**environmental-engineering-ai999x** is a compact, deterministic environmental engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified environmental engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified environmental engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 613 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** air pollution dispersion, water quality, water and wastewater treatment, gas transfer, noise and solid waste.

## Model overview

| Property | Value |
|---|---|
| Package | `environmental-engineering-ai999x` 1.0.0 (import `environmental_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic environmental engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 99 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 613 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install environmental-engineering-ai999x
```

```python
import environmental_engineering_ai999x as environmental_engineering

environmental_engineering.ask("Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L").show()
# -> SOLVED — D = 0.00272617, DO = 0.00637383, D_mgL = 2.72617, DO_mgL = 6.37383  (enveng_streeter_phelps_deficit)
environmental_engineering.ask("Briggs plume rise: vs = 15 m/s, d = 3 m, Ts = 420 K, Ta = 288 K, u = 5 m/s, hs = 100 m").show()
# -> SOLVED — F = 104.021, x_f = 762.772, dh = 125.638, H = 225.638  (enveng_briggs_plume_rise)
environmental_engineering.solve("enveng_gaussian_plume_concentration", Q=0.1, u=4.0, sigma_y=60.0, sigma_z=30.0, H=50.0, y=20.0, z=0.0).show()
# -> SOLVED — C = 1.04281e-06, C_ugm3 = 1042.81  (enveng_gaussian_plume_concentration)
environmental_engineering.ask("Activated sludge complete-mix design: Y = 0.5, k = 5 1/d, Ks = 60 mg/L, kd = 0.06 1/d, SRT = 10 d, HRT = 6 h, S0 = 250 mg/L").show()
# -> SOLVED — S = 0.00410256, S_mgL = 4.10256, X = 3.07372, X_mgL = 3073.72  (enveng_activated_sludge_cstr_design)
environmental_engineering.solve("enveng_decibel_addition", L=[85.0, 85.0, 90.0]).show()
# -> SOLVED — L_total = 92.1284  (enveng_decibel_addition)
environmental_engineering.ask("Who created you?").message
# -> environmental-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `environmental-engineering-ai999x "<question>"`, `environmental-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on environmental-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

99 certified procedures. A few of them:

- **Procedures:** Activated carbon dose from the Freundlich isotherm; Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty); Adding sound levels in decibels; Adsorbent dose from the Langmuir isotherm; BOD exerted after a time (first-order); BOD from a dilution test (with optional seed correction); BOD rate constant and ultimate BOD fitted to BOD data; Box model of an air basin (well-mixed, time-dependent); Briggs dispersion coefficients sigma_y and sigma_z; Briggs plume rise and effective stack height (neutral or unstable air); Briggs plume rise in stable air; Chlorine dose and feed rate; Clean-bed head loss through a granular filter (Ergun / Carman-Kozeny); Concentration after mixing a discharge with a river; Concentration in a well-mixed lake over time; Critical (minimum) dissolved oxygen of the oxygen sag; Cyclone cut diameter and fractional efficiency (Lapple); Day-night (Ldn) and day-evening-night (Lden) sound levels; Disinfection CT value of a contact basin; Disinfection by the Chick-Watson law; Dissolved concentration in equilibrium with a gas (Henry's law); Dissolved oxygen at a distance downstream (Streeter-Phelps); Dissolved oxygen during aeration and the transfer rate; Dissolved oxygen saturation concentration (Benson-Krause) …

`environmental_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified environmental engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (air pollution dispersion, water quality, water and wastewater treatment, gas transfer, noise and solid waste); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{environmental_engineering_ai999x_1_0_0,
  title  = {environmental-engineering-ai999x: an offline, verified environmental engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/environmental-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/environmental-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

environmental-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/environmental-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/environmental-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/environmental-engineering-ai999x/tree/main/onnx).
