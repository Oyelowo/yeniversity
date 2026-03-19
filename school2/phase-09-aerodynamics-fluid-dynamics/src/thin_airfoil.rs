//! Thin airfoil theory stub.

pub use crate::potential_flow::thin_airfoil_cl;

/// Dynamic pressure: q = ½ρV²
pub fn dynamic_pressure(rho: f64, velocity: f64) -> f64 {
    0.5 * rho * velocity * velocity
}

/// Lift per unit span: L' = q · c · C_L
pub fn lift_per_span(rho: f64, velocity: f64, chord: f64, cl: f64) -> f64 {
    dynamic_pressure(rho, velocity) * chord * cl
}
