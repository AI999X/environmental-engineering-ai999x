# Environmental Engineering (environmental-engineering-ai999x)

[![tests](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/environmental-engineering-ai999x)](https://pypi.org/project/environmental-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/environmental-engineering-ai999x)

Environmental Engineering covers Gaussian plume dispersion and plume rise, BOD and Streeter-Phelps oxygen sag, water and wastewater treatment design, activated sludge, disinfection CT, Henry's law and air stripping, noise and solid waste: 132 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Environmental Engineering desktop app: Fraction of discrete particles removed in an ideal settling basin, with its worked example and live scene](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/environmental-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `environmental-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install environmental-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import environmental_engineering_ai999x as environmental_engineering

a = environmental_engineering.ask("Add sound levels: L = [85, 85]")
print(a.outcome, a.values)
# SOLVED {'L_total': 88.0102999566}

a = environmental_engineering.ask(
    "Chronic daily intake: C = 0.05 mg/L, IR = 2 L/d, BW = 70 kg, RfD "
    "= 0.02 mg/kg/d"
)
print(a.outcome, a.values)
# SOLVED {'CDI': 0.00142857142857, 'HQ': 0.0714285714286}

a = environmental_engineering.ask(
    "Freundlich carbon dose: C0 = 10 mg/L, Ce = 1 mg/L, K = 50, n = 2"
)
print(a.outcome, a.values)
# SOLVED {'qe': 50.0, 'dose': 0.18, 'dose_mgL': 180.0}

a = environmental_engineering.ask(
    "Day-night sound level: L = [50, 50, 50, 50, 50, 50, 50, 65, 65, "
    "65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 60, 60, 60, 55, 55], "
    "metric = ldn"
)
print(a.outcome, a.values)
# SOLVED {'L': 63.5436368222}

a = environmental_engineering.ask(
    "BOD dilution test: D1 = 8.5 mg/L, D2 = 4 mg/L, P = 0.05"
)
print(a.outcome, a.values)
# SOLVED {'BOD': 0.09, 'BOD_mgL': 90.0}

a = environmental_engineering.ask(
    "Langmuir adsorbent dose: C0 = 10 mg/L, Ce = 1 mg/L, qmax = 150, b "
    "= 0.8"
)
print(a.outcome, a.values)
# SOLVED {'qe': 66.6666666667, 'dose': 0.135, 'dose_mgL': 135.0}

a = environmental_engineering.ask(
    "Briggs dispersion coefficients: x = 1000 m, cls = d, terrain = "
    "rural"
)
print(a.outcome, a.values)
# SOLVED {'sigma_y': 76.2770071396, 'sigma_z': 37.947331922}

a = environmental_engineering.ask(
    "Equivalent sound level: L = [70, 85, 60], t = [4, 1, 3]"
)
print(a.outcome, a.values)
# SOLVED {'Leq': 76.522799105}

a = environmental_engineering.ask(
    "Occupational noise dose: L = [90, 95, 85], hours = [4, 2, 2]"
)
print(a.outcome, a.values)
# SOLVED {'dose_percent': 112.5, 'TWA': 90.8496433979}

a = environmental_engineering.ask(
    "Landfill life: capacity = 2000000 m^3, M = 50000 t/yr, rho = 600 "
    "kg/m^3, cover = 0.25, g = 0"
)
print(a.outcome, a.values)
# SOLVED {'V_first_year': 104166.666667, 'life_years': 19.2}

a = environmental_engineering.ask(
    "Food-to-microorganism ratio: Q = 10000 m^3/d, S0 = 200 mg/L, V = "
    "3000 m^3, X = 2500 mg/L"
)
print(a.outcome, a.values)
# SOLVED {'FM': 3.08641975309e-06, 'FM_per_day': 0.266666666667}

a = environmental_engineering.ask(
    "Mixing of discharge and river: Qw = 0.5 m^3/s, Cw = 200 mg/L, Qr "
    "= 5 m^3/s, Cr = 2 mg/L"
)
print(a.outcome, a.values)
# SOLVED {'C': 0.02, 'C_mgL': 20.0, 'Q': 5.5}

a = environmental_engineering.ask(
    "Noise distance attenuation: L1 = 90 dB, r1 = 10 m, r2 = 80 m, "
    "source = point"
)
print(a.outcome, a.values)
# SOLVED {'L2': 71.9382002602}

a = environmental_engineering.ask(
    "Filter clean-bed head loss: d = 0.5 mm, eps = 0.4, psi = 0.85, L "
    "= 0.7 m, v = 8 m/h, rho = 998.2 kg/m^3, mu = 0.001002 Pa*s"
)
print(a.outcome, a.values)
# SOLVED {'dP': 7414.17092785, 'h': 0.757398347116, 'Re': 1.56810453168}

a = environmental_engineering.ask(
    "Cyclone efficiency Lapple: mu = 1.8e-05 Pa*s, W = 0.25 m, Ne = 6, "
    "Vi = 15 m/s, rho_p = 1500 kg/m^3, rho_g = 1.2 kg/m^3, dp = 10 um"
)
print(a.outcome, a.values)
# SOLVED {'d50': 6.9126486021e-06, 'eta': 0.676659984169}
```

## Screenshots

[![The Environmental Engineering desktop app: Fraction of discrete particles removed in an ideal settling basin, with its worked example and live scene](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/environmental-engineering-ai999x/)

[![Concentration after mixing a discharge with a river: the live scene in the Environmental Engineering desktop app](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/environmental-engineering-ai999x/)

*Concentration after mixing a discharge with a river*

[![Landfill methane generation (first-order decay, constant acceptance): the live scene in the Environmental Engineering desktop app](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/environmental-engineering-ai999x/)

*Landfill methane generation (first-order decay, constant acceptance)*

[![Dissolved concentration in equilibrium with a gas (Henry's law): the live scene in the Environmental Engineering desktop app](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/environmental-engineering-ai999x/)

*Dissolved concentration in equilibrium with a gas (Henry's law)*

Every calculation, with its formula, a worked example and its figures: **[Environmental Engineering on ai999x.com](https://www.ai999x.com/models/environmental-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/environmental-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/environmental-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `environmental-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/environmental-engineering`) and its data (`rust/models/environmental_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p environmental-engineering-ai999x
./target/release/environmental-engineering --help
cargo test -p environmental-engineering-ai999x
```
