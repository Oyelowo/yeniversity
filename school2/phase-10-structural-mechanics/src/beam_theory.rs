//! Euler-Bernoulli beam theory: deflection formulas, shear and moment diagrams.

/// Max deflection of a simply-supported beam with central point load P.
/// δ_max = PL³/(48EI)
pub fn ss_beam_central_load_deflection(p: f64, l: f64, e: f64, i: f64) -> f64 {
    p * l.powi(3) / (48.0 * e * i)
}

/// Max deflection of a cantilever beam with tip load P.
/// δ_max = PL³/(3EI)
pub fn cantilever_tip_deflection(p: f64, l: f64, e: f64, i: f64) -> f64 {
    p * l.powi(3) / (3.0 * e * i)
}

/// Section modulus for a solid rectangular cross-section b×h (bending about neutral axis).
/// S = bh²/6
pub fn rect_section_modulus(b: f64, h: f64) -> f64 { b * h * h / 6.0 }

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn steel_cantilever_tip_deflection() {
        let p = 1000.0_f64; // 1 kN
        let l = 1.0_f64;    // 1 m
        let e = 200e9_f64;  // steel
        let i = 8.333e-6_f64; // I for 0.1m × 0.05m rectangle  bh³/12
        let delta = cantilever_tip_deflection(p, l, e, i);
        // ≈ 2e-3 m
        assert!(delta > 0.0 && delta < 0.01);
    }
}
