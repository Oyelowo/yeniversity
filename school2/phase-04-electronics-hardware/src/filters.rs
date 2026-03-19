//! Analog filters: RC low-pass/high-pass frequency response, IIR filter, simple FIR.

use num_complex::Complex64;
use std::f64::consts::PI;

/// Frequency response of an RC low-pass filter: H(jω) = 1 / (1 + jωRC)
pub fn rc_lowpass_response(freq_hz: f64, resistance: f64, capacitance: f64) -> Complex64 {
    let omega = 2.0 * PI * freq_hz;
    let tau = resistance * capacitance;
    Complex64::new(1.0, 0.0) / Complex64::new(1.0, omega * tau)
}

/// Magnitude response in dB.
pub fn magnitude_db(h: Complex64) -> f64 {
    20.0 * h.norm().log10()
}

/// Phase response in degrees.
pub fn phase_deg(h: Complex64) -> f64 {
    h.arg().to_degrees()
}

/// First-order IIR low-pass filter (direct form I).
/// alpha = RC / (RC + T_s); input samples → filtered output samples.
pub fn iir_lowpass(input: &[f64], alpha: f64) -> Vec<f64> {
    let mut output = Vec::with_capacity(input.len());
    let mut y_prev = 0.0_f64;
    for &x in input {
        let y = alpha * y_prev + (1.0 - alpha) * x;
        output.push(y);
        y_prev = y;
    }
    output
}

/// Simple moving-average (box) FIR filter of length `n`.
pub fn fir_moving_average(input: &[f64], n: usize) -> Vec<f64> {
    assert!(n >= 1);
    let n_f = n as f64;
    input
        .windows(n)
        .map(|w| w.iter().sum::<f64>() / n_f)
        .collect()
}

/// Cutoff frequency (–3 dB) of an RC LP filter: f_c = 1/(2π RC).
pub fn rc_cutoff_hz(resistance: f64, capacitance: f64) -> f64 {
    1.0 / (2.0 * PI * resistance * capacitance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn rc_response_at_cutoff_is_minus_3db() {
        let r = 1000.0;  // 1 kΩ
        let c = 159.15e-9; // ≈ 1 µF so f_c ≈ 1 kHz (1/(2π·1000·159.15e-9))
        let fc = rc_cutoff_hz(r, c);
        let h = rc_lowpass_response(fc, r, c);
        // |H(jω_c)| = 1/√2 → −3.01 dB
        assert_abs_diff_eq!(h.norm(), 1.0 / 2.0_f64.sqrt(), epsilon = 1e-3);
    }

    #[test]
    fn iir_steady_state_passes_dc() {
        let input: Vec<f64> = vec![1.0; 100];
        let output = iir_lowpass(&input, 0.9);
        // After enough samples, output should converge to 1.0
        assert_abs_diff_eq!(*output.last().unwrap(), 1.0, epsilon = 0.01);
    }

    #[test]
    fn fir_average_of_constant() {
        let input = vec![3.0; 10];
        let output = fir_moving_average(&input, 4);
        for y in &output {
            assert_abs_diff_eq!(*y, 3.0, epsilon = 1e-12);
        }
    }
}
