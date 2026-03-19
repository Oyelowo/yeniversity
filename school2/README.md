# Yeniversity — Engineering from First Principles

A self-directed curriculum covering mathematics, physics, chemistry, electronics, control theory,
robotics, AI/ML, aerodynamics, structural mechanics, propulsion, optics, semiconductor fabrication,
VLSI design, and systems engineering.

**Goal:** Full first-principles mastery of every technology domain — from axioms to designing,
building, and fabricating devices, vehicles, chips, sensors, robots, aircraft, rockets, and
large-scale systems entirely from scratch.

**Primary implementation language: Rust**

---

## Philosophy

- **Depth over breadth** — understand one thing completely before moving on
- **First principles always** — derive before you use; never accept a formula without tracing it to its origin
- **Build as you learn** — every phase produces working simulations, implementations, or physical artifacts
- **Rust for computation** — all numerical solvers, simulations, and implementations are written in Rust, building systems-level depth alongside the science
- **The goal is not to use tools — it is to understand them completely enough to build them**

---

## Repository Structure

```
yeniversity/
├── Cargo.toml                            # Cargo workspace — all phases and projects
├── README.md                             # Master curriculum (this file)
│
├── phase-00-mathematical-maturity/       # Logic, proofs, sets, mathematical reasoning
├── phase-01-core-mathematics/            # Calculus, linear algebra, ODEs/PDEs, probability
├── phase-02-classical-physics/           # Mechanics, EM, thermodynamics, waves
├── phase-03-chemistry-materials/         # Physical chem, materials science, semiconductor physics
├── phase-04-electronics-hardware/        # Analog, digital, signals, embedded systems
├── phase-05-control-theory/              # Classical + modern control, optimal control
├── phase-06-robotics/                    # Kinematics, dynamics, SLAM, motion planning
├── phase-07-quantum-mechanics/           # QM, solid state physics, band theory
├── phase-08-ai-ml/                       # ML, deep learning, reinforcement learning
├── phase-09-aerodynamics-fluid-dynamics/ # Fluid mechanics, aerodynamics, CFD
├── phase-10-structural-mechanics/        # Stress/strain, FEM, structural analysis
├── phase-11-propulsion/                  # Rocket + jet engines, combustion thermodynamics
├── phase-12-optics-imaging/              # Ray/wave optics, Fourier optics, image sensors
├── phase-13-semiconductor-fabrication/   # Fab processes, device physics, lithography
├── phase-14-vlsi-design/                 # CMOS design, HDL, open PDKs, tapeout
├── phase-15-systems-engineering/         # Requirements, architecture, reliability, integration
│
└── projects/                             # Cross-phase build projects
    ├── ode-solver/
    ├── kalman-filter/
    ├── matrix-library/
    ├── risc-v-cpu/
    ├── drone-flight-controller/
    └── pid-controller/
```

Each phase directory has a consistent layout:

```
phase-XX-name/
├── README.md       # Phase curriculum: objectives, texts, topics, exercises, Rust projects
├── Cargo.toml      # Cargo crate for this phase's simulations and implementations
├── src/
│   └── lib.rs      # Rust simulation code (modules per sub-topic)
├── notes/          # Your handwritten or typed derivations and lecture notes (Markdown)
└── exercises/      # Problem sets and worked solutions (Markdown)
```

---

## Curriculum Overview

| #  | Phase                          | Est. Duration | Core Topics                                           |
|----|-------------------------------|---------------|-------------------------------------------------------|
| 00 | Mathematical Maturity          | 4–8 weeks     | Logic, set theory, proof techniques, induction        |
| 01 | Core Mathematics               | 4–6 months    | Calculus, linear algebra, ODEs, PDEs, probability     |
| 02 | Classical Physics              | 4–6 months    | Mechanics, electromagnetism, thermodynamics, waves    |
| 03 | Chemistry & Materials          | 2–3 months    | Physical chemistry, materials science, semiconductors |
| 04 | Electronics & Hardware         | 3–5 months    | Analog, digital, signals & systems, embedded          |
| 05 | Control Theory                 | 2–3 months    | PID, state-space, LQR, Kalman filter, stability       |
| 06 | Robotics                       | 3–4 months    | Kinematics, dynamics, SLAM, motion planning           |
| 07 | Quantum Mechanics              | 2–3 months    | Wave mechanics, operators, solid state, band theory   |
| 08 | AI / Machine Learning          | 3–5 months    | Linear models, neural nets, deep learning, RL         |
| 09 | Aerodynamics & Fluid Dynamics  | 2–4 months    | Navier-Stokes, airfoil theory, compressible flow      |
| 10 | Structural Mechanics           | 2–3 months    | Stress/strain, failure, FEM, structural analysis      |
| 11 | Propulsion                     | 2–3 months    | Rocket engines, jet propulsion, combustion            |
| 12 | Optics & Imaging               | 2–3 months    | Ray/wave optics, Fourier optics, image sensors        |
| 13 | Semiconductor Fabrication      | 2–3 months    | Fab processes, device physics, photolithography       |
| 14 | VLSI Design                    | 2–3 months    | CMOS cells, HDL, chip design, open PDKs, tapeout      |
| 15 | Systems Engineering            | 2–3 months    | Architecture, reliability, FMEA, large systems        |

