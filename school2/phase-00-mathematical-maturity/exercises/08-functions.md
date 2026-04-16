# Exercises 08 — Functions

> **Instructions:** Work through each exercise without referring to the notes first.
> After finishing, check your answers against the worked solutions.
> For finite examples you can verify against `src/functions.rs`.

---

## Tier 1 — Recognise what is and isn't a function

**E1.** Let A = {1, 2, 3} and B = {a, b, c}. Decide whether each relation is a function from A to B.

1. $$R_1 = \{(1,a),(2,b),(3,c)\}$$
2. $$R_2 = \{(1,a),(1,b),(2,c),(3,a)\}$$
3. $$R_3 = \{(1,a),(2,b)\}$$
4. $$R_4 = \{(1,c),(2,c),(3,c)\}$$

**Worked answers:**

1. Yes — every element of A appears exactly once as a first component.
2. No — element 1 maps to both a and b (not functional).
3. No — element 3 has no image (not total).
4. Yes — every element of A maps to c. (Multiple elements may share an output; that is fine.)

---

## Tier 2 — Prove or disprove injectivity / surjectivity

**E2.** For each function $$f : \mathbb{Z} \to \mathbb{Z}$$, classify as injective, surjective, both, or neither.

(a) $$f(n) = n + 7$$  
(b) $$f(n) = 2n$$  
(c) $$f(n) = n^2$$  
(d) $$f(n) = n^3$$  

**Worked answers:**

**(a) $$f(n) = n + 7$$ — bijective.**

*Injective:* Suppose $$f(n_1) = f(n_2)$$. Then $$n_1 + 7 = n_2 + 7$$, so $$n_1 = n_2$$. ✓  
*Surjective:* Given $$m \in \mathbb{Z}$$, take $$n = m - 7$$. Then $$f(n) = m$$. ✓

**(b) $$f(n) = 2n$$ — injective, not surjective.**

*Injective:* $$2n_1 = 2n_2 \Rightarrow n_1 = n_2$$. ✓  
*Not surjective:* 3 is odd, so $$2n = 3$$ has no integer solution. ✗

**(c) $$f(n) = n^2$$ — neither.**

*Not injective:* $$f(2) = f(-2) = 4$$ but $$2 \ne -2$$. ✗  
*Not surjective:* $$-1$$ is negative; no integer squares to −1. ✗

**(d) $$f(n) = n^3$$ — injective, not surjective.**

*Injective:* If $$n_1^3 = n_2^3$$, then $$n_1 = n_2$$ because cubing is strictly increasing on $$\mathbb{Z}$$. ✓

*Not surjective:* 2 is not a perfect cube, so there is no integer $$n$$ with $$n^3 = 2$$. ✗

**Common trap:** on $$\mathbb{R}$$, the function $$x \mapsto x^3$$ is bijective. On $$\mathbb{Z}$$,
it is not surjective because most integers are not cubes.

---

**E3.** Prove that $$f : \mathbb{R} \to \mathbb{R}$$, $$f(x) = 5x - 3$$, is a bijection.

**Worked answer:**

*Injective:* Suppose $$5x_1 - 3 = 5x_2 - 3$$. Then $$5x_1 = 5x_2$$, so $$x_1 = x_2$$. ✓

*Surjective:* Given $$y \in \mathbb{R}$$, set $$x = \frac{y+3}{5}$$. Then $$f(x) = 5 \cdot \frac{y+3}{5} - 3 = y+3-3 = y$$. ✓

Both hold, so $$f$$ is bijective. ∎

---

## Tier 3 — Composition and inverses

**E4.** Let $$f, g : \mathbb{R} \to \mathbb{R}$$ be defined by $$f(x) = 2x + 1$$ and $$g(x) = x^2$$.

(a) Compute $$(g \circ f)(3)$$.  
(b) Compute $$(f \circ g)(3)$$.  
(c) Is $$g \circ f = f \circ g$$?

**Worked answers:**

(a) $$(g \circ f)(3) = g(f(3)) = g(7) = 49$$

(b) $$(f \circ g)(3) = f(g(3)) = f(9) = 19$$

(c) No. Composition is not commutative in general.

---

**E5.** Find the inverse of $$f : \mathbb{R} \to \mathbb{R}$$, $$f(x) = 3x - 2$$.

**Worked answer:**

