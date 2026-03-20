//! Lesson 3: Quantifiers — ∀ (for_all) and ∃ (there_exists) over finite domains.

/// ∀x∈domain P(x) — true if predicate holds for every element.
pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    domain.iter().all(|&x| predicate(x))
}

/// ∃x∈domain P(x) — true if predicate holds for at least one element.
pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    domain.iter().any(|&x| predicate(x))
}

/// Verifies negation duality: ¬(∀x P(x)) ≡ ∃x ¬P(x)
/// Both sides must always agree.
pub fn negation_duality(domain: &[i32], predicate: impl Fn(i32) -> bool) -> bool {
    let lhs = !domain.iter().all(|&x| predicate(x));
    let rhs = domain.iter().any(|&x| !predicate(x));
    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_all_squares_nonneg() {
        // ∀x∈[1,2,3,4]: x² ≥ 0  →  true
        assert!(for_all(&[1, 2, 3, 4], |x| x * x >= 0));
    }

    #[test]
    fn for_all_gt3_is_false() {
        // ∀x∈[1,2,3,4]: x > 3  →  false (counterexample: 1)
        assert!(!for_all(&[1, 2, 3, 4], |x| x > 3));
    }

    #[test]
    fn there_exists_gt3() {
        // ∃x∈[1,2,3,4]: x > 3  →  true (witness: 4)
        assert!(there_exists(&[1, 2, 3, 4], |x| x > 3));
    }

    #[test]
    fn there_exists_neg_square_is_false() {
        // ∃x∈[-2,-1,0,1]: x² < 0  →  false
        assert!(!there_exists(&[-2, -1, 0, 1], |x| x * x < 0));
    }

    #[test]
    fn negation_duality_holds() {
        // ¬(∀x P(x)) ≡ ∃x ¬P(x) — must agree for any domain and predicate
        assert!(negation_duality(&[1, 2, 3, 4], |x| x > 2));
        assert!(negation_duality(&[1, 2, 3, 4], |x| x > 0));
        assert!(negation_duality(&[-1, 0, 1], |x| x >= 0));
    }
}
