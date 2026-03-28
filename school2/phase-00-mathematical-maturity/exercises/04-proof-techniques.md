# Exercises — Lesson 4: Proof Techniques

> Fill in every blank. Show all steps. Name your proof strategy.  
> "By definition", "by algebra", "by Example 4.1" etc. are all valid justifications.  
> Submit when done and the tutor will mark each exercise.

---
Symbols
¬ negation
∀ For All
≡ Equal
∃ There Exists/Any
∈ Membershipt/In
⊥ False
## E1 — Classify the proof strategy

For each claim, state which proof strategy you would use *first* and why (one sentence). You do not need to write the full proof.

**(a)** For all n ∈ ℤ: if n³ is odd, then n is odd.

> Your answer:
I will use contrapositive, because it is easier to work with n even than with n³ odd.

Original claim:
∀n ∈ ℤ, if n³ is odd, then n is odd.

Contrapositive claim:
∀n ∈ ℤ, if n is even, then n³ is even.

Assumption: n is even.
Goal: n³ is even.

Since n is even, there exists k ∈ ℤ such that n = 2k. [definition of even]

Then
n³ = (2k)³
   = 8k³
   = 2(4k³).

Since integers are closed under multiplication, 4k³ ∈ ℤ.
Therefore n³ has the form 2(integer), so n³ is even. [definition of even]

Thus the contrapositive is true, and therefore the original claim is true.

**(b)** There is no largest prime number.

> Your answer:
Claim: There is no largest prime number.

Strategy: Contradiction.

To use contradiction, negate the claim:

Original claim P: "There is no largest prime number."
Negation ¬P: "There is a largest prime number."

Assume, for contradiction, that there is a largest prime number.

Goal: derive a contradiction from the assumption that there is a largest prime number.

Then the set of all prime numbers is finite, so we can list it as

{p1, p2, ..., pn}

where each pi is prime and pn is the largest prime.

Let

P = p1 * p2 * ... * pn

and define

N = P + 1.

For each prime pi in the list, pi divides P because pi is one of the factors of P.
So when N is divided by pi, the remainder is 1:

N mod pi = 1.

Therefore none of p1, p2, ..., pn divides N.

Also, every prime is at least 2, so P >= 2, hence N = P + 1 >= 3 > 1.
Every integer greater than 1 has a prime divisor, so there exists a prime q such that q | N.

But q cannot be any of p1, p2, ..., pn, because none of those primes divides N.
So q is a prime not in the supposedly complete list {p1, p2, ..., pn}.

This contradicts the assumption that {p1, p2, ..., pn} contains all prime numbers.
Therefore there is no largest prime number. □

Rust-style pseudocode:

```rust
let primes = vec![p1, p2, /* ... */, pn]; // assumed complete list of all primes
let product: u128 = primes.iter().product();
let n = product + 1;

for p in &primes {
    assert_eq!(product % p, 0);
    assert_eq!(n % p, 1); // so no listed prime divides n
}

let q = prime_divisor(n); // valid because n > 1
assert!(!primes.contains(&q)); // contradiction
```



**(c)** For all n ∈ ℤ: n² − n is even.

> Your answer:
Claim: For all n ∈ ℤ: n² − n is even
Assumption: n² − n is even for all integers n
Goal: For both odd and even, n² − n is even

Claim: For all n ∈ ℤ, n² − n is even.

Strategy: Cases.

Why cases are valid: every integer is either even or odd.

Case 1: n is even.
Then there exists k ∈ ℤ such that n = 2k. [definition of even]

So
n² − n = (2k)² − 2k
      = 4k² − 2k
      = 2(2k² − k).

Since integers are closed under multiplication and subtraction, 2k² − k ∈ ℤ.
Therefore n² − n has the form 2(integer), so it is even.

Case 2: n is odd.
Then there exists k ∈ ℤ such that n = 2k + 1. [definition of odd]

So
n² − n = (2k + 1)² − (2k + 1)
      = (4k² + 4k + 1) − (2k + 1)
      = 4k² + 2k
      = 2(2k² + k).

Since integers are closed under multiplication and addition, 2k² + k ∈ ℤ.
Therefore n² − n has the form 2(integer), so it is even.

In both cases, n² − n is even.
Therefore, for all n ∈ ℤ, n² − n is even. □

