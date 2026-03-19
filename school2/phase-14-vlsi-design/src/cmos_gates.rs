//! CMOS gate static analysis: trip point, noise margins, sizing.

/// CMOS inverter switching threshold (approximate symmetric sizing: kn=kp).
/// V_M ≈ VDD/2 when (W/L)_p / (W/L)_n = μn/μp ≈ 2.5
pub fn inverter_trip_point(vdd: f64) -> f64 { vdd / 2.0 }

/// Noise margin high: NMH = VOH - VIH. Very simplified model.
pub fn noise_margin_high(voh: f64, vih: f64) -> f64 { voh - vih }

/// Noise margin low: NML = VIL - VOL.
pub fn noise_margin_low(vil: f64, vol: f64) -> f64 { vil - vol }

/// CMOS NAND2: logic output (true = logic-1).
pub fn nand2(a: bool, b: bool) -> bool { !(a && b) }

/// CMOS NOR2: logic output.
pub fn nor2(a: bool, b: bool) -> bool { !(a || b) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand2_truth_table() {
        assert!(nand2(false, false));
        assert!(nand2(true, false));
        assert!(nand2(false, true));
        assert!(!nand2(true, true));
    }

    #[test]
    fn nor2_truth_table() {
        assert!(nor2(false, false));
        assert!(!nor2(true, false));
        assert!(!nor2(false, true));
        assert!(!nor2(true, true));
    }
}
