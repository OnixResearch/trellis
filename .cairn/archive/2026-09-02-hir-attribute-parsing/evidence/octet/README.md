# Octet Identity Ratchet

The untouched `origin/main` commit `36f372f` reported 3,867 accepted IDs with no new or stale IDs.

The new `src/hir/attribute/` files have zero Octet findings. Adding the public module changed only the existing `src/lib.rs` file-length finding identity:

| Old ID | New ID | Existing finding |
|---|---|---|
| `b3:14c0af1063feb826` | `b3:9f88eccc3f56b25e` | `src/lib.rs` file length |

`migration-guard.rs` requires this exact one-for-one replacement. It preserves every unchanged finding record and rejects an accepted finding under `src/hir/attribute/`.

The accepted count remains 3,867. No warning budget or HIR attribute finding was added. The final pinned Nix check reported:

```text
Baseline: 3867 accepted, 3867 current accepted, 0 new, 0 stale
```
