# Exercises — Lesson 3: Quantifiers

Write all answers symbolically and in plain English. Show your reasoning.

---

## E1. True or False?

State whether each is TRUE or FALSE and give a brief reason (a witness or a counterexample where relevant). Domain is stated per question.

**(a)** ∀x∈ℤ (x² ≥ 0)

> Your answer:
> ✅ TRUE. Excellent — correct classification, correct domain name (ℤ/Zahlen), correct Rust analogy,
> correct reasoning. One small point: don't forget negative integers too: (−3)² = 9 ≥ 0. ✓

**(b)** ∃x∈ℤ (x² < 0)

> Your answer:
> ✅ FALSE. Perfect — correct quantifier, correct domain, correct Rust translation, correct reasoning.
> Squaring any integer (positive, negative, or zero) always gives ≥ 0. No witness exists.

**(c)** ∀x∈ℝ (x² = x)

> Your answer:
> ✅ FALSE. Correct. Counterexample: x = 2, 2² = 4 ≠ 2. Good instinct to pick a number > 1.
> Note: x² = x ↔ x(x−1) = 0, so the only solutions are x = 0 and x = 1 — not *all* reals.

**(d)** ∃x∈ℝ (x² = x)

> Your answer:
> ✅ TRUE. Correct — witnesses 0 and 1 work. One correction: (−1)² = 1 ≠ −1, so −1 is NOT a witness.
> Only x = 0 and x = 1 satisfy x² = x.

**(e)** ∀x∈ℤ ∃y∈ℤ (x + y = 0)

> Your answer:
> ✅ TRUE. Perfect — and the Rust nested-iterator translation is exactly right.
> The witness is always y = −x, which is an integer whenever x is. Clean reasoning.

**(f)** ∃y∈ℤ ∀x∈ℤ (x + y = 0)

> Your answer:
> ✅ FALSE. Correct. The reasoning is right — no single fixed y can satisfy x + y = 0 for EVERY x.
> Minor note in your Rust: the variable names got swapped (`|x|` in the outer, `|y|` in the inner)
> but the logic you described in English is correct.
> Correct Rust: `integers_y.any(|y| integers_x.all(|x| x + y == 0))`

---

## E2. Negate These Statements

Write the negation symbolically **and** in plain English. Do not simplify further — just push the ¬ inward correctly.

**(a)** ∀x∈ℝ (x² ≥ 0)

