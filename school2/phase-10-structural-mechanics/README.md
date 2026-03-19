# Phase 10 — Structural Mechanics

**Duration:** 2–3 months  
**Prerequisites:** Phase 01 (calculus, linear algebra, ODEs), Phase 02 (classical mechanics)  
**Can start:** After Phase 02; in parallel with Phase 09 or 11

---

## Why This Phase Exists

Every physical object you build — a robot arm, an aircraft frame, a rocket body, a bridge, a
PCB, a heat sink — must not break. Structural mechanics is the science of predicting whether
it will. This phase teaches you to calculate stress and strain in any loaded body, design
cross-sections for minimum weight under given loads, and use the Finite Element Method — the
algorithm underlying every serious engineering simulation tool in existence — from scratch.

---

## Learning Objectives

- [ ] Define stress and strain tensors; transform them using Mohr's circle and rotation matrices
- [ ] Apply equilibrium equations to statically determinate and indeterminate structures
- [ ] Compute stresses in bars, beams, and shafts under axial, bending, shear, and torsion loads
- [ ] Apply failure criteria: maximum normal stress, maximum shear stress (Tresca), von Mises, fatigue
- [ ] Derive and solve the beam bending differential equation for various loading and boundary conditions
- [ ] Analyse truss and frame structures using the stiffness method
- [ ] Understand buckling: Euler's column buckling; critical loads; slenderness ratio
- [ ] Implement the Finite Element Method (FEM) for 1D bars and 2D planar frames in Rust
- [ ] Understand basic fracture mechanics: stress intensity factor, fatigue crack growth

---

## Topics

### 1. Stress and Strain

#### Stress
- Traction vector; Cauchy stress tensor $\sigma_{ij}$; 3D and 2D (plane stress, plane strain)
- Normal stress $\sigma_{xx}$, $\sigma_{yy}$; shear stress $\tau_{xy}$
- Principal stresses; principal planes; stress invariants
- Mohr's circle for 2D stress transformation
- Equilibrium equations: $\partial\sigma_{ij}/\partial x_j + b_i = 0$

#### Strain
- Axial strain $\varepsilon = \delta/L$; shear strain $\gamma$
- Strain tensor $\varepsilon_{ij} = \frac{1}{2}(\partial u_i/\partial x_j + \partial u_j/\partial x_i)$
- Volumetric strain; dilatation
- Strain compatibility equations

#### Constitutive Relations
- Hooke's law for isotropic elastic solid: $\sigma = E\varepsilon$; $\tau = G\gamma$
- Generalised Hooke's law (3D); stiffness and compliance tensors
- Young's modulus $E$; Poisson's ratio $\nu$; shear modulus $G$
- Relationship: $G = E / [2(1+\nu)]$
- Thermal strains; thermoelastic constitutive law

### 2. Statically Determinate Structures

#### Axial Members (Bars and Rods)
- Internal force diagrams; axial stress and deformation
- Statically determinate bar assemblies; finding reactions
- Elastic deformation: $\delta = PL/AE$; stiffness $k = AE/L$
- Statically indeterminate bars: compatibility equations

#### Torsion of Circular Shafts
- Shear stress distribution in circular shaft: $\tau = Tr/J$
- Polar moment of inertia $J$; solid and hollow shafts
- Angle of twist: $\phi = TL/GJ$
- Power transmission shafts; design for given torque and angular velocity

#### Shear Force and Bending Moment Diagrams
- Differential relationships: $dV/dx = -q$, $dM/dx = V$
- Construction of shear force (SFD) and bending moment diagrams (BMD) for various loads and supports
- Relationship between loading, SFD, and BMD

### 3. Beam Theory

