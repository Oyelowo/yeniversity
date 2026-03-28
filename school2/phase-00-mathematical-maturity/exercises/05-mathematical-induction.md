# Exercises 05 — Mathematical Induction

> **Instructions:** Attempt each proof on paper first. Then implement the Rust witness in `src/induction.rs`.  
> Mark each exercise ✅ only when you can reproduce the proof from memory.

---

## Tier 1 — Mechanical (apply the template directly)

**E1.** Prove by induction: for all n ≥ 1,

$$\sum_{i=1}^{n} 2i = n(n+1)$$

> **Proof strategy:** Weak induction
>
> **Why this strategy:**
> the claim is indexed by n, and the (k+1)-case can be rewritten as the k-case plus one new term.
>
> **Proof:**
Let

$$P(n): \sum_{i=1}^{n} 2i = n(n+1).$$

Base case (n = 1):

Left side:

$$\sum_{i=1}^{1} 2i = 2 \cdot 1 = 2$$

Right side:

$$1(1+1) = 2$$

So P(1) is true.

Inductive step:

Assume P(k) is true for some arbitrary k ≥ 1.
This is the inductive hypothesis:

$$\sum_{i=1}^{k} 2i = k(k+1).$$

Goal: show

$$\sum_{i=1}^{k+1} 2i = (k+1)(k+2).$$

Start from the left side of the (k+1)-case:

$$\sum_{i=1}^{k+1} 2i = \sum_{i=1}^{k} 2i + 2(k+1).$$

Use the IH:

$$= k(k+1) + 2(k+1)$$

Factor out (k+1):

$$= (k+1)(k+2).$$

This is exactly the right side of P(k+1).

Therefore P(k+1) is true.

By the principle of mathematical induction, for all n ≥ 1,

$$\sum_{i=1}^{n} 2i = n(n+1). \quad \square$$

Rust-style pseudocode:

```rust
fn sum_even_numbers_iter(n: u64) -> u64 {
	let mut total = 0;
	for i in 1..=n {
		total += 2 * i;
	}
	total
}

fn sum_even_numbers_formula(n: u64) -> u64 {
	n * (n + 1)
}
```

**E2.** Prove by induction: for all n ≥ 1,

$$\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}$$

(Hint: the algebra is heavier here — be patient with the factoring.)

> **Proof strategy:** Weak induction
>
> **Why this strategy:**
> the claim is indexed by n, and the (k+1)-case can be rewritten as the k-case plus one new term, namely (k+1)^2.
>
> **Proof:**
Let

$$P(n): \sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}.$$

Base case (n = 1):

Left side:

$$\sum_{i=1}^{1} i^2 = 1^2 = 1$$

Right side:

$$\frac{1(1+1)(2\cdot1+1)}{6} = \frac{1\cdot2\cdot3}{6} = 1$$

So P(1) is true.

Inductive step:

Assume P(k) is true for some arbitrary k ≥ 1.
This is the inductive hypothesis:

$$\sum_{i=1}^{k} i^2 = \frac{k(k+1)(2k+1)}{6}.$$

Goal: show

$$\sum_{i=1}^{k+1} i^2 = \frac{(k+1)(k+2)(2k+3)}{6}.$$

Start from the left side of the (k+1)-case:

$$\sum_{i=1}^{k+1} i^2 = \sum_{i=1}^{k} i^2 + (k+1)^2.$$

Use the IH:

$$= \frac{k(k+1)(2k+1)}{6} + (k+1)^2$$

Put everything over a common denominator of 6:

$$= \frac{k(k+1)(2k+1)}{6} + \frac{6(k+1)^2}{6}$$

Factor out (k+1)/6:

$$= \frac{k+1}{6}\bigl(k(2k+1) + 6(k+1)\bigr)$$

Simplify inside the brackets:

$$= \frac{k+1}{6}(2k^2 + k + 6k + 6)$$

$$= \frac{k+1}{6}(2k^2 + 7k + 6)$$

Factor the quadratic:

$$= \frac{k+1}{6}(2k+3)(k+2)$$

