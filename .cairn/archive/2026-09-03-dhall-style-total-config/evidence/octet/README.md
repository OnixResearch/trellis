# Octet baseline migration

The first check produced four new findings. Three applied to the typed normal-form
component. Merging narrow modules, splitting cycle admission, and using a
non-repeating graph path removed all three findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:4a37b4f2612a74b5` | `b3:b7e9d6a957701268` | `src/lib.rs` file length, 1,090 to 1,093 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/normal_input/`. The pinned Nix Octet check must then
confirm 3,867 accepted findings with zero new or stale findings.
