//! Linear algebra utilities built on `nalgebra`.
//! Also see `projects/matrix-library` for from-scratch implementations.

use nalgebra::{DMatrix, DVector};

/// Solve Ax = b via nalgebra's LU decomposition.
pub fn solve_linear(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<DVector<f64>> {
    a.clone().lu().solve(b)
}

/// Compute eigenvalues of a real symmetric matrix using nalgebra's symmetric eigen.
pub fn symmetric_eigenvalues(m: &DMatrix<f64>) -> DVector<f64> {
    m.clone().symmetric_eigen().eigenvalues
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::{dmatrix, dvector};

    #[test]
    fn solve_2x2() {
        // 2x + y = 5, x + 3y = 10  =>  x = 1, y = 3
        let a = dmatrix![2.0, 1.0; 1.0, 3.0];
        let b = dvector![5.0, 10.0];
        let x = solve_linear(&a, &b).unwrap();
        assert_abs_diff_eq!(x[0], 1.0, epsilon = 1e-12);
        assert_abs_diff_eq!(x[1], 3.0, epsilon = 1e-12);
    }
}
