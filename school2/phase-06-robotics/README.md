# Phase 06 — Robotics

**Duration:** 3–4 months  
**Prerequisites:** Phase 02 (mechanics), Phase 05 (control theory), Phase 04 (embedded systems)  
**Key project:** `projects/drone-flight-controller`

---

## Why This Phase Exists

Robotics is the synthesis of everything that came before: rigid-body mechanics, sensors,
actuators, estimation, control, and computation all working together in real-time on physical
hardware. A robot arm, a drone, a self-driving vehicle, an exoskeleton — these are Systems of
systems designed to interact with the physical world. This phase gives you the full toolkit:
how to model any robot's kinematics and dynamics from first principles, how to control it,
how to perceive an unknown environment, and how to plan motion through it.

---

## Learning Objectives

- [ ] Represent rigid-body poses using rotation matrices, Euler angles, quaternions, and homogeneous transforms
- [ ] Compute forward kinematics of serial manipulators using the Denavit-Hartenberg convention
- [ ] Solve inverse kinematics analytically (closed-form) and numerically (Jacobian methods)
- [ ] Understand the manipulator Jacobian; detect and handle singularities
- [ ] Derive the equations of motion for a robot arm using Lagrangian dynamics; compute inertia matrix, Coriolis, and gravity terms
- [ ] Implement computed torque control and PD gravity compensation for manipulator control
- [ ] Implement attitude estimation using an EKF or complementary filter on IMU data
- [ ] Implement a PID-based quadrotor position and attitude controller from scratch
- [ ] Understand probabilistic state estimation: Bayesian filter, particle filter
- [ ] Implement a 2D Kalman-filter SLAM pipeline (EKF-SLAM or graph SLAM)
- [ ] Implement path planning algorithms: A*, Dijkstra, RRT, RRT*

---

## Topics

### 1. Rigid-Body Geometry & Kinematics

#### Rotations
- Rotation matrices: SO(3); properties, composition, inverse
- Axis-angle representation; Rodrigues' rotation formula
- Euler angles: ZYX (roll-pitch-yaw) and ZYZ; gimbal lock problem
- Unit quaternions: SU(2) parameterisation of SO(3); advantages for computation
- Converting between representations: rotation matrix ↔ quaternion ↔ Euler angles
- Angular velocity: body frame vs. world frame; $\dot{R} = R[\omega]_\times$

#### Homogeneous Transforms
- SE(3): 4×4 homogeneous transformation matrix for rotation + translation
- Composition of transforms; inverse transform
- Frames: world, body, tool frames; changing reference frame

#### Forward Kinematics
- Serial manipulator link-frame convention
- Denavit-Hartenberg (DH) parameters: $a$, $\alpha$, $d$, $\theta$
- DH table to transformation matrix; product of matrices along kinematic chain
- Worked examples: 2R planar arm, 3R spatial arm, 6-DOF industrial arm

#### Inverse Kinematics
- Reachable workspace and workspace singularities
- Closed-form IK: geometric method for 2R, 3R planar; spherical wrist decoupling for 6-DOF
- Numerical IK: Jacobian transpose, Jacobian pseudo-inverse (damped least squares / Tikhonov)
- Redundancy resolution for redundant manipulators

#### Velocity Kinematics
- Manipulator Jacobian $J(\theta)$: geometric and analytic Jacobians
- Jacobian rank; kinematic singularities; manipulability ellipsoid
- Static force relationships: $\tau = J^T(\theta) f$

### 2. Robot Dynamics

#### Lagrangian Dynamics for Robots
- Kinetic energy of each link: translational + rotational
- Potential energy for each link
- Equations of motion: $M(\theta)\ddot{\theta} + C(\theta, \dot{\theta})\dot{\theta} + g(\theta) = \tau$
- Inertia matrix $M(\theta)$: positive definite, symmetric
- Coriolis and centripetal matrix $C(\theta, \dot{\theta})$
- Gravity vector $g(\theta)$
- Numerical computation: recursive Newton-Euler algorithm (efficient $O(n)$)

#### Control of Robot Arms
- PD control with gravity compensation: $\tau = K_p e + K_d \dot{e} + g(\theta)$
- Computed torque control (inverse dynamics): exact linearisation and decoupling
- Operational space control (Cartesian control)
- Joint space vs. operational space impedance control

### 3. Mobile Robots & State Estimation

#### Probabilistic State Estimation
- Probability review in robotics context; sensor models; motion models
- Bayes filter: predict and update cycle (general form)
- Histogram filter (grid localisation); overview
- Particle filter (Monte Carlo localisation): importance sampling, resampling
- Gaussian filters: the Kalman filter family specialised for robotics

#### Sensors for Robotics
- Encoder, IMU (accelerometer + gyroscope + magnetometer), GPS
- Lidar: time-of-flight, scan matching, point clouds
- Camera: pinhole model, intrinsic/extrinsic calibration, depth cameras (stereo, ToF, structured light)
- Sensor fusion: combining IMU and GPS with a Kalman filter; combining IMU and vision (VIO)

