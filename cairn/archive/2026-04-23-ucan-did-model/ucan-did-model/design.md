## Context

UCAN issuer and audience fields are `did:key` identifiers. The landed
module in this repo does not implement a full DID library; it provides a
verified byte-oriented framing layer for `did:key:z...` values so later
UCAN code can rely on stable canonical bytes.

## Goals / Non-Goals

**Goals:** Verified contracts for byte-oriented `did:key` framing:
canonical encoding layout, spec-level encode/decode round-trip,
injectivity, prefix validity, codec preservation, varint round-trip,
empty/truncated varint rejection, and base58btc round-trip. Keep
Ed25519 (`0xed`) as the canonical UCAN example without hard-coding this
change to Ed25519 alone.

**Non-Goals:** A generic runtime DID parser, DID document resolution,
codec-registry validation, unsupported-codec rejection, or codec-specific
key-length checks. Those belong to higher-level key-specific modules.

## Decisions

### 1. Varint framing is fully internalized for arbitrary codec values

**Choice:** `varint_encode` / `varint_decode` are fully verified for any
`u64` codec value.

**Rationale:** The landed code is generic over codec numbers. Ed25519
`0xed` / `[0xed, 0x01]` is the canonical runtime example used by UCAN,
but the proof model preserves any codec value rather than rejecting all
non-Ed25519 cases.

**Implementation:** Recursive spec helpers plus matching exec wrappers;
proofs establish round-trip and empty/truncated rejection.

### 2. The public exec API is encode-only; decode stays at the spec layer

**Choice:** `encode_did_key` is the public exec DID helper. DID decoding
exists only as `decode_did_key_spec(...)` for proof/model purposes.

**Rationale:** This matches `src/ucan_did.rs` exactly. The module exports
an encode-side `DidKey` carrier and the proofs reason about a spec-level
parser, but there is no landed public runtime `decode_did_key` function.
OpenSpec must reflect that boundary instead of promising runtime decode
coverage the code does not expose.

**API contract:** Public exec functions are byte-oriented:
`encode_did_key(&Vec<u8>, u64) -> DidKey`,
`varint_encode(u64) -> Vec<u8>`, `varint_decode(&Vec<u8>) -> Option<(u64, usize)>`,
and `base58btc_encode/decode(&Vec<u8>)`. The proof-only DID parser takes
`Seq<u8>` and returns raw `(key_bytes, codec)` parts.

### 3. Payload bytes stay opaque

**Choice:** The DID layer preserves exact key bytes and codec values, but
it does not impose codec-specific key lengths.

**Rationale:** The landed module is a framing primitive, not a registry of
key algorithms. Ed25519 32-byte keys are the primary UCAN example and
are exercised by tests, but length contracts belong in the key-specific
modules that know what a valid payload means.

**Implementation:** `DidKey` stores the exact bytes supplied to
`encode_did_key`, and the round-trip proof shows those bytes come back
unchanged through `decode_did_key_spec`.

### 4. Base58btc stays external with an explicit proof-side trust boundary

**Choice:** Base58btc encode/decode remains `#[verifier::external_body]`.
At the requirement level, this change only claims
`r[ucan-did.base58btc-round-trip]`. To discharge the higher-level DID
injectivity requirement `r[ucan-did.injective]`, the proof layer also
uses a separate external-body injectivity helper as an internal proof
assumption rather than as a standalone user-facing requirement.

**Rationale:** The arithmetic-heavy division algorithm is not the point of
this change. The trusted boundary is intentionally narrow at the spec
surface, while the proof implementation documents the extra helper needed
for the DID injectivity theorem.

**Implementation:** Runtime uses `bs58`; proofs consume the round-trip
contract for `r[ucan-did.base58btc-round-trip]` and the proof-local
injectivity helper only inside `did_key_injective`.

## Verification Mapping

### Requirement-level mapping

- `r[ucan-did.round-trip]` — `did_key_round_trip` plus runtime `ucan_did` transcript for the Ed25519 example path
- `r[ucan-did.injective]` — `did_key_injective`, which uses the trusted base58 proof helper documented in `spec.md`, plus distinct-key runtime test
- `r[ucan-did.prefix-valid]` — `did_key_has_valid_prefix` plus encode runtime test
- `r[ucan-did.codec-preserved]` — `did_key_codec_preserved`
- `r[ucan-did.varint-round-trip]` — `varint_round_trip` plus runtime varint round-trip test
- `r[ucan-did.varint-rejects-empty]` — `varint_decode_rejects_empty` plus runtime empty/truncated varint checks
- `r[ucan-did.base58btc-round-trip]` — base58 external-body round-trip assumption plus runtime round-trip test

### Scenario-level mapping

- `r[ucan-did.round-trip.scenario-ed25519]` — `test_did_key_encode_ed25519` plus `did_key_round_trip`
- `r[ucan-did.round-trip.scenario-carrier]` — `encode_did_key` postconditions preserving `public_key_bytes`, `codec`, and `encoded`
- `r[ucan-did.injective.scenario-distinct-keys]` — `test_did_key_distinct_keys` plus `did_key_injective`
- `r[ucan-did.prefix-valid.scenario-any-key]` — `did_key_has_valid_prefix` plus `test_did_key_encode_ed25519`
- `r[ucan-did.codec-preserved.scenario-ed25519]` — `did_key_codec_preserved` plus `did_key_round_trip`
- `r[ucan-did.varint-round-trip.scenario-ed25519]` — `test_varint_encode_two_bytes` plus `varint_round_trip`
- `r[ucan-did.varint-rejects-empty.scenario-empty]` — `test_varint_decode_empty` plus `varint_decode_rejects_empty`
- `r[ucan-did.varint-rejects-empty.scenario-truncated]` — `test_varint_decode_truncated_continuation` captured in `evidence/test-varint-truncated.txt`
- `r[ucan-did.base58btc-round-trip.scenario-ed25519-payload]` — `test_did_key_base58_round_trip`

Archive-ready validation uses the `openspec_gate` tool for
proposal/design/tasks plus
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_did`,
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_varint_decode_truncated_continuation`,
and `./scripts/verus.sh src/lib.rs`.

## Risks / Trade-offs

**[Base58 trust]** → Base58btc behavior is assumed, not proven.
Mitigated by runtime round-trip coverage over the payload boundary the
landed module actually exposes.

**[Decode surface is proof-only]** → There is no public runtime DID decode
API to validate malformed external strings directly in this change.
Mitigated by documenting that boundary explicitly and keeping the public
contract encode-oriented.