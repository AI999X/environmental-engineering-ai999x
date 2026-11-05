# environmental-engineering-ai999x — offline, verified environmental engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/environmental-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/environmental-engineering-ai999x)](https://pypi.org/project/environmental-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/environmental-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/environmental-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/environmental-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/environmental-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/environmental-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

environmental-engineering-ai999x is an offline, deterministic Python solver that answers environmental engineering questions with verified results from 99 certified capabilities, gives 613 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**environmental-engineering-ai999x** is a compact, deterministic environmental engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified environmental engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** environmental-engineering-ai999x comes as the AI999X desktop app and as the command-line program `environmental-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Environmental Engineering desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-environmental-engineering-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`environmental-engineering`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install environmental-engineering-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Environmental Engineering also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.0](https://github.com/AI999X/environmental-engineering-ai999x/releases/tag/v1.0.0) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Environmental Engineering) | Windows x86-64 | `ai999x-environmental-engineering-1.2.0-windows-x86_64.zip` | 7.2 MB | [Hub](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/rust/ai999x-environmental-engineering-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/environmental-engineering-ai999x/releases/download/v1.0.0/ai999x-environmental-engineering-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-environmental-engineering-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-environmental-engineering.exe` (no installer) |
| Desktop app (Environmental Engineering) | Linux x86-64 | `ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz` | 8.2 MB | [Hub](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/rust/ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/environmental-engineering-ai999x/releases/download/v1.0.0/ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-environmental-engineering` (X11 or Wayland desktop) |
| Command line (`environmental-engineering`) | Windows x86-64 | `ai999x-environmental-engineering-cli-1.2.0-windows-x86_64.zip` | 3.0 MB | [Hub](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/rust/ai999x-environmental-engineering-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/environmental-engineering-ai999x/releases/download/v1.0.0/ai999x-environmental-engineering-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `environmental-engineering.exe ask "<question>"` or `environmental-engineering.exe solve <capability> name=value` |
| Command line (`environmental-engineering`) | Linux x86-64 | `ai999x-environmental-engineering-cli-1.2.0-linux-x86_64.tar.gz` | 2.9 MB | [Hub](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/rust/ai999x-environmental-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/environmental-engineering-ai999x/releases/download/v1.0.0/ai999x-environmental-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./environmental-engineering ask "<question>"` or `./environmental-engineering solve <capability> name=value` |

The command-line archive holds `environmental-engineering` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Environmental Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Environmental Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-environmental-engineering-1.2.0-linux-x86_64.tar.gz
./ai999x-environmental-engineering-1.2.0-linux-x86_64/ai999x-environmental-engineering
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
environmental-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
environmental-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ environmental-engineering ask "Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L"
Read as: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time
D = 0.002726165085 kg/m^3
DO = 0.006373834915 kg/m^3
D_mgL = 2.726165085
DO_mgL = 6.373834915
  by: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time  (D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t))
  D0 = 0.001 kg/m^3
  DOsat = 0.0091 kg/m^3
  L0 = 0.01 kg/m^3
  ka = 5.324074074e-06 1/s
  kd = 2.662037037e-06 1/s
  t = 172800 s
  assumes: first-order deoxygenation and reaeration; plug flow; deficit stays below saturation
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ environmental-engineering info enveng_streeter_phelps_deficit
Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time
  D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t)
  in   kd       deoxygenation rate constant [1/s]
  in   ka       reaeration rate constant [1/s]
  in   L0       ultimate BOD of the mixed river water at the outfall [kg/m^3]
  in   D0       oxygen deficit at the outfall (DOsat - DO) [kg/m^3]
  in   t        travel time from the outfall [s]
  in   DOsat    saturation dissolved oxygen [kg/m^3]
  out  D        oxygen deficit at t [kg/m^3]
  out  DO       dissolved oxygen at t [kg/m^3]
  out  D_mgL    deficit in mg/L []
  out  DO_mgL   dissolved oxygen in mg/L []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ environmental-engineering solve enveng_streeter_phelps_deficit "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "t=2 day" "DOsat=9.1 mg/L"
D = 0.002726165085 kg/m^3
DO = 0.006373834915 kg/m^3
D_mgL = 2.726165085
DO_mgL = 6.373834915
  by: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time  (D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t))
  kd = 2.662037037e-06 1/s
  ka = 5.324074074e-06 1/s
  L0 = 0.01 kg/m^3
  D0 = 0.001 kg/m^3
  t = 172800 s
  DOsat = 0.0091 kg/m^3
  assumes: first-order deoxygenation and reaeration; plug flow; deficit stays below saturation
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | environmental-engineering batch

`cases.jsonl`:

```json
{"key": "enveng_streeter_phelps_deficit", "inputs": {"kd": "0.23 1/day", "ka": "0.46 1/day", "L0": "10 mg/L", "D0": "1 mg/L", "t": "2 day", "DOsat": "9.1 mg/L"}}
{"key": "enveng_briggs_plume_rise", "inputs": {"vs": "15 m/s", "d": "3 m", "Ts": "420 K", "Ta": "288 K", "u": "5 m/s", "hs": "100 m"}}
```

```console
$ environmental-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.00637383491469,"unit":"kg/m^3","solved_for":"DO","si":[["kd",2.662037037037037e-6],["ka",5.324074074074074e-6],["L0",0.01],["D0",0.001],["t",172800.0],["DOsat",0.0091]],"values":[["D",0.00272616508531,"kg/m^3"],["DO",0.00637383491469,"kg/m^3"],["D_mgL",2.72616508531,""],["DO_mgL",6.37383491469,""]],"target":"DO","phrase":"oxygen deficit at t","assumptions":["first-order deoxygenation and reaeration","plug flow","deficit stays below saturation"],"name":"Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time","equation":"D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t)"}
{"outcome":"SOLVED","message":"","value":125.637535501,"unit":"m","solved_for":"dh","si":[["vs",15.0],["d",3.0],["Ts",420.0],["Ta",288.0],["u",5.0],["hs",100.0]],"values":[["F",104.0205375,""],["x_f",762.771747653,"m"],["dh",125.637535501,"m"],["H",225.637535501,"m"]],"target":"dh","phrase":"buoyancy flux, m^4/s^3","assumptions":["Ts > Ta","neutral or unstable air (classes A-D)","buoyancy-dominated plume"],"name":"Briggs plume rise and effective stack height (neutral or unstable air)","equation":"F = g vs d^2 (Ts - Ta)/(4 Ts); x_f = 49 F^(5/8) (F < 55) or 119 F^(2/5); dh = 1.6 F^(1/3) x_f^(2/3)/u"}
```

### In your browser

