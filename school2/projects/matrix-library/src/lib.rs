//! Matrix library — from-scratch dense matrix implementation (no external deps for core ops).
//!
//! Implements: addition, subtraction, scalar multiply, transpose, matmul,
//!             LU decomposition, determinant, inverse, Frobenius norm.

mod matrix;
pub use matrix::Matrix;
