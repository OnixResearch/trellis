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
- canonical payload BLAKE3 / receipt field: `f69521c26cea77d964755c64f27f9fdd18b44725945ba21c6a4c7213285f14e4`.

`trial/receipt.json` binds Octet provider revision and observed input revision `374bd16b26cee2af34211a29bfa531c016811f51`, the May 17 verifier/profile/closure identities, the complete Trellis source tree, every corpus fixed-output source, both adaptation patches and adapted trees, normalized transcripts, trust-marker inventories, and the decision.

`adaptation-review.md` records the independent semantic and proof-boundary audit. The receipt reports IronKV and allocator only as adapted-source results; it does not certify their unmodified revisions or authorize runtime adoption.

## Validation

- The corpus checker and its negative self-test passed.
- The upstream-corpus, May 17 formatter ratchet, and freshness-bound upgrade-trial Nix checks passed against the exact published Octet revision.
- Cairn revision `010b975e2a7fbe5cf965c99cb87ee48ad6eabb1f`—the last revision compatible with Trellis's generated policy schema—passed proposal, design, tasks, and full repository validation.
- Repository-wide Rust formatting drift was repaired before the final source-bound receipt; the Nix `fmt` check passes.
- The Octet baseline was deterministically refreshed for six formatting-induced stable-ID replacements; the focused Octet check passes with no new debt.
- The full `x86_64-linux` flake check passes, including all `4871` nextest cases.
- Generic `git diff --check` passes for non-patch files. The two unified-diff artifacts retain required single-space blank context markers; `git diff --check` reports those patch-syntax markers as trailing whitespace, but deleting them would corrupt the reviewable/applicable patch.

## Promotion state

Octet revision `374bd16b26cee2af34211a29bfa531c016811f51` is published and locked from GitHub. Trellis's shipped proof check and wrapper consume the Octet May 17 package; the March package remains only as the receipt's historical comparison baseline.
