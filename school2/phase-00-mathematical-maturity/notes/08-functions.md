# Lesson 8 — Functions

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 5; Hammack *Book of Proof*, Ch. 12  
**Goal:** Understand functions as a special kind of relation; master injectivity, surjectivity, bijectivity, composition, and inverses.

---

## 1. Motivation

A relation says "a is connected to b in some way."  
A function says "given a, there is exactly one b it connects to."

Functions are the fundamental tool for *transforming* mathematical objects:  
mappings between sets, substitutions in proofs, variable bindings in languages,  
type-level transformations in type theory.

---

## 2. Definition

A **function** $$f : A \to B$$ is a relation between A and B such that every element of A is paired
with **exactly one** element of B.

Formally, f is a subset of $$A \times B$$ such that:

$$\forall a \in A,\; \exists! \, b \in B,\; (a, b) \in f.$$

The symbol $$\exists!$$ means "there exists exactly one."

### Terminology

- **Domain:** the set A — the inputs
- **Codomain:** the set B — the declared output type
- **Range / Image:** the set of outputs that actually appear: $$\{f(a) : a \in A\}$$

The **range is always a subset of the codomain**, but they need not be equal.

### Common pitfall: the formula is not the whole function

Injective, surjective, and bijective are properties of the full arrow $$f : A \to B$$,
not just of the expression "$$f(x) = \cdots$$".

The same rule can define different functions when the codomain changes.

- $$f : \mathbb{Z} \to \mathbb{Z},\; f(n) = 2n$$ is injective but not surjective.
- If $$E = \{2k : k \in \mathbb{Z}\}$$ and $$g : \mathbb{Z} \to E,\; g(n) = 2n$$, then $$g$$ is bijective.

So when classifying a function, always ask two questions:

1. What is the rule?
2. What is the codomain?

---

## 3. Three Key Properties

### Injective (one-to-one)

<!-- `$$f$$` is injective if distinct inputs produce distinct outputs: -->
`f` is injective if distinct inputs produce distinct outputs:

$$\forall a_1, a_2 \in A,\; f(a_1) = f(a_2) \Rightarrow a_1 = a_2.$$

Equivalently (contrapositive form, often easier to use in proofs):

$$a_1 \ne a_2 \Rightarrow f(a_1) \ne f(a_2).$$

**Intuition:** no two arrows land on the same element of B.

**Example:** $$f : \mathbb{Z} \to \mathbb{Z},\; f(n) = 2n$$ is injective.  
If $$2n = 2m$$ then $$n = m$$. ✓

**Non-example:** $$f(n) = n^2$$ on integers is not injective.  
$$f(2) = f(-2) = 4$$ but $$2 \ne -2$$. ✗

---

### Surjective (onto)

`f` is surjective if every element of B is hit by at least one element of A:

$$\forall b \in B,\; \exists a \in A,\; f(a) = b.$$

**Intuition:** no element of B is left uncovered.

**Example:** $$f : \mathbb{Z} \to \mathbb{Z},\; f(n) = n - 3$$ is surjective.  
Given any $$b$$, take $$a = b + 3$$; then $$f(a) = b$$. ✓

**Non-example:** $$f : \mathbb{Z} \to \mathbb{Z},\; f(n) = 2n$$ is not surjective.  
The integer 3 has no preimage (there is no integer $$n$$ with $$2n = 3$$). ✗

*Note how doubling is injective but not surjective; shifting is surjective but — as we'll see — also injective.*

---

### Bijective

`f` is bijective if it is both injective and surjective.

A bijection establishes a **perfect pairing** between A and B: every element of A maps to a unique
element of B, and every element of B gets hit by exactly one element of A.

**Consequence:** bijections have inverses (see §5).

---

## 4. Quick Proof Templates

### Proving injectivity

> Assume $$f(a_1) = f(a_2)$$. Derive $$a_1 = a_2$$.

This is the standard method. Avoid trying to assume $$a_1 \ne a_2$$ unless contradiction is needed.

**Example:** show $$f(x) = 3x + 1$$ on $$\mathbb{R}$$ is injective.

*Proof.* Suppose $$f(x_1) = f(x_2)$$. Then $$3x_1 + 1 = 3x_2 + 1$$, so $$x_1 = x_2$$. ∎

---

### Proving surjectivity

> Take an arbitrary $$b \in B$$. Exhibit an $$a \in A$$ with $$f(a) = b$$.

This requires solving for a in terms of b.

**Example:** show $$f(x) = 3x + 1$$ on $$\mathbb{R}$$ is surjective.

*Proof.* Let $$b \in \mathbb{R}$$. Set $$a = \frac{b-1}{3}$$. Then $$f(a) = 3 \cdot \frac{b-1}{3} + 1 = b$$. ∎

---

### Disproving

- **Disprove injective:** find a concrete counterexample $$a_1 \ne a_2$$ with $$f(a_1) = f(a_2)$$.
- **Disprove surjective:** find a concrete $$b \in B$$ with no preimage.

