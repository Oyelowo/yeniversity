//! Cardinality simulations — finite-set experiments illustrating the
//! bijection-based definition of "same size", the countable/uncountable
//! distinction, and a computational analogue of Cantor's diagonal argument.
//!
//! The real diagonal argument cannot be run on finite machines, but we can
//! simulate its *structure* on finite bit-strings to make the logic concrete.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Bijection-based cardinality equality (finite sets)
// ---------------------------------------------------------------------------

/// Returns `true` if two finite sets have the same cardinality, i.e.
/// there exists a bijection between them.
///
/// For finite sets this simply compares sizes.
pub fn same_cardinality_finite<T, U>(a: &HashSet<T>, b: &HashSet<U>) -> bool {
    a.len() == b.len()
}

// ---------------------------------------------------------------------------
// Injection witnesses
// ---------------------------------------------------------------------------

/// An explicit injection from A into B represented as a partial map.
/// All values are distinct.
pub fn is_injection<A: Eq + Hash + Clone, B: Eq + Hash + Clone>(
    map: &HashMap<A, B>,
) -> bool {
    let mut seen: HashSet<&B> = HashSet::new();
    map.values().all(|b| seen.insert(b))
}

// ---------------------------------------------------------------------------
// Countable-set helpers
// ---------------------------------------------------------------------------

/// Produce the first `n` terms of the standard bijection ℕ → ℤ:
///   0 ↦ 0, 1 ↦ -1, 2 ↦ 1, 3 ↦ -2, 4 ↦ 2, …
pub fn nat_to_int(index: u64) -> i64 {
    if index % 2 == 0 {
        (index / 2) as i64
    } else {
        -((index + 1) as i64 / 2)
    }
}

/// Produce the first `n` terms of the standard bijection ℕ → ℤ as a Vec.
pub fn nat_to_int_sequence(n: usize) -> Vec<i64> {
    (0..n as u64).map(nat_to_int).collect()
}

/// Given index n in ℕ×ℕ via Cantor pairing, return the (row, col) pair.
/// This enumerates all pairs of natural numbers, used to show ℚ is countable.
///
/// Standard Cantor diagonal order:
///   (0,0), (1,0), (0,1), (2,0), (1,1), (0,2), …
pub fn cantor_unpair(n: u64) -> (u64, u64) {
    // invert the triangular number to find the diagonal
    let w = (((8 * n + 1) as f64).sqrt() as u64 - 1) / 2;
    let t = (w * w + w) / 2;
    let col = n - t;
    let row = w - col;
    (row, col)
}

