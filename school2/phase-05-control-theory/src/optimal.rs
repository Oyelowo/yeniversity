//! Optimal control stubs — LQR via a simplified algebraic Riccati solver.
//! Full iterative DARE (Discrete Algebraic Riccati Equation) to come.

/// Placeholder: study topic for LQR, MPC, Pontryagin's minimum principle.
pub fn lqr_note() -> &'static str {
    "Minimise J = ∫(xᵀQx + uᵀRu)dt; solution: u* = -Kx where K = R⁻¹BᵀP and P solves the ARE."
}
