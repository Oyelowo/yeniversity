//! Discrete linear Kalman filter.
//!
//! State: x ∈ ℝⁿ,  Measurement: z ∈ ℝᵐ
//! Predict: x̂⁻ = F x̂,  P⁻ = F P Fᵀ + Q
//! Update:  K = P⁻ Hᵀ (H P⁻ Hᵀ + R)⁻¹
//!          x̂ = x̂⁻ + K(z − H x̂⁻),  P = (I − KH) P⁻

use nalgebra::{DMatrix, DVector};

pub struct KalmanFilter {
    pub f: DMatrix<f64>, // state transition
    pub h: DMatrix<f64>, // observation
    pub q: DMatrix<f64>, // process noise covariance
    pub r: DMatrix<f64>, // measurement noise covariance
    pub x: DVector<f64>, // state estimate
    pub p: DMatrix<f64>, // estimate covariance
}

impl KalmanFilter {
    pub fn new(
        f: DMatrix<f64>,
        h: DMatrix<f64>,
        q: DMatrix<f64>,
        r: DMatrix<f64>,
        x0: DVector<f64>,
        p0: DMatrix<f64>,
    ) -> Self {
        Self { f, h, q, r, x: x0, p: p0 }
    }

    /// Predict step (no control input).
    pub fn predict(&mut self) {
        self.x = &self.f * &self.x;
        self.p = &self.f * &self.p * self.f.transpose() + &self.q;
    }

    /// Update step given measurement z.
    pub fn update(&mut self, z: &DVector<f64>) {
        let s = &self.h * &self.p * self.h.transpose() + &self.r;
        let k = &self.p * self.h.transpose() * s.try_inverse().expect("S must be invertible");
        let innovation = z - &self.h * &self.x;
        self.x = &self.x + &k * innovation;
        let n = self.x.len();
        self.p = (DMatrix::identity(n, n) - &k * &self.h) * &self.p;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_position_with_noise_converges() {
        // Model: position constant, noisy observation
        let f = DMatrix::identity(1, 1);
        let h = DMatrix::identity(1, 1);
        let q = DMatrix::from_vec(1, 1, vec![0.0]);
        let r = DMatrix::from_vec(1, 1, vec![1.0]); // measurement noise σ=1
        let x0 = DVector::zeros(1);
        let p0 = DMatrix::from_vec(1, 1, vec![100.0]);
        let mut kf = KalmanFilter::new(f, h, q, r, x0, p0);
        let true_pos = 5.0_f64;
        for _ in 0..100 {
            kf.predict();
            kf.update(&DVector::from_vec(vec![true_pos]));
        }
        assert!((kf.x[0] - true_pos).abs() < 0.2);
    }
}
