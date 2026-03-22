# Exercises — Lesson 3: Quantifiers

Write all answers symbolically and in plain English. Show your reasoning.

---

## E1. True or False?

State whether each is TRUE or FALSE and give a brief reason (a witness or a counterexample where relevant). Domain is stated per question.

**(a)** ∀x∈ℤ (x² ≥ 0)

> Your answer:
True, 

Domain: signed integers
Universal Quantifier
Statement: For all x in zahlen(signed integers), x^2 >= 0.
in rust: signed_integers.all(|x| x**2 >= 0);

because Z for Zahlen means number and signed integer,
square of any signed integer is at lease 0, e.g 0^2, 1^2, .. n^2; all are 0 more
more.
So, Universally, for all x in domain signed integer, square of x is greater or equal
to 0.

**(b)** ∃x∈ℤ (x² < 0)
False:

Domain: signed integers
Statement: Existential Quantifier: For any x in Zahlen(signed integers), x^2 < 0.
in rust: signed_integers.any(|x| x**2 < 0);

because square of all signed integers whether < 0 or >=0, square of that is 0 or more,
not less. Therefore, there is no integer squared that is less than 0.

> Your answer:

**(c)** ∀x∈ℝ (x² = x)

> Your answer:
False.

Domain: Real Numbers
Universal Quantifier: For all x in Real numbers, x^2 = x
In rust: real_numbers.all(|x| x**2 == x);
because all real numbers above 1, when squared, are greater than the number.
So, any number above 1 would fail it.

**(d)** ∃x∈ℝ (x² = x)

> Your answer:
True.

Domain: Real Numbers
Existential Quantifier: For any x in Real numbers, x^2 = x
In rust: real_numbers.any(|x| x**2 == x)
Because 0 and 1 are real numbers, specifically Natural number and integer and real number,
and square of 0 and 1, and also -1(zahlen, integer), all equal themselves.

**(e)** ∀x∈ℤ ∃y∈ℤ (x + y = 0)

> Your answer:
True.

Domain: x as Zahlen, integer with Universal Quantifier.
Domain: y as Zahlen, integer with Existential Quantifier.

For all x in Integers:
    For any y in Integers:
        x + y == 0:
In rust:
integers_x.all(|x| {
    integers_y.any(|y| x + y == 0)
});
True, because 0 + 0 = 0; -1 + 1 = 0, -2 + 2 = 0, ... etc
Therefor, for all integers, you can find a witness that can balance it to make zero.
since: x + y∈ℤ = 0; x = 0 - y∈ℤ, and we can a witness to fulfill that balance which
basically would be negation of x which would also be an integer.

**(f)** ∃y∈ℤ ∀x∈ℤ (x + y = 0)

> Your answer:
False.

Domain: Existential y as integer
Domain: Uniersal x as integer
For any y in integer:
    For all x in integer:
        x + y == 0:
In rust:
integers_y.any(|x| {
    integers_x.all(|y| x + y == 0)
});

False, because, for some every integer, not all integers x can satisfy x + y == 0;
e.g if you have integer 1, only -1 can satisfy that condition, all others can not.
and that single witness already invalidates the the entire statement.

---

## E2. Negate These Statements

Write the negation symbolically **and** in plain English. Do not simplify further — just push the ¬ inward correctly.

**(a)** ∀x∈ℝ (x² ≥ 0) 
For all x in Real numbers if (x² ≥ 0)
in rust: real_numbers.all(|x| x² ≥ 0);

> Symbolic negation:
¬(∃x∈ℝ (x² < 0))
Not (For any x in Real numbers if (x² < 0))
In rust: !real_numbers.any(|x| x² < 0)

> English:
Negation of for any x in real numbers that its square is less than 0.

**(b)** ∃x∈ℤ (x is odd)
For any x in integers if (x % 2 == 1)
in rust: integers.any(|x| x % 2 == 1);

> Symbolic negation:
¬(∀x∈ℝ (x is even))
Not (For all x in integers if (x is even))
In rust: !integers.all(|x| x % 2 == 0)
> English:
Not all integers where all are even

**(c)** ∀x∈ℤ (x > 0 → x² > 0)
For all x in integers if (!(x > 0) or (x² > 0))
in rust: integers.all(|x| !(x > 0) or (x² > 0));

> Symbolic negation:
¬(∃x∈ℤ ¬(x > 0 → x² > 0))
¬(∃x∈ℤ ¬(¬(x > 0) v (x² > 0)))
Negation of for any x in integers if (¬(x > 0 → x² > 0))
In rust: !integers.any(|x| !(!(x>0) || (x**2 > 0)))

> English:
> Hint: the predicate here is an implication. Its negation is what we proved in Lesson 1 E3(b).
Not for any integers if the not we take not of the integer greater than 0 or we take the 
square of the integer greater than 0.

**(d)** ∀x∈ℝ ∃y∈ℝ (x·y = 1)
For  all x in real numbers:
    For  any y in real numbers:
        if x*y == 1:
In rust:
real_numbers_x.all(|x| real_numbers_y.any(|y| x * y == 1));

> Symbolic negation:
¬(∀x∈ℝ ∃y∈ℝ (x·y = 1))
    = ∃x∈ℝ ¬(∃y∈ℝ (x·y = 1))      [¬∀ = ∃¬]
    = ∃x∈ℝ ∀y∈ℝ ¬(x·y = 1)        [¬∃ = ∀¬]
    = ∃x∈ℝ ∀y∈ℝ (x·y != 1)        [negate the predicate]

