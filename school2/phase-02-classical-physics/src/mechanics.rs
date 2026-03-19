//! Newtonian and Lagrangian mechanics simulations.

use nalgebra::Vector3;

/// State of a particle: position and velocity.
#[derive(Clone, Debug)]
pub struct ParticleState {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub mass: f64,
}

impl ParticleState {
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.norm_squared()
    }
}

/// Gravitational acceleration between two point masses.
const G: f64 = 6.674e-11;

pub fn gravitational_acceleration(
    pos_self: Vector3<f64>,
    pos_other: Vector3<f64>,
    mass_other: f64,
) -> Vector3<f64> {
    let r = pos_other - pos_self;
    let dist = r.norm();
    if dist < 1e-10 {
        return Vector3::zeros();
    }
    G * mass_other / dist.powi(3) * r
}

/// Integrate an N-body system one step using RK4.
/// `positions[i]` and `velocities[i]` are 3-vectors; `masses[i]` is the mass.
pub fn nbody_rk4_step(
    positions: &mut Vec<Vector3<f64>>,
    velocities: &mut Vec<Vector3<f64>>,
    masses: &[f64],
    dt: f64,
) {
    let n = positions.len();

    // Compute acceleration from current configuration
    let accel = |pos: &[Vector3<f64>]| -> Vec<Vector3<f64>> {
        (0..n)
            .map(|i| {
                (0..n)
                    .filter(|&j| j != i)
                    .map(|j| gravitational_acceleration(pos[i], pos[j], masses[j]))
                    .fold(Vector3::zeros(), |a, b| a + b)
            })
            .collect()
    };

    let vel0 = velocities.clone();
    let pos0 = positions.clone();

    let a1 = accel(&pos0);
    let pos_k2: Vec<_> = (0..n).map(|i| pos0[i] + vel0[i] * (dt / 2.0)).collect();
    let vel_k2: Vec<_> = (0..n).map(|i| vel0[i] + a1[i] * (dt / 2.0)).collect();

    let a2 = accel(&pos_k2);
    let pos_k3: Vec<_> = (0..n).map(|i| pos0[i] + vel_k2[i] * (dt / 2.0)).collect();
    let vel_k3: Vec<_> = (0..n).map(|i| vel0[i] + a2[i] * (dt / 2.0)).collect();

    let a3 = accel(&pos_k3);
    let pos_k4: Vec<_> = (0..n).map(|i| pos0[i] + vel_k3[i] * dt).collect();
    let vel_k4: Vec<_> = (0..n).map(|i| vel0[i] + a3[i] * dt).collect();

    let a4 = accel(&pos_k4);

    for i in 0..n {
        positions[i] =
            pos0[i] + dt / 6.0 * (vel0[i] + 2.0 * vel_k2[i] + 2.0 * vel_k3[i] + vel_k4[i]);
        velocities[i] =
            vel0[i] + dt / 6.0 * (a1[i] + 2.0 * a2[i] + 2.0 * a3[i] + a4[i]);
    }
}

/// Simple harmonic oscillator: returns (x(t), v(t)) given initial conditions.
pub fn sho_analytic(x0: f64, v0: f64, omega: f64, t: f64) -> (f64, f64) {
    let x = x0 * (omega * t).cos() + (v0 / omega) * (omega * t).sin();
    let v = -x0 * omega * (omega * t).sin() + v0 * (omega * t).cos();
    (x, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn sho_energy_conservation() {
        let omega = 2.0;
        let (x0, v0) = (1.0, 0.0);
        let e0 = 0.5 * (v0 * v0 + omega * omega * x0 * x0);
        for i in 0..100 {
            let t = i as f64 * 0.1;
            let (x, v) = sho_analytic(x0, v0, omega, t);
            let e = 0.5 * (v * v + omega * omega * x * x);
            assert_abs_diff_eq!(e, e0, epsilon = 1e-12);
        }
    }
}
