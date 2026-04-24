## Context

`src/ucan_did.rs` already proves the high-level varint encode/decode round-trip
at the spec level, but the executable decoder is trusted through
`#[verifier::external_body]` and a plain Rust helper. That bridge hides the
hardest malformed-input cases: bounded u64 byte budget, arithmetic overflow,
and truncated continuations.

The current behavior rejects empty input, truncated continuation input, and
runtime overflows. The canonical spec should make that bounded-u64 validity
boundary explicit so the checked-arithmetic exec proof and requirement text
agree.

## Goals / Non-Goals

**Goals:** close `ucan_did::varint_decode`, preserve DID/multicodec proofs,
make malformed bounded-u64 input behavior explicit, update proof-gap counts.

**Non-Goals:** prove base58btc, add a public DID decode API, modify UCAN CID,
close SSA bridges, or Verus-ize instruction selection.

## Decisions

### 1. Decode core is a pure bounded state machine

**Choice:** Introduce named constants inside the `verus!` block and a small
proof/exec core for bounded u64 LEB128 decode. The verified decoder path carries
`VARINT_BASE`, `VARINT_CONTINUATION_BIT`, `VARINT_PAYLOAD_MASK`, and
`MAX_U64_VARINT_BYTES` as named values instead of raw numeric literals.
`MAX_U64_VARINT_BYTES` is the u64 LEB128 budget of `10` payload groups.

**Rationale:** A state-machine helper exposes the exact obligations that are
currently hidden by `external_body`: consumed length, accumulator bounds,
continuation handling, and checked-arithmetic overflow rejection.

**Implementation:** Factor decoder logic into pure spec helpers and a thin exec
wrapper:

- constants declared inside `verus!` so both spec and exec code can use them
  without Verus treating them as external Rust items
- `varint_decode_bounded_spec(bytes)` or equivalent open wrapper used by the
  public `varint_decode_spec`
- a recursive/fuel helper that carries the current offset and remaining byte
  budget, then composes tail results with checked u64 multiplication by
  `VARINT_BASE` and checked u64 addition of the current payload
- small lemmas for empty rejection, single-continuation truncation
  (`varint_decode_rejects_single_continuation`), continuation-only overlong
  rejection (`varint_decode_rejects_overlong_continuations`), final-payload
  overflow rejection (`varint_decode_rejects_overflowing_final_payload`),
  non-minimal zero acceptance (`varint_decode_accepts_nonminimal_zero`), and
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

Then update `docs/requirements.md`, `README.md`,
`openspec/specs/ucan-did/spec.md`, and
`openspec/specs/verification-gaps/spec.md` to the same requirement/count story.
The canonical `openspec/specs/ucan-did/spec.md` must carry the same checked
`VARINT_BASE * tail_value + payload` bounded-varint semantics as the
change-local UCAN DID spec before archive. The `assume\s*\(` audit is scoped to
`src/ucan_did.rs` and must show no new local decoder-helper trust replacement.

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

## Validation and evidence matrix

Phase 0 baseline evidence MUST be captured before implementation evidence is
claimed:

| Artifact | Command / source |
|---|---|
| `evidence/baseline-verus-ucan-did.txt` | `./scripts/verus.sh src/ucan_did.rs` |
| `evidence/baseline-cargo-test-ucan-did.txt` | `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did` |
| `evidence/baseline-proof-gap-counts.txt` | source audits for `external_body`, `assume_specification[...]`, `@verified-logic:no-ensures`, instruction-selection exemptions, and `rg -n 'assume\\s*\\(' src/ucan_did.rs`; the artifact records the expected baseline total `44` and is reconciled against the final `44 -> 43` delta |

Final validation evidence MUST include:

| Artifact | Command / source |
|---|---|
| `evidence/final-verus-ucan-did.txt` | `./scripts/verus.sh src/ucan_did.rs` |
| `evidence/final-verus-lib.txt` | `./scripts/verus.sh src/lib.rs` |
| `evidence/final-cargo-test-ucan-did.txt` | `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did` |
| `evidence/final-check-ensures.txt` | `./scripts/check-ensures.sh` |
| `evidence/final-tracey.txt` | `tracey query status`, then `tracey query uncovered`, then `tracey query untested` |
| `evidence/final-proof-gap-counts.txt` | source-derived count audit reconciled with README and canonical spec totals |
| `evidence/final-decoder-trust-audit.txt` | scoped decoder-path audit for absence of replacement trust, including `rg -n 'fn varint_decode|varint_decode_spec|varint_decode_fuel|checked_(mul|add)_u64|varint_decode_rejects|varint_decode_accepts' src/ucan_did.rs` and `rg -n 'external_body|assume\\s*\\(|assume_specification\\[|@verified-logic:no-ensures|@verified-logic:exempt' src/ucan_did.rs` |
| `evidence/final-semantic-reconciliation.txt` | deterministic pre-archive audit proving `openspec/specs/ucan-did/spec.md`, the change-local UCAN DID spec, and `src/ucan_did.rs` all carry the checked `VARINT_BASE * tail_value + payload` bounded-varint semantics; command: for each spec file, run separate `rg -n 'VARINT_BASE \\* tail_value' <file>`, `rg -n 'checked u64 operations' <file>`, `rg -n 'first `MAX_U64_VARINT_BYTES` bytes' <file>`, and `rg -n 'explicit proof artifacts' <file>` checks, then run source checks `rg -n 'checked_mul_u64_spec\\(rest_value, VARINT_BASE\\)' src/ucan_did.rs`, `rg -n 'checked_add_u64_spec\\(low, scaled_rest\\)' src/ucan_did.rs`, `rg -n 'checked_mul_u64\\(rest_value, VARINT_BASE\\)' src/ucan_did.rs`, and `rg -n 'checked_add_u64\\(low, scaled_rest\\)' src/ucan_did.rs` |
| `evidence/final-openspec-gates.txt` | `openspec_gate proposal`, `design`, and `tasks` for this change |

## Risks / Trade-offs

**Spec tightening may reveal malformed-input mismatch.** Mitigation: make the
bounded-u64 reject cases explicit in the change-local UCAN DID spec before code.

**Recursive helper may be easier to verify but less like the old loop.**
Mitigation: keep the public function as a thin shell; performance is irrelevant
for the tiny DID multicodec prefix.

**Count update could miss concurrent drift.** Mitigation: derive counts from the
current tree during implementation and update published totals only from that
evidence.
