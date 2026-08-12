## Why

The TLB (translation lookaside buffer) caches virtual-to-physical address
translations per core. When the kernel changes a mapping, every core that
might still cache the old translation must drop that stale entry before the
caller treats the new mapping as authoritative.

The minimal reusable property is not full MMU behavior; it is the
shootdown protocol boundary:

- the initiator invalidates its own cached entry first,
- other cores acknowledge after they have invalidated,
- completion is reported only after every targeted core has acknowledged,
- once completion is reported, no targeted core still caches the stale
  translation.

This is the smallest value-level model that downstream kernel and
hypervisor work can compose with without dragging in full page tables,
interrupt controllers, or TLB microarchitecture.

## What Changes

- **`tlb_shootdown`**: a new verified module for a page-local TLB
  shootdown protocol.

  The implementation will use a **single-page core** plus a **range
  lifting layer**:

  - a single page's cached translation state is modeled as one `Vec<u64>`
    indexed by core,
  - a single-page request carries an initiator, target page number, and a
    per-request acknowledgment bitmap,
  - range invalidation is modeled as a thin proof/spec layer over a
    page-indexed matrix that applies the single-page invalidation pointwise.

  Proves:
  - **Freshness**: once the protocol reports completion, every targeted
    core's cached translation for the target page is absent.
  - **Termination under responsiveness**: if each targeted core handles
    the IPI at most once and eventually acknowledges, the protocol reaches
    completion and pending acknowledgments never increase.
  - **Initiator safety**: the module does not expose completion early;
    callers can treat completion as the guard for proceeding.
  - **Self-invalidation**: the initiator invalidates its own cached
    translation before waiting on other cores.
  - **Isolation**: invalidating one target page does not modify cached
    translations for any different page.
  - **Range coverage**: range invalidation is pointwise complete for every
    page in `[start, end)` and preserves pages outside the range.

## Capability Surface

### Exec functions

- `initiate_shootdown`
- `handle_shootdown_ipi`
- `acknowledge_shootdown`
- `all_acknowledged`
- `pending_ack_count`
- `check_tlb_freshness`

### Spec / proof surface

- single-page freshness and acknowledgment invariants
- disjoint-page isolation lemmas over page matrices
- range-completeness and outside-range-preservation lemmas

## Impact

- **Files**: new `src/tlb_shootdown.rs`, `src/lib.rs`,
  `docs/requirements.md`, canonical `openspec/specs/tlb-shootdown/spec.md`,
  and archive evidence files
- **Dependencies**: `vstd` only
- **Testing**: unit tests and proptests for single-page request handling,
  random acknowledgment orderings, invalid-index negative cases, and range
  helper boundaries

## Model Assumptions

- The core protocol is page-local: one single-page request reasons about
  one target page at a time.
- Acknowledgment is per request, represented by a request-local bitmap.
- A core is **responsive** when it handles the request at most once and,
  if targeted, eventually acknowledges after invalidating.
- The TLB model tracks whether the stale cached translation is still
  present. It does **not** model page-table walks or the new mapping's
  full semantics.

## Non-Goals

- Real interrupt delivery timing, batching, or timeout behavior
- TLB replacement policy, ASIDs, page sizes, or hardware internals
- Page-table structure or memory-allocation lifecycle
- Speculative execution, permission checks, or microarchitectural side
  channels
- Fairness or recovery under non-responsive cores
