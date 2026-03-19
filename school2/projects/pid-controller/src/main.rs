//! PID Controller demo: step response on a first-order plant.
//!
//! Plant: τ dx/dt = -x + Ku
//! PID tunes output to drive x → setpoint = 5.0.

struct Pid {
    kp: f64, ki: f64, kd: f64,
    min: f64, max: f64,
    integrator: f64,
    prev_error: f64,
}

impl Pid {
    fn new(kp: f64, ki: f64, kd: f64, min: f64, max: f64) -> Self {
        Self { kp, ki, kd, min, max, integrator: 0.0, prev_error: 0.0 }
    }

    fn step(&mut self, sp: f64, pv: f64, dt: f64) -> f64 {
        let e = sp - pv;
        let d = (e - self.prev_error) / dt;
        let raw = self.kp * e + self.ki * self.integrator + self.kd * d;
        let out = raw.clamp(self.min, self.max);
        if raw == out { self.integrator += e * dt; }
        self.prev_error = e;
        out
    }
}

fn main() {
    let setpoint = 5.0_f64;
    let tau = 1.0_f64;
    let k_plant = 1.0_f64;
    let dt = 0.05_f64;

    let mut pid = Pid::new(2.0, 1.5, 0.3, -20.0, 20.0);
    let mut x = 0.0_f64;
    let t_end = 8.0_f64;

    println!("t       u       x");
    let mut t = 0.0_f64;
    while t <= t_end {
        let u = pid.step(setpoint, x, dt);
        let dx = (-x + k_plant * u) / tau;
        x += dx * dt;
        if (t / 0.5).fract() < dt / 0.5 {
            println!("{:.2}    {:.3}   {:.3}", t, u, x);
        }
        t += dt;
    }
    println!("\nFinal x = {:.4}  (setpoint = {})", x, setpoint);
}
