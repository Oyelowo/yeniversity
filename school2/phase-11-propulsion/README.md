# Phase 11 — Propulsion

**Duration:** 2–3 months  
**Prerequisites:** Phase 02 (thermodynamics), Phase 09 (compressible flow), Phase 03 (chemistry for combustion)  
**Can start:** After Phase 09

---

## Why This Phase Exists

Propulsion is the discipline that makes vehicles move. Every rocket that reaches orbit, every
jet aircraft that crosses an ocean, every racing car engine that produces 1000 hp — these are
applications of propulsion engineering. This phase covers the thermodynamic cycles underlying
all propulsion systems, the design of nozzles, the chemistry and gasdynamics of combustion, and
the end-to-end design of rocket and jet engines from first principles. After this phase you can
design a solid or liquid rocket, size a converging-diverging nozzle, and understand every number
on any engine specification sheet.

---

## Learning Objectives

- [ ] Apply the first and second laws of thermodynamics to open and closed propulsion cycles
- [ ] Derive and apply the rocket equation (Tsiolkovsky) from Newton's third law
- [ ] Design a converging-diverging (de Laval) nozzle for given thrust and area ratio from first principles
- [ ] Understand solid and liquid propellant chemistry; compute specific impulse from thermochemistry
- [ ] Derive the turbojet, turbofan, and ramjet ideal cycle analysis (Brayton cycle)
- [ ] Understand combustion instabilities and cooling requirements for liquid rocket engines
- [ ] Apply the fundamentals of electric propulsion: ion thrusters, Hall thrusters, cold gas thrusters
- [ ] Estimate mission $\Delta v$ for a given trajectory; use multi-staging to achieve target performance

---

## Topics

### 1. Thermodynamics Review for Propulsion

- First law applied to open systems (steady-state flow devices): $\dot{Q} - \dot{W}_s = \dot{m}(h_{out} - h_{in} + \Delta KE + \Delta PE)$
- Stagnation (total) enthalpy $h_0 = h + V^2/2$; stagnation temperature and pressure
- Isentropic relations; entropy generation in real processes
- Ideal gas assumptions; specific heat ratio $\gamma$; caloric equation of state

### 2. Basic Rocket Propulsion

#### The Rocket Equation
- Derivation from conservation of momentum for variable-mass system
- Tsiolkovsky rocket equation: $\Delta v = v_e \ln(m_0/m_f)$
- Specific impulse $I_{sp} = F / (\dot{m} g_0)$; relation to exhaust velocity $v_e = g_0 I_{sp}$
- Multi-staging: performance advantage; staging equation; optimal staging

#### Nozzle Theory
- Converging-diverging nozzle: isentropic flow relations; area-Mach relation
- Choked throat: $M = 1$ at minimum area; mass flow rate $\dot{m} = p_0 A^* \sqrt{\gamma/RT_0} \cdot f(\gamma)$
- Nozzle thrust equation: $F = \dot{m}v_e + (p_e - p_a)A_e$
- Optimum expansion: $p_e = p_a$; thrust coefficient $C_F$; characteristic velocity $c^*$
- Nozzle design: convergent geometry; divergent profile (conical vs. bell/parabolic)
- Off-design nozzle operation: over-expanded (oblique shocks in nozzle), under-expanded (expansion fans)

#### Propellant Combinations
- Solid propellants: oxidiser + fuel + binder; $I_{sp}$ values; burn rate law ($r = a p^n$)
- Liquid propellants: monopropellant (hydrazine), bipropellant (LOX/LH2, LOX/RP-1, N2O4/UDMH)
- Cryogenic vs. storable propellants; trade-offs
- Oxidiser-to-fuel ratio; mixture ratio optimisation for maximum $I_{sp}$
- Theoretical $I_{sp}$ from thermochemical combustion products: CEA (Chemical Equilibrium and Applications) concepts

#### Thrust Chamber
- Combustion chamber design: L* criterion; characteristic length for complete combustion
- Chamber pressure and temperature; equilibrium combustion products
- Heat transfer in the thrust chamber: regenerative cooling, film cooling, radiation cooling
- Combustion instability: chugging (low frequency), screaming (high frequency); stabilisation

### 3. Liquid Rocket Engines in Detail

#### Engine Cycles
- Pressure-fed cycle: simplest; limited by tank pressure; trade-off with pump mass
- Gas-generator cycle: turbine exhaust dumped overboard; SpaceX Merlin
- Staged combustion cycle: full-flow vs. oxidiser-rich vs. fuel-rich preburners; highest efficiency; SpaceX Raptor
- Expander cycle: fuel heated by chamber wall drives turbopump

#### Turbopumps
- Centrifugal pump performance: head, flow, efficiency; pump curve
- Turbine power matching pump requirements
- Cavitation: net positive suction head (NPSH); cavitation number; inducers
- NPSH requirements at inlet; feed system pressures

#### Valve and Feed System
- Propellant feed: tanks → valves → pumps → injector → chamber
- Injection elements: impinging doublet/triplet, coaxial shear, swirl; atomisation and mixing
- Injector design for uniform combustion and stability

