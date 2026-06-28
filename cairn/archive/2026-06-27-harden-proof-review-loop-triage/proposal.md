## Why

Recent proof-review iterations found loop gaps that make reviews noisier than necessary: active-change scanner evidence can reintroduce stale source paths, advisory packets can invite infeasible tests, validation instructions rely on prose for Tracey transient retries, and generated packages do not explicitly require target-cleared or frozen archive evidence.

## What Changes

- Ignore active-change `evidence/` markdown when extracting source references for scanner ranking.
- Strengthen VibeThinker packet output contracts toward feasible public-runtime test suggestions and explicit accepted/rejected advisory triage.
- Add a narrow Tracey coverage wrapper that retries only the documented transient `Error: Cancelled` daemon race.
- Generate future proof-review Cairn packages with target-cleared scanner tasks and immutable archive-evidence wording.

## Impact

- **Files**: `scripts/proof-review-candidates.rs`, `scripts/proof-review-loop.rs`, `scripts/proof-review-tracey-coverage.rs`, `proof-review/policy.ncl`, `proof-review/generated/policy.json`, and the proof-review automation spec.
- **Testing**: scanner self-test, loop self-test, Tracey wrapper self-test, policy freshness, Cairn gates, and repo verification.
