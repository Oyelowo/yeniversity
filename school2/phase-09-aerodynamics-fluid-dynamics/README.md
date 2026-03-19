# Phase 09 — Aerodynamics & Fluid Dynamics

**Duration:** 2–4 months  
**Prerequisites:** Phase 02 (classical mechanics, thermodynamics), Phase 01 (multivariable calculus, PDEs)  
**Can start:** After Phase 02; in parallel with Phase 06 or 10

---

## Why This Phase Exists

Aerodynamics and fluid dynamics are the physics of drones, aircraft, rockets, wind turbines,
cars at speed, HVAC systems, and the cooling of every high-performance electronic system you
will ever design. Lift, drag, thrust, and fuel efficiency are all answers to the equations in
this phase. The Navier-Stokes equations govern every fluid in the universe — understanding them
from first principles gives you the ability to analyse and design any system that moves through
or uses a fluid.

---

## Learning Objectives

- [ ] Derive the continuity equation, Euler equation, and Navier-Stokes equations from first principles
- [ ] Apply Bernoulli's equation correctly; understand its assumptions and when it fails
- [ ] Understand and apply dimensional analysis and the Buckingham π theorem
- [ ] Solve potential flow problems: superposition of elementary flows; lift from circulation
- [ ] Understand thin airfoil theory; derive the lift coefficient from the vortex sheet model
- [ ] Understand boundary layers: laminar and turbulent, separation, skin friction, drag
- [ ] Apply the equations of compressible flow; understand shocks and Prandtl-Meyer expansions
- [ ] Understand propeller and rotor disk theory (actuator disk); relate to drone and helicopter design
- [ ] Set up and solve a simple CFD problem: finite difference discretisation of a PDE in 2D

---

## Topics

### 1. Fluid Statics

- Hydrostatic pressure equation $dp/dz = -\rho g$; pressure in a stationary fluid
- Archimedes' principle; buoyancy force derivation
- Pressure measurement; manometry; gauge vs. absolute pressure
- Surface tension; capillary pressure; Young-Laplace equation

### 2. Fundamental Equations of Fluid Motion

#### Kinematics
- Eulerian vs. Lagrangian description; material (substantial) derivative $D/Dt = \partial/\partial t + (u\cdot\nabla)$
- Streamlines, streaklines, pathlines; when they coincide (steady flow)
- Vorticity $\omega = \nabla \times u$; irrotational flow; circulation $\Gamma = \oint u \cdot dl$
- Stream function $\psi$ and velocity potential $\phi$; relationship to velocity components
- Reynolds Transport Theorem: relating system and control volume descriptions

