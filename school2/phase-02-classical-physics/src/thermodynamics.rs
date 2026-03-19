//! Thermodynamics: ideal gas, Carnot efficiency, heat capacity.

/// Ideal gas pressure from the equation of state: PV = nRT.
pub fn ideal_gas_pressure(n: f64, temperature_k: f64, volume_m3: f64) -> f64 {
    const R: f64 = 8.314_462_618; // J/(mol·K)
    n * R * temperature_k / volume_m3
}

/// Carnot efficiency: η = 1 - T_cold/T_hot (temperatures in Kelvin).
pub fn carnot_efficiency(t_hot: f64, t_cold: f64) -> f64 {
    assert!(t_hot > t_cold && t_cold > 0.0, "Must have 0 < T_cold < T_hot");
    1.0 - t_cold / t_hot
}

/// Statistical entropy: S = k_B * ln(Ω).
pub fn boltzmann_entropy(omega: f64) -> f64 {
    const K_B: f64 = 1.380_649e-23; // J/K
    K_B * omega.ln()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn carnot_efficiency_known_values() {
        // η = 1 - 300/600 = 0.5
        assert_abs_diff_eq!(carnot_efficiency(600.0, 300.0), 0.5, epsilon = 1e-12);
    }
}
