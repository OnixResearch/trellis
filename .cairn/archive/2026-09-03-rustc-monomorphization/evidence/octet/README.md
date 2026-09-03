# Octet baseline migration

The first check produced no finding in the new item-partition component. It
reported only the existing crate-root file-length identity change.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:c0204b3016fbaccb` | `b3:b11221ab391e117f` | `src/lib.rs` file length, 1,084 to 1,087 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/instance_partition/`. The pinned Nix Octet check
must then confirm 3,867 accepted findings with zero new or stale findings.
