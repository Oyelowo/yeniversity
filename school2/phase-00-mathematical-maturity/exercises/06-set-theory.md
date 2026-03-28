# Exercises 06 — Set Theory

> **Instructions:** Do these by element-chasing first. When the sets are finite, check your intuition against `src/sets.rs`.  
> Mark an exercise ✅ only when you can reproduce the argument without notes.

---

## Tier 1 — Read the notation correctly

**E1.** Translate each statement into plain English.

1. $$3 \in A$$
2. $$5 \notin B$$
3. $$A \subseteq B$$
4. $$x \in A \cap B$$
5. $$y \in A \cup B$$
6. $$z \in A \setminus B$$

**Worked answers:**

1. 3 is an element of A.
2. 5 is not an element of B.
3. Every element of A is also an element of B.
4. x is in both A and B.
5. y is in A or in B or in both.
6. z is in A but not in B.

**E2.** Let

$$A = \{1,2,3\}, \qquad B = \{3,4,5\}.$$

Compute:

1. $$A \cup B$$
2. $$A \cap B$$
3. $$A \setminus B$$
4. $$B \setminus A$$
5. $$A \triangle B$$

**Worked answers:**

1. $$A \cup B = \{1,2,3,4,5\}$$
2. $$A \cap B = \{3\}$$
3. $$A \setminus B = \{1,2\}$$
4. $$B \setminus A = \{4,5\}$$
5. $$A \triangle B = \{1,2,4,5\}$$

**E3.** List every subset of $$\{a,b\}$$.

**Worked answer:**

$$\mathcal{P}(\{a,b\}) = \{\varnothing, \{a\}, \{b\}, \{a,b\}\}.$$

So there are 4 subsets.

---

## Tier 2 — Prove simple subset facts

**E4.** Prove that for any sets A and B,

$$A \cap B \subseteq A.$$

> **Proof strategy:** subset proof
>
> **Proof:**
Take arbitrary x and assume

$$x \in A \cap B.$$

By definition of intersection, this means

- $$x \in A$$
- $$x \in B$$

In particular, $$x \in A$$.

Therefore every element of $$A \cap B$$ is an element of A, so

$$A \cap B \subseteq A. \quad \square$$

**E5.** Prove that for any sets A and B,

$$A \setminus B \subseteq A.$$

> **Proof strategy:** subset proof
>
> **Proof:**
Take arbitrary x and assume

$$x \in A \setminus B.$$

By definition of difference, this means

- $$x \in A$$
- $$x \notin B$$

In particular, $$x \in A$$.

Hence

$$A \setminus B \subseteq A. \quad \square$$

**E6.** Prove that $$A \subseteq A \cup B$$ for any sets A and B.

> **Proof strategy:** subset proof
>
> **Proof:**
Take arbitrary x and assume

$$x \in A.$$

Then x is in A or B, because it is already in A.
So

$$x \in A \cup B.$$

Therefore

$$A \subseteq A \cup B. \quad \square$$

---

## Tier 3 — Set equality by double inclusion

**E7.** Prove that for any sets A and B,

$$A \cap B = B \cap A.$$

> **Proof strategy:** double inclusion
>
> **Proof:**
First show $$A \cap B \subseteq B \cap A$$.

Take arbitrary x and assume

$$x \in A \cap B.$$

Then x is in A and in B.
So x is in B and in A.
Hence

$$x \in B \cap A.$$

Therefore

$$A \cap B \subseteq B \cap A.$$

Now show $$B \cap A \subseteq A \cap B$$.

Take arbitrary x and assume

$$x \in B \cap A.$$

Then x is in B and in A.
So x is in A and in B.
Hence

$$x \in A \cap B.$$

Therefore

$$B \cap A \subseteq A \cap B.$$

Since both inclusions hold,

$$A \cap B = B \cap A. \quad \square$$

**E8.** Prove that for any sets A, B, C,

$$A \cap (B \cup C) = (A \cap B) \cup (A \cap C).$$

> **Proof strategy:** double inclusion
>
> **Proof:**
First show

$$A \cap (B \cup C) \subseteq (A \cap B) \cup (A \cap C).$$

Take arbitrary x and assume

$$x \in A \cap (B \cup C).$$

Then:

- $$x \in A$$
- $$x \in B \cup C$$

So either $$x \in B$$ or $$x \in C$$.

- If $$x \in B$$, then $$x \in A \cap B$$.
- If $$x \in C$$, then $$x \in A \cap C$$.

So in either case,

$$x \in (A \cap B) \cup (A \cap C).$$

Hence

$$A \cap (B \cup C) \subseteq (A \cap B) \cup (A \cap C).$$

Now show the reverse inclusion.

Take arbitrary x and assume

$$x \in (A \cap B) \cup (A \cap C).$$

Then either:

- $$x \in A \cap B$$, or
- $$x \in A \cap C$$.

In the first case, x is in A and in B, so x is in A and in $$B \cup C$$.
In the second case, x is in A and in C, so x is in A and in $$B \cup C$$.

Thus in either case,

$$x \in A \cap (B \cup C).$$

Therefore

$$ (A \cap B) \cup (A \cap C) \subseteq A \cap (B \cup C). $$

So the sets are equal. □

---

## Tier 4 — Think carefully

**E9.** Is it always true that

$$A \in B \Rightarrow A \subseteq B?$$

If yes, prove it. If not, give a counterexample.

**Answer:** No.

Counterexample:

Let

$$A = \{1\}, \qquad B = \{\{1\}, 2\}.$$

Then

$$A \in B$$

because the set $$\{1\}$$ is an element of B.

But

$$A \subseteq B$$

would mean every element of A is an element of B.
The only element of A is 1, and

$$1 \notin B,$$

because the elements of B are $$\{1\}$$ and 2, not 1 itself.

So $$A \in B$$ does not imply $$A \subseteq B$$.

**E10.** Explain why $$\varnothing \subseteq A$$ for every set A.

**Answer:**

To say $$\varnothing \subseteq A$$ means:

> for every x, if $$x \in \varnothing$$, then $$x \in A$$

But there is no x with $$x \in \varnothing$$.
So there is no counterexample.
Therefore the statement is true vacuously.

**E11.** Let $$A = \{1,2\}$$. Compute $$\mathcal{P}(A)$$ and its size.

**Answer:**

$$\mathcal{P}(A) = \{\varnothing, \{1\}, \{2\}, \{1,2\}\}$$

and so

$$|\mathcal{P}(A)| = 4 = 2^2.$$

Rust prompts:

```rust
// Build two finite HashSet values and compare the outputs of:
// union(&a, &b)
// intersection(&a, &b)
// difference(&a, &b)
// power_set(&a)
```