Rust-style pseudocode:

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn square_minus_self_is_even(n: i64) -> bool {
    if is_even(n) {
        let k = n / 2;
        n * n - n == 2 * (2 * k * k - k)
    } else {
        let k = (n - 1) / 2;
        n * n - n == 2 * (2 * k * k + k)
    }
}
```

**(d)** For all x ∈ ℝ: if x² = 2, then x ∉ ℚ.

> Your answer:
Claim: For all x ∈ ℝ, if x² = 2, then x ∉ ℚ.

Proof strategy: Contradiction.

Let
P = "x² = 2"
Q = "x ∉ ℚ"

Original claim: P -> Q

To use contradiction, negate the whole implication:

¬(P -> Q)
= ¬(¬P ∨ Q)        [implication law]
= P ∧ ¬Q           [De Morgan]

So the contradiction assumption is:
x² = 2 and x ∈ ℚ.

Assumption: x² = 2 and x ∈ ℚ.
Goal: derive a contradiction.

Since x ∈ ℚ, there exist integers p, q with q ≠ 0 such that

x = p / q

and we may assume gcd(p, q) = 1. [lowest terms]

Substitute into x² = 2:

(p / q)² = 2
p² / q² = 2
p² = 2q².

So p² is even, hence p is even.
Therefore there exists m ∈ ℤ such that p = 2m.

Substitute back:

(2m)² = 2q²
4m² = 2q²
2m² = q².

So q² is even, hence q is even.

Thus both p and q are even, so 2 divides both p and q.
Therefore gcd(p, q) ≠ 1.

But we assumed gcd(p, q) = 1.
Contradiction.

Therefore x ∉ ℚ. □

Alternate proof strategy: Contrapositive.

Contrapositive claim:
If x ∈ ℚ, then x² ≠ 2.

This comes from

P -> Q
≡ ¬Q -> ¬P

where
¬Q = "x ∈ ℚ"
¬P = "x² ≠ 2".

Assumption: x ∈ ℚ.
Goal: show x² ≠ 2.

Since x ∈ ℚ, write x = p / q with p, q ∈ ℤ, q ≠ 0, and gcd(p, q) = 1.

Assume, for contradiction, that x² = 2.
Then the same algebra as above gives:

p² = 2q²,

so p is even, and then q is even.
Thus gcd(p, q) ≠ 1, contradicting gcd(p, q) = 1.

So x² ≠ 2.
Hence, if x ∈ ℚ, then x² ≠ 2.
Therefore, by contrapositive, if x² = 2, then x ∉ ℚ. □

Rust-style pseudocode:

```rust
// Contradiction version
assume(x * x == 2.0);
assume_rational(x); // x = p / q, gcd(p, q) = 1

let (p, q) = reduced_fraction_for(x);
assert_eq!(p * p, 2 * q * q);
assert!(is_even(p));
assert!(is_even(q));
assert!(gcd(p, q) != 1); // contradiction

// Contrapositive version
assume_rational(x); // x = p / q, gcd(p, q) = 1
assume(x * x == 2.0); // temporary contradiction sub-assumption
let (p, q) = reduced_fraction_for(x);
assert_eq!(p * p, 2 * q * q);
assert!(is_even(p));
assert!(is_even(q));
assert!(gcd(p, q) != 1); // contradiction, so x*x != 2
```

---

## E2 — Direct proof

Write a complete direct proof for each. Label each step with a justification.

**(a)** **Claim:** For all m, n ∈ ℤ: if m is even and n is even, then m + n is even.

> **Proof strategy:** Direct  
> **Proof:**  
Assume m is even and n is even.

Since m is even, there exists k ∈ ℤ such that m = 2k. [definition of even]
Since n is even, there exists j ∈ ℤ such that n = 2j. [definition of even]

Then

m + n = 2k + 2j
      = 2(k + j).

Since integers are closed under addition, k + j ∈ ℤ.
Therefore m + n has the form 2(integer), so m + n is even. □

Rust-style pseudocode:

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn even_plus_even_is_even(m: i64, n: i64) -> bool {
    if is_even(m) && is_even(n) {
        let k = m / 2;
        let j = n / 2;
        m + n == 2 * (k + j)
    } else {
        true
    }
}
```

**(b)** **Claim:** For all m, n ∈ ℤ: if m is odd and n is odd, then mn is odd.