**Total: approximately 3–4 years of focused study (1–3 hours/day)**

> Phases 07–09 can run in parallel with Phase 06 once Phase 02 is complete.
> Phase 07 can start any time after Phase 02.

---

## What You Will Be Able to Build

| Capability                                          | Unlocked by Phase   |
|-----------------------------------------------------|---------------------|
| Numerical solvers, simulations in Rust              | 01                  |
| Simulate physical systems (N-body, EM fields)       | 02                  |
| Analog circuits, filters, signal processing         | 04                  |
| Bare-metal firmware for microcontrollers            | 04                  |
| Full flight controller (attitude + sensor fusion)   | 05–06               |
| Autonomous robot (kinematics, planning, SLAM)       | 06                  |
| Custom quadrotor / drone from scratch               | 06                  |
| Fixed-wing aircraft (aerodynamics + electronics)    | 09                  |
| High-power rocket (propulsion + guidance computer)  | 11                  |
| Custom camera (optics + CMOS readout + FPGA)        | 12                  |
| ASIC design submitted to open fab (Efabless)        | 14                  |
| Discrete transistors and simple ICs in a home lab   | 13                  |
| RISC-V soft CPU on FPGA                             | 04/14               |
| Full understanding of a smartphone at physics level | 03/04/12/14         |

---

## Build Projects

| Project                  | Phase    | Description                                                      |
|--------------------------|----------|------------------------------------------------------------------|
| `ode-solver`             | 01       | Euler, RK2, RK4, adaptive steppers; verified against known solutions |
| `matrix-library`         | 01       | Full matrix/vector library built from scratch without ndarray    |
| `kalman-filter`          | 05/06    | Linear KF + Extended KF + Unscented KF for nonlinear systems    |
| `pid-controller`         | 05       | PID with anti-windup, Bode/Nyquist plotting, step response       |
| `drone-flight-controller`| 06       | Quadrotor dynamics + attitude controller + IMU sensor fusion     |
| `risc-v-cpu`             | 04/14    | RV32I soft CPU (emulated in Rust; optionally synthesised to FPGA)|

---

## Cargo Workspace Usage

Build everything:
```bash
cargo build --workspace
cargo test --workspace
```

Run a specific project:
```bash
cargo run -p ode-solver
cargo run -p drone-flight-controller
```

Test a specific phase:
```bash
cargo test -p p01-core-mathematics
cargo test -p p05-control-theory
```

Run a specific simulation inside a phase (via example or binary):
```bash
cargo run --example rk4_pendulum -p p02-classical-physics
```

---

## Key Resources (Across All Phases)

| Resource                                              | Use                                      |
|-------------------------------------------------------|------------------------------------------|
| **Feynman Lectures on Physics** (Vols I–III)          | Physical intuition — read throughout 01–03 (free: feynmanlectures.caltech.edu) |
| **3Blue1Brown** (YouTube)                             | Geometric intuition for calculus, LA, neural nets |
| **MIT OpenCourseWare**                                | Free problem sets and lecture notes for most subjects |
| **Sam Zeloof** (YouTube / blog)                       | Garage IC fabrication — essential for Phase 13 |
| **Efabless / OpenMPW**                                | Free ASIC tapeout program — target for Phase 14 |
| **SkyWater 130nm / GF 180nm PDK** (GitHub/open-source)| Open process design kits for Phase 14   |

---

## Study Convention

### Notes (`notes/`)

Filename format: `NN-topic-name.md`
Example: `01-limits-and-continuity.md`

Every note should contain:
1. Core definition or theorem, stated precisely
2. Derivation from first principles (show every step)
3. Geometric or physical intuition
4. One fully worked example
5. Connection to adjacent topics and later phases

### Exercises (`exercises/`)

Filename format: `NN-topic-exercises.md`
Work every problem by hand first. Then verify the result computationally with a Rust test.

### Simulations (`src/`)

Every Rust simulation should:
- Have a `#[test]` verifying against a known analytic result
- Print or plot output that can be visually inspected
- Include a doc comment naming the physical/mathematical context it models
- Be kept small and focused — one concept per function or module

---

## Progression Notes

The phases are ordered by dependency, not difficulty. Each phase explicitly lists its prerequisites.
You will revisit earlier material constantly — that is by design, not a sign of weakness. 
The Feynman Lectures should be read continuously alongside Phases 01 and 02.
Do not use online courses as a primary resource — textbooks are the standard.
Do not move on until you can derive the key results of a phase without looking.
