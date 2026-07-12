# Repair evidence

The canonical trial is saved under `trial/` and is recursively freshness-checked by the Nix derivation.

## Result

- baseline Trellis: `10712 verified, 0 errors`, exit `0`;
- Octet May 17 Trellis: `10712 verified, 0 errors`, exit `0`;
- adapted IronKV: `312 verified, 0 errors`, linked exit `0`;
- adapted verified-memory-allocator: `732 verified, 0 errors`, linked exit `0`;
- unmodified verified-node-replication: `254 verified, 0 errors`, exit `0`;
- timeouts: none;
- decision: `promote`, promotion eligible `true`;
- canonical payload BLAKE3 / receipt field: `2a37daa5540a10917e8c320fd2ccc4d28c48aaae7326f673430628265c7bc40d`.

`trial/receipt.json` binds Octet provider revision and observed input revision `374bd16b26cee2af34211a29bfa531c016811f51`, the May 17 verifier/profile/closure identities, the complete Trellis source tree, every corpus fixed-output source, both adaptation patches and adapted trees, normalized transcripts, trust-marker inventories, and the decision.

`adaptation-review.md` records the independent semantic and proof-boundary audit. The receipt reports IronKV and allocator only as adapted-source results; it does not certify their unmodified revisions or authorize runtime adoption.

## Validation

- The corpus checker and its negative self-test passed.
- The upstream-corpus, May 17 formatter ratchet, and freshness-bound upgrade-trial Nix checks passed against the exact Octet revision.
- Cairn revision `010b975e2a7fbe5cf965c99cb87ee48ad6eabb1f`—the last revision compatible with Trellis's generated policy schema—passed proposal, design, tasks, and full repository validation.
- The broad flake check reached the pre-existing `fmt` rail and failed on committed formatting drift in `src/raft_consensus_core.rs`; that unrelated file was not changed or reformatted by this change.
- Generic `git diff --check` passes for non-patch files. The two unified-diff artifacts retain required single-space blank context markers; `git diff --check` reports those patch-syntax markers as trailing whitespace, but deleting them would corrupt the reviewable/applicable patch.

## Remaining blocker

The Octet revision is committed locally but is not published on `origin`. The exact `git+file` override is valid local trial evidence, not a durable product dependency. Trellis therefore retains its shipped verifier pin until explicit push authorization allows Octet publication and the Trellis GitHub input/lock can be updated and revalidated without a workspace-relative override.
