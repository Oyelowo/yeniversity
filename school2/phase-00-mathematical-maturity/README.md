# Phase 00 — Mathematical Maturity

**Duration:** 4–8 weeks  
**Prerequisites:** None — this is the entry point  
**Runs alongside:** Nothing; complete this before Phase 01

---

## Why This Phase Exists

Mathematics is a language with a grammar of logic and proof. Everything that follows — calculus,
physics, circuit theory, AI — is written in this language. Without fluency in it, you can only
follow recipes. With it, you can derive, invent, and verify anything.

This phase is not about computation. It is about *reasoning*.

---

## Learning Objectives

- [ ] Understand propositional and predicate logic; construct truth tables; identify tautologies
- [ ] Know the standard proof techniques: direct proof, proof by contradiction, proof by contrapositive, proof by induction (weak and strong), proof by construction
- [ ] Understand sets, subsets, power sets, Cartesian products, set operations
- [ ] Understand relations, equivalence relations, partial orders
- [ ] Understand functions: injective, surjective, bijective, composition, inverses
- [ ] Understand cardinality: finite, countable, uncountable sets; Cantor's diagonal argument
- [ ] Read and write rigorous mathematical proofs without assistance
- [ ] Understand the axiomatic method: what axioms are and why mathematics is built on them

---

## Topics

### Logic
- Propositions and connectives (AND, OR, NOT, implication, biconditional)
- Truth tables
- Tautologies and contradictions
- Logical equivalence
- Quantifiers: ∀ (for all) and ∃ (there exists)
- Nested quantifiers and their negation

### Proof Techniques
- Direct proof
- Proof by contrapositive
- Proof by contradiction (reductio ad absurdum)
- Proof by mathematical induction (weak form)
- Proof by strong induction
- Proof by smallest counterexample (well-ordering principle)
- Constructive vs. non-constructive proofs

### Set Theory
- Sets and membership
- Subsets, equality
- Power sets
- Set operations: union, intersection, difference, complement, symmetric difference
- Cartesian products
- Indexed families of sets
- Partitions

### Relations and Functions
- Binary relations; domain and range
- Reflexive, symmetric, antisymmetric, transitive properties
- Equivalence relations and equivalence classes (quotient sets)
- Partial and total orders
- Functions as relations; injectivity, surjectivity, bijectivity
- Composition and inverses
- Image and preimage of a set

### Cardinality
- Finite and infinite sets
- Countable vs. uncountable sets
- Bijection proofs of equal cardinality
- Cantor's diagonal argument (ℝ is uncountable)
- The continuum hypothesis (overview)

---

## Core Texts

| Text | Notes |
|------|-------|
| **How to Prove It** — Velleman (3rd ed.) | The single best book for this phase. Works through all proof techniques with exercises. Do every exercise. |
| **Book of Proof** — Hammack | Free at bookofproof.org. Good secondary reference, different examples from Velleman. |
| **Mathematics: Its Content, Methods and Meaning** — Aleksandrov et al. | Broad overview of what mathematics is and why. Read for perspective, not detail. |

---

## Supplementary
- MIT 6.042J *Mathematics for Computer Science* — free on OCW; great problem sets, slightly different framing
- *Naive Set Theory* — Halmos — short, elegant; read after Velleman for deeper set theory

---

## Exercises

Work every problem in Velleman chapters 1–6.  
Mark difficult proofs and return to re-derive them one week later without notes.  
Write at least 3 original proofs per week on topics of your choosing.

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Truth table generator | Takes a propositional formula as input, generates its full truth table |
| Proof verifier (toy) | Encodes simple natural-deduction rules; verifies step-by-step proofs |
| Set operations library | Implements union, intersection, power set, cartesian product generically |
| Cantor diagonalisation demo | Shows that any enumeration of binary strings is incomplete |

Notes on these are in `src/lib.rs`. Add implementations as you work through the text.

---

## Notes & Exercises Directory Convention

```
notes/
  01-propositional-logic.md
  02-quantifiers-and-predicates.md
  03-proof-techniques.md
  04-sets-and-functions.md
  05-cardinality.md

exercises/
  01-logic-exercises.md        ← problems + your worked solutions
  02-proof-exercises.md
  03-set-theory-exercises.md
```

---

## Completion Criteria

You are ready for Phase 01 when you can:
1. Write a rigorous proof of any induction claim on demand
2. Negate a quantified statement correctly without thinking
3. Prove that √2 is irrational from scratch in under 5 minutes
4. Describe the difference between a countable and uncountable set and prove it
