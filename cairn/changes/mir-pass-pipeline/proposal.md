## Why

The rustc-dev-guide MIR passes chapter describes a staged pipeline with intermediate MIR dialects, validation gates, in-place passes, optimization-level gating, and `Steal` ordering constraints. Trellis has individual pass primitives, but not a finite lifecycle model for MIR phase transitions.

This change defines a MIR pass-pipeline primitive that can make future compiler-pipeline evidence more explicit about when a pass is valid, what it may mutate, and which intermediate results may be read or stolen.

## What Changes

- Add a `mir-pass-pipeline` Cairn spec.
- Plan a `src/mir_pass_pipeline.rs` Verus module with finite phase, pass, query, and artifact-state models.
- Define phase order, pass compatibility, dialect validation, in-place mutation boundaries, optimization-level admission, and read-before-steal constraints.
- Require positive and negative tests for valid pipelines, out-of-order phases, invalid pass registration, stale reads after steal, and disabled optimization passes.

## Impact

- **Files**: new Cairn change package now; later implementation should touch `src/mir_pass_pipeline.rs`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include both accepted pipeline traces and rejected malformed traces.
- **Boundaries**: no claim about actual rustc query execution, incremental compilation, pass semantic correctness, or optimization soundness unless separately proven.
