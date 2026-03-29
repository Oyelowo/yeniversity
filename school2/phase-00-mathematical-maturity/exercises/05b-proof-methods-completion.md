# Exercises 05B — Completing the Proof Toolkit

> **Instructions:** These finish the proof-method block before moving on to later topics.  
> Write the proof strategy first, then justify each step explicitly.

---

## Tier 1 — Choose the right proof style

**E1.** For each claim, name the most natural proof strategy and give one sentence of justification.

1. Every integer $$n \ge 2$$ has a prime divisor.
2. There exists a rational number strictly between 1 and 2.
3. There exist irrational numbers a,b such that $$a^b$$ is rational.

**Worked answers:**

1. Smallest counterexample or strong induction, because if a number is composite then it breaks into smaller factors, so the proof naturally reduces the problem to smaller integers.
2. Construction, because we can explicitly exhibit a witness such as $$\frac{3}{2}$$ and then directly verify it satisfies the claim.
3. Non-constructive existence proof by cases, because the standard argument proves that one of two candidate choices must work without first identifying which one is the actual witness.

---

## Tier 2 — Smallest counterexample

**E2.** Prove by smallest counterexample that every integer $$n \ge 2$$ has a prime divisor.

> **Proof strategy:** smallest counterexample / well-ordering
>
> **Proof:**
Assume, for contradiction, that the claim is false for some integer $$n \ge 2$$.
Let

$$S = \{n \in \mathbb{N} : n \ge 2 \text{ and } n \text{ has no prime divisor}\}.$$

Then S is non-empty.
By the well-ordering principle, let $$m = \min(S)$$.

Then $$m \ge 2$$ and m has no prime divisor.
Also, every integer $$j$$ with $$2 \le j < m$$ is not in S, so every such j does have a prime divisor.

If m is prime, then m divides itself, so m has a prime divisor. Contradiction.

If m is composite, then

$$m = ab$$

for integers a,b with

$$2 \le a < m \quad \text{and} \quad 2 \le b < m.$$

Since $$2 \le a < m$$, the minimality of m implies that a has a prime divisor p.
Then $$p \mid a$$ and $$a \mid m$$, so $$p \mid m$$.

Thus m has a prime divisor, contradiction.

Therefore S is empty, so every integer $$n \ge 2$$ has a prime divisor. □

**E3.** Explain in one sentence why this proof is morally the same as strong induction.

**Worked answer:**

It is the same idea because once m is chosen as the smallest counterexample, every smaller case is automatically known to be correct, and that is exactly the information strong induction assumes.

---

## Tier 3 — Construction and non-construction

**E4.** Prove by construction that there exists an irrational number x such that $$x^2$$ is rational.

> **Proof strategy:** construction
>
> **Proof:**
Take

$$x = \sqrt{2}.$$

The number $$\sqrt{2}$$ is irrational.
But

$$x^2 = (\sqrt{2})^2 = 2,$$

and 2 is rational.

Therefore there exists an irrational number x such that $$x^2$$ is rational. □

**E5.** Prove by construction that there exists a rational number strictly between 1 and 2.

> **Proof strategy:** construction
>
> **Proof:**
Take

$$x = \frac{3}{2}.$$

Then x is rational, and

$$1 < \frac{3}{2} < 2.$$

So there exists a rational number strictly between 1 and 2. □

**E6.** Explain why the standard proof that there exist irrational numbers a,b such that $$a^b$$ is rational is non-constructive.

**Worked answer:**

It is non-constructive because the proof argues that one of two candidate pairs must work, but it does not initially tell us which pair is the genuine witness; it proves existence without directly constructing the final chosen example.

---

## Tier 4 — Compiler-oriented thinking

**E7.** Suppose you claim: for every typable term e, there exists a typing derivation for e. Is that statement best proved constructively or non-constructively? Explain in one or two sentences.

**Worked answer:**

Constructively, because a typing derivation is itself the witness, and in compiler work you usually want the derivation tree or the algorithm that builds it, not just a statement that some derivation exists in principle.

**E8.** A query optimizer proof says: there exists an optimal plan equivalent to the input plan. What extra value do you get if the proof is constructive?

**Worked answer:**

You get an actual way to compute or exhibit the optimal plan, so the proof becomes much closer to an implementation: it tells you how to obtain the witness plan instead of merely asserting that one exists somewhere.
