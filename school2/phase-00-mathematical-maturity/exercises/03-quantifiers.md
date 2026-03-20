# Exercises — Lesson 3: Quantifiers

Write all answers symbolically and in plain English. Show your reasoning.

---

## E1. True or False?

State whether each is TRUE or FALSE and give a brief reason (a witness or a counterexample where relevant). Domain is stated per question.

**(a)** ∀x∈ℤ (x² ≥ 0)

> Your answer:

**(b)** ∃x∈ℤ (x² < 0)

> Your answer:

**(c)** ∀x∈ℝ (x² = x)

> Your answer:

**(d)** ∃x∈ℝ (x² = x)

> Your answer:

**(e)** ∀x∈ℤ ∃y∈ℤ (x + y = 0)

> Your answer:

**(f)** ∃y∈ℤ ∀x∈ℤ (x + y = 0)

> Your answer:

---

## E2. Negate These Statements

Write the negation symbolically **and** in plain English. Do not simplify further — just push the ¬ inward correctly.

**(a)** ∀x∈ℝ (x² ≥ 0)

> Symbolic negation:
> English:

**(b)** ∃x∈ℤ (x is odd)

> Symbolic negation:
> English:

**(c)** ∀x∈ℤ (x > 0 → x² > 0)

> Symbolic negation:
> English:
> Hint: the predicate here is an implication. Its negation is what we proved in Lesson 1 E3(b).

**(d)** ∀x∈ℝ ∃y∈ℝ (x·y = 1)

> Symbolic negation:
> English:
> Is the original statement true or false? What is the counterexample?

**(e)** ∃x∈ℕ ∀y∈ℕ (x ≤ y)

> Symbolic negation:
> English:
> Is the original true or false? Why?

---

## E3. Translate English → Symbolic

Define your domain and predicates first.

**(a)** "Every real number has an additive inverse."

> Domain:
> Let P(x,y) =
> Formula:

**(b)** "There is a largest integer."

> Domain:
> Formula:
> Is this true or false?

**(c)** "Not every function is continuous."

> Domain:
> Let C(f) =
> Formula (using ¬ and ∀ or ∃):

**(d)** "For any two real numbers, there is a real number strictly between them."

> Domain:
> Let P(x,y,z) =
> Formula:
> What property of ℝ does this describe?

---

## E4. Translate Symbolic → English

Write the most natural English sentence for each. Then state if it is true (domain = ℝ unless noted).

**(a)** ∀x ∀y (x + y = y + x)

> English:
> True or false?

**(b)** ∃x∈ℝ (x² + 1 = 0)

> English:
> True or false (over ℝ)?

**(c)** ∀x∈ℝ ∃y∈ℝ (y² = x)

> English:
> True or false?
> Hint: think about negative x.

**(d)** ∃x∈ℤ ∀y∈ℤ (x · y = y)

> English:
> True or false? Give the witness.

---

## E5. (Harder) Negate a Complex Statement

Negate the following statement and simplify ¬P fully (flip quantifiers, negate the predicate):

**"Every function that is differentiable is continuous."**

First write it symbolically (let D(f) = "f is differentiable", C(f) = "f is continuous"), then negate it step by step.

> Symbolic form:
> Negation step 1 (push ¬ past ∀):
> Negation step 2 (negate the implication: ¬(D(f)→C(f))):
> Final symbolic form:
> English reading of the negation:

---

## Rust Challenge

In `src/quantifiers.rs`, implement and test:

```rust
pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    todo!() // hint: .iter().all(|&x| predicate(x))
}

pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    todo!() // hint: .iter().any(|&x| predicate(x))
}

pub fn negation_duality(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // ¬(∀x P(x)) ≡ ∃x ¬P(x)  — verify both sides agree
    todo!()
}
```

Tests to write:
1. `for_all(&[1,2,3,4], |x| x*x >= 0)` → `true`
2. `for_all(&[1,2,3,4], |x| x > 3)` → `false` (counterexample: 1)
3. `there_exists(&[1,2,3,4], |x| x > 3)` → `true` (witness: 4)
4. `there_exists(&[-2,-1,0,1], |x| x*x < 0)` → `false`
5. `negation_duality(&[1,2,3,4], |x| x > 2)` → `true`
