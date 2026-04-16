# Exercises 09 — Cardinality

> **Instructions:** Every exercise asks you to reason from definitions — bijections, countability,  
> and the diagonal argument. Resist the urge to wave your hands at "infinity."  
> Write out the actual bijection or the actual contradiction.

---

## Tier 1 — Warm-up: bijection proofs for finite sets

**E1.** Show that $$|\{a, b, c, d\}| = |\{1, 2, 3, 4\}|$$ by exhibiting a bijection.

**Worked answer:**

Define $$f : \{a,b,c,d\} \to \{1,2,3,4\}$$ by $$f(a)=1, f(b)=2, f(c)=3, f(d)=4$$.  
This is a bijection (every element of the codomain is hit exactly once). ∎

---

## Tier 2 — Countability proofs

**E2.** Show that the set of odd positive integers $$O = \{1, 3, 5, 7, \ldots\}$$ is countably infinite.

**Worked answer:**

Define $$f : \mathbb{N} \to O$$ by $$f(n) = 2n + 1$$ (indexing from 0).

- *Injective:* $$2n_1 + 1 = 2n_2 + 1 \Rightarrow n_1 = n_2$$. ✓
- *Surjective:* every odd positive integer has the form $$2k+1$$ for $$k \ge 0$$; take $$n = k$$. ✓

So $$|O| = |\mathbb{N}|$$. ∎

---

**E3.** Show that $$\mathbb{Z}$$ is countably infinite by exhibiting an explicit bijection with $$\mathbb{N}$$.

**Worked answer:**

Define $$f : \mathbb{N} \to \mathbb{Z}$$ by:

$$f(n) = \begin{cases} n/2 & \text{if } n \text{ is even} \\ -(n+1)/2 & \text{if } n \text{ is odd} \end{cases}$$

This gives the sequence $$0, -1, 1, -2, 2, -3, 3, \ldots$$

- *Injective:* even and odd cases map to non-negative and negative integers respectively, which are disjoint; within each case the map is injective. ✓
- *Surjective:* any $$m \ge 0$$ is hit by $$n = 2m$$; any $$m < 0$$ is hit by $$n = -2m - 1$$. ✓ ∎

---

**E4.** *(Challenge)* Explain informally, using a diagram, why $$\mathbb{Q}$$ (the rationals) is countable.

**Worked answer:**

Arrange all fractions $$p/q$$ (with $$q > 0$$, not necessarily in lowest terms) in a doubly-infinite grid:

```
  q=1   q=2   q=3   q=4  ...
p=0:  0/1   0/2   0/3   0/4
p=1:  1/1   1/2   1/3   1/4
p=2:  2/1   2/2   2/3   2/4
p=3:  3/1   3/2   3/3   3/4
 ...
```

Traverse the grid diagonally:

$$0/1 \to 1/1 \to 0/2 \to 0/3 \to 1/2 \to 2/1 \to 3/1 \to 2/2 \to \ldots$$

Skip entries already seen in reduced form. This visits every positive rational, giving a surjection  
(and hence a bijection after tidying up) from ℕ to ℚ⁺. Adding the negatives by interleaving  
then shows all of ℚ is countable. ∎

---

## Tier 3 — The diagonal argument

**E5.** Carry out the diagonal argument on the following supposed enumeration of reals in (0,1).  
Exhibit a specific $$x \in (0,1)$$ not in the list.

$$r_0 = 0.1234567890\ldots$$  
$$r_1 = 0.9876543210\ldots$$  
$$r_2 = 0.1111111111\ldots$$  
$$r_3 = 0.3141592653\ldots$$

Use the rule: $$x_i = 5$$ if $$d_{ii} \ne 5$$, else $$x_i = 6$$.

**Worked answer:**

Read off the diagonal: $$d_{00} = 1,\; d_{11} = 8,\; d_{22} = 1,\; d_{33} = 1$$.

Apply the rule:
- $$x_0 = 5$$ (since $$1 \ne 5$$)
- $$x_1 = 5$$ (since $$8 \ne 5$$)
- $$x_2 = 5$$ (since $$1 \ne 5$$)
- $$x_3 = 5$$ (since $$1 \ne 5$$)

So $$x = 0.5555\ldots$$

Check:
- $$x \ne r_0$$ because $$x_0 = 5 \ne 1 = d_{00}$$.
- $$x \ne r_1$$ because $$x_1 = 5 \ne 8 = d_{11}$$.
- $$x \ne r_2$$ because $$x_2 = 5 \ne 1 = d_{22}$$.
- $$x \ne r_3$$ because $$x_3 = 5 \ne 1 = d_{33}$$.

