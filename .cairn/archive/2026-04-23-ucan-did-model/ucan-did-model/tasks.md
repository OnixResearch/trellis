## Phase 1: Reconcile landed scope and codec framing

- [x] Sync `proposal.md`, `design.md`, `tasks.md`, and `specs/ucan-did/spec.md` to the landed `src/ucan_did.rs` scope by removing the stale generic `Did` commitment, keeping `DidKey` as the encode-side public carrier, and documenting that DID decode remains a spec-only byte parser rather than a public exec API
- [x] Reconcile multicodec wording so artifacts consistently distinguish codec value `0xed` from its varint bytes `[0xed, 0x01]`
- [x] Ensure every requirement and scenario in `specs/ucan-did/spec.md` has an `r[...]` ID line

## Phase 2: Requirement coverage against landed module

- [x] Confirm the landed module covers varint round-trip, empty/truncated varint rejection, DID round-trip, injectivity, prefix validity, codec preservation, and base58 round-trip
- [x] Clarify the validity boundary: the landed module preserves arbitrary codec values and opaque key bytes through did:key framing and does not impose codec-specific key-length validation
- [x] Decide how base58btc behavior is represented in the landed boundary: round-trip is a Tracey-scoped obligation, while malformed external DID parsing remains outside the public exec surface of this change
- [x] Ensure the `DidKey` encode-side carrier boundary is described and traced to the parser/constructor behavior that the landed module actually provides

## Phase 3: Baseline and fresh validation evidence

- [x] Run a focused baseline test before artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_did`
- [x] Re-run fresh focused runtime validation after artifact sync, including varint round-trip, empty-input rejection, truncated-continuation rejection, did:key prefix validity, base58 round-trip, and distinct-key DID separation
- [x] Run fresh `openspec_gate` tool checks for `proposal`, `design`, and `tasks`, and explicitly audit `specs/ucan-did/spec.md` against the findings
- [x] Capture fresh proof-oriented validation evidence for the landed module, including `./scripts/verus.sh src/lib.rs` and the focused `ucan_did` runtime transcript

## Phase 4: Archive

- [x] Capture fresh archive evidence for the validated landed scope, including gate results, proof/runtime transcripts, archive move result, and post-archive active-change recount output
- [x] Move the synced change under `openspec/changes/archive/<date>-ucan-did-model/`
- [x] Recheck active change count after archive so the autoresearch metric reflects the closeout