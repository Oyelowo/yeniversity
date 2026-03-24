# Exercises 05 — Mathematical Induction

> **Instructions:** Attempt each proof on paper first. Then implement the Rust witness in `src/induction.rs`.  
> Mark each exercise ✅ only when you can reproduce the proof from memory.

---

## Tier 1 — Mechanical (apply the template directly)

**E1.** Prove by induction: for all n ≥ 1,

$$\sum_{i=1}^{n} 2i = n(n+1)$$

**E2.** Prove by induction: for all n ≥ 1,

$$\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}$$

(Hint: the algebra is heavier here — be patient with the factoring.)

**E3.** Prove by induction: for all n ≥ 0, 3 | (4ⁿ − 1).

**E4.** Prove by induction: for all n ≥ 1, 2ⁿ ≥ n + 1.

---

## Tier 2 — Requires a small twist

**E5.** Prove by induction: for all n ≥ 4, n! > 2ⁿ.

(Note: the base case is n = 4, *not* n = 0. Show both sides for n = 4 first.)

**E6.** Prove by **strong** induction: every integer n ≥ 2 can be written as a product of primes (Fundamental Theorem of Arithmetic — existence part).

(This is the same structure as Example 8.1 in the notes; extend it to products.)

**E7.** Define the sequence:
- a(0) = 1
- a(1) = 3
- a(n) = a(n−1) + 2·a(n−2) for n ≥ 2

Prove by **strong** induction that a(n) = 2ⁿ⁺¹ − (−1)ⁿ for all n ≥ 0.

---

## Tier 3 — Reasoning carefully

**E8.** Find the flaw in this "proof":

> **Claim:** All horses are the same color.
>
> *Proof by induction on n (size of a group of horses).*
>
> Base case (n = 1): A group of one horse has only one color. ✓
>
> Inductive step: Assume any group of k horses is monochromatic. Take a group of k+1 horses. Remove horse 1 — the remaining k horses are all the same color (by IH). Remove horse k+1 — the remaining k horses are all the same color (by IH). The color is shared, so all k+1 are the same color. □

Where exactly does the inductive step fail? Write one sentence.

**E9.** We have the inequality:

$$\forall n \ge 1: \quad \left(1 + \frac{1}{n}\right)^n < 3$$

Verify this holds for n = 1, 2, 3 numerically. Then explain why a proof by ordinary weak induction is not straightforward here (the IH for n = k doesn't immediately give you n = k+1). What would a proof strategy look like?

---

## Rust Additions to `src/induction.rs`

Add these three functions and their tests:

```rust
/// Returns the sum 1² + 2² + … + n²  (iterative).
pub fn sum_of_squares_iter(n: u64) -> u64 { ... }

/// Closed form: n(n+1)(2n+1)/6.
pub fn sum_of_squares_formula(n: u64) -> u64 { ... }

/// Returns n! (factorial) — panics if n > 20 to avoid overflow.
pub fn factorial(n: u64) -> u64 { ... }
```

Write `#[test]` that:
1. Confirms `sum_of_squares_iter(n) == sum_of_squares_formula(n)` for n ∈ [0, 100].
2. Confirms `factorial(n) > 2u64.pow(n as u32)` for n ∈ [5, 20] (note: n ≥ 5 not 4, since 4! = 24 = 2⁴ — they're equal at n=4 and strict inequality starts at n=5; re-read E5 if this surprises you).

---

## Self-Test Checklist

- [ ] I can write a weak induction proof template from memory
- [ ] I can write a strong induction proof template from memory
- [ ] I know the difference: when does strong induction become necessary?
- [ ] I reproduced the triangular-sum proof without notes
- [ ] I found the exact flaw in E8 (hint: it is in the step n = 1 → n = 2)
