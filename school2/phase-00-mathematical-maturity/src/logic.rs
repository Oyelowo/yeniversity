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
    let d2 = !(p || q) == (!p && !q); // Law 2  (was: !p || !q — wrong)
    d1 && d2
}



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
