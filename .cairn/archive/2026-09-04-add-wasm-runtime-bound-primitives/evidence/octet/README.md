# Octet baseline migration

The first pinned source check reported seven new findings and one stale finding.
Six findings belonged to the initial runtime-bound component. The remaining
finding was the existing `src/lib.rs` file-length observation after adding the
public module.

Typed request input, non-repeating names, positive predicate names, fixed-width
public counts, and explicit conversion failure removed every component finding.
No exception, disabled lint, warning budget, or component baseline was added.

The final pre-migration check reported one new and one stale finding. Both are
the existing `src/lib.rs` file-length observation after adding the four-line
module documentation and declaration.

| Stale ID | Replacement ID | Bounded change |
|---|---|---|
| `b3:9eb4e48841031554` | `b3:a126ebe356b97c05` | `src/lib.rs` file length, 1,105 to 1,109 lines |

`migration-guard.rs` requires that exact replacement. It preserves all other
3,866 records at the decoded record level. It also rejects any accepted finding
under `src/wasm_runtime_bounds/`.

The pinned Nix Octet check reports 3,867 accepted findings with zero new or
stale findings.
