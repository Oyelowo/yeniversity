//! Quantum operators: Pauli matrices, commutators, spin-1/2 states.

use num_complex::Complex64;
type Matrix2c = nalgebra::Matrix2<Complex64>;

pub fn sigma_x() -> Matrix2c {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    Matrix2c::new(z, o, o, z)
}

pub fn sigma_y() -> Matrix2c {
    let z = Complex64::new(0.0, 0.0);
    let i = Complex64::new(0.0, 1.0);
    Matrix2c::new(z, -i, i, z)
}

pub fn sigma_z() -> Matrix2c {
    let z = Complex64::new(0.0, 0.0);
    let o = Complex64::new(1.0, 0.0);
    Matrix2c::new(o, z, z, -o)
}

/// Commutator [A, B] = AB − BA
pub fn commutator(a: &Matrix2c, b: &Matrix2c) -> Matrix2c {
    a * b - b * a
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn pauli_commutation_relations() {
        // [σ_x, σ_y] = 2i σ_z
        let comm = commutator(&sigma_x(), &sigma_y());
        let expected = sigma_z() * Complex64::new(0.0, 2.0);
        for (a, b) in comm.iter().zip(expected.iter()) {
            assert_abs_diff_eq!(a.re, b.re, epsilon = 1e-12);
            assert_abs_diff_eq!(a.im, b.im, epsilon = 1e-12);
        }
    }
}
