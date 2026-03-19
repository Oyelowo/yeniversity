//! Fluid statics: hydrostatic pressure, buoyancy.

/// Hydrostatic pressure at depth h (metres) in a fluid of density ρ (kg/m³).
pub fn hydrostatic_pressure(rho: f64, g: f64, h: f64) -> f64 {
    rho * g * h
}

/// Buoyancy force on a submerged object of volume V in fluid of density ρ.
pub fn buoyancy_force(rho: f64, g: f64, volume: f64) -> f64 {
    rho * g * volume
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn archimedes_steel_cube_in_water() {
        // 1 cm³ cube submerged in water (ρ=1000 kg/m³)
        let fb = buoyancy_force(1000.0, 9.81, 1e-6);
        assert_abs_diff_eq!(fb, 9.81e-3, epsilon = 1e-10);
    }
}
