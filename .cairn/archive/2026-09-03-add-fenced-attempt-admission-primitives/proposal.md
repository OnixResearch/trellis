# Change: Add fenced-attempt admission primitives

## Why

Trellis verifies fencing tokens, lease ownership, durable job admission, idempotency, retry bounds, and rejection preservation in separate modules. It does not provide one finite model for a durable remote attempt that combines current job and attempt identity, fence freshness, event idempotence, phase transition, result linkage, and output-admission authority.

Mantle has this exact pure decision seam in its remote-attempt core. A product-neutral Trellis model can prove the reusable safety properties while Mantle keeps ownership of runtime semantics and adapter correctness.

## What Changes

- Add a `fenced_attempt_admission` module with finite phase, event, fence, history, authorization, linkage, and decision types.
- Add spec and executable functions for fence classification, event classification, transition admission, and report application.
- Prove stale and mismatched fence rejection, terminal-state closure, identical-event idempotence, conflicting-event rejection, completion linkage, representable fence advance, and rejected-state preservation.
- Add positive, negative, exhaustive finite, and property tests for the executable model.
- Export proof evidence with exact source, requirements, verifier, assumptions, trusted boundaries, and non-claims.
- Provide a consumer mapping profile for Mantle without importing Mantle types or claiming implementation equivalence.

## Non-Goals

- Network delivery, durable persistence, cryptography, clocks, scheduling, worker execution, sandboxing, liveness, or availability proofs.
- A Mantle runtime dependency or a workspace-relative product dependency.
- Proof of all future consumer states or report variants.
- Treating passing consumer parity tests as a theorem about consumer source.

## Impact

- **Affected spec:** `fenced-attempt-admission`
- **Affected code:** one verified module, `src/lib.rs`, requirements and module indexes, proof evidence, and focused tests
- **Consumers:** Mantle can project admitted remote-attempt facts into the model through its own fail-closed adapter
- **Testing:** direct `proof fn` artifacts, positive and negative tests, finite matrix tests, property tests, full relevant Verus checks, Tracey, and Cairn gates