> **Proof strategy:** Direct  
> **Proof:**  
Assume m is odd and n is odd.

Since m is odd, there exists a ∈ ℤ such that m = 2a + 1. [definition of odd]
Since n is odd, there exists b ∈ ℤ such that n = 2b + 1. [definition of odd]

Then

mn = (2a + 1)(2b + 1)
   = 4ab + 2a + 2b + 1
   = 2(2ab + a + b) + 1.

Since integers are closed under multiplication and addition, 2ab + a + b ∈ ℤ.
Therefore mn has the form 2(integer) + 1, so mn is odd. □

Rust-style pseudocode:

```rust
fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

fn odd_times_odd_is_odd(m: i64, n: i64) -> bool {
    if is_odd(m) && is_odd(n) {
        let a = (m - 1) / 2;
        let b = (n - 1) / 2;
        m * n == 2 * (2 * a * b + a + b) + 1
    } else {
        true
    }
}
```

**(c)** **Claim:** For all m ∈ ℤ: if m is even, then m² is divisible by 4.

> **Proof strategy:** Direct  
> **Proof:**
Assume m is even.

Then there exists k ∈ ℤ such that m = 2k. [definition of even]

So

m² = (2k)²
   = 4k²
   = 4(k²).

Since integers are closed under multiplication, k² ∈ ℤ.
Therefore m² has the form 4(integer), so 4 divides m². □

Rust-style pseudocode:

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn even_square_divisible_by_4(m: i64) -> bool {
    if is_even(m) {
        let k = m / 2;
        m * m == 4 * (k * k)
    } else {
        true
    }
}
```

---

## E3 — Proof by contrapositive

Write a complete proof by contrapositive. Clearly state what the contrapositive is before proving it.

**(a)** **Claim:** For all n ∈ ℤ: if n² is even, then n is even.

> **State the contrapositive:**  
If n is odd, then n² is odd.

> **Proof:**
We prove the contrapositive.

Assume n is odd.
Then there exists k ∈ ℤ such that n = 2k + 1. [definition of odd]

So

n² = (2k + 1)²
   = (2k + 1)(2k + 1)
   = 4k² + 4k + 1
   = 2(2k² + 2k) + 1.

Since integers are closed under multiplication and addition, 2k² + 2k ∈ ℤ.
Therefore n² has the form 2(integer) + 1, so n² is odd.

Thus the contrapositive is true.
Therefore, if n² is even, then n is even. □

Rust-style pseudocode:

```rust
fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

fn odd_implies_odd_square(n: i64) -> bool {
    if is_odd(n) {
        let k = (n - 1) / 2;
        n * n == 2 * (2 * k * k + 2 * k) + 1
    } else {
        true
    }
}
```

**(b)** **Claim:** For all a, b ∈ ℤ: if a × b is odd, then both a and b are odd.  
*(Hint: the contrapositive uses "or — use De Morgan.)*

> **State the contrapositive:**  
If a is even or b is even, then ab is even.

> **Proof:**
We prove the contrapositive.

Original claim:
If ab is odd, then both a and b are odd.

Let
P = "ab is odd"
Q = "a and b are both odd"

Then the contrapositive is
¬Q -> ¬P.

By De Morgan,
¬Q = ¬(a and b both odd)
   = (a is not odd) or (b is not odd)
   = (a is even) or (b is even).

So the contrapositive is:
If a is even or b is even, then ab is even.

Case 1: a is even.
Then there exists k ∈ ℤ such that a = 2k. [definition of even]
So

ab = (2k)b = 2(kb).

Since integers are closed under multiplication, kb ∈ ℤ.
Therefore ab has the form 2(integer), so ab is even.

Case 2: b is even.
Then there exists j ∈ ℤ such that b = 2j. [definition of even]
So

ab = a(2j) = 2(aj).

Since integers are closed under multiplication, aj ∈ ℤ.
Therefore ab has the form 2(integer), so ab is even.

In either case, if a is even or b is even, then ab is even.
Thus the contrapositive is true.
Therefore, if ab is odd, then both a and b are odd. □

Rust-style pseudocode:

```rust
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn even_factor_implies_even_product(a: i64, b: i64) -> bool {
    if is_even(a) || is_even(b) {
        (a * b) % 2 == 0
    } else {
        true
    }
}
```
---

## E4 — Proof by contradiction

**(a)** **Claim:** √3 is irrational.

> **Proof strategy:** Contradiction  
> **Proof:**  
Assume, for contradiction, that √3 ∈ ℚ.
Then there exist integers p, q with q ≠ 0 such that

√3 = p / q

and we may assume gcd(p, q) = 1. [lowest terms]

Square both sides:

(p / q)² = 3
p² / q² = 3
p² = 3q².

Since q ∈ ℤ, we have q² ∈ ℤ. [integers are closed under multiplication]
So p² = 3 · (integer).
Therefore 3 | p². [definition of divisibility]

Euclid's lemma: if a prime number r divides ab, then r | a or r | b.

Here 3 is prime and p² = p · p.
So from 3 | p², Euclid's lemma gives:

3 | p or 3 | p.

Hence 3 | p.
Therefore there exists m ∈ ℤ such that p = 3m. [definition of divisibility]

Substitute back:

(3m)² = 3q²
9m² = 3q²
3m² = q².

Since m ∈ ℤ, we have m² ∈ ℤ. [integers are closed under multiplication]
So q² = 3 · (integer).
Therefore 3 | q². [definition of divisibility]

Again, q² = q · q and 3 is prime.
So by Euclid's lemma, 3 | q or 3 | q.
Hence 3 | q.

Thus 3 divides both p and q, so p and q have common factor 3.
Therefore gcd(p, q) ≠ 1.

But we assumed gcd(p, q) = 1.
Contradiction.

Therefore √3 ∉ ℚ. □

Rust-style pseudocode:

```rust
assume_rational(sqrt_3); // sqrt_3 = p / q, gcd(p, q) = 1

