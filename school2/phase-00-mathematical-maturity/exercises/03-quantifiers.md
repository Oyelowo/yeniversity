# Exercises — Lesson 3: Quantifiers

Write all answers symbolically and in plain English. Show your reasoning.

---

## E1. True or False?

State whether each is TRUE or FALSE and give a brief reason (a witness or a counterexample where relevant). Domain is stated per question.

**(a)** ∀x∈ℤ (x² ≥ 0)

> **Your answer:**
> True,
>
> Domain: signed integers
> Universal Quantifier
> Statement: For all x in zahlen(signed integers), x^2 >= 0.
> in rust: signed_integers.all(|x| x**2 >= 0);
>
> because Z for Zahlen means number and signed integer,
> square of any signed integer is at least 0, e.g 0^2, 1^2, .. n^2; all are 0 or more.
> So, Universally, for all x in domain signed integer, square of x is greater or equal to 0.

> **Tutor feedback:**
> ✅ Excellent — correct classification, correct domain name (ℤ/Zahlen), correct Rust analogy,
> correct reasoning. One small point: don't forget negative integers too: (−3)² = 9 ≥ 0. ✓

---

**(b)** ∃x∈ℤ (x² < 0)

> **Your answer:**
> False:
>
> Domain: signed integers
> Statement: Existential Quantifier: For any x in Zahlen(signed integers), x^2 < 0.
> in rust: signed_integers.any(|x| x**2 < 0);
>
> because square of all signed integers whether < 0 or >=0, square of that is 0 or more,
> not less. Therefore, there is no integer squared that is less than 0.

> **Tutor feedback:**
> ✅ Perfect — correct quantifier, correct domain, correct Rust translation, correct reasoning.
> Squaring any integer (positive, negative, or zero) always gives ≥ 0. No witness exists.

---

**(c)** ∀x∈ℝ (x² = x)

> **Your answer:**
> False.
>
> Domain: Real Numbers
> Universal Quantifier: For all x in Real numbers, x^2 = x
> In rust: real_numbers.all(|x| x**2 == x);
> because all real numbers above 1, when squared, are greater than the number.
> So, any number above 1 would fail it.

> **Tutor feedback:**
> ✅ FALSE. Correct. Counterexample: x = 2, 2² = 4 ≠ 2. Good instinct to pick a number > 1.
> Note: x² = x ↔ x(x−1) = 0, so the only solutions are x = 0 and x = 1 — not *all* reals.

---

**(d)** ∃x∈ℝ (x² = x)

> **Your answer:**
> True.
>
> Domain: Real Numbers
> Existential Quantifier: For any x in Real numbers, x^2 = x
> In rust: real_numbers.any(|x| x**2 == x)
> Because 0 and 1 are real numbers, specifically Natural number and integer and real number,
> and square of 0 and 1, and also -1(zahlen, integer), all equal themselves.

> **Tutor feedback:**
> ✅ TRUE. Correct — witnesses 0 and 1 work. One correction: (−1)² = 1 ≠ −1, so −1 is NOT a witness.
> Only x = 0 and x = 1 satisfy x² = x.

---

**(e)** ∀x∈ℤ ∃y∈ℤ (x + y = 0)

> **Your answer:**
> True.
>
> Domain: x as Zahlen, integer with Universal Quantifier.
> Domain: y as Zahlen, integer with Existential Quantifier.
>
> For all x in Integers:
>     For any y in Integers:
>         x + y == 0:
> In rust:
> integers_x.all(|x| {
>     integers_y.any(|y| x + y == 0)
> });
> True, because 0 + 0 = 0; -1 + 1 = 0, -2 + 2 = 0, ... etc
> Therefore, for all integers, you can find a witness that can balance it to make zero.
> since: x + y∈ℤ = 0; x = 0 - y∈ℤ, and we can find a witness to fulfill that balance which
> basically would be negation of x which would also be an integer.

> **Tutor feedback:**
> ✅ Perfect — and the Rust nested-iterator translation is exactly right.
> The witness is always y = −x, which is an integer whenever x is. Clean reasoning.

---

**(f)** ∃y∈ℤ ∀x∈ℤ (x + y = 0)

> **Your answer:**
> False.
>
> Domain: Existential y as integer
> Domain: Universal x as integer
> For any y in integer:
>     For all x in integer:
>         x + y == 0:
> In rust:
> integers_y.any(|x| {
>     integers_x.all(|y| x + y == 0)
> });
>
> False, because, for some every integer, not all integers x can satisfy x + y == 0;
> e.g if you have integer 1, only -1 can satisfy that condition, all others can not.
> and that single witness already invalidates the entire statement.

