# Coinduction for Compiler Work

**Phase:** supplemental note  
**Goal:** Explain what coinduction is, why it is the dual of induction, and where it matters in type systems, semantics, and optimizer reasoning.

---

## 1. Short Answer

Yes, coinduction is a real and important thing.

It is not just an advanced curiosity.
If you are working on:

- recursive types
- type equality / subtyping
- infinite syntax trees or cyclic graphs
- operational semantics of non-terminating programs
- equivalence of state machines
- fixed-point query semantics

then coinductive reasoning shows up naturally.

---

## 2. Induction vs Coinduction

### Induction

Induction is for things built in finitely many steps.

Examples:

- natural numbers
- finite lists
- finite syntax trees
- finite derivations

To prove a property by induction, you show:

1. it holds for the base constructors
2. if it holds for smaller pieces, it holds for the bigger object

This is usually tied to a least fixed point intuition.

### Coinduction

Coinduction is for potentially infinite or self-referential objects.

Examples:

- infinite streams
- cyclic graphs
- infinite proof trees
- non-terminating processes
- recursive types viewed observationally

To prove something coinductively, you often show that two objects cannot be distinguished by any finite observation.

This is usually tied to a greatest fixed point intuition.

---

## 3. The Useful Mental Picture

### Induction

Build from below.

If every finite construction step preserves the property, then all finitely built objects have the property.

### Coinduction

Observe forever.

If after one observation step the two systems still match in the same relation, then they are behaviorally the same.

---

## 4. Bisimulation: the Core Coinductive Pattern

The most common coinductive proof method is bisimulation.

Suppose you want to show two processes or two streams are equivalent.

You define a relation R and prove:

1. the two objects are related by R
2. whenever two objects are related by R, they have the same observable one-step behavior
3. their successors are again related by R

Then R is a bisimulation, and the objects are coinductively equal.

### Stream intuition

To prove two infinite streams are equal, it is enough to show:

1. their heads are equal
2. their tails are again related in the same way

That is the coinductive analogue of an induction step.

---

## 5. Why This Matters for Type Systems

### Recursive types

Suppose you have types like:

$$T = \mu X.\, 1 + X$$

or more operationally,

```text
type Stream = Cons(Int, Stream)
```

These are self-referential.

If your type equality or subtype relation unfolds recursive types, a naive inductive proof may fail because the structure can keep unfolding forever.

Coinduction is often the right tool because:

- you do not need to fully unfold forever
- you show the unfolded views stay in a stable relation

### Example use cases

- equi-recursive type equality
- recursive subtyping
- session type equivalence
- behavioral typing of processes

---

## 6. Why This Matters for Query Optimization

There are at least three ways coinductive thinking appears.

### 1. Fixed-point semantics

Recursive queries, Datalog, and dataflow analyses often have semantics defined as fixed points.

- inductive reading often corresponds to least fixed points
- coinductive reading often corresponds to greatest fixed points

Least fixed points capture things built from finite evidence.
Greatest fixed points capture persistent or global behavior, often relevant to liveness-style properties.

### 2. Equivalence of cyclic plans or memoized rewrite graphs

If your optimizer represents search state with shared or cyclic structure, equivalence may be more naturally shown behaviorally than by finite tree induction.

### 3. Observational equivalence

Two plans may be considered equivalent if no query context can distinguish them by outputs, cost model, or semantics under the chosen observations.

That style of same observable behavior is coinductive in flavor.

---

## 7. Least vs Greatest Fixed Points

This distinction is the real bridge to compiler work.

### Least fixed point

- smallest solution closed under the rules
- corresponds to induction
- finite derivability
- only what must be generated

### Greatest fixed point

- largest solution consistent with the rules
- corresponds to coinduction
- potentially infinite behavior allowed
- everything that cannot be ruled out by observation

If you work on type systems or semantics, you will keep seeing these two viewpoints.

---

## 8. A Tiny Example With Infinite Streams

Consider the stream of all zeros:

```text
zeros = 0 :: zeros
```

Suppose another stream is defined by:

```text
zs = 0 :: zs
```

To show zeros = zs, an inductive proof is awkward because there is no finite base case that finishes the whole object.

A coinductive proof is natural:

1. both heads are 0
2. both tails are again the same kind of stream

So they are bisimilar, hence equal as streams.

---

## 9. What to Learn Next If You Need This for a Compiler

If this connects to your current work, the useful follow-up topics are:

1. structural induction on syntax and derivations
2. well-founded induction
3. least and greatest fixed points
4. bisimulation
5. equi-recursive types and iso-recursive types
6. logical relations if you go deeper into semantics

---

## 10. What You Should Retain Right Now

- induction is for finite construction
- coinduction is for potentially infinite behavior or self-reference
- induction often corresponds to least fixed points
- coinduction often corresponds to greatest fixed points
- bisimulation is the standard coinductive proof pattern
- for compiler and type-system work, coinduction is very relevant, but it comes after the ordinary proof toolkit is solid
