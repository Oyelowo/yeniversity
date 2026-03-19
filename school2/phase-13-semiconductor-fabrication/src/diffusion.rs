//! Dopant diffusion in silicon: Fick's second law solutions.

use std::f64::consts::PI;

/// Gaussian dopant profile from a finite source (pre-dep drive-in).
/// N(x, t) = (Q / sqrt(π D t)) · exp(–x²/(4Dt))
/// Q: dopant dose (cm⁻²), D: diffusivity (cm²/s), t: time (s), x: depth (cm).
pub fn gaussian_profile(x: f64, dose: f64, diffusivity: f64, time: f64) -> f64 {
    let dt = diffusivity * time;
    (dose / (PI * dt).sqrt()) * (-(x * x) / (4.0 * dt)).exp()
}

/// Erfc (complementary error function approximation using Horner's method).
pub fn erfc_approx(x: f64) -> f64 {
    // Simple rational approximation (max error ~1.5e-7)
    if x < 0.0 { return 2.0 - erfc_approx(-x); }
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    poly * (-(x * x)).exp()
}

/// Erfc dopant profile from a constant surface concentration (pre-dep).
/// N(x, t) = N_s · erfc(x / (2 sqrt(D·t)))
pub fn erfc_profile(x: f64, surface_conc: f64, diffusivity: f64, time: f64) -> f64 {
    surface_conc * erfc_approx(x / (2.0 * (diffusivity * time).sqrt()))
}

/// Junction depth: where N(x_j) = N_background.
/// Solve numerically; for Gaussian: x_j = 2 sqrt(Dt · ln(Q / (N_bg sqrt(π D t))))
pub fn junction_depth_gaussian(dose: f64, background_conc: f64, diffusivity: f64, time: f64) -> f64 {
    let dt = diffusivity * time;
    let arg = dose / (background_conc * (PI * dt).sqrt());
    if arg <= 1.0 { return 0.0; }
    2.0 * (dt * arg.ln()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn gaussian_peak_at_surface() {
        let dose = 1e14_f64;   // cm⁻²
        let d = 1e-13_f64;     // cm²/s
        let t = 3600_f64;      // s
        let peak = gaussian_profile(0.0, dose, d, t);
        let expected = dose / (std::f64::consts::PI * d * t).sqrt();
        assert_abs_diff_eq!(peak, expected, epsilon = 1e-10 * expected);
    }
}