> **Tutor feedback:**
> ✅ FALSE. Correct. The reasoning is right — no single fixed y can satisfy x + y = 0 for EVERY x.
> Minor note in your Rust: the variable names got swapped (`|x|` in the outer, `|y|` in the inner)
> but the logic you described in English is correct.
> Correct Rust: `integers_y.any(|y| integers_x.all(|x| x + y == 0))`

---

## E2. Negate These Statements

Write the negation symbolically **and** in plain English. Do not simplify further — just push the ¬ inward correctly.

**(a)** ∀x∈ℝ (x² ≥ 0)
Attempt 2:
> **Your answer:**
Original stated
For all x in Real numbers:
    if (x² ≥ 0)
> in rust: 
real_numbers.all(|x| x² ≥ 0);
>
> Symbolic negation: 
¬(∀x∈ℝ (x² ≥ 0))
= ∃x∈ℝ ¬(x² >= 0)     [¬∀x∈ℝ = ∃x∈ℝ¬]
= ∃x∈ℝ (x² < 0)     [Flip the predicate]

> For any x in Real numbers:
    if (x² < 0)

> In rust: 
real_numbers.any(|x| x² < 0)
>
> English: There exists a real number whose square is less than 0
False, because all real numbers squared become positive or 0

> **Tutor feedback (Attempt 2):**
> ✅ PERFECT. All three steps correctly labelled and the result is right:
> - ¬∀ flipped to ∃¬, predicate ≥0 negated to <0 ✅
> - Rust: `real_numbers.any(|x| x² < 0)` — no outer `!` ✅
> - You noted the negation is FALSE over ℝ while still being the correct logical form ✅
> That last point is exactly the right level of understanding: "correct negation" and "true statement" are separate questions.