/// Enumerate the first `n` (numerator, denominator) pairs corresponding to
/// rational numbers via Cantor pairing.  Denominator is `col + 1` to skip 0.
///
/// Note: this includes non-reduced fractions; deduplication to ℚ is not done
/// here — the point is to demonstrate the diagonal traversal structure.
pub fn rational_enumeration(n: usize) -> Vec<(i64, u64)> {
    (0..n as u64)
        .map(|k| {
            let (row, col) = cantor_unpair(k);
            (row as i64, col + 1)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Diagonal argument simulation (on finite bit-strings)
// ---------------------------------------------------------------------------

/// Simulate Cantor's diagonal argument on a finite list of binary strings.
///
/// Given a list of `n`-bit strings, construct a new `n`-bit string that
/// differs from the i-th string in position i.
///
/// The flip rule: 0 ↦ 1, 1 ↦ 0 (analogue of the digit flipping in the reals).
///
/// Returns the diagonal string, which is guaranteed not to appear in `list`
/// (provided the list has at least `n` entries; we use min(list.len(), n)).
pub fn diagonal_flip(list: &[Vec<u8>]) -> Vec<u8> {
    let n = list.len();
    (0..n)
        .map(|i| {
            let bit = list[i].get(i).copied().unwrap_or(0);
            1 - bit // flip 0 ↔ 1
        })
        .collect()
}

/// Verify that the diagonal result differs from every input string at the
/// expected position.
pub fn diagonal_differs_from_all(list: &[Vec<u8>], diagonal: &[u8]) -> bool {
    list.iter().enumerate().all(|(i, row)| {
        let d_bit = row.get(i).copied().unwrap_or(0);
        let x_bit = diagonal.get(i).copied().unwrap_or(0);
        d_bit != x_bit
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- same_cardinality_finite ---

    #[test]
    fn same_size_sets_have_same_cardinality() {
        let a: HashSet<u8> = [1, 2, 3].into_iter().collect();
        let b: HashSet<char> = ['a', 'b', 'c'].into_iter().collect();
        assert!(same_cardinality_finite(&a, &b));
    }

    #[test]
    fn different_size_sets_differ() {
        let a: HashSet<u8> = [1, 2].into_iter().collect();
        let b: HashSet<u8> = [1, 2, 3].into_iter().collect();
        assert!(!same_cardinality_finite(&a, &b));
    }

    // --- nat_to_int bijection ---

    #[test]
    fn nat_to_int_covers_expected_values() {
        let seq = nat_to_int_sequence(7);
        // 0 → 0, 1 → -1, 2 → 1, 3 → -2, 4 → 2, 5 → -3, 6 → 3
        assert_eq!(seq, vec![0, -1, 1, -2, 2, -3, 3]);
    }

    #[test]
    fn nat_to_int_no_repeats_in_prefix() {
        let n = 100;
        let seq = nat_to_int_sequence(n);
        let unique: HashSet<i64> = seq.iter().cloned().collect();
        assert_eq!(unique.len(), n, "bijection must produce distinct values");
    }

    // --- Cantor pairing ---

    #[test]
    fn cantor_unpair_first_few() {
        // Standard Cantor diagonal order: (0,0),(1,0),(0,1),(2,0),(1,1),(0,2)
        assert_eq!(cantor_unpair(0), (0, 0));
        assert_eq!(cantor_unpair(1), (1, 0));
        assert_eq!(cantor_unpair(2), (0, 1));
        assert_eq!(cantor_unpair(3), (2, 0));
        assert_eq!(cantor_unpair(4), (1, 1));
        assert_eq!(cantor_unpair(5), (0, 2));
    }

    #[test]
    fn cantor_pairing_produces_distinct_pairs() {
        let n = 200u64;
        let pairs: HashSet<(u64, u64)> = (0..n).map(cantor_unpair).collect();
        assert_eq!(pairs.len() as u64, n, "every index must produce a unique pair");
    }

    // --- Diagonal argument simulation ---

    #[test]
    fn diagonal_flip_constructs_differing_string() {
        // Simulate a "listing" of 4-bit strings
        let list: Vec<Vec<u8>> = vec![
            vec![0, 1, 0, 1],
            vec![1, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 1, 0, 0],
        ];
        let diag = diagonal_flip(&list);
        // diag[0] ≠ list[0][0]=0, diag[1] ≠ list[1][1]=0, etc.
        assert_eq!(diag, vec![1, 1, 0, 1]);
        assert!(diagonal_differs_from_all(&list, &diag));
    }

    #[test]
    fn diagonal_not_in_list() {
        let list: Vec<Vec<u8>> = vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ];
        let diag = diagonal_flip(&list);
        // diagonal flips: 1→0, 1→0, 1→0 → [0,0,0]
        assert_eq!(diag, vec![0, 0, 0]);
        assert!(!list.contains(&diag));
        assert!(diagonal_differs_from_all(&list, &diag));
    }

    // --- is_injection ---

    #[test]
    fn detects_valid_injection() {
        let map: HashMap<u8, u8> = [(1, 10), (2, 20), (3, 30)].into_iter().collect();
        assert!(is_injection(&map));
    }

    #[test]
    fn detects_non_injection() {
        let map: HashMap<u8, u8> = [(1, 10), (2, 10), (3, 30)].into_iter().collect();
        assert!(!is_injection(&map));
    }
}
