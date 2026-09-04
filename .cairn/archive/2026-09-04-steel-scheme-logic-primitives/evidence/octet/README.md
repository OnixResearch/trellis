# Octet baseline migration

The first pinned source check reported 13 new findings and one stale finding.
Twelve findings belonged to the initial scheme-logic layout. The remaining
finding was the existing `src/lib.rs` file-length observation after adding the
public module.

Owned queue parameters, checked fixed-width conversions, split control flow,
non-repeating file vocabulary, and bounded helpers removed every component
finding. No exception, disabled lint, warning budget, or component baseline was
added.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the four-line
module documentation and declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:a126ebe356b97c05` | `b3:370738e60362a2d8` | `src/lib.rs` file length, 1,109 to 1,113 lines |

`migration-guard.rs` derives and requires that exact set difference. It
preserves all other 3,866 records at the decoded record level. It also rejects
any accepted finding under `src/scheme_logic/`.

The pinned Nix Octet check reports 3,867 accepted findings with zero new or
stale findings.