> **Tutor feedback (Attempt 1):**
> ❌ CORRECTED. You wrote ¬(∃x∈ℝ (x²<0)) — that wraps the result in an extra ¬.
> Negation pushes the ¬ *inward*; it doesn't wrap the whole flipped statement again.
> Correct: **∃x∈ℝ (x² < 0)**
> The ¬ flips ∀ to ∃ and negates the predicate (≥0 becomes <0). Done. No outer ¬.
>
> Rust: `real_numbers.any(|x| x*x < 0)` — no `!` on the outside.
>
> English: "There exists a real number whose square is negative."
> (This is FALSE over ℝ — but that's the correct negation regardless.)

---

**(b)** ∃x∈ℤ (x is odd)

> **Your answer:**
> For any x in integers if (x % 2 == 1)
> in rust: integers.any(|x| x % 2 == 1);
>
> Symbolic negation: 
¬(∃x∈ℤ (x is odd))
= ∀x∈ℤ ¬(x is odd)   [¬∃x∈ℤ = ∀x∈ℤ¬]
= ∀x∈ℤ (x is even)   [flip the predicate]

> For all x in integers:
    if (x is even)

> In rust: 
integers.all(|x| x % 2 == 0)
>
> English: All integers where all are even

> **Tutor feedback (Attempt 2):**
> ✅ PERFECT. ¬∃ flipped to ∀¬, predicate negated from "odd" to "even". Final form ∀x∈ℤ (x is even) ✅
> Rust: `integers.all(|x| x % 2 == 0)` ✅ — and this is actually more correct than the old
> feedback's suggestion of `x % 2 != 1`: in Rust, `-3 % 2 == -1` (not `1`), so
> `x % 2 != 1` would incorrectly classify `-3` as even. `x % 2 == 0` correctly identifies
> all even integers, including negatives. Your answer was better than the original correction. ✓

> **Tutor feedback (Attempt 1):**
> ❌ Two errors:
> 1. Domain changed from ℤ to ℝ — the domain must stay the same.
> 2. Same outer-¬ mistake: you wrote ¬(∀x ...) keeping the ¬ outside. Push it inward.
> Correct: **∀x∈ℤ (x is even)**
> Rust: `integers.all(|x| x % 2 != 1)` — no `!` on the outside.
>
> English: "Every integer is even." (This is FALSE — but it is the correct negation.)

---

**(c)** ∀x∈ℤ (x > 0 → x² > 0)

Attempt 2
> **Your answer:**
> For all x in integers if (!(x > 0) or (x² > 0))
> in rust: integers.all(|x| !(x > 0) or (x² > 0));
>
¬(P->Q) 
= ¬(¬P v Q)    [Implication law]
= P ^ ¬Q    [De Morgan 2: ¬(A v B) = ¬A ^ ¬B], where A = ¬P, B = Q

> Symbolic negation: 
¬(∀x∈ℤ (x > 0 → x² > 0))
= ∃x∈ℤ (x > 0 → x² > 0)        [¬∀x∈ℤ = ∃x∈ℤ¬]
= ∃x∈ℤ ¬(x > 0 → x² > 0)       [Negate predicate inside]
= ∃x∈ℤ (x > 0 ^ ¬(x² > 0))     [P->Q = P ^ ¬Q proven from above]
= ∃x∈ℤ (x > 0 ^ x² <= 0)       [Apply negation to rhs]

> Negation of for any x in integers if (¬(x > 0 → x² > 0))
> In rust: integers.any(|x| !(!(x>0) || (x**2 > 0)))
>
> English: 
For any x in integers(Zehlan):
    if (x > 0 ^ x² <= 0):

There exists an integer that is positive and its square is negative or 0.
False: There is no intger greater than 0, yet with square less than or equal 0.
Only 0 squared is less than or equal zero but 0 is not greater than zero, so changes nothing.

In rust:
integers.any(|x| (x > 0) && (x**2 <= 0))

> **Tutor feedback (Attempt 2):**
> ✅ Correct final result. One notation slip:
> Line 1 of your chain reads `∃x∈ℤ (x > 0 → x² > 0)` — the ¬ is missing from the predicate.
> Your bracket label `[¬∀x∈ℤ = ∃x∈ℤ¬]` shows you knew ¬ moves inside; it's a typographical slip.
> The chain should be two steps only:
>   ∃x∈ℤ ¬(x > 0 → x² > 0)    [¬∀x∈ℤ = ∃x∈ℤ¬]
>   ∃x∈ℤ (x > 0 ∧ x² ≤ 0)     [¬(P→Q) ≡ P∧¬Q]
> The separate ¬(P→Q) derivation shown at the top is correct and well-justified. ✅
> Final form, Rust, and English all correct. ✅

----
> **Tutor feedback (Attempt 1):**
> ❌ CORRECTED. You wrote ¬(∃x∈ℤ ¬(...)) — double-negating the whole thing gives back the original.
> Push the ¬ all the way inward in one pass:
>
> Step 1 — flip ∀ to ∃, negate the predicate:
>   ∃x∈ℤ ¬(x > 0 → x² > 0)
>
> Step 2 — negate the implication (Lesson 1: ¬(P→Q) ≡ P∧¬Q):
>   **∃x∈ℤ (x > 0 ∧ x² ≤ 0)**
>
> Rust: `integers.any(|x| x > 0 && x*x <= 0)`
>
> English: "There exists an integer that is positive but whose square is not positive."
> (This is FALSE over ℤ — but it is the correct negation.)

---

**(d)** ∀x∈ℝ ∃y∈ℝ (x·y = 1)

> **Your answer:**
> For all x in real numbers:
>     For any y in real numbers:
>         if x*y == 1:
> In rust:
> real_numbers_x.all(|x| real_numbers_y.any(|y| x * y == 1));
>
> Symbolic negation:
> ¬(∀x∈ℝ ∃y∈ℝ (x·y = 1))
>     = ∃x∈ℝ ¬(∃y∈ℝ (x·y = 1))      [¬∀ = ∃¬]
>     = ∃x∈ℝ ∀y∈ℝ ¬(x·y = 1)        [¬∃ = ∀¬]
>     = ∃x∈ℝ ∀y∈ℝ (x·y != 1)        [negate the predicate]
>
> In rust:
> real_numbers_x.any(|x| real_numbers_y.all(|y| x * y != 1));
>
> The original statement is False, because, 0 multiplied by anything would be 0.
> so, x == 0 is the counter example, anything multiplied by 0 would never give 1.

> **Tutor feedback:**
> ✅ PERFECT — every step is correctly labelled and the Rust translation is correct.
> Final form: ∃x∈ℝ ∀y∈ℝ (x·y ≠ 1)
>
> Original TRUE or FALSE? ✅ FALSE. Correct — x = 0 is the counterexample. 0·y = 0 ≠ 1 always.
> The negation ∃x∈ℝ ∀y∈ℝ (x·y≠1) is therefore TRUE (witnessed by x=0).

---

**(e)** ∃x∈ℕ ∀y∈ℕ (x ≤ y)

> **Your answer:**
> For any x in natural numbers:
>     For all y in natural numbers:
>         if x <= y:
>
> Symbolic negation:
> ¬(∃x∈ℕ ∀y∈ℕ (x ≤ y))
> = ∀x∈ℕ ¬(∀y∈ℕ (x ≤ y))       [¬∃x∈ℕ = ∀x∈ℕ ¬]
> = ∀x∈ℕ ∃y∈ℕ ¬(x ≤ y)         [¬∀y∈ℕ = ∃y∈ℕ ¬]
> = ∀x∈ℕ ∃y∈ℕ (x > y)          [negate the predicate]
>
> Technically, the original is true, since 0 is less than or equal any other natural number.
> if you have 0, it would be technically less than or equal to the minimum of any other natural
> number which the minimum would be 0 anyways.

> **Tutor feedback:**
> ✅ PERFECT — both quantifier flips and the predicate negation are correct.
> Final form: ∀x∈ℕ ∃y∈ℕ (x > y)
>
> Original TRUE or FALSE? ✅ TRUE. Correct — witness: x = 0, and 0 ≤ y for every y∈ℕ.
> The negation ∀x∈ℕ ∃y∈ℕ (x>y) is therefore FALSE.

---

## E3. Translate English → Symbolic

Define your domain and predicates first.

**(a)** "Every real number has an additive inverse."

> **Your answer:**
Attempt 2
> Domain: ∀x∈ℝ ∃y∈ℝ
> Let P(x,y) = x + y = 0 
> Formula: ∀x∈ℝ ∃y∈ℝ (x+y = 0)
For all x in real numbers, there exists a real number y with an additive inverse to x
Witness: y = -x, there is always a real inverse number, so statement is TRUE.

> **Tutor feedback (Attempt 2):**
> ✅ PERFECT. All three errors from attempt 1 corrected:
> - Domain is ℝ, not ℕ ✅
> - Predicate is x + y = 0 (additive inverse, not multiplicative) ✅
> - Formula: ∀x∈ℝ ∃y∈ℝ (x+y = 0) ✅
> - Witness y = −x identified, truth value TRUE ✅

> **Tutor feedback (Attempt 1):**
> ❌ CORRECTED. Three issues:
> 1. Domain should be ℝ not ℕ (we're talking about real numbers, not naturals).
> 2. An *additive* inverse of x is y such that x + y = 0, not a multiplicative formula.
> 3. The formula x = 1/(x+y) is unrelated to additive inverses.
>
> Correct:
> Domain: x∈ℝ, y∈ℝ
> Let P(x,y) = (x + y = 0)
> Formula: **∀x∈ℝ ∃y∈ℝ (x + y = 0)**
>
> Witness: y = −x, which is always real. Statement is TRUE.

---

**(b)** "There is a largest integer."

> **Your answer:**
> Domain: ∀x∈ℤ ∃x∈ℤ
> Formula: 
∀x∈ℤ ∃y∈ℤ ( x <= y)
∃x∈ℤ ∀y∈ℤ ( x >= y)
> Is this true or false?
> FALSE, because integers are mathematically infinite. so, there is always x + 1 for every x.
Different, from computers/programming with implementation finiteness.

> **Tutor feedback (Attempt 2):**
> ✅ Correct formula and correct truth value (FALSE). Two small notes:
> 1. The domain line `∀x∈ℤ ∃x∈ℤ` uses `x` twice — the second variable should be `y`.
>    Write: variables x, y ∈ ℤ (both range over ℤ).
> 2. You wrote two formulas. The correct translation is `∃x∈ℤ ∀y∈ℤ (x ≥ y)` ✅.
>    The other one, `∀x∈ℤ ∃y∈ℤ (x ≤ y)`, is a *different* statement ("every integer has
>    something at least as large" — which is TRUE). Good observation, just label it separately
>    so it doesn't get confused with the answer to the problem.
> Reasoning (x+1 is always larger, ℤ is mathematically infinite) is correct. ✅

> **Tutor feedback (Attempt 1):**
> ❌ CORRECTED. The OS/int-max confusion is a programming instinct — mathematically ℤ is infinite.
>
> The statement says: there is some integer that is ≥ every other integer.
> Correct formula: **∃x∈ℤ ∀y∈ℤ (x ≥ y)**
>
> This is **FALSE**. For any integer x you claim is the largest, x+1 is a larger integer.
> There is no maximum element in ℤ.
>
> In programming, `i64::MAX` exists because memory is finite — that is an implementation limit,
> not a mathematical truth. In math, ℤ is unbounded in both directions.

---

**(c)** "Not every function is continuous."

> **Your answer:**
> Domain: ¬∀x∈f
> Let C(f) = true
> Formula (using ¬ and ∀ or ∃):
> ¬(∀x∈f C(f) = true)
> In rust: !funcs.all(|cb| C(cb) == true);
>
> (∃x∈f C(f) = false)
> In rust: funcs.any(|cb| C(cb) == false);

> **Tutor feedback:**
> ✅ Logic correct — you recognised this pushes to an existential. Two refinements:
> 1. The domain is the set of all functions, written 𝓕. The variable ranging over it is f.
> 2. C(f) is already a proposition — write it as a predicate, not `C(f) = true`.
>
> Clean form:
> Domain: f ∈ 𝓕
> Let C(f) = "f is continuous"
> Formula: **∃f∈𝓕 ¬C(f)**
>
> Rust: `functions.any(|f| !is_continuous(f))`
>
> TRUE — e.g. f(x) = 1/x is not continuous at x = 0.

---

**(d)** "For any two real numbers, there is a real number strictly between them."
Attempt 2
> **Your answer:**
> Domain: x∈ℝ y∈ℝ z∈ℝ
> Let P(x,y,z) = (x < z < y)
> Formula: ∀x∈ℝ ∀y∈ℝ (x<y -> ∃z∈ℝ (x < z < y))
The guard x <-y is needed because if x >= y, there is no "between"
> What property of ℝ does this describe?
> Between any two real numbers, there is always another real number.

> **Tutor feedback (Attempt 2):**
> ✅ PERFECT. All errors from attempt 1 corrected:
> - Domain uses separate ∀x∈ℝ ∀y∈ℝ quantifiers ✅
> - Formula: ∀x∈ℝ ∀y∈ℝ (x < y → ∃z∈ℝ (x < z < y)) ✅
> - Guard x < y noted with correct reason ✅
> - Property: "between any two real numbers there is always another" is the exact definition
>   of the **density of ℝ**. That's the technical name. ✅
> Minor: "x <-y" appears to be a typo for "x < y". ✓

> **Tutor feedback (Attempt 1):**
> ✅ Formula almost right — just fix the domain notation. {x,y}⊂ℝ is set notation, not a quantifier.
> Use two separate universal quantifiers:
>
> Domain: x∈ℝ, y∈ℝ, z∈ℝ
> Formula: **∀x∈ℝ ∀y∈ℝ (x < y → ∃z∈ℝ (x < z < y))**
>
> (The guard x<y is needed — if x≥y there is no "between".)
>
> What property? ❌ CORRECTED: This is the **density of ℝ** (and of ℚ). Between any two real
> numbers there is always another. "ℝ is irrational" is not meaningful — irrationality is a
> property of individual numbers (like √2), not of ℝ as a set.

---

## E4. Translate Symbolic → English

Write the most natural English sentence for each. Then state if it is true (domain = ℝ unless noted).

**(a)** ∀x ∀y (x + y = y + x)

> **Your answer:**
> For all x:
>     For all y:
>         if ( x + y = y + x)
> True

> **Tutor feedback:**
> ✅ TRUE. This is the **commutativity of addition**. More natural English: "Addition is commutative."

---

**(b)** ∃x∈ℝ (x² + 1 = 0)

> **Your answer:**
> For any x in real numbers:
>     if (x**2 + 1 == 0)
> False, since every real numbers squared would be positive.

> **Tutor feedback:**
> ✅ FALSE over ℝ. x² ≥ 0 for all real x, so x²+1 ≥ 1 > 0. No solution in ℝ.
> (Over ℂ it is TRUE — witness: x = i, since i²+1 = −1+1 = 0.)

---

**(c)** ∀x∈ℝ ∃y∈ℝ (y² = x)

> **Your answer:**
> For all x in real numbers:
>     For any y in real numbers:
>         if y**2 == x:
> False, since y squared will always be positive which would not be equal to some real x numbers.

> **Tutor feedback:**
> ✅ FALSE. Correct reasoning — counterexample: x = −1. No real y satisfies y² = −1.
> (Over ℂ: TRUE — y = i works for x = −1.)

---

**(d)** ∃x∈ℤ ∀y∈ℤ (x · y = y)

> **Your answer:**
> For any x in integers(Zahlen):
>     For all y in integers(Zahlen):
>         if (x * y == y)
> True, the witness is 1, 1 times any y integer, would be equal to y integer

> **Tutor feedback:**
> ✅ TRUE. Witness: x = 1. 1·y = y for all y. This is the **multiplicative identity**.

---

## E5. (Harder) Negate a Complex Statement

Negate the following statement and simplify ¬P fully (flip quantifiers, negate the predicate):

**"Every function that is differentiable is continuous."**

First write it symbolically (let D(f) = "f is differentiable", C(f) = "f is continuous"), then negate it step by step.

Attempt 2:
> **Your answer:**
> Symbolic form: ∀f∈𝓕 (D(f)->C(f))
>
> Negation step 1 (push ¬ past ∀): ¬(D(f)->C(f))
¬(∀f∈𝓕 (D(f)->C(f)))
= ∃f∈𝓕 ¬(D(f)->C(f))      [¬∀f∈𝓕 = ∃f∈𝓕¬]
= ∃f∈𝓕 ¬(¬D(f) v C(f))    [Implication law]
= ∃f∈𝓕 D(f) ^ ¬C(f)       [De Morgan 2: ¬(A v B) = ¬A ^ ¬B, where A=¬D(f), B=C(f)]
>
> Negation step 2 (negate the implication):
> ∃f∈𝓕 D(f) ^ ¬C(f)  
>
> English reading of the negation:
> There exists a function that is differentiable but not continuous.

> **Tutor feedback (Attempt 2):**
> ✅ PERFECT. Every step is correct:
> - Symbolic form now correctly includes ∀f∈𝓕 ✅
> - ¬∀f∈𝓕 → ∃f∈𝓕¬ ✅
> - Implication law: ¬(D(f)→C(f)) = ¬(¬D(f)∨C(f)) ✅
> - De Morgan 2 applied with A=¬D(f), B=C(f) correctly identified ✅
> - Final form: ∃f∈𝓕 (D(f) ∧ ¬C(f)) ✅
> - English reading correct ✅
> This is textbook-clean. The key fix from attempt 1 (adding ∀f∈𝓕 to the symbolic form) is
> solid, and you clearly understand WHY it's needed — "every function" quantifies over all functions.
> This is FALSE because every differentiable function is continuous (a theorem from calculus). 

> **Tutor feedback:**
> ❌ CORRECTED. You forgot the ∀ quantifier — "Every function" requires ∀f∈𝓕.
>
> Correct symbolic form: **∀f∈𝓕 (D(f) → C(f))**
>
> Negation step 1 — push ¬ past ∀ (flip to ∃, negate the predicate):
>   ∃f∈𝓕 ¬(D(f) → C(f))
>
> Negation step 2 — negate the implication (¬(P→Q) ≡ P∧¬Q):
>   **∃f∈𝓕 (D(f) ∧ ¬C(f))**
>
> English: "There exists a function that is differentiable but not continuous."
>
> This is FALSE — every differentiable function is continuous (a theorem from calculus).
> But that is the correct logical negation of the original. Notice how the negation shapes
> the counterexample: to disprove "all differentiable → continuous" you'd need one
> differentiable function that fails to be continuous.

---

## Rust Challenge

> **Your answer:**
> ```rust
> pub fn for_all(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
>     domain.iter().all(|&x| predicate(x))
> }
>
> pub fn there_exists(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
>     domain.iter().any(|&x| predicate(x))
> }
>
> pub fn negation_duality(domain: &[i32], predicate: fn(i32) -> bool) -> bool {
>     let lhs = !domain.iter().all(|&x| predicate(x));
>     let rhs = domain.iter().any(|&x| !predicate(x));
>     lhs == rhs
> }
> ```

> **Tutor feedback:**
> ✅ `for_all` and `there_exists` — perfect.
> ✅ `negation_duality` — correct: compute both sides and assert they agree.
> Note: the version in `src/quantifiers.rs` uses `impl Fn` instead of `fn` pointer — that was a
> compile fix because closures that capture variables cannot coerce to bare `fn` pointers.
> Your logic here is exactly right.
