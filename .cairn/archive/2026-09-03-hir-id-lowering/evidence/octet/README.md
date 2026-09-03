# Octet baseline migration

The first run produced nine new findings. Eight applied to the new HIR ID
component. Path-local vocabulary that does not repeat the public module path
removed all eight findings without exceptions.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the three-line
public module declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:572cd9d794f63231` | `b3:d799f03fd91835c1` | `src/lib.rs` file length, 1,078 to 1,081 lines |

`migration-guard.rs` requires that exact replacement, preserves all other
3,866 records byte-for-byte at the decoded record level, and rejects any
accepted finding under `src/frontend_lower/`. The pinned Nix Octet check must
then confirm 3,867 accepted findings with zero new or stale findings.
