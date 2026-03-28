# Lesson 5B — Completing the Proof Toolkit

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, proof-methods continuation  
**Goal:** Finish the proof block by covering smallest counterexample / well-ordering, proof by construction, and constructive vs non-constructive arguments.

---

## 1. Why This Lesson Exists

We already covered:

- direct proof
- contrapositive
- contradiction
- weak induction
- strong induction

That is most of the proof machinery you use day to day.

But the proof section in the curriculum also included:

- proof by smallest counterexample
- proof by construction
- constructive vs non-constructive proofs

This lesson closes that gap.

---

## 2. Proof by Smallest Counterexample

This is really a well-ordering plus contradiction proof.

The basic shape is:

1. Suppose the claim is false for at least one natural number.
2. Let S be the set of counterexamples.
3. Since S is non-empty, the well-ordering principle says S has a least element m.
4. Show that m cannot actually be a counterexample.
5. Contradiction.

So the statement must have been true for all n.

### Why this works

The natural numbers do not allow an infinite descent:

$$m > m_1 > m_2 > m_3 > \cdots$$

with all terms in $$\mathbb{N}$$.

So if there were any failures, there would be a first failure.
Then you attack that first failure.

### Template

```text
Claim: for all n in N, P(n).

Proof by smallest counterexample.

Assume, for contradiction, that the claim is false for some n.
Let S = {n in N : P(n) is false}.
Then S is non-empty.
By well-ordering, let m = min(S).

Now m is the smallest counterexample.
Use that fact to prove P(m) must actually hold.
Contradiction.

Therefore there is no counterexample, so for all n, P(n).
```

---

## 3. Smallest Counterexample vs Induction

These are extremely close.

Induction says:

- there is a base case
- once everything up to k works, the next case works

Smallest counterexample says:

- assume something fails somewhere
- look at the first place it fails
- use earlier cases to show it cannot fail there either

They are different presentations of the same well-founded idea.

### Mental model

If induction is a forward domino argument,
smallest counterexample is a first broken domino argument.

---

## 4. Example — Every Integer n ≥ 2 Has a Prime Divisor

We already proved this by strong induction. Here is the smallest-counterexample version.

**Claim:** Every integer $$n \ge 2$$ has a prime divisor.

**Proof by smallest counterexample.**

Assume, for contradiction, that the claim is false for some integer $$n \ge 2$$.
Let

$$S = \{n \in \mathbb{N} : n \ge 2 \text{ and } n \text{ has no prime divisor}\}.$$

Then S is non-empty.
By the well-ordering principle, S has a least element. Call it $$m$$.

So:

- $$m \ge 2$$
- m has no prime divisor
- every integer $$2 \le j < m$$ does have a prime divisor

Now there are two cases.

Case 1: $$m$$ is prime.

Then m divides itself, so m has a prime divisor, namely m.
Contradiction.

Case 2: $$m$$ is composite.

Then

$$m = ab$$

for integers a,b with

$$2 \le a < m \quad \text{and} \quad 2 \le b < m.$$

Since $$a < m$$ and $$a \ge 2$$, the minimality of m says that a has a prime divisor p.
Then

$$p \mid a \quad \text{and} \quad a \mid m,$$

so

$$p \mid m.$$

Thus m has a prime divisor, contradiction again.

So S must be empty.
Therefore every integer $$n \ge 2$$ has a prime divisor. □

---

## 5. Proof by Construction

A proof by construction proves existence by actually giving an example.

If the claim is:

$$\exists x\; P(x),$$

then a constructive proof usually looks like:

1. define or present a specific x
2. check that P(x) holds

### Example

**Claim:** There exists an irrational number x such that $$x^2$$ is rational.

**Constructive proof:**

Take

$$x = \sqrt{2}.$$

We know $$\sqrt{2}$$ is irrational.
But

$$x^2 = (\sqrt{2})^2 = 2,$$

and 2 is rational.

So we explicitly exhibited such an x. □

### Compiler analogy

Construction is like proving feasibility by building an explicit witness:

- a concrete typing derivation
- a concrete lowering path
- a specific rewrite sequence
- a specific counterexample program

You are not just saying something exists. You are handing over the object.

---

## 6. Non-Constructive Existence Proofs

Sometimes you prove existence without identifying the actual object.

### Classic example

**Claim:** There exist irrational numbers a and b such that $$a^b$$ is rational.

One standard proof goes like this:

Take

$$x = \sqrt{2}^{\sqrt{2}}.$$

Now either x is rational or x is irrational.

Case 1: x is rational.

Then let

$$a = \sqrt{2}, \qquad b = \sqrt{2}.$$

Then $$a,b$$ are irrational and $$a^b = x$$ is rational.

Case 2: x is irrational.

Then let

$$a = x, \qquad b = \sqrt{2}.$$

Now

$$a^b = \left(\sqrt{2}^{\sqrt{2}}\right)^{\sqrt{2}} = \sqrt{2}^{2} = 2,$$

which is rational.

So in either case, such irrational numbers exist. □

### Why this is non-constructive

The proof does not tell you which case is actually true.
It proves existence by a case split, but does not identify a definite witness from within the proof itself.

---

## 7. Constructive vs Non-Constructive

### Constructive

- gives an explicit witness, algorithm, or object
- often stronger for computation
- useful in compilers and type systems because you often want the witness itself

### Non-constructive

- proves something must exist
- may rely on contradiction or excluded middle
- may not give an algorithm or witness

### In compiler work

Constructive reasoning is usually more operationally valuable.

Examples:

- proving a term is typable by building a derivation tree
- proving a query plan exists by constructing one
- proving two forms are equivalent by giving a transformation sequence

Non-constructive reasoning still matters, but it is less immediately executable.

---

## 8. When to Use Which Proof Style

| Situation | Natural proof style |
|---|---|
| Show every object has a property by unfolding definitions | Direct proof |
| Implication easier backward than forward | Contrapositive |
| Assume the opposite and force impossibility | Contradiction |
| Indexed claim over naturals | Induction |
| First place failure happens reasoning | Smallest counterexample / WOP |
| Existence by exhibiting an example | Construction |
| Existence without explicit witness | Non-constructive existence |

---

## 9. Connection to Type Systems and Query Optimizers

These proof styles map directly onto compiler reasoning.

### Induction

- on syntax trees
- on derivation height
- on list length
- on evaluation steps

### Smallest counterexample

- assume there is a smallest ill-typed term breaking preservation
- assume there is a shortest optimizer bug witness
- assume there is a minimal counterexample query plan

### Construction

- build a typing derivation
- build a normal form
- build a plan transformation
- build a witness substitution

### Non-constructive reasoning

- prove existence of an optimal plan without showing how to compute it
- prove existence of a model without constructing it

For engineering, constructive proofs are often the ones that translate into code.

---

## 10. Memory Version

- smallest counterexample = contradiction + pick the least failure
- it works because naturals are well ordered
- construction proves existence by giving a witness
- non-constructive proofs prove existence without giving the witness
- in compiler work, constructive proofs often correspond to actual algorithms
