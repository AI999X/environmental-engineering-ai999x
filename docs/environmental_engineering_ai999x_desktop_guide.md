# Environmental Engineering: the desktop app, user guide

**Environmental Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Environmental Engineering covers Gaussian plume dispersion and plume rise, BOD and Streeter-Phelps oxygen sag, water and wastewater treatment design, activated sludge, disinfection CT, Henry's law and air stripping, noise and solid waste. Its desktop app holds all 132 calculations in 14 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-environmental-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-environmental-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-environmental-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-environmental-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-environmental-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-environmental-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-environmental-engineering-1.6.5-linux-x86_64/ai999x-environmental-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Environmental Engineering app, opened on Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time with its worked example.](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/hero.webp)

*The Environmental Engineering app, opened on Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Water quality › Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time (or type part of its name in the search box). It opens with a worked example filled in: “Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L” The answer: D = 0.002726165085 kg/m^3, DO = 0.006373834915 kg/m^3, D_mgL = 2.726165085, DO_mgL = 6.373834915.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-1.webp)

*Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time
D = 0.002726165085 kg/m^3
DO = 0.006373834915 kg/m^3
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Briggs plume rise and effective stack height (neutral or unstable air)](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-2.webp)

*Briggs plume rise and effective stack height (neutral or unstable air)*

![Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-3.webp)

*Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty)*

![Adding sound levels in decibels](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-4.webp)

*Adding sound levels in decibels*

![Volatile solids reduction in digestion (Van Kleeck)](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-5.webp)

*Volatile solids reduction in digestion (Van Kleeck)*

![Population projection by the logistic method (three censuses)](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-6.webp)

*Population projection by the logistic method (three censuses)*

![Packed-tower air stripping: stripping factor, transfer units and height](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-7.webp)

*Packed-tower air stripping: stripping factor, transfer units and height*

![Darcy's law: groundwater flow and seepage velocity](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/scene-8.webp)

*Darcy's law: groundwater flow and seepage velocity*

![Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time: a chart, enlarged.](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/chart-1.webp)

*Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time: a chart, enlarged.*

![Briggs plume rise and effective stack height (neutral or unstable air): a chart, enlarged.](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/chart-2.webp)

*Briggs plume rise and effective stack height (neutral or unstable air): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Environmental Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=environmental-engineering-ai999x/hero-dark.webp)

*The Environmental Engineering app in the dark theme.*

## 7. Every calculation

