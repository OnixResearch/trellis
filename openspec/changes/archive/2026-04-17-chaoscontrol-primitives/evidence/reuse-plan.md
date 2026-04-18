## Reuse plan

Phase 0 decision record for existing generic helpers mentioned in
`design.md` Decision 9.

### Summary

| Helper | Decision | Planned use | Rationale |
| --- | --- | --- | --- |
| `bitmap` | skip direct reuse | none for primary coverage storage | ChaosControl coverage uses fixed `65536`-entry `u8` counters with pointwise-`max` merge and AFL bucketization, not a bitset. Reusing `bitmap` for the main representation would hide the counted-counter semantics we need to specify and verify. |
| `quorum` | reuse | majority threshold facts for `chaoscontrol_fault_state` | `quorum_reachable` needs the standard `majority(n)` threshold. Reusing `quorum` keeps threshold arithmetic shared instead of re-proving the same majority facts. |
| `monotonic` | reuse selectively | scheduler `remaining` countdown monotonicity and fault-schedule cursor monotonicity | These modules need simple monotone-decrease / monotone-advance facts that already match existing repo-local monotonic helpers. |
| `seq_set` | reuse selectively | fault-schedule subset reasoning and input-tree distinct/prefix reasoning | `subset(indices)` and deterministic alternative streams both need sequence/set-style uniqueness and subset/filter facts; reuse avoids cloning those lemmas under ChaosControl names. |

### Per-module notes

- `chaoscontrol_coverage`
  - keep module-specific `Vec<u8>` / `Seq<u8>` model
  - do **not** wrap `bitmap`; counter-valued cells are the core contract
- `chaoscontrol_oracle`
  - no direct helper reuse planned from this list
  - record-validity and verdict rules are domain-specific
- `chaoscontrol_scheduler`
  - reuse `monotonic` for countdown / cursor-style progress facts where it fits
  - keep scheduler state machine and fingerprint mix module-local
- `chaoscontrol_fault_schedule`
  - reuse `monotonic` for cursor monotonicity
  - reuse `seq_set` for subset/dedup/order reasoning where the existing lemmas match
- `chaoscontrol_frontier`
  - no direct helper reuse planned from this list
  - exact rational ordering and tie-break rules are domain-specific
- `chaoscontrol_input_tree`
  - reuse `seq_set` for distinctness/prefix reasoning where useful
  - keep exact denominator-order comparator module-local
- `chaoscontrol_fault_state`
  - reuse `quorum` for majority threshold facts
  - keep pair-connected `quorum_reachable` heuristic module-local because it is intentionally narrower than generic quorum predicates

### Exceptions / non-reuse justification

- `bitmap` stays unused for this change's main storage model because it proves set-membership over bits, not exact `u8` hit-count algebra.
- `quorum` is reused only for `majority(n)` arithmetic, not for the exported `quorum_reachable` predicate; that predicate is intentionally the current ChaosControl majority-alive + direct-bidirectional-pair heuristic.
- `monotonic` and `seq_set` are reused opportunistically only when their existing lemmas line up exactly; otherwise small local lemmas are preferred over force-fitting unrelated abstractions.
