//! Engineering budget roll-ups: mass, power, link budget.

/// Mass budget: total mass = sum of component masses with margin.
pub fn mass_budget_with_margin(masses: &[f64], margin_fraction: f64) -> f64 {
    let total: f64 = masses.iter().sum();
    total * (1.0 + margin_fraction)
}

/// Power budget: peak load with contingency.
pub fn power_budget_with_contingency(loads: &[f64], contingency: f64) -> f64 {
    let total: f64 = loads.iter().sum();
    total + contingency
}

/// Link budget: received power (dBW) = Pt + Gt - FSPL - La + Gr
/// FSPL = 20 log10(4πd/λ)  in dB
pub fn free_space_path_loss_db(distance_m: f64, frequency_hz: f64) -> f64 {
    const C: f64 = 2.998e8;
    let lambda = C / frequency_hz;
    20.0 * (4.0 * std::f64::consts::PI * distance_m / lambda).log10()
}

pub fn received_power_dbw(pt_dbw: f64, gt_db: f64, fspl_db: f64, la_db: f64, gr_db: f64) -> f64 {
    pt_dbw + gt_db - fspl_db - la_db + gr_db
}
