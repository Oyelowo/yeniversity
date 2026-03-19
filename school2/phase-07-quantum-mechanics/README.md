# Phase 07 — Quantum Mechanics

**Duration:** 2–3 months  
**Prerequisites:** Phase 01 (linear algebra, PDEs, complex analysis), Phase 02 (classical mechanics through Hamiltonian formulation)  
**Can start:** After Phase 02, in parallel with Phase 06

---

## Why This Phase Exists

Quantum mechanics is not optional for hardware engineering at depth. It is the only explanation
for why semiconductors work, why lasers exist, why materials have the properties they do, why
nuclear energy is possible, and why quantum computing is different in kind from classical
computing. The transistor — the basis of all modern electronics — only exists because someone
understood quantum mechanics well enough to engineer a band gap. This phase gives you that
foundation, and then applies it to the solid state: from Schrödinger's equation to the silicon
lattice you fabricate in Phase 13.

---

## Learning Objectives

- [ ] Understand the postulates of quantum mechanics: state vectors, observables as operators, measurement, time evolution
- [ ] Solve the time-independent Schrödinger equation for standard potentials (infinite/finite square well, harmonic oscillator, hydrogen atom)
- [ ] Understand the operator formalism: commutators, uncertainty principle, eigenstates, eigenvalues
- [ ] Understand angular momentum in quantum mechanics: $L^2$ and $L_z$ operators, spherical harmonics
- [ ] Understand spin: spin-1/2, Pauli matrices, spinors, addition of angular momenta
- [ ] Understand time-independent perturbation theory; apply to real atoms
- [ ] Understand time-dependent perturbation theory; Fermi's golden rule; selection rules
- [ ] Understand the quantum mechanics of identical particles; symmetrisation postulate; Pauli exclusion
- [ ] Understand the free electron model and band theory: Bloch's theorem, Brillouin zones, energy bands
- [ ] Understand the quantum mechanical basis of semiconductor physics (connecting Phase 03 to deeper foundations)
- [ ] Understand the basics of a qubit; superposition and entanglement; quantum gate model

---

## Topics

### 1. Foundations of Quantum Mechanics

#### Historical Context and Motivation
- Blackbody radiation; Planck hypothesis; quantisation of energy
- Photoelectric effect; Einstein's photon hypothesis
- de Broglie hypothesis; wave-particle duality; double-slit experiment
- Bohr model of hydrogen; energy quantisation (and its limitations)
- The need for a fully quantum theory

#### Mathematical Framework
- State vectors and Hilbert space; Dirac bra-ket notation $|\psi\rangle$, $\langle\phi|$
- Operators: Hermitian operators; eigenvalues; eigenstates
- Completeness; orthonormality; eigenbasis expansion
- Commutators $[A, B] = AB - BA$; compatible and incompatible observables
- The generalised uncertainty principle $\Delta A \Delta B \geq \frac{1}{2}|\langle[A,B]\rangle|$
- The position and momentum operators in position space: $\hat{x}$ and $\hat{p} = -i\hbar \partial/\partial x$

#### The Postulates
1. The state of a quantum system is described by a normalised state vector $|\psi\rangle$ in Hilbert space
2. Observables correspond to Hermitian operators
3. Measurement of observable $A$ yields eigenvalue $a$; probability $|\langle a|\psi\rangle|^2$; state collapses to $|a\rangle$
4. Between measurements, the state evolves according to the time-dependent Schrödinger equation
5. The Hamiltonian is the energy operator governing time evolution

#### The Schrödinger Equation
- Time-dependent Schrödinger equation: $i\hbar \frac{\partial}{\partial t}|\psi\rangle = \hat{H}|\psi\rangle$
- Time-independent equation: $\hat{H}|\psi\rangle = E|\psi\rangle$ for stationary states
- Wave functions in position representation; probability density $|\psi(x)|^2$
- Boundary conditions; continuity requirements; normalisation

### 2. Standard Problems (1D)

#### Infinite Square Well
- Boundary conditions; quantised energy levels $E_n = n^2\pi^2\hbar^2 / 2mL^2$
- Stationary state wave functions; orthogonality
- Time evolution; probability current; energy eigenstates

#### Finite Square Well
- Bound states: transcendental eigenvalue equation; numerical solution
- Comparison with infinite well; barrier penetration
- Scattering states: transmission and reflection; resonances

#### Quantum Harmonic Oscillator
- Ladder operators (creation/annihilation): $a^+$ and $a^-$
- Energy eigenvalues: $E_n = (n + 1/2)\hbar\omega$
- Algebraic derivation of eigenstates; matrix elements
- Coherent states; classical limit
- Connection to phonons in solids (Phase 07 applies directly to Phase 13)

#### Barrier Tunnelling
- Rectangular potential barrier; transmission coefficient
- Quantum tunnelling; WKB approximation
- Applications: tunnel diode, scanning tunnelling microscope, nuclear alpha decay

#### Delta Function Potential
- Bound state of the delta function well
- Scattering off a delta function barrier

### 3. Quantum Mechanics in 3D

#### Central Potentials
- Separation in spherical coordinates; radial and angular parts
- Angular momentum operators $\hat{L}^2$ and $\hat{L}_z$; eigenvalues $\ell(\ell+1)\hbar^2$ and $m\hbar$
- Spherical harmonics $Y_\ell^m(\theta, \phi)$; their properties

#### The Hydrogen Atom
- Radial equation; effective potential; quantised energy $E_n = -13.6\text{ eV}/n^2$
- Radial wave functions; principal quantum number $n$, angular $\ell$, magnetic $m$
- Degeneracy of energy levels; spectral series: Lyman, Balmer, Paschen