Set $$y = f(x) = 3x - 2$$. Solve for x:

$$x = \frac{y + 2}{3}.$$

So $$f^{-1}(y) = \frac{y + 2}{3}$$.

Verify: $$f(f^{-1}(y)) = 3 \cdot \frac{y+2}{3} - 2 = y + 2 - 2 = y$$. ✓  
Also: $$f^{-1}(f(x)) = \frac{(3x-2)+2}{3} = \frac{3x}{3} = x$$. ✓

---

**E6.** Show that if $$f : A \to B$$ and $$g : B \to C$$ are both injective, then $$g \circ f$$ is injective.

**Worked answer:**

*Proof.* Suppose $$(g \circ f)(a_1) = (g \circ f)(a_2)$$.  
Then $$g(f(a_1)) = g(f(a_2))$$.  
Since $$g$$ is injective, $$f(a_1) = f(a_2)$$.  
Since $$f$$ is injective, $$a_1 = a_2$$. ∎

---

## Tier 4 — Image and preimage

**E7.** Let $$f : \mathbb{Z} \to \mathbb{Z}$$, $$f(n) = 2n$$. Compute:

(a) $$f(\{1, 2, 3\})$$  
(b) $$f^{-1}(\{2, 4, 6\})$$  
(c) $$f^{-1}(\{1, 3\})$$

**Worked answers:**

(a) $$\{2, 4, 6\}$$

(b) $$\{n \in \mathbb{Z} : 2n \in \{2,4,6\}\} = \{1, 2, 3\}$$

(c) $$\{n \in \mathbb{Z} : 2n \in \{1,3\}\}$$. Since 1 and 3 are odd, no integer n satisfies this. So $$f^{-1}(\{1,3\}) = \emptyset$$.

---

## Tier 5 — Structural reasoning

**E8.** Let A, B, C be sets. Suppose $$g \circ f : A \to C$$ is surjective. Prove that $$g$$ is surjective.

**Worked answer:**

*Proof.* Let $$c \in C$$.  
Since $$g \circ f$$ is surjective, there exists $$a \in A$$ such that $$(g \circ f)(a) = c$$, i.e., $$g(f(a)) = c$$.  
Let $$b = f(a) \in B$$. Then $$g(b) = c$$.  
So every element of C has a preimage in B under g. Therefore g is surjective. ∎

---

**E9.** Let A, B, C be sets. Suppose $$g \circ f : A \to C$$ is injective. Prove that $$f$$ is injective.

**Worked answer:**

*Proof.* Suppose $$f(a_1) = f(a_2)$$.  
Then $$g(f(a_1)) = g(f(a_2))$$, i.e., $$(g \circ f)(a_1) = (g \circ f)(a_2)$$.  
Since $$g \circ f$$ is injective, $$a_1 = a_2$$. ∎

*Note:* This shows injectivity passes backwards through composition (to f), and surjectivity passes backwards to g. These are dual statements.

**Do not over-generalise:**

- $$(g \circ f)$$ injective does **not** force $$g$$ to be injective.
- $$(g \circ f)$$ surjective does **not** force $$f$$ to be surjective.

The safe direction is exactly what E8 and E9 prove.

---

## Rust mirror

For the finite-set version of these exercises, read the symbols this way:

- | $$f : A \to B$$ becomes `FiniteFunction<A, B>`
- | $$f(S)$$ becomes `image_of(&subset)`
- | $$f^{-1}(T)$$ becomes `preimage_of(&subset)`

```rust
use std::collections::{HashMap, HashSet};

use p00_math_maturity::functions::FiniteFunction;

let f = FiniteFunction::new(
	HashMap::from([(1, 2), (2, 4), (3, 6)]),
	HashSet::from([2, 4, 6]),
);

assert_eq!(f.image_of(&HashSet::from([1, 2])), HashSet::from([2, 4]));
assert_eq!(f.preimage_of(&HashSet::from([2, 6])), HashSet::from([1, 3]));
```

---

## Self-Test Checklist

- [ ] Can you state the definitions of injective and surjective from memory?
- [ ] Can you prove functions injective or surjective using the standard templates?
- [ ] Can you find counterexamples quickly when a property fails?
- [ ] Can you compute inverses by solving f(x) = y for x?
- [ ] Can you prove E8 and E9-type structural lemmas on demand?
