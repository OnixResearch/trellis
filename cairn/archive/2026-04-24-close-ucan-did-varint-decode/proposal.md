## Why

`src/ucan_did.rs::varint_decode` is one of the smallest remaining closeable
trusted boundaries in the repo. It is currently `#[verifier::external_body]`
even though the surrounding DID model already has a Verus varint spec,
round-trip lemmas, multicodec framing proofs, and focused runtime tests.

Keeping this bridge trusted weakens every proof that parses a multicodec prefix.
Closing it is high ROI because the proof shape is local, reusable, and reduces
the audited proof-gap inventory without touching cryptographic base58btc trust.

## What Changes

- Replace the trusted `varint_decode` wrapper with verified bounded u64 LEB128
  decoder primitives in `src/ucan_did.rs`, with verifier-visible constants
  declared inside the `verus!` block.
- Make the spec/exec validity boundary explicit: decode consumes at most the
  named u64 varint byte budget, rejects empty/truncated/overlong/overflowing
  inputs, and returns a consumed length within the input.
- Preserve existing `varint_encode` round-trip and multicodec prefix proofs.
- Add explicit proof artifacts for single-continuation truncation,
  non-minimal zero acceptance, continuation-only overlong rejection, and
  final-payload overflow rejection.
- Remove the `#[verifier::external_body]` site from `ucan_did::varint_decode`
  without adding `assume(...)`, `assume_specification[...]`, or a replacement
  trusted bridge.
- Refresh `openspec/specs/ucan-did/spec.md`, `docs/requirements.md`,
  `openspec/specs/verification-gaps/spec.md`, and README proof-gap counts from
  fresh source-derived evidence.

## Capabilities

### Modified Capabilities

- `ucan-did.varint-round-trip`: Keep encode/decode round-trip, now backed by a
  verified bounded runtime decoder.
- `ucan-did.varint-rejects-empty`: Clarify that empty and truncated
  continuation varints reject.
- `ucan-did.varint-bounded-u64`: Make bounded u64 LEB128 decode semantics an
  explicit proof target, including overlong/overflow rejection and non-minimal
  representable acceptance.
- `verification-gaps.close-ucan-did-varint-decode`: Reduce the closeable trust
  inventory by closing the `ucan_did::varint_decode` external body.
- `verification-gaps.inventory-after-ucan-did-varint`: Keep README and canonical
  proof-gap inventory totals source-derived after the boundary closes.

## Impact

- **Files**: planned implementation touches `src/ucan_did.rs`,
  `docs/requirements.md`, `openspec/specs/ucan-did/spec.md`,
  `openspec/specs/verification-gaps/spec.md`, `README.md`, and change-local
  evidence under this directory.
- **APIs**: public Rust API shape stays `varint_decode(&Vec<u8>) ->
  Option<(u64, usize)>`; behavior is documented more precisely for malformed
  bounded-u64 inputs.
- **Dependencies**: none.
- **Testing / Validation**:
  - `./scripts/verus.sh src/ucan_did.rs`
  - `./scripts/verus.sh src/lib.rs`
  - `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did`
  - `./scripts/check-ensures.sh`
  - `tracey query status` to start/check the daemon, then sequential
    `tracey query uncovered` and `tracey query untested`
  - `openspec/changes/close-ucan-did-varint-decode/evidence/baseline-verus-ucan-did.txt`
    from `./scripts/verus.sh src/ucan_did.rs`
  - `openspec/changes/close-ucan-did-varint-decode/evidence/baseline-cargo-test-ucan-did.txt`
    from `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did`
    captured as a Phase 0 implementation task, because the selected ROI report
    identified this as missing context
  - `openspec/changes/close-ucan-did-varint-decode/evidence/baseline-proof-gap-counts.txt`
    and `openspec/changes/close-ucan-did-varint-decode/evidence/final-proof-gap-counts.txt`
    with exact source/count audits showing the proof-gap total falls from `44`
    to `43`; commands include `rg -n '^[[:space:]]*#\\[verifier::external_body\\]' src`,
    `rg -n 'assume_specification\\[' src`, `rg -n '@verified-logic:no-ensures' src`,
    `rg -n '@verified-logic:exempt runtime-only instruction-selection surface' src/instruction_selection.rs`,
    and `rg -n 'assume\\s*\\(' src/ucan_did.rs`
  - canonical `openspec/specs/ucan-did/spec.md` checked against the
    change-local checked-mul/add bounded-varint semantics before archive
  - `openspec_gate proposal/design/tasks close-ucan-did-varint-decode`

## Out of Scope

- Base58btc encode/decode proof closure.
- DID runtime decode API expansion; DID decode remains spec-only.
- UCAN CID varint implementation changes.
- SSA bridge closure or instruction-selection runtime-only proof work.
