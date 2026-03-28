# Lesson 04 — Proof Techniques

**Phase:** 00 — Mathematical Maturity  
**Ref:** Velleman *How to Prove It*, Ch. 3 §3.1–3.6  
**Depends on:** Lessons 01 (truth tables), 02 (logical equivalence, ¬¬P≡P, P→Q≡¬Q→¬P), 03 (quantifiers)

---

## Teaching philosophy for this lesson

Every proof technique is a *named, reusable pattern of logical reasoning*. None are magic.
Each one reduces to something you can verify with a truth table or derive directly from a
definition. This lesson shows the derivation before using each technique. If something is
stated without justification here, treat it as missing — ask for the derivation.

---

## 0. Prerequisite: Definitions we will use

Before writing any proof you must know the precise definitions. Imprecise definitions produce
imprecise proofs.

**Definition (even):** n ∈ ℤ is even iff ∃k ∈ ℤ, n = 2k.  
**Definition (odd):** n ∈ ℤ is odd iff ∃k ∈ ℤ, n = 2k + 1.

**Division Algorithm (stated, not proved here):** For any n ∈ ℤ and d ∈ ℤ with d > 0,
there exist unique q, r ∈ ℤ with 0 ≤ r < d such that n = dq + r.

*For d = 2, the only possible remainders are 0 and 1:*
- r = 0 → n = 2q (even)
- r = 1 → n = 2q + 1 (odd)

Therefore every integer is even or odd — never both, never neither. This is not assumed; it
follows from the Division Algorithm.

**Definition (rational):** r ∈ ℝ is rational (r ∈ ℚ) iff ∃p, q ∈ ℤ, q ≠ 0, r = p/q.  
Every rational can be written in *lowest terms*: reduce by gcd(p,q). This uses Euclidean
division, which guarantees gcd(p,q) exists for any integers p,q (not both 0).

---

## 1. What exactly is a proof?

A **proof** is a finite sequence of statements where each statement is justified by exactly one of:

1. It is a hypothesis (something we were explicitly given or assuming temporarily).
2. It follows from a definition.
3. It follows from a previously established fact (lemma, theorem, earlier step).
4. It follows by an algebraic rule (e.g., distributive law; equals can have the same operation applied to both sides).
5. It follows by a named logical rule (modus ponens, disjunction elimination, etc.).

"Clearly", "obviously", "it is easy to see" — these are never justifications. They mean a
step was skipped. In your own proofs: if you want to write "clearly", write the step instead.

---

## 2. Direct Proof

**What it proves:** P → Q.

**The underlying rule — Modus Ponens:**

> Given P, and given P → Q, conclude Q.

Why is this valid? Check the truth table: the only row where P=T and P→Q=T is Q=T.

| P | Q | P→Q |
|---|---|-----|
| T | T | T   |  ← only row with P=T and P→Q=T: forces Q=T
| T | F | F   |  ← P→Q=F, so this premise fails
| F | T | T   |  ← P=F, so this premise fails
| F | F | T   |  ← P=F, so this premise fails

A direct proof is a chain of modus ponens applications:

```
Assume P.               [hypothesis]
P → A:  [derive A]      [A is now known true]
A → B:  [derive B]      [B is now known true]
...
... → Q: [derive Q]     [Q is now known true]
□
```

At each step, the justification must come from the list in section 1.

### Example 2.1 — Sum of two odd integers is even

**Claim:** ∀m, n ∈ ℤ: (m odd ∧ n odd) → (m + n even)

**Proof (direct):**

Assume m is odd and n is odd. [hypothesis]  
By definition of odd: ∃j ∈ ℤ, m = 2j + 1; and ∃k ∈ ℤ, n = 2k + 1.  
m + n = (2j + 1) + (2k + 1)     [substitute definitions]  
      = 2j + 2k + 2              [associativity + commutativity of integer addition]  
      = 2(j + k + 1)             [distributive law: 2j + 2k + 2 = 2(j+k+1)]  
j + k + 1 ∈ ℤ                   [ℤ is closed under addition]  
So m + n = 2 × (integer). By definition of even, m + n is even. □

### Rust witness

```rust
#[cfg(test)]
mod tests {
    // Direct computational verification of Example 2.1 over a finite range
    #[test]
    fn sum_of_two_odds_is_even() {
        for m in -20i64..=20 {
            for n in -20i64..=20 {
                if m % 2 != 0 && n % 2 != 0 {
                    assert_eq!((m + n) % 2, 0, "m={m}, n={n}");
                }
            }
        }
    }
}
```

---

## 3. Proof by Contrapositive

**What it proves:** P → Q (via the logically equivalent form ¬Q → ¬P).

**The underlying equivalence:** (P → Q) ≡ (¬Q → ¬P).

This is not assumed — verify it by truth table:

| P | Q | ¬P | ¬Q | P → Q | ¬Q → ¬P |
|---|---|----|----|-------|---------|
| T | T | F  | F  | **T** | **T**   |
| T | F | F  | T  | **F** | **F**   |
| F | T | T  | F  | **T** | **T**   |
| F | F | T  | T  | **T** | **T**   |

Columns 5 and 6 are identical in every row. They are the same statement in every possible
world. Proving ¬Q → ¬P *is* proving P → Q — not a shortcut, the same claim.

**When is contrapositive easier?**  
When ¬Q provides a strong, concrete assumption while P is hard to work forward from.

**Structure:**
```
[We want P → Q. Equivalently, we prove ¬Q → ¬P.]
Assume ¬Q.
[derive ¬P step by step, with justifications]
Therefore ¬Q → ¬P, hence P → Q. □
```

### Example 3.1 — If n² is odd then n is odd

**Claim:** ∀n ∈ ℤ: n² odd → n odd.

**Why direct is awkward:** Assume n² = 2k+1. We need to say something about n itself.
But extracting n from n² requires more machinery than we have.

**Contrapositive:** Prove n even → n² even (i.e., ¬(n odd) → ¬(n² odd)).

Assume n is even. [hypothesis — this is ¬Q]  
∃k ∈ ℤ, n = 2k. [by definition of even]  
n² = (2k)²       [substitute]  
   = 2k × 2k     [definition of squaring: x² = x × x]  
   = 4k²          [arithmetic: 2×2=4, k×k=k²]  
   = 2(2k²)       [distributive law: 4k² = 2 × 2k²]  
2k² ∈ ℤ          [ℤ closed under multiplication]  
So n² = 2 × (integer). By definition, n² is even. [this is ¬P]

We proved n even → n² even (¬Q → ¬P).  
By the truth table equivalence above, this proves n² odd → n odd. □

---

## 4. Proof by Contradiction

**What it proves:** Any statement P.

**The underlying equivalence:** ¬P → ⊥  is logically equivalent to P.

Where ⊥ denotes the constant False (the always-false statement).

Verification by truth table (⊥ is always F):

| P | ¬P | ¬P → ⊥ (= ¬P → F) | = P? |
|---|----|--------------------|------|
| T | F  | F → F = **T**      | T ✓  |
| F | T  | T → F = **F**      | F ✓  |

The column "¬P → ⊥" matches P exactly in both rows. **Proving ¬P → ⊥ is proving P.**

Alternatively, in words: ¬P → ⊥ means "if ¬P were true, something impossible would
follow." Since ⊥ is never true, ¬P cannot be true. Since P ∨ ¬P (law of excluded middle —
every proposition is T or F, no third option), and ¬P is false, P must be true.

A "contradiction" in practice is finding Q ∧ ¬Q for some statement Q, since:

| Q | ¬Q | Q ∧ ¬Q |
|---|----|----|
| T | F  | F  |
| F | T  | F  |

Q ∧ ¬Q is always False — it *is* ⊥. Deriving Q ∧ ¬Q completes the contradiction.

In actual proofs, contradictions often appear in several equivalent forms:

- deriving both Q and ¬Q
- deriving an impossible arithmetic statement such as 1 = 0
- deriving that a prime divides 1
- violating an earlier assumption, e.g. assuming gcd(p, q) = 1 and later proving gcd(p, q) ≠ 1

All of these count as contradictions because they amount to deriving ⊥ (False).

**Structure:**
```
[We want to prove P.]
Assume ¬P.          [hypothesis — the negation of what we want]
...steps...
...derive Q...
...derive ¬Q...     [we now have Q ∧ ¬Q — this is ⊥]
Contradiction. ¬P is false. Therefore P. □
```

### Example 4.1 — √2 is irrational ★ (Phase 00 gate proof)

**Claim:** √2 ∉ ℚ.

Equivalently: there is no p/q ∈ ℚ such that (p/q)² = 2.

**Lemma** (used mid-proof — proved here first):

> **L:** ∀p ∈ ℤ: p² even → p even.
>
> Proof by contrapositive (prove p odd → p² odd):  
> Assume p = 2k+1 for some k ∈ ℤ. [definition of odd]  
> p² = (2k+1)² = 4k²+4k+1 [expand: (a+b)² = a²+2ab+b², with a=2k, b=1]  
>    = 2(2k²+2k)+1.  [factor out 2 from first two terms]  
> 2k²+2k ∈ ℤ, so p² = 2(integer)+1 is odd by definition. □

**Main proof (contradiction):**

Assume ¬P: √2 ∈ ℚ.  
Then ∃p, q ∈ ℤ, q ≠ 0, √2 = p/q.  
Choose this representation in lowest terms: we can always do this by dividing numerator
and denominator by gcd(p,q) (Euclidean algorithm guarantees gcd exists).  
So **gcd(p, q) = 1**. [annotate this — it is the fact that will be contradicted]