Reorder the factors:

$$= \frac{(k+1)(k+2)(2k+3)}{6}$$

This is exactly the right side of P(k+1).

Therefore P(k+1) is true.

By the principle of mathematical induction, for all n ≥ 1,

$$\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}. \quad \square$$

Rust-style pseudocode:

```rust
fn sum_of_squares_iter(n: u64) -> u64 {
	let mut total = 0;
	for i in 1..=n {
		total += i * i;
	}
	total
}

fn sum_of_squares_formula(n: u64) -> u64 {
	n * (n + 1) * (2 * n + 1) / 6
}
```

**E3.** Prove by induction: for all n ≥ 0, 3 | (4ⁿ − 1).

> **Proof strategy:** Weak induction
>
> **Why this strategy:**
> the statement is indexed by n, and in the (k+1)-case we can rewrite 4^(k+1) in terms of 4^k, which lets the inductive hypothesis appear.
>
> **Proof:**
Let

$$P(n): 3 \mid (4^n - 1).$$

Base case (n = 0):

$$4^0 - 1 = 1 - 1 = 0.$$

Since 3 divides 0, P(0) is true.

Inductive step:

Assume P(k) is true for some arbitrary k ≥ 0.
Then

$$3 \mid (4^k - 1).$$

By definition of divisibility, this means there exists an integer m such that

$$4^k - 1 = 3m.$$

Goal: show

$$3 \mid (4^{k+1} - 1).$$

Start with the expression we want to study:

$$4^{k+1} - 1 = 4\cdot4^k - 1.$$

Now add and subtract 4 in a useful way:

$$4\cdot4^k - 1 = 4\cdot4^k - 4 + 3$$

$$= 4(4^k - 1) + 3.$$

Use the IH information that 4^k - 1 = 3m:

$$4^{k+1} - 1 = 4(3m) + 3$$

$$= 12m + 3$$

$$= 3(4m + 1).$$

Since 4m + 1 is an integer, 4^{k+1} - 1 is a multiple of 3.
Therefore

$$3 \mid (4^{k+1} - 1).$$

So P(k+1) is true.

By the principle of mathematical induction, for all n ≥ 0,

$$3 \mid (4^n - 1). \quad \square$$

Rust-style pseudocode:

```rust
fn divisible_by_3_power_minus_1(n: u32) -> bool {
	(4_i128.pow(n) - 1) % 3 == 0
}
```

**E4.** Prove by induction: for all n ≥ 1, 2ⁿ ≥ n + 1.

> **Proof strategy:** Weak induction
>
> **Why this strategy:**
> the claim is indexed by n, and the left side naturally changes from 2^k to 2^(k+1) = 2\cdot2^k, so the IH can be used directly.
>
> **Proof:**
Let

$$P(n): 2^n \ge n + 1.$$

Base case (n = 1):

Left side:

$$2^1 = 2$$

Right side:

$$1 + 1 = 2$$

So

$$2^1 \ge 1 + 1,$$

and P(1) is true.

Inductive step:

Assume P(k) is true for some arbitrary k ≥ 1.
Then

$$2^k \ge k + 1.$$

Goal: show

$$2^{k+1} \ge k + 2.$$

Start from the left side:

$$2^{k+1} = 2\cdot2^k.$$

Use the IH:

$$2^{k+1} \ge 2(k+1) = 2k + 2.$$

Now compare 2k + 2 with k + 2.
Since k ≥ 0, we have

$$2k + 2 \ge k + 2.$$

Therefore

$$2^{k+1} \ge 2k + 2 \ge k + 2.$$

So

$$2^{k+1} \ge k + 2,$$

which is exactly P(k+1).

Therefore P(k+1) is true.

By the principle of mathematical induction, for all n ≥ 1,

$$2^n \ge n + 1. \quad \square$$

Memory jog:

- Use the IH on 2^k.
- Multiplying by 2 makes the left side large enough.
- Then compare 2k + 2 with k + 2.

---

## Tier 2 — Requires a small twist

