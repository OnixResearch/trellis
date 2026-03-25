## Why

The library has single-bit operations (`bitwise.rs`) and two ad-hoc
multi-bit packers (`slab.rs` handle_pack/unpack, `permission.rs`
bit_position). There's no general-purpose verified bitfield module for
extracting and inserting multi-bit fields within a u64 word.

Protocol headers, hardware registers, page table entries, and packed
handles all need this. Every crate reinvents the same shift-and-mask
dance without proofs.

## What Changes

- **New module `src/bitfield.rs`**: four core functions (field_mask,
  extract_field, insert_field, clear_field) plus verified round-trip,
  non-interference, and boundedness proofs.

## Capabilities

### New Capabilities
- `bitfield.field-mask`: compute a bitmask for bits [offset, offset+width)
- `bitfield.extract-field`: extract a multi-bit field from a word
- `bitfield.insert-field`: insert a value into a multi-bit field
- `bitfield.clear-field`: zero a multi-bit field

## Impact

- **Files**: `src/bitfield.rs` (new), `src/lib.rs` (add module), `docs/requirements.md` (add reqs)
- **APIs**: 4 new pub functions + 4 spec fns
- **Dependencies**: none (uses only vstd bitwise primitives)
- **Testing**: by(bit_vector) proofs + proptest oracles
