//! LTI state-space model: ẋ = Ax + Bu, y = Cx + Du
//! Forward Euler integration, eigenvalue stability check.

use nalgebra::{DMatrix, DVector};

pub struct StateSpace {
    pub a: DMatrix<f64>,
    pub b: DMatrix<f64>,
    pub c: DMatrix<f64>,
    pub d: DMatrix<f64>,
    pub state: DVector<f64>,
}

impl StateSpace {
    pub fn new(a: DMatrix<f64>, b: DMatrix<f64>, c: DMatrix<f64>, d: DMatrix<f64>) -> Self {
        let n = a.nrows();
        Self { a, b, c, d, state: DVector::zeros(n) }
    }

    /// Advance one Euler step; returns output y.
    pub fn step(&mut self, u: &DVector<f64>, dt: f64) -> DVector<f64> {
        let x_dot = &self.a * &self.state + &self.b * u;
        self.state += x_dot * dt;
        &self.c * &self.state + &self.d * u
    }

    /// Returns eigenvalues (real). System is stable iff all < 0.
    pub fn eigenvalue_real_parts(&self) -> Vec<f64> {
        self.a.clone().eigenvalues()
            .map(|ev| ev.iter().copied().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_order_system_step_response() {
        // ẋ = -x + u  (time constant τ=1)
        let a = DMatrix::from_vec(1, 1, vec![-1.0]);
        let b = DMatrix::from_vec(1, 1, vec![1.0]);
        let c = DMatrix::from_vec(1, 1, vec![1.0]);
        let d = DMatrix::from_vec(1, 1, vec![0.0]);
        let mut sys = StateSpace::new(a, b, c, d);
        let u = DVector::from_vec(vec![1.0]);
        let dt = 0.01;
        let mut y = DVector::zeros(1);
        for _ in 0..500 { // 5 time constants
            y = sys.step(&u, dt);
        }
        // Steady-state output should be ≈ 1.0
        assert!((y[0] - 1.0).abs() < 0.01);
    }
}