#### Spin
- Intrinsic angular momentum; Stern-Gerlach experiment
- Spin-1/2 algebra; Pauli matrices $\sigma_x$, $\sigma_y$, $\sigma_z$
- Spinors; spin-up and spin-down states; probabilities
- Addition of angular momenta: Clebsch-Gordan coefficients
- Spin-orbit coupling; fine structure of hydrogen

#### Identical Particles
- Symmetrisation postulate: bosons (symmetric) vs. fermions (antisymmetric)
- Pauli exclusion principle from antisymmetry
- Slater determinants for multi-electron systems
- Consequences for chemistry: electron configurations, periodic table

### 4. Approximation Methods

#### Time-Independent Perturbation Theory
- Non-degenerate case: first- and second-order energy corrections
- Degenerate perturbation theory: diagonalising the perturbation within the degenerate subspace
- Applications: fine structure of hydrogen (relativistic + spin-orbit), Stark effect, Zeeman effect

#### Variational Method
- Variational principle: $\langle\psi|\hat{H}|\psi\rangle \geq E_{ground}$
- Selecting trial wave functions; applications to helium ground state

#### WKB Approximation
- Classical turning points; quantisation condition
- Tunnelling probability through a barrier

#### Time-Dependent Perturbation Theory
- First-order transition amplitude; transition probability
- Fermi's golden rule: transition rate to a continuum of states; density of states
- Electromagnetic radiation: absorption, stimulated and spontaneous emission; selection rules

### 5. Quantum Statistical Mechanics

- Density operator (density matrix) formalism
- Pure states vs. mixed states
- Entropy of a quantum state; relation to thermodynamics
- Fermi-Dirac distribution derived from quantum statistical mechanics
- Bose-Einstein distribution; Bose-Einstein condensation (overview)

### 6. Solid-State Physics (Quantum Basis)

#### Free Electron Model Revisited
- Electrons in a box; allowed k-values; density of states in 3D
- Fermi energy $E_F$; Fermi surface; electronic heat capacity

#### Bloch's Theorem and Band Theory
- Periodic potential; Bloch's theorem: $\psi_{n,k}(r) = e^{ik\cdot r} u_{n,k}(r)$
- Brillouin zones; reduced zone scheme
- Band gaps at zone boundaries; origin from nearly-free-electron model
- Tight-binding model: bands from atomic orbitals
- Effective mass approximation; electrons and holes
- Metals, insulators, and semiconductors from band theory — rigorous derivation

#### Phonons
- Lattice vibrations; normal modes
- Quantisation of lattice vibrations: phonons
- Acoustic and optical phonon branches
- Debye model from phonon density of states

#### Quantum Mechanical Basis of the p-n Junction
- Fermi level alignment at junctions
- Tunnelling in the tunnel diode
- Quantum confinement in quantum wells, wires, and dots

### 7. Introduction to Quantum Computing

- Qubit as a two-level quantum system; Bloch sphere representation
- Single-qubit gates: X (NOT), Y, Z, Hadamard (H), S, T as unitary matrices
- Two-qubit gates: CNOT; concept of entanglement; Bell states
- Quantum circuit model; universality
- Key quantum algorithms (conceptual): Deutsch, Grover, Shor

---

## Core Texts

| Text | Notes |
|------|-------|
| **Griffiths — Introduction to Quantum Mechanics** (3rd ed.) | The clearest, most accessible rigorous QM text. Solve every problem in Chapters 1–6. |
| **Shankar — Principles of Quantum Mechanics** (2nd ed.) | Deeper, more mathematical treatment; starts from linear algebra and builds up. Read after Griffiths. |
| **Kittel — Introduction to Solid State Physics** (8th ed.) | Applies QM to real materials: free electron model, band theory, phonons, semiconductors, superconductivity. |
| **Kittel & Kroemer — Thermal Physics** | Quantum statistical mechanics done rigorously; supplements Schroeder from Phase 02. |

---

## Supplementary
- Feynman Lectures Vol. III — quantum mechanics in Feynman's unique style; read alongside Griffiths
- Sakurai & Napolitano — *Modern Quantum Mechanics* — graduate-level; read after Shankar
- Nielson & Chuang — *Quantum Computation and Quantum Information* — the standard QC reference

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Particle-in-a-box | Numerical solution of TISE; compute energy levels and plot wavefunctions |
| Quantum harmonic oscillator | Ladder operator algebra; coherent state time evolution |
| Finite square well solver | Transcendental equation numerical solution; bound state wavefunctions |
| Hydrogen radial wavefunctions | Plot $R_{nl}(r)$ and radial probability density for various quantum numbers |
| Tunnelling probability | Compute transmission coefficient vs. barrier height and width |
| Density of states | Compute and plot DOS for 1D, 2D, 3D free electron gas |
| Band structure (tight-binding) | 1D chain under tight-binding model; plot $E(k)$ dispersion |
| Qubit simulator (statevector) | Single and two-qubit gate operations; Bell state preparation |

---

## Completion Criteria

You are ready to move on when you can:
1. Solve the time-independent Schrödinger equation for the infinite square well from the boundary conditions, without reference to notes
2. Derive the energy levels of the quantum harmonic oscillator using ladder operators
3. Explain why silicon has a band gap in terms of Bloch's theorem and the nearly-free-electron model
4. State Fermi's golden rule and explain physically what it means for absorption of light
5. Describe a qubit, the Hadamard gate, and what "entanglement" means mathematically