Open [Environmental Engineering on ai999x.com](https://www.ai999x.com/models/environmental-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `environmental_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install environmental-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Box model of an air basin (well-mixed, time-dependent)

The concentration over a city treated as a well-mixed box: emission flux per area, wind through a box of length L under a mixing height, background air; the concentration after time t and at steady state.

Command line:

```console
$ environmental-engineering solve enveng_box_model "q=4 ug/(m^2*s)" "L=15000 m" "u=3 m/s" "Hm=600 m" "Cb=20 ug/m^3" "C0=20 ug/m^3" "t=2 h"
C = 4.543574138e-08 kg/m^3
C_ss = 5.333333333e-08 kg/m^3
C_ugm3 = 45.43574138
  by: Box model of an air basin (well-mixed, time-dependent)  (dC/dt = q/Hm - u (C - Cb)/L, so C = Css + (C0 - Css) e^(-u t/L) with Css = Cb + q L/(u Hm))
  q = 4e-09 kg/(m^2*s)
  L = 15000 m
  u = 3 m/s
  Hm = 600 m
  Cb = 2e-08 kg/m^3
  C0 = 2e-08 kg/m^3
  t = 7200 s
  assumes: well-mixed box; constant wind, mixing height and emissions
```

Python:

```python
import environmental_engineering_ai999x as m

a = m.solve("enveng_box_model", q="4 ug/(m^2*s)", L="15000 m", u="3 m/s", Hm="600 m", Cb="20 ug/m^3", C0="20 ug/m^3", t="2 h")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#cap-enveng_box_model) and press **Compute in your browser**.

### Briggs dispersion coefficients sigma_y and sigma_z

The horizontal and vertical plume spreads (Pasquill-Gifford curves in Briggs' formulas) at a downwind distance for a stability class, open-country or urban.

Command line:

```console
$ environmental-engineering solve enveng_briggs_dispersion_coefficients "x=1000 m" cls=d terrain=rural
sigma_y = 76.27700714 m
sigma_z = 37.94733192 m
  by: Briggs dispersion coefficients sigma_y and sigma_z  (Briggs (1973) interpolation formulas sigma = a x (1 + b x)^p for each class and terrain)
  x = 1000 m
  cls = d
  terrain = rural
  assumes: 100 m <= x <= 10 km
```

Python:

```python
import environmental_engineering_ai999x as m

a = m.solve("enveng_briggs_dispersion_coefficients", x="1000 m", cls="d", terrain="rural")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#cap-enveng_briggs_dispersion_coefficients) and press **Compute in your browser**.

### Briggs plume rise and effective stack height (neutral or unstable air)

The buoyancy flux, the final plume rise by Briggs' 2/3 law (neutral or unstable air) and the effective stack height.

Command line:

```console
$ environmental-engineering solve enveng_briggs_plume_rise "vs=15 m/s" "d=3 m" "Ts=420 K" "Ta=288 K" "u=5 m/s" "hs=100 m"
F = 104.0205375
x_f = 762.7717477 m
dh = 125.6375355 m
H = 225.6375355 m
  by: Briggs plume rise and effective stack height (neutral or unstable air)  (F = g vs d^2 (Ts - Ta)/(4 Ts); x_f = 49 F^(5/8) (F < 55) or 119 F^(2/5); dh = 1.6 F^(1/3) x_f^(2/3)/u)
  vs = 15 m/s
  d = 3 m
  Ts = 420 K
  Ta = 288 K
  u = 5 m/s
  hs = 100 m
  assumes: Ts > Ta; neutral or unstable air (classes A-D); buoyancy-dominated plume
```

Python:

```python
import environmental_engineering_ai999x as m

a = m.solve("enveng_briggs_plume_rise", vs="15 m/s", d="3 m", Ts="420 K", Ta="288 K", u="5 m/s", hs="100 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#cap-enveng_briggs_plume_rise) and press **Compute in your browser**.

### Briggs plume rise in stable air

The final rise of a buoyant plume in stable air (classes E, F) from the buoyancy flux, wind speed and potential-temperature gradient, and the effective stack height.

Command line:

```console
$ environmental-engineering solve enveng_briggs_stable_plume_rise "vs=15 m/s" "d=3 m" "Ts=420 K" "Ta=288 K" "u=4 m/s" "dtheta=0.02 K/m" "hs=100 m"
F = 104.0205375
s = 0.0006810173611
dh = 87.55352869 m
H = 187.5535287 m
  by: Briggs plume rise in stable air  (s = (g/Ta) dtheta/dz; dh = 2.6 (F/(u s))^(1/3))
  vs = 15 m/s
  d = 3 m
  Ts = 420 K
  Ta = 288 K
  u = 4 m/s
  dtheta = 0.02 K/m
  hs = 100 m
  assumes: Ts > Ta; stable air: dtheta/dz > 0
```

Python:

```python
import environmental_engineering_ai999x as m

a = m.solve("enveng_briggs_stable_plume_rise", vs="15 m/s", d="3 m", Ts="420 K", Ta="288 K", u="4 m/s", dtheta="0.02 K/m", hs="100 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/environmental-engineering-ai999x/#cap-enveng_briggs_stable_plume_rise) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/environmental-engineering-ai999x/blob/main/docs/environmental_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/environmental-engineering-ai999x/blob/main/docs/environmental_engineering_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/environmental-engineering-ai999x/blob/main/docs/environmental_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/environmental-engineering-ai999x/blob/main/docs/environmental_engineering_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/environmental-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified environmental engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 613 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `environmental-engineering` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** air pollution dispersion, water quality, water and wastewater treatment, gas transfer, noise and solid waste.

## Model overview

| Property | Value |
|---|---|
| Package | `environmental-engineering-ai999x` 1.0.0 (import `environmental_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic environmental engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 99 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 613 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `environmental-engineering-ai999x "<question>"`; for the native program use `environmental-engineering` (above). The outputs above were produced by running this exact code on environmental-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `environmental-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time** · _Water quality_

> Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L

→ **D = 0.002726165085 kg/m^3, DO = 0.006373834915 kg/m^3, D_mgL = 2.726165085, DO_mgL = 6.373834915**

<details><summary>Steps</summary>

- Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L → D = 0.002726165085 kg/m^3, DO = 0.006373834915 kg/m^3, D_mgL = 2.726165085 kg/m^3, DO_mgL = 6.373834915 kg/m^3

</details>

**Briggs plume rise and effective stack height (neutral or unstable air)** · _Air pollution_

> Briggs plume rise: vs = 15 m/s, d = 3 m, Ts = 420 K, Ta = 288 K, u = 5 m/s, hs = 100 m

→ **F = 104.0205375, x_f = 762.7717477 m, dh = 125.6375355 m, H = 225.6375355 m**

<details><summary>Steps</summary>

- Briggs plume rise and effective stack height (neutral or unstable air): vs = 15 m/s, d = 3 m, Ts = 420 K, Ta = 288 K, u = 5 m/s, hs = 100 m → F = 104.0205375 m, x_f = 762.7717477 m, dh = 125.6375355 m, H = 225.6375355 m

</details>

**Gaussian plume concentration at a receptor (ground reflection)** · _Air pollution_

`m.solve("enveng_gaussian_plume_concentration", Q=0.1, u=4.0, sigma_y=60.0, sigma_z=30.0, H=50.0, y=20.0, z=0.0)`

→ **C = 1.042805659e-06 kg/m^3, C_ugm3 = 1042.805659**

<details><summary>Steps</summary>

- Gaussian plume concentration at a receptor (ground reflection): Q = 0.1 kg/s, u = 4 m/s, sigma_y = 60 m, sigma_z = 30 m, H = 50 m, y = 20 m, z = 0 m → C = 1.042805659e-06 kg/m^3, C_ugm3 = 1042.805659 kg/m^3

</details>

**Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)** · _Wastewater_

> Activated sludge complete-mix design: Y = 0.5, k = 5 1/d, Ks = 60 mg/L, kd = 0.06 1/d, SRT = 10 d, HRT = 6 h, S0 = 250 mg/L

→ **S = 0.004102564103 kg/m^3, S_mgL = 4.102564103, X = 3.073717949 kg/m^3, X_mgL = 3073.717949, Y_obs = 0.3125**

<details><summary>Steps</summary>

- Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty): Y = 0.5, k = 5 1/day, Ks = 60 mg/L, kd = 0.06 1/day, SRT = 10 day, HRT = 6 h, S0 = 250 mg/L → S = 0.004102564103 kg/m^3, S_mgL = 4.102564103 kg/m^3, X = 3.073717949 kg/m^3, X_mgL = 3073.717949 kg/m^3, Y_obs = 0.3125 kg/m^3

</details>

**Adding sound levels in decibels** · _Noise_

`m.solve("enveng_decibel_addition", L=[85.0, 85.0, 90.0])`

→ **L_total = 92.1284136**

<details><summary>Steps</summary>

- Adding sound levels in decibels: L = [85, 85, 90] → L_total = 92.1284136

</details>

**Packed-tower air stripping: stripping factor, transfer units and height** · _Gas transfer_

> Air stripping tower: Qw = 0.05 m^3/s, Qa = 1.5 m^3/s, H = 0.2, Cin = 200 ug/L, Cout = 5 ug/L, HTU = 1 m

→ **R = 6, NTU = 4.213854527, Z = 4.213854527 m, air_to_water_min = 4.875**

<details><summary>Steps</summary>

- Packed-tower air stripping: stripping factor, transfer units and height: Qw = 0.05 m^3/s, Qa = 1.5 m^3/s, H = 0.2, Cin = 200 ug/L, Cout = 5 ug/L, HTU = 1 m → R = 6 m, NTU = 4.213854527 m, Z = 4.213854527 m, air_to_water_min = 4.875 m

</details>

## Figures

Every point was computed by `environmental-engineering-ai999x`.

**Briggs plume rise and effective stack height (neutral or unstable air)**

![Briggs plume rise and effective stack height (neutral or unstable air) — computed by environmental-engineering-ai999x](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/figures/enveng-briggs-plume-rise-dh-vs-vs.svg)

_dh (plume rise) against vs (stack gas exit velocity), with Ta = 288 K, Ts = 420 K, d = 3 m, hs = 100 m, u = 5 m/s. 31 points computed by environmental-engineering-ai999x; marked: vs = 15, dh = 125.638 m._

**Packed-tower air stripping: stripping factor, transfer units and height**

![Packed-tower air stripping: stripping factor, transfer units and height — computed by environmental-engineering-ai999x](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/figures/enveng-air-stripping-tower-z-vs-qw.svg)

_Z (packed height HTU NTU) against Qw (water flow), with Cin = 2.00e-4 kg/m^3, Cout = 5.00e-6 kg/m^3, H = 0.2, HTU = 1 m, Qa = 1.5 m^3/s. 31 points computed by environmental-engineering-ai999x; marked: Qw = 0.05, Z = 4.21385 m._

**Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)**

![Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty) — computed by environmental-engineering-ai999x](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/figures/enveng-activated-sludge-cstr-design-s-vs-y.svg)

_S (effluent soluble substrate) against Y (true yield coefficient, kg VSS per kg BOD), with HRT = 21600 s, Ks = 0.06 kg/m^3, S0 = 0.25 kg/m^3, SRT = 8.64e5 s, k = 5.79e-5 1/s, kd = 6.94e-7 1/s. 31 points computed by environmental-engineering-ai999x; marked: Y = 0.5, S = 0.00410256 kg/m^3._

**Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time**

![Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time — computed by environmental-engineering-ai999x](https://huggingface.co/AI999X/environmental-engineering-ai999x/resolve/main/figures/enveng-streeter-phelps-deficit-do-vs-kd.svg)

_DO (dissolved oxygen at t) against kd (deoxygenation rate constant), with D0 = 0.001 kg/m^3, DOsat = 0.0091 kg/m^3, L0 = 0.01 kg/m^3, ka = 5.32e-6 1/s, t = 1.73e5 s. 31 points computed by environmental-engineering-ai999x; marked: kd = 2.66204e-06, DO = 0.00637383 kg/m^3._

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
