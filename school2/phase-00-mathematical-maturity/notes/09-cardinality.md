# Lesson 9 — Cardinality

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 7; Hammack *Book of Proof*, Ch. 13  
**Goal:** Understand what it means for sets to have the same size, distinguish countable from uncountable sets, and follow Cantor's diagonal argument.

---

## 1. Motivation

How do you compare the sizes of infinite sets?

You cannot count them, and you cannot subtract ("infinity minus infinity" is meaningless).  
The answer — due to Cantor — is to use **bijections**.

Two sets have the same size if and only if there is a bijection between them.  
This single idea leads to one of the most surprising results in all of mathematics:  
not all infinities are the same size.

---

## 2. Cardinality via Bijections

**Definition.** Two sets A and B have the **same cardinality**, written $$|A| = |B|$$, if there exists a bijection $$f : A \to B$$.

For finite sets this agrees with the usual notion of "same number of elements."

**Example.** $$|\{a, b, c\}| = |\{1, 2, 3\}|$$ because $$a \mapsto 1, b \mapsto 2, c \mapsto 3$$ is a bijection.

---

## 3. Comparing Infinite Sets

### A = ℕ and E = even naturals have the same cardinality

Define $$f : \mathbb{N} \to E$$ by $$f(n) = 2n$$.

- Injective: $$2n = 2m \Rightarrow n = m$$.
- Surjective: every even number $$2k$$ has preimage $$k$$.

So $$|\mathbb{N}| = |E|$$, even though E is a *proper subset* of ℕ.

**This is a defining feature of infinite sets:** an infinite set can be in bijection with a proper subset of itself.
(For finite sets, this is impossible.)

---

## 4. Countable Sets

A set A is **countably infinite** if $$|A| = |\mathbb{N}|$$, i.e., there is a bijection between A and the natural numbers.

Informally: a set is countably infinite if you can list its elements as a (non-repeating) sequence that eventually reaches every element:

$$a_0, a_1, a_2, a_3, \ldots$$

A set is **countable** if it is finite or countably infinite.

### Standard countably infinite sets

| Set | Why countable |
|-----|--------------|
| $$\mathbb{N}$$ | by definition |
| $$\mathbb{Z}$$ | list as $$0, 1, -1, 2, -2, 3, -3, \ldots$$ |
| $$\mathbb{Q}$$ | diagonal sweep of the $$\mathbb{Z} \times \mathbb{Z}^+$$ grid |
| Any finite union of countable sets | interleaving argument |
| Any countable union of countable sets | diagonal argument (positive direction) |

### ℤ is countable — explicit bijection

Define $$f : \mathbb{N} \to \mathbb{Z}$$ by:

$$f(n) = \begin{cases} n/2 & \text{if } n \text{ is even} \\ -(n+1)/2 & \text{if } n \text{ is odd} \end{cases}$$

This maps $$0 \mapsto 0,\; 1 \mapsto -1,\; 2 \mapsto 1,\; 3 \mapsto -2,\; 4 \mapsto 2, \ldots$$

Verify it is a bijection to confirm $$|\mathbb{Z}| = |\mathbb{N}|$$.

---

## 5. Uncountable Sets — ℝ is Strictly Larger

**Theorem (Cantor, 1874/1891).** $$|\mathbb{R}| > |\mathbb{N}|$$.

That is, there is no surjection $$f : \mathbb{N} \to \mathbb{R}$$, so no bijection either.

We prove this for the open interval $$(0, 1)$$, which is enough because one can show  
$$|(0, 1)| = |\mathbb{R}|$$ (via the bijection $$x \mapsto \tan(\pi x - \pi/2)$$).

---

## 6. Cantor's Diagonal Argument

**Claim.** The set $$(0,1) = \{x \in \mathbb{R} : 0 < x < 1\}$$ is not countable.

**Proof (by contradiction).**

Assume for contradiction that $$(0,1)$$ is countable.  
Then there exists a list that enumerates every real in $$(0,1)$$:

$$r_0, r_1, r_2, r_3, \ldots$$

Write each $$r_i$$ in its decimal expansion:

$$r_0 = 0.\;d_{00}\;d_{01}\;d_{02}\;d_{03}\;\cdots$$
$$r_1 = 0.\;d_{10}\;d_{11}\;d_{12}\;d_{13}\;\cdots$$
$$r_2 = 0.\;d_{20}\;d_{21}\;d_{22}\;d_{23}\;\cdots$$
$$r_3 = 0.\;d_{30}\;d_{31}\;d_{32}\;d_{33}\;\cdots$$

