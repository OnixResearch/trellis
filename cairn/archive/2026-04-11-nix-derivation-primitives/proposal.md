## Why

The existing nix modules verify builtins that operate on already-evaluated
values: version comparison, string manipulation, attribute sets, lists,
closures. But the layer beneath those — how Nix computes store paths from
derivations — remains unverified.

Store path computation is the foundation of Nix's content-addressable build
model. A derivation's output path is determined by hashing a fingerprint
constructed from the derivation's type, inputs, and name. The algorithm
involves three non-trivial sub-operations:

1. **Nix base32 encoding** — Nix uses a non-standard alphabet
   (`0123456789abcdfghijklmnpqrsvwxyz`, omitting `e`, `o`, `t`, `u`) and a
   reversed bit-packing order compared to RFC 4648. Encoding/decoding must
   be bijective. A bug here means store paths silently collide or point to
   wrong content.

2. **Hash truncation** — `compressHash` XOR-folds a hash (32 bytes for
   SHA-256) down to 20 bytes for the store path prefix. The fold must
   preserve collision resistance and be deterministic. Off-by-one in the
   fold window breaks every path.

3. **Store path fingerprint construction** — The fingerprint is a structured
   string (`type:sha256:<hash>:/nix/store:<name>`) that gets hashed to
   produce the final path. Fixed-output derivations use a different
   fingerprint format (`fixed:out:<hashAlgo>:<hash>:`). The fingerprint
   format must be unambiguous — two different derivations must never produce
   the same fingerprint (injectivity).

4. **Derivation ATerm serialization** — Derivations are serialized in ATerm
   format for hashing. The serialization must be deterministic: sorted env
   vars, sorted input derivations, sorted input sources. Non-deterministic
   serialization breaks reproducibility.

5. **Derivation output path resolution** — Input-addressed derivations hash
   the `.drv` contents (with self-references masked). Content-addressed
   derivations hash the realized output. Fixed-output derivations use the
   declared hash. Each path must land in the correct computation.

These primitives are where bugs cause the most damage. A hash truncation
error silently corrupts the entire store. A base32 encoding mistake makes
`nix-store --verify` fail on valid paths. A fingerprint format ambiguity
lets two different derivations claim the same output path.

tvix, lix, and harmonia all reimplement this logic independently. A
machine-checked reference would give them something to test against.

## What Changes

Five new modules, each following the established `spec fn` / `exec fn` /
`proof fn` pattern:

- `nix_base32` — encode/decode using Nix's custom base32 alphabet.
  Bijectivity proof: `decode(encode(x)) == x` and
  `encode(decode(y)) == y` for valid encodings.

- `nix_hash_truncate` — `compress_hash(hash, target_len)` XOR-folds a
  byte sequence to a shorter length. Determinism proof: same input always
  produces same output. Length proof: output is exactly `target_len` bytes.

- `nix_fingerprint` — construct store path fingerprints for regular,
  fixed-output, and content-addressed derivations. Injectivity proof:
  distinct (type, hash, name) triples produce distinct fingerprints.

- `nix_drv_serialize` — deterministic ATerm serialization of derivation
  structures. Determinism proof: same derivation always serializes
  identically. Injectivity proof: distinct derivations serialize
  differently.

- `nix_store_path` — compose the above to compute complete store paths
  (`/nix/store/<base32(compress(sha256(fingerprint)))>-<name>`).
  End-to-end correctness: the composition preserves the injectivity
  and determinism of its components.

## Capabilities

### New Capabilities

- `nix-base32`: Verified Nix base32 encode/decode with bijectivity proof
- `nix-hash-truncate`: Verified XOR-fold hash truncation with length/determinism proofs
- `nix-fingerprint`: Verified store path fingerprint construction with injectivity proof
- `nix-drv-serialize`: Verified deterministic ATerm derivation serialization
- `nix-store-path`: Verified end-to-end store path computation

### Modified Capabilities

None.

## Impact

- **Files**: Five new `src/nix_*.rs` modules, additions to `src/lib.rs`
- **APIs**: Five new public modules, all functions with verified contracts
- **Dependencies**: No new crate dependencies. Uses existing `vstd` Seq types.
  SHA-256 is modeled as an opaque spec function (not reimplemented).
- **Testing**: Each module gets proptest oracles cross-checked against
  `nix-store --dump-db` and `nix hash` CLI output. `verus` and
  `tracey query uncovered` gates must pass after each phase.
