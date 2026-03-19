//! FMEA: Risk Priority Number = Severity × Occurrence × Detection

#[derive(Debug, Clone)]
pub struct FmeaEntry {
    pub function: String,
    pub failure_mode: String,
    pub effect: String,
    pub severity: u8,     // 1-10
    pub occurrence: u8,   // 1-10
    pub detection: u8,    // 1-10
}

impl FmeaEntry {
    pub fn rpn(&self) -> u32 {
        self.severity as u32 * self.occurrence as u32 * self.detection as u32
    }
}

/// Sort FMEA entries by RPN descending (highest risk first).
pub fn rank_by_rpn(entries: &mut Vec<FmeaEntry>) {
    entries.sort_by(|a, b| b.rpn().cmp(&a.rpn()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpn_calculation() {
        let e = FmeaEntry {
            function: "Fuel pump".into(),
            failure_mode: "No flow".into(),
            effect: "Engine flameout".into(),
            severity: 9, occurrence: 3, detection: 4,
        };
        assert_eq!(e.rpn(), 108);
    }
}
