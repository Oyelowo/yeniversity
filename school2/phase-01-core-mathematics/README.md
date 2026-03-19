# Phase 01 — Core Mathematics

**Duration:** 4–6 months  
**Prerequisites:** Phase 00 (mathematical maturity, proof fluency)  
**Runs alongside:** Begin reading Feynman Lectures Vol. I alongside this phase

---

## Why This Phase Exists

Every quantitative discipline you will ever study — physics, engineering, AI, control theory,
robotics — is applied mathematics. The four subjects in this phase *are* those applications'
foundations. Linear algebra is the language of quantum mechanics, robotics, and machine learning.
Calculus is the language of dynamics, fields, and optimization. ODEs/PDEs are the equations of
every physical system. Probability is the language of measurement, inference, and learning.

Do not rush this phase. Gaps here propagate through everything that follows.

---

## Learning Objectives

- [ ] Understand limits, continuity, and differentiability rigorously from the ε–δ definition
- [ ] Compute and interpret derivatives and integrals of all standard function types
- [ ] Master multivariable calculus: partial derivatives, gradient, divergence, curl, Jacobian, Hessian
- [ ] Understand line, surface, and volume integrals; theorems of Green, Stokes, and Gauss
- [ ] Understand vector spaces, linear maps, bases, dimension, inner products rigorously
- [ ] Compute determinants, eigenvalues, eigenvectors, matrix decompositions (LU, QR, SVD)
- [ ] Understand and apply the spectral theorem; diagonalisation
- [ ] Solve first- and second-order ODEs; understand existence and uniqueness
- [ ] Understand linear ODE systems in state-space form; phase portraits; stability
- [ ] Understand Fourier series and Fourier transform; solve heat, wave, and Laplace PDEs
- [ ] Derive and apply the axioms of probability; understand random variables, distributions, expectation
- [ ] Understand the Law of Large Numbers and Central Limit Theorem with proofs
- [ ] Apply Bayes' theorem correctly to inference problems

---

## Topics

### 1. Single-Variable Calculus
- The real number system and completeness axiom (skim; detailed in Spivak Ch. 1)
- Limits: ε–δ definition, one-sided limits, limits at infinity
- Continuity: definition, intermediate value theorem, extreme value theorem
- Differentiation: definition from first principles, rules, implicit differentiation, related rates
- Mean value theorem and its consequences
- Integration: Riemann sums, definite integral, fundamental theorem of calculus
- Techniques of integration: substitution, integration by parts, partial fractions, trig substitution
- Improper integrals; convergence tests
- Sequences and series: convergence, ratio test, root test, comparison tests
- Power series, Taylor series, radius of convergence; Taylor's theorem with remainder

### 2. Multivariable Calculus
- Functions of several variables; level sets; partial derivatives
- Gradient as direction of steepest ascent; directional derivatives
- Total derivative as a linear map (Jacobian); chain rule for compositions
- Higher-order derivatives; Hessian; second derivative test for critical points
- Lagrange multipliers: constrained optimisation
- Multiple integrals: Fubini's theorem, change of variable, polar/cylindrical/spherical coordinates
- Vector fields: divergence, curl, conservative fields, potential functions
- Line integrals; Green's theorem in the plane
- Surface integrals; Stokes' theorem; Gauss' divergence theorem

### 3. Linear Algebra
- Vector spaces over ℝ and ℂ: axioms, subspaces, spans, linear independence
- Basis and dimension; coordinate vectors
- Linear maps: null space, image, rank-nullity theorem
- Matrices as representations of linear maps; matrix multiplication
- Determinants: geometric interpretation (signed volume), cofactor expansion, properties
- Eigenvalues and eigenvectors; characteristic polynomial; algebraic vs. geometric multiplicity
- Diagonalisation; conditions for diagonalisability
- Inner product spaces: dot product, norm, orthogonality, orthonormal bases
- Gram-Schmidt orthogonalisation
- Symmetric matrices; spectral theorem (real symmetric ⟹ orthogonally diagonalisable)
- Singular value decomposition (SVD); pseudoinverse; least-squares via SVD
- LU decomposition; QR decomposition
- Positive definite matrices; Cholesky decomposition

### 4. Ordinary Differential Equations
- Separable ODEs; first-order linear ODEs (integrating factor method)
- Exact equations and conservation laws
- Second-order linear ODEs with constant coefficients: homogeneous and particular solutions
- Methods: undetermined coefficients, variation of parameters
- Reduction of order; Euler-Cauchy equation
- Series solutions about ordinary and regular singular points (Frobenius method)
- Systems of ODEs: state-space form; matrix exponential
- Phase plane analysis; equilibria; stability (Lyapunov methods overview)
- Laplace transform: definition, properties, solving IVPs with discontinuous inputs