Square both sides (equality is preserved: x = y iff x² = y² when both sides are ≥ 0,  
and √2 > 0, p/q > 0 here):  
2 = p²/q².

Multiply both sides by q² (q ≠ 0, so this is valid — multiplying both sides of an  
equation by the same nonzero number preserves equality):  
p² = 2q².

p² = 2q² means p² is even (equals 2 times the integer q²).  
By Lemma L: p is even.  
∃m ∈ ℤ, p = 2m. [by definition of even]

Substitute p = 2m into p² = 2q²:  
(2m)² = 2q²    [replace p with 2m]  
4m²   = 2q²    [(2m)² = 4m²]  
2m²   = q².    [divide both sides by 2]

2m² = q² means q² is even (equals 2 times the integer m²).  
By Lemma L: q is even.  
∃n ∈ ℤ, q = 2n.

Now: p = 2m means 2 | p; q = 2n means 2 | q.  
2 divides both p and q, so 2 is a common divisor of p and q.  
Since gcd(p,q) is the *greatest* common divisor, and 2 divides both, 2 | gcd(p,q).  
So gcd(p,q) ≥ 2.

But we said gcd(p,q) = 1.  
gcd(p,q) cannot be both 1 and ≥ 2 simultaneously.  

Let Q = "gcd(p,q) = 1" and ¬Q = "gcd(p,q) ≠ 1". We have both Q and ¬Q. **Contradiction.** □

Therefore √2 ∉ ℚ.

**Why gcd=1 is load-bearing:**  
Without the lowest-terms reduction, you'd derive "p and q are both even" but that doesn't
contradict anything — you can have p=4, q=2 with both even. The contradiction only fires
because we already reduced, meaning there's no common factor left. Finding one (namely, 2)
is impossible — that's the absurdity.

---

## 5. Proof by Cases

**What it proves:** ∀x ∈ D, P(x) — when D can be partitioned into exhaustive parts.

**The underlying rule — Disjunction Elimination:**

> Given A ∨ B. Given A → P. Given B → P. Conclude P.

Truth table — verify the rule is valid (check that when all three premises hold, P holds):

| A | B | P | A→P | B→P | A∨B | All premises true? | P? |
|---|---|---|-----|-----|-----|-------------------|----|
| T | T | T | T   | T   | T   | YES               | T ✓|
| T | F | T | T   | T   | T   | YES               | T ✓|
| F | T | T | T   | T   | T   | YES               | T ✓|
| T | T | F | F   | F   | T   | NO (A→P fails)    | — |
| T | F | F | F   | T   | T   | NO (A→P fails)    | — |
| F | T | F | T   | F   | T   | NO (B→P fails)    | — |
| F | F | T | T   | T   | F   | NO (A∨B fails)    | — |
| F | F | F | T   | T   | F   | NO (A∨B fails)    | — |

Every row with all premises true has P=T. The rule is valid.

**The obligation:** You must prove A ∨ B — that the cases cover everything.
If there is a gap (some x not in any case), the premise A∨B may be false for that x, and
the rule doesn't apply. Without the exhaustiveness argument, your proof has a hole.

For even/odd case splits: every integer is even or odd (proved from the Division Algorithm in
section 0). That is the exhaustiveness argument to cite.

**Structure:**
```
Every x in D satisfies A(x) ∨ B(x) because [justification].

Case 1 (A(x)): Assume A(x). ... Therefore P(x). ✓
Case 2 (B(x)): Assume B(x). ... Therefore P(x). ✓

P(x) holds in all cases; cases are exhaustive. Therefore ∀x∈D P(x). □
```

### Example 5.1 — n(n+1) is always even

**Claim:** ∀n ∈ ℤ, n(n+1) is even.

Every integer n is even or odd (Division Algorithm, d=2 — proved in section 0).

*Case 1: n is even.*  
∃k ∈ ℤ, n = 2k.  
n(n+1) = 2k(n+1)   [substitute n = 2k]  
       = 2 × [k(n+1)].  
k(n+1) ∈ ℤ (ℤ closed under multiplication).  
So n(n+1) = 2 × (integer). By definition, n(n+1) is even. ✓

*Case 2: n is odd.*  
∃k ∈ ℤ, n = 2k+1.  
n+1 = (2k+1)+1 = 2k+2 = 2(k+1).  [algebra, then distributive law]  
k+1 ∈ ℤ, so n+1 is even.  
n(n+1) = n × 2(k+1) = 2 × [n(k+1)].  
n(k+1) ∈ ℤ. So n(n+1) = 2 × (integer). By definition, n(n+1) is even. ✓

Both cases yield n(n+1) even; cases are exhaustive (all integers covered). □

---

## 6. Choosing a Strategy

