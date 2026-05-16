## Why

The GPU primitive set now admits finite launch, layout, and program-shape envelopes, but programs that write output buffers still lack a verified structural contract for non-overlapping writes. This change specifies the next slice: finite output-write regions whose admitted program instances cannot alias each other inside the model.

## What Changes

- Specify finite GPU output-region envelopes and pairwise write-disjointness admission.
- Model elementwise and tiled-output write domains as structural program properties.
- Keep runtime CUDA scheduling, memory consistency, and performance behavior out of scope.

## Scope

In scope: requirements, design notes, and implementation tasks for this GPU program-semantics slice.

Out of scope:
- CUDA runtime scheduling or driver behavior.
- Device allocation provenance or pointer validity beyond declared finite buffers.
- Performance, occupancy, cache/coalescing, or numerical-kernel correctness claims unless explicitly named by a future requirement.

## Verification

Validate this OpenSpec package, then implement the finite Rust/Verus contracts with Tracey `r[impl]` / `r[verify]` coverage, focused GPU tests, focused Verus, clippy, Tracey uncovered/untested checks, and the full repository verification gate.
