//! 1-D quantum mechanics: particle in an infinite square well.

use std::f64::consts::PI;

// Physical constants (SI)
const HBAR: f64 = 1.054_571_817e-34; // J·s
const M_E: f64 = 9.109_383_7015e-31; // kg (electron mass)

/// Energy eigenvalue for a particle in an infinite square well of width L (metres).
/// E_n = (n²π²ℏ²)/(2mL²), n = 1, 2, 3, …
pub fn infinite_well_energy(n: u32, l: f64, mass: f64) -> f64 {
    let n = n as f64;
    (n * n * PI * PI * HBAR * HBAR) / (2.0 * mass * l * l)
}

/// Normalised wavefunction ψ_n(x) for an infinite square well [0, L].
pub fn infinite_well_psi(n: u32, x: f64, l: f64) -> f64 {
    (2.0 / l).sqrt() * ((n as f64 * PI * x) / l).sin()
}

/// Expectation value ⟨x⟩ = ∫ψ*xψ dx via trapezoidal rule over N points.
pub fn expectation_position(n: u32, l: f64, num_points: usize) -> f64 {
    let dx = l / (num_points as f64);
    (0..=num_points)
        .map(|i| {
            let x = i as f64 * dx;
            let psi = infinite_well_psi(n, x, l);
            psi * psi * x
        })
        .sum::<f64>()
        * dx
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ground_state_expectation_x_is_l_over_2() {
        // By symmetry, ⟨x⟩ = L/2 for all n
        let l = 1e-9; // 1 nm well
        let expected = l / 2.0;
        let computed = expectation_position(1, l, 10_000);
        // Relative tolerance 0.1%
        assert_abs_diff_eq!(computed / l, expected / l, epsilon = 1e-3);
    }

    #[test]
    fn energy_levels_scale_as_n_squared() {
        let l = 1e-9;
        let e1 = infinite_well_energy(1, l, M_E);
        let e2 = infinite_well_energy(2, l, M_E);
        assert_abs_diff_eq!(e2 / e1, 4.0, epsilon = 1e-10);
    }
}
