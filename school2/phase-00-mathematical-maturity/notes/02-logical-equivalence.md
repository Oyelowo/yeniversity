# Lesson 2 — Logical Equivalence (Algebraic Style)

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 1 §1.3  
**Goal:** Prove that two formulas are equivalent by algebraic manipulation — no truth tables.

---

## 1. Why Avoid Truth Tables?

Truth tables work but they scale badly. With n variables you need 2ⁿ rows:
- 2 vars → 4 rows  
- 3 vars → 8 rows  
- 10 vars → 1024 rows  

The algebraic method lets you prove equivalences in a few lines regardless of how many variables are involved. It is also the method used in real mathematical proofs.

---

## 2. The Law Table — Your Algebra Rulebook

Memorise these. You will use them constantly.

| Name | Law |
|------|-----|
| **Double Negation** | ¬¬P ≡ P |
| **Identity** | P ∧ T ≡ P   \|   P ∨ F ≡ P |
| **Domination** | P ∨ T ≡ T   \|   P ∧ F ≡ F |
| **Idempotence** | P ∧ P ≡ P   \|   P ∨ P ≡ P |
| **Commutativity** | P ∧ Q ≡ Q ∧ P   \|   P ∨ Q ≡ Q ∨ P |
| **Associativity** | (P ∧ Q) ∧ R ≡ P ∧ (Q ∧ R)   \|   (P ∨ Q) ∨ R ≡ P ∨ (Q ∨ R) |
| **Distributivity** | P ∧ (Q ∨ R) ≡ (P ∧ Q) ∨ (P ∧ R)   \|   P ∨ (Q ∧ R) ≡ (P ∨ Q) ∧ (P ∨ R) |
| **De Morgan 1** | ¬(P ∧ Q) ≡ ¬P ∨ ¬Q |
| **De Morgan 2** | ¬(P ∨ Q) ≡ ¬P ∧ ¬Q |
| **Contrapositive** | P → Q ≡ ¬Q → ¬P |
| **Implication** | P → Q ≡ ¬P ∨ Q |
| **Biconditional** | P ↔ Q ≡ (P → Q) ∧ (Q → P) |
| **Exportation** | P → (Q → R) ≡ (P ∧ Q) → R |
| **Absorption** | P ∨ (P ∧ Q) ≡ P   \|   P ∧ (P ∨ Q) ≡ P |
| **Complement** | P ∧ ¬P ≡ F   \|   P ∨ ¬P ≡ T |

---

## 3. The Method — Chain of Equivalences

Write a sequence where each step is justified by exactly one law:

```
Formula A
  ≡  ... step 1 ...   [ Law name ]
  ≡  ... step 2 ...   [ Law name ]
  ≡  Formula B
```

When you reach B, you have proved A ≡ B. No rows, no tables — just substitution.

**Critical rule:** You may replace any *sub-formula* with an equivalent one. The rest of the formula stays unchanged.

---

## 4. Worked Example 1 — Simplify ¬(P → Q)

We want to find a simpler equivalent.

```
¬(P → Q)
  ≡  ¬(¬P ∨ Q)          [ Implication: P→Q ≡ ¬P∨Q ]
  ≡  ¬(¬P) ∧ ¬Q         [ De Morgan 2: ¬(A∨B) ≡ ¬A∧¬B, with A=¬P, B=Q ]
  ≡  P ∧ ¬Q             [ Double Negation: ¬¬P ≡ P ]
```

Result: **¬(P → Q) ≡ P ∧ ¬Q**

This is the same identity we verified in E3(b) with a truth table.  
Notice how much faster this was.

---

## 5. Worked Example 2 — Prove (P → Q) ∧ (P → R) ≡ P → (Q ∧ R)

This says: "P implies Q, and P implies R" is the same as "P implies both Q and R."

```
(P → Q) ∧ (P → R)
  ≡  (¬P ∨ Q) ∧ (¬P ∨ R)     [ Implication, applied twice ]
  ≡  ¬P ∨ (Q ∧ R)             [ Distributivity: A∨(B∧C) wait — let's be careful ]
```

Distributivity here: `(A ∨ B) ∧ (A ∨ C) ≡ A ∨ (B ∧ C)` with A = ¬P, B = Q, C = R:

```
(¬P ∨ Q) ∧ (¬P ∨ R)
  ≡  ¬P ∨ (Q ∧ R)             [ Distributivity ]
  ≡  P → (Q ∧ R)              [ Implication in reverse ]
```

Result: **(P → Q) ∧ (P → R) ≡ P → (Q ∧ R)** ✓

---

## 6. Worked Example 3 — Contrapositive in Action

Prove: `¬Q → ¬P ≡ P → Q`

```
¬Q → ¬P
  ≡  ¬(¬Q) ∨ ¬P     [ Implication ]
  ≡  Q ∨ ¬P         [ Double Negation ]
  ≡  ¬P ∨ Q         [ Commutativity ]
  ≡  P → Q          [ Implication in reverse ]
```

This is the **contrapositive law** derived, not assumed. Internalise it: *an implication is always equivalent to its contrapositive*. This is used constantly in proofs by contrapositive.

---

## 7. Common Mistakes to Avoid

| Mistake | Why it's wrong |
|---------|----------------|
| Applying a law to the *wrong* connective | `¬(P ∧ Q) ≡ ¬P ∧ ¬Q` — this is De Morgan **swapped** (∧ must become ∨) |
| Confusing contrapositive with converse | Contrapositive of P→Q is ¬Q→¬P (equivalent). Converse is Q→P (NOT equivalent). |
| Skipping steps | Each step must be justified by exactly one law |
| "Cancelling" a variable | You cannot drop P from `(P ∨ Q) ∧ P` by "cancelling" — use Absorption: `P ∧ (P ∨ Q) ≡ P` |

---

## 8. The Substitution Theorem (Formal)

If A ≡ B, then any formula containing A remains equivalent when A is replaced by B anywhere inside it.

This is what authorises every step in a chain of equivalences — you are substituting one sub-formula for an equivalent one.

---

## 9. Tautologies and Contradictions Revisited

A formula is a **tautology** if it can be simplified to **T** using the laws.  
A formula is a **contradiction** if it simplifies to **F**.

**Example:** Show `P ∨ ¬P` is a tautology purely algebraically — it directly matches the Complement law `P ∨ ¬P ≡ T`. Done in one step.

**Example:** Show `(P → Q) ∧ P ∧ ¬Q` is a contradiction.

```
(P → Q) ∧ P ∧ ¬Q
  ≡  (¬P ∨ Q) ∧ P ∧ ¬Q         [ Implication ]
  ≡  ((¬P ∧ P) ∨ (Q ∧ P)) ∧ ¬Q [ Distributivity ]
  ≡  (F ∨ (Q ∧ P)) ∧ ¬Q        [ Complement: ¬P∧P ≡ F ]
  ≡  (Q ∧ P) ∧ ¬Q              [ Identity: F∨A ≡ A ]
  ≡  P ∧ (Q ∧ ¬Q)              [ Associativity + Commutativity ]
  ≡  P ∧ F                     [ Complement: Q∧¬Q ≡ F ]
  ≡  F                         [ Domination: A∧F ≡ F ]
```

This formula says "P→Q is true, P is true, and Q is false" — which is impossible. The algebra confirms it.

---

## 10. Rust Code — Exercises

Open `src/logic.rs`. Your tasks:

```rust
// 1. Implement `implies(p: bool, q: bool) -> bool`
//    using ONLY NOT and OR — do not use Rust's `if` or `>`
//    (matching the Implication law: P→Q ≡ ¬P∨Q)

// 2. Implement `equivalent(p: bool, q: bool, f: fn(bool,bool)->bool, g: fn(bool,bool)->bool) -> bool`
//    Returns true if f(p,q) == g(p,q) for ALL four (T,T),(T,F),(F,T),(F,F)
//    Hint: use a loop over all pairs like in demorgan_check

// 3. Use `equivalent` to assert in a test that:
//    ¬(P→Q)  ≡  P∧¬Q
```

---

## Key Takeaways

- **Algebraic proof > truth table** for any formula with 3+ variables
- **Implication law** `P→Q ≡ ¬P∨Q` is the key to expanding most implications
- **De Morgan's laws** let you push negation inward through ∧ and ∨
- **Contrapositive** is the single most important equivalence for theorem proving
- Every step in a chain must be justified — don't skip or combine steps until comfortable

---

## Exercises

Go to `exercises/02-logical-equivalence.md`