let (p, q) = reduced_fraction_for_sqrt_3();
assert_eq!(p * p, 3 * q * q);

assert!(divides(3, p * p));
assert!(divides(3, p));

assert!(divides(3, q * q));
assert!(divides(3, q));

assert!(gcd(p, q) != 1); // contradiction
```



**(b)** **Claim:** There is no largest integer.  
*(i.e. there is no n ∈ ℤ such that n ≥ m for all m ∈ ℤ)*

> **Proof:**  
> Assume ___  
> *(derive a contradiction)*

---

## E5 — Proof by cases

**(a)** **Claim:** For all n ∈ ℤ: n³ − n is divisible by 3.  
*(Hint: every integer is congruent to 0, 1, or 2 mod 3 — that's your three cases.)*  
*(You may use: a ≡ r (mod 3) means a = 3k + r for some k ∈ ℤ)*

> **Case 1 (n ≡ 0 mod 3):**  
a = 
> **Case 2 (n ≡ 1 mod 3):**  
> **Case 3 (n ≡ 2 mod 3):**  
> **Conclusion:**

**(b)** **Claim:** For all n ∈ ℤ: n² is either ≡ 0 (mod 4) or ≡ 1 (mod 4).  
*(Hint: split into even n and odd n.)*

> **Case 1 (n even):**  
> **Case 2 (n odd):**  
> **Conclusion:**

---

## E6 — Full proof (choose your strategy)

Write a clean, complete proof. After finishing, explain in one sentence why you chose that strategy.

**(a)** **Claim:** For all a, b ∈ ℤ: if a + b is odd, then exactly one of a, b is odd.

**(b)** **Claim:** log₂ 3 is irrational.  
*(Recall: log₂ 3 = x means 2^x = 3.)*

---

## Rust Challenge

The following skeleton is in `src/proofs.rs`. Complete the `todo!()` bodies.

```rust
/// Returns true if n is even.
pub fn is_even(n: i64) -> bool {
    todo!()
}

/// Returns true if n is odd.
pub fn is_odd(n: i64) -> bool {
    todo!()
}

/// Verifies E2(a): even + even = even, for all m,n in domain.
pub fn even_plus_even_is_even(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E2(b): odd * odd = odd, for all m,n in domain.
pub fn odd_times_odd_is_odd(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E3(a) contrapositive: even n → even n², for all n in domain.
pub fn even_square_contrapositive(domain: &[i64]) -> bool {
    todo!()
}

/// Verifies E5(a): n³ - n divisible by 3, for all n in domain.
pub fn cube_minus_n_div3(domain: &[i64]) -> bool {
    todo!()
}
```

Then make all the tests in `src/proofs.rs` pass.
