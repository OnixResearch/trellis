# Repo Indexing — Validation Evidence

## Tests

`cargo test` completed clean.

- `src/lib.rs`: `3599 passed, 0 failed`
- `src/bin/repo-index.rs`: `13 passed, 0 failed`
- doc-tests: `0 passed, 0 failed`

See `test-transcript.txt`.

## Clippy

`cargo clippy --all-targets -- -D warnings` completed clean.

See `clippy-transcript.txt`.

## Verus

`./scripts/verus.sh src/lib.rs` completed with `6724 verified, 0 errors`.

See `verus-transcript.txt`.

## Tracey

Direct reruns are clean:

- `tracey query uncovered`: `0 uncovered out of 1452 rules`
- `tracey query untested`: `0 untested (impl but no verify) out of 1452 rules`

See `tracey-transcript.txt`.

## OpenSpec extraction smoke

`cargo run --bin repo-index -- requirement sel4-invocation.decode-soundness` now returns the active OpenSpec definition from committed active change file `openspec/changes/sel4-invocation-decode/specs/sel4-invocation/spec.md`.

See `openspec-query-transcript.txt`.

## Archive status

`openspec/changes/repo-indexing` is absent in the current tree (`active-absent`).

See `archive-status.txt`.
