//! Electromagnetism: Coulomb's law, field superposition, Biot-Savart.

use nalgebra::Vector3;

const K_E: f64 = 8.987_551_787e9; // Coulomb constant k = 1/(4πε₀)  [N·m²/C²]

/// Electric field at `r_obs` due to a point charge `q` at `r_src`.
pub fn electric_field_point_charge(
    r_obs: Vector3<f64>,
    r_src: Vector3<f64>,
    q: f64,
) -> Vector3<f64> {
    let r = r_obs - r_src;
    let dist = r.norm();
    if dist < 1e-12 {
        return Vector3::zeros();
    }
    K_E * q / dist.powi(3) * r
}

/// Coulomb force on charge `q1` at `r1` due to charge `q2` at `r2`.
pub fn coulomb_force(r1: Vector3<f64>, q1: f64, r2: Vector3<f64>, q2: f64) -> Vector3<f64> {
    electric_field_point_charge(r1, r2, q2) * q1
}

/// Electric potential at `r_obs` due to a point charge `q` at `r_src`.
pub fn electric_potential(r_obs: Vector3<f64>, r_src: Vector3<f64>, q: f64) -> f64 {
    let dist = (r_obs - r_src).norm();
    K_E * q / dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn coulomb_force_magnitude() {
        // Two unit charges 1 m apart: F = k
        let r1 = Vector3::new(0.0, 0.0, 0.0);
        let r2 = Vector3::new(1.0, 0.0, 0.0);
        let f = coulomb_force(r1, 1.0, r2, 1.0);
        assert_abs_diff_eq!(f.norm(), K_E, epsilon = 1.0);
    }
}
