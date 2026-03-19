# Lesson 1 — Propositional Logic

**Phase:** 00 — Mathematical Maturity  
**Reference:** Velleman *How to Prove It*, Chapter 1 §1.1–1.2  

---

## 1. What Is a Proposition?

A **proposition** is any declarative sentence that is either true or false —
never both, never neither. This is called the **Law of the Excluded Middle** and
it is foundational to classical logic.

**Propositions:**
- "7 is a prime number." ✓ (true)
- "Every even number greater than 2 is the sum of two primes." ✓ (true — Goldbach's conjecture is believed true; it *has* a definite truth value even if unproved)
- "The sky is green." ✓ (false)

**Not propositions:**
- "Close the door." (a command — no truth value)
- "Is x > 5?" (a question)
- "x > 5" (has truth value only once x is given — this becomes a **predicate**, covered in Lesson 3)

We denote propositions with letters: P, Q, R, ...

---

## 2. Connectives — Building Compound Propositions

From simple propositions we build compound ones using **logical connectives**.

### 2.1 Negation — NOT (¬)

| P | ¬P |
|---|-----|
| T | F |
| F | T |

"It is not the case that P."  
If P = "It is raining", then ¬P = "It is not raining."

### 2.2 Conjunction — AND (∧)

| P | Q | P ∧ Q |
|---|---|--------|
| T | T | T |
| T | F | F |
| F | T | F |
| F | F | F |

P ∧ Q is true **only when both** P and Q are true.

### 2.3 Disjunction — OR (∨)

| P | Q | P ∨ Q |
|---|---|--------|
| T | T | T |
| T | F | T |
| F | T | T |
| F | F | F |

P ∨ Q is true **when at least one** of P, Q is true.  
This is *inclusive* or. Mathematics always uses inclusive or unless stated otherwise.

### 2.4 Implication — IF…THEN (→)

This is the most important connective in mathematics.

| P | Q | P → Q |
|---|---|--------|
| T | T | T |
| T | F | **F** |
| F | T | T |
| F | F | T |

"If P then Q" is false **only when P is true and Q is false**.

The two rows where P is false are called **vacuously true**. Example:

> "If you score 100% on every exam, I will give you a car."

If you *don't* score 100%, I've broken no promise — the statement is vacuously true regardless of whether you got a car.

Terminology:
- P is the **hypothesis** (antecedent)
- Q is the **conclusion** (consequent)

### 2.5 Biconditional — IF AND ONLY IF (↔)

| P | Q | P ↔ Q |
|---|---|--------|
| T | T | T |
| T | F | F |
| F | T | F |
| F | F | T |

"P if and only if Q" (written P iff Q). True exactly when P and Q have the same truth value.

Note: P ↔ Q is equivalent to (P → Q) ∧ (Q → P).

---

## 3. The Full Truth Table for a Compound Formula

To evaluate a compound formula, build the truth table column by column,
respecting operator precedence: ¬ binds tightest, then ∧, then ∨, then →, then ↔.

**Example:** Evaluate (P ∨ Q) → ¬P

| P | Q | P ∨ Q | ¬P | (P ∨ Q) → ¬P |
|---|---|--------|-----|----------------|
| T | T | T | F | F |
| T | F | T | F | F |
| F | T | T | T | T |
| F | F | F | T | T |

This formula is not a tautology (not always true) and not a contradiction
(not always false). It is **contingent** — its truth depends on P and Q.

---

## 4. Tautologies and Contradictions

A **tautology** is a formula that is true for every possible assignment of truth values.  
A **contradiction** is a formula that is false for every assignment.

**Classic tautology — Law of Excluded Middle:**  
P ∨ ¬P

| P | ¬P | P ∨ ¬P |
|---|-----|---------|
| T | F | T |
| F | T | T |

**Classic contradiction:**  
P ∧ ¬P

| P | ¬P | P ∧ ¬P |
|---|-----|---------|
| T | F | F |
| F | T | F |

---

## 5. Logical Equivalence

Two formulas are **logically equivalent** (written ≡) if they have identical
truth tables — they agree on every row.

**De Morgan's Laws** (you must know these cold):

$$\neg(P \wedge Q) \equiv \neg P \vee \neg Q$$
$$\neg(P \vee Q) \equiv \neg P \wedge \neg Q$$

Verify the first one:

| P | Q | P ∧ Q | ¬(P ∧ Q) | ¬P | ¬Q | ¬P ∨ ¬Q |
|---|---|--------|-----------|-----|-----|----------|
| T | T | T | **F** | F | F | **F** |
| T | F | F | **T** | F | T | **T** |
| F | T | F | **T** | T | F | **T** |
| F | F | F | **T** | T | T | **T** |

Columns 4 and 7 are identical → the equivalence holds. ✓

**The Contrapositive** (crucial for proofs):

$$P \rightarrow Q \equiv \neg Q \rightarrow \neg P$$

Memorise this. Proving "if P then Q" is identical to proving "if not-Q then not-P."
Often the contrapositive is dramatically easier to prove.

---

## 6. Useful Logical Equivalences to Memorise

| Name | Equivalence |
|------|-------------|
| Double negation | ¬¬P ≡ P |
| Implication as disjunction | P → Q ≡ ¬P ∨ Q |
| Contrapositive | P → Q ≡ ¬Q → ¬P |
| De Morgan 1 | ¬(P ∧ Q) ≡ ¬P ∨ ¬Q |
| De Morgan 2 | ¬(P ∨ Q) ≡ ¬P ∧ ¬Q |
| Biconditional | P ↔ Q ≡ (P → Q) ∧ (Q → P) |
| Absorption (∧) | P ∧ (P ∨ Q) ≡ P |
| Absorption (∨) | P ∨ (P ∧ Q) ≡ P |

---

## 7. The Rust Code in Your Library

Open `school2/phase-00-mathematical-maturity/src/logic.rs`.

The `truth_table` function prints a 2-variable truth table for any Boolean formula
you pass as a closure. The `evaluate` function evaluates P → Q.

**Exercise in code:** Add a function `demorgan_check` that verifies De Morgan's
first law computationally by checking all four (P, Q) combinations. Return
`true` if the law holds for all of them.

---

## 8. Exercises — Do These Now

Work these before the next lesson. Write your answers in
`school2/phase-00-mathematical-maturity/exercises/01-logic-exercises.md`.

**E1.** Classify as proposition / not a proposition:
- (a) "2 + 2 = 5"
- (b) "What time is it?"
- (c) "n is even" (where n is unspecified)
- (d) "There exists an integer n such that n² = 2"

**E2.** Write out the full truth table (all rows) for each formula:
- (a) ¬(P → Q)
- (b) (P ∨ Q) ∧ ¬P
- (c) (P → Q) ∧ (Q → P)

**E3.** Are the following pairs logically equivalent? Prove by truth table.
- (a) P → Q and Q → P
- (b) ¬(P → Q) and P ∧ ¬Q

**E4.** Translate into symbolic logic (define your own P, Q, R):
- (a) "You can pass only if you study."
- (b) "You will get the job if and only if you know C++ or Python."
- (c) "If it rains and you have no umbrella, you will get wet."

**E5.** (Harder) Prove that P → (Q → R) is logically equivalent to (P ∧ Q) → R.
This equivalence is called **exportation** and is heavily used in proofs.

---

## Key Takeaways

1. A proposition has exactly one truth value: T or F.
2. Implication P → Q is false in exactly one case: P true, Q false.
3. The contrapositive (¬Q → ¬P) is logically equivalent to (P → Q).
4. De Morgan's laws turn ∧ into ∨ and vice versa when negating.
5. Build truth tables column by column; never jump ahead.

---

*Next: Lesson 2 — Logical Equivalence deeper dive, tautology proofs, and the algebra of logic (no truth tables needed).*
