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

## 5. The Domain Matters

The same predicate can be true or false depending on the domain.

| Statement | Domain | Truth |
|-----------|--------|-------|
| ∀x (x² ≥ 0) | ℝ | TRUE |
| ∀x (x² ≥ 0) | ℂ | FALSE (i² = −1) |
| ∃x (x² = 2) | ℝ | TRUE (x = √2) |
| ∃x (x² = 2) | ℤ | FALSE (√2 ∉ ℤ) |

**Always specify the domain when writing a quantified statement.**

---

## 6. Negating Quantifiers

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

## 7. Nested Quantifiers

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

## 8. Negating Nested Quantifiers

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

## 9. Translation Practice

Converting English ↔ symbolic notation is a core skill for reading and writing proofs.

| English | Symbolic |
|---------|----------|
| Every integer has a successor | ∀x∈ℤ ∃y∈ℤ (y = x + 1) |
| Some square is negative | ∃x∈ℝ (x² < 0) — FALSE |
| Not all primes are odd | ∃x (x is prime ∧ ¬(x is odd)) — TRUE, x=2 |
| No integer is between 0 and 1 | ∀x∈ℤ ¬(0 < x < 1) |
| |

---

## 10. Rust Code

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
