# Exercises — Lesson 4: Proof Techniques

> Fill in every blank. Show all steps. Name your proof strategy.  
> "By definition", "by algebra", "by Example 4.1" etc. are all valid justifications.  
> Submit when done and the tutor will mark each exercise.

---

## E1 — Classify the proof strategy

For each claim, state which proof strategy you would use *first* and why (one sentence). You do not need to write the full proof.

**(a)** For all n ∈ ℤ: if n³ is odd, then n is odd.

> Your answer:

**(b)** There is no largest prime number.

> Your answer:

**(c)** For all n ∈ ℤ: n² − n is even.

> Your answer:

**(d)** For all x ∈ ℝ: if x² = 2, then x ∉ ℚ.

> Your answer:

---

## E2 — Direct proof

Write a complete direct proof for each. Label each step with a justification.

**(a)** **Claim:** For all m, n ∈ ℤ: if m is even and n is even, then m + n is even.

> **Proof strategy:** Direct  
> **Proof:**  
> Assume ___  
> Step 1:  
> Step 2:  
> Therefore ___ □

**(b)** **Claim:** For all m, n ∈ ℤ: if m is odd and n is odd, then mn is odd.

> **Proof strategy:** Direct  
> **Proof:**  
> Assume ___  
> Step 1:  
> Step 2:  
> Therefore ___ □

**(c)** **Claim:** For all m ∈ ℤ: if m is even, then m² is divisible by 4.

> **Proof strategy:** Direct  
> **Proof:**

---

## E3 — Proof by contrapositive

Write a complete proof by contrapositive. Clearly state what the contrapositive is before proving it.

**(a)** **Claim:** For all n ∈ ℤ: if n² is even, then n is even.

> **State the contrapositive:**  
> **Proof:**

**(b)** **Claim:** For all a, b ∈ ℤ: if a × b is odd, then both a and b are odd.  
*(Hint: the contrapositive uses "or — use De Morgan.)*

> **State the contrapositive:**  
> **Proof:**

---

## E4 — Proof by contradiction

**(a)** **Claim:** √3 is irrational.

> **Proof strategy:** Contradiction  
> **Proof:**  
> Assume √3 ∈ ℚ. Then √3 = p/q where ___  
> *(follow the same structure as the √2 proof in the notes)*

**(b)** **Claim:** There is no largest integer.  
*(i.e. there is no n ∈ ℤ such that n ≥ m for all m ∈ ℤ)*

> **Proof:**  
> Assume ___  
> *(derive a contradiction)*

---

## E5 — Proof by cases

**(a)** **Claim:** For all n ∈ ℤ: n³ − n is divisible by 3.  
*(Hint: every integer is congruent to 0, 1, or 2 mod 3 — that's your three cases.)*  
*(You may use: a ≡ r (mod 3) means a = 3k + r for some k ∈ ℤ)*

> **Case 1 (n ≡ 0 mod 3):**  
> **Case 2 (n ≡ 1 mod 3):**  
> **Case 3 (n ≡ 2 mod 3):**  
> **Conclusion:**

**(b)** **Claim:** For all n ∈ ℤ: n² is either ≡ 0 (mod 4) or ≡ 1 (mod 4).  
*(Hint: split into even n and odd n.)*

> **Case 1 (n even):**  
> **Case 2 (n odd):**  
> **Conclusion:**

---

## E6 — Full proof (choose your strategy)

Write a clean, complete proof. After finishing, explain in one sentence why you chose that strategy.

**(a)** **Claim:** For all a, b ∈ ℤ: if a + b is odd, then exactly one of a, b is odd.

**(b)** **Claim:** log₂ 3 is irrational.  
*(Recall: log₂ 3 = x means 2^x = 3.)*

---

## Rust Challenge

The following skeleton is in `src/proofs.rs`. Complete the `todo!()` bodies.

```rust
/// Returns true if n is even.
pub fn is_even(n: i64) -> bool {
    todo!()
}

/// Returns true if n is odd.
pub fn is_odd(n: i64) -> bool {
    todo!()
}

/// Verifies E2(a): even + even = even, for all m,n in domain.
pub fn even_plus_even_is_even(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E2(b): odd * odd = odd, for all m,n in domain.
pub fn odd_times_odd_is_odd(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E3(a) contrapositive: even n → even n², for all n in domain.
pub fn even_square_contrapositive(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E5(a): n³ - n divisible by 3, for all n in domain.
pub fn cube_minus_n_div3(domain: &[i64]) -> bool {
    todo!()
}
```

Then make all the tests in `src/proofs.rs` pass.
