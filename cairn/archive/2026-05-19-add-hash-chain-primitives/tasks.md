## Phase 1: Core Commitments

- [x] [serial] Add canonical `chain` OpenSpec/domain requirements and docs anchors for tip commitment, extension/prefix, checkpoint anchoring, and fork detection. ✅ 2m (started: 2026-05-19T20:26:28Z → completed: 2026-05-19T20:28:01Z)
- [x] [depends:chain.tip-commitment] Implement `r[impl]`/`r[verify]` coverage for `chain_tip_spec`, including empty genesis tip and non-empty last-hash cases. ✅ 5m (started: 2026-05-19T20:28:01Z → completed: 2026-05-19T20:33:24Z)
- [x] [depends:chain.extension-prefix] Implement prefix/extension predicates and proofs that valid full chains yield valid prefixes without changing prefix entries. ✅ 5m (started: 2026-05-19T20:28:01Z → completed: 2026-05-19T20:33:24Z)
- [x] [depends:chain.checkpoint-anchor] Implement checkpoint anchoring over `(index, expected_hash)` and prove matching checkpoint hashes authenticate the corresponding valid prefix. ✅ 5m (started: 2026-05-19T20:28:01Z → completed: 2026-05-19T20:33:24Z)
- [x] [depends:chain.fork-detection] Implement first-divergence/fork predicates and prove same-length valid chains with equal tips cannot diverge. ✅ 5m (started: 2026-05-19T20:28:01Z → completed: 2026-05-19T20:33:24Z)

## Phase 2: Append and Shape Primitives

- [x] [depends:core-commitments] Implement batch/fold append specs and proofs by composing the existing append-preserves theorem. ✅ 7m (started: 2026-05-19T20:33:24Z → completed: 2026-05-19T20:40:05Z)
- [x] [depends:chain.log-shape] Add log-shape predicates for contiguous/increasing indices and the chosen term policy; keep shape validity separate from hash validity. ✅ 7m (started: 2026-05-19T20:33:24Z → completed: 2026-05-19T20:40:05Z)
- [x] [depends:chain.log-shape] Add positive tests/proptests for valid shaped hash chains and negative tests for gaps, non-monotone index cases, and term-policy violations. ✅ 7m (started: 2026-05-19T20:33:24Z → completed: 2026-05-19T20:40:05Z)

## Phase 3: Tamper and Domain Boundaries

- [x] [depends:core-commitments] Add tamper-classification predicates/proofs for data, index, term, and predecessor/hash tampering, reusing collision-free and agreement/divergence facts. ✅ 6m (started: 2026-05-19T20:40:05Z → completed: 2026-05-19T20:46:25Z)
- [x] [depends:chain.domain-separated-hash] Add domain-separated hash input contracts for entry, checkpoint, receipt, and snapshot-style domains; document any compatibility wrapper for existing `compute_entry_hash`. ✅ 6m (started: 2026-05-19T20:40:05Z → completed: 2026-05-19T20:46:25Z)
- [x] [depends:chain.runtime-blake3-alignment] Align runtime serialization/BLAKE3 behavior with the spec boundary or explicitly preserve the existing test-mixer bridge as a named trusted/runtime boundary. ✅ 6m (started: 2026-05-19T20:40:05Z → completed: 2026-05-19T20:46:25Z)

## Phase 4: Optional Compact History Overlay

- [x] [parallel] Specify the compact-history overlay (`chain.compact-history-proof`) as optional and base-chain-composing, not a replacement for `chain_valid_spec`. ✅ 3m (started: 2026-05-19T20:46:25Z → completed: 2026-05-19T20:49:35Z)
- [x] [depends:chain.compact-history-proof] If implemented in this slice, add membership/range proof predicates and tests; otherwise leave the requirement as planned/future with no runtime claim. ✅ 3m (started: 2026-05-19T20:46:25Z → completed: 2026-05-19T20:49:35Z)

## Phase 5: Validation and Evidence

- [x] [serial] Run and capture focused chain tests/proptests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, and strict OpenSpec validation for this change. ✅ 10m (started: 2026-05-19T20:49:35Z → completed: 2026-05-19T20:59:36Z)
- [x] [serial] Run final full-tree `./scripts/verify.sh`, reconcile proof-gap inventory if runtime hash bridges changed, then sync/archive the change when all tasks are complete. ✅ 17m (started: 2026-05-19T20:59:36Z → completed: 2026-05-19T21:16:34Z)
