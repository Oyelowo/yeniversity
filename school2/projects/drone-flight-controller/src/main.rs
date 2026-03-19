//! Quadrotor flight controller simulation.
//!
//! Models a simplified quadrotor in hover:
//! - 6-DOF rigid body dynamics (translational + rotational)
//! - Cascade PID: altitude → throttle, attitude → torques
//! - Runs a step-response simulation to show climb & level-off

use nalgebra::Vector3;

// ────────────────────────────── physical model ──────────────────────────────

const MASS: f64 = 1.5;   // kg
const G: f64 = 9.81;     // m/s²
const IXX: f64 = 0.02;   // kg·m²
const IYY: f64 = 0.02;
const IZZ: f64 = 0.04;

#[derive(Default, Clone)]
struct State {
    pos: Vector3<f64>,   // x, y, z  (z up)
    vel: Vector3<f64>,
    euler: Vector3<f64>, // roll, pitch, yaw (rad)
    omega: Vector3<f64>, // body angular rates
}

fn dynamics(s: &State, thrust: f64, torques: Vector3<f64>, dt: f64) -> State {
    let (phi, theta, _psi) = (s.euler[0], s.euler[1], s.euler[2]);
    // Forces in world frame
    let az = thrust / MASS - G;
    let ax = thrust / MASS * (phi.sin() * _psi.sin() + theta.cos() * phi.cos() * _psi.cos());
    let ay = thrust / MASS * (-phi.sin() * _psi.cos() + theta.cos() * phi.cos() * _psi.sin());

    let mut next = s.clone();
    next.vel  = s.vel  + Vector3::new(ax, ay, az) * dt;
    next.pos  = s.pos  + s.vel * dt;
    next.omega = s.omega + Vector3::new(
        torques[0] / IXX,
        torques[1] / IYY,
        torques[2] / IZZ,
    ) * dt;
    next.euler = s.euler + s.omega * dt;
    next
}

// ─────────────────────────────── PID helpers ────────────────────────────────

struct Pid1 { kp: f64, ki: f64, kd: f64, integ: f64, prev: f64, min: f64, max: f64 }
impl Pid1 {
    fn new(kp: f64, ki: f64, kd: f64, min: f64, max: f64) -> Self {
        Self { kp, ki, kd, integ: 0.0, prev: 0.0, min, max }
    }
    fn step(&mut self, err: f64, dt: f64) -> f64 {
        let d = (err - self.prev) / dt;
        let raw = self.kp * err + self.ki * self.integ + self.kd * d;
        let out = raw.clamp(self.min, self.max);
        if raw == out { self.integ += err * dt; }
        self.prev = err;
        out
    }
}

// ──────────────────────────────── main loop ─────────────────────────────────

fn main() {
    let target_altitude = 5.0_f64;
    let dt = 0.01_f64;

    let mut state = State::default();
    let mut alt_pid   = Pid1::new(4.0, 0.5, 2.0,  0.0, 3.0 * G * MASS);
    let mut roll_pid  = Pid1::new(8.0, 0.1, 1.0, -0.5, 0.5);
    let mut pitch_pid = Pid1::new(8.0, 0.1, 1.0, -0.5, 0.5);

    println!("t      z       vz     roll   pitch");
    for step in 0..1000 {
        let t = step as f64 * dt;

        let thrust = MASS * G + alt_pid.step(target_altitude - state.pos[2], dt);
        let tau_roll  = roll_pid.step(-state.euler[0], dt);
        let tau_pitch = pitch_pid.step(-state.euler[1], dt);
        let torques = Vector3::new(tau_roll, tau_pitch, 0.0);

        state = dynamics(&state, thrust, torques, dt);

        if step % 100 == 0 {
            println!("{:.1}    {:.3}   {:.3}   {:.3}  {:.3}",
                t, state.pos[2], state.vel[2], state.euler[0].to_degrees(), state.euler[1].to_degrees());
        }
    }
    println!("\nFinal altitude: {:.3} m  (target: {} m)", state.pos[2], target_altitude);
}
