## Phase 1: Policy model

- [x] [serial] Define the Nickel verification policy schema with contracts for checker names, inputs, generated artifact path, and bounded exemptions. ✅ 12m (started: 2026-05-19T23:50:00Z → completed: 2026-05-20T00:02:00Z; evidence: `verification-policy/policy.ncl`, `openspec/changes/roi-02-nickel-verification-policy/evidence/verification-policy.txt`)
- [x] [serial] Add an initial policy instance that represents the current Rust checker gates without changing behavior. ✅ 8m (started: 2026-05-19T23:54:00Z → completed: 2026-05-20T00:02:00Z; evidence: `verification-policy/generated/policy.json`, `openspec/changes/roi-02-nickel-verification-policy/evidence/checker-gates.txt`)

## Phase 2: Export and integration

- [x] [depends:policy-model] Add deterministic JSON export and a freshness check. ✅ 8m (started: 2026-05-20T00:00:00Z → completed: 2026-05-20T00:08:00Z; evidence: `scripts/check-verification-policy.rs`, `openspec/changes/roi-02-nickel-verification-policy/evidence/verification-policy.txt`)
- [x] [depends:policy-model] Integrate the policy check into `flake.nix` while keeping Rust scanner logic authoritative for parsing. ✅ 5m (started: 2026-05-20T00:03:00Z → completed: 2026-05-20T00:08:00Z; evidence: `openspec/changes/roi-02-nickel-verification-policy/evidence/nix-verification-policy.txt`)

## Phase 3: Validation

- [x] [depends:export] Run Nickel typecheck/export/freshness checks and record output. ✅ 1m (started: 2026-05-20T00:08:00Z → completed: 2026-05-20T00:09:00Z; evidence: `openspec/changes/roi-02-nickel-verification-policy/evidence/verification-policy.txt`, `openspec/changes/roi-02-nickel-verification-policy/evidence/nix-verification-policy.txt`)
- [x] [depends:integration] Run checker gates and `./scripts/verify.rs`; record evidence before archive. ✅ 4m (started: 2026-05-20T00:09:00Z → completed: 2026-05-20T00:13:00Z; evidence: `openspec/changes/roi-02-nickel-verification-policy/evidence/checker-gates.txt`, `openspec/changes/roi-02-nickel-verification-policy/evidence/verify.txt`)
