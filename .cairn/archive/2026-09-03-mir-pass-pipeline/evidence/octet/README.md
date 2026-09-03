# Octet Identity Ratchet

The untouched `origin/main` commit `df0d012` reported 3,867 accepted IDs with no new or stale IDs.

The new `src/mir_pass_pipeline/` files initially exposed 31 Octet findings. The implementation removed all of them by preserving qualified owner paths, using path-local item names, and using a predicate-form Boolean binding.

The final new module has zero Octet findings. Adding its three-line crate-root declaration changed only the existing `src/lib.rs` file-length finding:

| Old ID | New ID | Existing finding |
|---|---|---|
| `b3:439339c318e5657f` | `b3:6922f47a9ce649d3` | `src/lib.rs` file length, 1,069 to 1,072 lines |

`migration-guard.rs` requires this exact one-for-one replacement. It preserves every unchanged finding record and rejects an accepted finding under `src/mir_pass_pipeline/`.

The accepted count remains 3,867. No warning budget or MIR pass finding was added. The final pinned Nix check reported:

```text
Baseline: 3867 accepted, 3867 current accepted, 0 new, 0 stale
```
