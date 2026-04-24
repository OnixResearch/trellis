## Context

`src/ucan_did.rs` already proves the high-level varint encode/decode round-trip
at the spec level, but the executable decoder is trusted through
`#[verifier::external_body]` and a plain Rust helper. That bridge hides the
hardest malformed-input cases: bounded u64 byte budget, shift width, arithmetic
overflow, and truncated continuations.

The current behavior rejects empty input, truncated continuation input, and
runtime overflows. The canonical spec should make that bounded-u64 validity
boundary explicit so the exec proof and requirement text agree.

## Goals / Non-Goals

**Goals:** close `ucan_did::varint_decode`, preserve DID/multicodec proofs,
make malformed bounded-u64 input behavior explicit, update proof-gap counts.

**Non-Goals:** prove base58btc, add a public DID decode API, modify UCAN CID,
close SSA bridges, or Verus-ize instruction selection.

## Decisions

### 1. Decode core is a pure bounded state machine

**Choice:** Introduce named constants inside the `verus!` block and a small
proof/exec core for bounded u64 LEB128 decode. Use names for the byte budget,
group bit width, continuation bit, payload mask, and maximum final shift instead
of raw numeric literals in verified code.

**Rationale:** A state-machine helper exposes the exact obligations that are
currently hidden by `external_body`: consumed length, accumulator bounds, shift
bounds, continuation handling, and overflow rejection.

**Implementation:** Factor decoder logic into pure spec helpers and a thin exec
wrapper:

- constants declared inside `verus!` so both spec and exec code can use them
  without Verus treating them as external Rust items
- `varint_decode_bounded_spec(bytes)` or equivalent open wrapper used by the
  public `varint_decode_spec`
- a recursive/prefix helper that carries `index`, `shift`, `acc`, and remaining
  budget
- small lemmas for empty rejection, truncated rejection, overlong rejection,
  final-byte overflow rejection, non-minimal representable acceptance, and
  encode-prefix round-trip
- `varint_decode(bytes: &Vec<u8>)` as the imperative shell with the existing
  public signature and an `ensures` contract tied to `varint_decode_spec`

### 2. Preserve existing proof consumers through wrapper lemmas

**Choice:** Keep the public spec names used by existing DID proofs, especially
`varint_decode_spec`, `varint_round_trip`, and `varint_decode_prefix_free`.
Change the internal definition only as needed to make bounded-u64 decode precise.

**Rationale:** Multicodec and DID proofs already build on these names. Keeping
that API stable contains proof churn.

**Implementation:** Re-prove existing lemmas against the bounded helper, then
reuse them in `multicodec_round_trip` and `did_key_round_trip` without changing
base58btc assumptions.

### 3. Inventory update is source-derived

**Choice:** The proof-gap count update and requirement sync are part of the
implementation, not manual prose-only edits. Evidence must derive counts from
source, reconcile README plus canonical `verification-gaps` text, and update
`docs/requirements.md` so Tracey indexes any new or clarified requirement IDs.

**Rationale:** This repo has had stale proof-gap counts before. Source-derived
count evidence reduces review risk.

**Implementation:** Capture commands or deterministic audit output for:

```bash
rg -n '^[[:space:]]*#\[verifier::external_body\]' src
rg -n 'assume_specification\[' src
rg -n '@verified-logic:no-ensures' src
rg -n '@verified-logic:exempt runtime-only instruction-selection surface' src/instruction_selection.rs
rg -n 'assume\s*\(' src/ucan_did.rs
```

Then update `docs/requirements.md`, `README.md`, and
`openspec/specs/verification-gaps/spec.md` to the same requirement/count story.
The `assume\s*\(` audit is scoped to `src/ucan_did.rs` and must show no new
local decoder-helper trust replacement.

### 4. Tracey mapping is explicit

**Choice:** Map every new or clarified requirement ID to a planned proof, test,
or source annotation before implementation.

**Rationale:** Tracey coverage comes from `docs/requirements.md`, not OpenSpec
alone, so each new or clarified ID needs an implementation and verification
anchor.

**Implementation:**

| Requirement ID | Planned anchors |
|---|---|
| `ucan-did.varint-bounded-u64` | bounded decode spec/helper, verified `varint_decode`, Ed25519 `[0xed, 0x01]` decode test, overflow/overlong/non-minimal tests |
| `ucan-did.varint-round-trip` | `varint_round_trip`, `varint_decode_prefix_free`, trailing-suffix consumed-length proof/test, existing round-trip test |
| `ucan-did.varint-rejects-empty` | `varint_decode_rejects_empty`, truncated continuation lemma/test |
| `verification-gaps.close-ucan-did-varint-decode` | source audit proving no external body or replacement trust on decoder path |
| `verification-gaps.inventory-after-ucan-did-varint` | count audit plus README/canonical spec inventory updates |

## Risks / Trade-offs

**Spec tightening may reveal malformed-input mismatch.** Mitigation: make the
bounded-u64 reject cases explicit in the change-local UCAN DID spec before code.

**Recursive helper may be easier to verify but less like the old loop.**
Mitigation: keep the public function as a thin shell; performance is irrelevant
for the tiny DID multicodec prefix.

**Count update could miss concurrent drift.** Mitigation: derive counts from the
current tree during implementation and update published totals only from that
evidence.
