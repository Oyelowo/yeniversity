# Exercises — Lesson 2: Logical Equivalence (Algebraic Style)

Write your solutions as a **chain of equivalences**. Each step must name the law used.
Do NOT use truth tables — that defeats the purpose.

---

## E1. Warm-up: Expand and Simplify

Simplify each formula to its simplest equivalent using the laws. Show every step.

**(a)** ¬(¬P ∨ Q)

> Your chain:
> ```
> ¬(¬P ∨ Q)
A = ¬P, B = Q
De Morgan 2: ¬(A v B) = ¬A ^ ¬B
>   ≡  ¬(¬P) ^ ¬B     [ De Morgan 2     ]
>   ≡  P ^ ¬ B        [ Double Negation ]
> ```

**(b)** P ∧ (P ∨ Q)

> Your chain:
> ```
> P ∧ (P ∨ Q)
>   ≡  P     [ Absorption ]
> ```

**(c)** (P ∨ Q) ∧ ¬P

> Your chain:
> ```
> (P ∨ Q) ∧ ¬P
¬P ^ (P v Q)
>   ≡  (¬P ^ P) v (¬P ^ Q)     [ Distribution ]
>   ≡  F v (¬P ^ Q)     [ Complement ]
>   ≡  (¬P ^ Q)     [ Identity ]
> ```
> Hint: distribute ∧¬P inward.

---

## E2. Prove These Equivalences Algebraically

**(a)** Prove: ¬(P ∨ Q) → R  ≡  (¬P ∧ ¬Q) → R

> Your chain:

**(b)** Prove: P → (P → Q)  ≡  P → Q

> Your chain:
> Hint: start by expanding the inner P→Q with the Implication law.

**(c)** Prove: ¬(P ↔ Q)  ≡  (P ∧ ¬Q) ∨ (¬P ∧ Q)

> Your chain:
> Hint: expand ↔ using (P→Q)∧(Q→P), then expand each →, then push the ¬ inward with De Morgan.

---

## E3. Identify the Flaw

Each chain below has exactly one wrong step. Find it and write the correct version.

**(a)**
```
¬(P ∧ Q) → R
  ≡  (¬P ∧ ¬Q) → R     [ De Morgan ]    ← is this step right?
  ≡  ¬(¬P ∧ ¬Q) ∨ R   [ Implication ]
```

> Your answer (which step is wrong and why):

**(b)**
```
P → Q
  ≡  ¬P → ¬Q            [ Contrapositive ]
```

> Your answer (which step is wrong and why):

---

## E4. Tautology or Not?

Use the algebraic method (laws only) to determine whether each formula is a tautology, a contradiction, or neither. Show your chain.

**(a)** (P → Q) → ((Q → R) → (P → R))

> Your answer and chain:
> Hint: this is the famous **hypothetical syllogism**. Expect several steps. Start by letting the outermost → become ¬(...) ∨ (...) via the Implication law.

**(b)** P ↔ ¬P

> Your answer and chain:
> This should reduce to F in about 4 steps.

---

## E5. (Harder) Prove the Biconditional Equivalence

Show that: **P ↔ Q  ≡  (P ∧ Q) ∨ (¬P ∧ ¬Q)**

This is the "same-truth-value" reading of the biconditional — it's true exactly when P and Q are both true or both false.

> Your chain:
> Hint: start from the Biconditional law P↔Q ≡ (P→Q)∧(Q→P), expand each →, then distribute.

---

## Rust Challenge

In `src/logic.rs`, implement:

```rust
pub fn implies(p: bool, q: bool) -> bool {
    // Use ONLY ! and || — no if, no match
    // (must match the Implication law: P→Q ≡ ¬P∨Q)
    todo!()
}

pub fn logically_equivalent(
    f: fn(bool, bool) -> bool,
    g: fn(bool, bool) -> bool,
) -> bool {
    // Return true if f and g agree on ALL four (p,q) combinations
    todo!()
}
```

Then add tests to verify:
1. `implies` agrees with the direct Rust `!p || q` expression
2. `logically_equivalent(|p,q| !(p && q), |p,q| !p || !q)` returns `true` (De Morgan 1)
3. `logically_equivalent(|p,q| p && !q, |p,q| !p || q)` returns `false` (they are NOT equivalent)
