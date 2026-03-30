# Exercises 07 — Relations

> **Instructions:** Expand the definitions explicitly. Do not rely on pattern recognition alone.  
> For finite examples, you can check your intuition against `src/relations.rs`.

---

## Tier 1 — Read the relation correctly

Let

$$A = \{1,2,3\}$$

and

$$R = \{(1,1),(2,2),(3,3),(1,2),(2,1)\}.$$

**E1.** Decide whether each statement is true or false.

1. $$1R2$$
2. $$2R1$$
3. $$1R3$$
4. R is reflexive on A
5. R is symmetric on A

**Worked answers:**

1. True, because $$(1,2) \in R$$.
2. True, because $$(2,1) \in R$$.
3. False, because $$(1,3) \notin R$$.
4. True, because $$(1,1),(2,2),(3,3)$$ are all in R.
5. True, because the only non-diagonal arrows are $$(1,2)$$ and $$(2,1)$$, which appear together.

---

## Tier 2 — Classify properties

**E2.** On the integers, classify the relation $$\le$$ as reflexive, symmetric, antisymmetric, and transitive.

**Worked answer:**

- reflexive: yes, because $$a \le a$$ for every integer a
- symmetric: no, because $$2 \le 3$$ but $$3 \le 2$$ is false
- antisymmetric: yes, because if $$a \le b$$ and $$b \le a$$, then $$a=b$$
- transitive: yes, because $$a \le b$$ and $$b \le c$$ imply $$a \le c$$

**E3.** On the integers, classify the relation $$<$$ as reflexive, symmetric, antisymmetric, and transitive.

**Worked answer:**

- reflexive: no, because $$a < a$$ is never true
- symmetric: no, because $$2 < 3$$ but $$3 < 2$$ is false
- antisymmetric: yes, because there are no distinct a,b with both $$a < b$$ and $$b < a$$
- transitive: yes, because $$a < b$$ and $$b < c$$ imply $$a < c$$

**E4.** On positive integers, classify divisibility $$\mid$$ as reflexive, symmetric, antisymmetric, and transitive.

**Worked answer:**

- reflexive: yes, because $$a = a \cdot 1$$ so $$a \mid a$$
- symmetric: no, because $$2 \mid 4$$ but $$4 \mid 2$$ is false
- antisymmetric: yes on positive integers, because if $$a \mid b$$ and $$b \mid a$$ then a and b must be equal
- transitive: yes, because divisibility composes through multiplication

---

## Tier 3 — Write the proofs

**E5.** Prove that the relation $$\subseteq$$ on sets is reflexive.

> **Proof strategy:** definition expansion
>
> **Proof:**
Take an arbitrary set A.
To show $$A \subseteq A$$, we must show that every element of A is an element of A.

So let x be arbitrary and assume $$x \in A$$.
Then of course $$x \in A$$.

Therefore $$A \subseteq A$$.
So the subset relation is reflexive. □

**E6.** Prove that the relation $$\subseteq$$ on sets is antisymmetric.

> **Proof strategy:** definition expansion + set equality by double inclusion
>
> **Proof:**
Assume

$$A \subseteq B \quad \text{and} \quad B \subseteq A.$$

To show $$A=B$$, it is enough to show the sets have the same elements.
But the two assumptions already say exactly that:

- every element of A is in B
- every element of B is in A

So by double inclusion,

$$A = B.$$

Therefore $$\subseteq$$ is antisymmetric. □

**E7.** Prove that divisibility on positive integers is transitive.

> **Proof strategy:** definition expansion
>
> **Proof:**
Assume

$$a \mid b \quad \text{and} \quad b \mid c.$$

Then there exist integers m,n such that

$$b = am, \qquad c = bn.$$

Substitute:

$$c = (am)n = a(mn).$$

Since mn is an integer, $$a \mid c$$.

Therefore divisibility is transitive. □

---

## Tier 4 — Equivalence relations and orders

**E8.** Explain why congruence modulo 3 is an equivalence relation.

**Worked answer:**

It is reflexive because $$3 \mid (a-a)$$, symmetric because $$3 \mid (a-b)$$ implies $$3 \mid (b-a)$$, and transitive because $$3 \mid (a-b)$$ and $$3 \mid (b-c)$$ imply $$3 \mid (a-c)$$.

**E9.** What are the equivalence classes of the integers modulo 2?

**Worked answer:**

There are exactly two classes:

- the even integers
- the odd integers

because every integer has remainder 0 or 1 when divided by 2.

**E10.** Why is divisibility on positive integers a partial order but not a total order?

**Worked answer:**

It is a partial order because it is reflexive, antisymmetric, and transitive. It is not total because some pairs are incomparable, for example 2 and 3: neither $$2 \mid 3$$ nor $$3 \mid 2$$.

Rust prompts:

```rust
// Build a finite universe and relation as a HashSet<(T, T)>.
// Then check:
// is_reflexive(&universe, &relation)
// is_symmetric(&relation)
// is_antisymmetric(&relation)
// is_transitive(&relation)
// is_equivalence_relation(&universe, &relation)
// is_partial_order(&universe, &relation)
```