### 5. Partial Differential Equations
- Classification: elliptic, parabolic, hyperbolic
- Method of characteristics (first-order PDEs)
- Separation of variables
- Fourier series: derivation, convergence (Dirichlet conditions), Parseval's identity
- Fourier transform: definition, properties (convolution, shift, Parseval), inverse
- Heat equation (diffusion equation): derivation, solution by separation and Fourier transform
- Wave equation: derivation, d'Alembert's formula, standing waves
- Laplace's equation: harmonic functions, boundary value problems

### 6. Probability & Statistics
- Probability spaces: sample space, events, σ-algebras, axioms of probability (Kolmogorov)
- Conditional probability; independence; Bayes' theorem
- Discrete random variables: PMF, CDF, expectation, variance
- Standard discrete distributions: Bernoulli, Binomial, Geometric, Poisson
- Continuous random variables: PDF, CDF, expectation, variance, moment generating functions
- Standard continuous distributions: Uniform, Exponential, Normal (Gaussian), Gamma, Beta
- Jointly distributed random variables: joint, marginal, conditional distributions; independence
- Covariance, correlation
- Law of Large Numbers (weak and strong); proof of weak LLN
- Central Limit Theorem: statement, intuition, proof sketch
- Maximum likelihood estimation; method of moments
- Confidence intervals; hypothesis testing basics

---

## Core Texts

| Text | Notes |
|------|-------|
| **Spivak — Calculus** | Gold standard for rigorous single-variable. Derives everything from the real number axioms. Do the problems — they are essential. |
| **Apostol — Calculus Vol. I & II** | Covers single and multivariable rigorously; more applications than Spivak. Good second read or parallel reference. |
| **Axler — Linear Algebra Done Right** (3rd ed.) | Rigorous treatment; builds all of LA from maps rather than matrices. Read this after or alongside Strang. |
| **Strang — Introduction to Linear Algebra** (5th ed.) | Geometric and computational intuition. His MIT 18.06 lectures (free on YouTube) are invaluable. |
| **Tenenbaum & Pollard — Ordinary Differential Equations** | Dover paperback. Dense with worked examples; covers all standard techniques rigorously. |
| **Strauss — Partial Differential Equations: An Introduction** | Clear, accessible entry to PDEs. Covers heat, wave, and Laplace equations well. |
| **Blitzstein & Hwang — Introduction to Probability** (2nd ed.) | The best probability text. Stat 110 lectures are free on YouTube. Bayesian thinking from the start. |

---

## Supplementary
- 3Blue1Brown *Essence of Calculus* and *Essence of Linear Algebra* — for geometric intuition
- MIT 18.06 (Strang, OCW) — free lectures + problem sets
- MIT 18.03 (OCW) — ODEs course with excellent problem sets
- Evans — *Partial Differential Equations* — the rigorous graduate-level reference for later

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Numerical differentiation | Forward, backward, central difference; Richardson extrapolation; error analysis |
| Numerical integration | Trapezoidal rule, Simpson's rule, Gaussian quadrature; compare error orders |
| ODE solvers | Euler, RK2, RK4, adaptive RK45 — verify against analytic solutions |
| Linear system solver | Gaussian elimination, LU, back-substitution from scratch (the `matrix-library` project) |
| Eigenvalue solver | Power iteration, QR algorithm — verify against nalgebra results |
| Fourier transform (DFT) | Naive DFT, then Cooley-Tukey FFT from scratch |
| Monte Carlo integration | Estimate π, compute high-dim integrals; verify CLT numerically |
| Random variable sampler | Implement Box-Muller, inverse CDF method, rejection sampling |

The main implementation home for larger work is the `projects/matrix-library` and `projects/ode-solver` crates.

---

## Notes & Exercises Directory Convention

```
notes/
  calculus/
    01-limits-and-continuity.md
    02-differentiation.md
    03-integration.md
    04-series-and-power-series.md
    05-multivariable-calculus.md
    06-vector-calculus-theorems.md
  linear-algebra/
    01-vector-spaces.md
    02-linear-maps-and-matrices.md
    03-determinants.md
    04-eigenvalues-eigenvectors.md
    05-inner-products-and-svd.md
  differential-equations/
    01-first-order-odes.md
    02-second-order-odes.md
    03-systems-of-odes.md
    04-laplace-transform.md
    05-pdes-and-fourier-methods.md
  probability-statistics/
    01-probability-axioms.md
    02-random-variables.md
    03-standard-distributions.md
    04-joint-distributions.md
    05-limit-theorems.md
    06-estimation-and-testing.md

exercises/
  calculus/          ← one file per topic with problems + worked solutions
  linear-algebra/
  differential-equations/
  probability-statistics/
```

---

## Completion Criteria

You are ready for Phase 02 when you can:
1. Derive the fundamental theorem of calculus from the definition of the integral
2. State and prove the rank-nullity theorem
3. Find eigenvalues of a 3×3 matrix and diagonalise it
4. Solve $y'' + 3y' + 2y = e^t$ by hand using undetermined coefficients
5. Derive the heat equation solution using separation of variables
6. Apply Bayes' theorem correctly to a multi-step inference problem
