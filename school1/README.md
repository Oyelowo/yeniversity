# First Principles Engineering

This directory is the operating system for rebuilding mathematics, physics, chemistry, hardware, robotics, AI, and engineering from first principles.

## Structure

- `curriculum/`: the roadmap and subject plans.
- `lecture-notes/`: concept summaries, derivations, worked examples, and synthesis notes.
- `exercises/`: handwritten or typed problem sets, drill work, and review sheets.
- `simulations/`: numerical models and experiments. Rust is the default language here.
- `projects/`: hands-on builds and integrated engineering work.
- `references/`: book lists, course links, papers, and standards.
- `templates/`: note and exercise templates.

## Study Loop

For each topic, work in this order:

1. Read and derive the theory.
2. Solve problems by hand.
3. Rebuild the same system numerically in Rust.
4. If possible, build or measure a physical version.
5. Write a short summary from memory.

If a topic has only reading and no exercises, simulations, or recall, it is not yet learned.

## Recommended Weekly Rhythm

- `Math`: 5 sessions
- `Physics`: 4 sessions
- `Rust simulations / programming`: 4 sessions
- `Hardware / projects`: 2 to 3 sessions
- `Review / memory consolidation`: 1 session

Baseline target:

- Weekdays: 2 focused hours per day
- Weekend: 4 to 6 total hours for projects, summaries, and backlog cleanup

## Suggested Workflow

For a new topic such as `mechanics`:

1. Add lecture notes under `lecture-notes/02-physics/`.
2. Add exercise sheets under `exercises/02-physics/`.
3. Add a Rust simulation binary under `simulations/rust/src/bin/`.
4. If it leads to a build, create a project directory under `projects/`.

## Execution Rule

Advance only when you can do all of the following without notes:

1. State the key definitions.
2. Derive the governing equations.
3. Solve a nontrivial exercise.
4. Explain the physical meaning of each term.
5. Simulate the system and interpret the output.

## First 90 Days

Focus on:

1. Algebra and precalculus cleanup
2. Calculus I
3. Mechanics
4. Basic electricity and circuit intuition
5. Rust refresh for scientific programming

Start with the files in `curriculum/`.

## Where To Start Now

Begin with:

1. `curriculum/11-phase-00-mathematical-maturity.md`
2. `lecture-notes/01-mathematics/phase-00-mathematical-maturity/00-how-to-study-this-curriculum.md`
3. `lecture-notes/01-mathematics/phase-00-mathematical-maturity/01-propositions-truth-values-and-connectives.md`
4. `exercises/01-mathematics/phase-00-mathematical-maturity/01-propositions-truth-values-and-connectives.md`
5. `PROGRESS.md`
