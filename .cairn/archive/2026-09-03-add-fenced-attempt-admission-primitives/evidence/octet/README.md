# Octet baseline migration

The first check produced thirty-one new findings. All thirty applied to the new
fenced-attempt component: qualified owner paths, path-local type names,
positive-predicate bindings, and an import-free fmt path removed every one of
them without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:f4519fdd89bdb23e` | `b3:8615a1197127a747` | `src/lib.rs` file length, 1,099 to 1,102 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/fenced_attempt/`. The pinned Nix Octet check must
then confirm 3,867 accepted findings with zero new or stale findings.
