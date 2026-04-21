## Why

When a kernel changes a page-table mapping, every core that might cache the
old translation must stop using it before the kernel proceeds. Missing one TLB
invalidation can preserve a stale mapping long enough to read freed memory,
write through an old alias, or bypass a permission downgrade.

This repo already models page- and cache-adjacent primitives, but it does not
have a verified cross-core shootdown protocol tying range invalidation,
pending acknowledgments, initiator self-invalidation, and completion safety
into one exact executable model.

## What Changes

- Add `src/tlb_shootdown.rs` with a finite executable model:
  - `TlbEntry { vpn, ppn }`
  - `CoreTlb { entries: Vec<TlbEntry> }`
  - `ShootdownRequest { initiator, vpn_start, vpn_end, new_ppn, pending_acks, completed }`
- Add exact shootdown APIs:
  - `initiate_shootdown(..., core_count, initiator_tlb) -> (ShootdownRequest, CoreTlb)`
  - `initiate_single_shootdown(..., core_count, initiator_tlb) -> (ShootdownRequest, CoreTlb)`
  - `handle_shootdown_ipi(core, request, tlb) -> CoreTlb`
  - `acknowledge_shootdown(core, request, tlb) -> ShootdownRequest`
  - `all_acknowledged`, `check_tlb_freshness`, and `safe_to_proceed`
- Keep `new_ppn` as request metadata carried alongside the invalidation so the
  caller can associate the shootdown with its remap target; this verified model
  does not update TLB entries to `new_ppn`, it only removes stale ones.
- Scope this landing to at most one outstanding `ShootdownRequest` at a time;
  concurrent request identity/routing is intentionally out of scope.
- Prove exact range freshness and completion contracts in Verus, with
  disjoint-range non-interference covered by regression tests.
- Add Tracey requirements in `docs/requirements.md` and sync canonical/change
  specs for the landed model.

## Capabilities

### New Capabilities

- `tlb-shootdown.freshness` — completed shootdowns leave no in-range TLB entry in a request-compatible per-core TLB slice
- `tlb-shootdown.termination` — pending acknowledgments decrease only after fresh invalidation and empty pending means complete
- `tlb-shootdown.initiator-safety` — kernel may proceed only after completion and freshness across that request-compatible slice
- `tlb-shootdown.self-invalidation` — initiation invalidates the initiator immediately
- `tlb-shootdown.isolation` — disjoint shootdown ranges preserve unrelated entries
- `tlb-shootdown.range` — IPI handling removes exactly the targeted VPN interval

## Impact

- **Files**: new `src/tlb_shootdown.rs`, `src/lib.rs`, `docs/requirements.md`,
  `README.md`, `openspec/specs/tlb-shootdown/spec.md`, and change-local
  spec/tasks/design
- **APIs**: finite per-core TLB model plus exact shootdown helpers listed above
- **Dependencies**: existing `vstd` and `proptest`
- **Validation**: targeted cargo/verus for `tlb_shootdown`, then full-tree
  cargo/clippy/verus/check-ensures/tracey before archive
