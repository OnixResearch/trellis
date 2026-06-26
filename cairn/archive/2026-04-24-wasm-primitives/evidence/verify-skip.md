# verify.sh Skip Note

`./scripts/verify.sh` remains intentionally skipped for this archive because the
repo still has an unrelated `r[impl]` coverage blocker in
`src/instruction_selection.rs`, documented in `.agent/napkin.md`.

This Wasm archive instead uses the focused proof/integration rail promised in the
proposal and design:

- `cargo test --quiet wasm_`
- `./scripts/verus.sh src/wasm_primitives.rs`
- `./scripts/verus.sh src/lib.rs`
- `./scripts/check-ensures.sh`
- `tracey query uncovered`
- `tracey query untested`
- `./autoresearch.sh` / `autoresearch.checks.sh`
