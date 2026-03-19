//! Phase 05 — Control Theory
//!
//! Modules:
//! - `pid`          — PID controller with anti-windup
//! - `state_space`  — LTI state-space model, simulation, stability
//! - `optimal`      — LQR cost minimisation (Riccati equation)
//! - `kalman`       — Discrete linear Kalman filter
//!
//! See `README.md` for the full curriculum.

pub mod kalman;
pub mod optimal;
pub mod pid;
pub mod state_space;
