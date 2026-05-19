## Context

The existing Raft core already contains verified boolean helpers for the OpenRaft-inspired operational family:

- `raft_operational_log_pointers_valid{,_spec}`
- `raft_io_progress_valid{,_spec}` and `raft_io_progress_advance_admit{,_spec}`
- `raft_vote_log_io_ordered{,_spec}`
- `raft_effective_membership_valid{,_spec}` and rollback helpers
- `raft_replication_session_matches{,_spec}` and progress-update helpers
- `raft_read_linearizer_ready{,_spec}`

Those helpers are correct as boolean cores, but they do not expose the deterministic first-failure metadata style used by the newer Raft model/node/applied/client envelope slices. This change narrows to metadata/reporting around the existing finite model rather than adding new OpenRaft semantics.

## Goals / Non-Goals

**Goals:**
- Define a small copyable admission-result surface for OpenRaft-inspired operational checks.
- Keep result-code ordering deterministic and documented per primitive family.
- Prove that OK results imply the existing boolean predicate and that representative rejection results preserve/no-mutate the relevant input state.
- Add positive and negative tests that exercise first-failure ordering.

**Non-Goals:**
- No storage durability, fsync, crash-recovery, network quorum, leader lease, clock, or production OpenRaft implementation claims.
- No replacement of the existing boolean predicates.
- No broad Raft transition-engine or cluster-runtime integration.

## Decisions

### 1. Reuse boolean predicates as the semantic core

**Choice:** Admission-result specs should be thin deterministic classifiers over the existing boolean predicates and their component checks.

**Rationale:** This keeps the slice small, reviewable, and aligned with already-verified OpenRaft-inspired helpers.

**Alternative:** Replace existing predicates with richer result types. Rejected because it would increase API churn and obscure the already-green proof surface.

### 2. First-failure ordering is part of the contract

**Choice:** Each classifier checks fields in a documented stable order and reports the first modeled violation.

**Rationale:** Deterministic metadata is useful only if future callers/tests can rely on stable failure precedence.

**Alternative:** Return an unordered set/list of all failures. Rejected because it is heavier to prove and unnecessary for the current finite primitive layer.

### 3. Keep constructors permissive and validity in admission helpers

**Choice:** Copyable metadata structs remain raw data carriers; public helpers decide validity via specs/ensures rather than requiring smart constructors.

**Rationale:** This matches the surrounding Raft primitives and keeps invalid test cases easy to construct.

## Risks / Trade-offs

**Result-code proliferation** → Use a compact primitive-family code set and document non-claims in requirement prose.

**Spec overclaiming** → Keep every requirement phrased as finite metadata/admission only, and explicitly exclude OpenRaft runtime behavior.

**Tracey drift** → Add `r[impl]` and `r[verify]` tags to every new public helper/proof/test surface and capture uncovered/untested evidence before archive.

## Validation Plan

1. `nix develop -c openspec validate add-raft-openraft-admission-metadata --strict`
2. Focused Rust tests: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c cargo test --quiet raft_consensus_core`
3. `nix develop -c ./scripts/verus.sh src/lib.rs`
4. `nix develop -c tracey query uncovered` and `nix develop -c tracey query untested`
5. `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c ./scripts/verify.sh`
6. Sync canonical specs and archive once all tasks are complete.
