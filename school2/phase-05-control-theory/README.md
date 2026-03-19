# Phase 05 — Control Theory

**Duration:** 2–3 months  
**Prerequisites:** Phase 01 (ODEs, Laplace transform, linear algebra), Phase 04 (signals & systems)  
**Feeds directly into:** Phase 06 (robotics); projects `pid-controller` and `kalman-filter`

---

## Why This Phase Exists

Control theory is the mathematics of making physical systems *behave the way you want them to*.
Every motor you drive, every flight controller you write, every chemical process you regulate,
every autonomous vehicle you design depends on control theory. It connects differential equations
to practical engineering decisions: how do you make a system stable? How do you reject
disturbances? How do you estimate state you cannot measure? This phase also introduces the
Kalman filter — the most important estimation algorithm in all of engineering.

---

## Learning Objectives

- [ ] Model physical systems as transfer functions and state-space representations
- [ ] Analyse stability using Routh-Hurwitz, Bode plots, Nyquist criterion, and phase/gain margins
- [ ] Design PID controllers; tune them analytically and empirically; understand anti-windup
- [ ] Use root locus to reason about closed-loop pole placement as gain varies
- [ ] Apply frequency-domain lead/lag compensator design
- [ ] Formulate and analyse systems in state-space: controllability, observability
- [ ] Design state feedback controllers (pole placement); design state observers (Luenberger)
- [ ] Derive the LQR optimal controller and understand the Riccati equation
- [ ] Derive the Kalman filter as the optimal state estimator; implement it in Rust
- [ ] Understand the Extended Kalman Filter (EKF) and Unscented Kalman Filter (UKF) for nonlinear systems

---

## Topics

### 1. System Modelling

#### Physical System Modelling
- Mechanical systems: translational and rotational; free-body diagram to ODE
- Electrical systems: RLC circuits as differential equations; analogies with mechanical systems
- Thermal and fluid systems: RC thermal models; tank and pipe hydraulics

#### Transfer Functions
- Laplace transform review; transfer function $G(s) = Y(s)/U(s)$
- Block diagram algebra: series, parallel, feedback loops; simplification rules
- Signal flow graphs; Mason's gain formula
- Standard first-order and second-order responses: time constant, natural frequency $\omega_n$, damping ratio $\zeta$
- Steady-state error; system type; error constants

### 2. Stability Analysis

#### BIBO Stability
- Pole locations in the s-plane and their time-domain signatures
- Routh-Hurwitz stability criterion: tabular method, special cases

#### Frequency-Domain Analysis
- Bode plots: magnitude and phase; asymptotic approximations
- Gain margin and phase margin: definitions and physical meaning
- Nyquist stability criterion: encirclements, the $-1$ point, RHP poles
- Nichols chart (overview)

### 3. Classical Control Design

#### PID Control
- Proportional, integral, derivative actions: effect on steady-state error, transient response, noise
- Ziegler-Nichols and Cohen-Coon tuning rules
- Anti-windup strategies (back-calculation, conditional integration)
- Derivative filtering; practical implementation considerations
- Digital PID: backward Euler and bilinear (Tustin) discretisation

#### Root Locus
- Rules for constructing root locus from open-loop poles and zeros
- Closed-loop pole placement via gain selection
- Design for specified damping ratio and natural frequency

#### Frequency-Domain Compensator Design
- Lead compensator: increased phase margin, faster response
- Lag compensator: improved steady-state accuracy
- Lead-lag design procedure

### 4. State-Space (Modern Control)

#### State-Space Representation
- State equations $\dot{x} = Ax + Bu$, $y = Cx + Du$
- Relationship to transfer function: $G(s) = C(sI - A)^{-1}B + D$
- State transition matrix; matrix exponential $e^{At}$
- Solutions to state equations: homogeneous and particular

