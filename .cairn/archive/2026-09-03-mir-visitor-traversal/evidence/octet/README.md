# Octet baseline migration

The first run produced six new findings. Five applied to the new visitor
component. Exhaustive enum matching, bounded output allocation, positive
Boolean naming, and non-repeating file paths removed all five findings.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:7255e092be9f2780` | `b3:572cd9d794f63231` | `src/lib.rs` file length, 1,075 to 1,078 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/mir_walk/`. The pinned Nix Octet check must then
confirm 3,867 accepted findings with zero new or stale findings.
