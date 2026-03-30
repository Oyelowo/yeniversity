# Lesson 7 — Relations

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, relations basics  
**Goal:** Understand binary relations, their standard properties, equivalence relations, and partial orders.

---

## 1. Motivation

Sets tell us what objects exist in a collection.
Relations tell us how objects are connected.

Examples:

- equality on numbers
- less-than on integers
- divisibility on positive integers
- same type as, reduces to, dominates, equivalent plan to

In compiler and type-system work, relations are everywhere.

---

## 2. What a Relation Is

A binary relation on a set A is a subset of $$A \times A$$.

So a relation is just a set of ordered pairs.

If R is a relation on A, then writing

$$a \mathrel{R} b$$

means:

> the pair $$(a,b)$$ belongs to the relation R

Formally:

$$a \mathrel{R} b \iff (a,b) \in R.$$

Example:

If A = {1,2,3} and

$$R = \{(1,1),(2,2),(3,3),(1,2)\},$$

then $$1 \mathrel{R} 2$$ is true, but $$2 \mathrel{R} 1$$ is false.

---

## 3. Domain and Range

If R is a relation from A to B, then:

- the **domain** is the set of first coordinates that actually appear
- the **range** is the set of second coordinates that actually appear

Example:

If

$$R = \{(1,a),(2,a),(2,b)\},$$

then:

- domain(R) = {1,2}
- range(R) = {a,b}

---

## 4. The Four Main Properties

Let R be a relation on a set A.

### Reflexive

R is reflexive if every element is related to itself:

$$\forall a \in A,\; aRa.$$

### Symmetric

R is symmetric if relation goes both ways:

$$\forall a,b \in A,\; aRb \Rightarrow bRa.$$

### Antisymmetric

R is antisymmetric if the only way relation can go both ways is when the elements are equal:

$$\forall a,b \in A,\; (aRb \land bRa) \Rightarrow a=b.$$

Important:

- antisymmetric does **not** mean not symmetric
- a relation can be neither symmetric nor antisymmetric

### Transitive

R is transitive if chains can be collapsed:

$$\forall a,b,c \in A,\; (aRb \land bRc) \Rightarrow aRc.$$

---

## 5. Fast Intuition for the Properties

- reflexive: every node has a self-loop
- symmetric: every arrow can be reversed
- antisymmetric: two-way arrows only happen on the diagonal
- transitive: two-step paths imply a direct path

---

## 6. Standard Examples

### Equality = on any set

- reflexive: yes
- symmetric: yes
- antisymmetric: yes
- transitive: yes

### Less than or equal $$\le$$ on integers

- reflexive: yes
- symmetric: no
- antisymmetric: yes
- transitive: yes

### Less than $$<$$ on integers

- reflexive: no
- symmetric: no
- antisymmetric: yes, vacuously
- transitive: yes

### Divisibility $$\mid$$ on positive integers

- reflexive: yes, because $$a \mid a$$
- symmetric: no
- antisymmetric: yes on positive integers
- transitive: yes

---

## 7. Equivalence Relations

A relation is an **equivalence relation** if it is:

1. reflexive
2. symmetric
3. transitive

These are relations that formalize "same kind of thing".

Examples:

- equality
- congruence mod n
- has the same remainder mod n
- same connected component, in graph settings

### Example: congruence mod 3

Define on integers:

$$a \sim b \iff 3 \mid (a-b).$$

This is an equivalence relation.

Why intuitively?

- every number has the same remainder as itself
- if a has the same remainder as b, then b has the same remainder as a
- if a matches b and b matches c, then a matches c

---

## 8. Equivalence Classes

If $$\sim$$ is an equivalence relation on A, the equivalence class of a is:

$$[a] = \{x \in A : x \sim a\}.$$

This means all elements equivalent to a.

For congruence mod 3, there are three classes:

- [0] = integers congruent to 0 mod 3
- [1] = integers congruent to 1 mod 3
- [2] = integers congruent to 2 mod 3

These classes partition the set: every element lies in exactly one class.

---

## 9. Partial Orders

A relation is a **partial order** if it is:

1. reflexive
2. antisymmetric
3. transitive

This formalizes an ordering where some pairs may be incomparable.

Examples:

- $$\le$$ on integers
- subset $$\subseteq$$ on sets
- divisibility on positive integers

### Why partial?

Because not every two elements need be comparable.

Example: under divisibility, neither 2 divides 3 nor 3 divides 2.

So 2 and 3 are incomparable.

---

## 10. Total Orders

A partial order is **total** if every two elements are comparable:

$$\forall a,b \in A,\; aRb \text{ or } bRa.$$

Examples:

- $$\le$$ on integers is a total order
- subset on a power set is usually not total

Example:

- {1} and {2} are not comparable under $$\subseteq$$

---

## 11. How Relation Proofs Usually Work

To prove a relation has a property, you expand the definition.

### Reflexive proof shape

Take arbitrary a in A and show $$aRa$$.

### Symmetric proof shape

Take arbitrary a,b and assume $$aRb$$.
Show $$bRa$$.

### Antisymmetric proof shape

Take arbitrary a,b and assume both $$aRb$$ and $$bRa$$.
Show $$a=b$$.

### Transitive proof shape

Take arbitrary a,b,c and assume $$aRb$$ and $$bRc$$.
Show $$aRc$$.

---

## 12. Example Proof — Divisibility is Transitive

Claim: on positive integers, divisibility is transitive.

Assume

$$a \mid b \quad \text{and} \quad b \mid c.$$

Then there exist integers m,n such that

$$b = am, \qquad c = bn.$$

Substitute the first into the second:

$$c = (am)n = a(mn).$$

Since mn is an integer, $$a \mid c$$.
So divisibility is transitive. □

---

## 13. Compiler Connection

Relations are not abstract decoration. They are the language of metatheory.

Examples:

- typing relation: $$\Gamma \vdash e : T$$
- reduction relation: $$e \to e'$$
- subtyping relation: $$S <: T$$
- equivalence relation between programs or plans
- reachability relation in control-flow or query graphs

Later proofs will ask questions like:

- is this relation transitive?
- is this equivalence relation well defined?
- is this subtyping relation a partial order?
- does this rewrite preserve semantic equivalence?

---

## 14. Rust Interpretation

For a finite universe, a relation can be represented as a `HashSet<(T, T)>`.

That is exactly what [school2/phase-00-mathematical-maturity/src/relations.rs](school2/phase-00-mathematical-maturity/src/relations.rs) uses.

It includes checks for:

- reflexive
- symmetric
- antisymmetric
- transitive
- equivalence relation
- partial order

and a helper for equivalence classes.

---

## 15. Memory Version

- a relation is a set of ordered pairs
- reflexive = every element relates to itself
- symmetric = arrows reverse
- antisymmetric = two-way relation forces equality
- transitive = chains collapse
- equivalence relation = reflexive + symmetric + transitive
- partial order = reflexive + antisymmetric + transitive
- total order = partial order + every pair comparable
