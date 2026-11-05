# environmental-engineering: the Environmental Engineering program — user guide

**environmental-engineering-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Environmental Engineering covers Gaussian plume dispersion and plume rise, BOD and Streeter-Phelps oxygen sag, water and wastewater treatment design, activated sludge, disinfection CT, Henry's law and air stripping, noise and solid waste. This guide is about `environmental-engineering`, the model as a self-contained program for Windows and Linux: 99 capabilities in 7 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/environmental-engineering-ai999x/); the Python package (`pip install environmental-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS; the newest version is always under https://www.ai999x.com/downloads/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Environmental Engineering | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-environmental-engineering-1.4.0-windows-x86_64.zip |
| Desktop app: Environmental Engineering | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-environmental-engineering-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Environmental Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-environmental-engineering-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-environmental-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-environmental-engineering-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-environmental-engineering-1.4.0-linux-x86_64.tar.gz
./ai999x-environmental-engineering-1.4.0-linux-x86_64/ai999x-environmental-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `environmental-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
environmental-engineering --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
environmental-engineering --version
```

### In your browser

Open https://www.ai999x.com/models/environmental-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install environmental-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ environmental-engineering --version
environmental-engineering-ai999x 1.0.0 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ environmental-engineering search "streeter-phelps oxygen deficit and dissolved oxygen at a travel time"
enveng_streeter_phelps_deficit       Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time  (water quality / oxygen sag)
enveng_oxygen_sag_distance           Dissolved oxygen at a distance downstream (Streeter-Phelps)  (water quality / oxygen sag)
enveng_streeter_phelps_critical_point Critical (minimum) dissolved oxygen of the oxygen sag  (water quality / oxygen sag)
enveng_aeration_do_curve             Dissolved oxygen during aeration and the transfer rate  (wastewater / aeration)
enveng_do_saturation                 Dissolved oxygen saturation concentration (Benson-Krause)  (water quality / dissolved oxygen)
enveng_kla_two_point                 Overall oxygen transfer coefficient KLa from two DO readings  (wastewater / aeration)
enveng_theoretical_oxygen_demand     Theoretical oxygen demand of an organic compound  (water quality / oxygen demand)
enveng_aeration_oxygen_requirement   Oxygen requirement of an activated-sludge process  (wastewater / activated sludge)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ environmental-engineering list
enveng_activated_sludge_cstr_design      Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)
enveng_aeration_do_curve                 Dissolved oxygen during aeration and the transfer rate
enveng_aeration_oxygen_requirement       Oxygen requirement of an activated-sludge process
enveng_air_stripping_tower               Packed-tower air stripping: stripping factor, transfer units and height
enveng_anaerobic_methane_yield           Methane production from COD removed (anaerobic digestion)
enveng_background_noise_correction       Source level after removing background noise
enveng_backwash_bed_expansion            Filter bed expansion during backwash
enveng_bod_dilution_test                 BOD from a dilution test (with optional seed correction)
… 91 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ environmental-engineering solve enveng_streeter_phelps_deficit "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "t=2 day" "DOsat=9.1 mg/L" --json
{"outcome":"SOLVED","message":"","value":0.00637383491469,"unit":"kg/m^3","solved_for":"DO","si":[["kd",2.662037037037037e-6],["ka",5.324074074074074e-6],["L0",0.01],["D0",0.001],["t",172800.0],["DOsat",0.0091]],"values":[["D",0.00272616508531,"kg/m^3"],["DO",0.00637383491469,"kg/m^3"],["D_mgL",2.72616508531,""],["DO_mgL",6.37383491469,""]],"target":"DO","phrase":"oxygen deficit at t","assumptions":["first-order deoxygenation and reaeration","plug flow","deficit stays below saturation"],"name":"Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time","equation":"D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ environmental-engineering solve enveng_streeter_phelps_deficit "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "t=2 day"
NEEDS_INPUT: enveng_streeter_phelps_deficit needs DOsat (saturation dissolved oxygen). Example: solve('enveng_streeter_phelps_deficit', kd='2.66204e-06 1/s', ka='5.32407e-06 1/s', L0='0.01 kg/m^3', D0='0.001 kg/m^3', t='172800 s', DOsat='0.0091 kg/m^3').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ environmental-engineering figures enveng_streeter_phelps_deficit "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "t=2 day" "DOsat=9.1 mg/L" --out .
[
  {
    "slug": "enveng-streeter-phelps-deficit-do-vs-kd",
    "title": "DO against kd: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time",
    "caption": "DO (dissolved oxygen at t) against kd (deoxygenation rate constant) from 1.331e-06 1/s to 3.993e-06 1/s, the other inputs as given: 31 points, each a solve of the model. Marked: the given point, kd = 2.662e-06 1/s, DO = 0.006374 kg/m³. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-do-vs-kd.svg"
  },
  {
    "slug": "enveng-streeter-phelps-deficit-live-1",
    "title": "Dissolved oxygen at t as saturation dissolved oxygen varies",
    "caption": "Dissolved oxygen at t as DOsat runs from 2.957 to 15.93 mg/L, every other input as you entered it (39 solutions). Marked: your DOsat = 9.1 mg/L gives DO = 0.006374. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-live-1.svg"
  },
  {
    "slug": "enveng-streeter-phelps-deficit-live-2",
    "title": "Dissolved oxygen at t as ultimate BOD of the mixed river water at the ou... varies",
    "caption": "Dissolved oxygen at t as L0 runs from 2.5 to 17.5 mg/L, every other input as you entered it (29 solutions). Marked: your L0 = 10 mg/L gives DO = 0.006374. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | environmental-engineering batch

`cases.jsonl`:

```json
{"key": "enveng_streeter_phelps_deficit", "inputs": {"kd": "0.23 1/day", "ka": "0.46 1/day", "L0": "10 mg/L", "D0": "1 mg/L", "t": "2 day", "DOsat": "9.1 mg/L"}}
{"key": "enveng_briggs_plume_rise", "inputs": {"vs": "15 m/s", "d": "3 m", "Ts": "420 K", "Ta": "288 K", "u": "5 m/s", "hs": "100 m"}}
```

```shell
$ environmental-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.00637383491469,"unit":"kg/m^3","solved_for":"DO","si":[["kd",2.662037037037037e-6],["ka",5.324074074074074e-6],["L0",0.01],["D0",0.001],["t",172800.0],["DOsat",0.0091]],"values":[["D",0.00272616508531,"kg/m^3"],["DO",0.00637383491469,"kg/m^3"],["D_mgL",2.72616508531,""],["DO_mgL",6.37383491469,""]],"target":"DO","phrase":"oxygen deficit at t","assumptions":["first-order deoxygenation and reaeration","plug flow","deficit stays below saturation"],"name":"Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time","equation":"D = kd L0/(ka - kd) (e^(-kd t) - e^(-ka t)) + D0 e^(-ka t); for ka = kd, D = (kd L0 t + D0) e^(-kd t)"}
{"outcome":"SOLVED","message":"","value":125.637535501,"unit":"m","solved_for":"dh","si":[["vs",15.0],["d",3.0],["Ts",420.0],["Ta",288.0],["u",5.0],["hs",100.0]],"values":[["F",104.0205375,""],["x_f",762.771747653,"m"],["dh",125.637535501,"m"],["H",225.637535501,"m"]],"target":"dh","phrase":"buoyancy flux, m^4/s^3","assumptions":["Ts > Ta","neutral or unstable air (classes A-D)","buoyancy-dominated plume"],"name":"Briggs plume rise and effective stack height (neutral or unstable air)","equation":"F = g vs d^2 (Ts - Ta)/(4 Ts); x_f = 49 F^(5/8) (F < 55) or 119 F^(2/5); dh = 1.6 F^(1/3) x_f^(2/3)/u"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `environmental-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`environmental-engineering solve ... --json` prints one answer as one line of JSON. `environmental-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | environmental-engineering batch`. `environmental-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["environmental-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`environmental-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ environmental-engineering figures enveng_streeter_phelps_deficit "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "t=2 day" "DOsat=9.1 mg/L" --out .
[
  {
    "slug": "enveng-streeter-phelps-deficit-do-vs-kd",
    "title": "DO against kd: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time",
    "caption": "DO (dissolved oxygen at t) against kd (deoxygenation rate constant) from 1.331e-06 1/s to 3.993e-06 1/s, the other inputs as given: 31 points, each a solve of the model. Marked: the given point, kd = 2.662e-06 1/s, DO = 0.006374 kg/m³. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-do-vs-kd.svg"
  },
  {
    "slug": "enveng-streeter-phelps-deficit-live-1",
    "title": "Dissolved oxygen at t as saturation dissolved oxygen varies",
    "caption": "Dissolved oxygen at t as DOsat runs from 2.957 to 15.93 mg/L, every other input as you entered it (39 solutions). Marked: your DOsat = 9.1 mg/L gives DO = 0.006374. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-live-1.svg"
  },
  {
    "slug": "enveng-streeter-phelps-deficit-live-2",
    "title": "Dissolved oxygen at t as ultimate BOD of the mixed river water at the ou... varies",
    "caption": "Dissolved oxygen at t as L0 runs from 2.5 to 17.5 mg/L, every other input as you entered it (29 solutions). Marked: your L0 = 10 mg/L gives DO = 0.006374. Your inputs: kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L.",
    "file": ".\\enveng-streeter-phelps-deficit-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `environmental_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/environmental_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("environmental_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `environmental-engineering info <id>` shows the same inputs and outputs in the terminal.

### Air pollution (21)

<a id="enveng_box_model"></a>

#### Box model of an air basin (well-mixed, time-dependent) — `enveng_box_model`

The concentration over a city treated as a well-mixed box: emission flux per area, wind through a box of length L under a mixing height, background air; the concentration after time t and at steady state.

$$
C(t) = C_{ss} + (C_0 - C_{ss})e^{-ut/L},\quad C_{ss} = C_b + \frac{qL}{uH_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | emission rate per unit ground area | kg/(m^2*s) |
| `L` | in | box length along the wind | m |
| `u` | in | wind speed | m/s |
| `Hm` | in | mixing height | m |
| `Cb` | in | background (upwind) concentration | kg/m^3 |
| `C0` | in | concentration at t = 0 | kg/m^3 |
| `t` | in | elapsed time | s |
| `C` | out | concentration at time t | kg/m^3 |
| `C_ss` | out | steady-state concentration | kg/m^3 |
| `C_ugm3` | out | concentration at t in micrograms per cubic metre | — |

**Applies when:** well-mixed box; constant wind, mixing height and emissions.

```shell
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

In the worked example, q = 4 ug/(m^2*s), L = 15000 m, u = 3 m/s, Hm = 600 m, Cb = 20 ug/m^3, C0 = 20 ug/m^3, t = 2 h go through the equation above and give C = 4.543574138e-08 kg/m^3, C_ss = 5.333333333e-08 kg/m^3, C_ugm3 = 45.43574138. It applies only when well-mixed box; constant wind, mixing height and emissions: outside that it says what is wrong instead of returning a number.

<a id="enveng_briggs_dispersion_coefficients"></a>

#### Briggs dispersion coefficients sigma_y and sigma_z — `enveng_briggs_dispersion_coefficients`

The horizontal and vertical plume spreads (Pasquill-Gifford curves in Briggs' formulas) at a downwind distance for a stability class, open-country or urban.

$$
\sigma_y = \frac{a_y x}{\sqrt{1 + b_y x}},\quad \sigma_z = a_z x\,(1 + b_z x)^{p_z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | downwind distance from the source | m |
| `cls` | in | Pasquill stability class A-F (urban: A and B share A-B, E and F share E-F) | — |
| `terrain` | in | rural (open country) or urban coefficients | — |
| `sigma_y` | out | horizontal (crosswind) dispersion coefficient | m |
| `sigma_z` | out | vertical dispersion coefficient | m |

**Applies when:** 100 m <= x <= 10 km.

```shell
$ environmental-engineering solve enveng_briggs_dispersion_coefficients "x=1000 m" cls=d terrain=rural
sigma_y = 76.27700714 m
sigma_z = 37.94733192 m
  by: Briggs dispersion coefficients sigma_y and sigma_z  (Briggs (1973) interpolation formulas sigma = a x (1 + b x)^p for each class and terrain)
  x = 1000 m
  cls = d
  terrain = rural
  assumes: 100 m <= x <= 10 km
```

In the worked example, x = 1000 m, cls = d, terrain = rural go through the equation above and give sigma_y = 76.27700714 m, sigma_z = 37.94733192 m. It applies only when 100 m <= x <= 10 km: outside that it says what is wrong instead of returning a number.

<a id="enveng_briggs_plume_rise"></a>

#### Briggs plume rise and effective stack height (neutral or unstable air) — `enveng_briggs_plume_rise`

The buoyancy flux, the final plume rise by Briggs' 2/3 law (neutral or unstable air) and the effective stack height.

$$
F = \frac{g v_s d^2 (T_s - T_a)}{4T_s},\quad \Delta h = \frac{1.6F^{1/3}x_f^{2/3}}{u}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `vs` | in | stack gas exit velocity | m/s |
| `d` | in | stack inside diameter at the exit | m |
| `Ts` | in | stack gas temperature | K |
| `Ta` | in | ambient air temperature | K |
| `u` | in | wind speed at the stack top | m/s |
| `hs` | in | physical stack height | m |
| `F` | out | buoyancy flux, m^4/s^3 | — |
| `x_f` | out | distance to final rise | m |
| `dh` | out | plume rise | m |
| `H` | out | effective stack height hs + dh | m |

**Applies when:** Ts > Ta; neutral or unstable air (classes A-D); buoyancy-dominated plume.

```shell
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

In the worked example, vs = 15 m/s, d = 3 m, Ts = 420 K, Ta = 288 K, u = 5 m/s, hs = 100 m go through the equation above and give F = 104.0205375, x_f = 762.7717477 m, dh = 125.6375355 m, H = 225.6375355 m. It applies only when Ts > Ta; neutral or unstable air (classes A-D); buoyancy-dominated plume: outside that it says what is wrong instead of returning a number.

<a id="enveng_briggs_stable_plume_rise"></a>

#### Briggs plume rise in stable air — `enveng_briggs_stable_plume_rise`

The final rise of a buoyant plume in stable air (classes E, F) from the buoyancy flux, wind speed and potential-temperature gradient, and the effective stack height.

$$
\Delta h = 2.6\left(\frac{F}{u s}\right)^{1/3},\quad s = \frac{g}{T_a}\frac{\partial\theta}{\partial z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `vs` | in | stack gas exit velocity | m/s |
| `d` | in | stack inside diameter at the exit | m |
| `Ts` | in | stack gas temperature | K |
| `Ta` | in | ambient air temperature | K |
| `u` | in | wind speed at the stack top | m/s |
| `dtheta` | in | potential temperature gradient d(theta)/dz | K/m |
| `hs` | in | physical stack height | m |
| `F` | out | buoyancy flux, m^4/s^3 | — |
| `s` | out | stability parameter g/Ta dtheta/dz, 1/s^2 | — |
| `dh` | out | plume rise | m |
| `H` | out | effective stack height | m |

**Applies when:** Ts > Ta; stable air: dtheta/dz > 0.

```shell
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

In the worked example, vs = 15 m/s, d = 3 m, Ts = 420 K, Ta = 288 K, u = 4 m/s, dtheta = 0.02 K/m, hs = 100 m go through the equation above and give F = 104.0205375, s = 0.0006810173611, dh = 87.55352869 m, H = 187.5535287 m. It applies only when Ts > Ta; stable air: dtheta/dz > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_cyclone_lapple"></a>

#### Cyclone cut diameter and fractional efficiency (Lapple) — `enveng_cyclone_lapple`

The cut diameter (50 % efficiency) of a cyclone from the gas viscosity, inlet width, effective number of turns, inlet velocity and densities, and the collection efficiency of a particle size by Lapple's curve.

$$
d_{50} = \sqrt{\frac{9\mu W}{2\pi N_e V_i(\rho_p - \rho_g)}},\quad \eta = \frac{1}{1 + (d_{50}/d_p)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | gas viscosity | Pa*s |
| `W` | in | inlet width | m |
| `Ne` | in | effective number of turns | — |
| `Vi` | in | inlet gas velocity | m/s |
| `rho_p` | in | particle density | kg/m^3 |
| `rho_g` | in | gas density | kg/m^3 |
| `dp` | in | particle diameter | m |
| `d50` | out | cut diameter | m |
| `eta` | out | collection efficiency of particles of diameter dp | — |

**Applies when:** rho_p > rho_g.

```shell
$ environmental-engineering solve enveng_cyclone_lapple "mu=1.8e-05 Pa*s" "W=0.25 m" Ne=6 "Vi=15 m/s" "rho_p=1500 kg/m^3" "rho_g=1.2 kg/m^3" "dp=10 um"
d50 = 6.912648602e-06 m
eta = 0.6766599842
  by: Cyclone cut diameter and fractional efficiency (Lapple)  (d50 = sqrt(9 mu W/(2 pi Ne Vi (rho_p - rho_g))); eta = 1/(1 + (d50/dp)^2))
  mu = 1.8e-05 Pa*s
  W = 0.25 m
  Ne = 6
  Vi = 15 m/s
  rho_p = 1500 kg/m^3
  rho_g = 1.2 kg/m^3
  dp = 1e-05 m
  assumes: rho_p > rho_g
```

In the worked example, mu = 1.8e-05 Pa*s, W = 0.25 m, Ne = 6, Vi = 15 m/s, rho_p = 1500 kg/m^3, rho_g = 1.2 kg/m^3, dp = 10 um go through the equation above and give d50 = 6.912648602e-06 m, eta = 0.6766599842. It applies only when rho_p > rho_g: outside that it says what is wrong instead of returning a number.

<a id="enveng_esp_efficiency"></a>

#### Electrostatic precipitator efficiency (Deutsch-Anderson) — `enveng_esp_efficiency`

The collection efficiency and penetration of an electrostatic precipitator from the drift velocity, collecting area and gas flow.

$$
\eta = 1 - e^{-wA/Q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | effective drift (migration) velocity | m/s |
| `A` | in | collecting plate area | m^2 |
| `Q` | in | gas flow rate | m^3/s |
| `eta` | out | collection efficiency (fraction) | — |
| `penetration` | out | fraction passing (1 - eta) | — |

**Applies when:** w, A, Q > 0.

```shell
$ environmental-engineering solve enveng_esp_efficiency "w=0.1 m/s" "A=5000 m^2" "Q=200 m^3/s"
eta = 0.9179150014
penetration = 0.08208499862
  by: Electrostatic precipitator efficiency (Deutsch-Anderson)  (eta = 1 - exp(-w A/Q))
  w = 0.1 m/s
  A = 5000 m^2
  Q = 200 m^3/s
  assumes: w, A, Q > 0
```

In the worked example, w = 0.1 m/s, A = 5000 m^2, Q = 200 m^3/s go through the equation above and give eta = 0.9179150014, penetration = 0.08208499862. It applies only when w, A, Q > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_esp_area"></a>

#### Electrostatic precipitator plate area for a target efficiency — `enveng_esp_area`

The collecting plate area an electrostatic precipitator needs to reach a stated efficiency (Deutsch-Anderson).

$$
A = -\frac{Q}{w}\ln(1-\eta)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | effective drift velocity | m/s |
| `Q` | in | gas flow rate | m^3/s |
| `eta` | in | required collection efficiency (fraction or %) | — |
| `A` | out | required collecting area | m^2 |

**Applies when:** 0 < eta < 1.

```shell
$ environmental-engineering solve enveng_esp_area "w=0.1 m/s" "Q=200 m^3/s" eta=0.99
A = 9210.340372 m^2
  by: Electrostatic precipitator plate area for a target efficiency  (A = -Q ln(1 - eta)/w)
  w = 0.1 m/s
  Q = 200 m^3/s
  eta = 0.99
  assumes: 0 < eta < 1
```

In the worked example, w = 0.1 m/s, Q = 200 m^3/s, eta = 0.99 go through the equation above and give A = 9210.340372 m^2. It applies only when 0 < eta < 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_mass_concentration_to_ppm"></a>

#### Gas concentration: mass concentration to ppm (by volume) — `enveng_mass_concentration_to_ppm`

The volume mixing ratio (ppm and ppb) of a gaseous pollutant from its mass concentration, molar mass, temperature and pressure (ideal gas).

$$
\text{ppm} = \frac{C R T}{P M}\times10^{6}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | mass concentration | kg/m^3 |
| `M` | in | molar mass | kg/mol |
| `T` | in | air temperature | K |
| `P` | in | air pressure | Pa |
| `ppm` | out | volume mixing ratio in ppm | — |
| `ppb` | out | volume mixing ratio in ppb | — |

**Applies when:** ideal gas.

```shell
$ environmental-engineering solve enveng_mass_concentration_to_ppm "C=100 ug/m^3" "M=46.006 g/mol" "T=298.15 K" "P=101.325 kPa"
ppm = 0.05317872386
ppb = 53.17872386
  by: Gas concentration: mass concentration to ppm (by volume)  (ppm = C R T/(P M) 1e6)
  C = 1e-07 kg/m^3
  M = 0.046006 kg/mol
  T = 298.15 K
  P = 101325 Pa
  assumes: ideal gas
```

In the worked example, C = 100 ug/m^3, M = 46.006 g/mol, T = 298.15 K, P = 101.325 kPa go through the equation above and give ppm = 0.05317872386, ppb = 53.17872386. It applies only when ideal gas: outside that it says what is wrong instead of returning a number.

<a id="enveng_ppm_to_mass_concentration"></a>

#### Gas concentration: ppm (by volume) to mass concentration — `enveng_ppm_to_mass_concentration`

The mass concentration of a gaseous pollutant from its volume mixing ratio in ppm, its molar mass and the air temperature and pressure (ideal gas).

$$
C = \text{ppm}\times10^{-6}\,\frac{P M}{R T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ppm` | in | volume mixing ratio in parts per million | — |
| `M` | in | molar mass of the pollutant | kg/mol |
| `T` | in | air temperature | K |
| `P` | in | air pressure | Pa |
| `C` | out | mass concentration | kg/m^3 |
| `C_ugm3` | out | in micrograms per cubic metre | — |
| `C_mgm3` | out | in milligrams per cubic metre | — |

**Applies when:** ideal gas.

```shell
$ environmental-engineering solve enveng_ppm_to_mass_concentration ppm=0.14 "M=64.066 g/mol" "T=298.15 K" "P=101.325 kPa"
C = 3.666091151e-07 kg/m^3
C_ugm3 = 366.6091151
C_mgm3 = 0.3666091151
  by: Gas concentration: ppm (by volume) to mass concentration  (C = ppm 1e-6 P M/(R T))
  ppm = 0.14
  M = 0.064066 kg/mol
  T = 298.15 K
  P = 101325 Pa
  assumes: ideal gas
```

In the worked example, ppm = 0.14, M = 64.066 g/mol, T = 298.15 K, P = 101.325 kPa go through the equation above and give C = 3.666091151e-07 kg/m^3, C_ugm3 = 366.6091151, C_mgm3 = 0.3666091151. It applies only when ideal gas: outside that it says what is wrong instead of returning a number.

<a id="enveng_gaussian_plume_concentration"></a>

#### Gaussian plume concentration at a receptor (ground reflection) — `enveng_gaussian_plume_concentration`

The steady concentration downwind of a continuous point source at crosswind distance y and height z, given the dispersion coefficients at the receptor's distance and the effective stack height, with full reflection at the ground.

$$
C = \frac{Q}{2\pi u\sigma_y\sigma_z} e^{-\frac{y^2}{2\sigma_y^2}}\left[e^{-\frac{(z-H)^2}{2\sigma_z^2}} + e^{-\frac{(z+H)^2}{2\sigma_z^2}}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pollutant emission rate | kg/s |
| `u` | in | mean wind speed at the stack height | m/s |
| `sigma_y` | in | horizontal dispersion coefficient at the receptor distance | m |
| `sigma_z` | in | vertical dispersion coefficient at the receptor distance | m |
| `H` | in | effective stack height (stack height plus plume rise) | m |
| `y` | in | crosswind distance from the plume centreline | m |
| `z` | in | receptor height above the ground | m |
| `C` | out | concentration at the receptor | kg/m^3 |
| `C_ugm3` | out | concentration in micrograms per cubic metre | — |

**Applies when:** Q, u, sigma_y, sigma_z > 0; H, z >= 0; steady state, flat terrain.

```shell
$ environmental-engineering solve enveng_gaussian_plume_concentration "Q=100 g/s" "u=4 m/s" "sigma_y=60 m" "sigma_z=30 m" "H=50 m" "y=20 m" "z=0 m"
C = 1.042805659e-06 kg/m^3
C_ugm3 = 1042.805659
  by: Gaussian plume concentration at a receptor (ground reflection)  (C = Q/(2 pi u sy sz) exp(-y^2/2sy^2) [exp(-(z-H)^2/2sz^2) + exp(-(z+H)^2/2sz^2)])
  Q = 0.1 kg/s
  u = 4 m/s
  sigma_y = 60 m
  sigma_z = 30 m
  H = 50 m
  y = 20 m
  z = 0 m
  assumes: Q, u, sigma_y, sigma_z > 0; H, z >= 0; steady state, flat terrain
```

In the worked example, Q = 100 g/s, u = 4 m/s, sigma_y = 60 m, sigma_z = 30 m, H = 50 m, y = 20 m, z = 0 m go through the equation above and give C = 1.042805659e-06 kg/m^3, C_ugm3 = 1042.805659. It applies only when Q, u, sigma_y, sigma_z > 0; H, z >= 0; steady state, flat terrain: outside that it says what is wrong instead of returning a number.

<a id="enveng_plume_concentration_stability"></a>

#### Gaussian plume concentration from the stability class — `enveng_plume_concentration_stability`

The concentration at a receptor (x, y, z) downwind of an elevated point source, with the dispersion coefficients taken from Briggs' formulas for the stability class and terrain.

$$
C = \frac{Q}{2\pi u\sigma_y(x)\sigma_z(x)} e^{-\frac{y^2}{2\sigma_y^2}}\left[e^{-\frac{(z-H)^2}{2\sigma_z^2}} + e^{-\frac{(z+H)^2}{2\sigma_z^2}}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pollutant emission rate | kg/s |
| `u` | in | mean wind speed at the stack height | m/s |
| `H` | in | effective stack height | m |
| `x` | in | downwind distance | m |
| `y` | in | crosswind distance from the centreline | m |
| `z` | in | receptor height | m |
| `cls` | in | Pasquill stability class A-F | — |
| `terrain` | in | rural or urban dispersion coefficients | — |
| `C` | out | concentration at the receptor | kg/m^3 |
| `C_ugm3` | out | concentration in micrograms per cubic metre | — |
| `sigma_y` | out | horizontal dispersion coefficient at x | m |
| `sigma_z` | out | vertical dispersion coefficient at x | m |

**Applies when:** 100 m <= x <= 10 km; Q, u > 0; H, z >= 0.

```shell
$ environmental-engineering solve enveng_plume_concentration_stability "Q=200 g/s" "u=5 m/s" "H=80 m" "x=2000 m" "y=0 m" "z=0 m" cls=c terrain=rural
C = 3.935687085e-07 kg/m^3
C_ugm3 = 393.5687085
sigma_y = 200.8316044 m
sigma_z = 135.2246808 m
  by: Gaussian plume concentration from the stability class  (Briggs sigma_y(x), sigma_z(x) for the class, then the reflected Gaussian plume)
  Q = 0.2 kg/s
  u = 5 m/s
  H = 80 m
  x = 2000 m
  y = 0 m
  z = 0 m
  cls = c
  terrain = rural
  assumes: 100 m <= x <= 10 km; Q, u > 0; H, z >= 0
```

In the worked example, Q = 200 g/s, u = 5 m/s, H = 80 m, x = 2000 m, y = 0 m, z = 0 m, cls = c, terrain = rural go through the equation above and give C = 3.935687085e-07 kg/m^3, C_ugm3 = 393.5687085, sigma_y = 200.8316044 m, sigma_z = 135.2246808 m. It applies only when 100 m <= x <= 10 km; Q, u > 0; H, z >= 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_plume_mixing_lid"></a>

#### Gaussian plume trapped below an inversion lid (multiple reflections) — `enveng_plume_mixing_lid`

The concentration of a plume reflected both at the ground and at the base of an elevated inversion (mixing height), by the image-source series, with the fully mixed limit for comparison.

$$
C = \frac{Q e^{-y^2/2\sigma_y^2}}{2\pi u\sigma_y\sigma_z}\sum_{n=-\infty}^{\infty}\left[e^{-\frac{(z-H+2nL)^2}{2\sigma_z^2}} + e^{-\frac{(z+H+2nL)^2}{2\sigma_z^2}}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | emission rate | kg/s |
| `u` | in | mean wind speed | m/s |
| `sigma_y` | in | horizontal dispersion coefficient | m |
| `sigma_z` | in | vertical dispersion coefficient | m |
| `H` | in | effective stack height | m |
| `Lm` | in | mixing height (inversion base) | m |
| `y` | in | crosswind distance | m |
| `z` | in | receptor height | m |
| `C` | out | concentration with ground and lid reflections | kg/m^3 |
| `C_ugm3` | out | in micrograms per cubic metre | — |
| `C_mixed` | out | uniformly mixed limit Q/(sqrt(2 pi) u sy Lm) e^(-y^2/2sy^2) | kg/m^3 |

**Applies when:** H, z <= Lm; sigma_z <= 10 Lm.

```shell
$ environmental-engineering solve enveng_plume_mixing_lid "Q=100 g/s" "u=4 m/s" "sigma_y=900 m" "sigma_z=600 m" "H=80 m" "Lm=500 m" "y=0 m" "z=0 m"
C = 2.219531498e-08 kg/m^3
C_ugm3 = 22.19531498
C_mixed = 2.216346002e-08 kg/m^3
  by: Gaussian plume trapped below an inversion lid (multiple reflections)  (image sources at z = +-H + 2 n Lm summed until the terms vanish)
  Q = 0.1 kg/s
  u = 4 m/s
  sigma_y = 900 m
  sigma_z = 600 m
  H = 80 m
  Lm = 500 m
  y = 0 m
  z = 0 m
  assumes: H, z <= Lm; sigma_z <= 10 Lm
```

In the worked example, Q = 100 g/s, u = 4 m/s, sigma_y = 900 m, sigma_z = 600 m, H = 80 m, Lm = 500 m, y = 0 m, z = 0 m go through the equation above and give C = 2.219531498e-08 kg/m^3, C_ugm3 = 22.19531498, C_mixed = 2.216346002e-08 kg/m^3. It applies only when H, z <= Lm; sigma_z <= 10 Lm: outside that it says what is wrong instead of returning a number.

<a id="enveng_settling_chamber_efficiency"></a>

#### Gravity settling chamber collection efficiency — `enveng_settling_chamber_efficiency`

The Stokes settling velocity of a particle in a gas and the collection efficiency of a horizontal-flow settling chamber with N trays (laminar plug-flow model).

$$
\eta = \frac{v_t L W N}{Q},\quad v_t = \frac{g(\rho_p - \rho_g)d_p^2}{18\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dp` | in | particle diameter | m |
| `rho_p` | in | particle density | kg/m^3 |
| `rho_g` | in | gas density | kg/m^3 |
| `mu` | in | gas viscosity | Pa*s |
| `L` | in | chamber length | m |
| `W` | in | chamber width | m |
| `Q` | in | gas flow rate | m^3/s |
| `N` | in | number of trays (1 for a single chamber) | — |
| `vt` | out | Stokes terminal settling velocity | m/s |
| `eta` | out | collection efficiency (fraction, at most 1) | — |
| `Re` | out | particle Reynolds number | — |

**Applies when:** Stokes regime Re < 1; laminar plug flow.

```shell
$ environmental-engineering solve enveng_settling_chamber_efficiency "dp=50 um" "rho_p=2000 kg/m^3" "rho_g=1.2 kg/m^3" "mu=1.8e-05 Pa*s" "L=5 m" "W=2 m" "Q=10 m^3/s" N=1
vt = 0.151246389 m/s
eta = 0.151246389
Re = 0.5041546301
  by: Gravity settling chamber collection efficiency  (vt = g (rho_p - rho_g) dp^2/(18 mu); eta = min(1, vt L W N/Q))
  dp = 5e-05 m
  rho_p = 2000 kg/m^3
  rho_g = 1.2 kg/m^3
  mu = 1.8e-05 Pa*s
  L = 5 m
  W = 2 m
  Q = 10 m^3/s
  assumes: Stokes regime Re < 1; laminar plug flow
```

In the worked example, dp = 50 um, rho_p = 2000 kg/m^3, rho_g = 1.2 kg/m^3, mu = 1.8e-05 Pa*s, L = 5 m, W = 2 m, Q = 10 m^3/s, N = 1 go through the equation above and give vt = 0.151246389 m/s, eta = 0.151246389, Re = 0.5041546301. It applies only when Stokes regime Re < 1; laminar plug flow: outside that it says what is wrong instead of returning a number.

<a id="enveng_ground_centerline_concentration"></a>

#### Ground-level centreline concentration of a plume — `enveng_ground_centerline_concentration`

The ground-level concentration directly below the plume axis (y = 0, z = 0) from the emission rate, wind speed, dispersion coefficients and effective stack height (H = 0 for a ground-level source).

$$
C(x,0,0) = \frac{Q}{\pi u\sigma_y\sigma_z}\exp\!\left(-\frac{H^2}{2\sigma_z^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pollutant emission rate | kg/s |
| `u` | in | mean wind speed | m/s |
| `sigma_y` | in | horizontal dispersion coefficient | m |
| `sigma_z` | in | vertical dispersion coefficient | m |
| `H` | in | effective stack height (0 for a ground-level source) | m |
| `C` | out | ground-level centreline concentration | kg/m^3 |
| `C_ugm3` | out | concentration in micrograms per cubic metre | — |

**Applies when:** Q, u, sigma_y, sigma_z > 0; H >= 0.

```shell
$ environmental-engineering solve enveng_ground_centerline_concentration "Q=150 g/s" "u=5 m/s" "sigma_y=80 m" "sigma_z=40 m" "H=60 m"
C = 9.688133431e-07 kg/m^3
C_ugm3 = 968.8133431
  by: Ground-level centreline concentration of a plume  (C = Q/(pi u sy sz) exp(-H^2/(2 sz^2)))
  Q = 0.15 kg/s
  u = 5 m/s
  sigma_y = 80 m
  sigma_z = 40 m
  H = 60 m
  assumes: Q, u, sigma_y, sigma_z > 0; H >= 0
```

In the worked example, Q = 150 g/s, u = 5 m/s, sigma_y = 80 m, sigma_z = 40 m, H = 60 m go through the equation above and give C = 9.688133431e-07 kg/m^3, C_ugm3 = 968.8133431. It applies only when Q, u, sigma_y, sigma_z > 0; H >= 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_holland_plume_rise"></a>

#### Holland plume rise — `enveng_holland_plume_rise`

Plume rise by Holland's formula (momentum term plus a buoyancy term with the pressure in millibars) and the effective stack height.

$$
\Delta h = \frac{v_s d}{u}\left(1.5 + 2.68\times10^{-3}\,p\,d\,\frac{T_s - T_a}{T_s}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `vs` | in | stack gas exit velocity | m/s |
| `d` | in | stack inside diameter | m |
| `u` | in | wind speed at the stack top | m/s |
| `p` | in | atmospheric pressure | Pa |
| `Ts` | in | stack gas temperature | K |
| `Ta` | in | ambient air temperature | K |
| `hs` | in | physical stack height | m |
| `dh` | out | plume rise | m |
| `H` | out | effective stack height | m |

**Applies when:** Ts > Ta.

```shell
$ environmental-engineering solve enveng_holland_plume_rise "vs=10 m/s" "d=2 m" "u=4 m/s" "p=101.325 kPa" "Ts=400 K" "Ta=290 K" "hs=50 m"
dh = 14.9676525 m
H = 64.9676525 m
  by: Holland plume rise  (dh = vs d/u (1.5 + 2.68e-3 p[mbar] d (Ts - Ta)/Ts))
  vs = 10 m/s
  d = 2 m
  u = 4 m/s
  p = 101325 Pa
  Ts = 400 K
  Ta = 290 K
  hs = 50 m
  assumes: Ts > Ta
```

In the worked example, vs = 10 m/s, d = 2 m, u = 4 m/s, p = 101.325 kPa, Ts = 400 K, Ta = 290 K, hs = 50 m go through the equation above and give dh = 14.9676525 m, H = 64.9676525 m. It applies only when Ts > Ta: outside that it says what is wrong instead of returning a number.

<a id="enveng_max_ground_concentration_ratio"></a>

#### Maximum ground-level concentration (constant sigma_y/sigma_z) — `enveng_max_ground_concentration_ratio`

The largest ground-level centreline concentration of an elevated source when sigma_y/sigma_z is taken constant along the plume, and the vertical spread at which it occurs (sigma_z = H/sqrt 2).

$$
C_{max} = \frac{2Q}{\pi e\,u H^2}\frac{\sigma_z}{\sigma_y},\quad \sigma_z = \frac{H}{\sqrt 2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | emission rate | kg/s |
| `u` | in | mean wind speed | m/s |
| `H` | in | effective stack height | m |
| `k` | in | ratio sigma_y/sigma_z (constant along the plume) | — |
| `C_max` | out | maximum ground-level concentration | kg/m^3 |
| `C_max_ugm3` | out | maximum concentration in micrograms per cubic metre | — |
| `sigma_z_max` | out | vertical dispersion coefficient at the maximum | m |

**Applies when:** Q, u, H, k > 0; sigma_y/sigma_z constant along the plume.

```shell
$ environmental-engineering solve enveng_max_ground_concentration_ratio "Q=100 g/s" "u=5 m/s" "H=100 m" k=2
C_max = 2.341993261e-07 kg/m^3
C_max_ugm3 = 234.1993261
sigma_z_max = 70.71067812 m
  by: Maximum ground-level concentration (constant sigma_y/sigma_z)  (maximise C = Q/(pi u k sz^2) exp(-H^2/2sz^2) over sz: sz = H/sqrt2, Cmax = 2Q/(pi e u H^2 k))
  Q = 0.1 kg/s
  u = 5 m/s
  H = 100 m
  k = 2
  assumes: Q, u, H, k > 0; sigma_y/sigma_z constant along the plume
```

In the worked example, Q = 100 g/s, u = 5 m/s, H = 100 m, k = 2 go through the equation above and give C_max = 2.341993261e-07 kg/m^3, C_max_ugm3 = 234.1993261, sigma_z_max = 70.71067812 m. It applies only when Q, u, H, k > 0; sigma_y/sigma_z constant along the plume: outside that it says what is wrong instead of returning a number.

<a id="enveng_max_ground_concentration_briggs"></a>

#### Maximum ground-level concentration and its distance (Briggs coefficients) — `enveng_max_ground_concentration_briggs`

The distance of the largest ground-level centreline concentration of an elevated source and that concentration, found exactly on Briggs' dispersion curves for the stability class (no constant-ratio shortcut).

$$
\frac{d\ln C}{dx} = -\frac{\sigma_y'}{\sigma_y} + \frac{\sigma_z'}{\sigma_z}\left(\frac{H^2}{\sigma_z^2} - 1\right) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | emission rate | kg/s |
| `u` | in | mean wind speed | m/s |
| `H` | in | effective stack height | m |
| `cls` | in | Pasquill stability class A-F | — |
| `terrain` | in | rural or urban dispersion coefficients | — |
| `x_max` | out | downwind distance of the maximum | m |
| `C_max` | out | maximum ground-level concentration | kg/m^3 |
| `C_max_ugm3` | out | maximum concentration in micrograms per cubic metre | — |
| `sigma_y` | out | sigma_y at the maximum | m |
| `sigma_z` | out | sigma_z at the maximum | m |

**Applies when:** the maximum lies between 100 m and 10 km.

```shell
$ environmental-engineering solve enveng_max_ground_concentration_briggs "Q=100 g/s" "u=5 m/s" "H=50 m" cls=c terrain=rural
x_max = 459.3930757 m
C_max = 1.333568186e-06 kg/m^3
C_max_ugm3 = 1333.568186
sigma_y = 49.41102781 m
sigma_z = 35.17120532 m
  by: Maximum ground-level concentration and its distance (Briggs coefficients)  (root of d ln C/dx = -sy'/sy + (sz'/sz)(H^2/sz^2 - 1) by a log-spaced scan and bisection)
  Q = 0.1 kg/s
  u = 5 m/s
  H = 50 m
  cls = c
  terrain = rural
  assumes: the maximum lies between 100 m and 10 km
```

In the worked example, Q = 100 g/s, u = 5 m/s, H = 50 m, cls = c, terrain = rural go through the equation above and give x_max = 459.3930757 m, C_max = 1.333568186e-06 kg/m^3, C_max_ugm3 = 1333.568186, sigma_y = 49.41102781 m. It applies only when the maximum lies between 100 m and 10 km: outside that it says what is wrong instead of returning a number.

<a id="enveng_stability_class"></a>

#### Pasquill-Turner atmospheric stability class — `enveng_stability_class`

The Pasquill stability class (A very unstable to F stable) from the surface wind speed and the daytime insolation or night-time cloud cover.

$$
\text{class} = T(u,\ \text{insolation or cloud cover})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | surface wind speed at 10 m | m/s |
| `condition` | in | daytime insolation (strong, moderate, slight), overcast (day or night), night_cloudy (at least 4/8 low cloud) or night_clear (at most 3/8 cloud) | — |
| `stability` | out | Pasquill stability class (A-F; A-B etc. for intermediate cases) | — |

**Applies when:** u > 0; no class is defined at night with u below 2 m/s.

```shell
$ environmental-engineering solve enveng_stability_class "u=2.5 m/s" condition=strong
stability = A-B
  by: Pasquill-Turner atmospheric stability class  (Turner's table: wind-speed rows < 2, 2-3, 3-5, 5-6, > 6 m/s against insolation or cloud-cover columns; heavy overcast is D)
  u = 2.5 m/s
  condition = strong
  assumes: u > 0; no class is defined at night with u below 2 m/s
```

In the worked example, u = 2.5 m/s, condition = strong go through the equation above and give stability = A-B. It applies only when u > 0; no class is defined at night with u below 2 m/s: outside that it says what is wrong instead of returning a number.

<a id="enveng_stack_plume_ground_concentration"></a>

#### Stack plume: effective height, dispersion and ground-level concentration — `enveng_stack_plume_ground_concentration`

The whole chain for a buoyant stack in neutral or unstable air: Briggs plume rise to the effective height, Briggs dispersion coefficients at the receptor distance, then the ground-level centreline concentration.

$$
C = \frac{Q}{\pi u\sigma_y\sigma_z}\exp\!\left(-\frac{(h_s + \Delta h)^2}{2\sigma_z^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | emission rate | kg/s |
| `u` | in | wind speed at the stack top | m/s |
| `vs` | in | stack gas exit velocity | m/s |
| `d` | in | stack inside diameter at the exit | m |
| `Ts` | in | stack gas temperature | K |
| `Ta` | in | ambient air temperature | K |
| `hs` | in | physical stack height | m |
| `x` | in | downwind distance of the receptor | m |
| `cls` | in | Pasquill stability class A-D | — |
| `terrain` | in | rural or urban dispersion coefficients | — |
| `H` | out | effective stack height | m |
| `sigma_y` | out | sigma_y at x | m |
| `sigma_z` | out | sigma_z at x | m |
| `C` | out | ground-level centreline concentration at x | kg/m^3 |
| `C_ugm3` | out | in micrograms per cubic metre | — |

**Applies when:** Ts > Ta; class A-D; 100 m <= x <= 10 km.

```shell
$ environmental-engineering solve enveng_stack_plume_ground_concentration "Q=100 g/s" "u=6 m/s" "vs=10 m/s" "d=1.5 m" "Ts=400 K" "Ta=290 K" "hs=50 m" "x=3000 m" cls=c terrain=rural
H = 77.44719963 m
sigma_y = 289.4291464 m
sigma_z = 189.7366596 m
C = 8.888445322e-08 kg/m^3
C_ugm3 = 88.88445322
  by: Stack plume: effective height, dispersion and ground-level concentration  (Briggs final rise (2/3 law) -> H; Briggs sigma_y, sigma_z at x; C = Q/(pi u sy sz) exp(-H^2/2sz^2))
  Q = 0.1 kg/s
  u = 6 m/s
  vs = 10 m/s
  d = 1.5 m
  Ts = 400 K
  Ta = 290 K
  hs = 50 m
  x = 3000 m
  cls = c
  terrain = rural
  assumes: Ts > Ta; class A-D; 100 m <= x <= 10 km
```

In the worked example, Q = 100 g/s, u = 6 m/s, vs = 10 m/s, d = 1.5 m, Ts = 400 K, Ta = 290 K, hs = 50 m, x = 3000 m, cls = c, terrain = rural go through the equation above and give H = 77.44719963 m, sigma_y = 289.4291464 m, sigma_z = 189.7366596 m, C = 8.888445322e-08 kg/m^3. It applies only when Ts > Ta; class A-D; 100 m <= x <= 10 km: outside that it says what is wrong instead of returning a number.

<a id="enveng_so2_emission"></a>

#### Sulfur dioxide emission from fuel sulfur — `enveng_so2_emission`

The SO2 emission rate from a fuel burning rate and its sulfur mass fraction (all sulfur to SO2), before and after a control device.

$$
E = \dot m_{fuel}\,S\,\frac{M_{SO_2}}{M_S}(1-\eta)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m_fuel` | in | fuel burning rate | kg/s |
| `S` | in | sulfur mass fraction of the fuel (fraction or %) | — |
| `eta` | in | SO2 control (removal) efficiency (fraction or %; 0 if uncontrolled) | — |
| `E` | out | controlled SO2 emission rate | kg/s |
| `E_uncontrolled` | out | uncontrolled SO2 emission rate | kg/s |
| `E_kg_per_day` | out | controlled emission in kg per day | — |

**Applies when:** all fuel sulfur oxidised to SO2.

```shell
$ environmental-engineering solve enveng_so2_emission "m_fuel=36 t/h" S=0.02 eta=0.9
E = 0.03995870887 kg/s
E_uncontrolled = 0.3995870887 kg/s
E_kg_per_day = 3452.432447
  by: Sulfur dioxide emission from fuel sulfur  (E = m_fuel S (M_SO2/M_S)(1 - eta))
  m_fuel = 10 kg/s
  S = 0.02
  eta = 0.9
  assumes: all fuel sulfur oxidised to SO2
```

In the worked example, m_fuel = 36 t/h, S = 0.02, eta = 0.9 go through the equation above and give E = 0.03995870887 kg/s, E_uncontrolled = 0.3995870887 kg/s, E_kg_per_day = 3452.432447. It applies only when all fuel sulfur oxidised to SO2: outside that it says what is wrong instead of returning a number.

<a id="enveng_wind_power_law"></a>

#### Wind speed at height (power law) — `enveng_wind_power_law`

The wind speed at another height from a measured speed with the power-law profile u2 = u1 (z2/z1)^p (p from the stability class and terrain, e.g. 0.15 rural neutral).

$$
u_2 = u_1\left(\frac{z_2}{z_1}\right)^p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u1` | in | measured wind speed | m/s |
| `z1` | in | measurement height | m |
| `z2` | in | height of interest (e.g. the stack top) | m |
| `p` | in | power-law exponent (0 < p < 1) | — |
| `u2` | out | wind speed at z2 | m/s |

**Applies when:** 0 < p < 1.

```shell
$ environmental-engineering solve enveng_wind_power_law "u1=4 m/s" "z1=10 m" "z2=100 m" p=0.15
u2 = 5.650150178 m/s
  by: Wind speed at height (power law)  (u2 = u1 (z2/z1)^p)
  u1 = 4 m/s
  z1 = 10 m
  z2 = 100 m
  p = 0.15
  assumes: 0 < p < 1
```

In the worked example, u1 = 4 m/s, z1 = 10 m, z2 = 100 m, p = 0.15 go through the equation above and give u2 = 5.650150178 m/s. It applies only when 0 < p < 1: outside that it says what is wrong instead of returning a number.

### Gas transfer (5)

<a id="enveng_henry_law_equilibrium"></a>

#### Dissolved concentration in equilibrium with a gas (Henry's law) — `enveng_henry_law_equilibrium`

The equilibrium molar (and, with the molar mass, mass) concentration of a gas dissolved in water from its partial pressure and the Henry's volatility constant.

$$
C = \frac{p}{H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | partial pressure of the gas | Pa |
| `H` | in | Henry's volatility constant | Pa*m^3/mol |
| `M` | in | molar mass (for the mass concentration) (optional) | kg/mol |
| `C` | out | dissolved molar concentration | mol/m^3 |
| `C_mass` | out | dissolved mass concentration | kg/m^3 |
| `C_mgL` | out | in mg/L | — |

**Applies when:** dilute solution, equilibrium.

```shell
$ environmental-engineering solve enveng_henry_law_equilibrium "p=0.21 atm" "H=0.79 atm*m^3/mol" "M=31.9988 g/mol"
C = 0.2658227848 mol/m^3
C_mass = 0.008506010127 kg/m^3
C_mgL = 8.506010127
  by: Dissolved concentration in equilibrium with a gas (Henry's law)  (C = p/H)
  p = 21278.25 Pa
  H = 80046.75 Pa*m^3/mol
  M = 0.0319988 kg/mol
  assumes: dilute solution, equilibrium
```

In the worked example, p = 0.21 atm, H = 0.79 atm*m^3/mol, M = 31.9988 g/mol go through the equation above and give C = 0.2658227848 mol/m^3, C_mass = 0.008506010127 kg/m^3, C_mgL = 8.506010127. It applies only when dilute solution, equilibrium: outside that it says what is wrong instead of returning a number.

<a id="enveng_henry_constant_conversion"></a>

#### Henry's law constant conversion between forms — `enveng_henry_constant_conversion`

A Henry's law constant converted between its common forms: volatility (Pa m^3/mol, atm m^3/mol), dimensionless gas/liquid ratio at temperature T, mole-fraction form (atm) and solubility forms (mol/(m^3 Pa), mol/(L atm)).

$$
H_{cc} = \frac{H_{pc}}{RT},\quad H_{px} = H_{pc}\,c_w,\quad H_{cp} = \frac{1}{H_{pc}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | the Henry's constant value in the stated form | — |
| `form` | in | form of H: pa_m3_per_mol, atm_m3_per_mol, dimensionless (Cg/Cl), atm (p = H x), mol_per_m3_pa or m_per_atm (mol/(L atm)) | — |
| `T` | in | temperature (for the dimensionless form) | K |
| `H_pa_m3_per_mol` | out | volatility constant, Pa m^3/mol | — |
| `H_atm_m3_per_mol` | out | volatility constant, atm m^3/mol | — |
| `H_dimensionless` | out | dimensionless gas/liquid ratio at T | — |
| `H_atm` | out | mole-fraction form, atm | — |
| `H_mol_per_m3_pa` | out | solubility constant, mol/(m^3 Pa) | — |
| `H_m_per_atm` | out | solubility constant, mol/(L atm) | — |

**Applies when:** dilute aqueous solution (water 55344.59 mol/m^3).

```shell
$ environmental-engineering solve enveng_henry_constant_conversion H=0.0055 form=atm_m3_per_mol "T=298.15 K"
H_pa_m3_per_mol = 557.2875
H_atm_m3_per_mol = 0.0055
H_dimensionless = 0.2248072449
H_atm = 304.395245
H_mol_per_m3_pa = 0.001794405939
H_m_per_atm = 0.1818181818
  by: Henry's law constant conversion between forms  (all forms through the volatility constant Hpc: Hcc = Hpc/(R T), Hpx = Hpc rho_w, Hcp = 1/Hpc)
  H = 0.0055
  T = 298.15 K
  form = atm_m3_per_mol
  assumes: dilute aqueous solution (water 55344.59 mol/m^3)
```

In the worked example, H = 0.0055, form = atm_m3_per_mol, T = 298.15 K go through the equation above and give H_pa_m3_per_mol = 557.2875, H_atm_m3_per_mol = 0.0055, H_dimensionless = 0.2248072449, H_atm = 304.395245. It applies only when dilute aqueous solution (water 55344.59 mol/m^3): outside that it says what is wrong instead of returning a number.

<a id="enveng_two_film_transfer"></a>

#### Overall liquid-phase transfer coefficient and flux (two-film theory) — `enveng_two_film_transfer`

The overall liquid-side mass-transfer coefficient from the liquid- and gas-film coefficients and the dimensionless Henry's constant, the share of resistance in the liquid film, and the flux from water to air (negative when the gas dissolves).

$$
\frac{1}{K_L} = \frac{1}{k_l} + \frac{1}{Hk_g},\quad N = K_L\left(C - \frac{C_g}{H}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kl` | in | liquid-film coefficient | m/s |
| `kg` | in | gas-film coefficient | m/s |
| `H` | in | dimensionless Henry's constant (Cg/Cl) | — |
| `C` | in | bulk liquid concentration | kg/m^3 |
| `Cg` | in | bulk gas concentration | kg/m^3 |
| `KL` | out | overall liquid-phase coefficient | m/s |
| `flux` | out | flux from liquid to gas | kg/(m^2*s) |
| `liquid_resistance_fraction` | out | fraction of the resistance in the liquid film | — |

**Applies when:** linear equilibrium.

```shell
$ environmental-engineering solve enveng_two_film_transfer "kl=2e-05 m/s" "kg=0.005 m/s" H=0.2 "C=1 mg/L" "Cg=0 ug/m^3"
KL = 1.960784314e-05 m/s
flux = 1.960784314e-08 kg/(m^2*s)
liquid_resistance_fraction = 0.9803921569
  by: Overall liquid-phase transfer coefficient and flux (two-film theory)  (1/KL = 1/kl + 1/(H kg); flux = KL (C - Cg/H))
  kl = 2e-05 m/s
  kg = 0.005 m/s
  H = 0.2
  C = 0.001 kg/m^3
  Cg = 0 kg/m^3
  assumes: linear equilibrium
```

In the worked example, kl = 2e-05 m/s, kg = 0.005 m/s, H = 0.2, C = 1 mg/L, Cg = 0 ug/m^3 go through the equation above and give KL = 1.960784314e-05 m/s, flux = 1.960784314e-08 kg/(m^2*s), liquid_resistance_fraction = 0.9803921569. It applies only when linear equilibrium: outside that it says what is wrong instead of returning a number.

<a id="enveng_air_stripping_tower"></a>

#### Packed-tower air stripping: stripping factor, transfer units and height — `enveng_air_stripping_tower`

The stripping factor, number of transfer units, packed height and minimum air-to-water ratio of a countercurrent packed tower stripping a volatile compound with clean air.

$$
NTU = \frac{R}{R-1}\ln\!\left[\frac{(C_{in}/C_{out})(R-1) + 1}{R}\right],\quad Z = HTU\cdot NTU
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qw` | in | water flow | m^3/s |
| `Qa` | in | air flow | m^3/s |
| `H` | in | dimensionless Henry's constant (Cg/Cl) | — |
| `Cin` | in | influent concentration | kg/m^3 |
| `Cout` | in | required effluent concentration | kg/m^3 |
| `HTU` | in | height of a transfer unit | m |
| `R` | out | stripping factor H Qa/Qw | — |
| `NTU` | out | number of transfer units | — |
| `Z` | out | packed height HTU NTU | m |
| `air_to_water_min` | out | minimum air-to-water ratio (Cin - Cout)/(H Cin) | — |

**Applies when:** Qa/Qw above the minimum; clean inlet air; dilute, linear equilibrium.

```shell
$ environmental-engineering solve enveng_air_stripping_tower "Qw=0.05 m^3/s" "Qa=1.5 m^3/s" H=0.2 "Cin=200 ug/L" "Cout=5 ug/L" "HTU=1 m"
R = 6
NTU = 4.213854527
Z = 4.213854527 m
air_to_water_min = 4.875
  by: Packed-tower air stripping: stripping factor, transfer units and height  (R = H Qa/Qw; NTU = R/(R - 1) ln[((Cin/Cout)(R - 1) + 1)/R] (Cin/Cout - 1 when R = 1); Z = HTU NTU)
  Qw = 0.05 m^3/s
  Qa = 1.5 m^3/s
  H = 0.2
  Cin = 0.0002 kg/m^3
  Cout = 5e-06 kg/m^3
  HTU = 1 m
  assumes: Qa/Qw above the minimum; clean inlet air; dilute, linear equilibrium
```

In the worked example, Qw = 0.05 m^3/s, Qa = 1.5 m^3/s, H = 0.2, Cin = 200 ug/L, Cout = 5 ug/L, HTU = 1 m go through the equation above and give R = 6, NTU = 4.213854527, Z = 4.213854527 m, air_to_water_min = 4.875. It applies only when Qa/Qw above the minimum; clean inlet air; dilute, linear equilibrium: outside that it says what is wrong instead of returning a number.

<a id="enveng_henry_temperature_dependence"></a>

#### Temperature dependence of Henry's constant (van 't Hoff) — `enveng_henry_temperature_dependence`

The Henry's solubility constant at a temperature T from its value at a reference temperature and the van 't Hoff coefficient B = -d ln Hcp / d(1/T) (the form tabulated in solubility compilations).

$$
H^{cp}(T) = H^{cp}(T_{ref})\exp\!\left[B\left(\frac1T - \frac{1}{T_{ref}}\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Hcp` | in | Henry's solubility constant at Tref | mol/(m^3*Pa) |
| `B` | in | temperature coefficient d ln Hcp/d(1/T), in K | — |
| `T` | in | temperature of interest | K |
| `Tref` | in | reference temperature (usually 298.15 K) | K |
| `Hcp_T` | out | solubility constant at T | mol/(m^3*Pa) |
| `Hpc_T` | out | volatility constant 1/Hcp at T | Pa*m^3/mol |

**Applies when:** constant enthalpy of solution over the range.

```shell
$ environmental-engineering solve enveng_henry_temperature_dependence "Hcp=1.3e-05 mol/(m^3*Pa)" B=1500 "T=283.15 K" "Tref=298.15 K"
Hcp_T = 1.697039196e-05 mol/(m^3*Pa)
Hpc_T = 58926.15813 Pa*m^3/mol
  by: Temperature dependence of Henry's constant (van 't Hoff)  (Hcp(T) = Hcp(Tref) exp(B (1/T - 1/Tref)))
  Hcp = 1.3e-05 mol/(m^3*Pa)
  B = 1500
  T = 283.15 K
  Tref = 298.15 K
  assumes: constant enthalpy of solution over the range
```

In the worked example, Hcp = 1.3e-05 mol/(m^3*Pa), B = 1500, T = 283.15 K, Tref = 298.15 K go through the equation above and give Hcp_T = 1.697039196e-05 mol/(m^3*Pa), Hpc_T = 58926.15813 Pa*m^3/mol. It applies only when constant enthalpy of solution over the range: outside that it says what is wrong instead of returning a number.

### Noise (7)

<a id="enveng_decibel_addition"></a>

#### Adding sound levels in decibels — `enveng_decibel_addition`

The combined sound pressure level of several incoherent sources, added on an energy basis.

$$
L = 10\log_{10}\sum_i 10^{L_i/10}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | sound levels in dB | — |
| `L_total` | out | combined level, dB | — |

**Applies when:** incoherent sources.

```shell
$ environmental-engineering solve enveng_decibel_addition "L=[85, 85]"
L_total = 88.01029996
  by: Adding sound levels in decibels  (L = 10 log10(sum 10^(Li/10)))
  assumes: incoherent sources
```

In the worked example, L = [85, 85] go through the equation above and give L_total = 88.01029996. It applies only when incoherent sources: outside that it says what is wrong instead of returning a number.

<a id="enveng_day_night_level"></a>

#### Day-night (Ldn) and day-evening-night (Lden) sound levels — `enveng_day_night_level`

The day-night average level (10 dB night penalty, 22:00-07:00) or the day-evening-night level (5 dB evening 19:00-23:00, 10 dB night 23:00-07:00) from 24 hourly equivalent levels.

$$
L_{dn} = 10\log_{10}\frac{1}{24}\left(\sum_{day}10^{L_h/10} + \sum_{night}10^{(L_h+10)/10}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | 24 hourly Leq values in dB, the first for 00:00-01:00 | — |
| `metric` | in | ldn or lden | — |
| `L` | out | Ldn or Lden, dB | — |

**Applies when:** exactly 24 hourly levels.

```shell
$ environmental-engineering solve enveng_day_night_level "L=[50, 50, 50, 50, 50, 50, 50, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 60, 60, 60, 55, 55]" metric=ldn
L = 63.54363682
  by: Day-night (Ldn) and day-evening-night (Lden) sound levels  (L = 10 log10((1/24) sum 10^((L_h + penalty_h)/10)))
  metric = ldn
  assumes: exactly 24 hourly levels
```

In the worked example, L = [50, 50, 50, 50, 50, 50, 50, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 60, 60, 60, 55, 55], metric = ldn go through the equation above and give L = 63.54363682. It applies only when exactly 24 hourly levels: outside that it says what is wrong instead of returning a number.

<a id="enveng_equivalent_sound_level"></a>

#### Equivalent continuous sound level Leq — `enveng_equivalent_sound_level`

The equivalent continuous sound level of a period made of intervals at different levels (energy average weighted by duration).

$$
L_{eq} = 10\log_{10}\frac{\sum_i t_i10^{L_i/10}}{\sum_i t_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | levels of the intervals in dB | — |
| `t` | in | durations of the intervals (any one unit) | — |
| `Leq` | out | equivalent continuous level, dB | — |

**Applies when:** one duration per level.

```shell
$ environmental-engineering solve enveng_equivalent_sound_level "L=[70, 85, 60]" "t=[4, 1, 3]"
Leq = 76.52279911
  by: Equivalent continuous sound level Leq  (Leq = 10 log10(sum t_i 10^(L_i/10)/sum t_i))
  assumes: one duration per level
```

In the worked example, L = [70, 85, 60], t = [4, 1, 3] go through the equation above and give Leq = 76.52279911. It applies only when one duration per level: outside that it says what is wrong instead of returning a number.

<a id="enveng_occupational_noise_dose"></a>

#### Occupational noise dose and 8-hour TWA (OSHA, 5 dB exchange) — `enveng_occupational_noise_dose`

The daily noise dose (per cent of the permissible exposure) and the 8-hour time-weighted average from levels and exposure times, with the 90 dBA criterion, 5 dB exchange rate and 80 dBA threshold.

$$
D = 100\sum_i\frac{C_i}{T_i},\ T_i = \frac{8}{2^{(L_i-90)/5}},\ TWA = 16.61\log_{10}\frac{D}{100} + 90
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | A-weighted levels in dBA | — |
| `hours` | in | exposure time at each level in hours | — |
| `dose_percent` | out | noise dose, per cent | — |
| `TWA` | out | 8-hour time-weighted average, dBA | — |

**Applies when:** levels below 80 dBA excluded.

```shell
$ environmental-engineering solve enveng_occupational_noise_dose "L=[90, 95, 85]" "hours=[4, 2, 2]"
dose_percent = 112.5
TWA = 90.8496434
  by: Occupational noise dose and 8-hour TWA (OSHA, 5 dB exchange)  (T = 8/2^((L - 90)/5); D = 100 sum C/T; TWA = 16.61 log10(D/100) + 90)
  assumes: levels below 80 dBA excluded
```

In the worked example, L = [90, 95, 85], hours = [4, 2, 2] go through the equation above and give dose_percent = 112.5, TWA = 90.8496434. It applies only when levels below 80 dBA excluded: outside that it says what is wrong instead of returning a number.

<a id="enveng_noise_distance_attenuation"></a>

#### Sound level at another distance (point or line source) — `enveng_noise_distance_attenuation`

The sound level at a second distance from a point source (6 dB per doubling) or a line source such as a busy road (3 dB per doubling), free field.

$$
L_2 = L_1 - 20\log_{10}\frac{r_2}{r_1}\ \text{(point)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L1` | in | level at distance r1, dB | — |
| `r1` | in | first distance | m |
| `r2` | in | second distance | m |
| `source` | in | point or line source | — |
| `L2` | out | level at distance r2, dB | — |

**Applies when:** free field, no absorption.

```shell
$ environmental-engineering solve enveng_noise_distance_attenuation L1=90 "r1=10 m" "r2=80 m" source=point
L2 = 71.93820026
  by: Sound level at another distance (point or line source)  (L2 = L1 - 20 log10(r2/r1) (point), L1 - 10 log10(r2/r1) (line))
  L1 = 90
  r1 = 10 m
  r2 = 80 m
  source = point
  assumes: free field, no absorption
```

In the worked example, L1 = 90, r1 = 10 m, r2 = 80 m, source = point go through the equation above and give L2 = 71.93820026. It applies only when free field, no absorption: outside that it says what is wrong instead of returning a number.

<a id="enveng_sound_power_to_pressure"></a>

#### Sound pressure level from sound power level — `enveng_sound_power_to_pressure`

The sound pressure level at a distance from a source of given sound power level and directivity factor (1 free field, 2 on a hard floor, 4 at a wall-floor edge, 8 in a corner).

$$
L_p = L_W + 10\log_{10}\frac{Q}{4\pi r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lw` | in | sound power level, dB re 1 pW | — |
| `r` | in | distance from the source | m |
| `Qd` | in | directivity factor (1, 2, 4 or 8) | — |
| `Lp` | out | sound pressure level, dB | — |

**Applies when:** far field, no absorption; rho c taken as 400 rayl (Lp = LI).

```shell
$ environmental-engineering solve enveng_sound_power_to_pressure Lw=110 "r=20 m" Qd=2
Lp = 75.9976014
  by: Sound pressure level from sound power level  (Lp = Lw + 10 log10(Q/(4 pi r^2)))
  Lw = 110
  r = 20 m
  Qd = 2
  assumes: far field, no absorption; rho c taken as 400 rayl (Lp = LI)
```

In the worked example, Lw = 110, r = 20 m, Qd = 2 go through the equation above and give Lp = 75.9976014. It applies only when far field, no absorption; rho c taken as 400 rayl (Lp = LI): outside that it says what is wrong instead of returning a number.

<a id="enveng_background_noise_correction"></a>

#### Source level after removing background noise — `enveng_background_noise_correction`

The sound level of a source alone from the total measured level and the background level (energy subtraction).

$$
L_s = 10\log_{10}\left(10^{L_t/10} - 10^{L_b/10}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lt` | in | total measured level, dB | — |
| `Lb` | in | background level, dB | — |
| `L_source` | out | level of the source alone, dB | — |

**Applies when:** Lt > Lb.

```shell
$ environmental-engineering solve enveng_background_noise_correction Lt=85 Lb=80
L_source = 83.34911461
  by: Source level after removing background noise  (Ls = 10 log10(10^(Lt/10) - 10^(Lb/10)))
  Lt = 85
  Lb = 80
  assumes: Lt > Lb
```

In the worked example, Lt = 85, Lb = 80 go through the equation above and give L_source = 83.34911461. It applies only when Lt > Lb: outside that it says what is wrong instead of returning a number.

### Solid waste (5)

<a id="enveng_dulong_heating_value"></a>

#### Heating value of waste from its elemental composition (modified Dulong) — `enveng_dulong_heating_value`

The higher heating value of a waste or fuel from its dry mass fractions of carbon, hydrogen, oxygen, sulfur and nitrogen by the modified Dulong formula.

$$
HHV = 337C + 1419\left(H - \frac O8\right) + 93S + 23N\ \ [\mathrm{kJ/kg},\ \%]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | carbon mass fraction (fraction or %) | — |
| `H` | in | hydrogen mass fraction | — |
| `O` | in | oxygen mass fraction | — |
| `S` | in | sulfur mass fraction | — |
| `N` | in | nitrogen mass fraction | — |
| `HHV` | out | higher heating value | J/kg |
| `HHV_kJ_per_kg` | out | in kJ/kg | — |
| `HHV_MJ_per_kg` | out | in MJ/kg | — |

**Applies when:** fractions on the same (dry) basis, sum at most 1.

```shell
$ environmental-engineering solve enveng_dulong_heating_value C=0.48 H=0.064 O=0.376 S=0.004 N=0.008
HHV = 18643900 J/kg
HHV_kJ_per_kg = 18643.9
HHV_MJ_per_kg = 18.6439
  by: Heating value of waste from its elemental composition (modified Dulong)  (HHV (kJ/kg) = 337 C + 1419 (H - O/8) + 93 S + 23 N, mass per cent)
  C = 0.48
  H = 0.064
  O = 0.376
  S = 0.004
  N = 0.008
  assumes: fractions on the same (dry) basis, sum at most 1
```

In the worked example, C = 0.48, H = 0.064, O = 0.376, S = 0.004, N = 0.008 go through the equation above and give HHV = 18643900 J/kg, HHV_kJ_per_kg = 18643.9, HHV_MJ_per_kg = 18.6439. It applies only when fractions on the same (dry) basis, sum at most 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_landfill_life"></a>

#### Landfill life with daily cover and waste growth — `enveng_landfill_life`

The landfill volume used in the first year (waste at its in-place density plus cover soil) and the life of a landfill of given capacity when the waste grows at a constant yearly rate.

$$
n = \frac{\ln\left(1 + \frac{C g}{V_0}\right)}{\ln(1+g)},\quad V_0 = \frac{M(1 + c)}{\rho}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `capacity` | in | available landfill volume | m^3 |
| `M` | in | waste delivered in the first year | kg/s |
| `rho` | in | in-place (compacted) waste density | kg/m^3 |
| `cover` | in | cover soil volume per volume of waste (e.g. 0.25) | — |
| `g` | in | yearly growth of the waste quantity (fraction or %; 0 for none) | — |
| `V_first_year` | out | volume used in the first year, m^3 | — |
| `life_years` | out | landfill life, years | — |

**Applies when:** growth compounded yearly, life read continuously.

```shell
$ environmental-engineering solve enveng_landfill_life "capacity=2000000 m^3" "M=50000 t/yr" "rho=600 kg/m^3" cover=0.25 g=0
V_first_year = 104166.6667
life_years = 19.2
  by: Landfill life with daily cover and waste growth  (V0 = M (1 + cover)/rho per year; capacity = V0 ((1 + g)^n - 1)/g, so n = ln(1 + capacity g/V0)/ln(1 + g))
  capacity = 2000000 m^3
  M = 1.584404391 kg/s
  rho = 600 kg/m^3
  cover = 0.25
  g = 0
  assumes: growth compounded yearly, life read continuously
```

In the worked example, capacity = 2000000 m^3, M = 50000 t/yr, rho = 600 kg/m^3, cover = 0.25, g = 0 go through the equation above and give V_first_year = 104166.6667, life_years = 19.2. It applies only when growth compounded yearly, life read continuously: outside that it says what is wrong instead of returning a number.

<a id="enveng_landfill_gas_generation"></a>

#### Landfill methane generation (first-order decay, constant acceptance) — `enveng_landfill_gas_generation`

The methane generation rate and the cumulative methane of a landfill receiving waste at a constant rate for its operating life, each deposit decaying at first order (LandGEM-type model in continuous form), and the landfill gas flow at 50 % methane.

$$
Q(t) = \int_0^{\min(t,t_c)} kL_0M\,e^{-k(t-\tau)}d\tau = ML_0\left(e^{-k(t-\min(t,t_c))} - e^{-kt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | waste acceptance rate | kg/s |
| `L0` | in | methane generation potential | m^3/kg |
| `k` | in | methane generation rate constant | 1/s |
| `tc` | in | operating (filling) period | s |
| `t` | in | time since opening | s |
| `Q_CH4` | out | methane generation rate at t | m^3/s |
| `Q_CH4_m3_per_year` | out | in m^3 per year | — |
| `V_CH4` | out | cumulative methane generated up to t | m^3 |
| `Q_LFG` | out | landfill gas rate at 50 % methane | m^3/s |

**Applies when:** constant acceptance from opening to tc; first-order decay of every deposit.

```shell
$ environmental-engineering solve enveng_landfill_gas_generation "M=100000 t/yr" "L0=100 m^3/t" "k=0.05 1/yr" "tc=20 yr" "t=25 yr"
Q_CH4 = 0.1559991844 m^3/s
Q_CH4_m3_per_year = 4922959.862
V_CH4 = 101540802.8 m^3
Q_LFG = 0.3119983688 m^3/s
  by: Landfill methane generation (first-order decay, constant acceptance)  (Q(t) = M L0 (e^(-k (t - min(t, tc))) - e^(-k t)); cumulative volume by integrating Q)
  M = 3.168808781 kg/s
  L0 = 0.1 m^3/kg
  k = 1.584404391e-09 1/s
  tc = 631152000 s
  t = 788940000 s
  assumes: constant acceptance from opening to tc; first-order decay of every deposit
```

In the worked example, M = 100000 t/yr, L0 = 100 m^3/t, k = 0.05 1/yr, tc = 20 yr, t = 25 yr go through the equation above and give Q_CH4 = 0.1559991844 m^3/s, Q_CH4_m3_per_year = 4922959.862, V_CH4 = 101540802.8 m^3, Q_LFG = 0.3119983688 m^3/s. It applies only when constant acceptance from opening to tc; first-order decay of every deposit: outside that it says what is wrong instead of returning a number.

<a id="enveng_waste_moisture_content"></a>

#### Moisture content of mixed solid waste — `enveng_waste_moisture_content`

The moisture content (wet and dry basis) and the dry mass of a waste mixture from the masses of its components and their moisture fractions.

$$
M_{wet} = \frac{\sum m_iw_i}{\sum m_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | wet masses of the components (any one unit) | — |
| `w` | in | moisture fractions of the components, wet basis | — |
| `moisture_wet` | out | moisture content, wet basis | — |
| `dry_mass` | out | dry mass, in the unit of m | — |
| `moisture_dry` | out | moisture content, dry basis | — |

**Applies when:** moisture fractions below 1.

```shell
$ environmental-engineering solve enveng_waste_moisture_content "m=[40, 25, 15, 20]" "w=[0.7, 0.06, 0.02, 0.2]"
moisture_wet = 0.338
dry_mass = 66.2
moisture_dry = 0.5105740181
  by: Moisture content of mixed solid waste  (water = sum m_i w_i; M_wet = water/sum m; M_dry = water/(sum m - water))
  assumes: moisture fractions below 1
```

In the worked example, m = [40, 25, 15, 20], w = [0.7, 0.06, 0.02, 0.2] go through the equation above and give moisture_wet = 0.338, dry_mass = 66.2, moisture_dry = 0.5105740181. It applies only when moisture fractions below 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_solid_waste_generation"></a>

#### Municipal solid waste generation and collection trucks — `enveng_solid_waste_generation`

The daily and yearly mass of municipal solid waste of a population, its compacted volume and the number of truck loads per day.

$$
\dot m = P\,r,\quad n = \left\lceil\frac{\dot m}{\rho V_t}\right\rceil
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pop` | in | population served | — |
| `rate` | in | generation rate in kg per person per day | — |
| `rho` | in | compacted density in the truck | kg/m^3 |
| `Vt` | in | truck body volume | m^3 |
| `m` | out | waste generation | kg/s |
| `m_kg_per_day` | out | in kg/d | — |
| `m_t_per_year` | out | in t per 365-day year | — |
| `V_m3_per_day` | out | compacted volume per day, m^3 | — |
| `trucks_per_day` | out | truck loads per day (rounded up) | — |

**Applies when:** full truck loads at the stated density.

```shell
$ environmental-engineering solve enveng_solid_waste_generation pop=50000 rate=1.5 "rho=400 kg/m^3" "Vt=15 m^3"
m = 0.8680555556 kg/s
m_kg_per_day = 75000
m_t_per_year = 27375
V_m3_per_day = 187.5
trucks_per_day = 13
  by: Municipal solid waste generation and collection trucks  (m = P r; V = m/rho; trucks = ceil(V/Vt))
  rate = 1.5
  rho = 400 kg/m^3
  Vt = 15 m^3
  assumes: full truck loads at the stated density
```

In the worked example, pop = 50000, rate = 1.5, rho = 400 kg/m^3, Vt = 15 m^3 go through the equation above and give m = 0.8680555556 kg/s, m_kg_per_day = 75000, m_t_per_year = 27375, V_m3_per_day = 187.5. It applies only when full truck loads at the stated density: outside that it says what is wrong instead of returning a number.

### Wastewater (18)

<a id="enveng_activated_sludge_cstr_design"></a>

#### Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty) — `enveng_activated_sludge_cstr_design`

The effluent soluble substrate, the mixed-liquor biomass and the observed yield of a complete-mix activated-sludge reactor with recycle, from the kinetic coefficients, the SRT and HRT and the influent substrate.

$$
S = \frac{K_s(1 + k_d\theta_c)}{\theta_c(Yk - k_d) - 1},\quad X = \frac{\theta_c Y(S_0 - S)}{\theta(1 + k_d\theta_c)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Y` | in | true yield coefficient, kg VSS per kg BOD | — |
| `k` | in | maximum specific substrate utilisation rate | 1/s |
| `Ks` | in | half-saturation constant | kg/m^3 |
| `kd` | in | endogenous decay coefficient | 1/s |
| `SRT` | in | solids retention time | s |
| `HRT` | in | hydraulic retention time | s |
| `S0` | in | influent substrate (BOD) | kg/m^3 |
| `S` | out | effluent soluble substrate | kg/m^3 |
| `S_mgL` | out | in mg/L | — |
| `X` | out | biomass concentration in the reactor | kg/m^3 |
| `X_mgL` | out | in mg/L | — |
| `Y_obs` | out | observed yield Y/(1 + kd SRT) | — |

**Applies when:** steady state, complete mix; SRT above the washout limit; S < S0.

```shell
$ environmental-engineering solve enveng_activated_sludge_cstr_design Y=0.5 "k=5 1/day" "Ks=60 mg/L" "kd=0.06 1/day" "SRT=10 day" "HRT=6 h" "S0=250 mg/L"
S = 0.004102564103 kg/m^3
S_mgL = 4.102564103
X = 3.073717949 kg/m^3
X_mgL = 3073.717949
Y_obs = 0.3125
  by: Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)  (S = Ks (1 + kd SRT)/(SRT (Y k - kd) - 1); X = (SRT/HRT) Y (S0 - S)/(1 + kd SRT))
  Y = 0.5
  k = 5.787037037e-05 1/s
  Ks = 0.06 kg/m^3
  kd = 6.944444444e-07 1/s
  SRT = 864000 s
  HRT = 21600 s
  S0 = 0.25 kg/m^3
  assumes: steady state, complete mix; SRT above the washout limit; S < S0
```

In the worked example, Y = 0.5, k = 5 1/day, Ks = 60 mg/L, kd = 0.06 1/day, SRT = 10 day, HRT = 6 h, S0 = 250 mg/L go through the equation above and give S = 0.004102564103 kg/m^3, S_mgL = 4.102564103, X = 3.073717949 kg/m^3, X_mgL = 3073.717949. It applies only when steady state, complete mix; SRT above the washout limit; S < S0: outside that it says what is wrong instead of returning a number.

<a id="enveng_aeration_do_curve"></a>

#### Dissolved oxygen during aeration and the transfer rate — `enveng_aeration_do_curve`

The dissolved oxygen after an aeration time from the starting DO, the saturation DO and KLa, and the oxygen transfer rate KLa (Cs - C) at that moment.

$$
C = C_s - (C_s - C_0)e^{-K_La\,t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cs` | in | saturation DO | kg/m^3 |
| `C0` | in | starting DO | kg/m^3 |
| `KLa` | in | volumetric transfer coefficient | 1/s |
| `t` | in | aeration time | s |
| `C` | out | DO at t | kg/m^3 |
| `C_mgL` | out | in mg/L | — |
| `OTR` | out | oxygen transfer rate at t | kg/(m^3*s) |
| `OTR_mg_per_L_h` | out | transfer rate in mg/(L h) | — |

**Applies when:** C0 <= Cs; no oxygen uptake.

```shell
$ environmental-engineering solve enveng_aeration_do_curve "Cs=9.1 mg/L" "C0=1 mg/L" "KLa=10 1/h" "t=10 min"
C = 0.007570107617 kg/m^3
C_mgL = 7.570107617
OTR = 4.249701064e-06 kg/(m^3*s)
OTR_mg_per_L_h = 15.29892383
  by: Dissolved oxygen during aeration and the transfer rate  (dC/dt = KLa (Cs - C), C = Cs - (Cs - C0) e^(-KLa t))
  Cs = 0.0091 kg/m^3
  C0 = 0.001 kg/m^3
  KLa = 0.002777777778 1/s
  t = 600 s
  assumes: C0 <= Cs; no oxygen uptake
```

In the worked example, Cs = 9.1 mg/L, C0 = 1 mg/L, KLa = 10 1/h, t = 10 min go through the equation above and give C = 0.007570107617 kg/m^3, C_mgL = 7.570107617, OTR = 4.249701064e-06 kg/(m^3*s), OTR_mg_per_L_h = 15.29892383. It applies only when C0 <= Cs; no oxygen uptake: outside that it says what is wrong instead of returning a number.

<a id="enveng_field_oxygen_transfer"></a>

#### Field (actual) oxygen transfer rate of an aeration system — `enveng_field_oxygen_transfer`

The actual oxygen transfer rate in the process from the standard (clean water, 20 degC, zero DO) rate, corrected by alpha, beta, the temperature coefficient, the site saturation and the operating DO.

$$
AOTR = SOTR\,\alpha\,\theta^{T-20}\,\frac{\beta C_{s,T}F - C_L}{C_{s,20}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SOTR` | in | standard oxygen transfer rate | kg/s |
| `alpha` | in | alpha factor (process/clean water KLa) | — |
| `beta` | in | beta factor (process/clean water saturation) | — |
| `theta` | in | temperature coefficient (1.024) | — |
| `CsT` | in | clean-water saturation DO at the process temperature | kg/m^3 |
| `Cs20` | in | clean-water saturation DO at 20 degC | kg/m^3 |
| `CL` | in | operating DO | kg/m^3 |
| `T` | in | process temperature | K |
| `Fp` | in | pressure (elevation) correction of saturation (optional) | — |
| `AOTR` | out | actual oxygen transfer rate | kg/s |
| `AOTR_kg_per_h` | out | in kg O2/h | — |
| `ratio` | out | AOTR/SOTR | — |

**Applies when:** beta CsT Fp > CL.

```shell
$ environmental-engineering solve enveng_field_oxygen_transfer "SOTR=100 kg/h" alpha=0.6 beta=0.95 theta=1.024 "CsT=8.26 mg/L" "Cs20=9.09 mg/L" "CL=2 mg/L" "T=25 degC"
AOTR = 0.01207029108 kg/s
AOTR_kg_per_h = 43.45304789
ratio = 0.4345304789
  by: Field (actual) oxygen transfer rate of an aeration system  (AOTR = SOTR alpha theta^(T - 20) (beta CsT Fp - CL)/Cs20)
  SOTR = 0.02777777778 kg/s
  alpha = 0.6
  beta = 0.95
  theta = 1.024
  CsT = 0.00826 kg/m^3
  Cs20 = 0.00909 kg/m^3
  CL = 0.002 kg/m^3
  T = 298.15 K
  assumes: beta CsT Fp > CL
```

In the worked example, SOTR = 100 kg/h, alpha = 0.6, beta = 0.95, theta = 1.024, CsT = 8.26 mg/L, Cs20 = 9.09 mg/L, CL = 2 mg/L, T = 25 degC go through the equation above and give AOTR = 0.01207029108 kg/s, AOTR_kg_per_h = 43.45304789, ratio = 0.4345304789. It applies only when beta CsT Fp > CL: outside that it says what is wrong instead of returning a number.

<a id="enveng_food_to_microorganism_ratio"></a>

#### Food-to-microorganism ratio (F/M) — `enveng_food_to_microorganism_ratio`

The food-to-microorganism ratio of an aeration tank from the flow, influent BOD, tank volume and mixed-liquor volatile suspended solids.

$$
\frac FM = \frac{QS_0}{VX}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | influent flow | m^3/s |
| `S0` | in | influent BOD (or COD) | kg/m^3 |
| `V` | in | aeration tank volume | m^3 |
| `X` | in | mixed liquor volatile suspended solids (MLVSS) | kg/m^3 |
| `FM` | out | F/M ratio | 1/s |
| `FM_per_day` | out | F/M in kg BOD/(kg MLVSS d) | — |

**Applies when:** Q, S0, V, X > 0.

```shell
$ environmental-engineering solve enveng_food_to_microorganism_ratio "Q=10000 m^3/day" "S0=200 mg/L" "V=3000 m^3" "X=2500 mg/L"
FM = 3.086419753e-06 1/s
FM_per_day = 0.2666666667
  by: Food-to-microorganism ratio (F/M)  (F/M = Q S0/(V X))
  Q = 0.1157407407 m^3/s
  S0 = 0.2 kg/m^3
  V = 3000 m^3
  X = 2.5 kg/m^3
  assumes: Q, S0, V, X > 0
```

In the worked example, Q = 10000 m^3/day, S0 = 200 mg/L, V = 3000 m^3, X = 2500 mg/L go through the equation above and give FM = 3.086419753e-06 1/s, FM_per_day = 0.2666666667. It applies only when Q, S0, V, X > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_hrt_organic_loading"></a>

#### Hydraulic retention time and volumetric organic loading — `enveng_hrt_organic_loading`

The hydraulic retention time of a reactor and its volumetric organic (BOD) loading rate from the flow, influent BOD and volume.

$$
\theta = \frac VQ,\quad L_v = \frac{QS_0}{V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | influent flow | m^3/s |
| `S0` | in | influent BOD | kg/m^3 |
| `V` | in | reactor volume | m^3 |
| `HRT` | out | hydraulic retention time | s |
| `HRT_h` | out | HRT in hours | — |
| `OLR` | out | volumetric organic loading rate | kg/(m^3*s) |
| `OLR_kg_per_m3_day` | out | loading in kg BOD/(m^3 d) | — |

**Applies when:** Q, S0, V > 0.

```shell
$ environmental-engineering solve enveng_hrt_organic_loading "Q=10000 m^3/day" "S0=200 mg/L" "V=3000 m^3"
HRT = 25920 s
HRT_h = 7.2
OLR = 7.716049383e-06 kg/(m^3*s)
OLR_kg_per_m3_day = 0.6666666667
  by: Hydraulic retention time and volumetric organic loading  (HRT = V/Q; OLR = Q S0/V)
  Q = 0.1157407407 m^3/s
  S0 = 0.2 kg/m^3
  V = 3000 m^3
  assumes: Q, S0, V > 0
```

In the worked example, Q = 10000 m^3/day, S0 = 200 mg/L, V = 3000 m^3 go through the equation above and give HRT = 25920 s, HRT_h = 7.2, OLR = 7.716049383e-06 kg/(m^3*s), OLR_kg_per_m3_day = 0.6666666667. It applies only when Q, S0, V > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_kla_regression"></a>

#### KLa fitted to a reaeration test — `enveng_kla_regression`

The volumetric oxygen-transfer coefficient KLa from a series of dissolved-oxygen readings during a reaeration test, by linear regression of ln(Cs - C) against time.

$$
\ln(C_s - C) = \ln(C_s - C_0) - K_La\,t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | times in minutes | — |
| `C` | in | DO readings in mg/L (same order) | — |
| `Cs` | in | saturation DO | kg/m^3 |
| `KLa_per_min` | out | KLa in 1/min | — |
| `KLa` | out | KLa | 1/s |
| `KLa_per_h` | out | KLa in 1/h | — |
| `r2` | out | coefficient of determination | — |

**Applies when:** every reading below Cs; at least three distinct times.

```shell
$ environmental-engineering solve enveng_kla_regression "t=[2, 4, 6, 8, 10, 12]" "C=[2.5, 4.1, 5.4, 6.3, 7, 7.6]" "Cs=9.1 mg/L"
KLa_per_min = 0.1469891116
KLa = 0.002449818526 1/s
KLa_per_h = 8.819346694
r2 = 0.9991806961
  by: KLa fitted to a reaeration test  (least squares of ln(Cs - C) = ln(Cs - C0) - KLa t)
  Cs = 0.0091 kg/m^3
  assumes: every reading below Cs; at least three distinct times
```

In the worked example, t = [2, 4, 6, 8, 10, 12], C = [2.5, 4.1, 5.4, 6.3, 7, 7.6], Cs = 9.1 mg/L go through the equation above and give KLa_per_min = 0.1469891116, KLa = 0.002449818526 1/s, KLa_per_h = 8.819346694, r2 = 0.9991806961. It applies only when every reading below Cs; at least three distinct times: outside that it says what is wrong instead of returning a number.

<a id="enveng_anaerobic_methane_yield"></a>

#### Methane production from COD removed (anaerobic digestion) — `enveng_anaerobic_methane_yield`

The methane gas flow from an anaerobic process from the COD removed less the COD converted to biomass (1.42 kg COD per kg VSS), at a stated temperature and pressure (64 g COD per mole of methane).

$$
Q_{CH_4} = \frac{\mathrm{COD} - 1.42P_x}{64\ \mathrm{g/mol}}\,\frac{RT}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `COD` | in | COD removed | kg/s |
| `Px` | in | biomass (VSS) produced | kg/s |
| `T` | in | gas temperature | K |
| `P` | in | gas pressure | Pa |
| `Q_CH4` | out | methane flow at T and P | m^3/s |
| `Q_CH4_m3_per_day` | out | in m^3/d at T and P | — |
| `Q_CH4_stp_m3_per_day` | out | in m^3/d at 0 degC and 1 atm | — |

**Applies when:** ideal gas; COD balance.

```shell
$ environmental-engineering solve enveng_anaerobic_methane_yield "COD=5000 kg/day" "Px=300 kg/day" "T=308.15 K" "P=101.325 kPa"
Q_CH4 = 0.02091695562 m^3/s
Q_CH4_m3_per_day = 1807.224965
Q_CH4_stp_m3_per_day = 1601.958459
  by: Methane production from COD removed (anaerobic digestion)  (n = (COD - 1.42 Px)/(64 g/mol); Q = n R T/P)
  COD = 0.05787037037 kg/s
  Px = 0.003472222222 kg/s
  T = 308.15 K
  P = 101325 Pa
  assumes: ideal gas; COD balance
```

In the worked example, COD = 5000 kg/day, Px = 300 kg/day, T = 308.15 K, P = 101.325 kPa go through the equation above and give Q_CH4 = 0.02091695562 m^3/s, Q_CH4_m3_per_day = 1807.224965, Q_CH4_stp_m3_per_day = 1601.958459. It applies only when ideal gas; COD balance: outside that it says what is wrong instead of returning a number.

<a id="enveng_minimum_srt"></a>

#### Minimum (washout) solids retention time — `enveng_minimum_srt`

The washout SRT of a complete-mix biological reactor: below it the organisms are lost faster than they grow on the influent substrate.

$$
\frac{1}{\theta_{c,min}} = \frac{YkS_0}{K_s + S_0} - k_d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Y` | in | true yield coefficient, kg VSS per kg BOD | — |
| `k` | in | maximum specific substrate utilisation rate | 1/s |
| `Ks` | in | half-saturation constant | kg/m^3 |
| `kd` | in | endogenous decay coefficient | 1/s |
| `S0` | in | influent substrate | kg/m^3 |
| `SRT_min` | out | minimum SRT | s |
| `SRT_min_days` | out | minimum SRT in days | — |

**Applies when:** Y k S0/(Ks + S0) > kd.

```shell
$ environmental-engineering solve enveng_minimum_srt Y=0.5 "k=5 1/day" "Ks=60 mg/L" "kd=0.06 1/day" "S0=250 mg/L"
SRT_min = 44168.86544 s
SRT_min_days = 0.5112137203
  by: Minimum (washout) solids retention time  (1/SRT_min = Y k S0/(Ks + S0) - kd)
  Y = 0.5
  k = 5.787037037e-05 1/s
  Ks = 0.06 kg/m^3
  kd = 6.944444444e-07 1/s
  S0 = 0.25 kg/m^3
  assumes: Y k S0/(Ks + S0) > kd
```

In the worked example, Y = 0.5, k = 5 1/day, Ks = 60 mg/L, kd = 0.06 1/day, S0 = 250 mg/L go through the equation above and give SRT_min = 44168.86544 s, SRT_min_days = 0.5112137203. It applies only when Y k S0/(Ks + S0) > kd: outside that it says what is wrong instead of returning a number.

<a id="enveng_monod_growth_rate"></a>

#### Monod specific growth rate — `enveng_monod_growth_rate`

The specific growth rate of microorganisms at a substrate concentration by the Monod equation, and the net rate when the endogenous decay coefficient is given.

$$
\mu = \frac{\mu_{max}S}{K_s + S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu_max` | in | maximum specific growth rate | 1/s |
| `Ks` | in | half-saturation constant | kg/m^3 |
| `S` | in | substrate concentration | kg/m^3 |
| `kd` | in | endogenous decay coefficient (optional) | 1/s |
| `mu` | out | specific growth rate | 1/s |
| `mu_per_day` | out | in 1/d | — |
| `mu_net` | out | net specific growth rate mu - kd | 1/s |
| `mu_net_per_day` | out | net rate in 1/d | — |

**Applies when:** Monod kinetics.

```shell
$ environmental-engineering solve enveng_monod_growth_rate "mu_max=3 1/day" "Ks=60 mg/L" "S=10 mg/L"
mu = 4.96031746e-06 1/s
mu_per_day = 0.4285714286
  by: Monod specific growth rate  (mu = mu_max S/(Ks + S); mu_net = mu - kd)
  mu_max = 3.472222222e-05 1/s
  Ks = 0.06 kg/m^3
  S = 0.01 kg/m^3
  assumes: Monod kinetics
```

In the worked example, mu_max = 3 1/day, Ks = 60 mg/L, S = 10 mg/L go through the equation above and give mu = 4.96031746e-06 1/s, mu_per_day = 0.4285714286. It applies only when Monod kinetics: outside that it says what is wrong instead of returning a number.

<a id="enveng_kla_two_point"></a>

#### Overall oxygen transfer coefficient KLa from two DO readings — `enveng_kla_two_point`

The volumetric mass-transfer coefficient KLa from two dissolved-oxygen readings during reaeration toward a known saturation concentration.

$$
K_La = \frac{\ln\frac{C_s - C_1}{C_s - C_2}}{t_2 - t_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cs` | in | saturation DO | kg/m^3 |
| `C1` | in | DO at the first time | kg/m^3 |
| `C2` | in | DO at the second time | kg/m^3 |
| `t1` | in | first time | s |
| `t2` | in | second time | s |
| `KLa` | out | KLa | 1/s |
| `KLa_per_h` | out | KLa in 1/h | — |

**Applies when:** C1 < C2 < Cs; t2 > t1.

```shell
$ environmental-engineering solve enveng_kla_two_point "Cs=9.1 mg/L" "C1=2 mg/L" "C2=6 mg/L" "t1=2 min" "t2=10 min"
KLa = 0.001726443068 1/s
KLa_per_h = 6.215195044
  by: Overall oxygen transfer coefficient KLa from two DO readings  (KLa = ln((Cs - C1)/(Cs - C2))/(t2 - t1))
  Cs = 0.0091 kg/m^3
  C1 = 0.002 kg/m^3
  C2 = 0.006 kg/m^3
  t1 = 120 s
  t2 = 600 s
  assumes: C1 < C2 < Cs; t2 > t1
```

In the worked example, Cs = 9.1 mg/L, C1 = 2 mg/L, C2 = 6 mg/L, t1 = 2 min, t2 = 10 min go through the equation above and give KLa = 0.001726443068 1/s, KLa_per_h = 6.215195044. It applies only when C1 < C2 < Cs; t2 > t1: outside that it says what is wrong instead of returning a number.

<a id="enveng_aeration_oxygen_requirement"></a>

#### Oxygen requirement of an activated-sludge process — `enveng_aeration_oxygen_requirement`

The carbonaceous oxygen requirement: ultimate BOD removed (BOD5 removed divided by the BOD5/BODu ratio) less 1.42 times the biomass wasted.

$$
\dot m_{O_2} = \frac{Q(S_0 - S)}{f} - 1.42P_x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow | m^3/s |
| `S0` | in | influent BOD5 | kg/m^3 |
| `S` | in | effluent soluble BOD5 | kg/m^3 |
| `f` | in | BOD5/BODu ratio (e.g. 0.68) | — |
| `Px` | in | biomass wasted (VSS) | kg/s |
| `O2` | out | oxygen required | kg/s |
| `O2_kg_per_day` | out | in kg O2/d | — |

**Applies when:** carbonaceous demand only (no nitrification).

```shell
$ environmental-engineering solve enveng_aeration_oxygen_requirement "Q=10000 m^3/day" "S0=250 mg/L" "S=5 mg/L" f=0.68 "Px=800 kg/day"
O2 = 0.02855255991 kg/s
O2_kg_per_day = 2466.941176
  by: Oxygen requirement of an activated-sludge process  (O2 = Q (S0 - S)/f - 1.42 Px)
  Q = 0.1157407407 m^3/s
  S0 = 0.25 kg/m^3
  S = 0.005 kg/m^3
  f = 0.68
  Px = 0.009259259259 kg/s
  assumes: carbonaceous demand only (no nitrification)
```

In the worked example, Q = 10000 m^3/day, S0 = 250 mg/L, S = 5 mg/L, f = 0.68, Px = 800 kg/day go through the equation above and give O2 = 0.02855255991 kg/s, O2_kg_per_day = 2466.941176. It applies only when carbonaceous demand only (no nitrification): outside that it says what is wrong instead of returning a number.

<a id="enveng_return_sludge_ratio"></a>

#### Return activated sludge ratio — `enveng_return_sludge_ratio`

The return sludge ratio and flow needed to hold the mixed-liquor concentration, from a solids balance around the aeration tank (influent solids neglected).

$$
R = \frac{Q_r}{Q} = \frac{X}{X_r - X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X` | in | mixed liquor suspended solids | kg/m^3 |
| `Xr` | in | return sludge concentration | kg/m^3 |
| `Q` | in | influent flow | m^3/s |
| `R` | out | return ratio Qr/Q | — |
| `Qr` | out | return sludge flow | m^3/s |

**Applies when:** Xr > X; influent solids negligible.

```shell
$ environmental-engineering solve enveng_return_sludge_ratio "X=3000 mg/L" "Xr=10000 mg/L" "Q=10000 m^3/day"
R = 0.4285714286
Qr = 0.0496031746 m^3/s
  by: Return activated sludge ratio  ((Q + Qr) X = Qr Xr, so R = X/(Xr - X))
  X = 3 kg/m^3
  Xr = 10 kg/m^3
  Q = 0.1157407407 m^3/s
  assumes: Xr > X; influent solids negligible
```

In the worked example, X = 3000 mg/L, Xr = 10000 mg/L, Q = 10000 m^3/day go through the equation above and give R = 0.4285714286, Qr = 0.0496031746 m^3/s. It applies only when Xr > X; influent solids negligible: outside that it says what is wrong instead of returning a number.

<a id="enveng_sludge_thickening"></a>

#### Sludge volume after thickening or dewatering — `enveng_sludge_thickening`

The sludge volume after thickening or dewatering from one solids fraction to another (same specific gravity), and the fractional volume reduction.

$$
V_2 = V_1\frac{P_1}{P_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V1` | in | sludge volume rate before | m^3/s |
| `P1` | in | solids fraction before (fraction or %) | — |
| `P2` | in | solids fraction after (fraction or %) | — |
| `V2` | out | sludge volume rate after | m^3/s |
| `reduction` | out | fractional volume reduction | — |

**Applies when:** P2 > P1; same specific gravity.

```shell
$ environmental-engineering solve enveng_sludge_thickening "V1=100 m^3/day" P1=0.01 P2=0.05
V2 = 0.0002314814815 m^3/s
reduction = 0.8
  by: Sludge volume after thickening or dewatering  (V2 = V1 P1/P2)
  V1 = 0.001157407407 m^3/s
  P1 = 0.01
  P2 = 0.05
  assumes: P2 > P1; same specific gravity
```

In the worked example, V1 = 100 m^3/day, P1 = 0.01, P2 = 0.05 go through the equation above and give V2 = 0.0002314814815 m^3/s, reduction = 0.8. It applies only when P2 > P1; same specific gravity: outside that it says what is wrong instead of returning a number.

<a id="enveng_sludge_volume_index"></a>

#### Sludge volume index (SVI) — `enveng_sludge_volume_index`

The sludge volume index from the 30-minute settled volume and the mixed-liquor suspended solids, with the customary estimate of the return sludge concentration (10^6/SVI mg/L).

$$
SVI = \frac{V_{30}}{MLSS}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V30` | in | settled sludge volume after 30 minutes, mL/L | — |
| `MLSS` | in | mixed liquor suspended solids | kg/m^3 |
| `SVI` | out | sludge volume index, mL/g | — |
| `Xr_estimate` | out | estimated maximum return sludge concentration | kg/m^3 |
| `Xr_estimate_mgL` | out | estimate in mg/L | — |

**Applies when:** V30 <= 1000 mL/L.

```shell
$ environmental-engineering solve enveng_sludge_volume_index V30=300 "MLSS=2500 mg/L"
SVI = 120
Xr_estimate = 8.333333333 kg/m^3
Xr_estimate_mgL = 8333.333333
  by: Sludge volume index (SVI)  (SVI = V30 (mL/L)/MLSS (g/L))
  V30 = 300
  MLSS = 2.5 kg/m^3
  assumes: V30 <= 1000 mL/L
```

In the worked example, V30 = 300, MLSS = 2500 mg/L go through the equation above and give SVI = 120, Xr_estimate = 8.333333333 kg/m^3, Xr_estimate_mgL = 8333.333333. It applies only when V30 <= 1000 mL/L: outside that it says what is wrong instead of returning a number.

<a id="enveng_solids_retention_time"></a>

#### Solids retention time (mean cell residence time) — `enveng_solids_retention_time`

The solids retention time (sludge age) of an activated-sludge system from the biomass in the tank and the biomass leaving in the waste sludge and the effluent.

$$
\theta_c = \frac{VX}{Q_wX_w + Q_eX_e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | aeration tank volume | m^3 |
| `X` | in | MLVSS in the tank | kg/m^3 |
| `Qw` | in | waste sludge flow | m^3/s |
| `Xw` | in | VSS in the waste sludge | kg/m^3 |
| `Qe` | in | effluent flow | m^3/s |
| `Xe` | in | VSS in the effluent | kg/m^3 |
| `SRT` | out | solids retention time | s |
| `SRT_days` | out | SRT in days | — |

**Applies when:** steady state.

```shell
$ environmental-engineering solve enveng_solids_retention_time "V=3000 m^3" "X=2500 mg/L" "Qw=100 m^3/day" "Xw=10000 mg/L" "Qe=9900 m^3/day" "Xe=15 mg/L"
SRT = 564214.1924 s
SRT_days = 6.530256857
  by: Solids retention time (mean cell residence time)  (SRT = V X/(Qw Xw + Qe Xe))
  V = 3000 m^3
  X = 2.5 kg/m^3
  Qw = 0.001157407407 m^3/s
  Xw = 10 kg/m^3
  Qe = 0.1145833333 m^3/s
  Xe = 0.015 kg/m^3
  assumes: steady state
```

In the worked example, V = 3000 m^3, X = 2500 mg/L, Qw = 100 m^3/day, Xw = 10000 mg/L, Qe = 9900 m^3/day, Xe = 15 mg/L go through the equation above and give SRT = 564214.1924 s, SRT_days = 6.530256857. It applies only when steady state: outside that it says what is wrong instead of returning a number.

<a id="enveng_nrc_trickling_filter"></a>

#### Trickling filter BOD removal efficiency (NRC equation) — `enveng_nrc_trickling_filter`

The BOD removal efficiency of a single-stage stone-media trickling filter (with its settling tank) by the NRC equation, from the BOD load, the filter volume and the recirculation ratio.

$$
E = \frac{100}{1 + 0.4432\sqrt{W/(VF)}},\quad F = \frac{1+R}{(1 + R/10)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | BOD load to the filter (excluding recycle) | kg/s |
| `V` | in | filter media volume | m^3 |
| `R` | in | recirculation ratio Qr/Q | — |
| `F` | out | recirculation factor (1 + R)/(1 + R/10)^2 | — |
| `E` | out | BOD removal efficiency (fraction) | — |
| `E_percent` | out | efficiency in per cent | — |

**Applies when:** first-stage stone filter at 20 degC; W in kg/d and V in m^3 inside the formula constant.

```shell
$ environmental-engineering solve enveng_nrc_trickling_filter "W=800 kg/day" "V=1000 m^3" R=2
F = 2.083333333
E = 0.7845346289
E_percent = 78.45346289
  by: Trickling filter BOD removal efficiency (NRC equation)  (F = (1 + R)/(1 + R/10)^2; E = 1/(1 + 0.4432 sqrt(W/(V F))))
  W = 0.009259259259 kg/s
  V = 1000 m^3
  R = 2
  assumes: first-stage stone filter at 20 degC; W in kg/d and V in m^3 inside the formula constant
```

In the worked example, W = 800 kg/day, V = 1000 m^3, R = 2 go through the equation above and give F = 2.083333333, E = 0.7845346289, E_percent = 78.45346289. It applies only when first-stage stone filter at 20 degC; W in kg/d and V in m^3 inside the formula constant: outside that it says what is wrong instead of returning a number.

<a id="enveng_sludge_volume"></a>

#### Volume of wet sludge from dry solids — `enveng_sludge_volume`

The volume (flow) of wet sludge from the dry-solids mass rate, the solids fraction and the specific gravity of the sludge (water 1000 kg/m^3).

$$
V = \frac{M_s}{\rho_w S_{sl} P_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ms` | in | dry solids mass rate | kg/s |
| `Ps` | in | solids fraction of the wet sludge (fraction or %) | — |
| `Ssl` | in | specific gravity of the wet sludge | — |
| `V` | out | wet sludge volume rate | m^3/s |
| `V_m3_per_day` | out | in m^3/d | — |

**Applies when:** 0 < Ps <= 1.

```shell
$ environmental-engineering solve enveng_sludge_volume "Ms=1000 kg/day" Ps=0.04 Ssl=1.02
V = 0.0002836782861 m^3/s
V_m3_per_day = 24.50980392
  by: Volume of wet sludge from dry solids  (V = Ms/(rho_w Ssl Ps))
  Ms = 0.01157407407 kg/s
  Ps = 0.04
  Ssl = 1.02
  assumes: 0 < Ps <= 1
```

In the worked example, Ms = 1000 kg/day, Ps = 0.04, Ssl = 1.02 go through the equation above and give V = 0.0002836782861 m^3/s, V_m3_per_day = 24.50980392. It applies only when 0 < Ps <= 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_sludge_production"></a>

#### Waste biomass production (observed yield) — `enveng_sludge_production`

The observed yield and the daily mass of biological solids produced (to be wasted) from the kinetic yield, decay coefficient, SRT, flow and substrate removed.

$$
P_x = \frac{Y}{1 + k_d\theta_c}Q(S_0 - S)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Y` | in | true yield, kg VSS per kg BOD | — |
| `kd` | in | endogenous decay coefficient | 1/s |
| `SRT` | in | solids retention time | s |
| `Q` | in | flow | m^3/s |
| `S0` | in | influent substrate | kg/m^3 |
| `S` | in | effluent substrate | kg/m^3 |
| `Y_obs` | out | observed yield | — |
| `Px` | out | biomass production | kg/s |
| `Px_kg_per_day` | out | in kg VSS/d | — |

**Applies when:** S < S0.

```shell
$ environmental-engineering solve enveng_sludge_production Y=0.5 "kd=0.06 1/day" "SRT=10 day" "Q=10000 m^3/day" "S0=250 mg/L" "S=5 mg/L"
Y_obs = 0.3125
Px = 0.008861400463 kg/s
Px_kg_per_day = 765.625
  by: Waste biomass production (observed yield)  (Y_obs = Y/(1 + kd SRT); Px = Y_obs Q (S0 - S))
  Y = 0.5
  kd = 6.944444444e-07 1/s
  SRT = 864000 s
  Q = 0.1157407407 m^3/s
  S0 = 0.25 kg/m^3
  S = 0.005 kg/m^3
  assumes: S < S0
```

In the worked example, Y = 0.5, kd = 0.06 1/day, SRT = 10 day, Q = 10000 m^3/day, S0 = 250 mg/L, S = 5 mg/L go through the equation above and give Y_obs = 0.3125, Px = 0.008861400463 kg/s, Px_kg_per_day = 765.625. It applies only when S < S0: outside that it says what is wrong instead of returning a number.

### Water quality (17)

<a id="enveng_bod_exerted"></a>

#### BOD exerted after a time (first-order) — `enveng_bod_exerted`

The BOD exerted after time t and the oxygen demand remaining, from the ultimate BOD and the first-order rate constant.

$$
BOD_t = L_0\left(1 - e^{-kt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L0` | in | ultimate (carbonaceous) BOD | kg/m^3 |
| `k` | in | BOD rate constant (base e) | 1/s |
| `t` | in | incubation or travel time | s |
| `BOD_t` | out | BOD exerted after t | kg/m^3 |
| `L_t` | out | BOD remaining after t | kg/m^3 |
| `BOD_t_mgL` | out | BOD exerted in mg/L | — |

**Applies when:** first-order kinetics.

```shell
$ environmental-engineering solve enveng_bod_exerted "L0=250 mg/L" "k=0.23 1/day" "t=5 day"
BOD_t = 0.1708408077 kg/m^3
L_t = 0.07915919234 kg/m^3
BOD_t_mgL = 170.8408077
  by: BOD exerted after a time (first-order)  (BOD_t = L0 (1 - e^(-k t)), L_t = L0 e^(-k t))
  L0 = 0.25 kg/m^3
  k = 2.662037037e-06 1/s
  t = 432000 s
  assumes: first-order kinetics
```

In the worked example, L0 = 250 mg/L, k = 0.23 1/day, t = 5 day go through the equation above and give BOD_t = 0.1708408077 kg/m^3, L_t = 0.07915919234 kg/m^3, BOD_t_mgL = 170.8408077. It applies only when first-order kinetics: outside that it says what is wrong instead of returning a number.

<a id="enveng_bod_dilution_test"></a>

#### BOD from a dilution test (with optional seed correction) — `enveng_bod_dilution_test`

The BOD of a sample from the dissolved oxygen of the diluted sample before and after incubation and the sample fraction, with the seed correction when the dilution water is seeded.

$$
BOD = \frac{(D_1 - D_2) - (B_1 - B_2)f}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D1` | in | DO of the diluted sample immediately after preparation | kg/m^3 |
| `D2` | in | DO of the diluted sample after incubation | kg/m^3 |
| `P` | in | decimal fraction of sample in the bottle (sample volume / bottle volume) | — |
| `B1` | in | DO of the seed control before incubation (optional) | kg/m^3 |
| `B2` | in | DO of the seed control after incubation (optional) | kg/m^3 |
| `f` | in | ratio of seed in the sample bottle to seed in the control (optional) | — |
| `BOD` | out | BOD of the sample | kg/m^3 |
| `BOD_mgL` | out | BOD in mg/L | — |

**Applies when:** D1 > D2; B1, B2 and f given together or not at all.

```shell
$ environmental-engineering solve enveng_bod_dilution_test "D1=8.5 mg/L" "D2=4 mg/L" P=0.05
BOD = 0.09 kg/m^3
BOD_mgL = 90
  by: BOD from a dilution test (with optional seed correction)  (BOD = ((D1 - D2) - (B1 - B2) f)/P (unseeded: BOD = (D1 - D2)/P))
  D1 = 0.0085 kg/m^3
  D2 = 0.004 kg/m^3
  P = 0.05
  assumes: D1 > D2; B1, B2 and f given together or not at all
```

In the worked example, D1 = 8.5 mg/L, D2 = 4 mg/L, P = 0.05 go through the equation above and give BOD = 0.09 kg/m^3, BOD_mgL = 90. It applies only when D1 > D2; B1, B2 and f given together or not at all: outside that it says what is wrong instead of returning a number.

<a id="enveng_bod_rate_fit"></a>

#### BOD rate constant and ultimate BOD fitted to BOD data — `enveng_bod_rate_fit`

The first-order rate constant k and ultimate BOD L0 that best fit a series of BOD measurements (nonlinear least squares of BOD = L0 (1 - e^(-k t))).

$$
\min_{k,L_0}\sum_i\left[y_i - L_0\left(1 - e^{-kt_i}\right)\right]^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | incubation times in days | — |
| `y` | in | measured BOD values in mg/L (same order) | — |
| `k` | out | fitted rate constant | 1/s |
| `k_per_day` | out | fitted rate constant in 1/d | — |
| `L0` | out | fitted ultimate BOD | kg/m^3 |
| `L0_mgL` | out | fitted ultimate BOD in mg/L | — |
| `rss` | out | residual sum of squares in (mg/L)^2 | — |

**Applies when:** at least three distinct positive times; a single rate constant between 0.001 and 20 per day fits.

```shell
$ environmental-engineering solve enveng_bod_rate_fit "t=[1, 2, 3, 5, 7, 10]" "y=[41, 74, 98, 132, 152, 168]"
k = 3.002440202e-06 1/s
k_per_day = 0.2594108335
L0 = 0.1815717527 kg/m^3
L0_mgL = 181.5717527
rss = 0.5327137697
  by: BOD rate constant and ultimate BOD fitted to BOD data  (for each k the best L0 is sum(y f)/sum(f^2) with f = 1 - e^(-k t); k is the root of the stationarity condition sum((y - L0 f) t e^(-k t)) = 0 (scan and bisection))
  assumes: at least three distinct positive times; a single rate constant between 0.001 and 20 per day fits
```

In the worked example, t = [1, 2, 3, 5, 7, 10], y = [41, 74, 98, 132, 152, 168] go through the equation above and give k = 3.002440202e-06 1/s, k_per_day = 0.2594108335, L0 = 0.1815717527 kg/m^3, L0_mgL = 181.5717527. It applies only when at least three distinct positive times; a single rate constant between 0.001 and 20 per day fits: outside that it says what is wrong instead of returning a number.

<a id="enveng_mixing_two_streams"></a>

#### Concentration after mixing a discharge with a river — `enveng_mixing_two_streams`

The fully mixed concentration (BOD, DO, a pollutant) and flow just below a wastewater outfall from the flows and concentrations of the discharge and the river.

$$
C = \frac{Q_w C_w + Q_r C_r}{Q_w + Q_r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qw` | in | wastewater (discharge) flow | m^3/s |
| `Cw` | in | concentration in the wastewater | kg/m^3 |
| `Qr` | in | river flow upstream | m^3/s |
| `Cr` | in | concentration in the river upstream | kg/m^3 |
| `C` | out | mixed concentration | kg/m^3 |
| `C_mgL` | out | in mg/L | — |
| `Q` | out | combined flow | m^3/s |

**Applies when:** complete mixing; conservative over the mixing zone.

```shell
$ environmental-engineering solve enveng_mixing_two_streams "Qw=0.5 m^3/s" "Cw=200 mg/L" "Qr=5 m^3/s" "Cr=2 mg/L"
C = 0.02 kg/m^3
C_mgL = 20
Q = 5.5 m^3/s
  by: Concentration after mixing a discharge with a river  (C = (Qw Cw + Qr Cr)/(Qw + Qr))
  Qw = 0.5 m^3/s
  Cw = 0.2 kg/m^3
  Qr = 5 m^3/s
  Cr = 0.002 kg/m^3
  assumes: complete mixing; conservative over the mixing zone
```

In the worked example, Qw = 0.5 m^3/s, Cw = 200 mg/L, Qr = 5 m^3/s, Cr = 2 mg/L go through the equation above and give C = 0.02 kg/m^3, C_mgL = 20, Q = 5.5 m^3/s. It applies only when complete mixing; conservative over the mixing zone: outside that it says what is wrong instead of returning a number.

<a id="enveng_lake_transient"></a>

#### Concentration in a well-mixed lake over time — `enveng_lake_transient`

The concentration in a completely mixed lake a time t after a change in load, from the initial concentration, the load, outflow, volume and first-order decay.

$$
C(t) = C_{ss} + (C_0 - C_{ss})e^{-(Q/V + k)t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | pollutant load after the change | kg/s |
| `Q` | in | outflow rate | m^3/s |
| `k` | in | first-order decay constant | 1/s |
| `V` | in | lake volume | m^3 |
| `C0` | in | concentration at t = 0 | kg/m^3 |
| `t` | in | elapsed time | s |
| `C` | out | concentration at t | kg/m^3 |
| `C_mgL` | out | in mg/L | — |
| `C_ss` | out | new steady state | kg/m^3 |

**Applies when:** completely mixed; constant flow, volume and load.

```shell
$ environmental-engineering solve enveng_lake_transient "W=100 kg/day" "Q=10 m^3/s" "k=0.2 1/day" "V=10000000 m^3" "C0=0 mg/L" "t=5 day"
C = 2.657713748e-05 kg/m^3
C_mgL = 0.02657713748
C_ss = 3.491620112e-05 kg/m^3
  by: Concentration in a well-mixed lake over time  (V dC/dt = W - Q C - k C V, so C = Css + (C0 - Css) e^(-(Q/V + k) t))
  W = 0.001157407407 kg/s
  Q = 10 m^3/s
  k = 2.314814815e-06 1/s
  V = 10000000 m^3
  C0 = 0 kg/m^3
  t = 432000 s
  assumes: completely mixed; constant flow, volume and load
```

In the worked example, W = 100 kg/day, Q = 10 m^3/s, k = 0.2 1/day, V = 10000000 m^3, C0 = 0 mg/L, t = 5 day go through the equation above and give C = 2.657713748e-05 kg/m^3, C_mgL = 0.02657713748, C_ss = 3.491620112e-05 kg/m^3. It applies only when completely mixed; constant flow, volume and load: outside that it says what is wrong instead of returning a number.

<a id="enveng_streeter_phelps_critical_point"></a>

#### Critical (minimum) dissolved oxygen of the oxygen sag — `enveng_streeter_phelps_critical_point`

The critical time at which the oxygen deficit of a river is largest, the critical deficit and the minimum dissolved oxygen (Streeter-Phelps).

$$
t_c = \frac{1}{k_a - k_d}\ln\!\left[\frac{k_a}{k_d}\left(1 - \frac{D_0(k_a - k_d)}{k_d L_0}\right)\right],\quad D_c = \frac{k_d}{k_a}L_0e^{-k_d t_c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kd` | in | deoxygenation rate constant | 1/s |
| `ka` | in | reaeration rate constant | 1/s |
| `L0` | in | ultimate BOD of the mixed river water at the outfall | kg/m^3 |
| `D0` | in | oxygen deficit at the outfall (DOsat - DO) | kg/m^3 |
| `DOsat` | in | saturation dissolved oxygen | kg/m^3 |
| `t_c` | out | critical travel time | s |
| `t_c_days` | out | critical time in days | — |
| `D_c` | out | critical (largest) deficit | kg/m^3 |
| `DO_min` | out | minimum dissolved oxygen | kg/m^3 |
| `DO_min_mgL` | out | minimum dissolved oxygen in mg/L | — |

**Applies when:** the critical deficit stays below saturation; if the deficit falls from the outfall on, t_c = 0 and D_c = D0.

```shell
$ environmental-engineering solve enveng_streeter_phelps_critical_point "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "DOsat=9.1 mg/L"
t_c = 220803.3385 s
t_c_days = 2.555594195
D_c = 0.002777777778 kg/m^3
DO_min = 0.006322222222 kg/m^3
DO_min_mgL = 6.322222222
  by: Critical (minimum) dissolved oxygen of the oxygen sag  (t_c = ln[(ka/kd)(1 - D0 (ka - kd)/(kd L0))]/(ka - kd); D_c = (kd/ka) L0 e^(-kd t_c))
  kd = 2.662037037e-06 1/s
  ka = 5.324074074e-06 1/s
  L0 = 0.01 kg/m^3
  D0 = 0.001 kg/m^3
  DOsat = 0.0091 kg/m^3
  assumes: the critical deficit stays below saturation; if the deficit falls from the outfall on, t_c = 0 and D_c = D0
```

In the worked example, kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, DOsat = 9.1 mg/L go through the equation above and give t_c = 220803.3385 s, t_c_days = 2.555594195, D_c = 0.002777777778 kg/m^3, DO_min = 0.006322222222 kg/m^3. It applies only when the critical deficit stays below saturation; if the deficit falls from the outfall on, t_c = 0 and D_c = D0: outside that it says what is wrong instead of returning a number.

<a id="enveng_oxygen_sag_distance"></a>

#### Dissolved oxygen at a distance downstream (Streeter-Phelps) — `enveng_oxygen_sag_distance`

The oxygen deficit and dissolved oxygen at a distance downstream of an outfall from the river velocity (travel time x/v) and the Streeter-Phelps sag.

$$
D(x) = \frac{k_d L_0}{k_a - k_d}\left(e^{-k_d x/v} - e^{-k_a x/v}\right) + D_0 e^{-k_a x/v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kd` | in | deoxygenation rate constant | 1/s |
| `ka` | in | reaeration rate constant | 1/s |
| `L0` | in | ultimate BOD of the mixed river water at the outfall | kg/m^3 |
| `D0` | in | oxygen deficit at the outfall (DOsat - DO) | kg/m^3 |
| `x` | in | distance downstream | m |
| `v` | in | mean river velocity | m/s |
| `DOsat` | in | saturation dissolved oxygen | kg/m^3 |
| `D` | out | oxygen deficit at x | kg/m^3 |
| `DO` | out | dissolved oxygen at x | kg/m^3 |
| `D_mgL` | out | deficit in mg/L | — |
| `DO_mgL` | out | dissolved oxygen in mg/L | — |
| `t` | out | travel time | s |

**Applies when:** plug flow at constant velocity; deficit stays below saturation.

```shell
$ environmental-engineering solve enveng_oxygen_sag_distance "kd=0.23 1/day" "ka=0.46 1/day" "L0=10 mg/L" "D0=1 mg/L" "x=30 km" "v=0.3 m/s" "DOsat=9.1 mg/L"
D = 0.002378123211 kg/m^3
DO = 0.006721876789 kg/m^3
D_mgL = 2.378123211
DO_mgL = 6.721876789
t = 100000 s
  by: Dissolved oxygen at a distance downstream (Streeter-Phelps)  (t = x/v, then the Streeter-Phelps deficit)
  kd = 2.662037037e-06 1/s
  ka = 5.324074074e-06 1/s
  L0 = 0.01 kg/m^3
  D0 = 0.001 kg/m^3
  x = 30000 m
  v = 0.3 m/s
  DOsat = 0.0091 kg/m^3
  assumes: plug flow at constant velocity; deficit stays below saturation
```

In the worked example, kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, x = 30 km, v = 0.3 m/s, DOsat = 9.1 mg/L go through the equation above and give D = 0.002378123211 kg/m^3, DO = 0.006721876789 kg/m^3, D_mgL = 2.378123211, DO_mgL = 6.721876789. It applies only when plug flow at constant velocity; deficit stays below saturation: outside that it says what is wrong instead of returning a number.

<a id="enveng_do_saturation"></a>

#### Dissolved oxygen saturation concentration (Benson-Krause) — `enveng_do_saturation`

The saturation concentration of dissolved oxygen in water in equilibrium with moist air at 1 atm, from the temperature and salinity (Benson and Krause equation as adopted in Standard Methods).

$$
\ln C_s = -139.34411 + \frac{1.575701\times10^5}{T} - \frac{6.642308\times10^7}{T^2} + \frac{1.2438\times10^{10}}{T^3} - \frac{8.621949\times10^{11}}{T^4} - S(\dots)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | water temperature | K |
| `S` | in | salinity in g/kg (0 for fresh water) | — |
| `DOsat` | out | saturation dissolved oxygen | kg/m^3 |
| `DOsat_mgL` | out | in mg/L | — |

**Applies when:** 0-40 degC; 0-40 g/kg salinity; 1 atm moist air.

```shell
$ environmental-engineering solve enveng_do_saturation "T=20 degC" S=0
DOsat = 0.009092426043 kg/m^3
DOsat_mgL = 9.092426043
  by: Dissolved oxygen saturation concentration (Benson-Krause)  (ln C = -139.34411 + 1.575701e5/T - 6.642308e7/T^2 + 1.2438e10/T^3 - 8.621949e11/T^4 - S(1.7674e-2 - 10.754/T + 2140.7/T^2))
  T = 293.15 K
  S = 0
  assumes: 0-40 degC; 0-40 g/kg salinity; 1 atm moist air
```

In the worked example, T = 20 degC, S = 0 go through the equation above and give DOsat = 0.009092426043 kg/m^3, DOsat_mgL = 9.092426043. It applies only when 0-40 degC; 0-40 g/kg salinity; 1 atm moist air: outside that it says what is wrong instead of returning a number.

<a id="enveng_river_first_order_decay"></a>

#### First-order decay of a pollutant downstream (plug flow) — `enveng_river_first_order_decay`

The concentration of a decaying pollutant (BOD, coliforms, a chemical) a distance downstream in a river in plug flow, and the travel time.

$$
C = C_0\,e^{-kx/u}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | concentration at the start (after mixing) | kg/m^3 |
| `k` | in | first-order decay constant | 1/s |
| `x` | in | distance downstream | m |
| `u` | in | mean velocity | m/s |
| `C` | out | concentration at x | kg/m^3 |
| `C_mgL` | out | in mg/L | — |
| `t` | out | travel time x/u | s |
| `fraction_remaining` | out | C/C0 | — |

**Applies when:** plug flow; first-order decay.

```shell
$ environmental-engineering solve enveng_river_first_order_decay "C0=20 mg/L" "k=0.3 1/day" "x=40 km" "u=0.25 m/s"
C = 0.01147506841 kg/m^3
C_mgL = 11.47506841
t = 160000 s
fraction_remaining = 0.5737534207
  by: First-order decay of a pollutant downstream (plug flow)  (C = C0 e^(-k x/u))
  C0 = 0.02 kg/m^3
  k = 3.472222222e-06 1/s
  x = 40000 m
  u = 0.25 m/s
  assumes: plug flow; first-order decay
```

In the worked example, C0 = 20 mg/L, k = 0.3 1/day, x = 40 km, u = 0.25 m/s go through the equation above and give C = 0.01147506841 kg/m^3, C_mgL = 11.47506841, t = 160000 s, fraction_remaining = 0.5737534207. It applies only when plug flow; first-order decay: outside that it says what is wrong instead of returning a number.

<a id="enveng_mixing_several_streams"></a>

#### Flow-weighted mixing of several streams — `enveng_mixing_several_streams`

The flow-weighted concentration (or temperature) of several streams combined, and the total flow; flows in any one unit, concentrations in any one unit.

$$
C = \frac{\sum_i Q_i C_i}{\sum_i Q_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flows of the streams (any one unit, e.g. m^3/s) | — |
| `C` | in | concentrations of the streams (any one unit, e.g. mg/L) | — |
| `C_mix` | out | mixed concentration, in the unit of C | — |
| `Q_total` | out | total flow, in the unit of Q | — |

**Applies when:** complete mixing; conservative substance.

```shell
$ environmental-engineering solve enveng_mixing_several_streams "Q=[2, 0.5, 1]" "C=[3, 120, 40]"
C_mix = 30.28571429
Q_total = 3.5
  by: Flow-weighted mixing of several streams  (C = sum(Q_i C_i)/sum(Q_i))
  assumes: complete mixing; conservative substance
```

In the worked example, Q = [2, 0.5, 1], C = [3, 120, 40] go through the equation above and give C_mix = 30.28571429, Q_total = 3.5. It applies only when complete mixing; conservative substance: outside that it says what is wrong instead of returning a number.

<a id="enveng_pollutant_load"></a>

#### Pollutant mass load of a flow — `enveng_pollutant_load`

The mass load (rate) carried by a flow at a concentration, in kg/s, kg per day and tonnes per year.

$$
W = Q\,C
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow rate | m^3/s |
| `C` | in | concentration | kg/m^3 |
| `W` | out | mass load | kg/s |
| `W_kg_per_day` | out | load in kg per day | — |
| `W_t_per_year` | out | load in tonnes per (365.25-day) year | — |

**Applies when:** Q > 0.

```shell
$ environmental-engineering solve enveng_pollutant_load "Q=0.5 m^3/s" "C=30 mg/L"
W = 0.015 kg/s
W_kg_per_day = 1296
W_t_per_year = 473.364
  by: Pollutant mass load of a flow  (W = Q C)
  Q = 0.5 m^3/s
  C = 0.03 kg/m^3
  assumes: Q > 0
```

In the worked example, Q = 0.5 m^3/s, C = 30 mg/L go through the equation above and give W = 0.015 kg/s, W_kg_per_day = 1296, W_t_per_year = 473.364. It applies only when Q > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_lake_steady_state"></a>

#### Steady-state concentration in a well-mixed lake (first-order decay) — `enveng_lake_steady_state`

The steady concentration of a pollutant in a lake treated as a completely mixed reactor with inflow load, outflow and first-order decay, and the hydraulic residence time.

$$
C = \frac{W}{Q + kV}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | total pollutant input (load) | kg/s |
| `Q` | in | outflow rate | m^3/s |
| `k` | in | first-order decay constant (0 if conservative) | 1/s |
| `V` | in | lake volume | m^3 |
| `C` | out | steady-state concentration | kg/m^3 |
| `C_mgL` | out | in mg/L | — |
| `tau` | out | hydraulic residence time V/Q | s |

**Applies when:** completely mixed; steady state.

```shell
$ environmental-engineering solve enveng_lake_steady_state "W=100 kg/day" "Q=10 m^3/s" "k=0.2 1/day" "V=10000000 m^3"
C = 3.491620112e-05 kg/m^3
C_mgL = 0.03491620112
tau = 1000000 s
  by: Steady-state concentration in a well-mixed lake (first-order decay)  (0 = W - Q C - k C V, C = W/(Q + k V))
  W = 0.001157407407 kg/s
  Q = 10 m^3/s
  k = 2.314814815e-06 1/s
  V = 10000000 m^3
  assumes: completely mixed; steady state
```

In the worked example, W = 100 kg/day, Q = 10 m^3/s, k = 0.2 1/day, V = 10000000 m^3 go through the equation above and give C = 3.491620112e-05 kg/m^3, C_mgL = 0.03491620112, tau = 1000000 s. It applies only when completely mixed; steady state: outside that it says what is wrong instead of returning a number.

<a id="enveng_reaeration_coefficient"></a>

#### Stream reaeration coefficient (O'Connor-Dobbins, Churchill, Owens-Gibbs) — `enveng_reaeration_coefficient`

The reaeration rate constant of a stream at 20 degC from its mean velocity and depth by a chosen empirical formula.

$$
k_a = \frac{3.93\,u^{0.5}}{H^{1.5}}\ \text{(O'Connor-Dobbins)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `method` | in | o_connor_dobbins, churchill or owens_gibbs | — |
| `u` | in | mean stream velocity | m/s |
| `H` | in | mean depth | m |
| `ka` | out | reaeration rate constant at 20 degC | 1/s |
| `ka_per_day` | out | in 1/d | — |

**Applies when:** u in m/s and H in m inside the formula constants.

```shell
$ environmental-engineering solve enveng_reaeration_coefficient method=o_connor_dobbins "u=0.3 m/s" "H=2 m"
ka = 8.808347541e-06 1/s
ka_per_day = 0.7610412275
  by: Stream reaeration coefficient (O'Connor-Dobbins, Churchill, Owens-Gibbs)  (ka = 3.93 u^0.5/H^1.5 (O'Connor-Dobbins), 5.026 u/H^1.67 (Churchill), 5.32 u^0.67/H^1.85 (Owens-Gibbs), per day)
  u = 0.3 m/s
  H = 2 m
  method = o_connor_dobbins
  assumes: u in m/s and H in m inside the formula constants
```

In the worked example, method = o_connor_dobbins, u = 0.3 m/s, H = 2 m go through the equation above and give ka = 8.808347541e-06 1/s, ka_per_day = 0.7610412275. It applies only when u in m/s and H in m inside the formula constants: outside that it says what is wrong instead of returning a number.

<a id="enveng_streeter_phelps_deficit"></a>

#### Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time — `enveng_streeter_phelps_deficit`

The dissolved-oxygen deficit and the dissolved oxygen of a river at a travel time downstream of a BOD discharge (Streeter-Phelps oxygen sag).

$$
D = \frac{k_d L_0}{k_a - k_d}\left(e^{-k_d t} - e^{-k_a t}\right) + D_0 e^{-k_a t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kd` | in | deoxygenation rate constant | 1/s |
| `ka` | in | reaeration rate constant | 1/s |
| `L0` | in | ultimate BOD of the mixed river water at the outfall | kg/m^3 |
| `D0` | in | oxygen deficit at the outfall (DOsat - DO) | kg/m^3 |
| `t` | in | travel time from the outfall | s |
| `DOsat` | in | saturation dissolved oxygen | kg/m^3 |
| `D` | out | oxygen deficit at t | kg/m^3 |
| `DO` | out | dissolved oxygen at t | kg/m^3 |
| `D_mgL` | out | deficit in mg/L | — |
| `DO_mgL` | out | dissolved oxygen in mg/L | — |

**Applies when:** first-order deoxygenation and reaeration; plug flow; deficit stays below saturation.

```shell
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

In the worked example, kd = 0.23 1/day, ka = 0.46 1/day, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 day, DOsat = 9.1 mg/L go through the equation above and give D = 0.002726165085 kg/m^3, DO = 0.006373834915 kg/m^3, D_mgL = 2.726165085, DO_mgL = 6.373834915. It applies only when first-order deoxygenation and reaeration; plug flow; deficit stays below saturation: outside that it says what is wrong instead of returning a number.

<a id="enveng_rate_constant_temperature"></a>

#### Temperature correction of a rate constant (theta model) — `enveng_rate_constant_temperature`

A first-order rate constant (BOD, reaeration, decay) at temperature T from its 20 degC value and the temperature coefficient theta.

$$
k_T = k_{20}\,\theta^{\,T - 20}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k20` | in | rate constant at 20 degC | 1/s |
| `theta` | in | temperature coefficient (1.047 BOD, 1.024 reaeration) | — |
| `T` | in | water temperature | K |
| `kT` | out | rate constant at T | 1/s |
| `kT_per_day` | out | rate constant in 1/d | — |

**Applies when:** 0.9 < theta < 1.2; -5 degC <= T <= 60 degC.

```shell
$ environmental-engineering solve enveng_rate_constant_temperature "k20=0.23 1/day" theta=1.047 "T=25 degC"
kT = 3.349249506e-06 1/s
kT_per_day = 0.2893751573
  by: Temperature correction of a rate constant (theta model)  (kT = k20 theta^(T - 20))
  k20 = 2.662037037e-06 1/s
  theta = 1.047
  T = 298.15 K
  assumes: 0.9 < theta < 1.2; -5 degC <= T <= 60 degC
```

In the worked example, k20 = 0.23 1/day, theta = 1.047, T = 25 degC go through the equation above and give kT = 3.349249506e-06 1/s, kT_per_day = 0.2893751573. It applies only when 0.9 < theta < 1.2; -5 degC <= T <= 60 degC: outside that it says what is wrong instead of returning a number.

<a id="enveng_theoretical_oxygen_demand"></a>

#### Theoretical oxygen demand of an organic compound — `enveng_theoretical_oxygen_demand`

The theoretical oxygen demand (g O2 per g compound) of a compound CaHbOcNd oxidised to CO2 and water, with its nitrogen released as ammonia or, with nitrification, oxidised to nitrate.

$$
\text{ThOD} = \frac{\left(a + \frac b4 - \frac c2 - \frac{3d}{4}\right)M_{O_2}}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | molecular formula with C, H, O, N only (e.g. C6H12O6) | — |
| `nitrification` | in | include the oxygen to nitrify ammonia (yes) or not (no) | — |
| `thod` | out | theoretical oxygen demand, g O2 per g compound | — |
| `o2_per_mol` | out | mol O2 per mol compound | — |
| `molar_mass` | out | molar mass, g/mol | — |

**Applies when:** elements C, H, O, N only; b >= 3d.

```shell
$ environmental-engineering solve enveng_theoretical_oxygen_demand formula=C6H12O6 nitrification=no
thod = 1.065703767
o2_per_mol = 6
molar_mass = 180.15588
  by: Theoretical oxygen demand of an organic compound  (CaHbOcNd + (a + b/4 - c/2 - 3d/4) O2 -> a CO2 + (b - 3d)/2 H2O + d NH3; nitrification adds 2d O2)
  nitrification = no
  formula = C6H12O6
  assumes: elements C, H, O, N only; b >= 3d
```

In the worked example, formula = C6H12O6, nitrification = no go through the equation above and give thod = 1.065703767, o2_per_mol = 6, molar_mass = 180.15588. It applies only when elements C, H, O, N only; b >= 3d: outside that it says what is wrong instead of returning a number.

<a id="enveng_ultimate_bod"></a>

#### Ultimate BOD from a BOD test — `enveng_ultimate_bod`

The ultimate carbonaceous BOD from a BOD measured after t days (e.g. BOD5) and the rate constant.

$$
L_0 = \frac{BOD_t}{1 - e^{-kt}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `BOD_t` | in | BOD measured after time t (e.g. BOD5) | kg/m^3 |
| `k` | in | BOD rate constant (base e) | 1/s |
| `t` | in | incubation time | s |
| `L0` | out | ultimate BOD | kg/m^3 |
| `L0_mgL` | out | ultimate BOD in mg/L | — |

**Applies when:** first-order kinetics.

```shell
$ environmental-engineering solve enveng_ultimate_bod "BOD_t=200 mg/L" "k=0.23 1/day" "t=5 day"
L0 = 0.2926701219 kg/m^3
L0_mgL = 292.6701219
  by: Ultimate BOD from a BOD test  (L0 = BOD_t/(1 - e^(-k t)))
  BOD_t = 0.2 kg/m^3
  k = 2.662037037e-06 1/s
  t = 432000 s
  assumes: first-order kinetics
```

In the worked example, BOD_t = 200 mg/L, k = 0.23 1/day, t = 5 day go through the equation above and give L0 = 0.2926701219 kg/m^3, L0_mgL = 292.6701219. It applies only when first-order kinetics: outside that it says what is wrong instead of returning a number.

### Water treatment (26)

<a id="enveng_freundlich_carbon_dose"></a>

#### Activated carbon dose from the Freundlich isotherm — `enveng_freundlich_carbon_dose`

The equilibrium adsorbent loading from the Freundlich isotherm q = K Ce^(1/n) (q in mg/g, Ce in mg/L) and the batch carbon dose that lowers C0 to Ce.

$$
q_e = K C_e^{1/n},\quad \frac{m}{V} = \frac{C_0 - C_e}{q_e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | kg/m^3 |
| `Ce` | in | target equilibrium concentration | kg/m^3 |
| `K` | in | Freundlich capacity factor, (mg/g)(L/mg)^(1/n) | — |
| `n` | in | Freundlich exponent n (isotherm exponent 1/n) | — |
| `qe` | out | equilibrium loading, mg/g | — |
| `dose` | out | adsorbent dose | kg/m^3 |
| `dose_mgL` | out | dose in mg/L | — |

**Applies when:** Ce < C0; batch equilibrium.

```shell
$ environmental-engineering solve enveng_freundlich_carbon_dose "C0=10 mg/L" "Ce=1 mg/L" K=50 n=2
qe = 50
dose = 0.18 kg/m^3
dose_mgL = 180
  by: Activated carbon dose from the Freundlich isotherm  (qe = K Ce^(1/n); dose = (C0 - Ce)/qe)
  C0 = 0.01 kg/m^3
  Ce = 0.001 kg/m^3
  K = 50
  n = 2
  assumes: Ce < C0; batch equilibrium
```

In the worked example, C0 = 10 mg/L, Ce = 1 mg/L, K = 50, n = 2 go through the equation above and give qe = 50, dose = 0.18 kg/m^3, dose_mgL = 180. It applies only when Ce < C0; batch equilibrium: outside that it says what is wrong instead of returning a number.

<a id="enveng_langmuir_carbon_dose"></a>

#### Adsorbent dose from the Langmuir isotherm — `enveng_langmuir_carbon_dose`

The equilibrium loading from the Langmuir isotherm q = qmax b Ce/(1 + b Ce) (q in mg/g, Ce in mg/L) and the batch adsorbent dose that lowers C0 to Ce.

$$
q_e = \frac{q_{max}bC_e}{1 + bC_e},\quad \frac{m}{V} = \frac{C_0 - C_e}{q_e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | kg/m^3 |
| `Ce` | in | target equilibrium concentration | kg/m^3 |
| `qmax` | in | Langmuir monolayer capacity, mg/g | — |
| `b` | in | Langmuir constant, L/mg | — |
| `qe` | out | equilibrium loading, mg/g | — |
| `dose` | out | adsorbent dose | kg/m^3 |
| `dose_mgL` | out | dose in mg/L | — |

**Applies when:** Ce < C0.

```shell
$ environmental-engineering solve enveng_langmuir_carbon_dose "C0=10 mg/L" "Ce=1 mg/L" qmax=150 b=0.8
qe = 66.66666667
dose = 0.135 kg/m^3
dose_mgL = 135
  by: Adsorbent dose from the Langmuir isotherm  (qe = qmax b Ce/(1 + b Ce); dose = (C0 - Ce)/qe)
  C0 = 0.01 kg/m^3
  Ce = 0.001 kg/m^3
  qmax = 150
  b = 0.8
  assumes: Ce < C0
```

In the worked example, C0 = 10 mg/L, Ce = 1 mg/L, qmax = 150, b = 0.8 go through the equation above and give qe = 66.66666667, dose = 0.135 kg/m^3, dose_mgL = 135. It applies only when Ce < C0: outside that it says what is wrong instead of returning a number.

<a id="enveng_chlorine_dose_feed"></a>

#### Chlorine dose and feed rate — `enveng_chlorine_dose_feed`

The chlorine dose (demand plus residual), the chlorine feed rate for a flow and the feed of a product with a stated available-chlorine fraction (e.g. 65 % calcium hypochlorite).

$$
\dot m = Q\,(\text{demand} + \text{residual}),\quad \dot m_{product} = \frac{\dot m}{p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow treated | m^3/s |
| `demand` | in | chlorine demand | kg/m^3 |
| `residual` | in | required free chlorine residual | kg/m^3 |
| `purity` | in | available chlorine fraction of the product (1 for chlorine gas; fraction or %) | — |
| `dose` | out | chlorine dose | kg/m^3 |
| `dose_mgL` | out | dose in mg/L | — |
| `feed` | out | chlorine feed rate | kg/s |
| `feed_kg_per_day` | out | chlorine feed in kg/d | — |
| `product_kg_per_day` | out | product feed in kg/d | — |

**Applies when:** 0 < purity <= 1.

```shell
$ environmental-engineering solve enveng_chlorine_dose_feed "Q=20000 m^3/day" "demand=2.5 mg/L" "residual=0.5 mg/L" purity=1
dose = 0.003 kg/m^3
dose_mgL = 3
feed = 0.0006944444444 kg/s
feed_kg_per_day = 60
product_kg_per_day = 60
  by: Chlorine dose and feed rate  (dose = demand + residual; feed = Q dose; product = feed/purity)
  Q = 0.2314814815 m^3/s
  demand = 0.0025 kg/m^3
  residual = 0.0005 kg/m^3
  purity = 1
  assumes: 0 < purity <= 1
```

In the worked example, Q = 20000 m^3/day, demand = 2.5 mg/L, residual = 0.5 mg/L, purity = 1 go through the equation above and give dose = 0.003 kg/m^3, dose_mgL = 3, feed = 0.0006944444444 kg/s, feed_kg_per_day = 60. It applies only when 0 < purity <= 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_filter_clean_bed_headloss"></a>

#### Clean-bed head loss through a granular filter (Ergun / Carman-Kozeny) — `enveng_filter_clean_bed_headloss`

The head loss through a clean granular filter bed from the grain size, sphericity, porosity, depth and filtration rate (Ergun equation, the Carman-Kozeny form with friction factor 150/Re + 1.75).

$$
\frac{h}{L} = \frac{150\mu(1-\varepsilon)^2 v}{\rho g\varepsilon^3(\psi d)^2} + \frac{1.75(1-\varepsilon)v^2}{g\varepsilon^3\psi d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | grain diameter | m |
| `eps` | in | bed porosity (fraction) | — |
| `psi` | in | grain sphericity (fraction, 1 for spheres) | — |
| `L` | in | bed depth | m |
| `v` | in | filtration (approach) velocity | m/s |
| `rho` | in | water density | kg/m^3 |
| `mu` | in | water viscosity | Pa*s |
| `dP` | out | pressure drop | Pa |
| `h` | out | head loss | m |
| `Re` | out | bed Reynolds number rho v psi d/(mu (1 - eps)) | — |

**Applies when:** 0 < eps < 1; 0 < psi <= 1; clean bed.

```shell
$ environmental-engineering solve enveng_filter_clean_bed_headloss "d=0.5 mm" eps=0.4 psi=0.85 "L=0.7 m" "v=8 m/h" "rho=998.2 kg/m^3" "mu=0.001002 Pa*s"
dP = 7414.170928 Pa
h = 0.7573983471 m
Re = 1.568104532
  by: Clean-bed head loss through a granular filter (Ergun / Carman-Kozeny)  (dP/L = 150 mu (1-eps)^2 v/(eps^3 (psi d)^2) + 1.75 rho (1-eps) v^2/(eps^3 psi d); h = dP/(rho g))
  d = 0.0005 m
  eps = 0.4
  psi = 0.85
  L = 0.7 m
  v = 0.002222222222 m/s
  rho = 998.2 kg/m^3
  mu = 0.001002 Pa*s
  assumes: 0 < eps < 1; 0 < psi <= 1; clean bed
```

In the worked example, d = 0.5 mm, eps = 0.4, psi = 0.85, L = 0.7 m, v = 8 m/h, rho = 998.2 kg/m^3, mu = 0.001002 Pa*s go through the equation above and give dP = 7414.170928 Pa, h = 0.7573983471 m, Re = 1.568104532. It applies only when 0 < eps < 1; 0 < psi <= 1; clean bed: outside that it says what is wrong instead of returning a number.

<a id="enveng_chick_watson"></a>

#### Disinfection by the Chick-Watson law — `enveng_chick_watson`

The survival fraction and log inactivation of organisms exposed to a disinfectant concentration C for time t under ln(N/N0) = -k C^n t (k in (L/mg)^n per minute, C in mg/L, t in minutes inside the law).

$$
\ln\frac{N}{N_0} = -k\,C^n t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | Chick-Watson coefficient in (L/mg)^n per minute | — |
| `n` | in | coefficient of dilution (1 for most disinfectants) | — |
| `C` | in | disinfectant concentration | kg/m^3 |
| `t` | in | contact time | s |
| `survival` | out | N/N0 | — |
| `log_inactivation` | out | log10 inactivation | — |

**Applies when:** constant concentration; 0.3 <= n <= 3.

```shell
$ environmental-engineering solve enveng_chick_watson k=0.1 n=1 "C=1 mg/L" "t=30 min"
survival = 0.04978706837
log_inactivation = 1.302883446
  by: Disinfection by the Chick-Watson law  (ln(N/N0) = -k C^n t with C in mg/L and t in minutes)
  k = 0.1
  n = 1
  C = 0.001 kg/m^3
  t = 1800 s
  assumes: constant concentration; 0.3 <= n <= 3
```

In the worked example, k = 0.1, n = 1, C = 1 mg/L, t = 30 min go through the equation above and give survival = 0.04978706837, log_inactivation = 1.302883446. It applies only when constant concentration; 0.3 <= n <= 3: outside that it says what is wrong instead of returning a number.

<a id="enveng_disinfection_ct"></a>

#### Disinfection CT value of a contact basin — `enveng_disinfection_ct`

The effective contact time t10 of a basin (baffling factor times the theoretical detention time) and the CT value of a disinfectant residual.

$$
CT = C\,t_{10} = C\,BF\,\frac{V}{Q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | disinfectant residual at the outlet | kg/m^3 |
| `V` | in | basin volume | m^3 |
| `Q` | in | peak flow | m^3/s |
| `BF` | in | baffling factor t10/T (0.1 unbaffled to 1 plug flow) | — |
| `t10` | out | effective contact time | s |
| `CT` | out | CT value | kg*s/m^3 |
| `CT_mg_min_L` | out | CT in mg min/L | — |

**Applies when:** 0 < BF <= 1.

```shell
$ environmental-engineering solve enveng_disinfection_ct "C=1 mg/L" "V=1000 m^3" "Q=20000 m^3/day" BF=0.5
t10 = 2160 s
CT = 2.16 kg*s/m^3
CT_mg_min_L = 36
  by: Disinfection CT value of a contact basin  (t10 = BF V/Q; CT = C t10)
  C = 0.001 kg/m^3
  V = 1000 m^3
  Q = 0.2314814815 m^3/s
  BF = 0.5
  assumes: 0 < BF <= 1
```

In the worked example, C = 1 mg/L, V = 1000 m^3, Q = 20000 m^3/day, BF = 0.5 go through the equation above and give t10 = 2160 s, CT = 2.16 kg*s/m^3, CT_mg_min_L = 36. It applies only when 0 < BF <= 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_backwash_bed_expansion"></a>

#### Filter bed expansion during backwash — `enveng_backwash_bed_expansion`

The expanded porosity, expanded depth and percentage expansion of a filter bed fluidised by backwash, from the backwash rate and the grains' settling velocity (expanded porosity (vb/vs)^0.22).

$$
\varepsilon_e = \left(\frac{v_b}{v_s}\right)^{0.22},\quad L_e = \frac{L(1-\varepsilon)}{1-\varepsilon_e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | unexpanded bed depth | m |
| `eps` | in | unexpanded porosity (fraction) | — |
| `vb` | in | backwash rate | m/s |
| `vs` | in | settling velocity of the grains | m/s |
| `eps_e` | out | expanded porosity | — |
| `Le` | out | expanded bed depth | m |
| `expansion` | out | fractional expansion Le/L - 1 | — |

**Applies when:** the bed is fluidised: eps < (vb/vs)^0.22 < 1.

```shell
$ environmental-engineering solve enveng_backwash_bed_expansion "L=0.7 m" eps=0.42 "vb=50 m/h" "vs=300 m/h"
eps_e = 0.6742279077
Le = 1.246270045 m
expansion = 0.780385778
  by: Filter bed expansion during backwash  (eps_e = (vb/vs)^0.22; Le = L (1 - eps)/(1 - eps_e))
  L = 0.7 m
  eps = 0.42
  vb = 0.01388888889 m/s
  vs = 0.08333333333 m/s
  assumes: the bed is fluidised: eps < (vb/vs)^0.22 < 1
```

In the worked example, L = 0.7 m, eps = 0.42, vb = 50 m/h, vs = 300 m/h go through the equation above and give eps_e = 0.6742279077, Le = 1.246270045 m, expansion = 0.780385778. It applies only when the bed is fluidised: eps < (vb/vs)^0.22 < 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_filtration_rate"></a>

#### Filtration (hydraulic loading) rate with one filter out of service — `enveng_filtration_rate`

The filtration rate of a bank of identical filters, with all in service and with one out of service (for backwashing).

$$
v = \frac{Q}{NA},\quad v' = \frac{Q}{(N-1)A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | plant flow | m^3/s |
| `A` | in | area of one filter | m^2 |
| `N` | in | number of filters | — |
| `v` | out | filtration rate with all filters | m/s |
| `v_m_per_h` | out | in m/h | — |
| `v_one_out` | out | rate with one filter out of service | m/s |
| `v_one_out_m_per_h` | out | in m/h | — |

**Applies when:** N >= 2 identical filters.

```shell
$ environmental-engineering solve enveng_filtration_rate "Q=40000 m^3/day" "A=50 m^2" N=6
v = 0.001543209877 m/s
v_m_per_h = 5.555555556
v_one_out = 0.001851851852 m/s
v_one_out_m_per_h = 6.666666667
  by: Filtration (hydraulic loading) rate with one filter out of service  (v = Q/(N A); v' = Q/((N - 1) A))
  Q = 0.462962963 m^3/s
  A = 50 m^2
  assumes: N >= 2 identical filters
```

In the worked example, Q = 40000 m^3/day, A = 50 m^2, N = 6 go through the equation above and give v = 0.001543209877 m/s, v_m_per_h = 5.555555556, v_one_out = 0.001851851852 m/s, v_one_out_m_per_h = 6.666666667. It applies only when N >= 2 identical filters: outside that it says what is wrong instead of returning a number.

<a id="enveng_ideal_basin_removal"></a>

#### Fraction of discrete particles removed in an ideal settling basin — `enveng_ideal_basin_removal`

The overflow rate of an ideal (Camp) settling basin and the fraction of particles of one settling velocity it removes (all if vs >= v0, otherwise vs/v0).

$$
R = \min\left(1, \frac{v_s}{v_0}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `vs` | in | settling velocity of the particles | m/s |
| `Q` | in | flow | m^3/s |
| `A` | in | basin surface area | m^2 |
| `v0` | out | overflow rate Q/A | m/s |
| `R` | out | fraction removed | — |

**Applies when:** ideal basin, discrete (type I) settling.

```shell
$ environmental-engineering solve enveng_ideal_basin_removal "vs=0.5 m/h" "Q=10000 m^3/day" "A=400 m^2"
v0 = 0.0002893518519 m/s
R = 0.48
  by: Fraction of discrete particles removed in an ideal settling basin  (v0 = Q/A; R = min(1, vs/v0))
  vs = 0.0001388888889 m/s
  Q = 0.1157407407 m^3/s
  A = 400 m^2
  assumes: ideal basin, discrete (type I) settling
```

In the worked example, vs = 0.5 m/h, Q = 10000 m^3/day, A = 400 m^2 go through the equation above and give v0 = 0.0002893518519 m/s, R = 0.48. It applies only when ideal basin, discrete (type I) settling: outside that it says what is wrong instead of returning a number.

<a id="enveng_hypochlorous_acid_fraction"></a>

#### Fraction of free chlorine as hypochlorous acid (pH) — `enveng_hypochlorous_acid_fraction`

The fractions of free chlorine present as HOCl and as OCl- at a pH, from the acid dissociation constant (given, or from the temperature by Morris' correlation).

$$
\alpha_{HOCl} = \frac{1}{1 + 10^{\,pH - pK_a}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pH` | in | pH of the water | — |
| `pKa` | in | pKa of HOCl (7.54 at 25 degC) (optional) | — |
| `T` | in | water temperature (to compute pKa instead) (optional) | K |
| `alpha_HOCl` | out | fraction as HOCl | — |
| `alpha_OCl` | out | fraction as OCl- | — |
| `pKa` | out | pKa used | — |

**Applies when:** either pKa or T; 0-35 degC for the correlation.

```shell
$ environmental-engineering solve enveng_hypochlorous_acid_fraction pH=7 pKa=7.54
alpha_HOCl = 0.7761545753
alpha_OCl = 0.2238454247
pKa = 7.54
  by: Fraction of free chlorine as hypochlorous acid (pH)  (alpha_HOCl = 1/(1 + 10^(pH - pKa)); pKa = 3000/T - 10.0686 + 0.0253 T (Morris))
  pH = 7
  pKa = 7.54
  assumes: either pKa or T; 0-35 degC for the correlation
```

In the worked example, pH = 7, pKa = 7.54 go through the equation above and give alpha_HOCl = 0.7761545753, alpha_OCl = 0.2238454247, pKa = 7.54. It applies only when either pKa or T; 0-35 degC for the correlation: outside that it says what is wrong instead of returning a number.

<a id="enveng_freundlich_fit"></a>

#### Freundlich isotherm constants fitted to adsorption data — `enveng_freundlich_fit`

The Freundlich constants K and 1/n fitted by linear regression of log q against log Ce, with the coefficient of determination.

$$
\log q_e = \log K + \frac1n\log C_e
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ce` | in | equilibrium concentrations in mg/L | — |
| `qe` | in | equilibrium loadings in mg/g (same order) | — |
| `K` | out | Freundlich K, (mg/g)(L/mg)^(1/n) | — |
| `inv_n` | out | isotherm exponent 1/n | — |
| `n` | out | Freundlich n | — |
| `r2` | out | coefficient of determination of the log-log fit | — |

**Applies when:** at least three positive points.

```shell
$ environmental-engineering solve enveng_freundlich_fit "Ce=[0.5, 1, 2, 5, 10]" "qe=[28, 40, 55, 86, 120]"
K = 39.47623429
inv_n = 0.483576413
n = 2.067925509
r2 = 0.9998093398
  by: Freundlich isotherm constants fitted to adsorption data  (least squares of log10 q = log10 K + (1/n) log10 Ce)
  assumes: at least three positive points
```

In the worked example, Ce = [0.5, 1, 2, 5, 10], qe = [28, 40, 55, 86, 120] go through the equation above and give K = 39.47623429, inv_n = 0.483576413, n = 2.067925509, r2 = 0.9998093398. It applies only when at least three positive points: outside that it says what is wrong instead of returning a number.

<a id="enveng_detention_time"></a>

#### Hydraulic detention time — `enveng_detention_time`

The hydraulic detention (residence) time of a tank or basin from its volume and the flow.

$$
\theta = \frac{V}{Q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | tank volume | m^3 |
| `Q` | in | flow rate | m^3/s |
| `theta` | out | detention time V/Q | s |
| `theta_h` | out | detention time in hours | — |

**Applies when:** V, Q > 0.

```shell
$ environmental-engineering solve enveng_detention_time "V=1000 m^3" "Q=12000 m^3/day"
theta = 7200 s
theta_h = 2
  by: Hydraulic detention time  (theta = V/Q)
  V = 1000 m^3
  Q = 0.1388888889 m^3/s
  assumes: V, Q > 0
```

In the worked example, V = 1000 m^3, Q = 12000 m^3/day go through the equation above and give theta = 7200 s, theta_h = 2. It applies only when V, Q > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_langmuir_fit"></a>

#### Langmuir isotherm constants fitted to adsorption data — `enveng_langmuir_fit`

The Langmuir capacity qmax and constant b fitted by linear regression of Ce/q against Ce (linearised Langmuir form), with the coefficient of determination.

$$
\frac{C_e}{q_e} = \frac{1}{q_{max}b} + \frac{C_e}{q_{max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ce` | in | equilibrium concentrations in mg/L | — |
| `qe` | in | equilibrium loadings in mg/g (same order) | — |
| `qmax` | out | monolayer capacity, mg/g | — |
| `b` | out | Langmuir constant, L/mg | — |
| `r2` | out | coefficient of determination of the linearised fit | — |

**Applies when:** positive slope and intercept of Ce/q against Ce.

```shell
$ environmental-engineering solve enveng_langmuir_fit "Ce=[0.5, 1, 2, 5, 10]" "qe=[40, 66, 97, 133, 152]"
qmax = 177.9004838
b = 0.5896757373
r2 = 0.9999728233
  by: Langmuir isotherm constants fitted to adsorption data  (least squares of Ce/q = 1/(qmax b) + Ce/qmax)
  assumes: positive slope and intercept of Ce/q against Ce
```

In the worked example, Ce = [0.5, 1, 2, 5, 10], qe = [40, 66, 97, 133, 152] go through the equation above and give qmax = 177.9004838, b = 0.5896757373, r2 = 0.9999728233. It applies only when positive slope and intercept of Ce/q against Ce: outside that it says what is wrong instead of returning a number.

<a id="enveng_ct_log_inactivation"></a>

#### Log inactivation from the CT ratio — `enveng_ct_log_inactivation`

The CT achieved, the ratio to the CT required for 3-log (99.9 %) inactivation from the regulatory tables, and the log inactivation credited (3 times the ratio).

$$
\text{log inactivation} = 3\,\frac{C t_{10}}{CT_{99.9}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | disinfectant residual | kg/m^3 |
| `t10` | in | effective contact time | s |
| `CT999` | in | CT required for 3-log inactivation (from the tables) | kg*s/m^3 |
| `CT` | out | CT achieved | kg*s/m^3 |
| `ratio` | out | CT achieved / CT for 3-log | — |
| `log_inactivation` | out | log inactivation credited | — |

**Applies when:** first-order (Chick-Watson) inactivation with n = 1.

```shell
$ environmental-engineering solve enveng_ct_log_inactivation "C=1.2 mg/L" "t10=40 min" "CT999=104 mg*min/L"
CT = 2.88 kg*s/m^3
ratio = 0.4615384615
log_inactivation = 1.384615385
  by: Log inactivation from the CT ratio  (log inactivation = 3 CT/CT_99.9)
  C = 0.0012 kg/m^3
  t10 = 2400 s
  CT999 = 6.24 kg*s/m^3
  assumes: first-order (Chick-Watson) inactivation with n = 1
```

In the worked example, C = 1.2 mg/L, t10 = 40 min, CT999 = 104 mg*min/L go through the equation above and give CT = 2.88 kg*s/m^3, ratio = 0.4615384615, log_inactivation = 1.384615385. It applies only when first-order (Chick-Watson) inactivation with n = 1: outside that it says what is wrong instead of returning a number.

<a id="enveng_log_removal"></a>

#### Log removal and percentage removal — `enveng_log_removal`

The log10 removal (log reduction value) and the percentage removal from the counts or concentrations before and after treatment.

$$
LR = \log_{10}\frac{N_0}{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | count or concentration before treatment | — |
| `N` | in | count or concentration after treatment (same unit) | — |
| `log_removal` | out | log10 removal | — |
| `percent_removal` | out | percentage removed | — |

**Applies when:** 0 < N < N0.

```shell
$ environmental-engineering solve enveng_log_removal N0=1000000 N=100
log_removal = 4
percent_removal = 99.99
  by: Log removal and percentage removal  (LR = log10(N0/N); percent = (1 - N/N0) 100)
  N0 = 1000000
  N = 100
  assumes: 0 < N < N0
```

In the worked example, N0 = 1000000, N = 100 go through the equation above and give log_removal = 4, percent_removal = 99.99. It applies only when 0 < N < N0: outside that it says what is wrong instead of returning a number.

<a id="enveng_velocity_gradient"></a>

#### Mean velocity gradient G (rapid mixing and flocculation) — `enveng_velocity_gradient`

The root-mean-square velocity gradient G of a mixing or flocculation tank from the power dissipated, the viscosity and the volume, and the Camp number G t when the detention time is given.

$$
G = \sqrt{\frac{P}{\mu V}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | power dissipated in the water | W |
| `mu` | in | water viscosity | Pa*s |
| `V` | in | tank volume | m^3 |
| `t` | in | detention time (for G t) (optional) | s |
| `G` | out | mean velocity gradient | 1/s |
| `Gt` | out | Camp number G t (dimensionless) | — |

**Applies when:** P, mu, V > 0.

```shell
$ environmental-engineering solve enveng_velocity_gradient "P=2000 W" "mu=0.001002 Pa*s" "V=50 m^3"
G = 199.8002995 1/s
  by: Mean velocity gradient G (rapid mixing and flocculation)  (G = sqrt(P/(mu V)); Camp number G t)
  P = 2000 W
  mu = 0.001002 Pa*s
  V = 50 m^3
  assumes: P, mu, V > 0
```

In the worked example, P = 2000 W, mu = 0.001002 Pa*s, V = 50 m^3 go through the equation above and give G = 199.8002995 1/s. It applies only when P, mu, V > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_membrane_mass_balance"></a>

#### Membrane recovery, rejection, flux and concentrate — `enveng_membrane_mass_balance`

The water recovery, solute rejection, permeate flux (also in L/(m^2 h)) and the concentrate flow and concentration of a membrane unit from the feed and permeate flows and concentrations and the membrane area.

$$
r = \frac{Q_p}{Q_f},\ R = 1 - \frac{C_p}{C_f},\ C_c = \frac{Q_fC_f - Q_pC_p}{Q_f - Q_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qf` | in | feed flow | m^3/s |
| `Qp` | in | permeate flow | m^3/s |
| `Cf` | in | feed concentration | kg/m^3 |
| `Cp` | in | permeate concentration | kg/m^3 |
| `A` | in | membrane area | m^2 |
| `recovery` | out | Qp/Qf | — |
| `rejection` | out | 1 - Cp/Cf | — |
| `J` | out | permeate flux | m/s |
| `J_LMH` | out | flux in L/(m^2 h) | — |
| `Qc` | out | concentrate flow | m^3/s |
| `Cc` | out | concentrate concentration | kg/m^3 |
| `Cc_mgL` | out | concentrate concentration in mg/L | — |

**Applies when:** Qp < Qf; Cp < Cf; steady state.

```shell
$ environmental-engineering solve enveng_membrane_mass_balance "Qf=1000 m^3/day" "Qp=750 m^3/day" "Cf=2000 mg/L" "Cp=50 mg/L" "A=1500 m^2"
recovery = 0.75
rejection = 0.975
J = 5.787037037e-06 m/s
J_LMH = 20.83333333
Qc = 0.002893518519 m^3/s
Cc = 7.85 kg/m^3
Cc_mgL = 7850
  by: Membrane recovery, rejection, flux and concentrate  (r = Qp/Qf; R = 1 - Cp/Cf; J = Qp/A; Qc = Qf - Qp; Cc = (Qf Cf - Qp Cp)/Qc)
  Qf = 0.01157407407 m^3/s
  Qp = 0.008680555556 m^3/s
  Cf = 2 kg/m^3
  Cp = 0.05 kg/m^3
  A = 1500 m^2
  assumes: Qp < Qf; Cp < Cf; steady state
```

In the worked example, Qf = 1000 m^3/day, Qp = 750 m^3/day, Cf = 2000 mg/L, Cp = 50 mg/L, A = 1500 m^2 go through the equation above and give recovery = 0.75, rejection = 0.975, J = 5.787037037e-06 m/s, J_LMH = 20.83333333. It applies only when Qp < Qf; Cp < Cf; steady state: outside that it says what is wrong instead of returning a number.

<a id="enveng_chick_law"></a>

#### Microbial inactivation by Chick's law (first order) — `enveng_chick_law`

The surviving organisms, survival fraction, log removal and percentage removal after a contact time under first-order (Chick's law) inactivation or die-off.

$$
\frac{N}{N_0} = e^{-kt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial organism concentration (count per volume, e.g. per 100 mL) | — |
| `k` | in | inactivation (die-off) rate constant | 1/s |
| `t` | in | contact time | s |
| `N` | out | surviving organisms, same unit as N0 | — |
| `survival` | out | N/N0 | — |
| `log_removal` | out | log10 removal | — |
| `percent_removal` | out | percentage removed | — |

**Applies when:** first-order kinetics.

```shell
$ environmental-engineering solve enveng_chick_law N0=1000000 "k=0.2 1/min" "t=30 min"
N = 2478.752177
survival = 0.002478752177
log_removal = 2.605766891
percent_removal = 99.75212478
  by: Microbial inactivation by Chick's law (first order)  (N = N0 e^(-k t); log removal = k t/ln 10)
  N0 = 1000000
  k = 0.003333333333 1/s
  t = 1800 s
  assumes: first-order kinetics
```

In the worked example, N0 = 1000000, k = 0.2 1/min, t = 30 min go through the equation above and give N = 2478.752177, survival = 0.002478752177, log_removal = 2.605766891, percent_removal = 99.75212478. It applies only when first-order kinetics: outside that it says what is wrong instead of returning a number.

<a id="enveng_osmotic_pressure"></a>

#### Osmotic pressure of a dilute solution (van 't Hoff) — `enveng_osmotic_pressure`

The osmotic pressure of a dilute salt solution from its mass concentration, molar mass, van 't Hoff factor and temperature.

$$
\pi = i\,\frac{C}{M}\,RT
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | solute mass concentration | kg/m^3 |
| `M` | in | solute molar mass | kg/mol |
| `i` | in | van 't Hoff factor (ions per formula unit, 2 for NaCl) | — |
| `T` | in | temperature | K |
| `pi` | out | osmotic pressure | Pa |
| `pi_bar` | out | osmotic pressure in bar | — |

**Applies when:** dilute (ideal) solution.

```shell
$ environmental-engineering solve enveng_osmotic_pressure "C=35 g/L" "M=58.44 g/mol" i=2 "T=298.15 K"
pi = 2969318.824 Pa
pi_bar = 29.69318824
  by: Osmotic pressure of a dilute solution (van 't Hoff)  (pi = i (C/M) R T)
  C = 35 kg/m^3
  M = 0.05844 kg/mol
  i = 2
  T = 298.15 K
  assumes: dilute (ideal) solution
```

In the worked example, C = 35 g/L, M = 58.44 g/mol, i = 2, T = 298.15 K go through the equation above and give pi = 2969318.824 Pa, pi_bar = 29.69318824. It applies only when dilute (ideal) solution: outside that it says what is wrong instead of returning a number.

<a id="enveng_mixing_power"></a>

#### Power needed for a target velocity gradient — `enveng_mixing_power`

The power that must be dissipated in a tank to give a target mean velocity gradient G.

$$
P = G^2\mu V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | target velocity gradient | 1/s |
| `mu` | in | water viscosity | Pa*s |
| `V` | in | tank volume | m^3 |
| `P` | out | power required | W |

**Applies when:** G, mu, V > 0.

```shell
$ environmental-engineering solve enveng_mixing_power "G=700 1/s" "mu=0.001002 Pa*s" "V=10 m^3"
P = 4909.8 W
  by: Power needed for a target velocity gradient  (P = G^2 mu V)
  G = 700 1/s
  mu = 0.001002 Pa*s
  V = 10 m^3
  assumes: G, mu, V > 0
```

In the worked example, G = 700 1/s, mu = 0.001002 Pa*s, V = 10 m^3 go through the equation above and give P = 4909.8 W. It applies only when G, mu, V > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_sedimentation_basin_sizing"></a>

#### Sedimentation basin sizing from overflow rate and detention time — `enveng_sedimentation_basin_sizing`

The surface area, volume, side-water depth and (circular) diameter of a settling basin from the design flow, the design overflow rate and the detention time.

$$
A = \frac{Q}{v_0},\ V = Q\theta,\ h = v_0\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | design flow | m^3/s |
| `v0` | in | design overflow rate (surface loading) | m/s |
| `theta` | in | design detention time | s |
| `A` | out | surface area Q/v0 | m^2 |
| `V` | out | volume Q theta | m^3 |
| `depth` | out | depth V/A = v0 theta | m |
| `D` | out | diameter of one circular basin | m |

**Applies when:** one basin.

```shell
$ environmental-engineering solve enveng_sedimentation_basin_sizing "Q=15000 m^3/day" "v0=40 m/day" "theta=2 h"
A = 375 m^2
V = 1250 m^3
depth = 3.333333333 m
D = 21.85096861 m
  by: Sedimentation basin sizing from overflow rate and detention time  (A = Q/v0; V = Q theta; depth = V/A; D = sqrt(4A/pi))
  Q = 0.1736111111 m^3/s
  v0 = 0.000462962963 m/s
  theta = 7200 s
  assumes: one basin
```

In the worked example, Q = 15000 m^3/day, v0 = 40 m/day, theta = 2 h go through the equation above and give A = 375 m^2, V = 1250 m^3, depth = 3.333333333 m, D = 21.85096861 m. It applies only when one basin: outside that it says what is wrong instead of returning a number.

<a id="enveng_stokes_settling_velocity"></a>

#### Stokes settling velocity of a discrete particle — `enveng_stokes_settling_velocity`

The terminal settling velocity of a small spherical particle in water by Stokes' law, with the particle Reynolds number that confirms laminar settling.

$$
v_s = \frac{g(\rho_p - \rho_w)d^2}{18\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | particle diameter | m |
| `rho_p` | in | particle density | kg/m^3 |
| `rho_w` | in | water density | kg/m^3 |
| `mu` | in | water viscosity | Pa*s |
| `vs` | out | settling velocity | m/s |
| `Re` | out | particle Reynolds number | — |
| `vs_m_per_h` | out | in m/h | — |

**Applies when:** Re <= 1; rho_p > rho_w.

```shell
$ environmental-engineering solve enveng_stokes_settling_velocity "d=0.05 mm" "rho_p=2650 kg/m^3" "rho_w=998.2 kg/m^3" "mu=0.001002 Pa*s"
vs = 0.002245318318 m/s
Re = 0.1118401569
vs_m_per_h = 8.083145943
  by: Stokes settling velocity of a discrete particle  (vs = g (rho_p - rho_w) d^2/(18 mu))
  d = 5e-05 m
  rho_p = 2650 kg/m^3
  rho_w = 998.2 kg/m^3
  mu = 0.001002 Pa*s
  assumes: Re <= 1; rho_p > rho_w
```

In the worked example, d = 0.05 mm, rho_p = 2650 kg/m^3, rho_w = 998.2 kg/m^3, mu = 0.001002 Pa*s go through the equation above and give vs = 0.002245318318 m/s, Re = 0.1118401569, vs_m_per_h = 8.083145943. It applies only when Re <= 1; rho_p > rho_w: outside that it says what is wrong instead of returning a number.

<a id="enveng_overflow_rate_circular"></a>

#### Surface overflow rate of a circular clarifier — `enveng_overflow_rate_circular`

The surface area and the surface overflow rate (surface loading) of a circular settling tank from the flow and the tank diameter.

$$
v_0 = \frac{Q}{\pi D^2/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow rate | m^3/s |
| `D` | in | tank diameter | m |
| `A` | out | surface area | m^2 |
| `v0` | out | overflow rate Q/A | m/s |
| `v0_m_per_day` | out | overflow rate in m^3/(m^2 d) | — |

**Applies when:** Q, D > 0.

```shell
$ environmental-engineering solve enveng_overflow_rate_circular "Q=10000 m^3/day" "D=20 m"
A = 314.1592654 m^2
v0 = 0.0003684142201 m/s
v0_m_per_day = 31.83098862
  by: Surface overflow rate of a circular clarifier  (A = pi D^2/4; v0 = Q/A)
  Q = 0.1157407407 m^3/s
  D = 20 m
  assumes: Q, D > 0
```

In the worked example, Q = 10000 m^3/day, D = 20 m go through the equation above and give A = 314.1592654 m^2, v0 = 0.0003684142201 m/s, v0_m_per_day = 31.83098862. It applies only when Q, D > 0: outside that it says what is wrong instead of returning a number.

<a id="enveng_terminal_settling_velocity"></a>

#### Terminal settling velocity in any flow regime (drag coefficient iteration) — `enveng_terminal_settling_velocity`

The terminal settling velocity of a sphere in a fluid for laminar, transitional or turbulent settling, solving the force balance with the drag law Cd = 24/Re + 3/sqrt(Re) + 0.34.

$$
v_t = \sqrt{\frac{4g d(\rho_p - \rho_f)}{3C_D\rho_f}},\quad C_D = \frac{24}{Re} + \frac{3}{\sqrt{Re}} + 0.34
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | particle diameter | m |
| `rho_p` | in | particle density | kg/m^3 |
| `rho_f` | in | fluid density | kg/m^3 |
| `mu` | in | fluid viscosity | Pa*s |
| `vt` | out | terminal settling velocity | m/s |
| `Re` | out | particle Reynolds number | — |
| `Cd` | out | drag coefficient | — |

**Applies when:** Re <= 2e5; rho_p > rho_f.

```shell
$ environmental-engineering solve enveng_terminal_settling_velocity "d=0.5 mm" "rho_p=2650 kg/m^3" "rho_f=998.2 kg/m^3" "mu=0.001002 Pa*s"
vt = 0.09058056284 m/s
Re = 45.11852187
Cd = 1.318558136
  by: Terminal settling velocity in any flow regime (drag coefficient iteration)  (vt^2 Cd(Re) = (4/3) g d (rho_p - rho_f)/rho_f with Cd = 24/Re + 3/sqrt(Re) + 0.34, solved by bisection below the Stokes velocity)
  d = 0.0005 m
  rho_p = 2650 kg/m^3
  rho_f = 998.2 kg/m^3
  mu = 0.001002 Pa*s
  assumes: Re <= 2e5; rho_p > rho_f
```

In the worked example, d = 0.5 mm, rho_p = 2650 kg/m^3, rho_f = 998.2 kg/m^3, mu = 0.001002 Pa*s go through the equation above and give vt = 0.09058056284 m/s, Re = 45.11852187, Cd = 1.318558136. It applies only when Re <= 2e5; rho_p > rho_f: outside that it says what is wrong instead of returning a number.

<a id="enveng_water_hardness"></a>

#### Water hardness as CaCO3 — `enveng_water_hardness`

Calcium, magnesium and total hardness expressed as mg/L of CaCO3 from the calcium and magnesium concentrations, with the usual classification (soft to very hard).

$$
TH = Ca\,\frac{50.04}{20.04} + Mg\,\frac{50.04}{12.15}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ca` | in | calcium concentration (as Ca) | kg/m^3 |
| `Mg` | in | magnesium concentration (as Mg) | kg/m^3 |
| `total_hardness` | out | total hardness, mg/L as CaCO3 | — |
| `calcium_hardness` | out | calcium hardness, mg/L as CaCO3 | — |
| `magnesium_hardness` | out | magnesium hardness, mg/L as CaCO3 | — |
| `classification` | out | soft (< 60), moderately hard (60-120), hard (120-180), very hard (>= 180) | — |

**Applies when:** only Ca and Mg contribute.

```shell
$ environmental-engineering solve enveng_water_hardness "Ca=60 mg/L" "Mg=15 mg/L"
total_hardness = 211.6074929
calcium_hardness = 149.8381656
magnesium_hardness = 61.7693273
classification = very hard
  by: Water hardness as CaCO3  (hardness = C (EW_CaCO3/EW_ion), EW_CaCO3 = 50.04, EW_Ca = 20.04, EW_Mg = 12.15)
  Ca = 0.06 kg/m^3
  Mg = 0.015 kg/m^3
  assumes: only Ca and Mg contribute
```

In the worked example, Ca = 60 mg/L, Mg = 15 mg/L go through the equation above and give total_hardness = 211.6074929, calcium_hardness = 149.8381656, magnesium_hardness = 61.7693273, classification = very hard. It applies only when only Ca and Mg contribute: outside that it says what is wrong instead of returning a number.

<a id="enveng_weir_loading_rate"></a>

#### Weir loading rate of a circular clarifier — `enveng_weir_loading_rate`

The weir length and weir loading rate of a circular clarifier with a peripheral weir.

$$
WL = \frac{Q}{\pi D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow rate | m^3/s |
| `D` | in | weir (tank) diameter | m |
| `weir_length` | out | weir length pi D | m |
| `WL` | out | weir loading rate Q/(pi D) | m^2/s |
| `WL_m3_per_m_day` | out | weir loading in m^3/(m d) | — |

**Applies when:** peripheral weir.

```shell
$ environmental-engineering solve enveng_weir_loading_rate "Q=10000 m^3/day" "D=20 m"
weir_length = 62.83185307 m
WL = 0.001842071101 m^2/s
WL_m3_per_m_day = 159.1549431
  by: Weir loading rate of a circular clarifier  (WL = Q/(pi D))
  Q = 0.1157407407 m^3/s
  D = 20 m
  assumes: peripheral weir
```

In the worked example, Q = 10000 m^3/day, D = 20 m go through the equation above and give weir_length = 62.83185307 m, WL = 0.001842071101 m^2/s, WL_m3_per_m_day = 159.1549431. It applies only when peripheral weir: outside that it says what is wrong instead of returning a number.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
