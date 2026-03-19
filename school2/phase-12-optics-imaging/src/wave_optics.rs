//! Wave optics: double-slit interference, single-slit diffraction intensity.

use std::f64::consts::PI;

/// Double-slit intensity pattern.
/// I(θ) = I0 · cos²(π d sinθ / λ)
pub fn double_slit_intensity(i0: f64, d: f64, lambda: f64, theta: f64) -> f64 {
    let phase = PI * d * theta.sin() / lambda;
    i0 * phase.cos().powi(2)
}

/// Single-slit diffraction intensity (Fraunhofer).
/// I(θ) = I0 · sinc²(π a sinθ / λ)  where sinc(x) = sin(x)/x
pub fn single_slit_intensity(i0: f64, a: f64, lambda: f64, theta: f64) -> f64 {
    let beta = PI * a * theta.sin() / lambda;
    let sinc = if beta.abs() < 1e-12 { 1.0 } else { beta.sin() / beta };
    i0 * sinc * sinc
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn double_slit_central_maximum() {
        // At θ=0 intensity equals I0
        assert_abs_diff_eq!(double_slit_intensity(1.0, 1e-4, 500e-9, 0.0), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn single_slit_central_maximum() {
        assert_abs_diff_eq!(single_slit_intensity(1.0, 1e-4, 500e-9, 0.0), 1.0, epsilon = 1e-12);
    }
}
