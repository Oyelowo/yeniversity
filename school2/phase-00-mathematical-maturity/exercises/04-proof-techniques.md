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
Claim: There is no largest prime number

Since it's harder to prove the non-existence,
I'll prove the existence, using contradiction

¬P->⊥ = P 
Which means if we can prove that the negation 
of original claim implies falsehood, then
we prove that the original claim is TRUE.


With that:
Original Claim: There is no largest prime number

Original Assumption(P): There is no largest prime number
Original goal: True

Restated Claim: There is a largest prime number
Restate Assumption(¬P): There is largest prime number
Restated Goal: False

For all integer, there exists an integer greater
∀n ∈ ℤ, ∃m ∈ ℤ, (m>n)
∃n ∈ ℤ, ∃m ∈ ℤ, (m>n)

By definition: Integers are infinite,
n+1>n, m=n+1

which is true

P=¬(∀n ∈ ℤ, ∃m ∈ ℤ, (m>n))

¬P=¬(¬(∀n ∈ ℤ, ∃m ∈ ℤ, (m>n)))
¬P=∀n ∈ ℤ, ∃m ∈ ℤ, (m>n) is True
P^¬P

Thus:
Restated Goal negated, we got TRUE.
¬P->False is False
This originaal claim is False



**(c)** For all n ∈ ℤ: n² − n is even.

> Your answer:
Claim: For all n ∈ ℤ: n² − n is even
Assumption: n² − n is even for all integers n
Goal: For both odd and even, n² − n is even

Given A ∨ B. Given A → P. Given B → P. Conclude P.

Case 1
A -> P
A=even
Assume n is even:
i.e ∀n ∈ Z, (n is even i.e n%2==0), ∃k ∈ Z, n=2k

(2k)² - (2k) = 4k² - 2k = 2(2k² - k)

(2k² - k) 
2k² is closed under multiplication 
2k² - k is closed under subtraction
2k² - k replaced by t
2(2k² - k) = 2t = 2(integers)

Proof: 
∀n ∈ Z, n² − n is even, when n is even


Case 2
B->P
B=Odd
Assume n is odd
i.e ∀n ∈ Z, (n is odd i.e n%2==1), ∃k ∈ Z, n=2k + 1

(2k+1)² - (2k+1) 
((2k+1) * (2k+1)) - (2k+1) 
= 4k²+2k+2k+1 - 2k+1
= 4k²+2k+2k - 2k+1+1
= 4k²+2k+2
= 2(2k²+k+1)

(2k² + k + 1) 
2k² is closed under multiplication 
2k² + k + 1 is closed under addition
2k² + k + 1 replaced by t
2(2k² + k + 1) = 2t = 2(integers)

Proof: 
∀n ∈ Z, n² − n is even, when n is odd

All premises A->P, B->P, AvB
are true.

**(d)** For all x ∈ ℝ: if x² = 2, then x ∉ ℚ.

> Your answer:
Claim: For all x ∈ ℝ: if x² = 2, then x ∉ ℚ
Assumption: x² = 2
Goal: x ∉ ℚ

P->Q, where P = x² = 2, Q=x ∉ ℚ
= ¬Q→¬P  
= ¬(x ∉ ℚ) -> ¬(x² = 2)
= (x ∈ ℚ) -> ¬(x² = 2)

x=p/q, (q!=0, lowest term and gcd(p,q)=1)
x ∈ ℝ 
algebra: √2 is a real number that is irrational

√2 = p/q
2=p²/q²
2q²=p²
from earlier:
2q² is even
p² is even
gcd(p,q)!=1 because 2>1
i.e 2|gcd(p,q)

(x ∈ ℚ) = false
Premise is false
thus, claim is false

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
