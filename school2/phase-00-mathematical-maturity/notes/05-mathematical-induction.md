# Lesson 5 — Mathematical Induction

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 3 §3.1–3.3  
**Goal:** Prove universal claims over an ordered domain (usually ℕ) using the induction principle.

---

## 1. Motivation

Consider the claim:

$$1 + 2 + 3 + \cdots + n = \frac{n(n+1)}{2}$$

You can verify it for n = 1, 2, 3, 100. But you can never check *all* infinitely many values by brute force.

Induction gives you a *two-step machine* that simultaneously proves all of them.

---

## 2. The Induction Principle (Weak / Simple Form)

Let P(n) be a predicate over ℕ (natural numbers, starting at 0 or 1 — be explicit).

**Induction principle:**

$$\underbrace{P(0)}_{\text{base case}} \;\land\; \underbrace{\bigl(\forall k \in \mathbb{N},\; P(k) \Rightarrow P(k+1)\bigr)}_{\text{inductive step}} \;\Longrightarrow\; \forall n \in \mathbb{N},\; P(n)$$

### Informal picture

```
P(0) ──[step]──▶ P(1) ──[step]──▶ P(2) ──[step]──▶ P(3) ──[step]──▶ …
```

- **Base case:** The first domino falls.  
- **Inductive step:** Every fallen domino knocks over the next one.  
- **Conclusion:** All dominoes fall.

### Proof template

```
Claim: ∀n ∈ ℕ, P(n).

Proof by (weak) induction on n.

Base case (n = 0):
  Show P(0).
  [computation / argument]

Inductive step:
  Let k ∈ ℕ be arbitrary.
  Inductive hypothesis (IH): Assume P(k).
  Goal: Show P(k + 1).
  [use IH + algebra / logic]

By the principle of mathematical induction, ∀n ∈ ℕ, P(n). □
```

---

## 3. Example 3.1 — Closed-Form Sum ★

**Claim:** For all n ≥ 1,

$$\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$$

**Proof by induction on n.**

**Base case (n = 1):**  
LHS = 1. RHS = 1·2/2 = 1. Equal. ✓

**Inductive step:**  
Assume $\sum_{i=1}^{k} i = \frac{k(k+1)}{2}$ for some k ≥ 1. (IH)

Show $\sum_{i=1}^{k+1} i = \frac{(k+1)(k+2)}{2}$.

$$\sum_{i=1}^{k+1} i = \underbrace{\sum_{i=1}^{k} i}_{\text{IH}} + (k+1) = \frac{k(k+1)}{2} + (k+1) = (k+1)\!\left(\frac{k}{2} + 1\right) = \frac{(k+1)(k+2)}{2}$$

✓ — matches the formula for n = k + 1. □

---

## 4. Example 4.1 — Sum of First n Odd Numbers

**Claim:** For all n ≥ 1,

$$\sum_{i=1}^{n} (2i - 1) = n^2$$

(i.e., 1 + 3 + 5 + … + (2n−1) = n²)

**Base case (n = 1):** LHS = 1 = 1² = RHS. ✓

**Inductive step:** Assume $\sum_{i=1}^{k}(2i-1) = k^2$ (IH).

$$\sum_{i=1}^{k+1}(2i-1) = k^2 + (2(k+1)-1) = k^2 + 2k + 1 = (k+1)^2 \checkmark$$

□

---

## 5. Example 5.1 — Geometric Series

**Claim:** For all n ≥ 0 and r ≠ 1,

$$\sum_{i=0}^{n} r^i = \frac{r^{n+1} - 1}{r - 1}$$

**Base case (n = 0):** LHS = r⁰ = 1. RHS = (r¹ − 1)/(r − 1) = (r−1)/(r−1) = 1. ✓

**Inductive step:** Assume the formula holds for k.

$$\sum_{i=0}^{k+1} r^i = \frac{r^{k+1}-1}{r-1} + r^{k+1} = \frac{r^{k+1}-1 + r^{k+1}(r-1)}{r-1} = \frac{r^{k+2}-1}{r-1} \checkmark$$

