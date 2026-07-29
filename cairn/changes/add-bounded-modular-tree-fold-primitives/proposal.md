## Why

Kamacite is preparing a CPU-only Bend 0.2.38 and HVM2 2.0.22 evaluation. That pilot needs an independent Rust oracle for a pure balanced tree reduction over declared `u24` modulo semantics.

Trellis already provides generic algebraic predicates, sequence counting, wrapping `u64` arithmetic, and proof-evidence exports. It does not provide a `u24` domain, a balanced reduction model, or a verified executable oracle for this workload. Without that layer, three matching backends remain useful differential evidence but do not establish that the Rust oracle implements the declared mathematical result.

## What Changes

- Add named `u24` domain constants, normalization, validity, and wrapping-add primitives.
- Prove normalization closure and identity, wrapping-add closure, identity, commutativity, and associativity.
- Define a finite non-empty power-of-two leaf profile with an explicit caller-provided leaf limit.
- Add an optimized iterative Rust sum oracle and a balanced pair-reduction reference implementation.
- Prove both executable paths refine one mathematical modular-sum specification.
- Prove balanced reduction equals the ordered sequential fold for admitted inputs.
- Prove exact abstract combine and round counts for the admitted balanced schedule.
- Export BLAKE3-bound proof evidence and a downstream handoff profile with explicit non-claims.
- Add positive, negative, boundary, and property tests plus full Verus and Tracey coverage.

## Impact

- **Surfaces**: new `u24` and balanced-reduction verified modules, public exports, requirements, proof functions, runtime tests, proof-evidence rows, module catalogs, and downstream handoff documentation.
- **Consumers**: Kamacite may use the verified executable oracle and proof artifact after exact pinning and local receipt validation. Other consumers may reuse only the declared local laws.
- **References**: Bend and HVM2 remain pinned external design and conformance references. Trellis does not vendor or execute either project.
- **Claims**: proofs cover Trellis-owned source, mathematical models, executable refinements, and explicit preconditions. They do not prove Bend, HVM2, generated C, a host compiler, benchmarks, effect absence, or external interaction counts.
