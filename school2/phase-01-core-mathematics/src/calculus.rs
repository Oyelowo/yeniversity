//! Numerical calculus: differentiation, integration, series.

/// Central-difference derivative approximation: f'(x) ≈ [f(x+h) - f(x-h)] / 2h.
pub fn derivative<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Composite Simpson's rule for ∫[a,b] f(x) dx.
/// `n` must be even.
pub fn integrate_simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    assert!(n % 2 == 0, "n must be even for Simpson's rule");
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + i as f64 * h;
        sum += if i % 2 == 0 { 2.0 * f(x) } else { 4.0 * f(x) };
    }
    sum * h / 3.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn derivative_of_x_squared() {
        // d/dx x² = 2x; at x=3 should be 6
        let d = derivative(|x| x * x, 3.0, 1e-5);
        assert_abs_diff_eq!(d, 6.0, epsilon = 1e-8);
    }

    #[test]
    fn integral_of_sin_over_half_period() {
        // ∫[0,π] sin(x) dx = 2
        let result = integrate_simpson(f64::sin, 0.0, std::f64::consts::PI, 1000);
        assert_abs_diff_eq!(result, 2.0, epsilon = 1e-10);
    }
}