□

---

## 6. Example 6.1 — Divisibility ★

**Claim:** For all n ≥ 0, 4 | (5ⁿ − 1).

**Base case (n = 0):** 5⁰ − 1 = 0. 4 | 0. ✓

**Inductive step:** Assume 4 | (5ᵏ − 1), i.e., 5ᵏ − 1 = 4m for some m ∈ ℤ.

$$5^{k+1} - 1 = 5 \cdot 5^k - 1 = 5(5^k - 1) + 5 - 1 = 5(4m) + 4 = 4(5m + 1)$$

So 4 | (5^(k+1) − 1). □

**Key technique:** Rewrite the (k+1)-th expression in terms of the k-th expression so the IH fires.

### NB — How did we find

$$5^{k+1} - 1 = 5(5^k - 1) + 4?$$

The inductive hypothesis talks about:

$$5^k - 1$$

So in the inductive step, we try to make that exact chunk appear.

Start with:

$$5^{k+1} - 1 = 5 \\cdot 5^k - 1$$

Now deliberately add and subtract 5:

$$5 \\cdot 5^k - 1 = 5 \\cdot 5^k - 5 + 4$$

Then factor 5 from the first two terms:

$$= 5(5^k - 1) + 4$$

This is not magic or guessing. It is reverse-engineering the expression so that the IH appears.

Memory rule:

- look at what the IH gives you
- rewrite the (k+1)-case until that exact expression appears inside it

---

## 7. Strong Induction

**Weak induction** uses only P(k) to prove P(k+1).  
**Strong induction** uses *all* of P(0), P(1), …, P(k) to prove P(k+1).

### Principle (strong form)

$$\underbrace{P(0)}_{\text{base}} \;\land\; \underbrace{\bigl(\forall k,\; [\forall j \le k,\; P(j)] \Rightarrow P(k+1)\bigr)}_{\text{strong step}} \;\Longrightarrow\; \forall n,\; P(n)$$

### When to use strong induction

- When the (k+1)-case depends on *earlier* values, not just the immediately preceding one.
- Classic sign: recurrences like Fibonacci, or arguments that split into sub-cases.

### Template

```
Claim: ∀n ∈ ℕ, P(n).

Proof by strong induction on n.

Base case(s): Show P(0) [and P(1) if your step needs k ≥ 1].

Inductive step:
  Let k ≥ 1 (or k ≥ base) be arbitrary.
  Strong IH: Assume P(j) holds for all j with 0 ≤ j ≤ k.
  Goal: Show P(k + 1).
  [use strong IH at some j ≤ k]

By strong induction, ∀n ∈ ℕ, P(n). □
```

---

## 8. Example 8.1 — Every Integer ≥ 2 Has a Prime Divisor

**Claim:** Every integer n ≥ 2 has at least one prime divisor.

**Proof by strong induction on n.**

**Base case (n = 2):** 2 is prime, and 2 | 2. ✓

**Inductive step:** Let k ≥ 2. Strong IH: every integer 2 ≤ j ≤ k has a prime divisor.  
Consider k + 1.  
- If k + 1 is prime: it divides itself. ✓  
- If k + 1 is composite: k + 1 = a · b with 2 ≤ a ≤ k.  
  By strong IH, a has a prime divisor p. Since p | a and a | (k+1), p | (k+1). ✓

By strong induction, every integer ≥ 2 has a prime divisor. □

### NB — What exactly happened in the composite case?

This is the key chain of reasoning:

1. If $$k+1$$ is composite, then

$$k+1 = ab$$

for some integers $$a,b$$ with

$$2 \\le a \\le k \\quad \\text{and} \\quad 2 \\le b \\le k$$

2. Since $$a$$ is between 2 and $$k$$, the strong IH applies to $$a$$.

So $$a$$ has a prime divisor. Call it $$p$$. Then:

$$p \\mid a$$

