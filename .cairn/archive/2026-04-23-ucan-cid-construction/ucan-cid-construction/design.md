## Context

The landed `src/ucan_cid.rs` module already implements a narrow,
spec-first CID framing model for UCAN proof references. The older active
change artifacts describe a broader runtime CID API with constructor,
encoder, decoder, and property-test commitments that the shipped source
does not provide. This design reconciles the change to the proof surface
that actually exists.

## Goals / Non-Goals

**Goals:** Document the landed runtime carrier structs, exact varint /
multihash / CID byte framing, byte-level multibase round-trip, and the
proof/test evidence that closes the current module.

**Non-Goals:** Public exec CID encode/decode, runtime hash helpers,
`CidString`, structured CID parsing from multibase strings, CID lookup,
CIDv0, or BLAKE3 support.

## Decisions

### 1. The landed module is spec-first, not an exec CID API

**Choice:** `Multihash` and `Cid` exist as runtime carrier structs, but
all construction, parsing, and round-trip logic lives in `spec` and
`proof` functions.

**Rationale:** This matches `src/ucan_cid.rs` exactly and avoids claiming
an exec surface that does not exist.

### 2. Varint stays fully internalized at the proof layer

**Choice:** `varint_encode_spec(...)` and `varint_decode_spec(...)` are
verified directly, with proofs `varint_round_trip(...)` and
`varint_rejects_empty()`.

**Rationale:** LEB128 framing is small arithmetic and already lands
cleanly in Verus. No external-body bridge is needed.

### 3. Multihash framing is explicit and uses representable digest lengths

**Choice:** The multihash byte layout is exactly:
`varint(code) ++ varint(digest_len) ++ digest`.
The landed round-trip proof assumes `digest.len() <= u64::MAX` so the
byte-length field is representable.

**Rationale:** Real digests come from concrete byte vectors and easily
satisfy that bound. Making the byte layout explicit is necessary for the
round-trip proof and for downstream CID framing.

### 4. CID injectivity is a header property in the landed proof

**Choice:** The landed injectivity proof establishes that equal CID byte
sequences imply equal `version` and `codec` headers.

**Rationale:** That is the exact requirement text in `docs/requirements.md`
and the precise proof now carried by `cid_bytes_injective_header(...)`.
This change does not claim full multihash equality from equal CID bytes.

### 5. Multibase round-trip is byte-level via an external axiom

**Choice:** `cid_encode_spec(...)` and `cid_decode_spec(...)` are
uninterpreted multibase specs, and `axiom_cid_encode_round_trip(...)`
gives the external round-trip fact over raw CID byte sequences.
`cid_bytes_round_trip(...)` lifts that axiom to canonical `cid_bytes_spec`
values.

**Rationale:** The landed module proves byte framing around the axiom but
does not expose a structured decode API returning `Cid`.

### 6. SHA-256 remains an uninterpreted digest model

**Choice:** `sha256_digest_spec(...)` is an uninterpreted digest sequence
built from `sha256_byte_spec(...)`, and `cid_from_bytes_spec(...)`
constructs a CID from that digest.

**Rationale:** The landed module models deterministic CID construction
without implementing or verifying runtime hash code.

## Verification Mapping

### Requirement-level mapping

- `r[ucan-cid.cid-deterministic]` — `src/ucan_cid.rs:cid_deterministic` and `src/ucan_cid.rs:cid_from_bytes_deterministic`
- `r[ucan-cid.cid-version]` — `src/ucan_cid.rs:cid_version_is_one` plus `src/ucan_cid.rs:test_cid_version`
- `r[ucan-cid.cid-round-trip]` — `src/ucan_cid.rs:axiom_cid_encode_round_trip` and `src/ucan_cid.rs:cid_bytes_round_trip`
- `r[ucan-cid.cid-injectivity]` — `src/ucan_cid.rs:cid_bytes_injective_header`
- `r[ucan-cid.multihash-digest-length]` — `src/ucan_cid.rs:multihash_digest_length_matches`
- `r[ucan-cid.multihash-round-trip]` — `src/ucan_cid.rs:multihash_round_trip`
- `r[ucan-cid.varint-round-trip]` — `src/ucan_cid.rs:varint_round_trip`
- `r[ucan-cid.varint-rejects-empty]` — `src/ucan_cid.rs:varint_rejects_empty`

### Scenario-level mapping

- `r[ucan-cid.cid-deterministic.scenario-repeated-construction]` — `src/ucan_cid.rs:cid_from_bytes_deterministic`
- `r[ucan-cid.cid-version.scenario-version-one]` — `src/ucan_cid.rs:cid_version_is_one` and `src/ucan_cid.rs:test_cid_version`
- `r[ucan-cid.cid-round-trip.scenario-base32-bytes]` — `src/ucan_cid.rs:cid_bytes_round_trip`
- `r[ucan-cid.cid-injectivity.scenario-equal-bytes-equal-header]` — `src/ucan_cid.rs:cid_bytes_injective_header`
- `r[ucan-cid.multihash-digest-length.scenario-sha256-digest]` — `src/ucan_cid.rs:multihash_digest_length_matches` and `src/ucan_cid.rs:test_sha256_code`
- `r[ucan-cid.multihash-round-trip.scenario-wrap-then-decode]` — `src/ucan_cid.rs:multihash_round_trip`
- `r[ucan-cid.varint-round-trip.scenario-single-byte-raw]` — `src/ucan_cid.rs:varint_round_trip` and `src/ucan_cid.rs:test_raw_codec`
- `r[ucan-cid.varint-round-trip.scenario-two-byte-300]` — `src/ucan_cid.rs:varint_round_trip`
- `r[ucan-cid.varint-rejects-empty.scenario-empty-none]` — `src/ucan_cid.rs:varint_rejects_empty`

## Validation

Archive-ready validation uses:

- `openspec_gate stage=proposal change=ucan-cid-construction`
- `openspec_gate stage=design change=ucan-cid-construction`
- `openspec_gate stage=tasks change=ucan-cid-construction`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_cid`
- `./scripts/verus.sh src/lib.rs`

## Risks / Trade-offs

**[Byte-level round-trip only]** → The landed module proves multibase
round-trip for canonical CID byte sequences, not a structured `Cid`
parser/encoder API.

**[Representable digest lengths]** → The multihash round-trip theorem is
stated for digest lengths that fit in `u64`. This matches real digest
sizes and the existing byte-length field representation.

**[No exec hash surface]** → Runtime callers still need separate hashing
helpers if they want concrete CID construction outside proofs.
