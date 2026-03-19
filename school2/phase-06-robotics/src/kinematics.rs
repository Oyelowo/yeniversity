//! Forward kinematics for a planar 2-DOF arm.

use nalgebra::{Point2, Vector2};

/// End-effector position of a planar 2-DOF arm given joint angles θ1, θ2 (radians).
pub fn planar_2dof_fk(l1: f64, l2: f64, theta1: f64, theta2: f64) -> Point2<f64> {
    let x = l1 * theta1.cos() + l2 * (theta1 + theta2).cos();
    let y = l1 * theta1.sin() + l2 * (theta1 + theta2).sin();
    Point2::new(x, y)
}

/// Jacobian of the planar 2-DOF arm (2×2 matrix, column-major).
pub fn planar_2dof_jacobian(l1: f64, l2: f64, theta1: f64, theta2: f64) -> nalgebra::Matrix2<f64> {
    let s1 = theta1.sin();
    let s12 = (theta1 + theta2).sin();
    let c1 = theta1.cos();
    let c12 = (theta1 + theta2).cos();
    nalgebra::Matrix2::new(
        -l1 * s1 - l2 * s12, -l2 * s12,
         l1 * c1 + l2 * c12,  l2 * c12,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn stretched_out_arm() {
        // θ1=0, θ2=0 → arm fully extended along x-axis
        let p = planar_2dof_fk(1.0, 1.0, 0.0, 0.0);
        assert_abs_diff_eq!(p.x, 2.0, epsilon = 1e-10);
        assert_abs_diff_eq!(p.y, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn folded_back_arm() {
        // θ1=0, θ2=π → second link points back
        let p = planar_2dof_fk(1.0, 1.0, 0.0, PI);
        assert_abs_diff_eq!(p.x, 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(p.y, 0.0, epsilon = 1e-10);
    }
}
