# Octet migration evidence

The baseline passed before the crate-root Tracey bridge was added.

The bridge appends requirement references to `src/lib.rs`. It does not change executable library code or any earlier source line.

The first pinned Nix run then reported:

- 3,867 accepted findings
- 3,866 current accepted findings
- 1 new finding
- 1 stale finding

The only replacement is:

- stale `b3:69dd8f7e6f7922b2`
- current `b3:0a9b4eca7243e493`
- file `src/lib.rs`
- lint `excessive_file_length`
- line 1 and column 1

The message changed from 1,117 lines to 1,145 lines. No other accepted finding record changed.

`migration-guard.rs` checks the exact replacement, stable metadata, counts, and every unchanged finding. It then writes the guarded baseline.

The final pinned Nix check reports 3,867 accepted and current findings, with zero new and zero stale findings.

No warning budget, disabled lint, suppression, broad migration, or baseline replacement was used.
