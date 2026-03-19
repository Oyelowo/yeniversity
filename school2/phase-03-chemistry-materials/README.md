# Phase 03 — Chemistry & Materials

**Duration:** 2–3 months  
**Prerequisites:** Phase 02 (thermodynamics and electromagnetism are essential here)  
**Runs alongside:** Can begin Phase 04 while finishing this phase

---

## Why This Phase Exists

Every physical device is made of matter. Understanding *why* silicon conducts, why copper is a
better wire than iron, why a lithium battery stores charge, why ceramics are brittle, why steel
fails under fatigue — all of this is chemistry and materials science. This phase provides the
physical basis for all hardware: the transistor, the battery, the sensor, the structural alloy,
the optical fibre. Without it, hardware design is cargo-cult engineering.

---

## Learning Objectives

- [ ] Understand atomic structure: quantum numbers, electron configurations, periodic trends
- [ ] Understand chemical bonding: ionic, covalent, metallic, Van der Waals; bond polarity
- [ ] Understand the laws of thermochemistry; apply Hess's law and Gibbs free energy to reactions
- [ ] Understand phase equilibria: phase diagrams, phase transitions, Clausius-Clapeyron
- [ ] Understand electrochemistry: electrochemical cells, Nernst equation, electrode potentials
- [ ] Understand the structure of solids: crystal lattices, unit cells, Miller indices, X-ray diffraction
- [ ] Understand mechanical properties: stress, strain, Young's modulus, plastic deformation, hardness, fracture
- [ ] Understand electronic band theory: metals, insulators, semiconductors; band gaps
- [ ] Understand intrinsic and extrinsic (doped) semiconductors: n-type, p-type, carrier concentrations
- [ ] Understand the p-n junction: built-in voltage, depletion region, I-V characteristic, diode equation

---

## Topics

### 1. Physical Chemistry

#### Atomic Structure
- Bohr model; limitations; introduction to quantum mechanics of the atom (preview Phase 07)
- Quantum numbers: principal $n$, angular momentum $\ell$, magnetic $m_\ell$, spin $m_s$
- Electron configurations; Aufbau principle, Hund's rules, Pauli exclusion
- Periodic trends: atomic radius, ionisation energy, electronegativity, electron affinity

#### Chemical Bonding
- Ionic bonding: lattice energy, Born-Haber cycle
- Covalent bonding: Lewis structures, VSEPR theory, molecular geometry
- Hybridisation: sp, sp², sp³
- Molecular orbital theory: bonding and antibonding MOs, bond order
- Metallic bonding: free electron model, band theory preview
- Intermolecular forces: Van der Waals, dipole-dipole, hydrogen bonding; effects on properties

#### Thermochemistry
- Internal energy, enthalpy; Hess's law; standard enthalpies of formation
- Gibbs free energy $G = H - TS$; spontaneity criterion $\Delta G < 0$
- Standard free energy change; relationship to equilibrium constant: $\Delta G^\circ = -RT \ln K$
- Temperature dependence of equilibrium: Van't Hoff equation

#### Chemical Kinetics
- Rate laws; rate constants; reaction orders
- Arrhenius equation: activation energy, temperature dependence of rate
- Reaction mechanisms; elementary steps; rate-determining step

#### Phase Equilibria
- Phase diagrams: single-component (P-T); triple point; critical point
- Clausius-Clapeyron equation: vapour pressure vs. temperature
- Binary phase diagrams: eutectic, solid solutions, lever rule

#### Electrochemistry
- Oxidation and reduction; oxidation states; balancing redox reactions
- Electrochemical cells: galvanic and electrolytic
- Standard electrode potentials; EMF from tabulated values
- Nernst equation: cell potential at non-standard conditions $E = E^\circ - \frac{RT}{nF}\ln Q$
- Faraday's laws of electrolysis
- Applications: batteries (Li-ion), fuel cells, corrosion

### 2. Materials Science & Engineering

#### Crystal Structure
- Unit cells: simple cubic, BCC, FCC, HCP
- Miller indices: planes and directions in crystals
- Bragg's law; X-ray diffraction as a probe of crystal structure
- Defects: point defects (vacancies, interstitials, substitutionals), dislocations, grain boundaries

#### Mechanical Properties
- Stress and strain: definitions, engineering vs. true stress/strain
- Young's modulus; shear modulus; Poisson's ratio
- Elastic vs. plastic deformation; yield strength; work hardening
- Fracture: ductile vs. brittle; fracture toughness; fatigue (S-N curves)
- Hardness; creep at elevated temperatures

