# Lesson 6 — Set Theory

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 1 and Ch. 4 basics  
**Goal:** Learn the basic language of sets so statements about objects, collections, functions, and proofs can be written precisely.

---

## 1. Motivation

Logic gave us the grammar of statements.
Set theory gives us the grammar of **collections**.

When we say things like:

- "the set of all even integers"
- "the solution set"
- "the domain of a function"
- "the image of a mapping"

we are using set-theoretic language.

This lesson is mostly about being precise with a small number of symbols.

---

## 2. The Two Most Important Symbols

### Membership

$$x \in A$$

reads:

> x is an element of A

Example:

$$3 \in \{1,2,3,4\}$$

But

$$5 \notin \{1,2,3,4\}$$

reads:

> 5 is not an element of that set

### Subset

$$A \subseteq B$$

reads:

> A is a subset of B

and means:

> every element of A is also an element of B

Formally:

$$A \subseteq B \iff \forall x\,(x \in A \Rightarrow x \in B).$$

This is important: subset is **not** about A being "smaller" in size. It is about **every element of A already being inside B**.

Example:

$$\{1,2\} \subseteq \{1,2,3\}$$

because anything in {1,2} is also in {1,2,3}.

---

## 3. Equality of Sets

Two sets are equal if they have exactly the same elements.

$$A = B \iff \forall x\,(x \in A \iff x \in B).$$

This is called **extensionality**.

So to prove two sets are equal, you usually prove both directions:

1. $$A \subseteq B$$
2. $$B \subseteq A$$

This is the **double-inclusion method**.

### Example

Let

$$A = \{1,2,3\}, \qquad B = \{3,2,1\}.$$

Then A = B because order does not matter in sets.

Also,

$$\{1,1,2,2,3\} = \{1,2,3\}$$

because repeated listing does not create new elements.

---

## 4. The Empty Set

The empty set is

$$\varnothing$$

and has no elements.

So for every x,

$$x \notin \varnothing.$$

Important fact:

$$\varnothing \subseteq A$$

for every set A.

Why? Because there are no elements of $$\varnothing$$ that could violate the subset condition.

This is a first example of a statement being true **vacuously**.

---

## 5. Basic Set Operations

Let A and B be sets.

### Union

$$A \cup B = \{x : x \in A \text{ or } x \in B\}$$

reads:

> all elements that are in A or in B or in both

### Intersection

$$A \cap B = \{x : x \in A \text{ and } x \in B\}$$

reads:

> all elements common to both sets

### Difference

$$A \setminus B = \{x : x \in A \text{ and } x \notin B\}$$

reads:

> the elements of A with anything from B removed

### Symmetric Difference

$$A \triangle B = (A \setminus B) \cup (B \setminus A)$$

reads:

> elements that are in exactly one of the two sets

---

## 6. Example With Concrete Sets

Let

$$A = \{1,2,3\}, \qquad B = \{3,4,5\}.$$

Then:

- $$A \cup B = \{1,2,3,4,5\}$$
- $$A \cap B = \{3\}$$
- $$A \setminus B = \{1,2\}$$
- $$B \setminus A = \{4,5\}$$
- $$A \triangle B = \{1,2,4,5\}$$

---

## 7. Set-Builder Notation

Sometimes it is awkward to list every element. Then we describe a set by a rule.

Example:

$$E = \{n \in \mathbb{Z} : n \text{ is even}\}$$

reads:

> E is the set of integers n such that n is even

Another example:

$$S = \{x \in \mathbb{R} : x^2 < 4\}.$$

This means all real numbers x with x^2 < 4, i.e.

$$S = (-2,2).$$

Memory rule:

- left of the colon: variable being described
- right of the colon: condition it must satisfy

---

## 8. Power Set

The power set of A is the set of all subsets of A.

$$\mathcal{P}(A) = \{X : X \subseteq A\}$$

Example:

If

$$A = \{1,2\},$$

then

$$\mathcal{P}(A) = \{\varnothing, \{1\}, \{2\}, \{1,2\}\}.$$

So if A has 2 elements, its power set has 4 elements.

In general, if A has n elements, then

$$|\mathcal{P}(A)| = 2^n.$$

Why? Each element has two choices in a subset:

- include it
- do not include it

So n independent yes/no choices give $$2^n$$ subsets.

This is exactly what the Rust implementation in [school2/phase-00-mathematical-maturity/src/sets.rs](school2/phase-00-mathematical-maturity/src/sets.rs) is doing with bitmasks.

---

## 9. Cartesian Product

The Cartesian product of A and B is

$$A \times B = \{(a,b) : a \in A,\; b \in B\}.$$