(The argument extends to all further rows in the infinite list.) ∎

---

**E6.** Why do we avoid using digits 0 and 9 when constructing the diagonal number?

**Worked answer:**

In decimal, some real numbers have two representations:

$$0.4999\ldots = 0.5000\ldots$$

If the diagonal number happened to be constructed using a digit 0 or 9, we might accidentally  
produce a number that coincides with one on the list under its alternative decimal form.

By choosing, say, only digits 5 and 6, we ensure the constructed $$x$$ has a unique decimal  
representation and the argument is airtight.

---

**E7.** Fill in the gap: the diagonal argument shows that no *list* can contain all of $$(0,1)$$.  
Why does this imply there is no *bijection* $$f : \mathbb{N} \to (0,1)$$?

**Worked answer:**

A bijection $$f : \mathbb{N} \to (0,1)$$ would produce exactly such a list by setting $$r_n = f(n)$$.  
Since the diagonal argument shows every such list is incomplete (misses at least one element),  
no such bijection can exist.

Therefore $$|(0,1)|  \ne |\mathbb{N}|$$, and since $$\mathbb{N}$$ injects into $$(0,1)$$, we have $$|(0,1)| > |\mathbb{N}|$$. ∎

---

## Tier 4 — Cantor's theorem

**E8.** State Cantor's theorem and prove it for a finite set $$A = \{1, 2\}$$.

**Worked answer:**

**Cantor's theorem:** For any set A, $$|A| < |\mathcal{P}(A)|$$.

*Finite check:* $$|\{1,2\}| = 2$$ and $$|\mathcal{P}(\{1,2\})| = 4$$. ✓

The general proof follows the same diagonal spirit:

*Proof.* There is an injection from A into $$\mathcal{P}(A)$$ (map $$a \mapsto \{a\}$$), so $$|A| \le |\mathcal{P}(A)|$$.

It remains to show there is no surjection $$f : A \to \mathcal{P}(A)$$.

Suppose $$f : A \to \mathcal{P}(A)$$ is any function. Define:

$$D = \{a \in A : a \notin f(a)\}.$$

Then $$D \subseteq A$$, so $$D \in \mathcal{P}(A)$$.

Suppose for contradiction that there exists $$a^* \in A$$ with $$f(a^*) = D$$.

- If $$a^* \in D$$: by definition of D, $$a^* \notin f(a^*) = D$$. Contradiction.
- If $$a^* \notin D$$: by definition of D, $$a^* \in f(a^*) = D$$. Contradiction.

Both cases are impossible. So D has no preimage under f, meaning f is not surjective.  
Since f was arbitrary, no surjection $$A \to \mathcal{P}(A)$$ exists. ∎

---

## Tier 5 — Reflection and connection

**E9.** A student claims: "The even integers are a proper subset of the integers, so there are  
fewer even integers than integers." Identify the flaw and give the correct answer.

**Worked answer:**

The flaw is applying finite-set logic to infinite sets.  
For finite sets, proper subsets do have fewer elements.  
For infinite sets, this fails: an infinite set can be in bijection with a proper subset.

The correct answer is: $$|\text{even integers}| = |\mathbb{Z}| = |\mathbb{N}|$$ — all three are countably infinite.  
The bijection $$n \mapsto 2n$$ from ℕ to the even natural numbers demonstrates this.

---

**E10.** Arrange the following sets in order of cardinality (smallest to largest), and justify.

$$\{1,2,3\},\quad \mathbb{N},\quad \mathbb{Z},\quad \mathbb{Q},\quad \mathbb{R},\quad \mathcal{P}(\mathbb{R})$$

**Worked answer:**

$$|\{1,2,3\}| < |\mathbb{N}| = |\mathbb{Z}| = |\mathbb{Q}| < |\mathbb{R}| < |\mathcal{P}(\mathbb{R})|$$

- $$|\{1,2,3\}| = 3$$ (finite).
- $$|\mathbb{N}| = |\mathbb{Z}| = |\mathbb{Q}| = \aleph_0$$ (all countably infinite).
- $$|\mathbb{R}| = \mathfrak{c} > \aleph_0$$ (diagonal argument).
- $$|\mathcal{P}(\mathbb{R})| > |\mathbb{R}|$$ (Cantor's theorem).

---

## Self-Test Checklist

- [ ] Can you exhibit a bijection between ℕ and ℤ?
- [ ] Can you explain, in one paragraph, why ℝ is uncountable?
- [ ] Can you carry out the diagonal construction given any proposed list?
- [ ] Can you state and prove Cantor's theorem?
- [ ] Can you identify when someone is wrongly applying finite intuition to infinite sets?
