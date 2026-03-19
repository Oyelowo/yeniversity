//! Probability and statistical utilities.

use rand::prelude::*;
use rand_distr::{Normal, Distribution};

/// Estimate E[f(X)] where X ~ Normal(mu, sigma) using Monte Carlo.
pub fn monte_carlo_expectation<F>(f: F, mu: f64, sigma: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let dist = Normal::new(mu, sigma).expect("invalid normal params");
    let mut rng = thread_rng();
    (0..n).map(|_| f(dist.sample(&mut rng))).sum::<f64>() / n as f64
}

/// Sample mean of a slice.
pub fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// Sample variance (unbiased, Bessel-corrected).
pub fn variance(xs: &[f64]) -> f64 {
    let m = mean(xs);
    xs.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / (xs.len() - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn mean_and_variance_known_data() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_abs_diff_eq!(mean(&data), 5.0, epsilon = 1e-10);
        assert_abs_diff_eq!(variance(&data), 4.571_428_571, epsilon = 1e-6);
    }

    #[test]
    fn mc_expectation_of_normal_squared() {
        // E[X²] for X ~ N(0,1) = Var(X) + E[X]² = 1
        let result = monte_carlo_expectation(|x| x * x, 0.0, 1.0, 1_000_000);
        assert_abs_diff_eq!(result, 1.0, epsilon = 0.01);
    }
}
