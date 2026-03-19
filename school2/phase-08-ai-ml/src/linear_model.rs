//! Linear and logistic regression via gradient descent.

/// Ordinary least squares: θ = (XᵀX)⁻¹Xᵀy
/// For small problems, solved via nalgebra. Returns weight vector.
pub fn linear_regression(x: &nalgebra::DMatrix<f64>, y: &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    let xt = x.transpose();
    let xtx = &xt * x;
    xtx.try_inverse().expect("XᵀX must be invertible") * &xt * y
}

/// Sigmoid activation.
pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

/// Binary cross-entropy loss.
pub fn binary_cross_entropy(y_true: &[f64], y_pred: &[f64]) -> f64 {
    assert_eq!(y_true.len(), y_pred.len());
    let n = y_true.len() as f64;
    y_true.iter().zip(y_pred.iter())
        .map(|(yt, yp)| -yt * yp.ln() - (1.0 - yt) * (1.0 - yp).ln())
        .sum::<f64>()
        / n
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn perfect_linear_fit() {
        // y = 2x + 1
        let x = nalgebra::DMatrix::from_vec(4, 2, vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 2.0, 3.0]);
        let y = nalgebra::DVector::from_vec(vec![1.0, 3.0, 5.0, 7.0]);
        let w = linear_regression(&x, &y);
        assert_abs_diff_eq!(w[0], 1.0, epsilon = 1e-9); // intercept
        assert_abs_diff_eq!(w[1], 2.0, epsilon = 1e-9); // slope
    }
}
