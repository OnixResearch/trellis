## Why

The repo already has verified scoring and ordering helpers such as
`eviction_score`, `weighted_round_robin`, `markov`, and
`chaoscontrol-heuristics`, but it still lacks a recommendation-specific core
for deterministic candidate scoring, ranking, deduplication, and per-source
capping. Downstream feed builders would otherwise re-derive those rules in ad
hoc ways, which risks drift in tie-break behavior, duplicate suppression, and
quota handling.

## Repo Audit

This repo already contains adjacent generic scoring and ordering surfaces such
as `eviction_score`, `weighted_round_robin`, `markov`, and ChaosControl
heuristics. The pinned pre-change baseline for this change is git commit
`c06bedc598e8` (jj change `ltklnzopqvqz`). Against that baseline,
recommendation-specific matches were absent from `src/lib.rs`, `README.md`,
`docs/requirements.md`, and `openspec/specs/`; only adjacent generic scoring
modules were present. The audit phase records that baseline comparison
formally, and if a later rerun finds the requested recommendation surface
already shipped, this change must narrow to metadata sync plus archive instead
of continuing as net-new implementation work.

## What Changes

- **recommendation-scoring**: Add public constants
  `RECOMMENDATION_WEIGHT_SCALE = 10_000` and
  `RECOMMENDATION_MAX_FEATURES = 1024` plus exact integer weighted scoring over
  `signals: Vec<u32>` and `weights: Vec<u16>` via `weights_valid` and
  `weighted_score`, with widened `u128` accumulation for both weight summation
  and score computation inside the explicit max-feature bound.
- **recommendation-selection**: Add `RecommendationCandidate` plus exact
  `rank_candidates`, `dedup_by_content_key`, `apply_source_cap`, and
  `top_k_recommendations` helpers, with pairwise-distinct `candidate_id`
  values as the admitted-input boundary for ranking and top-k selection.
- **deterministic pipeline model**: Pin one exact selection order for the
  public feed primitive: rank by descending score, break ties by lower
  `candidate_id`, deduplicate by `content_key`, apply a left-to-right
  `source_id` cap, then truncate to `limit`.

## Non-Goals

- Embedding models, ANN search, or approximate nearest-neighbor indexes
- Online learning, bandits, or exploration/exploitation policies
- Floating-point ranking semantics or ML training pipelines
- Application-specific policy fields beyond score, content key, source_id,
  and limit/cap controls

## Capabilities

### New Capabilities

- `recommendation.scoring`
- `recommendation.ranking`
- `recommendation.dedup`
- `recommendation.source-cap`
- `recommendation.topk-pipeline`

## Impact

- **Files**: New `src/recommendation_scoring.rs` and
  `src/recommendation_selection.rs`; update `src/lib.rs`, `README.md`,
  `docs/requirements.md`, and later sync canonical specs during the final
  repo-sync phase at
  `openspec/specs/recommendation-scoring/spec.md` and
  `openspec/specs/recommendation-selection/spec.md`.
- **APIs**: Introduce public named constants `RECOMMENDATION_WEIGHT_SCALE` and
  `RECOMMENDATION_MAX_FEATURES`, public helpers `weights_valid`,
  `weighted_score`, `rank_candidates`,
  `dedup_by_content_key`, `apply_source_cap`, `top_k_recommendations`, and
  public record `RecommendationCandidate { candidate_id, content_key,
  source_id, score }`.
- **Dependencies**: No new crates.
- **Testing**: Verus proofs, unit tests, and property tests with direct
  helper-level `r[impl ...]` / `r[verify ...]` coverage on every named public
  helper and matching `recommendation.*` anchors in `docs/requirements.md`,
  plus fresh captured runs of `cargo test --quiet`,
  `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`,
  `./scripts/check-ensures.sh`, `tracey query uncovered`,
  `tracey query untested`, and `./scripts/verify.sh`.
