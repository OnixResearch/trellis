## Phase 1: Specification

- [x] [serial] Define preservation-matrix and expanded step-preservation requirements, scenarios, proposal, design, and task plan. r[sel4-invariants.preservation-matrix] r[sel4-invariants.step-preservation-expansion]

## Phase 2: Implementation

- [x] [depends:spec] Add a preservation matrix for modeled seL4 steps with preconditions, mutated fields, unchanged fields, consumed sub-invariants, and evidence links. r[sel4-invariants.preservation-matrix]
- [x] [depends:spec] Refactor preservation proofs into local frame lemmas for unchanged object refs, legal TCB states, scheduler consistency, untyped ownership, ASID/VSpace facts, and IRQ uniqueness where the step does not mutate those domains. r[sel4-invariants.preservation-matrix]
- [x] [depends:spec] Strengthen endpoint/TCB/ready-queue preservation wrappers for accepted send/rendezvous/block/wake slices already modeled in local modules. r[sel4-invariants.step-preservation-expansion]
- [x] [depends:spec] Add rejected/no-op boundary helpers for invalid references, mismatched endpoint/TCB state, and out-of-scope step requests. r[sel4-invariants.step-preservation-expansion]
- [x] [depends:spec] Add positive tests for accepted modeled transitions and negative tests for rejected/no-op boundaries. r[sel4-invariants.step-preservation-expansion]

## Phase 3: Validation

- [x] [depends:implementation] Run focused seL4 cargo tests and the smallest relevant `./scripts/verus.sh` invocation, escalating to `./scripts/verus.sh src/lib.rs` before archive. r[sel4-invariants.step-preservation-expansion]
- [x] [depends:implementation] Run clippy, Tracey uncovered/untested checks, and Cairn validate/gates after requirement IDs and implementation markers are added. r[sel4-invariants.preservation-matrix] r[sel4-invariants.step-preservation-expansion]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. r[sel4-invariants.preservation-matrix]