**E5.** Prove by induction: for all n ≥ 4, n! > 2ⁿ.

(Note: the base case is n = 4, *not* n = 0. Show both sides for n = 4 first.)

> **Proof strategy:** Weak induction
>
> **Why this strategy:**
> the statement is indexed by n, and factorial naturally grows by one extra factor when we go from k! to (k+1)!, so the IH can be multiplied by (k+1).
>
> **Proof:**
Let

$$P(n): n! > 2^n \quad \text{for } n \ge 4.$$

Base case (n = 4):

Left side:

$$4! = 24$$

Right side:

$$2^4 = 16$$

Since

$$24 > 16,$$

P(4) is true.

Inductive step:

Assume P(k) is true for some arbitrary k ≥ 4.
Then

$$k! > 2^k.$$

Goal: show

$$ (k+1)! > 2^{k+1}. $$

Start from the left side:

$$ (k+1)! = (k+1)k!. $$

Use the IH:

$$ (k+1)! > (k+1)2^k. $$

Now compare (k+1)2^k with 2^{k+1} = 2\cdot2^k.
Since k ≥ 4, we have k+1 ≥ 5, and in particular

$$k+1 > 2.$$

Multiplying by 2^k > 0 gives

$$ (k+1)2^k > 2\cdot2^k = 2^{k+1}. $$

Therefore

$$ (k+1)! > (k+1)2^k > 2^{k+1}. $$

So P(k+1) is true.

By the principle of mathematical induction, for all n ≥ 4,

$$ n! > 2^n. \quad \square $$

Rust-style pseudocode:

```rust
fn factorial(n: u64) -> u64 {
	(1..=n).product()
}

fn factorial_beats_power_of_two(n: u64) -> bool {
	factorial(n) > 2_u64.pow(n as u32)
}
```

**E6.** Prove by **strong** induction: every integer n ≥ 2 can be written as a product of primes (Fundamental Theorem of Arithmetic — existence part).

(This is the same structure as Example 8.1 in the notes; extend it to products.)

> **Proof strategy:** Strong induction
>
> **Why this strategy:**
> if n is composite, we break it into smaller factors a and b. To conclude those smaller factors are products of primes, we need the statement for all smaller numbers, not just for n-1.
>
> **Proof:**
Let

$$P(n): \text{the integer } n \ge 2 \text{ can be written as a product of primes}.$$

Base case (n = 2):

The number 2 is itself prime, so it is already a product of primes.
Thus P(2) is true.

Inductive step:

Assume that for some k ≥ 2, every integer m with

$$2 \le m \le k$$

can be written as a product of primes.

This is the strong inductive hypothesis.

Goal: show that k+1 can be written as a product of primes.

There are two cases.

Case 1: k+1 is prime.

Then k+1 is already a product of primes, namely just itself.

Case 2: k+1 is composite.

Then there exist integers a and b such that

$$k+1 = ab$$

with

$$2 \le a < k+1 \quad \text{and} \quad 2 \le b < k+1.$$

Why are these inequalities true?

- Because k+1 is composite, it has a factorization with integers a,b > 1.
	So a ≥ 2 and b ≥ 2.
- Also, neither factor can equal k+1 itself, because then the other factor would have to be 1:

$$a = k+1 \implies b = 1, \qquad b = k+1 \implies a = 1,$$

which would contradict the fact that the number is composite.
- Therefore both factors are strictly smaller than k+1.

So in fact

$$2 \le a \le k \quad \text{and} \quad 2 \le b \le k.$$

Because a and b are both between 2 and k, the strong inductive hypothesis applies to both of them.
So:

- a can be written as a product of primes,
- b can be written as a product of primes.

More explicitly, the strong IH says:

> every integer m with 2 ≤ m ≤ k can be written as a product of primes.

Since a is such an integer, a can be written as a product of primes.
Since b is also such an integer, b can be written as a product of primes.

But then

$$k+1 = ab$$

is just the product of those prime factors together.
Therefore k+1 can be written as a product of primes.

In either case, P(k+1) is true.