In rust:
real_numbers_x.any(|x| real_numbers_y.all(|y| x * y != 1));

> English:
> Is the original statement true or false? What is the counterexample?
The original statement is False, because, 0 multiplied by anything would be 0.
so, x == 0 is the counter example, anything multipled by 0 would never give 1,
so, that invalidates the original statement.


**(e)** ∃x∈ℕ ∀y∈ℕ (x ≤ y)
For any x in natural numbers:
    For all y in natural numbers:
        if x <= y:

> Symbolic negation:
¬(∃x∈ℕ ∀y∈ℕ (x ≤ y))
= ∀x∈ℕ ¬(∀y∈ℕ (x ≤ y))       [¬∃x∈ℕ = ∀x∈ℕ ¬]
= ∀x∈ℕ ∃y∈ℕ ¬(x ≤ y)         [¬∀y∈ℕ = ∃y∈ℕ ¬]
= ∀x∈ℕ ∃y∈ℕ (x > y)          [negate the predicate]

> English:
> Is the original true or false? Why?
Technically, the original is true, since 0 is less than or equal any other natural number.
if you have 0, it would be technically less than or equal to the minimum of any other natural
number which the minimum would be 0 anyways.

---

## E3. Translate English → Symbolic

Define your domain and predicates first.

**(a)** "Every real number has an additive inverse."

> Domain: ∀x∈ℕ ∃y∈ℕ
> Let P(x,y) = x = (1 / x + y)
> Formula: ∀x∈ℕ ∃y∈ℕ (x = (1/x+y))

**(b)** "There is a largest integer."

> Domain: ∀x∈ℤ
> Formula: max(∃x∈ℤ) = true
> Formula: ∀x∈ℤ ∃y∈ℤ ( y >= 0)
> Is this true or false?
technically, true within the constrain of the environment e.g operating system.
but theoretically infinite.

**(c)** "Not every function is continuous."

> Domain: ¬∀x∈f
> Let C(f) = true
> Formula (using ¬ and ∀ or ∃): 
¬(∀x∈f C(f) = true)
In rust: !funcs.all(|cb| C(cb) == true);

(∃x∈f C(f) = false)
In rust: funcs.any(|cb| C(cb) == false);

**(d)** "For any two real numbers, there is a real number strictly between them."

> Domain: ∃{x,y}⊂ℝ ∃z∈ℝ 
> Let P(x,y,z) = (x < z < y)
> Formula: ∀{x,y}⊂ℝ ∃z∈ℝ (x < z < y)
> What property of ℝ does this describe?
R is irrational

---

## E4. Translate Symbolic → English

Write the most natural English sentence for each. Then state if it is true (domain = ℝ unless noted).

**(a)** ∀x ∀y (x + y = y + x)

> English: 
For all x:
    For all y:
        if ( x + y = y + x)
> True or false?
True

**(b)** ∃x∈ℝ (x² + 1 = 0)

> English:
For any x in real numbers:
    if (x**2 + 1 == 0)
> True or false (over ℝ)?
False, since every real numbers squared would be positive.

**(c)** ∀x∈ℝ ∃y∈ℝ (y² = x)

> English:
For all x in real numbers:
    For any y in real numbers:
        if y**2 == x:
> True or false?
False, since y squared will always be positive which would not be equal to somce real x numbers.
> Hint: think about negative x.

**(d)** ∃x∈ℤ ∀y∈ℤ (x · y = y)

> English:
For any x in integers(Zehlen):
    For all y in integers(Zehlen):
        if (x * y == y)
> True or false? Give the witness.
True, the witness is 1, 1 times any y integer, would be equal to y integer

---

## E5. (Harder) Negate a Complex Statement

Negate the following statement and simplify ¬P fully (flip quantifiers, negate the predicate):

**"Every function that is differentiable is continuous."**

First write it symbolically (let D(f) = "f is differentiable", C(f) = "f is continuous"), then negate it step by step.

> Symbolic form:
D(f)->C(f)
> Negation step 1 (push ¬ past ∀):
¬(D(f)->C(f))
> Negation step 2 (negate the implication: ¬(D(f)→C(f))):
> Final symbolic form:
¬(¬D(f) v C(f))
> English reading of the negation:
negation of negation of f is differentiable or f is continuous
---

## Rust Challenge

In `src/quantifiers.rs`, implement and test:

```rust
pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // todo!() // hint: .iter().all(|&x| predicate(x))
    domain.iter().all(|&x| predicate(x))
}

pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // todo!() // hint: .iter().any(|&x| predicate(x))
    domain.iter().any(|&x| predicate(x))
}

pub fn negation_duality(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
    // ¬(∀x P(x)) ≡ ∃x ¬P(x)  — verify both sides agree
    // todo!()
    domain.iter().any(|&x| !predicate(x))
}
```

Tests to write:
1. `for_all(&[1,2,3,4], |x| x*x >= 0)` → `true`
2. `for_all(&[1,2,3,4], |x| x > 3)` → `false` (counterexample: 1)
3. `there_exists(&[1,2,3,4], |x| x > 3)` → `true` (witness: 4)
4. `there_exists(&[-2,-1,0,1], |x| x*x < 0)` → `false`
5. `negation_duality(&[1,2,3,4], |x| x > 2)` → `true`
