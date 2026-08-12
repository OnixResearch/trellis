## Phase 1: Change boundary and traceability

- [x] Sync `proposal.md`, `design.md`, and `specs/tlb-shootdown/spec.md` to the page-local core plus range-lifting design
- [x] Keep one `r[tlb-shootdown.*]` ID per requirement and align task wording to those IDs

## Phase 2: Single-page protocol core (`r[tlb-shootdown.freshness]`, `r[tlb-shootdown.termination]`, `r[tlb-shootdown.initiator-safety]`, `r[tlb-shootdown.self-invalidation]`)

- [x] Add `src/tlb_shootdown.rs` with all-cores-wide `ShootdownRequest { initiator, target_page, acked }` and `TLB_UNMAPPED = 0u64`
- [x] Implement single-page spec helpers for initiation, IPI handling, acknowledgment, pending-count, completion, and freshness
- [x] Implement exec wrappers `initiate_shootdown`, `handle_shootdown_ipi`, `acknowledge_shootdown`, `pending_ack_count`, `all_acknowledged`, and `check_tlb_freshness`, with duplicate IPI/ack calls as no-ops and out-of-bounds indices returning `None`
- [x] Prove initiator self-invalidation and initiation-established ack invariant
- [x] Prove pending-count monotonicity / first-ack decrease and completion iff all acknowledgments are present
- [x] Prove `freshness_from_completion`: completion plus the ack invariant implies stale-entry absence for the target page
- [x] Export the module from `src/lib.rs`

## Phase 3: Disjoint-page isolation and range lifting (`r[tlb-shootdown.isolation]`, `r[tlb-shootdown.range]`)

- [x] Add page-matrix spec helpers for single-page updates, disjoint-page preservation, and `[start, end)` range lifting
- [x] Prove single-page invalidation preserves every different page unchanged
- [x] Prove range invalidation updates every page in `[start, end)` exactly as the single-page rule
- [x] Prove range invalidation preserves pages outside `[start, end)`
- [x] Prove empty-range invalidation is identity
- [x] Prove singleton-range invalidation matches the single-page rule

## Phase 4: Requirements, coverage, and tests

- [x] Add `tlb-shootdown.*` requirements to `docs/requirements.md`
- [x] Add `r[impl]`, `r[verify]`, and `r[depends]` annotations across `src/tlb_shootdown.rs`
- [x] Add positive unit tests for self-invalidation, sequential multi-core completion, disjoint-page preservation helper boundaries, and singleton/empty range boundaries
- [x] Add negative unit tests for invalid core indices and repeated acknowledgment no-op behavior
- [x] Add proptests (256+ cases each) for pending-count monotonicity, random acknowledgment order completion, and freshness after random valid acknowledgment schedules
- [x] Update README module tables and headline counts for the new module / proofs / tests / requirements

## Phase 5: Validation and archive

- [x] Run and capture `rustfmt src/tlb_shootdown.rs`
- [x] Run and capture `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet tlb_shootdown`
- [x] Run and capture `./scripts/verus.sh src/tlb_shootdown.rs`
- [x] Run and capture `./scripts/check-ensures.sh`
- [x] Run and capture `tracey query uncovered` and `tracey query untested`
- [x] Run and capture `openspec_gate` for proposal, design, and tasks on `tlb-shootdown`
- [x] Run and capture `./scripts/verify.sh`
- [x] Add canonical `openspec/specs/tlb-shootdown/spec.md`, force-add it if `.git/info/exclude` hides it, and archive the change under `openspec/changes/archive/` with review-sized evidence files
