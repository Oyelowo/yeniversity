# Proof Techniques — Clarifying Notes

These notes are the slower explanation that expands the hidden assumptions behind Lesson 04.
The target here is not memorization. The target is:

1. You know exactly what each symbol means.
2. You know why each line is legal.
3. You can choose a proof strategy for a new claim without guessing.
4. You can derive the proof from definitions, not from memory.

---

## 1. What proofs are actually made of

A proof is not "smart words." It is only these ingredients:

1. Definitions
2. Previously proved facts
3. Algebra / arithmetic
4. Logic

If a line in a proof does not come from one of those, it is unjustified.

So when reading a proof, keep asking:

1. What is the claim?
2. What definitions are relevant?
3. What am I allowed to assume?
4. Why does this next line follow?

If those are not clear, the proof is not clear.

---

## 2. The real source of the "made up" feeling

That feeling usually comes from skipping these two layers:

1. Why this proof strategy?
2. Where did this algebraic form come from?

For example:

- "Assume m is even, so m = 2k"
- "Assume sqrt(2) = p/q in lowest terms"

can feel arbitrary if the definitions are not made explicit first.

The missing thing is not more formulas. It is more explicit justification.

---

## 3. In these proofs, the first move is usually "unpack a definition"

Examples:

- "n is even" means: n = 2k for some integer k
- "n is odd" means: n = 2k + 1 for some integer k
- "a | b" means: b = ak for some integer k
- "x is rational" means: x = p/q for integers p, q with q != 0

These are not tricks. They are the meanings of the words.

---

## 4. The four proof techniques, stripped down

### Direct proof

Use when the assumption already gives something useful.

Form:

- Assume the hypothesis.
- Unpack definitions.
- Do algebra / logic.
- Reach the conclusion.

### Contrapositive

Use when proving P -> Q directly is awkward, but proving not Q -> not P is easier.

Why is this valid?

Because:

$$P \to Q \equiv \neg Q \to \neg P$$

So you are not changing the claim. You are proving the same claim in a more usable form.

### Contradiction

Use when the statement says something cannot exist, or when denying the claim quickly leads to impossibility.

Form:

- Assume the opposite.
- Derive an impossibility.
- Therefore the opposite was false.

### Cases

Use when every element falls into a small number of exhaustive categories.

Examples:

- every integer is even or odd
- every integer mod 3 is 0, 1, or 2

Again, not magic. Just partition the universe and prove each part.

---

## 5. Example: if m and n are even, then m + n is even

**Claim:** For all integers m, n, if m and n are even, then m + n is even.

**Why direct proof?**
Because the assumption already gives the exact structure of even numbers.

**Proof:**

Assume m and n are even.

By definition of even, there exist integers a, b such that

$$m = 2a, \quad n = 2b$$

Then

$$m+n = 2a + 2b = 2(a+b)$$

Since integers are closed under addition, a + b is an integer.

So m + n has the form 2(integer).

Therefore, by definition of even, m + n is even.

Every step came from:

- definition of even
- algebra
- closure of integers under addition

---

## 6. Example: if n^2 is even, then n is even

**Claim:** If n^2 is even, then n is even.

This is where people often feel lost.

### Why not direct proof?

In a direct proof, the legal first line is:

- Assume n^2 is even.

By definition of even, this gives:

$$n^2 = 2k$$

for some integer k.

But from there, it is hard to conclude directly that n = 2m.

### Why contrapositive helps

Instead of proving:

$$n^2 \text{ even} \to n \text{ even}$$

prove the equivalent statement:

$$n \text{ odd} \to n^2 \text{ odd}$$

Now the legal assumption is:

- Assume n is odd.

That immediately gives:

$$n = 2k+1$$

Then square it:

$$n^2 = (2k+1)^2 = 4k^2 + 4k + 1 = 2(2k^2 + 2k) + 1$$

This is of the form 2t + 1, so it is odd.

So it is not roundabout for no reason. It is choosing the version where the assumption gives useful structure.

---

## 7. What does "Since integers are closed under addition, a+b is in Z" mean?

It means:

> If a and b are integers, then a + b is also an integer.

Examples:

- 2 and 5 are integers, and 2 + 5 = 7 is an integer
- -3 and 8 are integers, and -3 + 8 = 5 is an integer

So when we write:

$$m+n = 2(a+b)$$

we still need to know that a + b is an integer before concluding the result is even.

Similar closure facts:

- integers are closed under subtraction
- integers are closed under multiplication
- integers are not closed under division

---

## 8. Where do n = 2k and n = 2k + 1 come from?

These come from the definitions.

### Even

An integer n is even iff there exists an integer k such that

$$n = 2k$$

This is either taken as the definition, or derived from divisibility:

- "n is even" means 2 divides n
- "2 divides n" means there exists integer k such that n = 2k

### Odd

An integer n is odd iff there exists an integer k such that

$$n = 2k + 1$$

So when a proof says:

> Assume n is even. Then n = 2k for some integer k.

that is not inventing a theorem. It is unpacking the meaning of "even."

---

## 9. Do we need to prove those definitions?

Not in the same way we prove theorems.

A definition assigns meaning to a word.

Examples:

- "even" means "divisible by 2"
- "rational" means "can be written as p/q with integers p, q and q != 0"
- "prime" means "integer greater than 1 with exactly two positive divisors"

Definitions are not true or false. They are conventions for language.

What we can prove is that two descriptions are equivalent.

Example:

- n is even
- 2 divides n
- there exists k in Z such that n = 2k

These line up because of the definition of divisibility.

---

## 10. Claim vs assumption vs goal

Keep these separate.

### Claim

The statement you want to establish.

Example:

> Claim: If n^2 is even, then n is even.

### Assumption

Something you are temporarily allowed to use inside the proof.

In a direct proof of P -> Q, you assume P.

So for the claim above, the assumption would be:

- Assume n^2 is even.

not:

- Assume n is even.

because that would be assuming the conclusion.

### Goal

What you are trying to derive from the assumption.

For that same proof:

- assumption: n^2 is even
- goal: show n is even

So:

- claim = overall target statement
- assumption = what the proof method lets you take as given
- goal = what remains to be shown

---

## 11. Why can't we just start with n = 2k in the n^2-even proof?

Because for the claim

$$n^2 \text{ even} \to n \text{ even}$$

the legal starting assumption in a direct proof is only:

- Assume n^2 is even.

You are not allowed to assume:

- n is even

because that is exactly the conclusion you are trying to prove.

That would be circular reasoning.

So this suggestion:

- n^2
- n = 2k
- (2k)^2

is not legal in a direct proof of this claim, because it sneaks in the conclusion.

---

## 12. Expanding "Since 2k^2 + 2k is an integer, this is of the form 2t + 1"

Suppose we have:

$$n^2 = 4k^2 + 4k + 1$$

Factor out 2 from the even part:

$$n^2 = 2(2k^2 + 2k) + 1$$

Now define:

$$t = 2k^2 + 2k$$

Why is t an integer?

- k is an integer
- integers are closed under multiplication, so k^2 is an integer
- therefore 2k^2 is an integer
- also 2k is an integer
- integers are closed under addition, so 2k^2 + 2k is an integer

So t is an integer, and therefore:

$$n^2 = 2t + 1$$

By definition, any integer of the form 2t + 1 is odd.

---

## 13. What does "lowest terms" mean?

If a rational number is written as p/q, then "lowest terms" means:

> numerator and denominator have no common factor greater than 1.

Examples:

- 6/8 is not in lowest terms because both are divisible by 2
- 3/4 is in lowest terms

So "lowest terms" means the fraction has been reduced as far as possible.

---

## 14. What is gcd?

gcd(p, q) means greatest common divisor of p and q.

It is the largest positive integer dividing both.

Examples:

- gcd(6, 8) = 2
- gcd(12, 18) = 6
- gcd(3, 4) = 1

So when we say:

$$\gcd(p, q) = 1$$

it means p and q share no common factor greater than 1.

This is another way of saying the fraction p/q is in lowest terms.

---

## 15. Why may we assume a rational number is in lowest terms?

Because every rational number can be reduced.

Example:

$$\frac{12}{18} = \frac{2}{3}$$

If numerator and denominator have a common factor, divide both by it. Keep doing that until no common factor remains.

So if we assume:

$$\sqrt{2} = \frac{p}{q}$$

we may replace that fraction by an equivalent reduced one and assume:

$$\gcd(p, q) = 1$$

This is not arbitrary. It is choosing the simplest representative of the same rational number.

Why do we want it?

Because later we will prove both p and q are even, which means they share a factor 2. That contradicts gcd(p, q) = 1.

Without the lowest-terms setup, the contradiction has nowhere to land.

---

