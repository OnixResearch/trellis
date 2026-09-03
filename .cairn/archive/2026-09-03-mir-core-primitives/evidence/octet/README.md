# Octet baseline migration

The implementation first produced 14 new findings. Thirteen applied to the new
MIR-core component. Fixed-width public indices, owned edge buffers, and a
non-repeating module path removed all thirteen findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:6922f47a9ce649d3` | `b3:7255e092be9f2780` | `src/lib.rs` file length, 1,072 to 1,075 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/mir_core/`. The pinned Nix Octet check must then
confirm 3,867 accepted findings with zero new or stale findings.
