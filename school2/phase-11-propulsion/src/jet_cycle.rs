//! Brayton cycle (ideal gas turbine): compressor work, turbine work, thermal efficiency.

/// Ideal isentropic temperature ratio across a compressor/turbine.
/// T2/T1 = (P2/P1)^((γ-1)/γ)
pub fn isentropic_temp_ratio(pressure_ratio: f64, gamma: f64) -> f64 {
    pressure_ratio.powf((gamma - 1.0) / gamma)
}

/// Ideal Brayton cycle thermal efficiency: η = 1 - T1/T2 = 1 - 1/r^((γ-1)/γ)
pub fn brayton_efficiency(pressure_ratio: f64, gamma: f64) -> f64 {
    1.0 - 1.0 / isentropic_temp_ratio(pressure_ratio, gamma)
}

/// Compressor work per unit mass (ideal): w_c = cp(T2 - T1)
pub fn compressor_work(t_in: f64, pressure_ratio: f64, gamma: f64, cp: f64) -> f64 {
    cp * t_in * (isentropic_temp_ratio(pressure_ratio, gamma) - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn brayton_efficiency_at_pr_10() {
        // γ=1.4, PR=10 → η ≈ 48.2%
        let eta = brayton_efficiency(10.0, 1.4);
        assert_abs_diff_eq!(eta, 0.4821, epsilon = 1e-4);
    }
}
