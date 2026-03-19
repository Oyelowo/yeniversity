//! Wave mechanics: travelling waves, superposition, string simulation.

use std::f64::consts::PI;

/// Displacement of a travelling sinusoidal wave at position x and time t.
/// y(x,t) = A cos(kx - ωt + φ₀)
pub fn wave_displacement(amplitude: f64, k: f64, omega: f64, x: f64, t: f64, phi0: f64) -> f64 {
    amplitude * (k * x - omega * t + phi0).cos()
}

/// Standing wave from superposition of +x and -x travelling waves of equal amplitude.
/// y_standing(x,t) = 2A cos(kx) cos(ωt)
pub fn standing_wave(amplitude: f64, k: f64, omega: f64, x: f64, t: f64) -> f64 {
    2.0 * amplitude * (k * x).cos() * (omega * t).cos()
}

/// Angular frequency for a wave on a string: ω = k * v = k * sqrt(T/μ)
/// T = tension [N], mu = linear mass density [kg/m]
pub fn string_wave_speed(tension: f64, linear_density: f64) -> f64 {
    (tension / linear_density).sqrt()
}

/// Allowed frequencies for a string of length L with fixed ends: f_n = n v / 2L
pub fn harmonic_frequency(n: u32, length: f64, wave_speed: f64) -> f64 {
    n as f64 * wave_speed / (2.0 * length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn fundamental_frequency() {
        let v = string_wave_speed(100.0, 0.01); // 100 m/s
        let f1 = harmonic_frequency(1, 1.0, v); // 50 Hz for 1 m string
        assert_abs_diff_eq!(f1, 50.0, epsilon = 1e-10);
    }
}
