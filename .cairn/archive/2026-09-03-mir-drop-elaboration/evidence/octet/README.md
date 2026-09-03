# Octet baseline migration

The first check produced three new findings. Two applied to positive-predicate
names in the new destruction-decision component. Renaming those bindings removed
both findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:088e282042beded0` | `b3:f4519fdd89bdb23e` | `src/lib.rs` file length, 1,096 to 1,099 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/destruct_paths/`. The pinned Nix Octet check must
then confirm 3,867 accepted findings with zero new or stale findings.
