# Draft → Landed Scope Mapping

The original draft mixed three scopes. The landed change archives only the
self-contained stack-slot core that matches the repo's current pure-logic
module style.

## Landed now

- aligned local/spill slot model
- byte-range helpers (`byte_range`, `ranges_overlap`)
- exact rounded frame-size computation
- append-only slot assignment from the current maximum slot extent
- explicit safe-reuse checks over caller-supplied live ranges
- focused tests, Verus proofs, Tracey requirement sync, and canonical spec sync

## Explicitly deferred from the original draft

- callee-save save/restore instruction-sequence modeling
- outgoing-argument calling-convention placement
- automatic slot-reuse search inside `assign_slot`
- dynamic stack allocation or ABI-specific extras

## Why the scope changed

Those deferred items require either an instruction model or ABI tables that do
not fit the current repo's pure data-structure proof style. The landed module
keeps stack-frame reasoning useful and reviewable without introducing those new
surfaces.