| What you want to prove | Try | Why |
|------------------------|-----|-----|
| P → Q, and P gives useful facts | Direct | Modus ponens chain from P toward Q |
| P → Q, and ¬Q is more concrete | Contrapositive | Same statement, easier direction |
| P, especially "X is irrational" or "no X exists" | Contradiction | Deny P, derive Q∧¬Q |
| ∀x P(x) and domain splits naturally | Cases | Disjunction elimination |
| ∃x P(x) | Exhibition | Construct the specific witness |

These choices are not style — they are logical identities. Contrapositive works because
P→Q and ¬Q→¬P are the *same sentence* (identical truth values). Contradiction works
because ¬P→⊥ *is* P. When you pick a strategy you're choosing which direction to approach
the same wall.

### Implications: which assumptions are legal?

For a claim of the form

$$
P \to Q
$$

the proof method determines what you are allowed to assume:

| Method | Legal assumption | Why |
|--------|------------------|-----|
| Direct | P | You are proving P → Q forward |
| Contrapositive | ¬Q | Because P → Q ≡ ¬Q → ¬P |
| Contradiction | P ∧ ¬Q | Because ¬(P → Q) ≡ P ∧ ¬Q |

That last line is the key point many people miss. In a contradiction proof of an implication,
you are **not** assuming the conclusion. You are assuming the **negation of the whole implication**:

$$
\neg(P \to Q)
= \neg(\neg P \lor Q)
= P \land \neg Q
$$

So contradiction on an implication legally gives you both pieces at once:
- the hypothesis-side assumption P
- the negation of the conclusion ¬Q

This is why a contradiction proof of an implication is not circular reasoning.

---

## 7. Common Mistakes

| Mistake | Why it fails |
|---------|--------------|
| Proving Q → P instead of P → Q | The converse is not equivalent unless P↔Q |
| Assuming the conclusion (circular) | You've made Q a premise — you haven't proved Q without Q |
| One example proving ∀ | Verifying P(3) proves ∃x P(x), not ∀x P(x) |
| Non-exhaustive case split | A∨B must hold for every domain element; gaps leave the rule misapplied |
| Omitting "gcd=1" in √2 proof | Without lowest terms, "both even" is not a contradiction |
| Writing "clearly" anywhere | Means a step was skipped; write the step |

---

## 8. Proof Templates

### Direct
```
Claim: ∀[vars]: [hypothesis] → [conclusion]
Proof (direct):
  Assume [hypothesis].
  [Step 1: justification]
  ...
  Therefore [conclusion]. □
```

### Contrapositive
```
Claim: ∀[vars]: P → Q
Proof (contrapositive — we prove ¬Q → ¬P, which is equivalent by truth table):
  Assume ¬Q.
  [Step 1: justification]
  ...
  Therefore ¬P.
  Hence P → Q. □
```

### Contradiction
```
Claim: P
Proof (contradiction):
  Assume ¬P.
  [Step 1: justification]
  ...
  [derive Q]
  ...
  [derive ¬Q]
  Q and ¬Q cannot both hold — this is ⊥ (False).
  Therefore P. □
```

### Cases
```
Claim: ∀x∈D, P(x)
Proof (cases):
  Every x in D satisfies [A(x) ∨ B(x)] because [justification].
  Case 1 [A(x)]: Assume A(x). ... Therefore P(x). ✓
  Case 2 [B(x)]: Assume B(x). ... Therefore P(x). ✓
  P(x) holds in all cases. □
```

---

## 9. Curry–Howard Correspondence (Proofs as Programs)

There is a precise correspondence between logic and typed programming:

| Logic | Rust |
|-------|------|
| Proposition P | Type `P` |
| Proof of P | A value of type `P` |
| P → Q | `fn(p: P) -> Q` |
| P ∧ Q | `(P, Q)` |
| P ∨ Q | `enum Either<P,Q> { Left(P), Right(Q) }` |
| ⊥ (False) | `!` (never / Infallible) |
| Proof by contradiction | `fn(not_p: NotP) -> !` |

This isn't analogy — it's an isomorphism discovered by Haskell Curry and William Howard.
The type checker is a proof checker; every well-typed program is a proof.

---

## 10. Summary

| Technique | Assume | Derive | Justified by |
|-----------|--------|--------|-------------|
| Direct | P | Q | Modus ponens (truth table) |
| Contrapositive | ¬Q | ¬P | P→Q ≡ ¬Q→¬P (truth table column identity) |
| Contradiction | ¬P | ⊥ | ¬P→⊥ ≡ P (truth table; law of excluded middle) |
| Cases | A∨B exhaustive | P in each case | Disjunction elimination (truth table) |

**Gate check:** Without notes, reproduce the proof that √2 ∉ ℚ with every step justified.
Any step written as "clearly" is incomplete.

---

*Next: Lesson 05 — Mathematical Induction*
