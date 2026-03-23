# Exercise Sheet 02 - Implication, Equivalence, And Converse Errors

## Part A - Meaning

Let:

- `p`: `The sensor is calibrated.`
- `q`: `The robot can estimate position accurately.`

Write in plain English:

1. `p -> q`
if the sensor is calibrated, then the robot can estimate position accurately.
IOW, the sensor is calibrated implies the robot can estimate position accurately.

2. `q -> p`
if the robot can estimate position accurately, then the sensor is calibrated, 
IOW, the robot can estimate position accurately implies that the sensor is calibrated.

3. `!q -> !p`
if the robot cannot estimate position accurately, then the sensor is not calibrated, 
IOW, the robot cannot estimate position accurately implies that the sensor is not calibrated.

4. `p <-> q`
The sensor is calibrated and the robot can estimate position accurately; is true
also, the sensor is not calibrated, the robot cannot estimate position accurately; is true,
otherwise false,

## Part B - Truth Tables

Construct full truth tables for:

1. `p -> q`
2. `q -> p`
3. `p <-> q`

p   |   q   |   p->q    |   q->p    |   p<->q
T       T       T           T           T
T       F       F           T           F
F       T       T           F           F
F       F       T           T           T

## Part C - Identify The Error

For each pair, decide whether the second statement follows from the first.

1.

```text
If a number is divisible by 4, then it is even. = p->q
Therefore, if a number is even, it is divisible by 4. = q->p
```
False.
P = a number is divisible by 4
q = it is even
p->q(implication) != q->p(converse)

counterexample: because 2 is even but not divisible by 4


2.

```text
If a device is submerged in water, it may fail. = p->q
Therefore, if a device fails, it was submerged in water. q->q
```

p = a device is submerged in water
q = it may fail

p->q(implication) != q->p(converse)

3.

```text
If a file is deleted, it is no longer present in the folder. = p->q
Therefore, if a file is not present in the folder, it was deleted. = q->p
```
p = a file is deleted
q = it not not present in the folder

p->q(implication) != q->p(converse)

## Part D - Contrapositive

Write the contrapositive of each statement.

1. If a number is divisible by 6, then it is divisible by 3. = p->q
p = a number is divisible by 6
q = a number is divisible by 3
p->q
contrapositive = !q->!p
if a number is not divisible by 3, then it is not divisble by 6.

2. If the circuit is closed, current flows. = p->q
p = the circuit is closed
q = current flows
p->q
contrapositive = !q->!p
if the current does not flow, then the circuit is not closed.

3. If the authentication succeeds, the credentials were valid. = p->q
p = the authentication succeeds
q = the credentials were valid
p->q
contrapositive = !q->!p
if the credentials were not valid, then the authentication did not succeed.

## Part E - Equivalence Or Not

Say whether each statement describes implication or equivalence.

1. A number is even if and only if it is divisible by 2. = p<->q
It describes equivalence.
p = a number is even
q = it is divisible by 2

equivalence = p<->q
p   |   q   |   p<->q 
T       T         T
T       F         F
F       T         F
F       F         T

2. If a process crashes, the program stops executing normally. = p->q
It describes implication
p = a process crashes
q = the program stops executing normally
p   |   q   |   p->q 
T       T         T
T       F         F
F       T         T
F       F         T

3. A shape is a square only if it has four equal sides and four right angles. p->q
This is implication
p = A shape is a square
q = it has four equal sides and four right

p   |   q   |   p->q(impl) |   p<->q
T       T         T           T
T       F         F           F
F       T         T           F
F       F         T           T

## Part F - Reflection

Answer briefly:

1. Why is the converse a common trap?
because people assume propositions are always symmetric.

2. Why is the contrapositive safer than guessing from intuition?
It helps to guide thoughts with mathematical proofs

## Tutor Review

### Overall

```text
Status: pass with corrections
```

You understand the main ideas:

```text
implication      p -> q
converse         q -> p
contrapositive   !q -> !p
equivalence      p <-> q
```

### Strong Parts

1. Part B truth tables are correct.
2. Part C correctly identifies the converse error.
3. Part D contrapositives are correct.
4. Part E.1 and Part E.2 are correct.

### Corrections

#### Part A.4

Your explanation of equivalence is basically pointing at the right truth pattern, but the clean meaning is:

```text
p <-> q means both p -> q and q -> p are true.
```

For this problem:

```text
The sensor is calibrated if and only if the robot can estimate position accurately.
```

#### Part C.2

There is a notation typo in the exercise:

```text
Therefore, if a device fails, it was submerged in water. = q -> p
```

not

```text
q -> q
```

The reasoning error is still the same: it incorrectly treats the converse as if it follows from the original implication.

#### Part C.3

Your `q` description should read:

```text
q = the file is not present in the folder
```

#### Part E.3

This one is now corrected and the final answer is:

```text
p = the shape is a square
q = the shape has four equal sides and four right angles
```

```text
p only if q  means  p -> q
```

So the correct rewrite is:

```text
If a shape is a square, then it has four equal sides and four right angles.
```

It is implication, not equivalence.

### Reflection Notes

#### Why the converse is a common trap

Better version:

```text
The converse is a common trap because people often assume that if p leads to q, then q must also lead back to p, which is not generally true.
```

#### Why the contrapositive is safer

Better version:

```text
The contrapositive is safer because it is logically equivalent to the original implication, while the converse is not.
```

### Memory Summary

```text
if p then q        => p -> q
p only if q        => p -> q
p if q             => q -> p
p iff q            => p <-> q
```

```text
p -> q is false only when:
p = T and q = F
```
