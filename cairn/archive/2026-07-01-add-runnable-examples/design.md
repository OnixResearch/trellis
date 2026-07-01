## Context

The README contains a large module catalog and a short usage snippet, but there is no checked `examples/` directory. Users need copy-pasteable programs that exercise public APIs without requiring Verus knowledge.

## Goals

- Provide small examples that compile with normal `cargo` where ghost code is erased.
- Show positive and negative admission paths where the public API exposes both.
- Keep examples thin and deterministic: no network, time, randomness, or filesystem mutation.
- Link examples from README and user-facing guide docs.

## Non-Goals

- Do not expand the proof surface solely for example coverage.
- Do not add runtime dependencies for examples unless a selected public API already requires them.
- Do not claim external protocol conformance from toy examples.

## Decisions

### Example selection

The initial slice SHALL include examples for widely useful domains: arithmetic/bounds/quorum basics, consensus admission helpers, UCAN/web-of-trust policy admission, R1CS constraint checking, and allocator/storage-style state checks.

### Functional core / imperative shell

Examples SHOULD keep setup logic local and boring. If an example needs reusable logic, extract that logic into a deterministic helper with positive and negative tests before exposing it from the example.

### Validation boundary

The validation gate SHALL at least compile all examples. Examples that can run without nondeterminism SHOULD also be executed under `cargo run --example ...` or an equivalent checked command.
