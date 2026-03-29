# Structural Induction for ASTs and Typing Derivations

**Phase:** supplemental note  
**Goal:** Bridge ordinary induction to the form you will actually use in compilers: induction on syntax trees, derivation trees, and recursively defined judgments.

---

## 1. Why This Matters

Ordinary induction is over numbers.

Compiler proofs are more often about things like:

- expressions
- statements
- types
- derivation trees
- evaluation sequences

Those are not naturally indexed by a single number in the source language, but they are still built recursively.

That is where structural induction comes in.

---

## 2. The Core Idea

If an object is defined recursively, then proofs about it are usually done recursively too.

That is structural induction.

Instead of proving:

$$P(0), P(1), P(2), \dots$$

you prove:

- P holds for the base constructors
- whenever P holds for the immediate sub-objects, it holds for the bigger object built from them

So structural induction is induction over shape rather than over a numeric index.

---

## 3. Tiny AST Example

Suppose expressions are defined by:

```text
e ::= Int(n)
    | Add(e1, e2)
```

This says every expression is either:

1. an integer literal, or
2. an addition node with two child expressions

Now suppose we define `size(e)` as:

```text
size(Int(n)) = 1
size(Add(e1, e2)) = 1 + size(e1) + size(e2)
```

Claim:

> every expression e has size(e) ≥ 1

### Proof by structural induction on e

Case 1: $$e = Int(n)$$

Then

$$size(e) = 1,$$

so certainly $$size(e) \ge 1$$.

Case 2: $$e = Add(e_1, e_2)$$

Inductive hypotheses:

- $$size(e_1) \ge 1$$
- $$size(e_2) \ge 1$$

Then

$$size(e) = 1 + size(e_1) + size(e_2) \ge 1.$$

So the claim holds for `Add(e1, e2)` as well.

Therefore every expression has size at least 1. □

---

## 4. What the Induction Hypothesis Really Means Here

This part often gets glossed over.

When you are in the `Add(e1, e2)` case, you are allowed to assume the statement for `e1` and `e2` because they are the immediate subexpressions used to build the larger expression.

This is the structural analogue of assuming P(k) when proving P(k+1).

So the pattern is:

- bigger syntax node depends on smaller syntax nodes
- prove the property for the smaller nodes
- lift it to the bigger node

---

## 5. Structural Induction on Typing Derivations

Sometimes the right induction is not on the syntax tree, but on the **typing derivation**.

Example judgment:

```text
Gamma |- e : T
```

Suppose the typing rules are:

```text
Gamma |- Int(n) : Int

Gamma |- e1 : Int    Gamma |- e2 : Int
--------------------------------------
Gamma |- Add(e1, e2) : Int
```

Now suppose you want to prove:

> if $$\Gamma \vdash e : Int$$ then e contains only integer-typed subexpressions

The natural proof is by induction on the derivation of $$\Gamma \vdash e : Int$$.

Why?

Because each typing derivation was itself built by applying one typing rule on top of earlier derivations.

So derivation induction is just structural induction on proof trees.

---

## 6. Structural Induction vs Strong Induction

They are close cousins.

### Strong induction

You may use all smaller earlier cases.

### Structural induction

You may use the statement for the immediate substructures used to build the current structure.

In many formalizations, structural induction can be reduced to well-founded induction on a size measure.

But operationally, structural induction is the cleaner proof language for ASTs.

---

## 7. Where You Will Use This in Compiler Proofs

### Type preservation

Often proved by induction on evaluation derivations or typing derivations.

### Progress

Often proved by induction on typing derivations.

### Substitution lemma

Often proved by induction on typing derivations.

### Optimizer correctness

For recursive rewrites over expression trees, proofs are often by structural induction on expressions.

### Free-variable lemmas

Also usually structural induction on syntax.

---

## 8. Compiler Example: Constant Folding

Suppose you define an optimizer:

```text
fold(Int(n)) = Int(n)
fold(Add(Int(a), Int(b))) = Int(a + b)
fold(Add(e1, e2)) = Add(fold(e1), fold(e2))
```

If you want to prove:

> `eval(fold(e)) = eval(e)` for every expression e

the natural proof is structural induction on e.

Why?

Because the optimizer itself recursively traverses the AST.

The proof mirrors the program:

- literal case
- addition case
- use IH for the subexpressions

This is one of the most important proof patterns in compiler work.

---

## 9. Memory Version

- structural induction = induction on recursively defined shape
- use it for ASTs, derivations, and recursive judgments
- the IH applies to immediate substructures
- many compiler metatheory lemmas are structural-induction proofs
- if the code is recursive on syntax, the proof often is too
