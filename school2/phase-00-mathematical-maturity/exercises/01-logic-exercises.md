# Exercises — Lesson 1: Propositional Logic

Write your solutions here. Show all reasoning.

---

## E1. Proposition or Not?

(a) "2 + 2 = 5"
> Your answer: 4

(b) "What time is it?"
> Your answer: now(): 01:44PM

(c) "n is even" (n unspecified)
> Your answer: if n % 2 == 0

(d) "There exists an integer n such that n² = 2"
> Your answer: 2^1/2 i.e √2

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
| F | T |  T  |  F  |      T      |
| F | F |  T  |  T  |      T      |

---

## E3. Logical Equivalence?

**(a)** Is P → Q equivalent to Q → P?

> Your answer and truth table: 
No
P  Q  P -> Q   Q -> P  (p -> Q) == (Q == P)

**(b)** Is ¬(P → Q) equivalent to P ∧ ¬Q?

> Your answer and truth table:

---

## E4. Translation to Symbolic Logic

Define your propositions first (P = "...", Q = "..."), then write the formula.

**(a)** "You can pass only if you study."
> P = , Q =  
> Formula:

**(b)** "You will get the job if and only if you know C++ or Python."
> P = , Q = , R =  
> Formula:

**(c)** "If it rains and you have no umbrella, you will get wet."
> P = , Q = , R =  
> Formula:

---

## E5. (Harder) Prove Exportation

Show that P → (Q → R) ≡ (P ∧ Q) → R using a truth table.

| P | Q | R | Q→R | P→(Q→R) | P∧Q | (P∧Q)→R | Equal? |
|---|---|---|-----|---------|-----|---------|--------|
| T | T | T | | | | | |
| T | T | F | | | | | |
| T | F | T | | | | | |
| T | F | F | | | | | |
| F | T | T | | | | | |
| F | T | F | | | | | |
| F | F | T | | | | | |
| F | F | F | | | | | |

> Conclusion:
