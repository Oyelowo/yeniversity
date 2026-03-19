//! Semiconductor physics: carrier densities, p-n junction, diode equation.

const K_B: f64 = 8.617_333_262e-5; // eV/K

/// Intrinsic carrier density for silicon: n_i(T) using a simplified exponential model.
/// n_i ≈ n_i_ref * exp(-Eg / (2 k_B T))  (rough approximation for illustration)
/// Returns n_i in m^-3.
pub fn intrinsic_carrier_density(temperature_k: f64) -> f64 {
    const N_C_N_V_SQRT: f64 = 2.8e25; // √(Nc * Nv) for Si at 300K [m^-3] (approx)
    const EG_EV: f64 = 1.12;          // Silicon band gap at 300 K [eV]
    N_C_N_V_SQRT * (-EG_EV / (2.0 * K_B * temperature_k)).exp()
}

/// Thermal voltage V_T = k_B T / q  [Volts]
pub fn thermal_voltage(temperature_k: f64) -> f64 {
    K_B * temperature_k // in volts (since K_B is in eV/K, eV/charge = V)
}

/// Ideal diode current: I = I_s * (exp(V / V_T) - 1)
pub fn diode_current(vd: f64, i_sat: f64, temperature_k: f64) -> f64 {
    let vt = thermal_voltage(temperature_k);
    i_sat * ((vd / vt).exp() - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn thermal_voltage_at_300k() {
        // V_T ≈ 25.85 mV at 300 K
        assert_abs_diff_eq!(thermal_voltage(300.0), 0.02585, epsilon = 0.0001);
    }
}
