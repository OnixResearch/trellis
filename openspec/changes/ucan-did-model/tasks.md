## Phase 1: Reconcile landed scope and codec framing

- [x] Sync `proposal.md`, `design.md`, `tasks.md`, and `specs/ucan-did/spec.md` to the landed `src/ucan_did.rs` scope by removing the stale generic `Did` commitment and keeping `DidKey` as the validated public DID carrier
- [x] Reconcile multicodec wording so artifacts consistently distinguish codec value `0xed` from its varint bytes `[0xed, 0x01]`
- [x] Ensure every requirement and scenario in `specs/ucan-did/spec.md` has an `r[...]` ID line

## Phase 2: Requirement coverage against landed module

- [x] Confirm the landed module covers varint round-trip, empty-input rejection, DID round-trip, injectivity, prefix validity, codec preservation, and base58 round-trip
- [x] Add explicit closeout coverage for truncated non-empty varint rejection and wrong-prefix / malformed-payload DID rejection in the change artifacts
- [x] Decide how `base58btc` invalid-character rejection is represented in the landed boundary: explicit runtime test, explicit trusted external-body assumption, or both
- [x] Ensure the `DidKey` validated-newtype boundary is described and traced to the parser/constructor behavior that the landed module actually provides

## Phase 3: Baseline and fresh validation evidence

- [x] Run a focused baseline test before artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_did`
- [x] Re-run fresh focused runtime validation after artifact sync, including varint round-trip, empty-input rejection, did:key prefix validity, base58 round-trip, and distinct-key DID separation
- [ ] Run fresh `openspec_gate` tool checks for `proposal`, `design`, and `tasks`, and explicitly audit `specs/ucan-did/spec.md` against the findings
- [x] Capture fresh proof-oriented validation evidence for the landed module, including `./scripts/verus.sh src/lib.rs` and the focused `ucan_did` runtime transcript

## Phase 4: Archive

- [ ] Capture fresh archive evidence for the validated landed scope, including gate results, proof/runtime transcripts, archive move result, and post-archive active-change recount output
- [ ] Move the synced change under `openspec/changes/archive/<date>-ucan-did-model/`
- [ ] Recheck active change count after archive so the autoresearch metric reflects the closeout
