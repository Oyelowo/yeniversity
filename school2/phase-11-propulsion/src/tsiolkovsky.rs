//! Tsiolkovsky rocket equation and staging analysis.

/// Ideal delta-v from the rocket equation: Δv = v_e · ln(m0/mf)
/// v_e: effective exhaust velocity (m/s)
pub fn delta_v(v_exhaust: f64, mass_initial: f64, mass_final: f64) -> f64 {
    v_exhaust * (mass_initial / mass_final).ln()
}

/// Specific impulse from exhaust velocity: Isp = v_e / g0
pub fn specific_impulse(v_exhaust: f64) -> f64 {
    v_exhaust / 9.80665
}

/// Mass ratio needed to achieve Δv with exhaust velocity v_e.
pub fn required_mass_ratio(delta_v: f64, v_exhaust: f64) -> f64 {
    (delta_v / v_exhaust).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn falcon9_like_first_stage_dv() {
        // v_e ≈ 3050 m/s (RP-1/LOX), mass ratio ≈ 4.0 → Δv ≈ 4228 m/s
        let dv = delta_v(3050.0, 4.0, 1.0);
        assert_abs_diff_eq!(dv, 3050.0 * 4_f64.ln(), epsilon = 0.01);
    }
}
