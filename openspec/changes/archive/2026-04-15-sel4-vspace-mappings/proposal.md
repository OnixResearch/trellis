## Why

Mapping and unmapping are reusable MMU-facing kernel primitives even before full architecture-specific page-table proofs. A separate change isolates alignment, non-overlap, exact unmap behavior, and rights restriction.

## What Changes

- Add abstract VSpace mappings over virtual ranges and object-aligned frames/page tables.
- Model map and unmap operations.
- Prove alignment, non-overlap, unmap exactness, and rights restriction.

## Capabilities

### New Capabilities

- `sel4-vspace.map`: Verified aligned non-overlapping mapping installation
- `sel4-vspace.unmap`: Verified exact removal of one mapping
- `sel4-vspace.rights`: Verified effective mapping rights are restricted by cap rights

## Impact

- **Files**: New `src/sel4_vspace.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for virtual memory mapping
- **Dependencies**: Reuses `region`, `page`, `interval`, and `sel4_cap_rights`
- **Testing**: Verus proofs, unit tests, and proptests for aligned map, overlap rejection, and unmap cases

## Out of Scope

- Architecture-specific page-table encoding
- ASID pool allocation policy
- Fault IPC payload layout
