## Context

Nickel is the right boundary for Trellis policy data: it can describe contracts, defaults, enum domains, and merge behavior while Rust remains responsible for source parsing, filesystem traversal, and execution. The current repo has three policy families with duplicated local predicates and limited cross-field validation:

- `verification-policy/policy.ncl`
- `proof-review/policy.ncl`
- `cairn-policy/contracts.ncl` and `cairn-policy/default.ncl`

The goal is to make malformed policy data impossible to export without moving imperative checks into Nickel.

## Decisions

### 1. Shared contracts are small and policy-neutral

**Choice:** Add a shared Nickel module for common predicates such as non-empty strings, safe relative paths, BLAKE3 hex digests, non-empty arrays, unique IDs, supported severity/stage/mode enums, and membership helpers.

**Rationale:** Most policy files need the same guardrails. A shared module avoids divergence while keeping policy-specific contracts readable.

### 2. Cross-field checks live beside the policy contract

**Choice:** Keep generic helpers in the shared module and express policy-specific reference integrity in each policy contract.

Examples:
- proof-review `apply_recipes[].rule_id` must match `rules[].id`.
- proof-review `apply_recipes[].validation_gate` must match `loop.validation_commands[].id`.
- proof-review `validation_commands[].modes[]` must match `loop.modes[].name`.
- cairn-policy replay groups must reference declared replay cases.
- cairn-policy determinism surfaces must reference declared replay cases and replay groups.
- artifact dependency IDs must reference declared artifacts.

**Rationale:** Reference graphs are policy semantics, not generic data-shape facts.

### 3. Rust stays the imperative shell

**Choice:** Existing Rust checker scripts continue to run `nickel typecheck`, `nickel export`, freshness checks, and fixture checks. Nickel does not read the filesystem or run source scanners at runtime.

**Rationale:** This preserves the current functional-core / imperative-shell split and keeps Nickel as a deterministic configuration admission layer.

### 4. Negative fixtures are first-class evidence

**Choice:** Add malformed Nickel policy fixtures for duplicate IDs, dangling references, unsafe paths, empty required arrays, unsupported enum values, and stale generated JSON.

**Rationale:** The repo requires sad-path coverage. Contract strength is only reviewable when rejected inputs are executable evidence.

## Risks / Trade-offs

- Nickel predicates can become dense if all cross-field logic is inlined. Keep helpers named and small.
- More policy data will fail earlier. This is desired, but diagnostics should include enough field context to fix the policy.
- Shared contracts should not overfit one policy family; policy-specific restrictions remain local.
