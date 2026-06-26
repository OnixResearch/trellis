## Context

Unison content-addresses definitions by hashing normalized syntax trees with dependency identities instead of names. `verified-logic` can model the structural side of that idea without claiming real compiler hashing or cryptographic collision resistance.

## Goals / Non-Goals

**Goals:**
- Add finite definition, name-binding, dependency, and recursive-group envelopes.
- Prove deterministic admission and closure completeness facts over declared hashes.
- Produce reusable primitives for later remote-computation, typed-durable-reference, and semantic-refactoring changes.

**Non-Goals:**
- Real parser/typechecker/compiler correctness.
- Cryptographic security of any hash function.
- Runtime execution or network synchronization.

## Decisions

### 1. Hashes are opaque finite identifiers

**Choice:** Model hashes as bounded byte/vector identifiers with equality and declared membership.

**Rationale:** This lets Verus prove structural closure and aliasing facts while keeping cryptographic properties explicit trust boundaries.

### 2. Names are aliases, not identity

**Choice:** Name bindings point to definition hashes and are checked separately from definition identity.

**Rationale:** This captures the Unison distinction that renaming should not mutate content identity.

### 3. Closure before sync/execution

**Choice:** This slice only reports missing dependencies and closure completeness.

**Rationale:** Remote fetching, cache install, and execution eligibility require ability/resource checks and belong in later changes.

## Risks / Trade-offs

**Hash overclaiming** → State that hashes are finite identifiers and do not prove collision resistance.

**Graph proof churn** → Use fuel-bounded scans over finite definition tables and small missing-dependency witnesses.

**Recursive group ambiguity** → Keep canonical order deterministic and explicitly reject out-of-range member references.
