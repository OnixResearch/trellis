# Recommendation Selection Change Spec

## Purpose

Define deterministic ranking, deduplication, source-capping, and top-k
selection primitives for recommendation candidates.

## Requirements

Traceability rule: every named public helper introduced below MUST carry
direct helper-level `r[impl ...]` coverage on the defining helper plus matching
`r[verify ...]` coverage on proofs/tests; requirement-level tags alone are not
sufficient for this change.

Repo-sync rule: every `recommendation.*` requirement/scenario family introduced
here MUST gain matching `r[...]` anchors in `docs/requirements.md`; OpenSpec
files alone are not sufficient for Tracey coverage in this repo.

Repo-doc rule: `README.md` MUST gain matching recommendation-module summaries
when this change lands so the documented public surface matches the code and
requirements.

Crate-public rule: `src/lib.rs` MUST export the `recommendation_selection`
module so `RecommendationCandidate`, `rank_candidates`,
`dedup_by_content_key`, `apply_source_cap`, and `top_k_recommendations` are
public crate-surface items.

Preflight audit rule: before implementation or sync-only closure continues,
this change MUST record a pinned pre-change baseline audit against `src/lib.rs`,
`README.md`, `docs/requirements.md`, and `openspec/specs/`, then either
continue as net-new work or narrow to metadata-sync-plus-archive scope if the
requested recommendation surface is already shipped.

Validation-evidence rule: when this change lands, it MUST capture fresh output
for `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`,
`./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`,
`tracey query uncovered`, `tracey query untested`, and `./scripts/verify.sh`.

### Requirement: Deterministic Candidate Ranking

ID: recommendation.ranking
r[recommendation.ranking]
`src/recommendation_selection.rs` MUST expose public record
`RecommendationCandidate { candidate_id: u64, content_key: u64, source_id:
 u64, score: u128 }` and helper
`rank_candidates(candidates: Vec<RecommendationCandidate>) -> Vec<RecommendationCandidate>`.
Ranking consumes inputs with pairwise distinct `candidate_id` values and
returns a permutation ordered by descending `score`, breaking equal-score ties
by lower `candidate_id`.

#### Scenario: Higher score is emitted earlier

ID: recommendation.ranking.higher-score-first
- GIVEN two candidates with distinct `candidate_id` values and different `score` values
- WHEN both appear in `rank_candidates(candidates)`
- THEN the candidate with the higher `score` appears earlier

#### Scenario: Equal scores break to lower candidate ID

ID: recommendation.ranking.equal-score-tiebreak
- GIVEN two candidates with equal `score` and distinct `candidate_id` values
- WHEN both appear in `rank_candidates(candidates)`
- THEN the candidate with the lower `candidate_id` appears earlier

#### Scenario: Ranking preserves exactly the input candidates

ID: recommendation.ranking.permutation
- GIVEN candidates with pairwise distinct `candidate_id` values
- WHEN `rank_candidates(candidates)` is computed
- THEN every input candidate appears exactly once in the output
- AND no new candidate is introduced

#### Scenario: Duplicate candidate IDs are rejected

ID: recommendation.ranking.reject-duplicate-id
- GIVEN two distinct input positions with the same `candidate_id`
- WHEN `rank_candidates(candidates)` is invoked
- THEN the call is rejected by an explicit public-helper precondition

### Requirement: Content-Key Deduplication

ID: recommendation.dedup
r[recommendation.dedup]
`dedup_by_content_key(candidates: Vec<RecommendationCandidate>) -> Vec<RecommendationCandidate>`
MUST scan left-to-right, keep the first occurrence of each `content_key`, drop
later candidates with the same `content_key`, and preserve the relative order
of the kept candidates.

#### Scenario: First occurrence of a content key wins

ID: recommendation.dedup.first-occurrence-wins
- GIVEN two candidates with the same `content_key`
- WHEN `dedup_by_content_key(candidates)` is computed
- THEN only the first occurrence is kept
- AND later occurrences of that `content_key` are dropped

#### Scenario: Unique content keys are preserved

ID: recommendation.dedup.unique-keys-preserved
- GIVEN candidates whose `content_key` values are pairwise distinct
- WHEN `dedup_by_content_key(candidates)` is computed
- THEN the output equals the input

#### Scenario: Relative order of survivors is preserved

ID: recommendation.dedup.order-preserved
- GIVEN any candidate list
- WHEN `dedup_by_content_key(candidates)` is computed
- THEN the surviving candidates appear in the same relative order as in the input

### Requirement: Per-Source Cap

ID: recommendation.source-cap
r[recommendation.source-cap]
`apply_source_cap(candidates: Vec<RecommendationCandidate>, max_per_source:
 usize) -> Vec<RecommendationCandidate>` MUST scan left-to-right, emit a
candidate exactly when fewer than `max_per_source` earlier emitted candidates
share its `source_id`, preserve survivor order, and emit an empty list when
`max_per_source == 0`.

#### Scenario: No source exceeds the cap

ID: recommendation.source-cap.enforced
- GIVEN any candidate list and cap `c`
- WHEN `apply_source_cap(candidates, c)` is computed
- THEN no `source_id` appears more than `c` times in the output

#### Scenario: Zero cap emits no candidates

ID: recommendation.source-cap.zero-cap-empty
- GIVEN any candidate list
- WHEN `apply_source_cap(candidates, 0)` is computed
- THEN the output is empty

#### Scenario: Survivor order is preserved under capping

ID: recommendation.source-cap.order-preserved
- GIVEN any candidate list and cap `c`
- WHEN `apply_source_cap(candidates, c)` is computed
- THEN the surviving candidates appear in the same relative order as in the input

### Requirement: Top-K Recommendation Pipeline

ID: recommendation.topk-pipeline
r[recommendation.topk-pipeline]
`top_k_recommendations(candidates: Vec<RecommendationCandidate>, limit:
 usize, max_per_source: usize) -> Vec<RecommendationCandidate>` MUST consume
inputs with pairwise distinct `candidate_id` values and equal exactly the first
`limit` entries of
`apply_source_cap(dedup_by_content_key(rank_candidates(candidates)), max_per_source)`.
Deduplication happens before source-cap counting, and truncation happens last.

#### Scenario: Result length is bounded by the limit

ID: recommendation.topk-pipeline.limit-bounds-length
- GIVEN any admitted candidate list and limit `k`
- WHEN `top_k_recommendations(candidates, k, max_per_source)` is computed
- THEN the output length is at most `k`

#### Scenario: Zero limit emits an empty selection

ID: recommendation.topk-pipeline.zero-limit-empty
- GIVEN any admitted candidate list
- WHEN `top_k_recommendations(candidates, 0, max_per_source)` is computed
- THEN the output is empty

#### Scenario: Deduplication happens before capping and truncation

ID: recommendation.topk-pipeline.dedup-before-cap
- GIVEN admitted candidates with repeated `content_key` values and repeated `source_id` values
- WHEN `top_k_recommendations(candidates, limit, max_per_source)` is computed
- THEN the surviving representative for each `content_key` is chosen before source counts are computed
- AND source counts are computed on the deduplicated stream
- AND truncation to `limit` happens after ranking, deduplication, and source capping

#### Scenario: Duplicate candidate IDs are rejected by the pipeline

ID: recommendation.topk-pipeline.reject-duplicate-id
- GIVEN two distinct input positions with the same `candidate_id`
- WHEN `top_k_recommendations(candidates, limit, max_per_source)` is invoked
- THEN the call is rejected by an explicit public-helper precondition
