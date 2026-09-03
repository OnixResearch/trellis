# Octet baseline migration

The first run produced two new findings. One applied to the new builder-record
component. Splitting block-assignment validation removed that function-length
finding without an exception.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:d799f03fd91835c1` | `b3:c0204b3016fbaccb` | `src/lib.rs` file length, 1,081 to 1,084 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/builder_lower/`. The pinned Nix Octet check must
then confirm 3,867 accepted findings with zero new or stale findings.