#### Attitude Estimation
- IMU integration: integrating gyroscope for orientation; drift accumulation
- Complementary filter: high-pass gyroscope + low-pass accelerometer
- Mahony and Madgwick filters
- EKF-based attitude estimator: state = quaternion + gyro bias

### 4. Simultaneous Localisation and Mapping (SLAM)

#### EKF-SLAM
- State vector: robot pose + landmark positions
- Prediction step: motion model with noise
- Update step: landmark observation model
- Data association: nearest-neighbour; JCBB
- Scalability problem: $O(n^2)$ with number of landmarks

#### Graph-Based SLAM (Overview)
- Pose graph: nodes = poses, edges = relative constraints from odometry or loop closures
- Back-end: nonlinear least-squares optimisation (g2o, GTSAM concepts)
- Loop closure detection

#### 2D Lidar SLAM (Practical)
- Occupancy grid mapping
- Iterative Closest Point (ICP) scan matching
- Hector SLAM / Cartographer concepts

### 5. Motion Planning

#### Graph Search
- Configuration space $\mathcal{C}$: free space $\mathcal{C}_{free}$ and obstacle space $\mathcal{C}_{obs}$
- Dijkstra's algorithm; A* search with heuristics; admissibility and optimality
- Weighted A*; D* for dynamic environments

#### Sampling-Based Planning
- Probabilistic Roadmap (PRM): build graph offline; query online
- Rapidly-Exploring Random Trees (RRT): incremental tree growth
- RRT*: asymptotically optimal variant; rewiring
- CHOMP and STOMP (gradient-based smoothing, overview)

#### Trajectory Planning
- Polynomial trajectories: cubic and quintic splines; boundary condition selection
- Trapezoidal velocity profiles; time-optimal planning under jerk limits
- Minimum-snap trajectory optimisation (quadrotor-relevant)

### 6. Quadrotor Dynamics & Control

#### Quadrotor Model
- Body frame; four motor positions
- Thrust and torque from each motor; mapping from motor speeds to forces/moments
- Rigid-body dynamics: Newton-Euler equations in body frame
- State vector: position, velocity, attitude (quaternion), angular velocity

#### Quadrotor Control
- Attitude control loop (inner loop): rate PID + attitude PID
- Position control loop (outer loop): 3D position PID
- Attitude to thrust/moment allocation (control mixing matrix)
- Motor speed to PWM (ESC calibration)
- Full cascade controller: position → attitude → rates → motors

---

## Core Texts

| Text | Notes |
|------|-------|
| **Lynch & Park — Modern Robotics: Mechanics, Planning, and Control** | Free at modernrobotics.org. Rigorous Lie-group treatment of kinematics and dynamics. The best modern text. The companion Coursera course is also excellent. |
| **Thrun, Burgard & Fox — Probabilistic Robotics** | The definitive reference for Bayesian estimation, Kalman filter in robotics, particle filter, SLAM. Dense but complete. |
| **Siciliano et al. — Robotics: Modelling, Planning and Control** | Comprehensive reference; stronger on dynamics and control than Lynch & Park. |
| **Mahony, Kumar et al. — Multirotor Aerial Vehicles** | Research papers; free online; essential for quadrotor dynamics and control. |

---

## Supplementary
- Craig — *Introduction to Robotics* — classical DH-convention reference
- Choset et al. — *Principles of Robot Motion* — planning algorithms reference
- MIT 6.832 *Underactuated Robotics* (Tedrake, free online) — advanced dynamics and control

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Forward/inverse kinematics | 2R planar arm and 6-DOF arm; IK via Jacobian pseudoinverse |
| Manipulator dynamics | Compute $M$, $C$, $g$ symbolically and numerically; computed torque controller |
| Particle filter localisation | 2D mobile robot localising in a known map |
| EKF-SLAM (2D) | Simultaneous localisation and mapping with range-bearing sensor |
| A* path planner | 2D occupancy grid; compare to RRT |
| RRT / RRT* implementation | 2D and 3D configuration spaces |
| Quadrotor simulator | Full 6-DOF dynamics + cascade PID controller + sensor noise |

The primary build target is `projects/drone-flight-controller`.

---

## Build Projects (Physical Hardware)

| Project | Notes |
|---------|-------|
| 2-DOF robot arm | Servo motors; custom kinematics solver running on microcontroller; teach pendant |
| Quadrotor from scratch | Frame + ESCs + motors + RP2040 or STM32 + IMU; all firmware in Rust, no Betaflight |
| IMU-based attitude estimator | Mahony or EKF quaternion estimator; verify with reference hardware |

---

## Completion Criteria

You are ready for Phase 07 (or Phase 09/10) when you can:
1. Derive the forward kinematics of a 3R arm using the DH convention and homogeneous transforms
2. Solve inverse kinematics numerically using the damped Jacobian pseudoinverse
3. Write down and explain the equations of motion $M\ddot{\theta} + C\dot{\theta} + g = \tau$
4. Implement a particle filter for 2D localisation in Rust
5. Describe the full quadrotor cascade control loop from position command to motor output
