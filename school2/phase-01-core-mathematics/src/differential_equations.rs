//! ODE solvers: Euler, RK2, RK4.
//! See `projects/ode-solver` for the full featured implementation.

/// Single step of the explicit (forward) Euler method.
/// dy/dt = f(t, y);  y_{n+1} = y_n + h * f(t_n, y_n)
pub fn euler_step<F>(f: F, t: f64, y: f64, h: f64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    y + h * f(t, y)
}

/// Single step of 4th-order Runge-Kutta (RK4).
pub fn rk4_step<F>(f: F, t: f64, y: f64, h: f64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let k1 = f(t, y);
    let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
    let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
    let k4 = f(t + h, y + h * k3);
    y + h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}

/// Integrate an ODE from t0 to t_end using RK4.
/// Returns (times, values).
pub fn rk4_integrate<F>(f: F, t0: f64, y0: f64, t_end: f64, n_steps: usize) -> (Vec<f64>, Vec<f64>)
where
    F: Fn(f64, f64) -> f64,
{
    let h = (t_end - t0) / n_steps as f64;
    let mut t = t0;
    let mut y = y0;
    let mut ts = Vec::with_capacity(n_steps + 1);
    let mut ys = Vec::with_capacity(n_steps + 1);
    ts.push(t);
    ys.push(y);
    for _ in 0..n_steps {
        y = rk4_step(&f, t, y, h);
        t += h;
        ts.push(t);
        ys.push(y);
    }
    (ts, ys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn rk4_exponential_decay() {
        // dy/dt = -y,  y(0) = 1  =>  y(t) = e^{-t}
        let (_, ys) = rk4_integrate(|_t, y| -y, 0.0, 1.0, 5.0, 5000);
        let y5 = *ys.last().unwrap();
        assert_abs_diff_eq!(y5, (-5.0_f64).exp(), epsilon = 1e-10);
    }
}
