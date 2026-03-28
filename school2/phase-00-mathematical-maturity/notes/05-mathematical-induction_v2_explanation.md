# Mathematical Induction — Clarifying Notes

These notes are the slower explanation for Lesson 05.

The goal here is not to memorize a magic ritual. The goal is:

1. You know what the symbols mean.
2. You know what you are allowed to assume.
3. You know why the induction step works.
4. You can read induction proofs as legal line-by-line arguments.

---

## 1. What induction is trying to prove

Induction is for claims of the form:

- for every natural number `n`, a statement `P(n)` is true

Examples:

- `1 + 2 + ... + n = n(n+1)/2`
- `1 + 3 + 5 + ... + (2n-1) = n^2`
- `4 | (5^n - 1)`

The problem is that there are infinitely many natural numbers. You cannot check them one by one.

Induction replaces infinite checking with a two-part machine:

1. Show the first case is true.
2. Show that truth at one step forces truth at the next step.

If both parts work, the statement holds for all `n` in the range.

---

## 2. How to read the symbols

### `P(n)`

This means:

- the statement we care about at number `n`

Example:

```text
P(n): 1 + 2 + ... + n = n(n+1)/2
```

So:

- `P(1)` means the formula at `n = 1`
- `P(k)` means the formula at `n = k`
- `P(k+1)` means the formula at `n = k+1`

### `sum_{i=1}^n`

This is summation notation.

```text
sum_{i=1}^n i
```

means:

```text
1 + 2 + 3 + ... + n
```

Another example:

```text
sum_{i=1}^n (2i - 1)
```

means:

```text
(2·1 - 1) + (2·2 - 1) + ... + (2·n - 1)
= 1 + 3 + 5 + ... + (2n - 1)
```

### Rust equivalent of summation

```rust
let mut total = 0;
for i in 1..=n {
    total += i;
}
```

That is the Rust version of:

```text
sum_{i=1}^n i
```

And this:

```rust
let mut total = 0;
for i in 1..=n {
    total += 2 * i - 1;
}
```

is the Rust version of:

```text
sum_{i=1}^n (2i - 1)
```

### NB — Summation is basically a for loop

This:

$$
\sum_{i=1}^{n} i
$$

is mentally very close to:

```rust
let mut total = 0;
for i in 1..=n {
    total += i;
}
```

So when you read a summation, think:

- what is the loop variable?
- where does it start?
- where does it stop?
- what expression gets added each time?

Examples:

$$
\sum_{i=1}^{n} i
$$

means:

$$
1 + 2 + 3 + \cdots + n
$$

and

$$
\sum_{i=1}^{n} (2i-1)
$$

means:

$$
(2\cdot1-1) + (2\cdot2-1) + \cdots + (2\cdot n-1)
$$

So yes: summation is very much "math loop notation."

### `forall n >= 1`

This means:

- for every `n` starting from `1`

So the base case should usually start at `n = 1` for that kind of claim, not at `0`.

---

## 3. The weak induction template

To prove:

```text
for all n >= 1, P(n)
```

you do this:

```text
Base case:
show P(1)

Inductive step:
assume P(k)
show P(k+1)
```

This assumption:

```text
assume P(k)
```

is called the **inductive hypothesis** or **IH**.

The legal pattern is:

```text
IH: P(k)
Goal: P(k+1)
```

You are **not** allowed to assume `P(k+1)`.

That would be circular.

---

## 4. What the induction step is really doing

Most induction problems are really asking this:

```text
Can I rewrite the (k+1)-case so that the k-case appears inside it?
```

That is the whole induction instinct.

If the old case appears inside the new case, you can use the IH.

This is why induction feels somewhat like bottom-up dynamic programming:

- DP reuses smaller solved values to compute bigger ones
- induction reuses the statement at `k` to prove the statement at `k+1`

But induction is not computing a value. It is proving a claim.

---

## 5. Example — triangular sum

Claim:

```text
for all n >= 1,
1 + 2 + ... + n = n(n+1)/2
```

Let:

```text
P(n): 1 + 2 + ... + n = n(n+1)/2
```

### Base case

At `n = 1`:

```text
LHS = 1
RHS = 1(2)/2 = 1
```

So `P(1)` is true.

### Inductive step

Assume `P(k)`:

```text
1 + 2 + ... + k = k(k+1)/2
```

Goal: show `P(k+1)`:

```text
1 + 2 + ... + k + (k+1) = (k+1)(k+2)/2
```

Start from the left side of the `k+1` case:

```text
1 + 2 + ... + k + (k+1)
```

Now the `k`-case appears inside it:

```text
= [1 + 2 + ... + k] + (k+1)
```

Use the IH:

```text
= k(k+1)/2 + (k+1)
```

Simplify:

```text
= k(k+1)/2 + 2(k+1)/2
= (k+1)(k+2)/2
```

So `P(k+1)` is true.

That is the induction step.

---

## 6. Example — first n odd numbers

Claim:

```text
for all n >= 1,
1 + 3 + 5 + ... + (2n-1) = n^2
```

Let:

```text
P(n): 1 + 3 + 5 + ... + (2n-1) = n^2
```

At the inductive step, assume:

```text
1 + 3 + 5 + ... + (2k-1) = k^2
```

The `(k+1)` case is:

```text
1 + 3 + 5 + ... + (2k-1) + (2(k+1)-1)
```

Again, the old case appears inside the new case.