By the principle of strong induction, every integer n ≥ 2 can be written as a product of primes. \quad \square

Memory jog:

- If the number is prime, stop.
- If it is composite, split it into smaller factors.
- Apply the strong IH to the smaller factors.
- Multiply the prime factorizations together.

**E7.** Define the sequence:
- a(0) = 1
- a(1) = 3
- a(n) = a(n−1) + 2·a(n−2) for n ≥ 2

Prove by **strong** induction that

$$a(n) = \frac{2^{n+2} - (-1)^n}{3}$$

for all n ≥ 0.

> **Proof strategy:** Strong induction
>
> **Why this strategy:**
> the recurrence for a(n) depends on the two previous terms, a(n-1) and a(n-2). To prove the formula for a(k+1), we need formulas for both earlier terms.
>
> **Proof:**
Let

$$P(n): a(n) = 2^{n+1} - (-1)^n.$$

We prove P(n) for all n ≥ 0 by strong induction.

Base case 1 (n = 0):

From the definition,

$$a(0) = 1.$$

The claimed formula gives

$$2^{0+1} - (-1)^0 = 2^1 - 1 = 2 - 1 = 1.$$

So P(0) is true.

Base case 2 (n = 1):

From the definition,

$$a(1) = 3.$$

Let

$$Q(n): a(n) = \frac{2^{n+2} - (-1)^n}{3}.$$

Base cases:

For n = 0:

$$a(0) = 1 = \frac{4 - 1}{3}.$$

For n = 1:

$$a(1) = 3 = \frac{8 - (-1)}{3} = \frac{9}{3}.$$

So Q(0) and Q(1) are true.

Inductive step:

Assume that for some k ≥ 1, Q(j) is true for every j with 0 ≤ j ≤ k.

In particular,

$$a(k) = \frac{2^{k+2} - (-1)^k}{3}$$

and

$$a(k-1) = \frac{2^{k+1} - (-1)^{k-1}}{3}.$$

Goal: show

$$a(k+1) = \frac{2^{k+3} - (-1)^{k+1}}{3}.$$

Use the recurrence:

$$a(k+1) = a(k) + 2a(k-1).$$

Substitute the inductive formulas:

$$a(k+1) = \frac{2^{k+2} - (-1)^k}{3} + 2\left(\frac{2^{k+1} - (-1)^{k-1}}{3}\right).$$

Put over the common denominator 3:

$$a(k+1) = \frac{2^{k+2} - (-1)^k + 2^{k+2} - 2(-1)^{k-1}}{3}.$$

Now simplify the sign part.
Since

$$(-1)^k = -(-1)^{k-1},$$

we have

$$-(-1)^k - 2(-1)^{k-1} = -\bigl(-(-1)^{k-1}\bigr) - 2(-1)^{k-1}$$

$$= (-1)^{k-1} - 2(-1)^{k-1}$$

$$= -(-1)^{k-1} = (-1)^k.$$

So

$$a(k+1) = \frac{2^{k+2} + 2^{k+2} + (-1)^k}{3}$$

$$= \frac{2^{k+3} + (-1)^k}{3}.$$

Finally, since

$$-(-1)^{k+1} = (-1)^k,$$

this becomes

$$a(k+1) = \frac{2^{k+3} - (-1)^{k+1}}{3}.$$

Thus Q(k+1) is true.

By strong induction, for all n ≥ 0,

$$a(n) = \frac{2^{n+2} - (-1)^n}{3}. \quad \square$$

Memory jog:

- First check the claimed closed form against the base cases.
- If a base case fails, the formula is wrong.
- For 2-step recurrences, strong induction is natural because you need two earlier terms.

---

## Tier 3 — Reasoning carefully

**E8.** Find the flaw in this "proof":

> **Claim:** All horses are the same color.
>
> *Proof by induction on n (size of a group of horses).*
>
> Base case (n = 1): A group of one horse has only one color. ✓
>
> Inductive step: Assume any group of k horses is monochromatic. Take a group of k+1 horses. Remove horse 1 — the remaining k horses are all the same color (by IH). Remove horse k+1 — the remaining k horses are all the same color (by IH). The color is shared, so all k+1 are the same color. □

