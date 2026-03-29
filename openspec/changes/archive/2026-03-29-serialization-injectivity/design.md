## Context

Three modules axiomatize hash collision resistance over fixed-width u64
inputs:

| Module | Hash function | Arity | Collision axiom |
|---|---|---|---|
| `chain` | `hash_spec(prev, index, term, data_id)` | 4 × u64 | `hash_collision_free` |
| `snapshot_integrity` | `dual_hash_spec(a, b)` | 2 × u64 | `dual_hash_collision_resistant` |
| `merkle` | reuses `hash_spec` with 0-padding | 2 × u64 (effective) | via chain |

All three implicitly serialize structures as fixed-width field
concatenations. The injectivity of this encoding is obvious to a
human but never stated as a theorem. This change makes it explicit.

## Goals / Non-Goals

**Goals:** Prove that canonical byte encodings are injective. Compose
with existing collision-resistance axioms to get structure-level
injectivity. Provide reusable building blocks for future modules that
hash richer structures.

**Non-Goals:** Changing the hash function abstraction. The existing
`hash_spec` / `dual_hash_spec` axiomatizations remain as-is. We're
not implementing runtime serialization — this is spec-level reasoning
about byte layouts. Not implementing LEB128 or other variable-length
integer encodings yet (those can come later when a module needs them).

## Decisions

### 1. Spec-level `Seq<u8>`, not runtime `Vec<u8>`

**Choice:** All encode functions are `spec fn` operating on `Seq<u8>`.
Exec wrappers exist where useful for testing but the proofs target the
spec level.
**Rationale:** The point is proving properties of the encoding scheme,
not building a serialization library. Keeping it ghost-only means zero
runtime cost and simpler proof obligations — no overflow checks, no
allocation failure, no mut references.
**Alternative:** Runtime-first with ensures. Rejected — adds proof
complexity (loop invariants for byte copying) with no payoff. Downstream
modules already have their own exec code; they just need the injectivity
lemma at the spec level.

### 2. Fixed-width first, variable-length later

**Choice:** Start with u64 fixed-width encoding and struct-of-u64
compositions. Length-prefixed and tagged encodings are included but
at the `Seq<u8>` payload level, not with runtime parsers.
**Rationale:** Every current consumer (chain, snapshot_integrity, merkle)
uses fixed-width u64 fields. Fixed-width injectivity proofs are
straightforward positional splitting. Variable-length encodings
(LEB128, length-delimited strings) are future work when a module
needs them.

### 3. Injectivity stated as `spec → ensures`, not as standalone axioms

**Choice:** Each injectivity result is a `proof fn` with requires/ensures,
not an `external_body` axiom.
**Rationale:** These are provable from first principles (sequence
indexing, length reasoning). Adding axioms would weaken the trust model.
The whole point is that this half of the argument is NOT trusted — only
the collision-resistance half is.

### 4. Composition theorem uses generic predicate, not specific hash

**Choice:** `hash_compose_injective` takes a generic
`spec fn hash(Seq<u8>) -> u64` and a proof of its collision-freeness,
then composes with serialization injectivity.
**Rationale:** Works for both `hash_spec` (4-arg) and `dual_hash_spec`
(2-arg) by first encoding args to `Seq<u8>`. Avoids duplicating the
composition proof per hash function. In practice, the cross-module
proofs in `properties.rs` instantiate it for each concrete hash.

Actually — Verus doesn't have higher-order proof arguments (no
`FnSpec` in proof signatures the way we'd need). Instead: state the
composition as a standalone lemma per hash function.

**Revised choice:** Two composition proofs:
- `chain_hash_struct_injective` — for 4-field chain entries
- `dual_hash_struct_injective` — for 2-field snapshot pairs

Each chains `encode_*_injective` with the module's collision-resistance
axiom directly.

### 5. Encode spec uses `Seq::subrange` for field extraction

**Choice:** Injectivity proofs extract fields via
`encoded.subrange(offset, offset + 8)` and appeal to
`u64_from_le_bytes_injective`.
**Rationale:** Verus handles `Seq` indexing and subrange equality well.
The key lemma is: if two length-8 subsequences at the same offset are
equal, the u64 values they encode are equal. This is `u64_to_le_bytes`
followed by `u64_from_le_bytes` — a roundtrip.

## Risks / Trade-offs

**Z3 rlimit on sequence reasoning** → Sequence subrange equality with
arithmetic offsets can cause Z3 to struggle. Mitigation: split proofs
into per-field helper lemmas (one assert per field position) rather
than a single quantified statement.

**Spec-only encode fns are untestable** → The spec functions produce
ghost `Seq<u8>` which can't be executed. Mitigation: provide thin exec
wrappers that proptest can exercise for roundtrip testing. The exec
wrappers are trivial (just copy bytes into a Vec) and their ensures
clauses link to the specs.
