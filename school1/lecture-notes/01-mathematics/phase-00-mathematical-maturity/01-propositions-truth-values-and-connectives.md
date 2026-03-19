# Lesson 01 - Propositions, Truth Values, And Logical Connectives

## Why This Matters

Logic is the base layer of rigorous reasoning. Mathematics, proofs, program conditions, digital logic, and engineering assertions all depend on clear statements whose truth can be assessed.

If you are sloppy here, later mistakes in algebra, proofs, software, and system design become much harder to detect.

## Core Definitions

### Proposition

A proposition is a statement that is either true or false.

Examples:

1. `2 + 2 = 4` is a proposition.
2. `The Earth orbits the Sun` is a proposition.
3. `x + 1 = 3` is not a proposition by itself if `x` is unspecified.
4. `Close the door` is not a proposition because it is a command.

### Truth Value

The truth value of a proposition is either `true` or `false`.

### Logical Connectives

We combine propositions using connectives.

Let `p` and `q` be propositions.

1. Negation: `not p`, written `!p` or `~p`
2. Conjunction: `p and q`, written `p ∧ q`
3. Disjunction: `p or q`, written `p ∨ q`
4. Exclusive or is a different operation and should not be confused with the standard mathematical `or`

## Meaning Of The Connectives

### Negation

`!p` is true exactly when `p` is false.

Example:

If `p` is `It is raining`, then `!p` is `It is not raining`.

### Conjunction

`p ∧ q` is true only when both `p` and `q` are true.

Example:

If `p` is `The battery is charged` and `q` is `The switch is on`, then `p ∧ q` says both conditions hold.

### Disjunction

`p ∨ q` is true when at least one of `p` or `q` is true.

In mathematics, `or` is usually inclusive unless explicitly stated otherwise.

That means `p ∨ q` is also true when both are true.

## Truth Tables

Truth tables list every possible truth-value combination.

### Negation

| p | !p |
|---|----|
| T | F  |
| F | T  |

### Conjunction

| p | q | p ∧ q |
|---|---|-------|
| T | T | T     |
| T | F | F     |
| F | T | F     |
| F | F | F     |

### Disjunction

| p | q | p ∨ q |
|---|---|-------|
| T | T | T     |
| T | F | T     |
| F | T | T     |
| F | F | F     |

## Common Mistakes

1. Treating every sentence as a proposition. Questions and commands are not propositions.
2. Forgetting that mathematical `or` is inclusive by default.
3. Confusing a variable-containing expression with a complete proposition.
4. Using intuition instead of checking the actual truth table.

## Engineering Connection

This lesson connects directly to:

1. Boolean logic in circuits
2. Conditions in programs
3. Safety conditions in engineering systems
4. Proof writing later in mathematics

## Worked Examples

### Example 1

Let:

- `p`: `The sensor is calibrated.`
- `q`: `The robot can estimate position.`

Then:

1. `!p` means `The sensor is not calibrated.`
2. `p ∧ q` means both statements are true.
3. `p ∨ q` means at least one statement is true.

### Example 2

Statement: `7 is even or 3 < 5`

Break it into:

1. `p`: `7 is even` which is false
2. `q`: `3 < 5` which is true

Then `p ∨ q` is true.

## Memory Summary

1. A proposition is a statement with a definite truth value.
2. `!p` flips the truth value.
3. `p ∧ q` needs both true.
4. `p ∨ q` needs at least one true.
5. Inclusive `or` is the default in mathematics.

## Next Lesson

Lesson 02 will cover implication, equivalence, converse, inverse, and contrapositive. That is where many reasoning mistakes begin.