This is the set of **ordered pairs**.

Example:

If

$$A = \{1,2\}, \qquad B = \{x,y\},$$

then

$$A \times B = \{(1,x),(1,y),(2,x),(2,y)\}.$$

Order matters:

$$ (1,x) \neq (x,1). $$

---

## 10. Indexed Families and Big Unions / Intersections

Sometimes we have many sets at once: $$A_1, A_2, A_3, \dots$$

Then:

$$\bigcup_{i=1}^{n} A_i$$

means all elements that lie in **at least one** of the sets.

And:

$$\bigcap_{i=1}^{n} A_i$$

means all elements that lie in **every** one of the sets.

This is just union/intersection repeated many times.

---

## 11. Common Laws

These behave like algebraic laws.

### Commutative laws

$$A \cup B = B \cup A, \qquad A \cap B = B \cap A$$

### Associative laws

$$A \cup (B \cup C) = (A \cup B) \cup C$$

$$A \cap (B \cap C) = (A \cap B) \cap C$$

### Distributive laws

$$A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$$

$$A \cup (B \cap C) = (A \cup B) \cap (A \cup C)$$

### Identity laws

$$A \cup \varnothing = A, \qquad A \cap \varnothing = \varnothing$$

### Idempotent laws

$$A \cup A = A, \qquad A \cap A = A$$

These are usually proved by element-chasing:

> take an arbitrary x and show x is in the left side iff x is in the right side

---

## 12. How Set Proofs Usually Work

There are three standard proof patterns.

### Pattern A: prove subset

To prove

$$A \subseteq B,$$

do this:

1. Let x be arbitrary.
2. Assume $$x \in A$$.
3. Prove $$x \in B$$.

### Pattern B: prove set equality

To prove

$$A = B,$$

usually prove:

1. $$A \subseteq B$$
2. $$B \subseteq A$$

### Pattern C: prove two sets are not equal

Find a witness.

That means: find some x such that

- x is in one set, but
- x is not in the other

---

## 13. Example Proof — A \setminus B \subseteq A

Take arbitrary x and assume

$$x \in A \setminus B.$$

By definition of difference, this means:

- $$x \in A$$
- $$x \notin B$$

In particular, $$x \in A$$.

So every element of $$A \setminus B$$ is in A, hence

$$A \setminus B \subseteq A. \quad \square$$

---

## 14. Example Proof — A \cap (B \cup C) = (A \cap B) \cup (A \cap C)

We prove both inclusions.

### First inclusion

Take arbitrary x and assume

$$x \in A \cap (B \cup C).$$

Then:

- $$x \in A$$
- $$x \in B \cup C$$

So either $$x \in B$$ or $$x \in C$$.

- If $$x \in B$$, then $$x \in A \cap B$$.
- If $$x \in C$$, then $$x \in A \cap C$$.

Therefore

$$x \in (A \cap B) \cup (A \cap C).$$

So

$$A \cap (B \cup C) \subseteq (A \cap B) \cup (A \cap C).$$

### Second inclusion

Take arbitrary x and assume

$$x \in (A \cap B) \cup (A \cap C).$$

Then either:

- $$x \in A \cap B$$, or
- $$x \in A \cap C$$.

In the first case, x is in A and in B, hence in A and in B union C.
In the second case, x is in A and in C, hence in A and in B union C.

So in both cases,

$$x \in A \cap (B \cup C).$$

Therefore

$$ (A \cap B) \cup (A \cap C) \subseteq A \cap (B \cup C). $$

Thus the sets are equal. □

---

## 15. Rust Interpretation

For finite sets, a Rust `HashSet<T>` is a concrete model of many set operations.

Examples in [school2/phase-00-mathematical-maturity/src/sets.rs](school2/phase-00-mathematical-maturity/src/sets.rs):

- `union(a, b)` corresponds to $$A \cup B$$
- `intersection(a, b)` corresponds to $$A \cap B$$
- `difference(a, b)` corresponds to $$A \setminus B$$
- `power_set(s)` corresponds to $$\mathcal{P}(S)$$ for finite S

Important caveat:

- mathematical sets may be infinite
- a Rust `HashSet` is always finite in memory

So the code is a model for finite examples, not the whole abstract theory.

---

## 16. Memory Version

- $$x \in A$$ means x is in A
- $$A \subseteq B$$ means every element of A is also in B
- prove set equality by double inclusion
- $$A \cup B$$ means in A or B
- $$A \cap B$$ means in both
- $$A \setminus B$$ means in A but not in B
- $$\mathcal{P}(A)$$ is the set of all subsets of A
- set proofs are usually element-chasing proofs
