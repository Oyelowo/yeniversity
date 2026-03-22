# Lesson 3 — Quantifiers: ∀ and ∃

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 1 §1.4–1.5  
**Goal:** Express and negate statements about all or some members of a domain.

---

## 1. Motivation

Propositional logic only handles fixed propositions. But mathematics is full of statements like:

- "Every even number is divisible by 2"
- "There exists a prime greater than 100"
- "For all x, if x > 0 then x² > 0"

These talk about *all* or *some* elements of a set. You need **predicate logic** and **quantifiers**.

---

## 2. Predicates

A **predicate** is a sentence with a free variable — not a proposition yet, but becomes one when a value is substituted.

| Predicate | Value | Proposition | Truth |
|-----------|-------|-------------|-------|
| P(x): "x is even" | x = 4 | "4 is even" | TRUE |
| P(x): "x is even" | x = 7 | "7 is even" | FALSE |
| Q(x): "x² > 0" | x = 3 | "9 > 0" | TRUE |
| Q(x): "x² > 0" | x = 0 | "0 > 0" | FALSE |

---

## 3. The Universal Quantifier: ∀

**∀x P(x)** — "For all x in the domain, P(x) is true."

- TRUE iff P(x) holds for **every** element in the domain.
- FALSE iff **at least one** element fails — called a **counterexample**.

**One counterexample is enough to disprove a ∀ statement.**

**Examples** (domain = integers):
- ∀x (x² ≥ 0): **TRUE** — squaring never gives a negative integer
- ∀x (x² > 0): **FALSE** — counterexample: x = 0, since 0² = 0 ≯ 0

---

## 4. The Existential Quantifier: ∃

**∃x P(x)** — "There exists at least one x in the domain for which P(x) is true."

- TRUE iff **at least one** element satisfies P(x) — called a **witness**.
- FALSE iff P(x) is false for **every** element.

**One witness is enough to prove an ∃ statement.**

**Examples:**
- ∃x (x² = 4), domain = integers: **TRUE** — witness: x = 2
- ∃x (x² < 0), domain = reals: **FALSE** — no real square is negative

---

## 5. The Standard Number Domains

These are the domains you will see in almost every proof. Think of each as a progressively larger Rust type.

Each is a strict subset of the next: **ℕ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ**

```
u64  →  i64  →  "exact fraction"  →  f64  →  Complex<f64>
 ℕ   →   ℤ   →        ℚ          →   ℝ   →       ℂ
```

(In Rust, `f64` is the closest approximation to ℝ, but is technically a finite subset of ℚ
under the hood — binary fractions. True ℝ is infinite and uncountable. More in Lesson 8.)

### ℕ — Natural numbers `{0, 1, 2, 3, …}`

```rust
type N = u64; // unsigned: no negatives, no fractions
```

Whole counting numbers from 0. (Some books start at 1 — convention varies, always check.)
No negatives, no fractions, no decimals.

- 0 ∈ ℕ ✅  5 ∈ ℕ ✅  −3 ∉ ℕ ❌  1.5 ∉ ℕ ❌

### ℤ — Integers `{…, −2, −1, 0, 1, 2, …}`

```rust
type Z = i64; // signed: negatives allowed, still no fractions
```

The Z comes from German *Zahlen* ("numbers"). Extends ℕ with negatives.

- −7 ∈ ℤ ✅  0 ∈ ℤ ✅  1.5 ∉ ℤ ❌

### ℚ — Rationals `{p/q | p,q ∈ ℤ, q ≠ 0}`

```rust
struct Q { numerator: i64, denominator: i64 } // no built-in; see the `num` crate
```

All exact fractions. Q comes from *quotient*. Every integer is rational (p/1).

- 1/2 ∈ ℚ ✅  −3/7 ∈ ℚ ✅  √2 ∉ ℚ ❌ (irrational)  π ∉ ℚ ❌

### ℝ — Real numbers

```rust
type R = f64; // closest Rust approximation
```

Every point on the number line — all rationals *plus* all irrationals (√2, π, e, …).

- √2 ∈ ℝ ✅  π ∈ ℝ ✅  i ∉ ℝ ❌ (imaginary)

### ℂ — Complex numbers `{a + bi | a, b ∈ ℝ}`

```rust
// from the `num` crate:
use num::Complex;
type C = Complex<f64>; // C { re: a, im: b }
```