### 4. Solid Rocket Motors

- Grain design: end-burning, core-burning (star, finocyl, wagon wheel); neutral vs. progressive vs. regressive thrust
- Burn rate: $r = a p^n$ (Saint-Robert's law); pressure exponent $n$; sensitivity
- Ballistic prediction: internal ballistics equations; $K_n$ (Klemens factor)
- Case design: liner, insulation, nozzle throat erosion
- Hybrid rockets: GOX + HTPB; regression rate uncertainties

### 5. Air-Breathing Propulsion

#### Brayton Cycle (Ideal Turbojet)
- Cycle description: intake compression → combustion → turbine expansion → exhaust
- Thermal efficiency: $\eta_{th} = 1 - T_1/T_3$
- Specific thrust and thrust specific fuel consumption (TSFC)
- Effect of turbine inlet temperature (TIT) and compressor pressure ratio on performance
- Component efficiencies: isentropic efficiency of compressor and turbine

#### Turbojet Engine Components
- Axial compressor: velocity triangles; work input; polytropic efficiency; stage loading
- Annular combustor: stoichiometry; flame stabilisation; emissions
- Axial turbine: velocity triangles; work output; coupling with compressor
- Afterburner (reheat): additional combustion between turbine and nozzle; thrust augmentation

#### Turbofan, Turboprop, Ramjet, Scramjet
- Turbofan: bypass ratio; separation of core and fan streams; fan nozzle; civil vs. military fans
- Turboprop: power extraction vs. jet thrust; propulsive efficiency advantage at low speeds
- Ramjet: no rotating parts; requires supersonic intake; minimum Mach ~2; vehicle integration
- Scramjet: supersonic combustion; hypersonic applications (Mach 5+); enormous engineering challenges

### 6. Electric & Alternative Propulsion

#### Electric Propulsion
- Hall-effect thruster: principle; $I_{sp}$ range 1000–3000 s; low thrust but high efficiency for long missions
- Ion thruster (gridded ion engine): electrostatic acceleration; Dawn mission
- Cold gas thruster: inert gas expansion; low $I_{sp}$; simplicity for attitude control
- Arcjet and resistojet: thermal heating of propellant; intermediate $I_{sp}$

#### Mission Analysis
- Orbital mechanics review: Hohmann transfer; $\Delta v$ budget
- Typical $\Delta v$ values: LEO ($\approx$ 9.4 km/s from Earth surface), GTO ($\approx$ 4.1 km/s from LEO), TLI
- Multi-stage vehicle sizing: payload fraction as function of stage $I_{sp}$ and mass ratio
- Launch vehicle design: first-stage recovery; reusability $\Delta v$ penalty

---

## Core Texts

| Text | Notes |
|------|-------|
| **Sutton & Biblarz — Rocket Propulsion Elements** (9th ed.) | The definitive rocket propulsion textbook. Every engineer working on rockets uses this. |
| **Mattingly — Elements of Gas Turbine Propulsion** (2nd ed.) | Complete treatment of jet engine thermodynamics, cycle analysis, and component design. |
| **Anderson — Modern Compressible Flow** (3rd ed.) | Rigorous compressible gasdynamics needed for nozzle, shock, and supersonic flow design. |
| **Humble, Henry & Larson — Space Propulsion Analysis and Design** | Systems-level propulsion: integrates propulsion into mission/vehicle design. |

---

## Supplementary
- Heister et al. — *Rocket Propulsion* — modern, strong on liquid engine systems
- NASA SP-125 — *Liquid Rocket Engine Injectors* — detailed free reference
- NASA CEA (Chemical Equilibrium with Applications) — online tool for combustion chemistry
- MIT 16.522 *Space Propulsion* (OCW) — lecture notes and problem sets

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Rocket trajectory simulator | Integrate rocket equation with gravity + drag; 2D/3D trajectory |
| Nozzle designer | Given chamber conditions and $p_a$, compute $A^*$, $A_e/A^*$, $F$, $c^*$, $C_F$ |
| Solid rocket internal ballistics | Given grain geometry and $a, n$; compute thrust vs. time |
| Brayton cycle analyser | Turbojet ideal cycle; thrust and TSFC vs. compressor pressure ratio and TIT |
| Multi-stage vehicle sizer | Compute payload fraction for given $\Delta v$, $I_{sp}$, and structural margins |
| CEA-like combustion calculator | Adiabatic flame temperature and product composition for a propellant pair |

---

## Completion Criteria

You are ready to move on when you can:
1. Derive the Tsiolkovsky rocket equation from conservation of momentum
2. Design a nozzle (compute throat area, exit area, and exit velocity) for a given thrust, chamber pressure, and ambient pressure
3. Compute $I_{sp}$ from the exhaust velocity and explain its physical meaning
4. Describe the Brayton cycle and compute turbojet thermal efficiency for given temperatures
5. Choose between pressure-fed, gas-generator, and staged combustion cycles for a given application
