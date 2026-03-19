# Rust Learning Track

Rust is the default language for simulations, utilities, data processing, and later embedded systems work.

## Why Rust Fits This Curriculum

1. Good performance for numerical work
2. Strong type system for modeling systems carefully
3. Useful later for embedded development
4. Good discipline for building reliable tools

## Stage 1: Core Language Fluency

Topics:

1. Variables and control flow
2. Functions and modules
3. Structs and enums
4. Ownership and borrowing
5. Traits and generics
6. Error handling
7. Collections and iterators

Outputs:

1. Small CLI tools
2. CSV writers for simulation output
3. Unit-safe helper types where useful

## Stage 2: Scientific And Numerical Rust

Topics:

1. Floating-point behavior
2. Vector and matrix libraries
3. Random sampling
4. Reading and writing datasets
5. Simple plotting workflows via exported CSV or compatible tools

Suggested crates:

1. `nalgebra`
2. `rand`
3. `csv`
4. `serde`

Outputs:

1. Numerical integration utilities
2. Linear algebra exercises
3. Monte Carlo experiments

## Stage 3: Systems And Embedded Rust

Topics:

1. Memory layout intuition
2. Concurrency basics when needed
3. No-std basics later
4. MCU ecosystem overview
5. Hardware interface patterns

Outputs:

1. Small system utilities
2. Sensor-processing prototypes
3. Embedded experiments when hardware work begins

## Project Pattern

For each simulation or engineering topic:

1. Implement a direct version first
2. Validate against hand calculations
3. Add input parameters
4. Export results cleanly
5. Write a short note on numerical limitations
