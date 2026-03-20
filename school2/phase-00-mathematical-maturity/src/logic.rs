//! Propositional logic: truth tables, connectives, tautology checking.

/// Evaluates a two-variable boolean formula given values for p and q.
/// Replace the body with whichever formula you are studying.
pub fn evaluate(p: bool, q: bool) -> bool {
    // Example: p → q  (¬p ∨ q)
    !p || q
}

/// Generates and prints the full truth table for a two-variable formula.
pub fn truth_table(label: &str, formula: fn(bool, bool) -> bool) {
    println!("{:<6} {:<6} | {}", "p", "q", label);
    println!("{}", "-".repeat(20));
    for &p in &[false, true] {
        for &q in &[false, true] {
            println!("{:<6} {:<6} | {}", p, q, formula(p, q));
        }
    }
}

/// Verifies both De Morgan laws for the given (p, q):
///   Law 1: ¬(P ∧ Q)  ≡  ¬P ∨ ¬Q
///   Law 2: ¬(P ∨ Q)  ≡  ¬P ∧ ¬Q
/// Returns true if both sides agree for the given inputs (they always must).
pub fn demorgan_check(p: bool, q: bool) -> bool {
    let d1 = !(p && q) == (!p || !q); // Law 1
    let d2 = !(p || q) == (!p && !q); // Law 2
    d1 && d2
}

/// Lesson 2: Implication law — P→Q ≡ ¬P∨Q
/// Implemented using ONLY NOT and OR (no if/match), matching the law exactly.
pub fn implies(p: bool, q: bool) -> bool {
    !p || q
}

/// Returns true if two-variable formula `f` and `g` agree on all four (p,q) inputs.
/// Use this to verify logical equivalences without truth tables in hand.
pub fn logically_equivalent(f: fn(bool, bool) -> bool, g: fn(bool, bool) -> bool) -> bool {
    for &p in &[false, true] {
        for &q in &[false, true] {
            if f(p, q) != g(p, q) {
                return false;
            }
        }
    }
    true
}

// // Then add tests to verify:
// // 1. `implies` agrees with the direct Rust `!p || q` expression
// // 2. `logically_equivalent(|p,q| !(p && q), |p,q| !p || !q)` returns `true` (De Morgan 1)
// // 3. `logically_equivalent(|p,q| p && !q, |p,q| !p || q)` returns `false` (they are NOT equivalent)
// In `src/logic.rs`, implement:

// ```rust
// pub fn implies(p: bool, q: bool) -> bool {
//     // Use ONLY ! and || — no if, no match
//     // (must match the Implication law: P→Q ≡ ¬P∨Q)
//     todo!()
// }

// pub fn logically_equivalent(
//     f: fn(bool, bool) -> bool,
//     g: fn(bool, bool) -> bool,
// ) -> bool {
//     // Return true if f and g agree on ALL four (p,q) combinations
//     todo!()
// }


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn de_morgan_check() {
        assert!(demorgan_check(true, true));
        assert!(demorgan_check(true, false));
        assert!(demorgan_check(false, true));
        assert!(demorgan_check(false, false));
    }

    #[test]
    fn implication_law_matches_direct_formula() {
        // implies() must agree with !p || q on all inputs
        for &p in &[false, true] {
            for &q in &[false, true] {
                assert_eq!(implies(p, q), !p || q, "p={p}, q={q}");
            }
        }
    }

    #[test]
    fn equivalent_demorgan_1() {
        // ¬(P∧Q) ≡ ¬P∨¬Q
        assert!(logically_equivalent(|p, q| !(p && q), |p, q| !p || !q));
    }

    #[test]
    fn not_equivalent_example() {
        // P∧¬Q is NOT equivalent to ¬P∨Q
        assert!(!logically_equivalent(|p, q| p && !q, |p, q| !p || q));
    }

    #[test]
    fn implication_false_antecedent_is_true() {
        // p → q is vacuously true when p is false
        assert!(evaluate(false, false));
        assert!(evaluate(false, true));
    }

    #[test]
    fn implication_true_antecedent_follows_q() {
        assert!(!evaluate(true, false));
        assert!(evaluate(true, true));
    }
}
