# Proof Review Candidates

Deterministic prefilter for focused VibeThinker proof critique. VibeThinker output is advisory; validation remains cargo, Verus, Tracey, Cairn, and repo checkers.

> Note: legacy `openspec/changes` exists but is ignored by this native Cairn scanner. Native `cairn/changes` is the authoritative lifecycle input; legacy cleanup/migration is explicit.

## Active Cairn Changes

- `proof-review-beam-link`: 6 unchecked task(s); referenced source paths: src/beam_link.rs, src/lib.rs

## Ranked Candidates

| Rank | Score | Path | Top reasons |
| ---: | ---: | --- | --- |
| 1 | 1025 | `src/beam_link.rs` | referenced by active Cairn change `proof-review-beam-link`; source file is changed in the worki… |
| 2 | 835 | `src/ucan_signature.rs` | trusted or weakened proof marker; trusted or weakened proof marker; trusted or weakened proof m… |
| 3 | 775 | `src/lib.rs` | referenced by active Cairn change `proof-review-beam-link`; active-prefix or bounded-length sem… |
| 4 | 635 | `src/ucan_did.rs` | trusted or weakened proof marker; trusted or weakened proof marker; trusted or weakened proof m… |
| 5 | 255 | `src/beam_process.rs` | empty proof body for `beam_transition_is_forward` with nontrivial postcondition; empty proof bo… |
| 6 | 255 | `src/chain.rs` | empty proof body for `hash_deterministic` with nontrivial postcondition; empty proof body for `… |
| 7 | 255 | `src/client_session.rs` | empty proof body for `register_advances` with nontrivial postcondition; empty proof body for `a… |
| 8 | 255 | `src/commit_notify.rs` | empty proof body for `no_advance_zero` with nontrivial postcondition; empty proof body for `sin… |
| 9 | 255 | `src/concurrent_composition.rs` | empty proof body for `rwlock_write_excludes_readers` with nontrivial postcondition; empty proof… |
| 10 | 255 | `src/config_change.rs` | empty proof body for `no_change_during_joint` with nontrivial postcondition; empty proof body f… |
| 11 | 255 | `src/consistency_transactional.rs` | empty proof body for `read_committed_implies_read_uncommitted` with nontrivial postcondition; e… |
| 12 | 255 | `src/consistent_snapshot.rs` | empty proof body for `cut_le_reflexive` with nontrivial postcondition; empty proof body for `cu… |

## Candidate Details

### `src/beam_link.rs` — score 1025

Showing top 3 of 3 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **active-cairn-change** (+750): `src/beam_link.rs` — 6 unchecked task(s) remain in the active change.
- **worktree-changed-source** (+250): `src/beam_link.rs` — Changed verified source is immediately actionable for proof critique.
- **active-prefix-semantics** (+25): `src/beam_link.rs`:30 — Review boundary lemmas and sad-path tests for zero, max, inactive, out-of-range, and fuel exhaustion cases.

### `src/ucan_signature.rs` — score 835

Showing top 6 of 13 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:254 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:263 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:275 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:289 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:303 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_signature.rs`:315 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.

### `src/lib.rs` — score 775

Showing top 2 of 2 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **active-cairn-change** (+750): `src/lib.rs` — 6 unchecked task(s) remain in the active change.
- **active-prefix-semantics** (+25): `src/lib.rs`:106 — Review boundary lemmas and sad-path tests for zero, max, inactive, out-of-range, and fuel exhaustion cases.

### `src/ucan_did.rs` — score 635

Showing top 6 of 11 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **untrusted-boundary-marker** (+100): `src/ucan_did.rs`:623 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_did.rs`:631 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_did.rs`:640 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **untrusted-boundary-marker** (+100): `src/ucan_did.rs`:649 — found `#[verifier::external_body]`; confirm classification and avoid expanding trust.
- **empty-nontrivial-proof** (+35): `src/ucan_did.rs`:142 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/ucan_did.rs`:148 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/beam_process.rs` — score 255

Showing top 6 of 9 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:290 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:300 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:305 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:311 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:317 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/beam_process.rs`:323 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/chain.rs` — score 255

Showing top 6 of 18 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/chain.rs`:42 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/chain.rs`:111 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/chain.rs`:117 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/chain.rs`:123 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/chain.rs`:129 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/chain.rs`:210 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/client_session.rs` — score 255

Showing top 6 of 22 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/client_session.rs`:261 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/client_session.rs`:266 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/client_session.rs`:271 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/client_session.rs`:276 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/client_session.rs`:281 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/client_session.rs`:284 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/commit_notify.rs` — score 255

Showing top 6 of 10 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:248 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:254 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:261 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:270 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:276 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/commit_notify.rs`:283 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/concurrent_composition.rs` — score 255

Showing top 6 of 26 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:95 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:107 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:120 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:132 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:143 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/concurrent_composition.rs`:154 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/config_change.rs` — score 255

Showing top 6 of 15 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/config_change.rs`:246 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/config_change.rs`:252 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/config_change.rs`:258 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/config_change.rs`:264 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/config_change.rs`:270 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/config_change.rs`:277 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/consistency_transactional.rs` — score 255

Showing top 6 of 10 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:235 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:242 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:251 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:260 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:269 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistency_transactional.rs`:287 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

### `src/consistent_snapshot.rs` — score 255

Showing top 6 of 15 finding(s); score caps repeated findings for the same rule at 6 occurrence(s).

- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:197 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:203 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:210 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:217 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:226 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.
- **empty-nontrivial-proof** (+35): `src/consistent_snapshot.rs`:233 — Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.

## VibeThinker Prompt Shape

Use `max_tokens=16384`. Packets include a base task, a strict output contract, and rule-specific focus bullets derived from the top scanner findings.
