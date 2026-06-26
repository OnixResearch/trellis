## Context

This change lands a small verified TLB shootdown core, not a full MMU or
page-table model. The reusable proof boundary is: a request-local
acknowledgment bitmap plus page-local cached-translation state.

The design intentionally mirrors repo patterns like `mesi.rs` and
`store_buffer.rs`: a small immutable exec surface linked to recursive spec
helpers, plus proof lemmas that lift local update facts into global
protocol invariants.

## Goals / Non-Goals

**Goals:**
- Model a single-page shootdown request with per-request acknowledgments
- Invalidate the initiator's cached translation during initiation
- Invalidate one target core's cached translation during IPI handling
- Record acknowledgments monotonically
- Expose completion exactly when all acknowledgments are present
- Prove completion implies stale-entry absence for the target page
- Lift single-page invalidation to disjoint-page isolation and range coverage
- Add runtime tests and proptests for concrete request handling

**Non-Goals:**
- Interrupt delivery timing, batching, or timeout/failure recovery
- Page-table walk semantics or allocator/page-free behavior
- Multi-page request scheduling beyond a pointwise range helper
- TLB internals such as associativity, ASIDs, or replacement

## Data Model

### 1. Page-local TLB state

**Choice:** Model one target page's cached translation state as `Vec<u64>`
indexed by core. `TLB_UNMAPPED = 0u64` means the stale cached translation
is absent.

**Rationale:** The proof obligation is page-local. A single-page vector
lets the exec API stay simple and makes self-invalidation / IPI handling a
single-index update.

### 2. Request-local acknowledgment bitmap

**Choice:** `ShootdownRequest { initiator, target_page, acked: Vec<bool> }`.
The bitmap is all-cores-wide, indexed by core ID. The initiator starts
acknowledged; every other core starts unacknowledged.

**Rationale:** A bitmap matches repo sequence-heavy proof style better than
`Set<u32>`. It also makes pending-count monotonicity and completion scans
straightforward.

### 3. Range as a proof/spec lifting layer

**Choice:** Model disjoint-page isolation and range invalidation over a
page-indexed matrix `Seq<Seq<u64>>`. Range helpers apply the single-page
update pointwise and prove outside-range preservation.

**Rationale:** This keeps the executable surface small while still giving a
precise theorem for `[start, end)` coverage.

## API Shape

### Exec functions

- `initiate_shootdown(&Vec<u64>, initiator, target_page) -> Option<(ShootdownRequest, Vec<u64>)>`
- `handle_shootdown_ipi(&Vec<u64>, core) -> Option<Vec<u64>>`
- `acknowledge_shootdown(&ShootdownRequest, core) -> Option<ShootdownRequest>`
- `pending_ack_count(&ShootdownRequest) -> usize`
- `all_acknowledged(&ShootdownRequest) -> bool`
- `check_tlb_freshness(&Vec<u64>) -> bool`

Invalid indices return `None` rather than panicking. Duplicate
`handle_shootdown_ipi` calls are idempotent because invalidating an already
absent entry is a no-op, and duplicate `acknowledge_shootdown` calls are a
no-op because setting an already-true bitmap bit leaves the request state
unchanged.

### Spec / proof surface

- single-page update specs for initiation, IPI handling, acknowledgment,
  pending-count, and completion
- `ack_implies_invalidated` invariant over page-local TLB state plus ack bitmap
- disjoint-page matrix update helpers for isolation
- range update helpers for coverage, empty-range identity, and singleton
  equivalence

## Proof Plan

### 1. Local update facts first

Prove small lemmas for:
- initiator self-invalidation
- single-core IPI invalidates exactly one core entry
- acknowledgment flips exactly one bitmap bit
- pending-count decreases only on a first acknowledgment

These are the lowest-risk bridges between exec and spec.

### 2. Ack invariant

Define an invariant:

- `acked[i] ==> page_tlbs[i] == TLB_UNMAPPED`

Show:
- initiation establishes it for the initiator,
- IPI handling preserves it for all previously acknowledged cores,
- acknowledgment preserves it when the caller acknowledges only after a
  successful invalidation step for that same core.

### 3. Completion implies freshness

Use recursive scans over the ack bitmap to prove:
- if all acknowledgments are present,
- and the ack invariant holds,
- then every targeted core's page-local cached translation is absent.

This is the central freshness theorem.

### 4. Disjoint-page isolation and range coverage

Lift the single-page update to a page-indexed matrix:
- updating target page `p` leaves every `q != p` unchanged,
- range updates modify exactly pages in `[start, end)`,
- empty range is identity,
- singleton range matches the single-page helper.

## Risks / Trade-offs

**[No page-table result model]** -> accepted. The module proves stale-entry
absence, not the semantics of the new translation.

**[Request-local acknowledgments]** -> accepted. This makes concurrent
isolation structural for disjoint pages; that is the intended theorem.

**[Small exec surface, richer proof surface]** -> accepted. Range and
isolation are easier and clearer as lifting lemmas than as heavier runtime
APIs.
