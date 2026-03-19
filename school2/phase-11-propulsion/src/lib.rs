//! Phase 11 — Propulsion
//!
//! Modules:
//! - `rocket_nozzle`  — de Laval nozzle: throat area ratio, Isp, thrust
//! - `tsiolkovsky`    — Rocket equation: Δv, mass ratio, staging
//! - `jet_cycle`      — Brayton cycle: compressor/turbine work, thermal efficiency
//!
//! See `README.md` for the full curriculum.

pub mod jet_cycle;
pub mod rocket_nozzle;
pub mod tsiolkovsky;
