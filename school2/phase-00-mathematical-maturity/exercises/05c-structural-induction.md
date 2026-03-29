# Exercises 05C — Structural Induction for ASTs

> **Instructions:** These are bridge exercises from ordinary proof techniques into compiler-style proofs.

---

## Setup

Let expressions be defined by:

```text
e ::= Int(n)
    | Add(e1, e2)
```

Define:

```text
size(Int(n)) = 1
size(Add(e1, e2)) = 1 + size(e1) + size(e2)
```

---

## E1

Prove by structural induction that for every expression e,

$$size(e) \ge 1.$$

> **Proof strategy:** structural induction on e
>
> **Proof:**
There are two constructor cases.

Case 1: $$e = Int(n)$$.

Then

$$size(e) = 1,$$

so $$size(e) \ge 1$$.

Case 2: $$e = Add(e_1, e_2)$$.

Inductive hypotheses:

- $$size(e_1) \ge 1$$
- $$size(e_2) \ge 1$$

Now

$$size(e) = 1 + size(e_1) + size(e_2).$$

Since both sizes on the right are at least 1, certainly

$$size(e) \ge 1.$$

Therefore the claim holds for both constructors, so for every expression e,

$$size(e) \ge 1. \quad \square$$

## E2

Define the number of `Add` nodes by:

```text
adds(Int(n)) = 0
adds(Add(e1, e2)) = 1 + adds(e1) + adds(e2)
```

Prove by structural induction that for every expression e,

$$adds(e) < size(e).$$

## E3

Why is structural induction the natural proof method for recursive optimizers over ASTs? Answer in two or three sentences.

**Worked answer:**

Because the optimizer itself is usually defined by recursion on the syntax tree, the proof can follow exactly the same constructor cases. Each recursive call corresponds to an inductive hypothesis for the child expressions, so the proof mirrors the implementation.
