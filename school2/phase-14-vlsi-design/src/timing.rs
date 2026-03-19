//! Timing analysis: Elmore delay, setup/hold budget.

/// Elmore delay for an RC chain: τ = Σ Rk · (sum of C downstream of k)
/// Simple 2-segment chain: R1-C1-R2-C2
pub fn elmore_delay_2seg(r1: f64, c1: f64, r2: f64, c2: f64) -> f64 {
    r1 * (c1 + c2) + r2 * c2
}

/// Setup slack at a flip-flop: slack = T_clk − T_data_arrival − T_setup
pub fn setup_slack(t_clock: f64, t_data_arrival: f64, t_setup: f64) -> f64 {
    t_clock - t_data_arrival - t_setup
}
