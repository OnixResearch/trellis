# Octet Identity Ratchet

The untouched `origin/main` commit `470a0db` reported 3,867 accepted IDs with no new or stale IDs.

The new `src/rustc/metadata/` files have zero Octet findings. Adding the public module changed only the existing `src/lib.rs` file-length finding identity:

| Old ID | New ID | Existing finding |
|---|---|---|
| `b3:9f88eccc3f56b25e` | `b3:439339c318e5657f` | `src/lib.rs` file length |

`migration-guard.rs` requires this exact one-for-one replacement. It preserves every unchanged finding record and rejects an accepted finding under `src/rustc/metadata/`.

The accepted count remains 3,867. No warning budget or rustc metadata finding was added. The final pinned Nix check reported:

```text
Baseline: 3867 accepted, 3867 current accepted, 0 new, 0 stale
```
