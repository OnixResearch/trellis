# Octet Baseline Ratchet

The untouched `origin/main` commit `644603b` reported `0 new` and `20 stale` IDs. The current tree initially reported six replacement IDs and 26 stale IDs.

The six replacements cover existing findings whose normalized source identity changed in these files:

| Old ID | New ID | Existing finding |
|---|---|---|
| `b3:644af5b3acfa7b8f` | `b3:14c0af1063feb826` | `src/lib.rs` file length |
| `b3:d3df2446875e7eca` | `b3:14dd51bc32612746` | `src/hdl_adapters.rs` file length |
| `b3:6900b446747b5e73` | `b3:8e8dca620c694e29` | `src/hdl_adapters.rs` filename |
| `b3:d27ceebca886d503` | `b3:aafb1f64a9bc1288` | `hdl_source_result` path repetition |
| `b3:0ede41f0aeefc478` | `b3:c0180f11505c2d3a` | `src/hdl_ir.rs` filename |
| `b3:414959bcb389d1ee` | `b3:8959af256d377c72` | `hdl_next_state` path repetition |

`migration-guard.rs` proves that the migrated ID set equals the old set minus exactly 26 stale IDs plus these six replacements. It preserves every unchanged finding record and rejects any accepted finding under `src/track/caller/`.

The accepted count decreased from 3,887 to 3,867. No warning budget or new track-caller finding was added. The final pinned Nix check reported:

```text
Baseline: 3867 accepted, 3867 current accepted, 0 new, 0 stale
```