Where exactly does the inductive step fail? Write one sentence.

The inductive step fails when going from n = 1 to n = 2, because after removing one horse from a two-horse group, the two remaining one-horse subgroups have no overlapping horse, so there is no shared horse forcing the colors to match.

**E9.** We have the inequality:

$$\forall n \ge 1: \quad \left(1 + \frac{1}{n}\right)^n < 3$$

Verify this holds for n = 1, 2, 3 numerically. Then explain why a proof by ordinary weak induction is not straightforward here (the IH for n = k doesn't immediately give you n = k+1). What would a proof strategy look like?

Numerical check:

For n = 1:

$$\left(1 + \frac{1}{1}\right)^1 = 2 < 3.$$

For n = 2:

$$\left(1 + \frac{1}{2}\right)^2 = \left(\frac{3}{2}\right)^2 = \frac{9}{4} = 2.25 < 3.$$

For n = 3:

$$\left(1 + \frac{1}{3}\right)^3 = \left(\frac{4}{3}\right)^3 = \frac{64}{27} \approx 2.37 < 3.$$

Why ordinary weak induction is not straightforward:

If we assume

$$\left(1 + \frac{1}{k}\right)^k < 3,$$

this does not immediately tell us anything clean about

$$\left(1 + \frac{1}{k+1}\right)^{k+1}.$$

The problem is that both the base and the exponent change at the same time.
So the (k+1)-expression is not just the k-expression with one extra simple term attached.

In the easier induction examples, we could rewrite the new case so that the old case visibly appeared inside it:

- sums: old sum + one new term,
- factorial: old factorial times one new factor,
- divisibility powers: rewrite in terms of the old power.

Here, however,

$$\left(1 + \frac{1}{k+1}\right)^{k+1}$$

does not naturally contain

$$\left(1 + \frac{1}{k}\right)^k$$

in a usable algebraic way.

What a proof strategy would look like:

One standard strategy is **not** ordinary induction, but rather to show that the sequence

$$b_n = \left(1 + \frac{1}{n}\right)^n$$

is increasing and bounded above.

Then you prove separately that

$$b_n < 3$$

for all n.

Another standard route is to use the binomial theorem:

$$\left(1 + \frac{1}{n}\right)^n = 1 + 1 + \frac{n(n-1)}{2!n^2} + \frac{n(n-1)(n-2)}{3!n^3} + \cdots,$$

and then estimate the terms by a geometric-type series to show the total is less than 3.

So the key idea is:

- weak induction is awkward because the new expression does not expose the old one,
- a better proof uses monotonicity, binomial expansion, or a known bound related to e.

---

## Rust Additions to `src/induction.rs`

Add these three functions and their tests:

```rust
/// Returns the sum 1² + 2² + … + n²  (iterative).
pub fn sum_of_squares_iter(n: u64) -> u64 { ... }

/// Closed form: n(n+1)(2n+1)/6.
pub fn sum_of_squares_formula(n: u64) -> u64 { ... }

/// Returns n! (factorial) — panics if n > 20 to avoid overflow.
pub fn factorial(n: u64) -> u64 { ... }
```

Write `#[test]` that:
1. Confirms `sum_of_squares_iter(n) == sum_of_squares_formula(n)` for n ∈ [0, 100].
2. Confirms `factorial(n) > 2u64.pow(n as u32)` for n ∈ [5, 20] (note: n ≥ 5 not 4, since 4! = 24 = 2⁴ — they're equal at n=4 and strict inequality starts at n=5; re-read E5 if this surprises you).

---

## Self-Test Checklist

- [ ] I can write a weak induction proof template from memory
- [ ] I can write a strong induction proof template from memory
- [ ] I know the difference: when does strong induction become necessary?
- [ ] I reproduced the triangular-sum proof without notes
- [ ] I found the exact flaw in E8 (hint: it is in the step n = 1 → n = 2)
