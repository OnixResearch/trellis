## Why

Trellis has concrete dataflow-style modules such as liveness and constant propagation, but the rustc-dev-guide describes a reusable MIR dataflow framework: finite lattices, forward/backward direction, statement and terminator effects, per-edge effects, before effects, fixed-point iteration, and result cursors or visitors.

This change defines a small framework primitive so future analyses can share proof obligations instead of re-proving convergence and transfer-shape facts per module.

## What Changes

- Add a `mir-dataflow-framework` Cairn spec.
- Add a `src/finite_flow/` component with finite lattice and analysis specs, checked helpers, and a narrow public facade.
- Define direction, bottom/top, join-semilattice laws, monotone transfer, before/primary/per-edge effects, and fixed-point convergence obligations.
- Define result inspection metadata for block entries and intra-block locations.
- Require positive and negative tests for lattice law admission, invalid transfer functions, non-monotone effects, malformed edge effects, and convergence-budget rejection.

## Impact

- **Files**: implementation touches `src/finite_flow/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include happy-path and sad-path evidence for finite analyses and malformed analysis definitions.
- **Boundaries**: no claim about a specific borrow checker, move analysis, initializedness analysis, or whole rustc dataflow engine unless a later instantiated primitive proves that analysis separately.
