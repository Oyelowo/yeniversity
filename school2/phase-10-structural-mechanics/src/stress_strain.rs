//! Stress and strain: Hooke's law, principal stresses, von Mises criterion.

/// Axial stress σ = F / A
pub fn axial_stress(force: f64, area: f64) -> f64 { force / area }

/// Axial strain ε = δ / L
pub fn axial_strain(elongation: f64, length: f64) -> f64 { elongation / length }

/// Young's modulus: E = σ / ε (returns E if you supply stress and strain)
pub fn youngs_modulus(stress: f64, strain: f64) -> f64 { stress / strain }

/// Von Mises equivalent stress in 2D plane stress (σ_x, σ_y, τ_xy).
pub fn von_mises_2d(sx: f64, sy: f64, txy: f64) -> f64 {
    (sx * sx - sx * sy + sy * sy + 3.0 * txy * txy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn uniaxial_von_mises_equals_stress() {
        // Pure uniaxial σ_x = 100 MPa → von Mises = 100 MPa
        assert_abs_diff_eq!(von_mises_2d(100e6, 0.0, 0.0), 100e6, epsilon = 1.0);
    }

    #[test]
    fn strain_from_steel_bar() {
        // E_steel = 200 GPa, σ = 100 MPa → ε = 0.0005
        let e = 200e9_f64;
        let sigma = 100e6_f64;
        let eps = sigma / e;
        assert_abs_diff_eq!(eps, 5e-4, epsilon = 1e-12);
    }
}
