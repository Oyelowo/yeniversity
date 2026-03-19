//! Reliability engineering: MTTF, series/parallel reliability, availability.

/// Exponential reliability: R(t) = exp(–λt)
pub fn reliability(lambda: f64, t: f64) -> f64 { (-lambda * t).exp() }

/// MTTF for exponential component: MTTF = 1/λ
pub fn mttf(lambda: f64) -> f64 { 1.0 / lambda }

/// Series system reliability: R_s = Π R_i
pub fn series_reliability(reliabilities: &[f64]) -> f64 {
    reliabilities.iter().product()
}

/// Parallel (redundant) system reliability: R_p = 1 − Π(1 − R_i)
pub fn parallel_reliability(reliabilities: &[f64]) -> f64 {
    1.0 - reliabilities.iter().map(|&r| 1.0 - r).product::<f64>()
}

/// Availability: A = MTTF / (MTTF + MTTR)
pub fn availability(mttf: f64, mttr: f64) -> f64 {
    mttf / (mttf + mttr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn two_series_components() {
        // R1=0.9, R2=0.8 → R_s=0.72
        assert_abs_diff_eq!(series_reliability(&[0.9, 0.8]), 0.72, epsilon = 1e-12);
    }

    #[test]
    fn two_parallel_components() {
        // 1−(1−0.9)(1−0.8) = 1−0.02=0.98
        assert_abs_diff_eq!(parallel_reliability(&[0.9, 0.8]), 0.98, epsilon = 1e-12);
    }
}
