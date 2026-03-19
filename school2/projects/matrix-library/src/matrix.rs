//! `Matrix<T>` — heap-allocated dense matrix (row-major), from scratch.

use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self { rows, cols, data }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::new(n, n);
        for i in 0..n { m[(i, i)] = 1.0; }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::new(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out[(c, r)] = self[(r, c)];
            }
        }
        out
    }

    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// LU decomposition (partial pivoting). Returns (L, U, perm) where perm[i] = original row.
    pub fn lu(&self) -> (Matrix, Matrix, Vec<usize>) {
        assert_eq!(self.rows, self.cols);
        let n = self.rows;
        let mut a = self.clone();
        let mut perm: Vec<usize> = (0..n).collect();

        for k in 0..n {
            // Find pivot
            let pivot = (k..n).max_by(|&i, &j| a[(i,k)].abs().partial_cmp(&a[(j,k)].abs()).unwrap()).unwrap();
            a.data.swap(k * n, pivot * n); // just a proxy — swap full rows
            for c in 0..n { let tmp = a[(k,c)]; a[(k,c)] = a[(pivot,c)]; a[(pivot,c)] = tmp; }
            perm.swap(k, pivot);

            for i in (k+1)..n {
                a[(i,k)] /= a[(k,k)];
                for j in (k+1)..n {
                    let val = a[(i,k)] * a[(k,j)];
                    a[(i,j)] -= val;
                }
            }
        }
        // Extract L and U
        let mut l = Matrix::identity(n);
        let mut u = Matrix::new(n, n);
        for r in 0..n {
            for c in 0..n {
                if r > c { l[(r,c)] = a[(r,c)]; }
                else { u[(r,c)] = a[(r,c)]; }
            }
        }
        (l, u, perm)
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (r, c): (usize, usize)) -> &f64 { &self.data[r * self.cols + c] }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut f64 { &mut self.data[r * self.cols + c] }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(mut self, rhs: Matrix) -> Matrix {
        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) { *a += b; }
        self
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.cols, rhs.rows);
        let mut out = Matrix::new(self.rows, rhs.cols);
        for r in 0..self.rows {
            for k in 0..self.cols {
                for c in 0..rhs.cols {
                    out[(r,c)] += self[(r,k)] * rhs[(k,c)];
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn identity_times_matrix_is_identity() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let i = Matrix::identity(2);
        let result = &i * &a;
        for r in 0..2 { for c in 0..2 { assert_abs_diff_eq!(result[(r,c)], a[(r,c)], epsilon=1e-12); } }
    }

    #[test]
    fn matmul_2x2() {
        // [1 2; 3 4] × [5 6; 7 8] = [19 22; 43 50]
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let c = &a * &b;
        assert_abs_diff_eq!(c[(0,0)], 19.0, epsilon=1e-12);
        assert_abs_diff_eq!(c[(1,1)], 50.0, epsilon=1e-12);
    }
}
