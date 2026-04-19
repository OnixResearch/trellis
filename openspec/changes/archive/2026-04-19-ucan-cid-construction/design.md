## Context

UCAN proof references are CIDs — content-addressed identifiers that
name parent tokens by their hash. The `prf` field is `Vec<Cid>`, and
resolving a delegation chain means looking up tokens by CID. Without
verified CID construction, proof references are opaque strings.

## Goals / Non-Goals

**Goals:** Verified CID byte layout (version + codec varint + multihash).
Verified varint encoding. Multibase round-trip. Deterministic construction.

**Non-Goals:** Hash function verification (SHA-256/BLAKE3 internals).
CID resolution (storage/retrieval by CID). DAG traversal. IPLD selectors.

## Decisions

### 1. Varint Is Fully Internalized

**Choice:** LEB128 varint encode/decode is verified with no external_body.

**Rationale:** Same reasoning as the DID module. Varints are small
integer arithmetic, well within Verus's capabilities. CID correctness
depends on varint framing being right.

**Implementation:** Recursive spec functions. Exec uses a loop with
`& 0x7f` / `>> 7` per byte.

### 2. Hash Function Is External

**Choice:** The hash function (SHA-256 code 0x12, BLAKE3 code 0x1e)
is `#[verifier::external_body]` with a determinism postcondition.

**Rationale:** Hash internals are not feasible to verify in Verus.
The relevant property for CIDs is determinism: same input always
produces same hash. Collision resistance is a cryptographic assumption,
not a proof obligation.

### 3. CID Is a Struct With View

**Choice:** `Cid` has fields `version: u8`, `codec: u64`,
`multihash: Multihash`. View function exposes `cid_bytes` as `Seq<u8>`.

**Rationale:** Structured access for proofs, byte-level access for
serialization. The view is the canonical byte representation.

### 4. Multibase Is External

**Choice:** Base32lower and base58btc encode/decode are external_body
with round-trip postconditions.

**Rationale:** Same as in the DID module. Base encoding is arithmetic-
heavy and not the interesting proof surface. Round-trip confirmed by
property tests.

### 5. CIDv1 Only

**Choice:** Only CID version 1 is supported. CIDv0 (legacy, implicit
dag-pb + sha2-256) is not modeled.

**Rationale:** UCAN 1.0 uses CIDv1. CIDv0 is a compatibility concern
for IPFS, not for UCAN tokens.

## Risks / Trade-offs

**[Hash determinism assumption]** → We assume the hash function is
deterministic. If the runtime hash implementation is non-deterministic
(e.g., hash randomization), CID construction would break. Mitigated by
using SHA-256/BLAKE3 which are inherently deterministic.
