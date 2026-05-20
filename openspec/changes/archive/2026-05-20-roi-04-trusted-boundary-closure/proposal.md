## Why

The repository currently tracks 16 trusted/proof-gap boundaries. Closing even one suitable boundary reduces the trusted base and improves the value of the proof inventory. This change scopes the next closure slice so implementation can pick a tractable target instead of mixing proof work with unrelated cleanup.

## What Changes

- Select one closeable trusted boundary from the current proof-gap inventory.
- Replace that boundary with verified logic or a narrower explicitly justified trust contract.
- Update requirements, inventory, tests, and evidence so the trusted-boundary count changes only when justified.

## Scope

In scope:
- One targeted trusted-boundary closure or narrowing.
- Positive and negative runtime tests where the boundary has runtime behavior.
- Verus proof updates and proof-gap inventory refresh.

Out of scope:
- Closing all 16 trusted boundaries in one change.
- Changing cryptographic assumptions that must remain architectural trust boundaries.
- Count updates without source/proof changes.

## Impact

- **Files**: selected `src/*.rs`, `docs/proof-gap-sites.tsv`, README counts, relevant `docs/requirements.md` and OpenSpec specs.
- **APIs**: only if the chosen closure requires a small API redesign.
- **Testing**: focused tests, Verus, proof-gap inventory checker, Tracey, full verification.
