## Context

UCAN tokens reference principals by DID (`did:key:z6Mk...`), not raw
key bytes. The issuer/audience fields are DID strings, and chain
validation matches audience-to-issuer across hops by string equality.

## Goals / Non-Goals

**Goals:** Verified round-trip for did:key encoding. Injectivity (distinct
keys produce distinct DIDs). Correct varint framing for the supported
codec set in this change.

**Non-Goals:** Supporting DID methods other than did:key. Full DID
document resolution. DID authentication protocols. Supporting a broad
codec registry beyond the Ed25519 codec value modeled by the landed
module.

## Decisions

### 1. Multicodec Varint Is Internalized

**Choice:** Varint encode/decode is fully verified (no external_body).
This change models the Ed25519 codec value `0xed`; its corresponding
LEB128 bytes are derived by `varint_encode` as `[0xed, 0x01]`. Well-
formed payloads carrying other codec values are treated as unsupported
and must be rejected by the decode boundary for this change.

**Rationale:** Varints are pure arithmetic over small integers. This is
squarely in Verus's sweet spot. No reason to trust it.

**Implementation:** Recursive spec functions over `Seq<u8>` with
matching exec loops.

### 2. Decode Returns Raw Parts; `DidKey` Is Encode-Side Carrier

**Choice:** `decode_did_key` parses an externally supplied did:key string
into raw `(public_key_bytes, codec)` parts. `DidKey` remains the encode-
side carrier returned by `encode_did_key`.

**Rationale:** This matches the landed module API directly. The code does
not expose a separate constructor that turns an arbitrary external DID
string into a stored validated `DidKey` value, so the design should not
promise one.

**Validation boundary:** malformed `did:key` byte strings are rejected
by `decode_did_key_spec` when the prefix is wrong, the base58 payload
does not decode, or the multicodec framing does not parse.

### 3. Base58btc Is External

**Choice:** Base58btc encode/decode is `#[verifier::external_body]`
with assumed round-trip postcondition.

**Rationale:** Base58 involves repeated division by 58 with leading-zero
preservation. The arithmetic is verifiable in principle but not worth
the proof effort for this change. Round-trip is confirmed by property
tests.

### 4. `DidKey` Is a Validated Encode Result

**Choice:** `DidKey` wraps the encoded `did:key` bytes plus the same
public-key bytes and codec passed to `encode_did_key`.

**Rationale:** The landed module's strongest invariant is on the encode
path: every produced `DidKey` starts with `did:key:z`, carries the exact
codec and public-key bytes passed in, and its encoded bytes satisfy the
spec-level encoding function. Decode correctness is tracked separately
through `decode_did_key_spec` and its round-trip/rejection properties.

## Verification Mapping

- `r[ucan-did.round-trip]` — Verus proof `did_key_round_trip` plus runtime `ucan_did` test transcript
- `r[ucan-did.injective]` — Verus proof `did_key_injective` plus distinct-key runtime test
- `r[ucan-did.prefix-valid]` — Verus proof `did_key_has_valid_prefix` plus encode runtime test
- `r[ucan-did.codec-preserved]` — Verus proof `did_key_codec_preserved`
- `r[ucan-did.varint-round-trip]` — Verus proof `varint_round_trip` plus runtime varint round-trip test
- `r[ucan-did.varint-rejects-empty]` — Verus proof for empty rejection and runtime checks for empty/truncated decode behavior
- `r[ucan-did.base58btc-round-trip]` — external-body assumption plus runtime round-trip test
- `r[ucan-did.base58btc-invalid-char-rejects]` — external-body/runtime rejection contract, covered by explicit invalid-input runtime test evidence
- `r[ucan-did.wrong-prefix-rejects]` and `r[ucan-did.malformed-payload-rejects]` — spec-level decode rejection behavior plus explicit malformed-input runtime evidence

Archive-ready validation uses the `openspec_gate` tool for
proposal/design/tasks plus `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_did`
and `./scripts/verus.sh src/lib.rs`. In this session, gate execution is
constrained by reviewer quota, so artifact sync should continue with the
current gate findings reconciled and fresh gate reruns captured once the
quota resets.

## Risks / Trade-offs

**[Base58 trust]** → Base58btc round-trip is assumed, not proven.
Mitigated by runtime tests over valid payloads plus explicit malformed-
input rejection requirements at the `did:key` parser boundary.
