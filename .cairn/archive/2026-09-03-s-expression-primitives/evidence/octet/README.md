# Octet baseline migration

The first check produced eight new findings. Seven applied to the canonical-tree
component. Explicit bounded stacks replaced recursive walkers, narrow output
state replaced borrowed owned strings, and path-local names removed the
remaining findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:b11221ab391e117f` | `b3:4a37b4f2612a74b5` | `src/lib.rs` file length, 1,087 to 1,090 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/paren_tree/`. The pinned Nix Octet check must then
confirm 3,867 accepted findings with zero new or stale findings.
