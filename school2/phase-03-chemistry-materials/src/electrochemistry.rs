//! Electrochemistry: Nernst equation, cell EMF.

const R: f64 = 8.314_462_618; // J/(mol·K)
const F: f64 = 96_485.332_12; // C/mol (Faraday constant)

/// Nernst equation: E = E° - (RT / nF) * ln(Q)
/// e_standard: standard cell potential [V]
/// n: number of electrons transferred
/// q: reaction quotient (dimensionless)
/// temperature_k: temperature in Kelvin
pub fn cell_emf(e_standard: f64, n: f64, q: f64, temperature_k: f64) -> f64 {
    e_standard - (R * temperature_k / (n * F)) * q.ln()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn nernst_at_standard_conditions() {
        // When Q = 1, ln(Q) = 0, so E = E°
        assert_abs_diff_eq!(cell_emf(1.1, 2.0, 1.0, 298.15), 1.1, epsilon = 1e-12);
    }
}
