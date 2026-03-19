//! Phase 10 — Structural Mechanics
//!
//! Modules:
//! - `stress_strain`   — Hooke's law, principal stresses, Mohr's circle
//! - `beam_theory`     — Euler-Bernoulli beam: shear/moment diagrams, deflection
//! - `fem`             — 1-D bar/truss FEM: stiffness assembly, solve
//!
//! See `README.md` for the full curriculum.

pub mod beam_theory;
pub mod fem;
pub mod stress_strain;
