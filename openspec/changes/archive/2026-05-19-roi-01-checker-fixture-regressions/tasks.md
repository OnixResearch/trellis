## Phase 1: Fixture contract

- [x] [serial] Add valid and invalid fixtures for coverage, ensures, and iterative-exec checker behavior. ✅ 25m (started: 2026-05-19T23:20:00Z → completed: 2026-05-19T23:45:00Z; evidence: `tests/fixtures/checkers/**`, `openspec/changes/roi-01-checker-fixture-regressions/evidence/checker-fixtures.txt`)
- [x] [serial] Add a deterministic harness or self-test mode that asserts expected exit statuses and stable diagnostic substrings. ✅ 25m (started: 2026-05-19T23:20:00Z → completed: 2026-05-19T23:45:00Z; evidence: `scripts/checker-fixtures.rs`, `openspec/changes/roi-01-checker-fixture-regressions/evidence/checker-fixtures.txt`)

## Phase 2: Integration

- [x] [depends:fixture-contract] Wire checker fixture self-tests into `flake.nix` without replacing live-tree checker gates. ✅ 8m (started: 2026-05-19T23:33:00Z → completed: 2026-05-19T23:41:00Z; evidence: `openspec/changes/roi-01-checker-fixture-regressions/evidence/nix-checker-fixtures.txt`)
- [x] [parallel] Update README or developer docs if the new self-test command is user-facing. ✅ 2m (started: 2026-05-19T23:39:00Z → completed: 2026-05-19T23:41:00Z; evidence: `README.md` Verification section)

## Phase 3: Validation

- [x] [depends:integration] Run direct checker self-tests and record the command output. ✅ 1m (started: 2026-05-19T23:41:00Z → completed: 2026-05-19T23:42:00Z; evidence: `openspec/changes/roi-01-checker-fixture-regressions/evidence/checker-fixtures.txt`)
- [x] [depends:integration] Run the relevant Nix check(s) and `./scripts/verify.rs`; record evidence before archive. ✅ 6m (started: 2026-05-19T23:42:00Z → completed: 2026-05-19T23:47:34Z; evidence: `openspec/changes/roi-01-checker-fixture-regressions/evidence/nix-checker-fixtures.txt`, `openspec/changes/roi-01-checker-fixture-regressions/evidence/verify.txt`)
