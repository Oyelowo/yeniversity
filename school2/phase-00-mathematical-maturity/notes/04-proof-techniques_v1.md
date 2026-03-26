# Lesson 4 — Proof Techniques

> **Pre-reading**: Make sure Lessons 1–3 are solid.  
> You already know implication (P→Q), logical equivalence, and quantifiers.  
> Proofs are just *formally written deductions* using those tools.

---

## 1. What is a Proof?

A **proof** is a finite sequence of statements where:
- Each statement is either an assumption, a definition, or follows from earlier statements by a valid logical rule.
- The last statement is the **conclusion** you wanted to establish.

Proofs are the currency of mathematics. Without them a statement is just a conjecture, no matter how obviously true it looks.

**Analogy (Rust):** A proof is like a type-checked program. The type system only accepts programs that are *provably* safe at every step. Similarly, a proof referee only accepts arguments where *every* inference is justified.

---

## 2. Four Fundamental Strategies

| Strategy | When to use | Structure |
|---|---|---|
| Direct proof | P→Q feels "forward" | Assume P, derive Q step by step |
| Contrapositive | ¬Q→¬P is easier | Assume ¬Q, derive ¬P |
| Contradiction | Denying the conclusion breaks logic | Assume ¬P, derive F (False) |
| Proof by cases | Domain splits into exhaustive cases | Prove P in each case separately |

All four prove the *same kinds of things*; the art is picking the one that leads to a clean argument.

---

## 3. Direct Proof

**To prove P → Q:**  
Assume P is true. Show Q must follow.

**Structure:**
```
Assume P.
[chain of justified steps]
Therefore Q. □
```

The symbol □ (or QED, or ∎) signals the proof is finished.

### Example 3.1 — Even × anything is even

**Claim:** For all integers m, n: if m is even, then mn is even.

**Proof (direct):**  
Assume m is even.  
By definition of even: ∃k ∈ ℤ such that m = 2k.  
Then mn = (2k)n = 2(kn).  
Since kn ∈ ℤ, mn is even by definition. □

**In Rust:**
```rust
// "even × anything is even" — executable witness
fn product_even_when_first_even(m: i64, n: i64) -> bool {
    if m % 2 == 0 {
        (m * n) % 2 == 0  // must be true — Rust confirms it
    } else {
        true  // claim only covers the case where m is even
    }
}

#[test]
fn test_even_product() {
    for m in (-10..=10).step_by(2) {
        for n in -10..=10 {
            assert!(product_even_when_first_even(m, n));
        }
    }
}
```

---

### Example 3.2 — Sum of two odds is even

**Claim:** For all m, n ∈ ℤ: if m and n are both odd, then m + n is even.

**Proof (direct):**  
Assume m and n are odd.  
By definition: m = 2j + 1 and n = 2k + 1 for some j, k ∈ ℤ.  
Then m + n = (2j + 1) + (2k + 1) = 2j + 2k + 2 = 2(j + k + 1).  
Since j + k + 1 ∈ ℤ, m + n is even. □

---

## 4. Proof by Contrapositive

Recall from Lesson 2: **(P → Q)  ≡  (¬Q → ¬P)**.  
These are logically identical. So if ¬Q → ¬P is easier to prove, prove that instead.

**When is contrapositive easier?**  
- When Q looks easy to negate into a strong assumption (¬Q gives you more to work with).
- When P is hard to assume but ¬P is concrete.

**Structure:**
```
Assume ¬Q.
[chain of justified steps]
Therefore ¬P. □

(This proves ¬Q → ¬P, hence P → Q.)
```

### Example 4.1 — Odd square implies odd number

**Claim:** For all n ∈ ℤ: if n² is odd, then n is odd.

**Direct attempt:** Assume n² is odd. We need to show n is odd. But it's hard to extract information about n just from knowing n² is odd.

**Contrapositive:** Prove: if n is *even*, then n² is *even*.  
Assume n is even: n = 2k for some k ∈ ℤ.  
Then n² = (2k)² = 4k² = 2(2k²).  
Since 2k² ∈ ℤ, n² is even. □

(We proved ¬(n odd) → ¬(n² odd), which is equivalent to the original claim.)

**In Rust — contrapositive framing:**
```rust
fn odd_square_implies_odd(n: i64) -> bool {
    // Original: n² odd → n odd
    // We verify the contrapositive: n even → n² even
    if n % 2 == 0 {
        (n * n) % 2 == 0
    } else {
        true
    }
}
```

---

## 5. Proof by Contradiction

**Goal:** Prove statement P is true.

**Structure:**
```
Assume ¬P.           ← negate what you want to prove
[derive a contradiction — something of the form Q ∧ ¬Q]
Therefore ¬P is false, so P must be true. □
```

This works because if ¬P leads to a logical impossibility (anything false), then ¬P cannot be true, so P is true.

**Analogy (logic):** Contradiction proofs are the `panic!` of mathematics. If assuming the program is safe forces an impossible state, the program cannot be safe. Contradiction — the assumption was wrong.

---

### Example 5.1 — √2 is irrational ★ (Phase 00 gate proof)

**Claim:** √2 ∉ ℚ (√2 is not a rational number).

**Recall:** ℚ = {p/q | p, q ∈ ℤ, q ≠ 0}. We can always write p/q in *lowest terms* (fully reduced, gcd(p,q) = 1).

**Proof (contradiction):**  
Assume √2 ∈ ℚ. Then √2 = p/q for some p, q ∈ ℤ with q ≠ 0 and **gcd(p, q) = 1** (lowest terms).