> Symbolic negation:
> ❌ CORRECTED. You wrote ¬(∃x∈ℝ (x²<0)) — that adds an extra ¬ outside.
> Negation pushes the ¬ *inward*, it doesn't wrap the whole thing again.
> Correct: **∃x∈ℝ (x² < 0)**
> The ¬ flips ∀ to ∃ and negates the predicate (≥0 becomes <0). Done. No outer ¬.
>
> Rust: `real_numbers.any(|x| x*x < 0)` — note: no `!` on the outside.
>
> English: "There exists a real number whose square is negative."
> (This is FALSE over ℝ — but that's the correct negation regardless.)

**(b)** ∃x∈ℤ (x is odd)

> Symbolic negation:
> ❌ Two errors:
> 1. Domain changed from ℤ to ℝ — the domain must stay the same.
> 2. Same outer-¬ mistake: ¬(∀x∈ℤ ...) keeps the ¬ outside. Push it inward.
> Correct: **∀x∈ℤ (x is not odd)** equivalently **∀x∈ℤ (x is even)**
> Rust: `integers.all(|x| x % 2 != 1)` — no `!` on the outside.
>
> English: "Every integer is not odd" (i.e., every integer is even). This is FALSE — but it is the correct negation.

**(c)** ∀x∈ℤ (x > 0 → x² > 0)

> Symbolic negation:
> ❌ Same outer-¬ pattern again. You wrote ¬(∃x∈ℤ ¬(...)) — this is double-negating the whole thing,
> which gives back the original. Push the ¬ all the way inward in one pass:
>
> Step 1 — flip ∀ to ∃, negate the predicate:
>   ∃x∈ℤ ¬(x > 0 → x² > 0)
>
> Step 2 — negate the implication (Lesson 1: ¬(P→Q) ≡ P∧¬Q):
>   **∃x∈ℤ (x > 0 ∧ x² ≤ 0)**
>
> Rust: `integers.any(|x| x > 0 && x*x <= 0)`
>
> English: "There exists an integer that is positive but whose square is not positive."
> (This is FALSE over ℤ — 0 aside, all positive integers have positive squares — but it is the correct negation.)

**(d)** ∀x∈ℝ ∃y∈ℝ (x·y = 1)

> Symbolic negation:
> ✅ PERFECT — every step is correctly labelled and the Rust translation is correct.
> ∃x∈ℝ ∀y∈ℝ (x·y ≠ 1)
>
> Is the original true or false?
> ✅ FALSE. Correct — x = 0 is the counterexample. 0·y = 0 for every y, never 1.
> The negation ∃x∈ℝ ∀y∈ℝ (x·y≠1) is therefore TRUE (witnessed by x=0).


**(e)** ∃x∈ℕ ∀y∈ℕ (x ≤ y)

> Symbolic negation:
> ✅ PERFECT — both quantifier flips and the predicate negation are correct.
> ∀x∈ℕ ∃y∈ℕ (x > y)
>
> Is the original true or false?
> ✅ TRUE. Correct reasoning — 0 ∈ ℕ and 0 ≤ y for every y∈ℕ. Witness: x = 0.
> The negation ∀x∈ℕ ∃y∈ℕ (x>y) is therefore FALSE (no natural number is greater than something smaller in ℕ starting from 0).

---

## E3. Translate English → Symbolic

Define your domain and predicates first.

**(a)** "Every real number has an additive inverse."

> ❌ CORRECTED. Three issues:
> 1. Domain should be ℝ not ℕ (we're talking about real numbers, not naturals).
> 2. An *additive* inverse of x is the number y such that x + y = 0. You wrote a multiplicative-style formula.
> 3. The formula x = 1/(x+y) is unrelated to additive inverses.
>
> Correct:
> Domain: x∈ℝ, y∈ℝ
> Let P(x,y) = (x + y = 0)
> Formula: **∀x∈ℝ ∃y∈ℝ (x + y = 0)**
>
> Witness: y = −x, which is always real. Statement is TRUE.
> (Compare with E1(e) — same structure, but now over ℝ instead of ℤ.)

**(b)** "There is a largest integer."

> ❌ CORRECTED. The OS/int-max confusion is a programming instinct — mathematically ℤ is infinite.
>
> The statement says: there is some integer that is ≥ every other integer.
> Correct formula: **∃x∈ℤ ∀y∈ℤ (x ≥ y)**
>
> This is **FALSE**. For any integer x you claim is the largest, x+1 is a larger integer.
> There is no maximum element in ℤ.
>
> (In programming, `i64::MAX` exists because memory is finite — but that's an implementation limit,
> not a mathematical truth. In math, ℤ is unbounded in both directions.)

**(c)** "Not every function is continuous."

> ✅ Logic correct — you recognised this pushes to an existential. Two refinements:
> 1. The domain is the set of all functions, usually written 𝓕. The variable ranging over it is f.
> 2. C(f) is already a proposition (true/false) — write it as a predicate, not `C(f) = true`.
>
> Clean form:
> Domain: f ∈ 𝓕 (the set of all functions)
> Let C(f) = "f is continuous"
> Formula: **∃f∈𝓕 ¬C(f)**
>
> Rust: `functions.any(|f| !is_continuous(f))`
>
> This is TRUE — e.g. f(x) = 1/x is not continuous at x = 0.

**(d)** "For any two real numbers, there is a real number strictly between them."

> ✅ Formula almost right — just fix the domain notation. You can't write {x,y}⊂ℝ as a quantifier;
> use two separate universal quantifiers:
>
> Domain: x∈ℝ, y∈ℝ, z∈ℝ
> Let P(x,y,z) = (x < z < y)
> Formula: **∀x∈ℝ ∀y∈ℝ (x < y → ∃z∈ℝ (x < z < y))**
>
> (The implication x<y is needed — if x≥y there's nothing to be "between".)
>
> What property does this describe?
> ❌ CORRECTED: This is called the **density** of ℝ (and also of ℚ). Between any two real numbers
> there is always another real number — in fact, infinitely many. ℝ is *not* irrational;
> irrationality is a property of individual numbers (like √2), not of ℝ as a set.

---

## E4. Translate Symbolic → English

Write the most natural English sentence for each. Then state if it is true (domain = ℝ unless noted).

**(a)** ∀x ∀y (x + y = y + x)

> English: For all x and y (in ℝ), x + y equals y + x.
> ✅ TRUE. This is the **commutativity of addition**. More natural English: "Addition is commutative."

**(b)** ∃x∈ℝ (x² + 1 = 0)

> English: There exists a real number whose square plus one equals zero.
> ✅ FALSE over ℝ. x² ≥ 0 for all real x, so x²+1 ≥ 1 > 0. No solution exists in ℝ.
> (Over ℂ it is TRUE — witness: x = i, since i²+1 = −1+1 = 0.)

**(c)** ∀x∈ℝ ∃y∈ℝ (y² = x)

> English: For every real number x, there exists a real number y whose square equals x.
> ✅ FALSE. Correct reasoning — counterexample: x = −1. No real y satisfies y² = −1.
> (Over ℂ it would be TRUE — y = i works for x = −1.)

**(d)** ∃x∈ℤ ∀y∈ℤ (x · y = y)

> English: There exists an integer x such that x times y equals y for every integer y.
> ✅ TRUE. Witness: x = 1. 1·y = y for all y. This is the **multiplicative identity**.

---

## E5. (Harder) Negate a Complex Statement

Negate the following statement and simplify ¬P fully (flip quantifiers, negate the predicate):

**"Every function that is differentiable is continuous."**

First write it symbolically (let D(f) = "f is differentiable", C(f) = "f is continuous"), then negate it step by step.

> ❌ CORRECTED. You forgot the ∀ quantifier — "Every function" needs ∀f.
> The predicate on each f is D(f)→C(f), an implication.
>
> Symbolic form: **∀f∈𝓕 (D(f) → C(f))**
>
> Negation step 1 — push ¬ past ∀ (flip to ∃, negate the predicate):
>   ∃f∈𝓕 ¬(D(f) → C(f))
>
> Negation step 2 — negate the implication (¬(P→Q) ≡ P∧¬Q from Lesson 1):
>   **∃f∈𝓕 (D(f) ∧ ¬C(f))**
>
> English: "There exists a function that is differentiable but not continuous."
>
> This is **FALSE** — every differentiable function is in fact continuous (a theorem from calculus).
> But that is the correct logical negation of the original statement.
>
> Notice how the negation *finds the counterexample shape*: to disprove "all differentiable functions
> are continuous" you would need to exhibit one differentiable function that is not continuous.
---

## Rust Challenge

In `src/quantifiers.rs`, implement and test:

```rust
pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // todo!() // hint: .iter().all(|&x| predicate(x))
    domain.iter().all(|&x| predicate(x))
}

pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // todo!() // hint: .iter().any(|&x| predicate(x))
    domain.iter().any(|&x| predicate(x))
}

pub fn negation_duality(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // ¬(∀x P(x)) ≡ ∃x ¬P(x)  — verify both sides agree
    // todo!()
    let lhs = !domain.iter().all(|&x| predicate(x));
    let rhs = domain.iter().any(|&x| !predicate(x));
    lhs == rhs
}
```

Tests to write:
1. `for_all(&[1,2,3,4], |x| x*x >= 0)` → `true`
2. `for_all(&[1,2,3,4], |x| x > 3)` → `false` (counterexample: 1)
3. `there_exists(&[1,2,3,4], |x| x > 3)` → `true` (witness: 4)
4. `there_exists(&[-2,-1,0,1], |x| x*x < 0)` → `false`
5. `negation_duality(&[1,2,3,4], |x| x > 2)` → `true`