Every number of the form **a + bi**, where **i is defined as √(−1)**, so i² = −1.

```rust
let i: Complex<f64> = Complex::new(0.0, 1.0);
let i_squared = i * i; // Complex { re: -1.0, im: 0.0 }
assert!(i_squared.re < 0.0); // i² = −1 < 0 ← the counterexample to ∀x∈ℂ (x²≥0)
```

Every real number is complex with imaginary part 0:
```rust
let three: Complex<f64> = Complex::new(3.0, 0.0); // 3 + 0i = 3 ∈ ℝ ⊂ ℂ
```

This is why **ℝ ⊂ ℂ** — every real is a special case of a complex number.

### Key set-membership symbols

| Symbol | Meaning | Programming analogy |
|--------|---------|---------------------|
| x ∈ A | x is an element of set A | `A.contains(x)` returns true |
| x ∉ A | x is not in A | `!A.contains(x)` |
| A ⊂ B | every element of A is also in B | A's type can always be cast to B's type |
| A ⊄ B | some element of A is not in B | the cast would fail for some values |

Note: **∈ is element-level** (one object vs one set); **⊂ is set-level** (one whole set inside another).
- `2 ∈ ℕ` — the number 2 is a member of ℕ
- `ℕ ⊂ ℤ` — the entire set ℕ is contained inside ℤ

---

## 6. The Domain Matters — Full Explanation

The domain is the set of all values x is allowed to take. A quantifier without a domain is like
calling a function without passing the collection to iterate over — meaningless.

### Domains are iterables

```rust
// Math:  ∀x∈D P(x)              ∃x∈D P(x)
// Rust:
domain.iter().all(|x| p(x))   domain.iter().any(|x| p(x))
// Python:
all(p(x) for x in domain)     any(p(x) for x in domain)
```

The difference from ordinary iterables: math domains are usually **infinite** (you can't loop
over all of ℝ), so instead of checking every element you must *reason* about the set's
structure. The mental model is identical; the mechanism differs.

### Same predicate, different domain, different truth value

Let P(x) = "x² ≥ 0" (non-negative square):

| Domain | ∀x P(x) | Why |
|--------|---------|-----|
| ℕ | TRUE | u64 squares are always ≥ 0 |
| ℤ | TRUE | Squaring a negative int still gives a positive |
| ℝ | TRUE | Same reason |
| ℂ | **FALSE** | Counterexample: x = i, i² = −1 < 0 |

Let P(x) = "x² = 2":

| Domain | ∃x P(x) | Why |
|--------|---------|-----|
| ℕ | FALSE | No natural number squared equals 2 |
| ℤ | FALSE | No integer squared equals 2 |
| ℚ | FALSE | √2 is irrational — cannot be written as p/q |
| ℝ | **TRUE** | Witness: x = √2 ∈ ℝ |

**Always specify the domain.** The same formula can be true over ℝ and false over ℤ.

### Your predicate must be defined on the domain

You can't ask "is x even?" if x = 3.7. The predicate "x is even" only makes sense for
integers. In Rust terms: you can't call an `fn(i32) -> bool` predicate on an `f64` input —
the types don't match. Domains enforce that the predicate is well-typed for every element.

### Bounded quantifiers — restricting via implication

Sometimes you restrict the domain inline with a condition:

```
∀x∈ℝ, x > 0 ⇒ x² > 0
```

This is shorthand — the domain is still all of ℝ, but the implication makes the statement
vacuously true for x ≤ 0 (false antecedent). So it only really constrains positive x.

Negating: ¬(∀x∈ℝ (x>0 → x²>0)) ≡ ∃x∈ℝ (x>0 ∧ x²≤0)
The negation of the implication inside becomes a conjunction — exactly what we proved in Lesson 1.

### Proof scope: a proof over ℤ does not extend to ℝ

If you prove ∀x∈ℤ P(x), you have said nothing about irrational x. A counterexample in ℤ
is still a counterexample in ℝ (since ℤ ⊂ ℝ), but a proof in ℤ does not cover all of ℝ.

**Always specify the domain when writing a quantified statement.**

---

## 7. Negating Quantifiers

The two negation rules — memorise them:

| Original | Negation |
|----------|----------|
| ¬(∀x P(x)) | ≡ ∃x ¬P(x) |
| ¬(∃x P(x)) | ≡ ∀x ¬P(x) |

In words:
- "Not everything is P" ≡ "Something is not P"
- "Nothing is P" ≡ "Everything is not P"

**Both parts are required:** flip ∀↔∃ **and** negate P. Missing either is wrong.

**Examples:**

| Statement | Negation |
|-----------|----------|
| ∀x (x > 0) | ∃x (x ≤ 0) |
| ∃x (x² = −1) | ∀x (x² ≠ −1) |
| ∀x (x is prime → x is odd) | ∃x (x is prime ∧ x is even) — witness: 2 |

That last example: negating an implication inside a quantifier. The predicate was P(x) = "x is prime → x is odd". Its negation is ¬P(x) = "x is prime ∧ x is not odd" — the negation of an implication that we proved in Lesson 1.

---

## 8. Nested Quantifiers

When two quantifiers appear, **order is not commutative**.

Let P(x, y) mean "x + y = 0", domain = integers.

**∀x ∃y P(x,y):** For each x, find some y. Take y = −x. **TRUE.**  
**∃y ∀x P(x,y):** One single y works for every x. No such y exists. **FALSE.**

The key distinction:
- **∀x ∃y:** the choice of y may depend on which x you pick.
- **∃y ∀x:** one y must simultaneously work for all x.

More examples (domain = ℝ):

| Statement | Meaning | Truth |
|-----------|---------|-------|
| ∀x ∃y (y > x) | For every x, something is larger | TRUE (y = x+1) |
| ∃y ∀x (y > x) | One number is larger than everything | FALSE (no maximum in ℝ) |
| ∀x ∀y (x+y = y+x) | Addition is commutative | TRUE |
| ∃x ∃y (x·y = 0) | Some product is zero | TRUE (x=0, y=1) |

---

## 9. Negating Nested Quantifiers

Push ¬ inward **one quantifier at a time**, left to right:

```
¬(∀x ∃y P(x,y))
  ≡  ∃x ¬(∃y P(x,y))     [ ¬∀ ≡ ∃¬ ]
  ≡  ∃x ∀y ¬P(x,y)       [ ¬∃ ≡ ∀¬ ]
```

```
¬(∃x ∀y (x > y))
  ≡  ∀x ¬(∀y (x > y))    [ ¬∃ ≡ ∀¬ ]
  ≡  ∀x ∃y ¬(x > y)      [ ¬∀ ≡ ∃¬ ]
  ≡  ∀x ∃y (x ≤ y)       [ negate the predicate ]
```

Never flip two quantifiers at once — always go left to right, one step per quantifier.

---

## 10. Translation Practice

Converting English ↔ symbolic notation is a core skill for reading and writing proofs.

| English | Symbolic |
|---------|----------|
| Every integer has a successor | ∀x∈ℤ ∃y∈ℤ (y = x + 1) |
| Some square is negative | ∃x∈ℝ (x² < 0) — FALSE |
| Not all primes are odd | ∃x (x is prime ∧ ¬(x is odd)) — TRUE, x=2 |
| No integer is between 0 and 1 | ∀x∈ℤ ¬(0 < x < 1) |
| |

---

## 11. Rust Code

Open `src/quantifiers.rs` (new file). Implement quantifiers over a finite integer slice:

```rust
// ∀ over a slice — returns true if predicate holds for every element
pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // hint: use .iter() and .all()
    todo!()
}

// ∃ over a slice — returns true if predicate holds for at least one element
pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // hint: use .iter() and .any()
    todo!()
}

// Negation duality test:
// ¬(∀x P(x)) ≡ ∃x ¬P(x)
// Returns true if both sides agree (they always must).
pub fn negation_duality(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    let lhs = !for_all(domain, predicate);
    let rhs = there_exists(domain, |x| !predicate(x));
    lhs == rhs
}
```

---

## Key Takeaways

- **∀** is disproved by one counterexample; **∃** is proved by one witness
- Domain always matters — specify it
- Negation: flip ∀↔∃ **and** negate P (both required)
- **∀x ∃y ≠ ∃y ∀x** — the existential in the first can depend on x; in the second it cannot
- Negate nested quantifiers left to right, one at a time

---

## Exercises

See `exercises/03-quantifiers.md`
