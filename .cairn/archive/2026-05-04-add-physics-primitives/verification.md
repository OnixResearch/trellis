# Verification — add-physics-primitives

Implemented exact finite physics primitive checkers and synced requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.

## Scope

The slice is exact/discrete only: dimensional compatibility, bounded vectors, positive time-horizon admission, one-step bounded state admission, collision/contact envelopes, and energy/momentum drift predicates. It explicitly does not claim floating-point physics accuracy, continuous dynamics, solver completeness, broad collision detection, conservation-law completeness, runtime engine behavior, stochastic simulation, or production physics correctness.

## Commands

### Focused checks

```bash
cargo fmt
cargo test --quiet physics_
./scripts/verus.sh src/lib.rs 2>&1 | tee /tmp/vl-verus-physics-final.log
tracey query uncovered --prefix domain-primitives.physics
tracey query untested --prefix domain-primitives.physics
openspec validate add-physics-primitives --strict --json
openspec validate domain-primitives --strict --json
```

Results:
- `cargo test --quiet physics_`: `8 passed; 0 failed`.
- `./scripts/verus.sh src/lib.rs`: `8826 verified, 0 errors`.
- `tracey query uncovered --prefix domain-primitives.physics`: `0 uncovered out of 1787 rules`.
- `tracey query untested --prefix domain-primitives.physics`: `0 untested out of 1787 rules`.
- `openspec validate add-physics-primitives --strict --json`: passed.
- `openspec validate domain-primitives --strict --json`: passed.

### Full gate

```bash
git diff --check
./scripts/verify.sh 2>&1 | tee /tmp/vl-verify-physics-clean.log
if grep -q '^error:' /tmp/vl-verify-physics-clean.log; then echo 'raw verify log contains error diagnostics' >&2; exit 1; fi
openspec validate add-physics-primitives --strict --json
openspec validate domain-primitives --strict --json
```

Results:
- `git diff --check`: clean.
- `./scripts/verify.sh`: `=== ALL CHECKS PASSED ===` with no raw `error:` diagnostics in `/tmp/vl-verify-physics-clean.log`.
- OpenSpec strict validation for the active change and canonical domain spec passed.

## Notes

- `scripts/verus.sh` was refreshed to point at the current Verus/Rust toolchain store paths and allow environment overrides via `VERUS_ROOT`/`RUST_TOOLCHAIN`; the previous hard-coded Verus store path had been garbage-collected and prevented fresh reruns.
- A first full verify transcript `/tmp/vl-verify-physics.log` included raw Clippy cast diagnostics from `physics_momentum_budget`; those were fixed with explicit lint scoping and the clean rerun was captured in `/tmp/vl-verify-physics-clean.log`.
