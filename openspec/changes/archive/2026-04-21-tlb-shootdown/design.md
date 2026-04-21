## Context

The goal is a small exact shootdown model, not a full MMU, page-table walk, or
IPI-delivery subsystem. The executable surface should be finite and easy to run
in tests, while the proof surface should state the real safety boundary:
completion means no core still holds an in-range translation.

## Pre-implementation Repo Audit

Against the pre-change baseline, this domain was net-new:

- `src/lib.rs` had no `pub mod tlb_shootdown;`
- `docs/requirements.md` had no `r[tlb-shootdown.*]` requirements
- `README.md` had no `tlb_shootdown` module row
- `openspec/specs/` had no canonical `tlb-shootdown/spec.md`

## Goals

- Model per-core TLB contents as exact finite data
- Model shootdown state with exact pending-ack tracking
- Self-invalidate the initiator during initiation
- Remove pending acknowledgments only after a core is locally fresh
- Expose exact `all_acknowledged`, `check_tlb_freshness`, and `safe_to_proceed`
- Support both arbitrary ranges and single-VPN shootdowns
- Cover disjoint-range isolation with executable regression tests

## Non-Goals

- Page-table walk semantics or ASID/tag matching
- IPI transport, delivery latency, or timeout recovery
- Multiple concurrently outstanding shootdown requests; this landing assumes an external single-request-at-a-time discipline, and request identity/routing for overlapping requests is out of scope
- TLB replacement policy or multi-level TLB structure
- Lazy or deferred invalidation optimizations

## Decisions

### 1. Exact finite TLB model

Use:

- `TlbEntry { vpn, ppn }`
- `CoreTlb { entries: Vec<TlbEntry> }`

The per-core TLB slice uses the convention: slice index equals core ID and each
core appears exactly once. Requirement-level helper claims are stated over a
request-compatible slice relation:

- `request.initiator < tlbs.len()`
- `pending_acks` is duplicate-free
- no `pending_acks` entry equals `initiator`
- every `pending_acks` entry is `< tlbs.len()`

All freshness claims in this landing quantify over slices satisfying that
relation.

Rationale: shootdown safety only needs exact membership of cached VPNs. A
`Vec<TlbEntry>` keeps the executable surface simple while still letting specs
state exact range removal over the sequence view.

### 2. Request state is explicit and local

Use:

- `ShootdownRequest { initiator, vpn_start, vpn_end, new_ppn, pending_acks, completed }`

`core_count` is a constructor input, not a stored request field. The constructor
uses it once to build `pending_acks` as the duplicate-free sequence of remote
cores `0..core_count` excluding `initiator`. Requirement-level remote-core set
claims use the well-formed-request boundary: `pending_acks` is duplicate-free,
contains no initiator entry, every pending core index is within the supplied
TLB slice, and `completed == pending_acks.is_empty()`. `acknowledge_shootdown`
preserves that shape because it only removes a still-pending core and never
inserts new ones.

Rationale: this keeps the request surface minimal while still making the
termination measure exact: successful acknowledgments strictly shrink the
remote-core pending set.

### 3. Range invalidation is the core transition

`initiate_shootdown(..., core_count, initiator_tlb)` and
`initiate_single_shootdown(..., core_count, initiator_tlb)` return
`(ShootdownRequest, CoreTlb)`: the constructed request plus the initiator's
self-invalidated TLB. `handle_shootdown_ipi(core, request, tlb) -> CoreTlb`
then applies one exact transformation: remove every `TlbEntry` whose `vpn` lies
in `[vpn_start, vpn_end)` and preserve all others in order.

Rationale: freshness and isolation both reduce to exact range removal, and the
returned `(request, initiator_tlb_after)` pair makes the self-invalidation
boundary explicit in exec code.

### 4. Acknowledgment is freshness-gated

`acknowledge_shootdown(core, request, tlb)` removes `core` from `pending_acks`
only when:

- `core` is still pending, and
- `tlb` is already fresh for the request range.

Otherwise it is a no-op. On the successful path, the returned request removes
that one core from `pending_acks` and then sets `completed` from the resulting
emptiness predicate. On the no-op path, `pending_acks` stays unchanged and the
returned `completed` bit is recomputed to the same `pending_acks.is_empty()`
value.

Rationale: this encodes the protocol safety boundary directly in the exec
surface. The request cannot reach `completed` unless every removed core has
already invalidated locally.

### 5. Completion and initiator safety are derived executable predicates

Expose three read-only helpers:

