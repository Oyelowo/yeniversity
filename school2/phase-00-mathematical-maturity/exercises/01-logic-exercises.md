# Exercises — Lesson 1: Propositional Logic

Write your solutions here. Show all reasoning.

---

## E1. Proposition or Not?

(a) "2 + 2 = 5"
> Your answer: 4
> CORRECTED: YES — it is a proposition. It is a declarative statement with a definite truth value (FALSE). The question asks to *classify* the sentence, not solve the arithmetic.

(b) "What time is it?"
> Your answer: now(): 01:44PM
> CORRECTED: NO — not a proposition. It is a question. Questions have no truth value.

(c) "n is even" (n unspecified)
> Your answer: if n % 2 == 0
> CORRECTED: NO — not a proposition. Without a bound value for n the truth value is undefined. This is a **predicate** (propositional function). It becomes a proposition only when n is specified: "6 is even" (TRUE), "7 is even" (FALSE).

(d) "There exists an integer n such that n² = 2"
> Your answer: 2^1/2 i.e sqrt(2)
> CORRECTED: YES — it is a proposition, and it is FALSE. sqrt(2) is irrational, so no integer satisfies n²=2. The sentence makes a definite claim, so it has a fixed truth value.

---

## E2. Truth Tables

**(a)** ¬(P → Q)

| P | Q | P → Q  | ¬(P → Q)  |
|---|---|--------|-----------|
| T | T | T      | F         |
| T | F | F      | T         |
| F | T | T      | F         |
| F | F | T      | F         |

**(b)** (P ∨ Q) ∧ ¬P

| P | Q | P ∨ Q | ¬P | (P ∨ Q) ∧ ¬P |
|---|---|-------|----|--------------|
| T | T |   T   |  F |        F     |
| T | F |   T   |  F |        F     |
| F | T |   T   |  T |        T     |
| F | F |   F   |  T |        F     |

**(c)** (P → Q) ∧ (Q → P)

| P | Q | P→Q | Q→P | (P→Q)∧(Q→P) |
|---|---|-----|-----|-------------|
| T | T |  T  |  T  |      T      |
| T | F |  F  |  T  |      F      |
| F | T |  T  |  F  |      F      |
| F | F |  T  |  T  |      T      |

> CORRECTED row 3: T∧F = F (was incorrectly written as T). This table is exactly P↔Q — true only when P and Q match.

---

## E3. Logical Equivalence?

**(a)** Is P → Q equivalent to Q → P?

> Your answer and truth table: 
False/No
P  | Q  | P -> Q  |  Q -> P  | (p -> Q) == (Q -> P)
T    T       T         T                 T
T    F       F         T                 F (this shows not equal)
F    T       T         F                 F (this shows not equal)
F    F       T         T                 T

**(b)** Is ¬(P → Q) equivalent to P ∧ ¬Q?

> Your answer and truth table:
True/Yes
P  | Q     |  ¬Q   | P -> Q  | ¬(P -> Q) | P ^ ¬Q  |  ¬(p -> Q) == (P ^ ¬Q)
T    T        F         T           F        F                   T   
T    F        T         F           T        T                   T   
F    T        F         T           F        F                   T 
F    F        T         T           F        F                   T  

---

## E4. Translation to Symbolic Logic

Define your propositions first (P = "...", Q = "..."), then write the formula.

**(a)** "You can pass only if you study."
> P = Pass, Q = Study
> Formula:
P(Pass) -> Q(Study)
P | Q | P -> R
T   T      T
T   F      F
F   T      T
F   F      T


**(b)** "You will get the job if and only if you know C++ or Python."
> P = Know Python, Q = Know C++, R = Get the job
> Formula: **R ↔ (P ∨ Q)**
> CORRECTED: the formula label was written as "P(Pass) <-> Q(Study)" and the column header as "(Q v R) <-> R" — both wrong. The truth values you computed were correct.

| P | Q | P∨Q | R | R ↔ (P∨Q) |
|---|---|-----|---|-----------|
| T | T |  T  | T |     T     |
| T | T |  T  | F |     F     |
| T | F |  T  | T |     T     |
| T | F |  T  | F |     F     |
| F | T |  T  | T |     T     |
| F | T |  T  | F |     F     |
| F | F |  F  | T |     F     |
| F | F |  F  | F |     T     |
**(c)** "If it rains and you have no umbrella, you will get wet."
> P = Rains , Q = No umbrella , R =  Get wet
> Formula:
P  | Q | (P ^ Q) | R | (P ^ Q) -> R
T    T      T      T       T (true)
T    T      T      F       F
T    F      F      T       T
T    F      F      F       T
F    T      F      T       T
F    T      F      F       T
---

## E5. (Harder) Prove Exportation

Show that P → (Q → R) ≡ (P ∧ Q) → R using a truth table.

| P | Q | R | Q→R | P→(Q→R) | P∧Q | (P∧Q)→R | Equal? |
|---|---|---|-----|---------|-----|---------|--------|
| T | T | T |  T  |    T    |  T  |    T    |   T    |
| T | T | F |  F  |    F    |  T  |    F    |   T    |
| T | F | T |  T  |    T    |  F  |    T    |   T    |
| T | F | F |  T  |    T    |  F  |    T    |   T    |
| F | T | T |  T  |    T    |  F  |    T    |   T    |
| F | T | F |  F  |    T    |  F  |    T    |   T    |
| F | F | T |  T  |    T    |  F  |    T    |   T    |
| F | F | F |  T  |    T    |  F  |    T    |   T    |

> Conclusion:
they are equal