#### Controllability and Observability
- Controllability matrix $\mathcal{C}$; rank test; physical meaning
- Observability matrix $\mathcal{O}$; rank test; physical meaning
- Duality of controllability and observability

#### State Feedback & Pole Placement
- Full state feedback $u = -Kx$; how K shifts closed-loop poles
- Pole placement: Ackermann's formula; Bass-Gura method
- Integral action in state feedback for zero steady-state error

#### State Observers (Luenberger)
- Observer equations; observer error dynamics; observer gain $L$
- Observer pole placement; separation principle

### 5. Optimal Control

#### Linear Quadratic Regulator (LQR)
- Cost function $J = \int_0^\infty (x^T Q x + u^T R u)\, dt$
- Algebraic Riccati equation (ARE): solving for optimal gain $K$
- Effect of Q/R weighting matrices on performance/effort tradeoff

#### Kalman Filter (Linear Quadratic Estimator)
- Stochastic state-space model: process noise $Q$ and measurement noise $R$
- Prediction step: state and covariance propagation
- Update step: Kalman gain computation, state and covariance correction
- Steady-state Kalman filter; DARE
- Duality of LQR and Kalman filter (LQG design)

#### Extended Kalman Filter (EKF)
- Linearisation about current estimate: Jacobians of $f$ and $h$
- EKF prediction and update equations
- Limitations of EKF; when it fails

#### Unscented Kalman Filter (UKF)
- Sigma point selection and propagation
- Unscented transform; mean and covariance recovery
- UKF vs. EKF: when UKF is preferable

### 6. Discrete-Time Control
- Discretisation of continuous controllers: forward Euler, backward Euler, Tustin (bilinear)
- Discrete state-space; discrete system stability: poles inside the unit circle
- Discrete Riccati equation; discrete LQR
- Sampling rate selection; Nyquist criterion applied to control

---

## Core Texts

| Text | Notes |
|------|-------|
| **Ogata — Modern Control Engineering** (5th ed.) | The standard classical control text. Routh-Hurwitz, root locus, Bode, and PID — all rigorous with excellent problems. |
| **Åström & Murray — Feedback Systems: An Introduction for Scientists and Engineers** | Free at: caltechbook.library.caltech.edu. Modern, state-space-centred, rigorous. Use alongside Ogata. |
| **Chen — Linear System Theory and Design** (4th ed.) | The rigorous state-space reference. Controllability, observability, pole placement, observers. |
| **Welch & Bishop — An Introduction to the Kalman Filter** | Technical report (UNC TR 95-041, free online). The clearest Kalman filter derivation available. |

---

## Supplementary
- Franklin, Powell & Emami-Naeini — *Feedback Control of Dynamic Systems* — alternative classical reference
- Simon — *Optimal State Estimation* — deep Kalman, EKF, UKF, particle filter reference
- MIT 6.302 (OCW) — Feedback System Design; good problem sets

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| PID controller + plant simulator | Simulate a 2nd-order plant; tune PID; plot step response; anti-windup |
| Root locus plotter | Compute and plot root locus for arbitrary transfer function |
| Bode/Nyquist plotter | Generate Bode and Nyquist plots; mark gain/phase margins |
| Pole placement controller | State feedback for mass-spring-damper; compare to LQR |
| Linear Kalman filter | Estimate position+velocity from noisy measurement; compare to true state |
| EKF — pendulum angle   | Estimate pendulum angle from nonlinear dynamics and noisy measurement |

The primary implementation targets are `projects/pid-controller` and `projects/kalman-filter`.

---

## Completion Criteria

You are ready for Phase 06 when you can:
1. Design a PID controller with anti-windup for a given second-order plant using frequency-domain methods
2. Determine gain and phase margin from a Bode plot and explain their physical meaning
3. Test controllability and observability of a state-space system
4. Implement a discrete Kalman filter in Rust that estimates the states of a noisy linear system
5. Set up and solve the algebraic Riccati equation for an LQR controller
