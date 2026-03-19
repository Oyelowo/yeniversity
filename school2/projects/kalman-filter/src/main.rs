//! Linear Kalman Filter demo: tracking a 1-D object with constant velocity.
//!
//! State: [position, velocity]ᵀ
//! Measurement: noisy position only.

use nalgebra::{DMatrix, DVector};

struct Kf {
    f: DMatrix<f64>,
    h: DMatrix<f64>,
    q: DMatrix<f64>,
    r: DMatrix<f64>,
    x: DVector<f64>,
    p: DMatrix<f64>,
}

impl Kf {
    fn predict(&mut self) {
        self.x = &self.f * &self.x;
        self.p = &self.f * &self.p * self.f.transpose() + &self.q;
    }
    fn update(&mut self, z: f64) {
        let z_vec = DVector::from_vec(vec![z]);
        let s = &self.h * &self.p * self.h.transpose() + &self.r;
        let k = &self.p * self.h.transpose() * s.try_inverse().unwrap();
        self.x = &self.x + &k * (z_vec - &self.h * &self.x);
        self.p = (DMatrix::identity(2, 2) - &k * &self.h) * &self.p;
    }
}

fn main() {
    let dt = 0.1_f64;
    // Constant-velocity model
    let f = DMatrix::from_vec(2, 2, vec![1.0, 0.0, dt, 1.0]);
    let h = DMatrix::from_vec(1, 2, vec![1.0, 0.0]);
    let q = DMatrix::from_iterator(2, 2, vec![0.01, 0.0, 0.0, 0.01]);
    let r = DMatrix::from_vec(1, 1, vec![0.5]);
    let x0 = DVector::zeros(2);
    let p0 = DMatrix::identity(2, 2);

    let mut kf = Kf { f, h, q, r, x: x0, p: p0 };

    // Simulate: true position = 2.0 m/s * t
    let noisy_obs = [0.1, 0.3, 0.55, 0.85, 1.05, 1.22, 1.41, 1.59, 1.78, 2.01];
    println!("t     z_noisy  x_est   v_est");
    for (i, &z) in noisy_obs.iter().enumerate() {
        let t = (i + 1) as f64 * dt;
        kf.predict();
        kf.update(z);
        println!("{:.1}   {:.3}    {:.3}   {:.3}", t, z, kf.x[0], kf.x[1]);
    }
}
