## Why

Reduction kernels are common GPU programs. The current block-reduction admission checks work capacity and shared memory, but it does not specify the structural reduction-tree invariants that make staged reductions well formed.

## What Changes

- Specify finite reduction-tree shape admission for block-local and multi-block reductions.
- Model active lane counts, halving/rounding stages, and output cardinality.
- Exclude associativity/commutativity of user operators and floating-point numerical correctness.

## Scope

In scope: requirements, design notes, and implementation tasks for this GPU program-semantics slice.

Out of scope:
- CUDA runtime scheduling or driver behavior.
- Device allocation provenance or pointer validity beyond declared finite buffers.
- Performance, occupancy, cache/coalescing, or numerical-kernel correctness claims unless explicitly named by a future requirement.

## Verification

Validate this OpenSpec package, then implement the finite Rust/Verus contracts with Tracey `r[impl]` / `r[verify]` coverage, focused GPU tests, focused Verus, clippy, Tracey uncovered/untested checks, and the full repository verification gate.
