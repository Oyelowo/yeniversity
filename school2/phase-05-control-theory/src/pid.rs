//! PID controller with anti-windup clamping.

/// Discrete-time PID controller state.
pub struct Pid {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub output_min: f64,
    pub output_max: f64,
    integrator: f64,
    prev_error: f64,
}

impl Pid {
    pub fn new(kp: f64, ki: f64, kd: f64, output_min: f64, output_max: f64) -> Self {
        Self { kp, ki, kd, output_min, output_max, integrator: 0.0, prev_error: 0.0 }
    }

    /// Step the controller: returns clamped output, advances state.
    pub fn step(&mut self, setpoint: f64, measurement: f64, dt: f64) -> f64 {
        let error = setpoint - measurement;
        let derivative = (error - self.prev_error) / dt;
        let raw = self.kp * error + self.ki * self.integrator + self.kd * derivative;
        let output = raw.clamp(self.output_min, self.output_max);
        // Anti-windup: only integrate when not saturated
        if raw == output {
            self.integrator += error * dt;
        }
        self.prev_error = error;
        output
    }

    pub fn reset(&mut self) {
        self.integrator = 0.0;
        self.prev_error = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn proportional_only_reaches_setpoint() {
        let mut pid = Pid::new(1.0, 0.0, 0.0, -100.0, 100.0);
        let setpoint = 10.0;
        let mut plant = 0.0_f64;
        let dt = 0.1;
        for _ in 0..200 {
            let u = pid.step(setpoint, plant, dt);
            plant += u * dt; // integrating plant: dx/dt = u
        }
        assert_abs_diff_eq!(plant, setpoint, epsilon = 0.5);
    }
}
