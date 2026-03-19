//! Robot dynamics stubs — Lagrangian and Newton-Euler formulations.

/// Placeholder: double-pendulum Lagrangian EOM (study exercise).
/// Derive T and V, apply Euler-Lagrange to get coupled ODEs.
pub fn double_pendulum_description() -> &'static str {
    "d/dt(∂L/∂q̇ᵢ) - ∂L/∂qᵢ = τᵢ  where L = T - V"
}