3. Also, because $$k+1 = ab$$, the factor $$a$$ divides $$k+1$$:

$$a \\mid (k+1)$$

4. Now use the transitivity of divisibility:

if $$p \\mid a$$ and $$a \\mid n$$, then $$p \\mid n$$

So from $$p \\mid a$$ and $$a \\mid (k+1)$$, we get:

$$p \\mid (k+1)$$

That is enough. We do **not** need to prove that both $$a$$ and $$b$$ are prime, or that $$p$$ divides both of them. One prime divisor of one factor is already a prime divisor of $$k+1$$.

There are really only two possibilities:

- if $$k+1$$ is prime, it divides itself
- if $$k+1$$ is composite, one of its factors has a prime divisor, and that prime divisor also divides $$k+1$$

---

## 9. Example 9.1 — Fibonacci and Strong Induction

**Fibonacci sequence:** F(0) = 0, F(1) = 1, F(n) = F(n−1) + F(n−2) for n ≥ 2.

**Claim:** For all n ≥ 0, F(n) < 2ⁿ.

**Base cases:**  
- F(0) = 0 < 2⁰ = 1. ✓  
- F(1) = 1 < 2¹ = 2. ✓

**Inductive step (k + 1, need k ≥ 1):**  
Strong IH: F(j) < 2ʲ for all j ≤ k.  
Since k + 1 ≥ 2:

$$F(k+1) = F(k) + F(k-1) < 2^k + 2^{k-1} < 2^k + 2^k = 2^{k+1}$$

✓ □

---

## 10. The Well-Ordering Principle

**Well-Ordering Principle (WOP):** Every non-empty subset of ℕ has a least element.

This is *equivalent* to the principle of mathematical induction (they imply each other over ℕ). Some proofs are more natural with WOP.

**Proof pattern with WOP:**

```
Suppose for contradiction P(n) is false for some n.
Let S = {n ∈ ℕ | ¬P(n)}.  S is non-empty by assumption.
By WOP, let m = min(S).  So P(m) is false.
Show that P(m) must actually be true → contradiction.
```

### Example — same as induction

You rarely need WOP explicitly. But knowing it underpins induction explains why induction works: the inductive step closes off any "smallest counterexample."

---

## 11. Mistakes to Avoid

| Mistake | What goes wrong |
|---|---|
| Forgetting to state the IH explicitly | Reviewers — and you — can't tell what you're allowed to use |
| Using P(k+1) in the proof of P(k+1) | Circular. The IH is P(k), not P(k+1) |
| Skipping the base case | The chain has no anchor — the domino argument doesn't start |
| Base case doesn't match the quantifier | If claim is ∀n ≥ 3, base case must be n = 3, not n = 0 |
| Assuming "k is the largest" | You didn't say k is maximal; you only assumed P(k) |
| Getting k and k+1 confused mid-proof | Label your expressions clearly |

---

## 12. Rust Connection — Recursion is Structural Induction

Every recursive Rust function that terminates embodies induction:

```rust
// Recursive fn mirrors inductive proof
fn triangle(n: u64) -> u64 {
    match n {
        0 => 0,                          // base case
        k => k + triangle(k - 1),       // inductive step: use result for k-1
    }
}
// claim: triangle(n) == n*(n+1)/2
```

The compiler's termination argument (decreasing n) *is* the statement that the induction is well-founded. Type-checking recurse matches the inductive hypothesis check.

---

## 13. Summary

| Form | Hypothesis used in step | Use when |
|---|---|---|
| Weak induction | P(k) only | Next value depends on exactly the preceding one |
| Strong induction | P(0) … P(k) | Next value depends on two or more earlier values |
| Well-ordering | Contradiction on min | Easiest to phrase when talking about a "first failure" |

**Checklist for any induction proof:**
1. State what P(n) is.
2. Verify the base case.
3. State the IH explicitly.
4. Derive P(k+1) using IH (and only IH, not P(k+1) itself).
5. Conclude with "by induction."

---

*Next: Lesson 6 — Set Theory*