132 calculations in 14 topics; each has its own entry on the model's page (https://www.ai999x.com/models/environmental-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Air pollution | Atmospheric stability from the environmental lapse rate, Box model of an air basin (well-mixed, time-dependent), Briggs dispersion coefficients sigma_y and sigma_z, Briggs plume rise and effective stack height (neutral or unstable air), Briggs plume rise in stable air, Cyclone cut diameter and fractional efficiency (Lapple), Electrostatic precipitator efficiency (Deutsch-Anderson), Electrostatic precipitator plate area for a target efficiency, Gas concentration: mass concentration to ppm (by volume), Gas concentration: ppm (by volume) to mass concentration, Gaussian plume concentration at a receptor (ground reflection), Gaussian plume concentration from the stability class, Gaussian plume trapped below an inversion lid (multiple reflections), Gravity settling chamber collection efficiency, Ground-level centreline concentration of a plume, Holland plume rise, Maximum ground-level concentration (constant sigma_y/sigma_z), Maximum ground-level concentration and its distance (Briggs coefficients), Overall efficiency of collection devices in series, Pasquill-Turner atmospheric stability class, Stack plume: effective height, dispersion and ground-level concentration, Steady indoor air concentration (well-mixed room box model), Stoichiometric oxygen for complete combustion of a fuel, Sulfur dioxide emission from fuel sulfur, Wind speed at height (power law) |
| Biosolids | Volatile solids reduction in digestion (Van Kleeck) |
| Fundamentals | Population projection by the arithmetic and geometric methods, Population projection by the logistic method (three censuses), Water and wastewater demand with peaking factors |
| Gas transfer | Dissolved concentration in equilibrium with a gas (Henry's law), Dissolved gas concentration from Henry's solubility constant (C = KH P), Henry's law constant conversion between forms, Overall liquid-phase transfer coefficient and flux (two-film theory), Packed-tower air stripping: stripping factor, transfer units and height, Temperature dependence of Henry's constant (van 't Hoff) |
| Groundwater | Darcy's law: groundwater flow and seepage velocity, Retardation factor and contaminant velocity in groundwater |
| Hydraulics | Hazen-Williams equation for a full pipe (SI): velocity, flow and head loss, Manning's equation: velocity and flow in a channel or a full pipe (SI), Pump power from flow and head, with energy and electricity cost |
| Hydrology | Rational method: peak runoff Q = C i A, Return period, exceedance probability and risk over a design life |
| Noise | Adding sound levels in decibels, Day-night (Ldn) and day-evening-night (Lden) sound levels, Equivalent continuous sound level Leq, Occupational noise dose and 8-hour TWA (OSHA, 5 dB exchange), Sound level at another distance (point or line source), Sound pressure level from sound power level, Source level after removing background noise |
| Reactors | First-order decay in a batch reactor: C = C0 exp(-k t) and the half-life, First-order reaction in N stirred tanks in series and in plug flow |
| Risk assessment | Chronic daily intake, hazard quotient and cancer risk |
| Solid waste | Compaction ratio of solid waste, Heating value of waste from its elemental composition (modified Dulong), Heating value on a dry basis from the as-received value and the moisture, Landfill life with daily cover and waste growth, Landfill methane generation (first-order decay, constant acceptance), Landfill volume per day and per year with cover soil, and the landfill life, Moisture content of mixed solid waste, Municipal solid waste generation and collection trucks |
| Wastewater | Activated sludge (complete mix) effluent substrate and biomass (Lawrence-McCarty), Dissolved oxygen during aeration and the transfer rate, Field (actual) oxygen transfer rate of an aeration system, Food-to-microorganism ratio (F/M), Hydraulic retention time and volumetric organic loading, KLa fitted to a reaeration test, Mean cell residence time (sludge age) from the solids masses, Methane production from COD removed (anaerobic digestion), Minimum (washout) solids retention time, Monod specific growth rate, Overall oxygen transfer coefficient KLa from two DO readings, Oxygen requirement of an activated-sludge process, Return activated sludge ratio, Sludge volume after thickening or dewatering, Sludge volume index (SVI), Solids retention time (mean cell residence time), Trickling filter BOD removal efficiency (NRC equation), Volume of wet sludge from dry solids, Waste biomass production (observed yield) |
| Water quality | BOD exerted after a time (first-order), BOD from a dilution test (with optional seed correction), BOD rate constant and ultimate BOD fitted to BOD data, Concentration after mixing a discharge with a river, Concentration in a well-mixed lake over time, Critical (minimum) dissolved oxygen of the oxygen sag, Dissolved oxygen at a distance downstream (Streeter-Phelps), Dissolved oxygen saturation concentration (Benson-Krause), First-order decay of a pollutant downstream (plug flow), Flow-weighted mixing of several streams, Geometric mean of bacterial counts (with mean and standard deviation), pH and pOH from the hydrogen or hydroxide ion concentration, Pollutant mass load of a flow, Steady-state concentration in a well-mixed lake (first-order decay), Stream reaeration coefficient (O'Connor-Dobbins, Churchill, Owens-Gibbs), Streeter-Phelps oxygen deficit and dissolved oxygen at a travel time, Temperature correction of a rate constant (theta model), Theoretical oxygen demand of a solution (mg/L), Theoretical oxygen demand of an organic compound, Total alkalinity as CaCO3 from the carbonate species, Ultimate BOD from a BOD test |
| Water treatment | Activated carbon dose from the Freundlich isotherm, Adsorbent dose from the Langmuir isotherm, Chemical feed rate: mass per day = Q x dose / purity (kg/d and lb/d), Chlorine dose and feed rate, Clean-bed head loss through a granular filter (Ergun / Carman-Kozeny), Disinfection by the Chick-Watson law, Disinfection CT value of a contact basin, Filter bed expansion during backwash, Filtration (hydraulic loading) rate with one filter out of service, Flow through a filter from its loading rate and bed area, Flow velocity in a rectangular channel (continuity, grit chamber), Fraction of discrete particles removed in an ideal settling basin, Fraction of free chlorine as hypochlorous acid (pH), Freundlich isotherm constants fitted to adsorption data, Hydraulic detention time, Langmuir isotherm constants fitted to adsorption data, Log inactivation from the CT ratio, Log removal and percentage removal, Mean velocity gradient G (rapid mixing and flocculation), Membrane permeate flux J = Qp/A (LMH and m/d), or the area for a flux, Membrane recovery, rejection, flux and concentrate, Microbial inactivation by Chick's law (first order), Minimum disinfection contact time from a required CT, Osmotic pressure of a dilute solution (van 't Hoff), Power needed for a target velocity gradient, Sedimentation basin sizing from overflow rate and detention time, Stokes settling velocity of a discrete particle, Surface overflow rate of a circular clarifier, Terminal settling velocity in any flow regime (drag coefficient iteration), Transmembrane pressure of a crossflow membrane, Water hardness as CaCO3, Weir loading rate of a circular clarifier |

## 8. Other ways to use it

The same engine runs on the command line (`environmental-engineering`, its own guide), in Python (`pip install environmental-engineering-ai999x`, `import environmental_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/environmental-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/environmental-engineering-ai999x/
