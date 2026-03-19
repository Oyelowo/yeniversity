# Phase 02 — Classical Physics

**Duration:** 4–6 months  
**Prerequisites:** Phase 01 (especially calculus, ODEs, and linear algebra)  
**Read throughout:** Feynman Lectures on Physics (all 3 volumes) — free at feynmanlectures.caltech.edu

---

## Why This Phase Exists

Classical physics is the deterministic engine of the macroscopic world. Every machine you will ever
build — a motor, a rocket, a circuit, a robot arm — obeys these laws. Electromagnetism is the
foundation of every electronic device. Thermodynamics governs every energy conversion. You cannot
design anything physical without this phase as bedrock. The Lagrangian/Hamiltonian formulation of
mechanics you learn here also reappears in quantum mechanics and control theory (Pontryagin).

---

## Learning Objectives

- [ ] Apply Newton's laws to any mechanical system; analyse motion in non-inertial frames
- [ ] Derive and use the Lagrangian and Hamiltonian formulations of mechanics from the principle of least action
- [ ] Apply conservation laws: energy, momentum, angular momentum — and know when they hold
- [ ] Analyse rigid-body rotation: inertia tensor, Euler angles, Euler's equations
- [ ] Derive Maxwell's equations from physical laws (Coulomb, Biot-Savart, Faraday); understand their meaning
- [ ] Solve electrostatic and magnetostatic problems using potentials and Gauss's law
- [ ] Understand electromagnetic waves: derivation from Maxwell's equations, solutions in vacuum and media
- [ ] Apply the first and second laws of thermodynamics to cycles and processes
- [ ] Understand entropy, free energy, and the statistical interpretation of thermodynamics
- [ ] Understand wave motion: superposition, standing waves, dispersion, wave packets

---

## Topics

### 1. Classical Mechanics

#### Newtonian Mechanics
- Newton's three laws; inertial reference frames; Galilean relativity
- Kinematics in 1D, 2D, 3D; projectile motion; circular motion
- Work, kinetic energy, potential energy, conservation of mechanical energy
- Conservative forces and potential energy functions
- Momentum; impulse; conservation of momentum; centre of mass
- Collisions: elastic and inelastic
- Angular momentum; torque; conservation of angular momentum
- Rotation of rigid bodies: moment of inertia, parallel axis theorem
- Non-inertial frames: fictitious forces, Coriolis effect, centrifugal force

#### Lagrangian Mechanics
- Generalised coordinates; degrees of freedom; constraints (holonomic)
- Virtual work and d'Alembert's principle
- Lagrangian $L = T - V$; Euler-Lagrange equations
- Symmetry and Noether's theorem: conservation laws from invariances
- Lagrangians for pendula, coupled oscillators, particles in fields

#### Hamiltonian Mechanics
- Legendre transform from Lagrangian to Hamiltonian $H = T + V$
- Hamilton's equations of motion
- Phase space; Liouville's theorem
- Poisson brackets; connection to quantum mechanics commutators
- Hamilton-Jacobi equation (overview)

#### Oscillations
- Simple harmonic oscillator: natural frequency, phase, analytic solution
- Damped oscillator: underdamped, overdamped, critically damped regimes
- Driven oscillator: resonance, amplitude and phase response, quality factor Q
- Coupled oscillators: normal modes; general solution as superposition of modes

#### Central Force Motion
- Reduced mass; relative and centre-of-mass coordinates
- Effective potential; classification of orbits
- Kepler's laws derived from Newton's law of gravitation
- Orbital elements; escape velocity

### 2. Electromagnetism

#### Electrostatics
- Coulomb's law; superposition; electric field E
- Gauss's law in integral and differential form; using symmetry
- Electric potential V; relationship E = -∇V
- Conductors in electrostatic equilibrium
- Capacitance; energy stored in electric field
- Dielectrics; polarisation; bound charges; D field; boundary conditions

#### Magnetostatics
- Magnetic force on charges; Lorentz force law
- Biot-Savart law; Ampère's law in integral and differential form
- Magnetic vector potential A; gauge invariance
- Magnetic materials; magnetisation; H field; boundary conditions

#### Electrodynamics
- Faraday's law; Lenz's law; motional EMF
- Maxwell's displacement current; completing Ampère's law
- **Maxwell's equations** (integral and differential forms, in vacuum and in matter)
- The continuity equation; charge conservation
- Electromagnetic waves: derivation from Maxwell's equations in vacuum
- Wave equations for E and B; plane wave solutions; polarisation
- Poynting vector; energy density and energy flux
- Reflection and transmission at boundaries (Fresnel equations overview)
- Radiation from accelerating charges (Larmor formula — qualitative)

### 3. Thermodynamics & Statistical Mechanics

