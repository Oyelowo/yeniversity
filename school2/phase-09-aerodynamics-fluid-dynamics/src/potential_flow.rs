//! Potential flow: uniform flow + doublet = flow past a cylinder.

use std::f64::consts::PI;

/// Stream function for uniform flow (velocity U, angle = 0): ψ = U·y
pub fn stream_uniform(u: f64, x: f64, y: f64) -> f64 {
    let _ = x;
    u * y
}

/// Stream function for a doublet of strength κ centred at origin.
pub fn stream_doublet(kappa: f64, x: f64, y: f64) -> f64 {
    -kappa * y / (x * x + y * y)
}

/// Combined: uniform flow + doublet = flow past a cylinder of radius R = √(κ/U).
/// ψ = U·r·sin(θ)·(1 − R²/r²)
pub fn stream_cylinder(u: f64, radius: f64, x: f64, y: f64) -> f64 {
    let r2 = x * x + y * y;
    if r2 < 1e-30 { return 0.0; }
    u * y * (1.0 - radius * radius / r2)
}

/// Thin airfoil lift coefficient: C_L = 2π·α (α in radians, small-angle theory).
pub fn thin_airfoil_cl(alpha_rad: f64) -> f64 {
    2.0 * PI * alpha_rad
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn streamline_on_cylinder_surface_is_constant() {
        // On the cylinder surface r=R, ψ should be zero (stagnation streamline).
        let u = 10.0;
        let r = 1.0;
        for &theta in &[0.3_f64, 0.7, 1.2, 2.1, 3.0] {
            let x = r * theta.cos();
            let y = r * theta.sin();
            assert_abs_diff_eq!(stream_cylinder(u, r, x, y), 0.0, epsilon = 1e-12);
        }
    }
}
