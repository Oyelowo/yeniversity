//! Solid state physics stubs: Bloch's theorem, band gap concept, density of states.

/// Reduced zone scheme: fold a k-value into the first Brillouin zone [-π/a, π/a].
pub fn fold_into_first_bz(k: f64, lattice_const: f64) -> f64 {
    let g = 2.0 * std::f64::consts::PI / lattice_const;
    let mut k_red = k % g;
    if k_red > g / 2.0 { k_red -= g; }
    if k_red < -g / 2.0 { k_red += g; }
    k_red
}

/// Free-electron energy E = ℏ²k²/(2m) in eV (k in m⁻¹).
pub fn free_electron_energy_ev(k: f64) -> f64 {
    const HBAR: f64 = 1.054_571_817e-34;
    const M_E: f64 = 9.109_383_7015e-31;
    const EV: f64 = 1.602_176_634e-19;
    (HBAR * k).powi(2) / (2.0 * M_E * EV)
}
