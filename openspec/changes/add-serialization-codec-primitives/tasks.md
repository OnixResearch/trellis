## Coverage Index

| Requirement ID | Future module |
|---|---|
| `domain-primitives.serialization-codec.byte-buffer` | `src/codec_byte_buffer.rs` |
| `domain-primitives.serialization-codec.frame` | `src/codec_frame.rs` |
| `domain-primitives.serialization-codec.length-prefix` | `src/codec_length_prefix.rs` |
| `domain-primitives.serialization-codec.schema-version` | `src/codec_schema_version.rs` |
| `domain-primitives.serialization-codec.checksum-shape` | `src/codec_checksum_shape.rs` |
| `domain-primitives.serialization-codec.encoder-shape` | `src/codec_encoder_shape.rs` |
| `domain-primitives.serialization-codec.decoder-shape` | `src/codec_decoder_shape.rs` |
| `domain-primitives.serialization-codec.round-trip` | `src/codec_round_trip.rs` |

## Phase 1: Spec foundation

- [x] Define finite serialization/codec primitive requirements and exact-model non-goals.

## Phase 2: Future implementation slices

- [ ] Implement finite byte-buffer envelope admission and capacity/length predicates. `[covers=domain-primitives.serialization-codec.byte-buffer]`
- [ ] Implement finite framed-record envelopes with header/payload/trailer range checks. `[covers=domain-primitives.serialization-codec.frame]`
- [ ] Implement checked length-prefix metadata tied to remaining payload bounds. `[covers=domain-primitives.serialization-codec.length-prefix]`
- [ ] Implement deterministic schema/version admission metadata. `[covers=domain-primitives.serialization-codec.schema-version]`
- [ ] Implement checksum field shape metadata without integrity or security claims. `[covers=domain-primitives.serialization-codec.checksum-shape]`
- [ ] Implement deterministic encoder output-shape predicates for admitted finite models. `[covers=domain-primitives.serialization-codec.encoder-shape]`
- [ ] Implement deterministic decoder admission/rejection shape predicates for finite buffers. `[covers=domain-primitives.serialization-codec.decoder-shape]`
- [ ] Implement bounded round-trip envelopes over explicitly admitted finite models. `[covers=domain-primitives.serialization-codec.round-trip]`

## Phase 3: Evidence and archive

- [ ] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for each implemented slice.
- [ ] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final validation.
- [ ] Sync and archive the change once all implementation/evidence tasks are complete.
