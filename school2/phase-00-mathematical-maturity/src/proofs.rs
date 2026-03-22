//! Lesson 4 — Proof Techniques
//!
//! Computational witnesses for the four fundamental proof strategies:
//! direct proof, contrapositive, contradiction, and proof by cases.
//!
//! Each function verifies a mathematical claim over a given integer domain.
//! The tests below serve the same role as "checking your working" in pencil-and-paper proofs.

/// Returns true if n is even.
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// Returns true if n is odd.
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// Direct proof witness — E2(a): even + even = even.
///
/// For every pair (m, n) in domain × domain where both are even,
/// confirms m + n is even.
pub fn even_plus_even_is_even(domain: &[i64]) -> bool {
    for &m in domain {
        for &n in domain {
            if is_even(m) && is_even(n) {
                if !is_even(m + n) {
                    return false;
                }
            }
        }
    }
    true
}

/// Direct proof witness — E2(b): odd × odd = odd.
///
/// For every pair (m, n) in domain × domain where both are odd,
/// confirms m * n is odd.
pub fn odd_times_odd_is_odd(domain: &[i64]) -> bool {
    for &m in domain {
        for &n in domain {
            if is_odd(m) && is_odd(n) {
                if !is_odd(m * n) {
                    return false;
                }
            }
        }
    }
    true
}

/// Contrapositive witness — E3(a): n even → n² even.
///
/// This is the contrapositive of "n² even → n even".
/// Verifying the contrapositive is logically equivalent to the original claim.
pub fn even_square_contrapositive(domain: &[i64]) -> bool {
    domain
        .iter()
        .all(|&n| if is_even(n) { is_even(n * n) } else { true })
}

/// Cases witness — E5(a): n³ − n divisible by 3.
///
/// n(n−1)(n+1) is a product of three consecutive integers,
/// so one of them must be divisible by 3.
pub fn cube_minus_n_div3(domain: &[i64]) -> bool {
    domain.iter().all(|&n| (n * n * n - n) % 3 == 0)
}

/// Cases witness — E5(b): n² ≡ 0 or 1 (mod 4).
pub fn square_mod4(domain: &[i64]) -> bool {
    domain
        .iter()
        .all(|&n| (n * n) % 4 == 0 || (n * n) % 4 == 1)
}

/// Direct proof witness — E2(c): m even → m² divisible by 4.
pub fn even_square_div4(domain: &[i64]) -> bool {
    domain
        .iter()
        .all(|&m| if is_even(m) { (m * m) % 4 == 0 } else { true })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOMAIN: &[i64] = &[
        -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    #[test]
    fn even_plus_even() {
        assert!(even_plus_even_is_even(DOMAIN));
    }

    #[test]
    fn odd_times_odd() {
        assert!(odd_times_odd_is_odd(DOMAIN));
    }

    #[test]
    fn contrapositive_even_square() {
        assert!(even_square_contrapositive(DOMAIN));
    }

    #[test]
    fn cube_minus_n_divisible_by_3() {
        assert!(cube_minus_n_div3(DOMAIN));
    }

    #[test]
    fn square_residue_mod4() {
        assert!(square_mod4(DOMAIN));
    }

    #[test]
    fn even_square_divisible_by_4() {
        assert!(even_square_div4(DOMAIN));
    }
}
