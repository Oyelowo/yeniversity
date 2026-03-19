//! Discrete-time signals: DFT, FFT, convolution.

use num_complex::Complex64;
use std::f64::consts::PI;

/// Naive O(N²) DFT. For education only — use `fft` below for real work.
pub fn dft(x: &[f64]) -> Vec<Complex64> {
    let n = x.len();
    (0..n)
        .map(|k| {
            x.iter()
                .enumerate()
                .map(|(m, &xm)| {
                    let angle = -2.0 * PI * k as f64 * m as f64 / n as f64;
                    xm * Complex64::new(angle.cos(), angle.sin())
                })
                .sum()
        })
        .collect()
}

/// Cooley-Tukey radix-2 FFT. Input length must be a power of two.
pub fn fft(x: &[Complex64]) -> Vec<Complex64> {
    let n = x.len();
    if n == 1 {
        return x.to_vec();
    }
    assert!(n.is_power_of_two(), "FFT length must be a power of two");

    let even: Vec<Complex64> = x.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex64> = x.iter().skip(1).step_by(2).cloned().collect();

    let e = fft(&even);
    let o = fft(&odd);

    let mut out = vec![Complex64::default(); n];
    for k in 0..(n / 2) {
        let twiddle = Complex64::from_polar(1.0, -2.0 * PI * k as f64 / n as f64);
        out[k] = e[k] + twiddle * o[k];
        out[k + n / 2] = e[k] - twiddle * o[k];
    }
    out
}

/// Discrete linear convolution of two real sequences.
pub fn convolve(x: &[f64], h: &[f64]) -> Vec<f64> {
    let n = x.len() + h.len() - 1;
    let mut y = vec![0.0; n];
    for (i, &xi) in x.iter().enumerate() {
        for (j, &hj) in h.iter().enumerate() {
            y[i + j] += xi * hj;
        }
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn fft_matches_dft_for_real_signal() {
        let x: Vec<f64> = (0..8).map(|i| (i as f64).sin()).collect();
        let dft_result = dft(&x);
        let fft_result = fft(&x.iter().map(|&v| Complex64::new(v, 0.0)).collect::<Vec<_>>());
        for (d, f) in dft_result.iter().zip(fft_result.iter()) {
            assert_abs_diff_eq!(d.re, f.re, epsilon = 1e-10);
            assert_abs_diff_eq!(d.im, f.im, epsilon = 1e-10);
        }
    }
}