## 16. The correct logic in the sqrt(2) proof

The logic is:

1. Assume at the start that gcd(p, q) = 1.
2. Later prove both p and q are even.
3. If both are even, then 2 divides both.
4. Therefore p and q have a common divisor 2.
5. So gcd(p, q) cannot be 1.

That contradicts the earlier assumption.

So the contradiction is:

- assumed: gcd(p, q) = 1
- proved: gcd(p, q) != 1

---

## 17. Why "both even" implies gcd is not 1

If p is even, then:

$$p = 2a$$

for some integer a, so 2 divides p.

If q is even, then:

$$q = 2b$$

for some integer b, so 2 divides q.

Thus 2 is a common divisor of both p and q.

If two numbers have common divisor 2, then their gcd is at least 2, so it cannot be 1.

---

## 18. Why proofs can feel arbitrary when compressed

Compressed proofs often silently do this:

1. identify the statement shape
2. choose a strategy
3. unpack a definition
4. rewrite in algebraic form
5. use a known lemma
6. close the argument

If those moves are not named, it looks like invention.

But the navigation is much more mechanical than it seems.

---

## 19. A usable navigation rule

When facing a proof, do this every time.

### Step A: classify the claim

Is it:

- P -> Q?
- "there exists ..."?
- "there does not exist ..."?
- "for all n ..." with a natural split into cases?

That suggests the proof method.

### Step B: write the exact assumption and exact goal

Example:

- assumption: n^2 is even
- goal: n is even

This prevents circular reasoning.

### Step C: expand only the relevant definitions

Ask:

- what does even mean?
- what does odd mean?
- what does rational mean?
- what does divides mean?

### Step D: check whether the assumption gives useful algebraic form

If yes, direct proof may work.
If no, try contrapositive or contradiction.

For:

- assumption: n^2 is even

you get:

- n^2 = 2k

That is not very helpful for extracting n.

For the contrapositive:

- assumption: n is odd

you get:

- n = 2k + 1

That is very helpful.

So contrapositive is not arbitrary. It is chosen because the assumption becomes usable.

---

## 20. Compare direct vs contrapositive on the same claim

**Claim:** If n^2 is even, then n is even.

### Direct attempt

Assume n^2 is even.

Then:

$$n^2 = 2k$$

Now what?

You need to prove:

$$n = 2m$$

But there is no easy algebraic path from n^2 = 2k to n = 2m.

So direct proof gets stuck.

### Contrapositive

Prove instead:

If n is odd, then n^2 is odd.

Assume:

$$n = 2k + 1$$

Then square it and get:

$$n^2 = 2t + 1$$

Done.

So yes, both involve n^2. But they are not the same route. The difference is which thing you are legally allowed to assume at the start.

---

## 21. Better standard for explanations going forward

For the next lessons, the standard should be:

- I know why this proof starts the way it does.
- I know why each step is legal.
- I know how I would discover it myself.

That is the bar.

---

## 22. Practical way to read the notes

Annotate each proof like this:

- Goal: what exactly am I proving?
- Strategy: direct / contrapositive / contradiction / cases
- Definitions to unpack:
- Allowed assumption(s):
- Wanted conclusion:
- Justification for each line:

If you do that, the vagueness disappears quickly.

---

## 23. One fully explicit example

**Claim:** If m and n are odd integers, then mn is odd.

**Strategy:** Direct proof

**Assumption:** m and n are odd

**Goal:** show mn is odd

**Definition used:** Odd means "of the form 2k + 1 for some integer k."

**Proof:**

Since m is odd, there exists integer a such that

$$m = 2a + 1$$

Since n is odd, there exists integer b such that

$$n = 2b + 1$$

Multiply:

$$mn = (2a + 1)(2b + 1) = 4ab + 2a + 2b + 1$$

Group the even part:

$$mn = 2(2ab + a + b) + 1$$

Let

$$t = 2ab + a + b$$

Because integers are closed under multiplication and addition, t is an integer.

So

$$mn = 2t + 1$$

Therefore mn is odd by definition. □

---

## 24. Best next step

The right move after this note is not more lecture material. It is a slower proof walkthrough with all hidden steps exposed.

A good three-problem set is:

1. If m and n are even, then m + n is even.
2. If n^2 is even, then n is even.
3. sqrt(2) is irrational.

For each one, force the format:

- Claim
- Strategy
- Assumption
- Goal
- Definitions
- Proof with each line justified