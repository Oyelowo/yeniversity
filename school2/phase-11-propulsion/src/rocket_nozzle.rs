//! de Laval (converging-diverging) rocket nozzle: isentropic relations.

/// Exit Mach number given area ratio A_e/A_t for a supersonic nozzle.
/// Numerically solved by iterating on the isentropic area relation.
/// A/A* = (1/M)[(2/(γ+1))(1 + (γ-1)/2 · M²)]^((γ+1)/(2(γ-1)))
pub fn exit_mach_from_area_ratio(area_ratio: f64, gamma: f64) -> f64 {
    let exp = (gamma + 1.0) / (2.0 * (gamma - 1.0));
    let g1 = (gamma + 1.0) / 2.0;

    // A/A*(M) = (1/M) * [(1 + (γ-1)/2 · M²) / ((γ+1)/2)]^((γ+1)/(2(γ-1)))
    // d(A/A*)/dM = A/A* · [ -1/M  +  exp·(γ-1)·M / (1 + (γ-1)/2·M²) ]
    let area_mach = |m: f64| -> f64 {
        (1.0 / m) * ((1.0 + (gamma - 1.0) / 2.0 * m * m) / g1).powf(exp)
    };

    // Newton's method starting from a supersonic initial guess
    let mut m = 2.0_f64;
    for _ in 0..100 {
        let a = area_mach(m);
        let denom = 1.0 + (gamma - 1.0) / 2.0 * m * m;
        let da_dm = a * (-1.0 / m + exp * (gamma - 1.0) * m / denom);
        let f = a - area_ratio;
        if f.abs() < 1e-12 { break; }
        m -= f / da_dm;
        m = m.max(1.001); // stay supersonic
    }
    m
}

/// Effective exhaust velocity c* (characteristic velocity):
/// c* = (1/Cf) · (F / Ṁ) — here, dimensional: c* = sqrt(γ R T0) / Γ
/// Γ = sqrt(γ) * (2/(γ+1))^((γ+1)/(2(γ-1)))
pub fn c_star(gamma: f64, r: f64, t0: f64) -> f64 {
    let gam_const = (gamma * (2.0 / (gamma + 1.0)).powf((gamma + 1.0) / (gamma - 1.0))).sqrt();
    (r * t0).sqrt() / gam_const
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn area_ratio_2_gives_supersonic_mach() {
        // A/A* = 2.0, γ=1.4 → supersonic Mach ≈ 2.197
        let m = exit_mach_from_area_ratio(2.0, 1.4);
        assert_abs_diff_eq!(m, 2.197, epsilon = 1e-2);
    }
}
