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
>   ≡  ¬(¬P) ∧ ¬Q     [ De Morgan 2: ¬(A∨B) ≡ ¬A∧¬B, with A=¬P, B=Q ]
>   ≡  P ∧ ¬Q          [ Double Negation: ¬¬P ≡ P ]
> ```
> ✅ Correct. Minor fix: the final answer should use Q (not B) — B was just your substitution label.

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
¬(P ∨ Q) ≡ ¬P ^ ¬Q           De Morgan 2
thus:
(¬P ^ ¬Q) -> R

Final answer:
¬(P ∨ Q) → R  [De Morgan 2]
≡  (¬P ∧ ¬Q) → R

**(b)** Prove: P → (P → Q)  ≡  P → Q

> Your chain:
> Hint: start by expanding the inner P→Q with the Implication law.
P → (P → Q) 
≡ (P ^ P) -> Q   [Exporation  P → (Q → R) ≡ (P ∧ Q) → R]
≡ P -> Q         [Idempotence P ∧ P ≡ P | P ∨ P ≡ P    ]

**(c)** Prove: ¬(P ↔ Q)  ≡  (P ∧ ¬Q) ∨ (¬P ∧ Q)

> Your chain:
> Hint: expand ↔ using (P→Q)∧(Q→P), then expand each →, then push the ¬ inward with De Morgan.

> ❌ CORRECTED — you dropped the outer ¬ in the very first step.
> The Biconditional law says `P↔Q ≡ (P→Q)∧(Q→P)`, so `¬(P↔Q) ≡ ¬((P→Q)∧(Q→P))`.
> The negation must be preserved and pushed inward, not discarded.
>
> ```
> ¬(P ↔ Q)
>   ≡  ¬((P → Q) ∧ (Q → P))    [ Biconditional: P↔Q ≡ (P→Q)∧(Q→P) ]
>   ≡  ¬(P → Q) ∨ ¬(Q → P)    [ De Morgan 1: ¬(A∧B) ≡ ¬A∨¬B ]
>   ≡  (P ∧ ¬Q) ∨ (Q ∧ ¬P)    [ ¬(A→B) ≡ A∧¬B, applied twice — we proved this in E3(b) lesson 1 ]
>   ≡  (P ∧ ¬Q) ∨ (¬P ∧ Q)    [ Commutativity of ∧ inside second term: Q∧¬P ≡ ¬P∧Q ]
> ```
>
> The key insight: the ¬ never disappears — it rides in and gets pushed *through* the biconditional
> by first expanding ↔ then applying De Morgan 1 to split across ∧.

My Second Retry
Prove: ¬(P ∨ Q) → R  ≡  (¬P ∧ ¬Q) → R
¬(P ↔ Q) 
= ¬((P ->  Q) ^   (Q  -> P)) [Biconditional]
=  ¬(P ->  Q) v  ¬(Q  -> P) [De Morgan 1]
= ¬(¬P  v  Q) v ¬(¬Q  v  P) [Implication]
=(¬(¬P) ^ ¬Q) v(¬(¬Q) ^ ¬P) [De Moran 2]   1st part: A=¬P, B=Q| 2nd:A=¬Q, B=P
=   (P  ^ ¬Q) v   (Q  ^ ¬P) [Double Negation on 1 letters]   
=   (P  ^ ¬Q) v   (¬P ^ Q)  [Commutativity]   


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
first step is wrong cos AND was not switched to OR which De Morgan 1 requires.
Perhaps the bracket too shouldnt be required/used?

**(b)**
```
P → Q
  ≡  ¬P → ¬Q            [ Contrapositive ]
