# Octet baseline migration

The first check produced three new findings. Two applied to the new finite
analysis component. Moving transfer helpers to a non-repeating table module
removed both findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:b7e9d6a957701268` | `b3:088e282042beded0` | `src/lib.rs` file length, 1,093 to 1,096 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/finite_flow/`. The pinned Nix Octet check must then
confirm 3,867 accepted findings with zero new or stale findings.