where $$d_{ij} \in \{0,1,\ldots,9\}$$.

**Construct the diagonal number** $$x = 0.x_0 x_1 x_2 x_3 \cdots$$ where:

$$x_i = \begin{cases} 5 & \text{if } d_{ii} \ne 5 \\ 6 & \text{if } d_{ii} = 5 \end{cases}$$

(Any two distinct digits avoiding 0 and 9 work; avoiding 0 and 9 prevents the $$0.999\ldots = 1$$ ambiguity.)

**x is in (0,1).** Since x uses only digits 5 and 6, we have $$0 < x < 1$$. ✓

**x is not in the list.** For each index i:

- . $$x_i \ne d_{ii}$$ by construction.

- Therefore $$x \ne r_i$$, because they differ in the $$i$$-th decimal place.

So x is a real number in (0,1) that is not $$r_i$$ for any i.  
But our assumption was that the list contains *every* element of (0,1). Contradiction. ∎

Therefore (0,1) is uncountable, and so is $$\mathbb{R}$$.

---

## 7. The Argument as a Diagram

```
      col 0   col 1   col 2   col 3  ...
r₀ = 0. [d₀₀]  d₀₁   d₀₂   d₀₃  ...    ← diagonal cell (0,0)
r₁ = 0.  d₁₀  [d₁₁]  d₁₂   d₁₃  ...    ← diagonal cell (1,1)
r₂ = 0.  d₂₀   d₂₁  [d₂₂]  d₂₃  ...    ← diagonal cell (2,2)
r₃ = 0.  d₃₀   d₃₁   d₃₂  [d₃₃] ...    ← diagonal cell (3,3)

  x = 0.  x₀     x₁     x₂     x₃  ...  differs from every rᵢ at position i
```

The key insight: if the list has infinitely many rows, we can still construct x by reading
off the diagonal and flipping each digit — x then disagrees with every row in at least one spot.

---

## 8. Why This Works (and Why It's Subtle)

The argument does not say "the list misses some elements."  
It says: **given any proposed list**, we can construct a specific number not on it.

This works for *any* attempted enumeration — no matter how clever.  
That is the force of the proof.

The use of the two digits 5 and 6 (or any pair avoiding 0 and 9) is to sidestep the  
non-uniqueness of decimal representations (e.g. $$0.4999\ldots = 0.5000\ldots$$).

---

## 9. Sizes of Infinity — The Hierarchy

Cantor did not stop at two levels. He showed:

$$|\mathbb{N}| < |\mathcal{P}(\mathbb{N})| < |\mathcal{P}(\mathcal{P}(\mathbb{N}))| < \cdots$$

**Cantor's theorem:** For any set A, $$|A| < |\mathcal{P}(A)|$$.

This gives an infinite tower of strictly increasing cardinalities.

### Standard notation

| Symbol | Meaning | Cardinality name |
|--------|---------|-----------------|
| $$\aleph_0$$ | $$|\mathbb{N}|$$ | aleph-null, countably infinite |
| $$\mathfrak{c}$$ | $$|\mathbb{R}|$$ | the cardinality of the continuum |
| — | $$|\mathcal{P}(\mathbb{R})|$$ | strictly larger again |

It is a theorem that $$\mathfrak{c} = |\mathcal{P}(\mathbb{N})| = 2^{\aleph_0}$$.

---

## 10. The Continuum Hypothesis (Overview)

The **Continuum Hypothesis (CH)** asks whether there is a cardinality strictly between $$\aleph_0$$ and $$\mathfrak{c}$$.

Gödel (1940) showed CH cannot be *disproved* from the standard axioms (ZFC).  
Cohen (1963) showed CH cannot be *proved* from ZFC either.

CH is therefore **independent** of ZFC: you can consistently add it or its negation as an axiom.

This does not affect anything in this curriculum — it is mentioned to show that  
cardinality touches the deepest foundations of mathematics.

---

## 11. Summary

| Concept | Definition / Key fact |
|---------|----------------------|
| Same cardinality | bijection exists between the two sets |
| Countably infinite | bijection with ℕ exists |
| Countable | finite or countably infinite |
| ℤ countable | interleaving list |
| ℚ countable | diagonal sweep of fraction grid |
| ℝ uncountable | Cantor's diagonal argument |
| Cantor's theorem | $$|A| < |\mathcal{P}(A)|$$ for every A |
| Continuum hypothesis | independence from ZFC (not provable, not disprovable) |