- `all_acknowledged` — exact predicate `pending_acks.is_empty()`
- `check_tlb_freshness` — exact freshness check for a request-compatible per-core TLB slice
- `safe_to_proceed` — `request.completed && check_tlb_freshness(...)`

The request also carries `new_ppn` as caller-visible remap metadata only. This
model proves invalidation safety, not post-shootdown reinsertion of the new
mapping, so `new_ppn` is stored by initiation and preserved unchanged by later
request-transforming helpers.

Rationale: the kernel-side proceed/block decision should be one boring boolean.
`check_tlb_freshness` is total over any request-compatible slice and proves the
exact freshness predicate for that slice; `safe_to_proceed` then adds the
separate `request.completed` gate on top.

### 6. Validity boundary is explicit

Constructor-facing APIs require:

- `initiator < core_count`
- `vpn_start < vpn_end` for `initiate_shootdown`
- `vpn < u64::MAX` for `initiate_single_shootdown`

The constructor populates `pending_acks` with each remote core in
`0..core_count` exactly once and excludes the initiator. Public read-only
helpers (`check_tlb_freshness`, `safe_to_proceed`) quantify over the exact
caller-supplied slice through the request-compatible relation above.

Rationale: the executable model should reject malformed or overflowed ranges at
the API boundary and make the caller-supplied system boundary explicit instead
of inventing hidden global state.

### 7. Isolation is regression-backed over disjoint ranges

Keep a simple spec predicate `disjoint_ranges_spec` and cover the concrete
non-interference claim with targeted regression tests: a shootdown for one range
must preserve entries outside that range.

Rationale: the executable contract of `handle_shootdown_ipi` already states
exact range removal; the clearest review surface for disjointness here is an
explicit regression over separate ranges.

README sync is in scope for this landing: add the `tlb_shootdown` module row and
refresh the README headline counts from the final validation outputs. Metadata
sync review is also in scope for `docs/requirements.md` and
`openspec/specs/tlb-shootdown/spec.md`; final validation captures those files in
this change's review packet before archive.

## Verification Mapping

- `r[tlb-shootdown.freshness]`
  - proof: `invalidate_entries_fresh`
  - exact helper contracts: `core_tlb_fresh`, `check_tlb_freshness`, `core_tlb_fresh_spec`
  - tests: `test_multi_core_sequential_shootdown_finishes_fresh`
- `r[tlb-shootdown.termination]`
  - constructor initialization: `build_pending_acks`, `initiate_shootdown`, `initiate_single_shootdown`
  - preservation/readout: `acknowledge_shootdown`, `all_acknowledged`, `all_acknowledged_spec`
  - exact maintenance obligations: `acknowledge_shootdown` success/no-op postconditions preserve duplicate-free pending IDs, initiator exclusion, in-bounds pending indices, and `completed == pending_acks.is_empty()`
  - tests: `test_ack_before_invalidation_is_noop`, `test_double_ack_is_noop`, `prop_pending_count_never_increases`
- `r[tlb-shootdown.initiator-safety]`
  - proof: `completion_implies_freshness`
  - exact helper contracts: `safe_to_proceed`, `safe_to_proceed_spec`
  - tests: `test_multi_core_sequential_shootdown_finishes_fresh`
- `r[tlb-shootdown.self-invalidation]`
  - constructor postconditions: `initiate_shootdown`, `initiate_single_shootdown`, `pending_acks_spec`
  - preserved metadata: `new_ppn` field copied unchanged through `acknowledge_shootdown`
  - tests: `test_single_core_initiation_self_invalidates`
- `r[tlb-shootdown.isolation]`
  - spec: `disjoint_ranges_spec`
  - tests: `test_disjoint_requests_do_not_interfere`
- `r[tlb-shootdown.range]`
  - proof: `invalidation_clears_target_range`
  - exec/spec: `handle_shootdown_ipi`, `invalidate_entries_spec`, `vpn_in_range_spec`
  - tests: `test_range_invalidation_keeps_outside_entries`, `prop_range_invalidation_removes_exact_interval`

## Validation Plan

- Targeted: `cargo test --quiet tlb_shootdown`
- Targeted: `./scripts/verus.sh src/tlb_shootdown.rs`
- Full tree before archive:
  - `cargo test --quiet`
  - `cargo clippy --all-targets -- -D warnings`
  - `./scripts/verus.sh src/lib.rs`
  - `./scripts/verify.sh`
  - `./scripts/check-ensures.sh`
  - `tracey query uncovered`
  - `tracey query untested`
