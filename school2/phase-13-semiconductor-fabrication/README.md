# Phase 13 — Semiconductor Fabrication

**Duration:** 2–3 months  
**Prerequisites:** Phase 03 (semiconductor physics, materials science), Phase 07 (quantum mechanics for deep understanding)  
**Feeds into:** Phase 14 (VLSI design)

---

## Why This Phase Exists

Understanding fabrication is what separates an engineer who uses chips from one who can *build*
them. This phase covers the full CMOS process from blank silicon wafer to finished transistors:
oxidation, photolithography, doping, etching, deposition, metallisation. You will understand
every physical reason behind every process step — why thermal oxide grows, how a contact mask
defines geometry, what happens during ion implantation. Combined with Phase 07 quantum mechanics
and Phase 03 materials science, this gives you the ability to design process flows, understand
Sam Zeloof's garage IC work, and eventually tape out real chips through programmes like
Google's OpenMPW.

---

## Learning Objectives

- [ ] Describe the full CMOS process flow: wafer preparation through passivation and packaging
- [ ] Understand thermal oxidation of silicon: Deal-Grove model; oxide growth kinetics; oxide properties
- [ ] Understand photolithography: resolution limits, mask alignment, exposure, resist chemistry
- [ ] Understand doping: ion implantation (dose, energy, range) and diffusion (Fick's laws)
- [ ] Understand etching: wet (isotropic) and dry (plasma, RIE) — selectivity and anisotropy
- [ ] Understand thin-film deposition: PVD (sputtering, evaporation), CVD, ALD; step coverage
- [ ] Understand CMP (chemical mechanical planarisation) for multilevel metal
- [ ] Read and understand a process design kit (PDK) and design rule check (DRC) file
- [ ] Simulate a complete simple process flow for a NMOS transistor on paper and in simulation

---

## Topics

### 1. Silicon Wafer Technology

#### Crystal Growth and Wafer Preparation
- Czochralski crystal growth: pulling single-crystal silicon ingots; oxygen and crystal defects
- Float-zone (FZ) silicon: ultra-high purity; RF heating; ingot refinement
- Wafer slicing, lapping, etching, and polishing (CMP)
- Wafer specifications: diameter (100mm → 300mm → 450mm); orientation (100), (110), (111); resistivity; doping type
- Epitaxial silicon growth: CVD epitaxy; buried layers for bipolar and high-voltage CMOS

#### Cleanroom Environment
- Particle contamination: particle size vs. feature size; Moore's law sensitivity
- Cleanroom classifications (ISO 1–9); laminar flow; gowning protocols
- Chemical handling: acids (HF, H2SO4, HNO3), oxidisers, developer, solvent MSDS
- Contamination sources: metallic, organic, particle; cleaning sequences (RCA clean: SC-1 and SC-2)

### 2. Thermal Oxidation

#### Silicon Dioxide (SiO₂) Growth
- Reaction: Si + O₂ → SiO₂ (dry oxidation); Si + 2H₂O → SiO₂ + 2H₂ (wet oxidation)
- Volume expansion: 1 μm oxide consumes 0.45 μm of Si
- Oxide structure: amorphous; interface trap density; fixed oxide charge; mobile ionic charge

#### Deal-Grove Model
- Linear-parabolic growth kinetics: $x_{ox}^2 + A x_{ox} = B(t + \tau)$
- Linear rate constant $B/A$: surface reaction limited
- Parabolic rate constant $B$: diffusion limited
- Effect of temperature, pressure, steam vs. dry O₂, crystal orientation
- Thin oxide regime: initial rapid growth (below linear-parabolic model)

#### Oxide Properties and Applications
- SiO₂ as gate dielectric: oxide capacitance $C_{ox} = \varepsilon_{ox}/t_{ox}$; threshold voltage control
- Field oxide (LOCOS and STI); isolation between transistors
- Oxide as diffusion mask; passivation layer
- High-k dielectrics (HfO₂, Al₂O₃): why needed at sub-45nm; equivalent oxide thickness (EOT)

### 3. Photolithography

#### Optical Lithography Principles
- Process sequence: spin coat photoresist → soft bake → align and expose → post-exposure bake → develop → hard bake → etch or implant → strip
- Positive vs. negative photoresist: which areas dissolve; tone reversal
- Resolution limit: Rayleigh criterion applied to lithography $R = k_1 \lambda / NA$
- Depth of focus $DOF = k_2 \lambda / NA^2$; the exposure-DOF tradeoff
- Numerical aperture of the lens; immersion lithography (NA > 1 with water)

#### Masks (Reticles)
- Chrome-on-glass photomask; quartz for deep UV; pellicle protection
- 1× (contact/proximity), 4× stepper, 4× scanner configurations
- Mask defect inspection and repair; mask cost at advanced nodes

#### Resist Processing
- Spin coating: film thickness vs. spin speed and viscosity
- Chemical amplified resist (CAR): photoacid generator; diffusion-limited resolution
- Resolution enhancement techniques (RETs): off-axis illumination, phase-shift masks, OPC (optical proximity correction)
- E-beam direct write: no mask; slow; used for mask writing and research (<10 nm)
- EUV lithography (13.5 nm): current leading-edge; plasma source; multilayer optics

### 4. Doping

#### Ion Implantation
- Implant process: ion source → acceleration → mass spectrometer → scanning → wafer
- LSS theory: projected range $R_p$ and straggle $\Delta R_p$ (Gaussian approximation)
- Channelling: crystal direction sensitivity; tilt implants
- Dose and energy: typical values for threshold voltage adjustment, source/drain formation
- Post-implant damage: amorphisation of silicon; must anneal to activate dopants

#### Diffusion
- Fick's first and second laws of diffusion
- Solution to diffusion equation for implanted and deposited sources
- Drive-in diffusion: spreading the doping profile
- Thermal budget: temperature × time determines diffusional spread; must limit for shallow junctions
- Diffusivity vs. temperature: Arrhenius behaviour; activation energy for different dopants (B, P, As)
- Junction depth: $n$-$p$ junction formation; $x_j$ from dopant profiles

#### Well Formation in CMOS
- Twin-well CMOS: n-well and p-well formation; retrograde wells by deep implants
- Threshold voltage adjustment implant; anti-punch-through implant; halo implants

### 5. Etching

#### Wet Etching
- Isotropic etch: HF for SiO₂ (slow, controlled); HF: HNO₃: CH₃COOH for silicon
- Anisotropic wet etch: KOH for bulk micromachining; orientation-dependent etch rate
- Selectivity: etch rate of target material vs. mask material; design selectivity into process
- Lag and loading effects; micro-loading

#### Dry Etching (Plasma)
- Plasma generation: RF capacitively coupled plasma; inductively coupled plasma (ICP)
- Ion bombardment: energetic ions provide directional (anisotropic) etching
- Chemical component: radicals react with silicon to form volatile products
- Reactive ion etching (RIE): combination of physical sputtering + chemical etching
- Deep RIE (Bosch process): alternating etch / passivation cycles; high aspect ratio structures for MEMS
- Etch endpoint detection: optical emission spectroscopy (OES); laser interferometry

### 6. Thin Film Deposition

#### Physical Vapour Deposition (PVD)
- Evaporation: resistive, e-beam; line-of-sight; poor step coverage
- Sputtering (DC and RF): momentum transfer; better step coverage; metal and dielectric targets
- Applications: metal contacts (Al, W, TiN barriers), reflective coatings

#### Chemical Vapour Deposition (CVD)
- Process types: APCVD, LPCVD, PECVD; temperature ranges
- Reaction mechanisms: surface reaction vs. mass-transport limited; step coverage
- Key films: poly-Si (LPCVD, 620°C); SiO₂ (TEOS-based PECVD for ILD); Si₃N₄; W CVD for plugs
- Conformal deposition; void formation in high-aspect-ratio gaps; gap fill techniques

#### Atomic Layer Deposition (ALD)
- Self-limiting surface reactions: alternate precursor and oxidant pulses
- Angstrom-level thickness control; perfect conformality
- Applications: high-k dielectrics (HfO₂), diffusion barriers (TaN), seed layers

### 7. Multilevel Metallisation and Back-End-of-Line (BEOL)

- Contacts and vias: requirements for low-resistance silicide (TiSi₂, CoSi₂, NiSi) on source/drain
- Tungsten (W) plug fill for contacts; barrier metal TiN
- Copper damascene process: CMP-based patterning; dual damascene for via + metal in one step
- Low-k dielectric interlayer: SiCOH; porous low-k; RC delay in interconnect
- CMP: slurry chemistry; planarisation; polish uniformity; dishing and erosion
- Passivation: final SiN or SiO₂ layer; bond pad opening

### 8. CMOS Process Integration: The Full Flow

- NMOS-only process (simplified): starting wafer → isolation (STI) → gate oxidation → poly deposition → gate etch → source/drain implant → silicidation → ILD deposition → contact etch/fill → metal → passivation → pad open
- CMOS addition: twin-well formation before gate processing; NMOS + PMOS transistors; LDD (lightly doped drain) spacers
- Process design kit (PDK): layer definitions; design rules; SPICE models; layer stack cross-section
- Design rule check (DRC): minimum width, spacing, overlap, enclosure rules and their physical basis
- Layout vs. schematic (LVS): verifying drawn layout matches intended circuit
- The open-source SkyWater 130nm PDK: examining real layer definitions and design rules

### 9. MEMS and Non-Standard Fabrication (Overview)

- Bulk micromachining: anisotropic wet etch of silicon; membranes, cantilevers, trenches
- Surface micromachining: sacrificial layers; polysilicon structural layers
- Applications: accelerometers, gyroscopes, pressure sensors, microphones, micromirrors (DMD)
- Garage fab approach (Sam Zeloof method): UV photolithography at 5–10 μm; tube furnace oxidation; metal evaporation

---

## Core Texts

| Text | Notes |
|------|-------|
| **Jaeger — Introduction to Microelectronic Fabrication** (2nd ed., Modular Series Vol. V) | Clear, concise sequence through every major process step. The best starting point for fab. |
| **Neamen — Semiconductor Physics and Devices** (4th ed.) | Connects device physics to fabrication; complements Phase 03 semiconductor material. |
| **Madou — Fundamentals of Microfabrication and Nanotechnology** (3rd ed.) | Broader than Jaeger; covers MEMS; strong on photolithography and etching. |
| **Wolf & Tauber — Silicon Processing for the VLSI Era** (Vols 1–3) | The encyclopaedic reference. Look up any process detail here. |

---

## Supplementary
- Sam Zeloof blog and YouTube channel — practical garage fab; essential watching/reading
- Quirk & Serda — *Semiconductor Manufacturing Technology* — accessible industry perspective
- *The Art of MEMS* — Chang Liu — MEMS design and fabrication for sensors and actuators
- SkyWater 130nm PDK documentation (GitHub: google/skywater-pdk) — study real design rules

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Deal-Grove oxidation model | Compute $x_{ox}(t)$ for given temperature; fit to measured data |
| Implant profile calculator | Gaussian approximation; range and straggle tables; dose integration |
| Diffusion profile simulator | Solve Fick's second law numerically; show drive-in spreading |
| Junction depth estimator | Combine implant + anneal profiles; find $x_j$ from intersection with background |
| Etch rate calculator | Model isotropic and anisotropic etch profiles; simple 2D cross-section |
| Process flow document | Write a complete NMOS process flow with target dimensions and parameters |

---

## Physical Lab Goals (If Setting Up a Home Lab)

| Equipment | Purpose | Cost Estimate |
|-----------|---------|---------------|
| Tube furnace (1100°C, 2.5 cm ID) | Oxidation, diffusion, anneal | $300–$2000 used |
| Spin coater (DIY or commercial) | Photoresist coating | $100–$500 |
| UV light source (365 nm, collimated) | Photolithography exposure | $50–$200 |
| Chrome-on-glass or laser-film mask | Lithography pattern | $20–$200 |
| Probe station | Measuring IV characteristics | $200–$1000 used |
| Vacuum evaporator | Metal contact deposition | $500–$3000 used |
| Chemical wet bench | HF, developer, cleaning | $200–$1000 |
| Optical microscope (100×+) | Inspection and alignment | $200–$1000 used |

---

## Completion Criteria

You are ready for Phase 14 when you can:
1. Describe every step in a CMOS process flow and explain the physical reason for each step
2. Use the Deal-Grove model to predict how thick an oxide will grow in a given time at a given temperature
3. Explain why ion implantation is preferred over diffusion for modern shallow-junction formation
4. Compute the etch selectivity requirement given a mask material and target etch depth
5. Read a PDK layer stack cross-section and identify where each process step's results appear
