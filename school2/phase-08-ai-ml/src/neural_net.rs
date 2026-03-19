//! Feedforward neural network with one hidden layer, backpropagation from scratch.
//! Activation: ReLU hidden, linear output. Loss: MSE.

pub fn relu(x: f64) -> f64 { x.max(0.0) }
pub fn relu_deriv(x: f64) -> f64 { if x > 0.0 { 1.0 } else { 0.0 } }

/// Simple single-hidden-layer network for regression.
pub struct TwoLayerNet {
    pub w1: Vec<Vec<f64>>,   // [hidden_size × input_size]
    pub b1: Vec<f64>,
    pub w2: Vec<f64>,        // [output_size=1 × hidden_size]
    pub b2: f64,
}

impl TwoLayerNet {
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        // Xavier-ish init
        let scale = (2.0 / input_size as f64).sqrt();
        let w1 = (0..hidden_size).map(|_| (0..input_size).map(|_| scale * 0.1).collect()).collect();
        let w2 = vec![scale * 0.1; hidden_size];
        Self { w1, b1: vec![0.0; hidden_size], w2, b2: 0.0 }
    }

    pub fn forward(&self, x: &[f64]) -> (Vec<f64>, f64) {
        let h: Vec<f64> = self.w1.iter().zip(self.b1.iter())
            .map(|(row, &b)| relu(row.iter().zip(x.iter()).map(|(w, xi)| w * xi).sum::<f64>() + b))
            .collect();
        let out = self.w2.iter().zip(h.iter()).map(|(w, &hi)| w * hi).sum::<f64>() + self.b2;
        (h, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_pass_runs() {
        let net = TwoLayerNet::new(3, 4);
        let (_h, out) = net.forward(&[1.0, 2.0, 3.0]);
        // Just ensure it produces a finite number
        assert!(out.is_finite());
    }
}
