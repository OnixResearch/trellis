# Invariant Surface Catalog

<!-- generated-by: scripts/check-invariant-surface-catalog.rs; source: local deterministic facts -->

This catalog exposes objective local facts for selected public invariant, well-formedness, preservation, soundness, and admission-boundary surfaces. Labels are source-fact classifications, not proof-quality scores. `test` evidence is intentionally distinct from `proof` evidence. Run `./scripts/check-invariant-surface-catalog.rs --write` to refresh and `./scripts/check-invariant-surface-catalog.rs` to validate.

| Module | Requirement | Surface | Labels | Evidence | Trusted boundary adjacency | Source |
|---|---|---|---|---|---|---|
| `cache_hierarchy` | `hierarchy.inclusive-invariant` | `inclusive_invariant_spec` | state-invariant, preservation | proof+test | no | [`src/cache_hierarchy.rs`](../src/cache_hierarchy.rs) |
| `consensus_fast_path` | `consensus.fast-path.quorum` | `finite_membership_intersection` | state-invariant, soundness | proof | no | [`src/consensus_fast_path/proofs/arithmetic.rs`](../src/consensus_fast_path/proofs/arithmetic.rs) |
| `constant_folding` | `constfold.soundness` | `fixed_point_path_sound_top_free` | soundness, well-formedness | proof+test | no | [`src/constant_folding.rs`](../src/constant_folding.rs) |
| `control_flow_graph` | `control-flow-graph.well-formed` | `well_formed_cfg_spec` | well-formedness | proof+test | no | [`src/control_flow_graph.rs`](../src/control_flow_graph.rs) |
| `cryptoshedding` | `cryptoshedding.chain-recoverability` | `recoverable_spec` | state-invariant, soundness | proof+test | no | [`src/cryptoshedding.rs`](../src/cryptoshedding.rs) |
| `ghost_history` | `ghost-history.single-vote-per-term` | `single_vote_per_term_spec` | state-invariant | proof+test | no | [`src/ghost_history.rs`](../src/ghost_history.rs) |
| `last_applied` | `last-applied.advance` | `advance_preserves_invariant` | preservation | proof+test | no | [`src/last_applied.rs`](../src/last_applied.rs) |
| `liveness_analysis` | `liveness.equations` | `equations_hold_spec` | state-invariant, catalog-only | test | no | [`src/liveness_analysis.rs`](../src/liveness_analysis.rs) |
| `raft_step` | `raft-step.preserves` | `step_preserves_spec` | preservation | proof | no | [`src/raft_step.rs`](../src/raft_step.rs) |
| `scheme_logic` | `scheme-logic.reader-token-span` | `child_containment_is_transitive` | state-invariant, soundness | proof | no | [`src/scheme_logic/proofs/reader.rs`](../src/scheme_logic/proofs/reader.rs) |
| `sel4_invariants` | `sel4-invariants.preservation-matrix` | `kernel_state_unchanged_spec` | state-invariant, preservation | proof+test | no | [`src/sel4_invariants.rs`](../src/sel4_invariants.rs) |
| `ucan_proof_traversal` | `ucan-proof-traversal.transcript-consistency` | `traversal_transcript_valid_spec` | well-formedness, admission-boundary | proof+test | no | [`src/ucan_proof_traversal.rs`](../src/ucan_proof_traversal.rs) |
| `verification_frontier` | `verification-frontier.outcome-invariant-bundle` | `admission_is_proof_evidence_spec` | admission-boundary, soundness | proof+test | no | [`src/verification_frontier.rs`](../src/verification_frontier.rs) |
| `wasm_runtime_bounds` | `wasm-runtime-bounds.segmentation` | `segmented_steps_match_uninterrupted` | state-invariant, soundness | proof | no | [`src/wasm_runtime_bounds/proofs/control.rs`](../src/wasm_runtime_bounds/proofs/control.rs) |
