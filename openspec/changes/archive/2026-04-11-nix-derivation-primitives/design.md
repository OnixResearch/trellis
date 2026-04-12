## Context

The verified-logic library has five nix modules covering builtins over
evaluated values (version, string, attrset, list, closure). This change
adds the layer below: the derivation-to-store-path pipeline.

All existing nix modules follow the pattern: `spec fn` for the math,
`exec fn` with `ensures` for runtime code, `proof fn` for algebraic
properties. Proptest oracles validate against the real Nix implementation.

## Goals / Non-Goals

**Goals:**
- Verified base32 encode/decode with bijectivity proof
- Verified hash truncation with XOR-fold properties
- Verified fingerprint construction with injectivity proof
- Verified ATerm serialization with determinism and injectivity proofs
- Verified store path composition chaining all components
- Proptest oracles checked against `nix` CLI output

**Non-Goals:**
- Reimplementing SHA-256 — modeled as an opaque spec function
- NAR hashing — that's a serialization format, not a derivation primitive
- Derivation building/evaluation — only the path computation from a
  fully-resolved derivation structure
- Content-addressed derivation paths (experimental Nix feature, still
  in flux)
- Recursive Nix / IFD — evaluation-time concerns, not path computation

## Decisions

### 1. SHA-256 as Opaque Spec Function

**Choice:** Model SHA-256 as `pub open spec fn sha256_spec(input: Seq<u8>) -> Seq<u8>`
with axioms: output length is 32, deterministic (same input → same output),
collision-resistant (modeled as an assumption, not proved).

**Rationale:** Verifying SHA-256 itself is a separate project (thousands of
lines of bit-manipulation proofs). For store path computation, we only need
the interface properties. The opaque model lets us prove the composition
without getting stuck on hash internals.

**Alternative:** Use `#[verifier::external_body]` on a real SHA-256
implementation and trust it. Rejected because the spec-level axioms let
us prove injectivity of the full pipeline, not just test it.

### 2. Base32 Bit Packing as Index Arithmetic

**Choice:** Define encode/decode in terms of bit-index arithmetic over
the input byte sequence. For bit position `p` in the output, the
corresponding input bit is at byte `(input_len - 1) - p / 8`, bit
`p % 8`. Each output character packs 5 consecutive bits.

**Rationale:** Nix's base32 packs from the last byte forward (reversed
vs RFC 4648). Expressing this as arithmetic over bit positions makes the
bijectivity proof a matter of showing the bit-index mapping is a
bijection. Loop invariants track "all bits up to position p have been
correctly encoded."

**Implementation:**
```
spec fn encode_char_at(bytes: Seq<u8>, pos: nat) -> u8 {
    // extract 5 bits starting at bit position pos*5, reversed byte order
    let total_bits = bytes.len() * 8;
    ...
}
```

### 3. Derivation Structure as Plain Struct

**Choice:** Represent derivations as:
```rust
pub struct DerivationOutput {
    pub name: Vec<u8>,
    pub path: Vec<u8>,
    pub hash_algo: Vec<u8>,
    pub hash: Vec<u8>,
}

pub struct Derivation {
    pub outputs: Vec<DerivationOutput>,
    pub input_drvs: Vec<(Vec<u8>, Vec<Vec<u8>>)>,
    pub input_srcs: Vec<Vec<u8>>,
    pub platform: Vec<u8>,
    pub builder: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub env: Vec<(Vec<u8>, Vec<u8>)>,
}
```

**Rationale:** Matches the ATerm structure directly. Uses `Vec<u8>` for
all strings (same as existing nix modules). Sorted invariants on
`input_drvs`, `input_srcs`, `env`, and `outputs` are expressed as
`requires` preconditions, following the `nix_attrset` pattern.

**Alternative:** Use the `nix_attrset` type for env. Rejected because
env values are `Vec<u8>` not `u64`, and adding a generic parameter to
attrset would complicate the existing proofs.

### 4. Fingerprint Injectivity via Format Structure

**Choice:** Prove fingerprint injectivity by showing the colon-separated
format is unambiguous. The proof assumes names and hashes don't contain
colons (validated by `valid-name` and hex encoding respectively). Given
that, the format is a unique concatenation of typed fields — different
field values produce different strings.

**Rationale:** This is the same approach as `serialize_inj` — showing
that a serialization format with unambiguous delimiters is injective.
The existing `serialize_inj` module already has the proof machinery for
delimiter-based injectivity.

### 5. Module Layering

**Choice:** Five modules in dependency order:
1. `nix_base32` — no dependencies on other nix modules
2. `nix_hash_truncate` — no dependencies on other nix modules
3. `nix_fingerprint` — depends on name validation only
4. `nix_drv_serialize` — depends on nix_attrset for sorted-key invariant
5. `nix_store_path` — depends on all four above

**Rationale:** Each module is independently testable. The dependency
chain is linear, so implementation follows the same order. proptest
oracles can validate each layer against the Nix CLI before building
the next.

### 6. Proptest Oracle Strategy

**Choice:** Generate random derivation structures in proptest, serialize
them with our code, then compare against `nix derivation show` /
`nix-store --dump-db` output for known derivations from nixpkgs.

For base32 specifically: generate random byte sequences, encode with our
code, decode with `nix hash to-base32` (which uses the same alphabet),
and verify round-trip.

**Rationale:** The Nix CLI is the canonical reference. Testing against
it catches semantic mismatches that pure property tests might miss. The
existing nix modules use the same approach (version comparison tested
against `nix-instantiate --eval`).

## Risks / Trade-offs

**[Base32 bit-index arithmetic]** The reversed bit packing makes the
encode/decode proofs mirror images of each other. The bijectivity proof
requires showing the bit-index mapping is both injective and surjective.
This is straightforward but tedious — expect ~100 lines of bit-index
lemmas. Mitigation: start with a simple 1-byte encode/decode test to
validate the index formula before writing the general proof.

**[SHA-256 opacity]** Modeling SHA-256 as opaque means we can't prove
collision resistance — we assume it. The store path injectivity proof
is conditional on this assumption. This is standard practice (no one
proves SHA-256 collision resistance from scratch), but it means our
"injectivity" guarantee is really "injectivity modulo hash collisions."
Mitigation: document the assumption explicitly in the spec fn.

**[ATerm escaping edge cases]** The ATerm escape rules interact with
Nix's string encoding. Bytes outside ASCII printable range are passed
through verbatim in Nix (not hex-escaped). Our model must match this
exactly. Mitigation: generate test cases with embedded NUL bytes,
high bytes (0x80+), and all escape-triggering characters.

**[Derivation self-reference masking]** Input-addressed derivations
replace their own output paths with empty strings before hashing. This
circular reference is the trickiest part of the algorithm. The masking
must happen after serialization, replacing exact path strings. Mitigation:
model masking as a separate spec fn with its own correctness proof
(masked output contains no self-references).
