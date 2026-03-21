## Phase 1: Strip and restructure

- [x] Clean Cargo.toml — remove binary-crate deps ✅ 5m
- [x] Remove src/error.rs placeholder ✅ 1m
- [x] Clean src/lib.rs — module declarations for 4 domains ✅ 1m
- [x] Create stub modules: arithmetic.rs, bounds.rs, collections.rs, option.rs ✅ 1m
- [x] Verify cargo check passes ✅ 3m

## Phase 2: Tooling infrastructure

- [x] Create `.config/tracey/config.styx` ✅ 1m
- [x] Create `docs/requirements.md` with 17 requirements across 5 domains ✅ 2m
- [x] Create `scripts/verify.sh` — 7-gate pipeline ✅ 1m
- [x] Create `scripts/verus.sh` — verus wrapper with nix store paths ✅ 1m
- [x] Create `scripts/check-coverage.sh` — enforce r[impl] on every pub fn ✅ 5m
- [x] Create `scripts/check-ensures.sh` — enforce ensures on every verified pub fn ✅ 5m

## Phase 3: Verification pipeline

- [x] Implement all 17 requirements across 5 modules ✅ 20m
- [x] Embed verus! blocks in src/ — no separate verus/ directory ✅ 15m
- [x] Hand-implement binary_search and sorted_insert with loop invariants ✅ 15m
- [x] 7 cross-module proofs (triangle inequality, roundtrips, monotonicity) ✅ 10m
- [x] Property-based conformance tests for all modules ✅ 10m
- [x] 35 verus proofs, 32 cargo tests, 17/17 tracey coverage ✅

## Phase 4: Documentation and CI

- [x] Rewrite README.md ✅ 1m
- [x] Write agent.md — codify the verified pattern ✅ 10m
- [x] Configure clippy lint policy in Cargo.toml ✅ 5m
- [x] Wire verus + tracey + coverage into nix flake check (8 checks) ✅ 20m
- [x] All commits pushed to GitHub ✅
