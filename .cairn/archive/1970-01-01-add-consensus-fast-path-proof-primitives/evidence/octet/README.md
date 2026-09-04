# Octet baseline migration

The first source check reported 26 new findings. Twenty-five findings belonged
to the initial consensus fast-path layout. The remaining finding was the
existing `src/lib.rs` file-length observation after adding the public module.

The component now uses seven top-level source files. Qualified owner paths,
positive predicate names, bounded recovery sets, and non-repeating item names
removed every component finding without an exception or warning budget.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:8615a1197127a747` | `b3:9eb4e48841031554` | `src/lib.rs` file length, 1,102 to 1,105 lines |

`migration-guard.rs` requires that exact replacement. It preserves all other
3,866 records at the decoded record level. It also rejects any accepted
finding under `src/consensus_fast_path/`.

The pinned Nix Octet check must then report 3,867 accepted findings with zero
new or stale findings.
