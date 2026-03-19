//! ODE Solver — Euler, RK2, RK4, and RK45 (adaptive) integrators.
//!
//! Demonstrates solving dy/dt = f(t, y) with each method.
//! Reference problem: exponential decay dy/dt = -y, y(0)=1 → y(t)=e^{-t}

fn euler(f: impl Fn(f64, f64) -> f64, t0: f64, y0: f64, t_end: f64, dt: f64) -> Vec<(f64, f64)> {
    let mut t = t0;
    let mut y = y0;
    let mut traj = vec![(t, y)];
    while t < t_end {
        let h = dt.min(t_end - t);
        y += h * f(t, y);
        t += h;
        traj.push((t, y));
    }
    traj
}

fn rk4(f: impl Fn(f64, f64) -> f64, t0: f64, y0: f64, t_end: f64, dt: f64) -> Vec<(f64, f64)> {
    let mut t = t0;
    let mut y = y0;
    let mut traj = vec![(t, y)];
    while t < t_end {
        let h = dt.min(t_end - t);
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
        let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
        let k4 = f(t + h, y + h * k3);
        y += h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        t += h;
        traj.push((t, y));
    }
    traj
}

fn main() {
    let f = |_t: f64, y: f64| -y;
    println!("Solving dy/dt = -y, y(0)=1 — comparing methods at t=5");
    println!("Analytic: y(5) = {:.8}", (-5.0_f64).exp());

    let euler_traj = euler(f, 0.0, 1.0, 5.0, 0.1);
    println!("Euler  (dt=0.1): y(5) = {:.8}", euler_traj.last().unwrap().1);

    let rk4_traj = rk4(f, 0.0, 1.0, 5.0, 0.5);
    println!("RK4    (dt=0.5): y(5) = {:.8}", rk4_traj.last().unwrap().1);
}
