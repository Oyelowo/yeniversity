//! 1-D bar FEM: stiffness assembly and solve for a two-element bar.
//!
//! Element stiffness: k_e = (AE/L) [[1,-1],[-1,1]]
//! Assemble, apply BCs (pin left end), solve K u = f.

use nalgebra::{DMatrix, DVector};

/// Element stiffness matrix for one bar element.
pub fn bar_stiffness(a: f64, e: f64, l: f64) -> nalgebra::Matrix2<f64> {
    let k = a * e / l;
    nalgebra::Matrix2::new(k, -k, -k, k)
}

/// Assemble two equal bar elements (3 dofs: nodes 0,1,2).
/// Returns global stiffness matrix (3×3).
pub fn assemble_two_bar(ae_over_l: f64) -> DMatrix<f64> {
    let mut kg = DMatrix::zeros(3, 3);
    let k = ae_over_l;
    // Elem 1: nodes 0-1
    kg[(0,0)] += k; kg[(0,1)] -= k;
    kg[(1,0)] -= k; kg[(1,1)] += k;
    // Elem 2: nodes 1-2
    kg[(1,1)] += k; kg[(1,2)] -= k;
    kg[(2,1)] -= k; kg[(2,2)] += k;
    kg
}

/// Solve two-bar system clamped at node 0, loaded with force f at node 2.
/// Returns displacements [u0, u1, u2].
pub fn solve_two_bar_clamped(ae_over_l: f64, applied_force: f64) -> DVector<f64> {
    // After applying BC u0=0: reduce to 2×2 sub-system
    let k = ae_over_l;
    let k_red = DMatrix::from_vec(2, 2, vec![2.0 * k, -k, -k, k]);
    let f_red = DVector::from_vec(vec![0.0, applied_force]);
    let u_red = k_red.try_inverse().unwrap() * f_red;
    DVector::from_vec(vec![0.0, u_red[0], u_red[1]])
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn tip_displacement_matches_analytical() {
        // Two equal bars, total stiffness = AE/(2L)
        // Tip displacement = F / (AE/2L) = 2FL/(AE)
        let ae_l = 1e6_f64; // AE/L per element
        let f = 1000.0_f64;
        let u = solve_two_bar_clamped(ae_l, f);
        let expected_tip = f / (ae_l / 2.0);
        assert_abs_diff_eq!(u[2], expected_tip, epsilon = 1e-9);
    }
}