Squaring both sides: 2 = p²/q², so p² = 2q².

Since 2 | p², p² is even. By Example 4.1: p is even.  
Write p = 2k for some k ∈ ℤ.

Substitute: (2k)² = 2q² → 4k² = 2q² → 2k² = q².

Since 2 | q², q² is even. By Example 4.1: q is even.

Now both p and q are even → 2 | gcd(p, q) — but we assumed gcd(p, q) = 1. **Contradiction.** □

Therefore √2 ∉ ℚ.

**Why this proof matters:**  
- It established that ℝ ≠ ℚ — there exist numbers that are not rational.
- The ancient Greeks found this deeply disturbing (this was allegedly kept secret by the Pythagoreans).
- You are expected to reproduce this proof cold — it is a Phase 00 gate requirement.

---

### Example 5.2 — There are infinitely many primes

**Claim:** The set of primes is infinite.

**Proof (contradiction):**  
Assume there are finitely many primes: p₁, p₂, …, pₙ.  
Let N = (p₁ × p₂ × … × pₙ) + 1.

N > 1, so N has at least one prime divisor, call it p.  
But p must be one of p₁, …, pₙ (our complete list).  
Yet p | N and p | (p₁ × … × pₙ), so p | (N − p₁…pₙ) = 1.  
No prime divides 1. **Contradiction.** □

Therefore primes are infinite.

---

## 6. Proof by Cases

When the domain splits into finitely many exhaustive, mutually exclusive cases and the result holds in each case separately, you prove it in each case.

**Structure:**
```
Every element of the domain is in Case 1 or Case 2 or … or Case k (exhaustive).
Case 1: [proof]
Case 2: [proof]
…
Case k: [proof]
In every case P holds. □
```

### Example 6.1 — n² + n is always even

**Claim:** For all n ∈ ℤ, n² + n is even.

**Proof (by cases on parity of n):**

*Case 1: n is even.*  
n = 2k, so n² + n = 4k² + 2k = 2(2k² + k). Even. ✓

*Case 2: n is odd.*  
n = 2k + 1, so n² + n = (2k+1)² + (2k+1) = 4k² + 4k + 1 + 2k + 1 = 4k² + 6k + 2 = 2(2k² + 3k + 1). Even. ✓

In both cases n² + n is even. □

**Shortcut:** n² + n = n(n+1) — consecutive integers, so one of them must be even. But the case proof above works without that insight.

---

## 7. Choosing a Strategy

No rigid rule, but here is a useful decision tree:

```
Does the conclusion say something EXISTS?
    → Direct proof or contradiction (exhibit the witness or deny its nonexistence)

Is the conclusion of the form P → Q?
    → Try direct first
    → If Q is hard, try contrapositive (¬Q → ¬P)
    → If both are hard, try contradiction (assume P ∧ ¬Q → derive F)

Does the domain split naturally? (even/odd, positive/negative/zero, …)
    → Proof by cases

Is it a universal negative ("there is NO …", "X is irrational")?
    → Contradiction is often cleanest
```

---

## 8. What a Rigorous Proof Looks Like (Template)

```
Claim: [state the claim precisely, with quantifiers]

Proof strategy: [state which technique and why]

Proof:
[Assumption line or case split]
[Step 1 — justified by: definition / axiom / previous result / algebra]
[Step 2 — ...]
[...]
Therefore [conclusion]. □
```

Every step needs a **justification** — "definition of even", "by Example 4.1", "algebra", etc. In early proofs, write every justification explicitly. Experienced writers skip trivial ones.

---

## 9. Common Mistakes

| Mistake | Why it fails |
|---|---|
| Proving Q → P instead of P → Q | You reversed the direction |
| Assuming what you're trying to prove (circular reasoning) | Your assumption already contains Q |
| Saying "let n be arbitrary, clearly …" with no work | "Clearly" is never a justification |
| Using a specific example to prove a universal claim | One witness proves ∃; it does NOT prove ∀ |
| Forgetting to cover all cases | Proof by cases is only valid if cases are exhaustive |
| Dropping the "lowest terms" assumption in √2 proof | Without gcd=1 the contradiction doesn't fire |

---

## 10. Rust Connection — Proofs as Types (Curry–Howard)

There is a deep correspondence between proofs and programs:

| Logic | Rust/Programming |
|---|---|
| Proposition P | Type `P` |
| Proof of P | Value of type `P` |
| P → Q | Function `fn(P) -> Q` |
| P ∧ Q | Struct `(P, Q)` |
| P ∨ Q | Enum `Either<P, Q>` |
| ⊥ (False) | `!` (never type / `Infallible`) |
| Proof by contradiction | Showing `fn(not_p: NotP) -> Infallible` |

This is the **Curry–Howard correspondence**. Every well-typed Rust program *is* a proof in a certain logic. Type-checking *is* proof-checking.

You don't need to use this formally yet — but it explains why types and proofs feel so similar.

---

## 11. Summary

| Technique | Proof structure | Key logical law |
|---|---|---|
| Direct | Assume P, derive Q | Modus ponens |
| Contrapositive | Assume ¬Q, derive ¬P | P→Q ≡ ¬Q→¬P |
| Contradiction | Assume ¬P, derive F | ¬¬P ≡ P |
| Cases | Partition → prove in each | Law of excluded middle |

**Memorise this:** For √2 ∉ ℚ — assume it is p/q in lowest terms, square to get p²=2q², conclude p even, write p=2k, get 2k²=q², conclude q even, contradict gcd=1.

---

*Next: Lesson 5 — Mathematical Induction*