---

## 5. Composition

If $$f : A \to B$$ and $$g : B \to C$$, the **composition** $$g \circ f : A \to C$$ is defined by:

$$(g \circ f)(a) = g(f(a)).$$

**Order of operations:** apply $$f$$ first, then $$g$$. The notation reads right to left.

### Composition preserves structure

| If f is   | and g is    | then $$g \circ f$$ is |
|-----------|-------------|----------------------|
| injective | injective   | injective            |
| surjective| surjective  | surjective           |
| bijective | bijective   | bijective            |

**Proof sketch for injectivity of composition:**  
Assume $$(g \circ f)(a_1) = (g \circ f)(a_2)$$. Then $$g(f(a_1)) = g(f(a_2))$$. Since g is injective,
$$f(a_1) = f(a_2)$$. Since f is injective, $$a_1 = a_2$$. ∎

---

## 6. Inverse Functions

If $$f : A \to B$$ is bijective, its **inverse** $$f^{-1} : B \to A$$ is the function defined by:

$$f^{-1}(b) = a \iff f(a) = b.$$

Properties of inverses:

$$f^{-1} \circ f = \mathrm{id}_A, \qquad f \circ f^{-1} = \mathrm{id}_B.$$

Here $$\mathrm{id}_A(a) = a$$ for all $$a$$ is the **identity function** on A.

**A bijection is the only kind of function that has an inverse.**

- If not injective: $$f^{-1}$$ would have to send one element of B to multiple elements of A — not a function.
- If not surjective: some elements of B would have no preimage — $$f^{-1}$$ would be undefined there.

---

## 7. Image and Preimage

For $$f : A \to B$$:

- The **image** of a subset $$S \subseteq A$$ is:
  $$f(S) = \{f(a) : a \in S\}.$$

- The **preimage** (inverse image) of a subset $$T \subseteq B$$ is:
  $$f^{-1}(T) = \{a \in A : f(a) \in T\}.$$

The preimage is defined even when f has no inverse function.

**Key facts:**
- $$f^{-1}(f(S)) \supseteq S$$; equality holds when f is injective.
- $$f(f^{-1}(T)) \subseteq T$$; equality holds when f is surjective.

### Rust mirror for finite functions

In `src/functions.rs`, a finite function is represented as a map together with an
explicit codomain:

- $$f : A \to B$$ corresponds to `FiniteFunction<A, B>`
- $$S \subseteq A$$ corresponds to `HashSet<A>`
- $$f(S)$$ corresponds to `image_of(&subset)`
- $$f^{-1}(T)$$ corresponds to `preimage_of(&subset)`

```rust
use std::collections::{HashMap, HashSet};

use p00_math_maturity::functions::FiniteFunction;

let onto_small_codomain = FiniteFunction::new(
  HashMap::from([(1, 2), (2, 4), (3, 6)]),
  HashSet::from([2, 4, 6]),
);
assert!(onto_small_codomain.is_injective());
assert!(onto_small_codomain.is_surjective());

let same_rule_larger_codomain = FiniteFunction::new(
  HashMap::from([(1, 2), (2, 4), (3, 6)]),
  HashSet::from([1, 2, 3, 4, 5, 6]),
);
assert!(same_rule_larger_codomain.is_injective());
assert!(!same_rule_larger_codomain.is_surjective());
```

This is the finite-set version of the codomain warning above: the rule is the same,
but surjectivity changes when the target set changes.

---

## 8. Finite-Set Counting Intuition

If $$|A| = m$$ and $$|B| = n$$:

| Property   | Necessary condition |
|------------|---------------------|
| Injective  | $$m \le n$$ (can't inject a bigger set into a smaller one) |
| Surjective | $$m \ge n$$ (every element of B needs a source) |
| Bijective  | $$m = n$$           |

This intuition breaks down for infinite sets — which is exactly what makes cardinality interesting.

---

## 9. Connection to Relations

A function $$f : A \to B$$ is a relation $$R \subseteq A \times B$$ that satisfies:

1. **Total:** $$\forall a \in A,\; \exists b \in B,\; (a,b) \in R$$  (every input has at least one output)
2. **Functional:** $$\forall a,\; (a,b_1) \in R \land (a,b_2) \in R \Rightarrow b_1 = b_2$$  (at most one output)

This unifies relations and functions as the same set-theoretic object.

---

## 10. Summary Table

| Term      | Definition (informal) | Formal condition |
|-----------|----------------------|-----------------|
| Function  | each input has exactly one output | total + functional relation |
| Injective | no two inputs share an output | $$f(a_1)=f(a_2) \Rightarrow a_1=a_2$$ |
| Surjective| every output is reached | $$\forall b,\,\exists a,\,f(a)=b$$ |
| Bijective | both | injective ∧ surjective |
| Inverse   | undo f | only exists when f is bijective |
| Composition | chain two functions | $$(g\circ f)(a) = g(f(a))$$ |
