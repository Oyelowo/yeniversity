//! Thermochemistry: Gibbs free energy, equilibrium, Arrhenius.

const R: f64 = 8.314_462_618; // J/(mol·K)

/// Gibbs free energy: ΔG = ΔH - T·ΔS
pub fn gibbs_free_energy(delta_h: f64, temperature_k: f64, delta_s: f64) -> f64 {
    delta_h - temperature_k * delta_s
}

/// Equilibrium constant K from standard ΔG°: K = exp(-ΔG° / RT)
pub fn equilibrium_constant(delta_g_standard: f64, temperature_k: f64) -> f64 {
    (-delta_g_standard / (R * temperature_k)).exp()
}

/// Arrhenius rate constant: k = A * exp(-Ea / RT)
pub fn arrhenius_rate(a: f64, ea_joules: f64, temperature_k: f64) -> f64 {
    a * (-ea_joules / (R * temperature_k)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn gibbs_sign() {
        // Exothermic (ΔH < 0) and positive ΔS → always spontaneous
        let dg = gibbs_free_energy(-100e3, 300.0, 100.0);
        assert!(dg < 0.0);
    }
}
