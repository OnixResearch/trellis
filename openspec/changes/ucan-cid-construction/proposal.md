## Why

UCAN tokens reference their proof chain by CID (Content Identifier),
not by in-memory pointer or index. A CID is a self-describing content
address: multihash of the content bytes, wrapped in a multicodec
content type, with a CID version byte and multibase string encoding.
The `prf` field in a UCAN payload is `Vec<Cid>`, and resolving a
delegation chain means looking up parent tokens by their CID.

Without verified CID construction, the token envelope cannot reference
proofs, chain validation cannot resolve parent tokens, and the
content-addressing guarantee — that CID equality implies content
equality — is unverified. A CID bug means tokens that reference the
wrong parent, or two different tokens that hash to the same CID
(collision modeling), or a valid CID that decodes to the wrong
multihash.

The proof surface: CID construction is deterministic (same content
always produces the same CID), round-trip correct (decode recovers
the original multihash and codec), and injective within a hash
function (distinct content produces distinct CIDs, modulo hash
collision which is modeled but not proven — that stays in the
crypto trust boundary).

## What Changes

A new module `src/ucan_cid.rs` providing:

**Data model:**
- `Multihash` — hash function code (varint) + digest length + digest bytes
- `Cid` — version (1) + content codec (varint, e.g., 0x71 for dag-cbor) + multihash
- `CidString` — multibase-encoded CID (base32lower with 'b' prefix, or base58btc with 'z')

**Construction:**
- `multihash_wrap(code, digest) -> Multihash` — wrap a digest with its function code
- `cid_from_bytes(codec, hash_code, content) -> Cid` — hash content then wrap as CID
- `cid_encode(cid, base) -> CidString` — multibase encode a CID
- `cid_decode(s) -> Option<Cid>` — parse a multibase CID string
- `cid_bytes(cid) -> Seq<u8>` — raw CID bytes (version + codec varint + multihash)

**Verified properties:**
- Deterministic: `cid_from_bytes(c, h, content) == cid_from_bytes(c, h, content)`
- Round-trip: `cid_decode(cid_encode(cid, base)) == Some(cid)`
- Injective (modeled): `cid_bytes(a) == cid_bytes(b) ==> a.codec == b.codec && a.multihash == b.multihash`
- Version correct: constructed CIDs always have version 1
- Multihash structure: digest length field matches actual digest length
- Varint round-trip: multicodec varint encoding/decoding is correct

The hash function itself (SHA-256 or BLAKE3) stays `#[verifier::external_body]`.
The proof surface is the framing: CID bytes are correctly structured,
varints round-trip, and multibase encoding preserves the CID value.

## Capabilities

### New Capabilities
- `ucan-cid.deterministic`: same content always produces same CID
- `ucan-cid.round-trip`: encode then decode recovers original CID
- `ucan-cid.injective`: equal CID bytes imply equal components
- `ucan-cid.version-correct`: CID version is always 1
- `ucan-cid.multihash-structure`: digest length matches digest
- `ucan-cid.varint-round-trip`: varint encoding/decoding is correct

## Impact

- **Files**: `src/ucan_cid.rs` (new)
- **APIs**: ~8 pub fns (multihash_wrap, cid_from_bytes, cid_encode, cid_decode, cid_bytes, varint helpers)
- **Dependencies**: None for verification; optional sha2/blake3 behind cfg gate for runtime hashing
- **Testing**: Property tests for round-trip with random content; Verus proofs for structural correctness
