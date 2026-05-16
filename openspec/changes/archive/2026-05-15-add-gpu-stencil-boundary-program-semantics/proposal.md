## Why

Many real GPU kernels are stencil-shaped: each output reads a bounded neighborhood. The existing GPU program-shape primitives cover output coverage but not the guarded boundary checks that make stencil reads structurally safe.

## What Changes

- Specify finite stencil radius/window admission for GPU programs.
- Model interior points, boundary guards, and read-window in-bounds facts.
- Exclude cache/coalescing/performance and numerical stencil correctness.

## Scope

In scope: requirements, design notes, and implementation tasks for this GPU program-semantics slice.

Out of scope:
- CUDA runtime scheduling or driver behavior.
- Device allocation provenance or pointer validity beyond declared finite buffers.
- Performance, occupancy, cache/coalescing, or numerical-kernel correctness claims unless explicitly named by a future requirement.

## Verification

Validate this OpenSpec package, then implement the finite Rust/Verus contracts with Tracey `r[impl]` / `r[verify]` coverage, focused GPU tests, focused Verus, clippy, Tracey uncovered/untested checks, and the full repository verification gate.