```

> Your answer (which step is wrong and why):
The first step is wrong cos the letters P and Q are not swapped.
---

## E4. Tautology or Not?

Use the algebraic method (laws only) to determine whether each formula is a tautology, a contradiction, or neither. Show your chain.

**(a)** (P → Q) → ((Q → R) → (P → R))

> Your answer and chain:
> Hint: this is the famous **hypothetical syllogism**. Expect several steps. Start by letting the outermost → become ¬(...) ∨ (...) via the Implication law.

> COMPLETED (Tautology — reduces to T):
> ```
> (P → Q) → ((Q → R) → (P → R))
>   ≡  ((P → Q) ∧ (Q → R)) → (P → R)              [ Exportation: A→(B→C) ≡ (A∧B)→C ]
>   ≡  ¬((P → Q) ∧ (Q → R)) ∨ (P → R)            [ Implication ]
>   ≡  ¬(P → Q) ∨ ¬(Q → R) ∨ (P → R)            [ De Morgan 1 ]
>   ≡  (P ∧ ¬Q) ∨ (Q ∧ ¬R) ∨ (¬P ∨ R)           [ ¬(A→B)≡A∧¬B twice; Implication: P→R≡¬P∨R ]
>   ≡  ¬P ∨ (P ∧ ¬Q) ∨ R ∨ (Q ∧ ¬R)             [ Commutativity + Associativity ]
>   ≡  ((¬P ∨ P) ∧ (¬P ∨ ¬Q)) ∨ ((R ∨ Q) ∧ (R ∨ ¬R))  [ Distributivity ×2 ]
>   ≡  (T ∧ (¬P ∨ ¬Q)) ∨ ((R ∨ Q) ∧ T)           [ Complement ×2: ¬P∨P≡T, R∨¬R≡T ]
>   ≡  (¬P ∨ ¬Q) ∨ (R ∨ Q)                        [ Identity ×2 ]
>   ≡  ¬P ∨ R ∨ (¬Q ∨ Q)                          [ Commutativity + Associativity ]
>   ≡  ¬P ∨ R ∨ T                                  [ Complement: ¬Q∨Q≡T ]
>   ≡  T                                            [ Domination: A∨T≡T ]
> ```
> This is **hypothetical syllogism**: if P→Q and Q→R, then P→R. The algebra confirms it is always true.
> The Exportation trick in step 1 is key — it bundles the two hypotheses together cleanly.

**(b)** P ↔ ¬P

> Your answer and chain:
> This should reduce to F in about 4 steps.

> COMPLETED (Contradiction — reduces to F):
> ```
> P ↔ ¬P
>   ≡  (P → ¬P) ∧ (¬P → P)    [ Biconditional ]
>   ≡  (¬P ∨ ¬P) ∧ (¬¬P ∨ P)  [ Implication ×2 ]
>   ≡  ¬P ∧ (P ∨ P)            [ Idempotence: A∨A≡A; Double Negation: ¬¬P≡P ]
>   ≡  ¬P ∧ P                  [ Idempotence: A∨A≡A ]
>   ≡  F                        [ Complement: A∧¬A≡F ]
> ```
> Exactly 4 steps. A thing can never have the same truth value as its own negation — the algebra forces F.

---

## E5. (Harder) Prove the Biconditional Equivalence

Show that: **P ↔ Q  ≡  (P ∧ Q) ∨ (¬P ∧ ¬Q)**

This is the "same-truth-value" reading of the biconditional — it's true exactly when P and Q are both true or both false.

> Your chain:
> Hint: start from the Biconditional law P↔Q ≡ (P→Q)∧(Q→P), expand each →, then distribute.

> You got steps 1-2 right, then stalled. The issue: `¬P v Q ^ ¬Q v P` needs brackets — without them
> ∧ binds tighter than ∨ so it's ambiguous. Write it as `(¬P ∨ Q) ∧ (¬Q ∨ P)`, then distribute.
>
> COMPLETED:
> ```
> P ↔ Q
>   ≡  (P → Q) ∧ (Q → P)                               [ Biconditional ]
>   ≡  (¬P ∨ Q) ∧ (¬Q ∨ P)                             [ Implication ×2 ]
>   ≡  ((¬P ∨ Q) ∧ ¬Q) ∨ ((¬P ∨ Q) ∧ P)              [ Distributivity: A∧(B∨C)≡(A∧B)∨(A∧C) ]
>   ≡  ((¬P∧¬Q) ∨ (Q∧¬Q)) ∨ ((¬P∧P) ∨ (Q∧P))        [ Distributivity ×2 ]
>   ≡  ((¬P∧¬Q) ∨ F) ∨ (F ∨ (Q∧P))                   [ Complement ×2: Q∧¬Q≡F, ¬P∧P≡F ]
>   ≡  (¬P ∧ ¬Q) ∨ (Q ∧ P)                            [ Identity ×2: F∨A≡A ]
>   ≡  (P ∧ Q) ∨ (¬P ∧ ¬Q)                            [ Commutativity of ∧ in both terms ]
> ```
> Reading: P↔Q is true exactly when P and Q are both true, or both false. The algebra makes this concrete.


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
