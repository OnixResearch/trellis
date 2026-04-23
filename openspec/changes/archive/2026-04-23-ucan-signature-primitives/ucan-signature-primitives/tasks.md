## Phase 1: Reconcile landed scope with change artifacts

- [x] Confirm the landed module surface in `src/ucan_signature.rs`: `SecretKey` is a 32-byte seed, `PublicKey` is 32 bytes, `Ed25519Signature` is 64 bytes, and the public exec surface is `sign`, `verify`, and `public_from_secret`
- [x] Confirm `docs/requirements.md` already carries the `r[ucan-signature.*]` requirements used by the landed module
- [x] Sync `proposal.md`, `design.md`, and `specs/ucan-signature/spec.md` to the landed scope by removing the stale `keypair_generate` commitment, fixing the secret-key representation to a 32-byte seed, aligning length capability names, and documenting the byte-level token-validation boundary

## Phase 2: Traceability and requirement coverage

- [x] Ensure every requirement and scenario in `specs/ucan-signature/spec.md` has an `r[...]` ID line
- [x] Ensure the change artifacts explicitly cover determinism, 64-byte signature length, and 32-byte public-key length as landed obligations
- [x] Ensure the tasks track archive-closeout work instead of greenfield implementation work

## Phase 3: Baseline and fresh validation evidence

- [x] Run a focused baseline test before artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_signature`
- [x] Re-run focused runtime tests after artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_sign_verify_round_trip`, `... test_wrong_key_rejects`, `... test_wrong_message_rejects`, `... test_sign_deterministic`, and `... test_public_key_length`
- [x] Re-run explicit signature-width validation for `r[ucan-signature.signature-length]`: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_sign_verify_round_trip`
- [x] Run fresh `openspec_gate` tool checks for `proposal`, `design`, and `tasks`, and explicitly audit the current `specs/ucan-signature/spec.md` text against any gate findings before archive
- [x] Capture a fresh post-sync full-module runtime transcript: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_signature`
- [x] Capture fresh proof-oriented validation evidence for the landed module, including `./scripts/verus.sh src/lib.rs` and the focused runtime test transcripts
- [x] Capture fresh archive evidence for the validated landed scope, including the gate results, proof/runtime transcripts, the archive move result, and the post-archive active-change recount output

## Phase 4: Archive

- [x] Move the synced change under `openspec/changes/archive/<date>-ucan-signature-primitives/`
- [x] Recheck active change count after archive so the autoresearch metric reflects the closeout
