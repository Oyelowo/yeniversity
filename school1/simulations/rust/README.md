# Rust Simulations Workspace

This directory contains numerical experiments, small modeling tools, and later embedded-adjacent utilities.

## Conventions

1. Add one binary per topic under `src/bin/`.
2. Keep each program focused on one concept.
3. Validate outputs against hand calculations before extending the model.
4. Prefer clarity first, then optimization.

## Suggested Early Binaries

1. `projectile_motion.rs`
2. `pendulum.rs`
3. `spring_mass_damper.rs`
4. `rc_circuit.rs`
5. `matrix_solve.rs`

## Running

Examples:

```bash
cargo run --bin projectile_motion
```

Later, add tests and shared modules as the workspace grows.
