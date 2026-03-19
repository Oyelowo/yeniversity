//! Thermal oxidation of silicon: Deal-Grove model.
//!
//! tox² / B + tox / (B/A) = t + τ
//! where B = parabolic rate constant, B/A = linear rate constant, τ = initial offset.
//! Solve for tox at time t (iteratively).

/// Oxide thickness from the Deal-Grove model.
/// `b_para`: parabolic rate constant (µm²/h)
/// `b_over_a`: linear rate constant (µm/h)
/// `tau`: initial time offset (h) accounting for native oxide
/// `time_h`: oxidation time in hours
/// Returns oxide thickness in µm.
pub fn oxide_thickness(b_para: f64, b_over_a: f64, tau: f64, time_h: f64) -> f64 {
    let t_eff = time_h + tau;
    // Quadratic: tox² + A·tox - B·t_eff = 0  where A = B/(B/A)
    let a = b_para / b_over_a;
    let discriminant = a * a + 4.0 * b_para * t_eff;
    (discriminant.sqrt() - a) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn long_oxidation_approaches_parabolic_limit() {
        // At long times tox ≈ sqrt(B·t)
        let b = 0.0117;    // µm²/h   (dry O2 at 1000°C)
        let ba = 0.0208;   // µm/h
        let t = 10_000.0;  // h  (very long)
        let tox = oxide_thickness(b, ba, 0.0, t);
        let parabolic = (b * t).sqrt();
        // They should agree to within 5%
        assert!((tox - parabolic).abs() / parabolic < 0.05);
    }
}
