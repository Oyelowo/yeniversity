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
> Assume m is even, n is even  
> Step 1:  
For all m, there exists k, where m = 2k
m = 2k
For all n, there exists j, where m = 2j
n = 2j

> Step 2:  
m+n=2k+2j=2(k+j) closed under addition
2(integer)
> Therefore m+n is even □

**(b)** **Claim:** For all m, n ∈ ℤ: if m is odd and n is odd, then mn is odd.

> **Proof strategy:** Direct  
> **Proof:**  
> Assume ___ m is odd, n is odd
> Step 1:  
Defintion of odd:
For all n, there exists k, where n=2k+1

m=2a+1
n=2b+1
mn=(2a+1)*(2b+1)
=4ab+2a+2b+1
=2(2ab+a+b)+1

> Step 2:  
2ab+a+b, closed under multiplication, addition
t=2ab+a+b
2t+1

True

> Therefore ___ □ mn is odd

**(c)** **Claim:** For all m ∈ ℤ: if m is even, then m² is divisible by 4.

> **Proof strategy:** Direct  
> **Proof:**
Assumption: m is even, m=2k
Goal: m² is divisible by 4, 4|m²

m²/4=(2k)²/4 = 4k²/4 = k²
k², closed under multiplication
For all k ∈ ℤ, k² is an integer

True

---

## E3 — Proof by contrapositive

Write a complete proof by contrapositive. Clearly state what the contrapositive is before proving it.

**(a)** **Claim:** For all n ∈ ℤ: if n² is even, then n is even.

> **State the contrapositive:**  
P->Q=¬Q->¬P
if not(n is even), then not(n² is even)
= if n is odd, then n² is odd

> **Proof:**

n=2k+1
n²=(2k+1)²
=(2k+1)*(2k+1)
=4k²+2k+2k+1
=4k²+4k+1
=2(2k²+2k)+1
= 2(t)+1, t=Integers, 2k²+2k closed under multiplication, addition

2t+1 is odd

True

**(b)** **Claim:** For all a, b ∈ ℤ: if a × b is odd, then both a and b are odd.  
*(Hint: the contrapositive uses "or — use De Morgan.)*

> **State the contrapositive:**  
P->Q=¬Q->¬P
if not(both a and b are odd)
then not(a x b) is odd
<!-- =
if both a and b are even
then a x b is even -->

> **Proof:**
¬(a ^ b) = ¬a v ¬b
¬a=2k+1, ¬b=2j+1

ab
=(2k+1)*(2k+1)
=4k²+2k+2k+1
=4k²+4k+1
=2(2k²+2k)+1
=2(Integer)+1
=2t+1, closed under multiplication+1

True
---

## E4 — Proof by contradiction

**(a)** **Claim:** √3 is irrational.

> **Proof strategy:** Contradiction  
> **Proof:**  
> Assume √3 ∈ ℚ. Then √3 = p/q where q!=0, gcd(p,q)=1  
> *(follow the same structure as the √2 proof in the notes)*
(√3)^3 = (p/q)^3
3=p^3/q^3
3q^3=p^3

n=2k
3q^3=3(2k)^3=3(8k^3)

p^3=(2k)^3=8k^3

8k^3/3(8k^3)=1/3 in lowest term, q!=0, gcd(p,q)=1

<!-- 3(8k^3) = 8k^3 -->
<!-- =3k^3/j^3 -->



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