Use the IH:

```text
= k^2 + (2k + 1)
= k^2 + 2k + 1
= (k+1)^2
```

Same structure, different algebra.

---

## 6.5. One full induction proof read like a loop

Claim:

$$
\sum_{i=1}^{n} 2i = n(n+1)
\quad \text{for all } n \ge 1
$$

Let

$$
P(n): \sum_{i=1}^{n} 2i = n(n+1)
$$

### Base case

At $n=1$:

$$
\sum_{i=1}^{1} 2i = 2\cdot1 = 2
$$

and

$$
1(1+1)=2
$$

So $P(1)$ is true.

### Inductive hypothesis

Assume $P(k)$ is true:

$$
\sum_{i=1}^{k} 2i = k(k+1)
$$

### Goal

Show $P(k+1)$:

$$
\sum_{i=1}^{k+1} 2i = (k+1)(k+2)
$$

### Key line

The $(k+1)$-sum is the old sum plus the new last term:

$$
\sum_{i=1}^{k+1} 2i
=
\sum_{i=1}^{k} 2i + 2(k+1)
$$

This is the exact math version of:

```rust
let new_total = old_total + 2 * (k + 1);
```

Now use the IH:

$$
= k(k+1) + 2(k+1)
= (k+1)(k+2)
$$

So $P(k+1)$ is true.

Therefore, by induction,

$$
\sum_{i=1}^{n} 2i = n(n+1)
\quad \text{for all } n \ge 1
$$

Memory rule:

- new sum = old sum + new last term
- that is why induction fits sum formulas so naturally

---

## 7. Divisibility induction — the important rewrite trick

Example claim:

```text
for all n >= 0,
4 | (5^n - 1)
```

At the inductive step, the IH gives:

```text
4 | (5^k - 1)
```

So you want the exact chunk:

```text
5^k - 1
```

to appear in the `k+1` expression.

Start with:

```text
5^(k+1) - 1
= 5·5^k - 1
```

Now force the IH chunk to appear:

```text
= 5·5^k - 5 + 4
= 5(5^k - 1) + 4
```

That is not random.

It is reverse-engineering the expression so that:

```text
5^k - 1
```

shows up.

Memory rule:

- look at what the IH gives you
- rewrite the new expression until that exact thing appears

---

## 8. How to read divisibility in induction

```text
4 | (5^k - 1)
```

means:

```text
5^k - 1 = 4m
```

for some integer `m`.

So after rewriting:

```text
5^(k+1) - 1 = 5(5^k - 1) + 4
```

use the IH form:

```text
5^k - 1 = 4m
```

to get:

```text
5^(k+1) - 1 = 5(4m) + 4 = 4(5m + 1)
```

So the result is divisible by `4`.

---

## 9. Strong induction

Weak induction uses only:

```text
P(k)
```

to prove:

```text
P(k+1)
```

Strong induction uses:

```text
P(0), P(1), ..., P(k)
```

to prove:

```text
P(k+1)
```

So the strong IH is:

```text
Assume P(j) holds for every earlier j up to k.
```

Use strong induction when the new case depends on some smaller case that is not necessarily just `k`.

Examples:

- factorization into smaller numbers
- Fibonacci-like recurrences
- splitting a number into factors or parts

---

## 10. Strong induction example — prime divisor

Claim:

```text
Every integer n >= 2 has a prime divisor.
```

At the strong induction step, consider `k+1`.

Two cases:

1. `k+1` is prime
2. `k+1` is composite

If `k+1` is prime, it divides itself.

If `k+1` is composite, then:

```text
k+1 = ab
```

for some integers `a, b` with `2 <= a <= k`.

Since `a` is smaller than or equal to `k`, the strong IH applies to `a`.

So `a` has a prime divisor `p`:

```text
p | a
```

Also, because `k+1 = ab`, the factor `a` divides `k+1`:

```text
a | (k+1)
```

Now use the divisibility chain:

```text
if p | a and a | n, then p | n
```

So:

```text
p | (k+1)
```

Done.

Important:

You do **not** need `p | b`.
You only need one prime divisor of one factor.

---

## 11. How to interpret induction proofs in Rust

Recursive functions often mirror induction.

Example:

```rust
fn triangle(n: u64) -> u64 {
    match n {
        0 => 0,
        k => k + triangle(k - 1),
    }
}
```

This matches:

- base case: `triangle(0) = 0`
- step case: value at `k` uses the smaller case `k - 1`

Summation-style loop:

```rust
fn triangular_sum(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}
```

Formula side:

```rust
fn triangular_formula(n: u64) -> u64 {
    n * (n + 1) / 2
}
```

An induction proof is explaining why these two agree for all `n`.

---

## 12. What to ask yourself on every induction problem

1. What is `P(n)` exactly?
2. What is the first valid base case?
3. What does the IH give me exactly?
4. What is the exact goal at `k+1`?
5. Can I rewrite the `k+1` case so the `k` case appears?
6. If not, do I need strong induction instead?

---

## 13. Memory version

Weak induction:

```text
Base case
Assume P(k)
Show P(k+1)
Make the k-case appear
Use IH
Simplify
```

Strong induction:

```text
Base case(s)
Assume all earlier cases up to k
Show P(k+1)
Use whichever earlier case you need
```

Divisibility induction:

```text
Rewrite expression(k+1) so expression(k) appears
Use IH
Factor out the divisor
```