# Verification Evidence

Date: 2026-05-07

## Scope

Implemented finite serialization/codec primitive admission in `src/serialization_codec.rs` and exported it from `src/lib.rs`. Canonical requirements were synced into `docs/requirements.md` and `openspec/specs/domain-primitives/spec.md`.

Implemented surfaces:

- finite byte-buffer envelopes and half-open byte range admission
- framed record header/payload/trailer range checks
- checked length-prefix metadata tied to remaining payload windows
- deterministic schema/version admission
- checksum field/covered-range shape metadata without integrity/security claims
- encoder output-shape predicates for admitted finite models
- decoder shape predicates that compose finite buffer, frame, length, schema, and optional checksum shape admission
- bounded round-trip envelopes over explicitly admitted finite models and shapes

## Commands

- `cargo test --quiet serialization_codec` — pass; 5 tests passed.
- `./scripts/verus.sh src/lib.rs 2>&1 | tee /tmp/vl-verus-serialization-codec.log` — pass; `9558 verified, 0 errors`.
- `tracey query uncovered --prefix domain-primitives.serialization-codec` — pass; `0 uncovered out of 1866 rules`.
- `tracey query untested --prefix domain-primitives.serialization-codec` — pass; `0 untested (impl but no verify) out of 1866 rules`.
- `cargo clippy --all-targets -- -D warnings` — pass.
- `openspec validate add-serialization-codec-primitives --strict --json` — pass.
- `openspec validate domain-primitives --strict --json` — pass.
- active OpenSpec strict validation loop — pass; `all-active-openspec-valid`.
- `./scripts/verify.sh 2>&1 | tee /tmp/vl-verify-serialization-codec.log` — pass; `=== ALL CHECKS PASSED ===`.

## Non-claims

This change does not claim arbitrary external codec conformance, streaming I/O, compression ratio, encryption, authentication, checksum integrity, corruption detection, transport delivery, storage durability, performance, or semantic equivalence beyond explicitly declared finite envelopes.
