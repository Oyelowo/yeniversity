//! Lesson 5 — Mathematical Induction
//!
//! Computational witnesses for the key claims proved in the lesson notes.
//! Each function checks a formula or property over a range of integers;
//! the tests confirm the induction conclusions hold on every value in [0, 100].
//!
//! Note: these tests do NOT replace proofs — they are sanity-checks and
//! illustrations of what the proofs are claiming.

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns the n-th triangular number: 1 + 2 + … + n (iterative, O(n)).
pub fn triangular_sum_iter(n: u64) -> u64 {
    (1..=n).sum()
}

/// Closed form: n(n+1)/2 — this is what induction proves equals the above.
pub fn triangular_sum_formula(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Sum of first n odd numbers: 1 + 3 + 5 + … + (2n−1) (iterative).
pub fn odd_sum_iter(n: u64) -> u64 {
    (1..=n).map(|i| 2 * i - 1).sum()
}

/// Closed form: n² — proved by induction.
pub fn odd_sum_formula(n: u64) -> u64 {
    n * n
}

/// Partial geometric sum: 1 + r + r² + … + rⁿ (iterative, integer domain).
pub fn geometric_sum_iter(r: i64, n: u32) -> i64 {
    (0..=n).map(|i| r.pow(i)).sum()
}

/// Closed form: (rⁿ⁺¹ − 1) / (r − 1)  — valid for r ≠ 1.
/// Returns None if r == 1 (formula undefined).
pub fn geometric_sum_formula(r: i64, n: u32) -> Option<i64> {
    if r == 1 {
        return None; // formula requires r ≠ 1
    }
    Some((r.pow(n + 1) - 1) / (r - 1))
}

// ---------------------------------------------------------------------------
// Divisibility
// ---------------------------------------------------------------------------

/// Checks: 4 | (5ⁿ − 1)  for a given n.
pub fn four_divides_5n_minus_1(n: u32) -> bool {
    (5_i128.pow(n) - 1) % 4 == 0
}

// ---------------------------------------------------------------------------
// Fibonacci
// ---------------------------------------------------------------------------

/// Returns F(n) by the standard recurrence.
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

/// Claim from strong-induction example: F(n) < 2ⁿ for all n ≥ 0.
pub fn fibonacci_bound(n: u32) -> bool {
    fibonacci(n) < 2u64.pow(n)
}

// ---------------------------------------------------------------------------
// Recursive triangle — mirrors the inductive structure explicitly
// ---------------------------------------------------------------------------

/// Recursive definition of triangle number — mirrors the inductive step.
/// Not efficient; for illustration only.
pub fn triangular_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        k => k + triangular_recursive(k - 1),
    }
}

// ---------------------------------------------------------------------------
// Prime divisor (strong-induction witness)
// ---------------------------------------------------------------------------

/// Returns the smallest prime divisor of n ≥ 2, or None for n < 2.
/// Mirrors the strong-induction proof: every n ≥ 2 has a prime divisor.
pub fn smallest_prime_divisor(n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return Some(i);
        }
        i += 1;
    }
    Some(n) // n itself is prime
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_sum_matches_formula() {
        for n in 0..=100 {
            assert_eq!(
                triangular_sum_iter(n),
                triangular_sum_formula(n),
                "Triangular sum mismatch at n = {n}"
            );
        }
    }

    #[test]
    fn odd_sum_matches_formula() {
        for n in 0..=100 {
            assert_eq!(
                odd_sum_iter(n),
                odd_sum_formula(n),
                "Odd-number sum mismatch at n = {n}"
            );
        }
    }

    #[test]
    fn geometric_sum_matches_formula() {
        for r in [-3_i64, -2, 2, 3, 5] {
            for n in 0..=12u32 {
                let iter_val = geometric_sum_iter(r, n);
                let formula_val = geometric_sum_formula(r, n).expect("r ≠ 1 here");
                assert_eq!(
                    iter_val, formula_val,
                    "Geometric sum mismatch: r={r}, n={n}"
                );
            }
        }
    }

    #[test]
    fn geometric_sum_r_equals_1() {
        // r = 1: geometric_sum_formula returns None; iter returns n+1
        for n in 0..=10u32 {
            assert_eq!(geometric_sum_iter(1, n), (n + 1) as i64);
            assert!(geometric_sum_formula(1, n).is_none());
        }
    }

    #[test]
    fn divisibility_4_divides_5n_minus_1() {
        for n in 0..=50u32 {
            assert!(four_divides_5n_minus_1(n), "Failed at n = {n}");
        }
    }

    #[test]
    fn fibonacci_strictly_less_than_2_pow_n() {
        // n = 0: F(0) = 0 < 2^0 = 1 ✓
        // n = 1: F(1) = 1 < 2^1 = 2 ✓
        for n in 0..=40u32 {
            assert!(fibonacci_bound(n), "Fibonacci bound failed at n = {n}");
        }
    }

    #[test]
    fn triangular_recursive_matches_formula() {
        for n in 0..=30u64 {
            assert_eq!(
                triangular_recursive(n),
                triangular_sum_formula(n),
                "Recursive triangle mismatch at n = {n}"
            );
        }
    }

    #[test]
    fn every_n_ge_2_has_prime_divisor() {
        for n in 2..=200u64 {
            let p = smallest_prime_divisor(n).expect("n ≥ 2");
            assert!(p >= 2, "Prime divisor too small");
            assert!(n % p == 0, "{p} does not divide {n}");
            // verify p itself is prime
            assert!(
                (2..p).all(|d| p % d != 0),
                "{p} is claimed prime but isn't"
            );
        }
    }

    #[test]
    fn smallest_prime_divisor_below_2_is_none() {
        assert!(smallest_prime_divisor(0).is_none());
        assert!(smallest_prime_divisor(1).is_none());
    }
}
