# Lesson 03 - Sets, Elements, Subsets, And Operations

## Core Idea

A set is a collection of distinct objects.

You must separate these three ideas cleanly:

```text
set
element
subset
```

## Definitions

### Set

```text
A set is a collection of distinct objects.
```

Examples:

```text
A = {1, 2, 3}
B = {red, blue}
```

### Element

If an object is inside a set, it is an element of that set.

Notation:

```text
2 ∈ A      means 2 is an element of A
5 ∉ A      means 5 is not an element of A
```

### Subset

`A` is a subset of `B` if every element of `A` is also an element of `B`.

Notation:

```text
A ⊆ B
```

If `A ⊆ B` and `A ≠ B`, then `A` is a proper subset of `B`:

```text
A ⊂ B
```

## Important Distinction

```text
2 ∈ {1, 2, 3}          true
{2} ⊆ {1, 2, 3}        true
2 ⊆ {1, 2, 3}          not the right kind of statement
{2} ∈ {1, 2, 3}        false
```

## Set Operations

### Union

Elements in either set:

```text
A ∪ B
```

### Intersection

Elements common to both sets:

```text
A ∩ B
```

### Difference

Elements in `A` but not in `B`:

```text
A \ B
```

## Diagram

```text
        +------- A -------+
       /                   \
      /    +--- A ∩ B ---+  \
     /    /               \  \
    /    /                 \  \
   /    +-------------------+  \
   \                           /
    \        +------ B ------+/ 
     \      /                / 
      \    /                /  
       \  /                /   
        ++----------------+    
```

## Common Mistakes

1. confusing `∈` with `⊆`
2. treating an element as if it were a set
3. forgetting that order does not matter in a set
4. repeating the same element as if it changes the set

## Engineering Connection

Sets show up everywhere:

```text
allowed states
sensor groups
test cases
input domains
error classes
```

## Memory Summary

```text
x ∈ A     x is an element of A
A ⊆ B     every element of A is in B
A ∪ B     union
A ∩ B     intersection
A \ B     elements in A not in B
```
