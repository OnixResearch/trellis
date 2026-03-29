## Why

The chain, merkle, and snapshot_integrity modules all axiomatize hash
collision resistance: same hash output → same inputs. This gives us
"same hash → same bytes." But none of them prove the next step: that
the encoding fed into the hash function is injective — that same bytes
→ same structure.

Right now this doesn't bite us because all three modules feed fixed-width
`u64` fields directly into `hash_spec`. Four u64s in fixed positions are
trivially unambiguous. But the gap becomes real the moment any module
hashes variable-length data, nested structures, or tagged unions. Without
a serialization injectivity proof, two different structures could encode
to the same byte sequence, producing "collisions" that have nothing to
do with the hash function.

HashMath (a content-addressed CIC in Lean) demonstrates this pattern:
they state `serializeExpr_injective` and `hashExpr_injective` as explicit
proof targets, with the full reasoning chain:

```
hash(e₁) == hash(e₂)
  → serialize(e₁) == serialize(e₂)    [collision resistance axiom]
  → e₁ == e₂                           [serialization injectivity theorem]
```

Collision resistance is an axiom. Serialization injectivity is provable
from the encoding design. Together they give the real guarantee: same
hash → same structure. We should prove the second half.

## What Changes

A new `serialize_inj` module providing verified canonical serialization
primitives with injectivity proofs. The module works at the `Seq<u8>`
level (Verus ghost sequences) — no runtime allocations, no I/O, pure
reasoning about byte layouts.

**Fixed-width encoding**

- `encode_u64_spec` — 8-byte little-endian encoding of a u64
- `encode_u64_injective` — proof that equal byte sequences → equal values

**Tag-prefixed encoding**

- `encode_tagged_spec` — single tag byte prepended to a payload
- `tag_disjoint` — proof that different tags produce different first bytes,
  so tagged encodings with distinct tags never collide regardless of payload
- `encode_tagged_injective` — same tag + same bytes → same payload

**Length-prefixed encoding**

- `encode_len_prefixed_spec` — 8-byte length header followed by payload
- `encode_len_prefixed_injective` — proof that length-prefixed encoding is
  injective: equal concatenations → equal lengths → equal payloads

**Multi-field struct encoding**

- `encode_pair_spec` — concatenation of two fixed-width fields
- `encode_pair_injective` — positional splitting: bytes [0..8) determine
  field 1, bytes [8..16) determine field 2
- `encode_quad_spec` — four fixed-width u64 fields (matches chain.rs layout)
- `encode_quad_injective` — same bytes → same four fields

**Composition theorem**

- `hash_struct_injective` — given `serialize_injective` and
  `hash_collision_free`, prove `hash(serialize(a)) == hash(serialize(b)) → a == b`.
  This is the glue that connects the existing collision-resistance axiom
  to concrete structure equality.

**Cross-module links**

- Proof that `chain.rs`'s `hash_spec(prev, index, term, data_id)` is
  equivalent to hashing a 4-field fixed-width encoding, making
  `encode_quad_injective` applicable
- Proof that `snapshot_integrity.rs`'s `dual_hash_spec(a, b)` is
  equivalent to hashing a 2-field encoding, making `encode_pair_injective`
  applicable

## Capabilities

### New Capabilities
- `serialize-fixed-width-injective`: Fixed-width u64 encoding is injective
- `serialize-tagged-injective`: Tag-prefixed encoding is injective per tag
- `serialize-tagged-disjoint`: Different tags never produce equal encodings
- `serialize-len-prefixed-injective`: Length-prefixed encoding is injective
- `serialize-pair-injective`: Two-field struct encoding is injective
- `serialize-quad-injective`: Four-field struct encoding is injective
- `serialize-hash-compose`: hash ∘ serialize injective when both halves are

### Modified Capabilities
- Cross-module proofs in `properties.rs` linking serialize_inj to chain
  and snapshot_integrity

## Impact

- **Files**: `src/serialize_inj.rs` (new), `src/lib.rs` (module decl),
  `src/properties.rs` (cross-module proofs)
- **APIs**: ~7 spec fns, ~7 exec fns, ~10 proof fns
- **Dependencies**: Uses `chain::hash_spec` and `chain::hash_collision_free`;
  uses `snapshot_integrity::dual_hash_spec` and
  `snapshot_integrity::dual_hash_collision_resistant`
- **Testing**: Verus proofs + proptest for roundtrip and injectivity oracles
