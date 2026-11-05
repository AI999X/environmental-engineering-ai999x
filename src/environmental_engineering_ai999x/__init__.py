"""environmental-engineering-ai999x — offline environmental engineering answers with their formula, steps and verification.

    import environmental_engineering_ai999x as m
    m.ask("Streeter-Phelps oxygen deficit: kd = 0.23 1/d, ka = 0.46 1/d, L0 = 10 mg/L, D0 = 1 mg/L, t = 2 d, DOsat = 9.1 mg/L").show()
    m.solve("enveng_gaussian_plume_concentration", Q=0.1, u=4.0, sigma_y=60.0, sigma_z=30.0, H=50.0, y=20.0, z=0.0)
    m.capabilities()                   # every certified capability

99 certified capabilities: air pollution (Pasquill stability classes, Briggs dispersion coefficients, the Gaussian plume
with ground and inversion-lid reflections, maximum ground-level concentrations, Briggs and Holland plume rise, ppm and
mass concentration, the box model, settling chambers, cyclones, electrostatic precipitators); water quality (BOD
kinetics and fits, theoretical oxygen demand, the Streeter-Phelps oxygen sag and its critical point, reaeration, oxygen
saturation, mixing, lakes, rivers, pollutant loads); water treatment (detention time, overflow and weir rates, Stokes and
general settling, filtration and Ergun head loss, backwash, velocity gradient, disinfection CT, Chick and Chick-Watson,
chlorine dosing, hardness, adsorption isotherms, membranes); activated sludge (F/M, SRT, Monod, Lawrence-McCarty design,
sludge production, oxygen demand, SVI, methane, trickling filters, oxygen transfer, KLa); Henry's law and air stripping;
noise (decibel arithmetic, Leq, Ldn/Lden, noise dose); solid waste (generation, landfill life and gas, heating value).
Questions use one documented form, "<capability phrase>: name = value, ..." with units; a question that is not
determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.1.0"
_P = Product("environmental_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