#### Thermal Properties
- Heat capacity of solids (revisit from Phase 02 statistical mechanics)
- Thermal expansion; thermal conductivity
- Phase transformations in metals: iron-carbon (steel) phase diagram

#### Electronic Properties of Materials
- Free electron model: Drude model for conductivity
- Energy bands from the quantum mechanics of periodic potentials
- Fermi energy; density of states
- Metals: partially filled bands; high conductivity
- Insulators: large band gap; no conduction
- Semiconductors: small band gap; intrinsic conductivity; temperature dependence

### 3. Semiconductor Physics & Devices

#### Intrinsic Semiconductors
- Silicon crystal structure; covalent bonding in semiconductors
- Band gap; intrinsic carrier density $n_i$; mass action law $np = n_i^2$
- Fermi-Dirac statistics applied to semiconductors; Fermi level in intrinsic material
- Carrier transport: drift (mobility, conductivity); diffusion (Einstein relation)
- Continuity equations; minority carrier lifetime

#### Extrinsic (Doped) Semiconductors
- Donor and acceptor impurities; n-type and p-type doping
- Carrier concentrations in doped material; Fermi level shift with doping
- Charge neutrality; complete ionisation approximation

#### p-n Junction
- Formation of the depletion region; built-in potential $V_{bi}$
- Energy band diagram of the junction at equilibrium
- Forward and reverse bias; I-V characteristic; diode equation $I = I_s(e^{V/V_T} - 1)$
- Capacitance of the junction; depletion capacitance; diffusion capacitance
- Breakdown: Zener and avalanche mechanisms

#### Bipolar Junction Transistor (BJT) — Overview
- NPN and PNP structures; three regions of operation
- Common-emitter configuration; current gain $\beta$
- Ebers-Moll model (conceptual)

#### MOSFET — Overview
- Structure: source, drain, gate, body; oxide capacitance
- Strong inversion; threshold voltage $V_T$
- I-V characteristics: linear and saturation regions; MOSFET as a switch and amplifier
- CMOS: complementary n-MOS and p-MOS for logic

---

## Core Texts

| Text | Notes |
|------|-------|
| **Atkins' Physical Chemistry** (11th ed.) | Chapters 1–15 for thermodynamics, kinetics, equilibria, electrochemistry. The standard university physical chemistry text. |
| **Callister & Rethwisch — Materials Science and Engineering: An Introduction** (10th ed.) | Comprehensive on crystal structure, mechanical properties, phase diagrams, electronic materials. Very readable. |
| **Streetman & Banerjee — Solid State Electronic Devices** (7th ed.) | The clearest development of semiconductor physics through to device operation (diode, BJT, MOSFET). |
| **Neamen — Semiconductor Physics and Devices** (4th ed.) | More rigorous quantum treatment of semiconductors; good second reference for Phase 03/07 overlap. |

---

## Supplementary
- Kittel — *Introduction to Solid State Physics* (8th ed.) — deeper quantum mechanical treatment; read with Phase 07
- Newman & Thomas-Alyea — *Electrochemical Systems* — for serious battery/sensor work
- MIT 3.091 OCW (Sadoway) — Introduction to Solid-State Chemistry; excellent free lectures

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Gibbs free energy minimiser | Find equilibrium composition for a reaction at varying T and P |
| Phase diagram plotter | Iron-carbon phase diagram; lever rule calculator |
| Nernst equation solver | Compute cell EMF vs. concentration and temperature |
| Semiconductor carrier density | $n(T)$ and $p(T)$ for intrinsic and doped Si as a function of temperature |
| p-n junction I-V simulator | Plot diode I-V characteristic; explore effect of temperature and doping |
| Crystal structure visualiser | Render BCC, FCC, HCP unit cells; compute packing fractions |
| Battery discharge model | Simple Li-ion discharge curve: voltage vs. state of charge |

---

## Completion Criteria

You are ready for Phase 04 when you can:
1. Explain why silicon is a semiconductor and copper is a metal in terms of band theory
2. Derive the diode equation qualitatively from minority carrier injection across the p-n junction
3. Apply the Nernst equation to compute the EMF of a cell at non-standard conditions
4. Read an iron-carbon phase diagram and determine phases present at a given composition and temperature
5. Distinguish elastic from plastic deformation; explain fracture in terms of defect propagation