#### Conservation Equations
- **Continuity equation** (mass conservation): $\partial\rho/\partial t + \nabla\cdot(\rho u) = 0$
- Incompressible form: $\nabla \cdot u = 0$
- **Momentum equation** (Newton's 2nd law for fluids): $\rho Du/Dt = -\nabla p + \mu\nabla^2 u + \rho g$
- **Navier-Stokes equations** for incompressible Newtonian fluid — full derivation
- **Euler equations** (inviscid limit): $\rho Du/Dt = -\nabla p + \rho g$
- Energy equation: first law for a fluid element; enthalpy; total energy

#### Bernoulli's Equation
- Derivation from Euler equation along a streamline (steady, inviscid, incompressible)
- Extended Bernoulli: unsteady, compressible, rotational forms and their restrictions
- Applications: Venturi meter, pitot tube, flow over a wing (and the limits of this application)

### 3. Dimensional Analysis & Similitude

- Buckingham π theorem: counting independent dimensionless groups
- Key dimensionless numbers: Reynolds $Re = \rho u L / \mu$; Mach $M = u/a$; Froude $Fr = u/\sqrt{gL}$; Strouhal $St = fL/u$
- Dynamic similarity; model testing; wind tunnel and water tunnel scaling laws
- Physical interpretation of each dimensionless group

### 4. Viscous Flow & Boundary Layers

#### Internal Viscous Flow
- Couette flow (planar); Poiseuille flow (pipe flow): parabolic velocity profile; derivation
- Pipe flow: Moody chart; Darcy-Weisbach equation; minor losses
- Laminar vs. turbulent pipe flow; critical Reynolds number

#### External Boundary Layers
- Boundary layer concept: Prandtl's insight
- Blasius solution for laminar flat-plate boundary layer: $\delta/x \sim Re_x^{-1/2}$
- Displacement and momentum thickness; von Kármán momentum integral equation
- Turbulent boundary layer: logarithmic velocity profile; skin friction coefficient
- Boundary layer separation: adverse pressure gradient; effect of shape on separation
- Wake region behind bluff bodies; vortex shedding; Strouhal number

### 5. Potential Flow & Aerodynamics

#### Potential Flow Theory
- Velocity potential $\phi$; irrotational, incompressible flow → Laplace equation $\nabla^2\phi = 0$
- Elementary potential flows: uniform stream, source/sink, doublet, vortex
- Superposition: flow over a cylinder (stream + doublet); d'Alembert's paradox
- Kutta-Joukowski theorem: lift $L = \rho U_\infty \Gamma$ per unit span
- Joukowski transformation: conformal mapping of cylinder to airfoil

#### Thin Airfoil Theory
- Lift and moment of a thin cambered airfoil from vortex sheet model
- Thin symmetric airfoil: lift slope $dC_L/d\alpha = 2\pi$; centre of pressure
- Cambered airfoil: zero-lift angle of attack; pitching moment
- Flap effects; Kutta condition derivation

#### Finite Wings
- Lifting line theory (Prandtl): bound + trailing vortex sheet
- Elliptical lift distribution: minimum induced drag; span efficiency factor $e$
- Aspect ratio effect on $C_L$ slope; induced drag coefficient $C_{Di} = C_L^2 / (\pi e AR)$
- Winglets; non-planar lifting surfaces (overview)

#### Drag
- Profile drag = pressure drag + skin friction drag
- Drag polars; $C_D$ vs. $C_L$; maximum lift-to-drag ratio and its importance
- Drag reduction strategies: streamlining, laminar flow control, riblets

### 6. Compressible Flow

#### Basics
- Speed of sound: derivation from linearised compressibility $a = \sqrt{\partial p/\partial\rho}$
- Mach number regimes: subsonic, transonic, supersonic, hypersonic
- Stagnation conditions: stagnation pressure, temperature, density
- Isentropic flow relations: $p/p_0$, $T/T_0$, $\rho/\rho_0$ as functions of $M$
- Choked flow at sonic conditions ($M=1$) in a nozzle

#### Nozzle Flow (Critical for Rocket Propulsion — connects to Phase 11)
- De Laval (converging-diverging) nozzle; area-Mach number relation $A/A^* = f(M)$
- Nozzle operating conditions: design pressure ratio; over- and under-expanded flow
- Normal shocks: Rankine-Hugoniot conditions; shock relations for $p$, $T$, $\rho$, $M$
- Oblique shocks and Prandtl-Meyer expansion fans; shock-expansion method

### 7. Propeller & Rotor Theory

#### Actuator Disk Theory
- Momentum theory: thrust, power, and efficiency of an ideal disk in axial flow
- Figure of merit; induced velocity; thrust coefficient $C_T$; power coefficient $C_P$
- Comparison: hovering vs. forward flight efficiency
- Blade element momentum (BEM) theory: combining local blade aerodynamics with momentum theory

#### Helicopter and Drone Rotor Aerodynamics
- Hover, climb, and forward flight performance
- Rotor wake; vortex ring state
- Ground effect; ceiling (hover OGE vs. IGE)
- Application to quadrotor: thrust vs. RPM for a given rotor/propeller

### 8. Introduction to Computational Fluid Dynamics (CFD)

#### Numerical Methods for PDEs (Review applied to fluids)
- Finite difference discretisation: forward, backward, and central differences
- Stability analysis: CFL condition; von Neumann stability
- Explicit vs. implicit time stepping; Crank-Nicolson
- Upwind schemes for convection-dominated flows

#### Solving the 2D Navier-Stokes Equations (Lid-Driven Cavity)
- Pressure-velocity coupling: SIMPLE algorithm (reference only; implement FD variant)
- Vorticity-stream function formulation for 2D incompressible flow
- Finite difference solution of vorticity transport + Poisson equation for stream function

---

## Core Texts

| Text | Notes |
|------|-------|
| **Anderson — Introduction to Flight** (8th ed.) | Best first text. Covers aerodynamics, propulsion, stability in a unified way. Very readable. |
| **Anderson — Fundamentals of Aerodynamics** (6th ed.) | The rigorous aerodynamics reference. Derives all results. Use alongside Introduction to Flight. |
| **White — Fluid Mechanics** (8th ed.) | Comprehensive, rigorous fluid mechanics from viscous flow to turbulence to compressible flow. |
| **Shapiro — The Dynamics and Thermodynamics of Compressible Fluid Flow** (2 vols.) | The definitive reference on compressible flow and gas dynamics. Dense and masterful. |
| **Munson, Young & Okiishi — Fundamentals of Fluid Mechanics** | More accessible than White for first contact with the material. |

---

## Supplementary
- Prandtl & Tietjens — *Fundamentals of Hydro- and Aeromechanics* — historic, still insightful
- Pope — *Turbulent Flows* — graduate-level reference for turbulence
- MIT 16.100 *Aerodynamics* (OCW) — lecture notes and problem sets
- XFOIL documentation — understand how a panel method code works

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Potential flow visualiser | Superpose elementary flows; plot streamlines and pressure fields |
| 2D panel method | Solve for lift on arbitrary airfoil shape; compare to thin airfoil theory |
| Blasius boundary layer | Numerical solution of Blasius ODE; plot $\delta(x)$ and skin friction |
| Normal shock calculator | Compute upstream/downstream properties for any Mach number |
| Nozzle flow solver | Solve isentropic flow through a converging-diverging nozzle |
| Actuator disk calculator | Thrust, power, and figure of merit for a given rotor diameter and RPM |
| 2D vorticity-streamfunction solver | Lid-driven cavity flow; plot streamlines and vorticity contours |

---

## Completion Criteria

You are ready to move on when you can:
1. Derive the Navier-Stokes equations from the Reynolds transport theorem applied to momentum
2. Apply the Kutta-Joukowski theorem to compute lift from a given circulation
3. Explain the origin of induced drag on a finite wing and give the formula for $C_{Di}$
4. Compute the exit Mach number and thrust for a converging-diverging nozzle at a given area ratio
5. Describe what an actuator disk is and derive the thrust coefficient in terms of induced velocity
