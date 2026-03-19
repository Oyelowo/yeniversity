//! Phase 13 — Semiconductor Fabrication
//!
//! Modules:
//! - `diffusion`    — Fick's laws: Gaussian and erfc dopant profiles
//! - `oxidation`    — Deal-Grove model for thermal SiO₂ growth
//! - `process_flow` — High-level process step ordering utilities
//!
//! See `README.md` for the full curriculum.

pub mod diffusion;
pub mod oxidation;
pub mod process_flow;