#### Classical Thermodynamics
- Thermodynamic systems; state variables; equations of state
- The zeroth law; temperature; thermal equilibrium
- Work in thermodynamic processes; path dependence
- First law of thermodynamics: $\Delta U = Q - W$; internal energy
- Heat capacities $C_V$ and $C_P$; ideal gas relations
- Second law: Clausius and Kelvin-Planck statements; heat engines
- Carnot cycle; Carnot efficiency; universal upper bound
- Entropy: Clausius and statistical definitions; second law as entropy increase
- Thermodynamic potentials: enthalpy, Helmholtz and Gibbs free energies; Maxwell relations
- Third law of thermodynamics; absolute zero

#### Statistical Mechanics
- Microstates, macrostates, and the statistical definition of entropy $S = k_B \ln \Omega$
- Boltzmann distribution; partition function $Z$
- Ideal gas from statistical mechanics: energy, pressure, equipartition theorem
- Heat capacity of solids: Einstein and Debye models
- Grand canonical ensemble (overview); chemical potential
- Quantum statistics: Fermi-Dirac and Bose-Einstein distributions (needed for Phase 07)

### 4. Waves & Optics

#### Wave Motion
- Travelling waves; wave equation derivation for a string
- Superposition; interference (constructive and destructive)
- Standing waves; normal modes on strings and in cavities
- Wave speed in various media; dispersion relation $\omega(k)$
- Group velocity vs. phase velocity; wave packets
- Sound waves: pressure waves in fluids; speed of sound

#### Geometric and Wave Optics (overview — depth in Phase 12)
- Reflection, refraction; Snell's law; total internal reflection
- Thin lens equation; mirrors
- Diffraction: single-slit, double-slit (Young's experiment), diffraction grating
- Polarisation; Malus's law

---

## Core Texts

| Text | Notes |
|------|-------|
| **Feynman Lectures on Physics Vol. I** | Mechanics, waves, thermodynamics. Read alongside Kleppner. Irreplaceable for physical intuition. |
| **Feynman Lectures on Physics Vol. II** | Electromagnetism and matter. Read alongside Purcell/Griffiths. |
| **Kleppner & Kolenkow — An Introduction to Mechanics** (2nd ed.) | Rigorous, deep on Newtonian mechanics. Best problems at this level. |
| **Taylor — Classical Mechanics** | Adds Lagrangian/Hamiltonian formulation clearly. Read Ch. 6–13 after Kleppner. |
| **Purcell & Morin — Electricity and Magnetism** (3rd ed.) | Derives Maxwell's equations from Coulomb + relativity. The clearest derivation that exists. |
| **Griffiths — Introduction to Electrodynamics** (4th ed.) | The standard advanced EM text. Works through all of Maxwell's equations with complete problem sets. |
| **Schroeder — An Introduction to Thermal Physics** | Excellent balance of rigour and clarity. Best thermodynamics text at this level. |

---

## Supplementary
- MIT 8.01 and 8.02 (OCW) — excellent problem sets for Newtonian mechanics and EM
- Goldstein — *Classical Mechanics* — graduate-level Lagrangian/Hamiltonian reference
- Landau & Lifshitz — *Mechanics* (Vol. 1 of Course of Theoretical Physics) — short, dense, elegant; read after Taylor

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| N-body gravity simulator | Simulate planetary orbits; Euler and RK4 integrators; verify Kepler's laws |
| Pendulum (simple + double) | Phase space plots; demonstrate chaos in the double pendulum |
| Coupled oscillators | Solve for normal modes; animate motion decomposed into modes |
| Electrostatic field visualiser | Plot E and V fields for point charges, dipoles, sheets using numerical methods |
| Electromagnetic wave propagation | Simulate plane wave propagation in 1D with reflection at boundary |
| Carnot cycle simulator | P-V diagram; efficiency as function of temperatures |
| Heat diffusion (1D) | Solve heat equation numerically (finite differences); animated temperature profile |
| Wave on string | Solve wave equation with different boundary conditions and initial conditions |

---

## Build Projects This Phase Feeds Into
- Any controlled physical system (Phases 05–06 require mechanics and dynamics here)
- Circuit design (EM is the basis of every electronic device — Phase 04)
- Rocket propulsion (thermodynamics — Phase 11)
- Sensor design (EM + wave physics — Phases 12–13)

---

## Completion Criteria

You are ready for Phase 03 when you can:
1. Derive Kepler's second law from conservation of angular momentum using the Lagrangian
2. Write down Maxwell's equations in differential form and explain what each term means physically
3. Derive the electromagnetic wave equation from Maxwell's equations in vacuum
4. State and apply the first and second laws of thermodynamics to a heat engine cycle
5. Explain entropy statistically
6. Derive the wave equation for a stretched string from Newton's second law
