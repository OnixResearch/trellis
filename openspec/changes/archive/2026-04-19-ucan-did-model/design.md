## Context

UCAN tokens reference principals by DID (`did:key:z6Mk...`), not raw
key bytes. The issuer/audience fields are DID strings, and chain
validation matches audience-to-issuer across hops by string equality.

## Goals / Non-Goals

**Goals:** Verified round-trip for did:key encoding. Injectivity (distinct
keys produce distinct DIDs). Correct multicodec varint framing.

**Non-Goals:** Supporting DID methods other than did:key. Full DID
document resolution. DID authentication protocols.

## Decisions

### 1. Multicodec Varint Is Internalized

**Choice:** Varint encode/decode is fully verified (no external_body).
The values are small (Ed25519 = 0xed, P-256 = 0x80 0x24) and the
encoding is simple LEB128.

**Rationale:** Varints are pure arithmetic over small integers. This is
squarely in Verus's sweet spot. No reason to trust it.

**Implementation:** Recursive spec functions over `Seq<u8>` with
matching exec loops.

### 2. Base58btc Is External

**Choice:** Base58btc encode/decode is `#[verifier::external_body]`
with assumed round-trip postcondition.

**Rationale:** Base58 involves repeated division by 58 with leading-zero
preservation. The arithmetic is verifiable in principle but not worth
the proof effort for this change. Round-trip is confirmed by property
tests.

### 3. DID String Is a Validated Newtype

**Choice:** `Did` is a newtype wrapping a `Vec<u8>` of the full DID
string, constructed only through `encode_did_key` or validated
`decode_did_key`.

**Rationale:** Prevents constructing invalid DIDs. The invariant
(starts with `did:key:z`, valid multibase/multicodec interior) is
enforced by the constructor.

## Risks / Trade-offs

**[Base58 trust]** → Base58btc round-trip is assumed, not proven.
Mitigated by property tests with random byte inputs.
