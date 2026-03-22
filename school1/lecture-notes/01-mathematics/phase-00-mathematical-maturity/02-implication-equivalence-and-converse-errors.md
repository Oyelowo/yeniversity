# Lesson 02 - Implication, Equivalence, And Converse Errors

## Core Idea

This lesson is about three things:

1. implication: `p -> q`
2. equivalence: `p <-> q`
3. reasoning mistakes caused by confusing a statement with its converse

These are foundational for proofs, programming conditions, engineering requirements, and debugging logic.

## Implication

`p -> q` means:

```text
If p is true, then q must be true.
```

It does **not** mean:

```text
If q is true, then p must be true.
```

### Truth Table

```text
p   q   p -> q
T   T     T
T   F     F
F   T     T
F   F     T
```

## Why False Implies Anything Is True Here

If `p` is false, the implication does not get violated.

Example:

```text
If a number is divisible by 4, then it is even.
```

If the number is `6`, the first part is false, so the statement is still considered true.

## Converse

The converse of `p -> q` is:

```text
q -> p
```

These are **not** automatically equivalent.

### Example

```text
If a number is divisible by 4, then it is even.     true
If a number is even, then it is divisible by 4.     false
```

This is one of the most common reasoning errors in mathematics.

## Contrapositive

The contrapositive of `p -> q` is:

```text
!q -> !p
```

This is always logically equivalent to the original implication.

### Diagram

```text
p -> q
^    ^
|    |
equivalent to
|    |
!q -> !p
```

## Inverse

The inverse of `p -> q` is:

```text
!p -> !q
```

Do not confuse this with the contrapositive.

## Equivalence

`p <-> q` means both directions hold:

```text
(p -> q) and (q -> p)
```

### Truth Table

```text
p   q   p <-> q
T   T      T
T   F      F
F   T      F
F   F      T
```

## Engineering Connection

```text
Requirement: if battery is low, system enters safe mode.
```

That means:

```text
low_battery -> safe_mode
```

It does **not** mean:

```text
safe_mode -> low_battery
```

Safe mode may also happen because of overheating, sensor failure, or manual override.

## Common Mistakes

1. treating `p -> q` as if it also means `q -> p`
2. confusing converse with contrapositive
3. forgetting that `p <-> q` means both directions
4. feeling that `F -> F` should be false when it is actually true

## Memory Summary

```text
p -> q      one-way condition
q -> p      converse
!q -> !p    contrapositive, equivalent to p -> q
!p -> !q    inverse
p <-> q     both directions hold
```

## Next Step

Complete the exercise sheet for this lesson.