#### Euler-Bernoulli Beam Theory
- Assumptions: plane sections remain plane; linear normal strain distribution
- Flexure formula: $\sigma_x = -My/I$; neutral axis; second moment of area $I$
- Shear stress in beams: $\tau = VQ/Ib$; shear flow; shear centre
- Beam deflection differential equation: $EI d^2v/dx^2 = M(x)$
- Integration method for deflections; boundary conditions
- Singularity functions (Macaulay's method) for distributed and point loads
- Superposition of deflections

#### More Advanced Beams
- Castigliano's theorem: compute deflection from strain energy
- Statically indeterminate beams: three-moment equation; moment distribution
- Curved beams; thin rings under loading
- Beams on elastic foundations (Winkler foundation)

### 4. Failure and Design

#### Failure Criteria for Ductile and Brittle Materials
- Maximum normal stress theory (Rankine) — brittle materials
- Maximum shear stress criterion (Tresca) — ductile materials
- Distortion energy criterion (von Mises): $\sigma_{VM} = \sqrt{\sigma_x^2 - \sigma_x\sigma_y + \sigma_y^2 + 3\tau_{xy}^2}$
- Safety factors; factor of safety in design

#### Fatigue
- S-N (Wöhler) curves; endurance limit; fatigue strength
- Stress concentration factors; notch sensitivity
- Goodman, Gerber, Soderberg lines for combined static + cyclic loading
- Fatigue life prediction

#### Fracture Mechanics
- Griffith energy balance; critical flaw size
- Stress intensity factor $K = \sigma\sqrt{\pi a} f(a/W)$; mode I, II, III
- Fracture toughness $K_{IC}$; plane strain condition
- Paris law for fatigue crack growth: $da/dN = C(\Delta K)^m$

#### Column Buckling
- Euler's critical load formula: $P_{cr} = \pi^2 EI / (KL)^2$; effective length factor $K$
- Slenderness ratio; transition between elastic and inelastic buckling (Johnson curve)
- Eccentrically loaded columns; lateral-torsional buckling

### 5. Truss and Frame Analysis

#### Trusses
- Idealised truss; two-force members; method of joints; method of sections
- Statically determinate and indeterminate trusses
- Deflection of trusses via virtual work (unit load method)

#### Frame Structures
- Stiffness method (matrix structural analysis)
- Element stiffness matrix for a bar element: $k = AE/L \begin{bmatrix} 1 & -1 \\ -1 & 1 \end{bmatrix}$
- Assembly of global stiffness matrix; applying boundary conditions; solving for displacements
- Recovering element forces from nodal displacements
- Generalisations: 2D frame elements (axial + bending); 3D elements

### 6. Finite Element Method (FEM)

#### FEM Theory
- Weighted residual and Galerkin method: weak form derivation
- Principle of virtual work and minimum potential energy
- Shape functions for 1D bar element (linear); interpolation of displacement field
- Stiffness matrix from potential energy minimisation: $K = \int_V B^T C B\, dV$
- Assembly procedure; essential and natural boundary conditions
- Solving the linear system $K u = f$
- Stress and strain recovery from nodal displacements: $\varepsilon = B u$; $\sigma = C \varepsilon$

#### 2D FEM Elements
- Constant strain triangle (CST): shape functions, stiffness matrix
- Bilinear quadrilateral (Q4): isoparametric mapping; Gaussian quadrature
- Mesh convergence; $h$-refinement and $p$-refinement; error estimation

---

## Core Texts

| Text | Notes |
|------|-------|
| **Hibbeler — Mechanics of Materials** (10th ed.) | The best undergraduate-level text. Clear derivations, excellent problem sets. Cover chapters 1–12 thoroughly. |
| **Hibbeler — Structural Analysis** (10th ed.) | Trusses, frames, stiffness method. Work chapters on matrix analysis carefully. |
| **Timoshenko & Goodier — Theory of Elasticity** | The rigorous classical reference for 2D and 3D elasticity. Read after Hibbeler. |
| **Logan — A First Course in the Finite Element Method** (6th ed.) | Clear FEM derivations from first principles; includes 1D, 2D, and 3D elements. |

---

## Supplementary
- Ugural & Fenster — *Advanced Mechanics of Materials and Applied Elasticity* — good for plate and shell theory
- Bathe — *Finite Element Procedures* — graduate-level FEM reference
- Anderson & Fracture Mechanics — *Fracture Mechanics: Fundamentals and Applications*

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Mohr's circle visualiser | Input $\sigma_x$, $\sigma_y$, $\tau_{xy}$; compute and plot principal stresses |
| Beam deflection solver | Integrate $EI v'' = M(x)$ numerically; compare to analytic solutions |
| SFD and BMD plotter | Arbitrary combined loading; compute and plot SFD/BMD |
| Truss solver (stiffness method) | Build global $K$; solve $K u = f$; compute member forces |
| 1D FEM bar element | Full 1D FEM pipeline: mesh, stiffness assembly, boundary conditions, solve, post-process |
| 2D FEM (CST) | 2D plane-stress problem; mesh a simple geometry; compare to analytic stress |
| Column buckling calculator | Critical load vs. slenderness ratio; plot Euler and Johnson curves |

---

## Completion Criteria

You are ready to move on when you can:
1. Compute the maximum bending stress in a simply-supported beam under a distributed load
2. Draw the shear force and bending moment diagrams for a propped cantilever
3. Derive the element stiffness matrix for a 1D bar from the principle of minimum potential energy
4. Implement the FEM assembly procedure for a simple truss and solve for nodal deflections
5. Apply the von Mises criterion to determine whether a biaxial stress state causes yielding
