## Context

This repo already verifies integer scoring, ranking, and scheduling kernels in
modules such as `eviction_score`, `weighted_round_robin`, `markov`, and
`chaoscontrol-heuristics`. None of those modules define the exact public feed
selection pipeline that recommendation systems repeatedly need: integer feature
scoring, deterministic score ordering, content-key deduplication, per-source
caps, and final top-k truncation.

## Goals / Non-Goals

**Goals:**
- Add exact integer recommendation scoring with one named weight scale
- Add deterministic ranking over recommendation candidates
- Add stable content-key deduplication and per-source capping
- Add one exact top-k pipeline that downstream feeds can reuse

**Non-Goals:**
- Learning-to-rank or embedding retrieval
- Floating-point scoring or approximate ranking
- Personalized policy fields beyond score, content key, source_id, and
  limit/cap controls

## Repo Audit

The pinned pre-change baseline for this change is git commit
`c06bedc598e8` (jj change `ltklnzopqvqz`). Against that baseline,
recommendation-specific matches were absent from `src/lib.rs`, `README.md`,
`docs/requirements.md`, and `openspec/specs/`; only adjacent generic modules
such as `eviction_score`, `weighted_round_robin`, `markov`, and ChaosControl
heuristics were present. That scoped baseline result is already part of this
design: recommendation primitives are net-new at `c06bedc598e8`, so
implementation may proceed unless `R1` later contradicts the current audit and
forces the change onto the sync-only archive path.

## Decisions

### 1. Use an exact integer scoring model

**Choice:** Add `src/recommendation_scoring.rs` with public constants
`RECOMMENDATION_WEIGHT_SCALE = 10_000` and
`RECOMMENDATION_MAX_FEATURES = 1024`, executable validator
`weights_valid(weights: Vec<u16>) -> bool`, and scorer
`weighted_score(signals: Vec<u32>, weights: Vec<u16>) -> u128`. Valid weights
are non-empty, have length `<= RECOMMENDATION_MAX_FEATURES`, and sum exactly to
`RECOMMENDATION_WEIGHT_SCALE`; the scorer requires equal vector lengths and
valid weights and returns the exact integer weighted dot product. Bounds are
therefore the declared scalar types plus the named feature-count limit:
`signals` elements are `u32`, `weights` elements are `u16`, and
`signals.len() == weights.len() <= RECOMMENDATION_MAX_FEATURES`. Exec
arithmetic widens every weight to `u128` inside `weights_valid` and widens
every `signals[i]` and `weights[i]` to `u128` before multiply/add inside
`weighted_score`, so the runtime surface matches the exact mathematical sum
and dot product on the admitted bounded domain.

**Rationale:** Recommendation ranking must be deterministic and verifier-
friendly. Integer arithmetic avoids floating-point proof drift while still
supporting a normalized policy scale, and the named max-feature bound makes
the exact widened `u128` accumulation executable and reviewable.

### 2. Make candidate ranking fully deterministic

**Choice:** Add `src/recommendation_selection.rs` with public record
`RecommendationCandidate { candidate_id: u64, content_key: u64, source_id:
 u64, score: u128 }` and public helper
`rank_candidates(candidates: Vec<RecommendationCandidate>) -> Vec<RecommendationCandidate>`.
Ranking requires pairwise distinct `candidate_id` values and orders candidates
by descending `score`, breaking ties by lower `candidate_id`.

**Rationale:** Recommendation outputs must be reproducible across runs and
reviewers. Explicit tie-break rules prevent hidden instability.

### 3. Keep deduplication and source caps as stable left-to-right filters

**Choice:** `dedup_by_content_key` scans a candidate list left-to-right and
keeps the first occurrence of each `content_key`. `apply_source_cap` scans
left-to-right and emits a candidate exactly when fewer than `max_per_source`
previously emitted candidates share its `source_id`. Both helpers preserve the
relative order of emitted candidates; `max_per_source == 0` is total and emits
an empty list.

**Rationale:** Stable filters make the proof surface simple and match common
feed semantics: once a ranked winner is chosen for a content key, later
variants lose; once a source fills its cap, later items from that source are
suppressed.

### 4. Pin one public top-k pipeline

**Choice:** `top_k_recommendations(candidates, limit, max_per_source)` is
exactly `take(limit, apply_source_cap(dedup_by_content_key(rank_candidates(candidates)), max_per_source))`.
Deduplication happens before source-cap counting, and truncation happens last.
The helper requires pairwise distinct `candidate_id` values and is otherwise
total; `limit == 0` returns an empty list.

**Rationale:** Recommendation stacks often disagree on whether dedup happens
before or after quota handling. Pinning one exact order prevents downstream
semantic drift.

### 5. Direct helper-level traceability is mandatory

**Choice:** Every named public surface introduced by this change will carry a
direct helper-level `r[impl ...]` tag on the defining item and matching
`r[verify ...]` tags on the proofs/tests that establish its behavior.
Specifically: `RECOMMENDATION_WEIGHT_SCALE`,
`RECOMMENDATION_MAX_FEATURES`, `weights_valid`, `weighted_score`,
`RecommendationCandidate`, `rank_candidates`, `dedup_by_content_key`,
`apply_source_cap`, and `top_k_recommendations` each get a defining
`r[impl ...]` anchor rather than relying only on requirement-level
references.

**Rationale:** This repo's Tracey and OpenSpec gates are literal about helper-
level traceability, especially when a requirement is realized by several small
public surfaces instead of one wrapper.

### 6. Rejection evidence is mostly proof-only

**Choice:** Invalid weight vectors have an executable validator surface through
`weights_valid`. Other invalid states — mismatched scoring vector lengths and
non-distinct `candidate_id` inputs — stay as explicit public-helper
preconditions proved at the contract layer instead of wrapped in separate
runtime `Result` types.

**Rationale:** This repo prefers exact verified contracts over duplicated
validator-return wrappers when the public API is intended for admitted inputs.

## Repo Surface Synchronization

- `src/lib.rs` exports `recommendation_scoring` and `recommendation_selection`.
- `README.md` gains matching module summaries for the new recommendation
  primitives.
- `docs/requirements.md` gains matching `recommendation.*` anchors for every
  requirement and scenario family introduced by this change, because Tracey in
  this repo reads `docs/requirements.md` rather than OpenSpec files.
- `openspec/specs/recommendation-scoring/spec.md` and
  `openspec/specs/recommendation-selection/spec.md` are synced during the `R3`
  final repo-sync phase; they are future canonical sync targets, not part of
  the initial active-change scaffold.

## Verification Strategy

- `recommendation.scoring`: prove exact weight validation in widened `u128`
  accumulation, exact widened `u128` dot-product scoring on the admitted
  `RECOMMENDATION_MAX_FEATURES` domain, monotonicity when a positively
  weighted signal increases on that admitted domain, rejection of
  `signals.len() != weights.len()` at the public-helper contract boundary, and
  rejection of empty, wrong-sum, too-long-weight, or overlong-signal inputs;
  cover admitted and rejected boundaries with unit/property tests plus
  `weights_valid` negative tests.
- `recommendation.ranking`: prove permutation, descending-score ordering, and
  lower-`candidate_id` tie breaks; cover distinct-ID rejection at the contract
  boundary.
- `recommendation.dedup`: prove first-occurrence retention by `content_key`
  and preserved relative order of survivors.
- `recommendation.source-cap`: prove the exact left-to-right survivor rule
  “emit exactly when fewer than `max_per_source` earlier emitted candidates
  share the same `source_id`”, plus per-source emitted-count bounds,
  survivor-order preservation, and zero-cap emptiness.
- `recommendation.topk-pipeline`: prove exact composition of ranking, dedup,
  source-cap filtering, and final prefix truncation, and cover duplicate-ID
  rejection at the public-helper contract boundary.
- Repo validation after implementation: `cargo test --quiet`,
  `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`,
  `./scripts/check-ensures.sh`, `tracey query uncovered`,
  `tracey query untested`, and `./scripts/verify.sh`.

## Risks / Trade-offs

**[Recommendation policies are often richer than this core]** → Mitigate by
keeping this change focused on reusable deterministic primitives, not every
product-specific rule.

**[Verified ranking may favor simple algorithms over optimal asymptotics]** →
Mitigate by specifying correctness first; optimized implementations can still
refine the same contracts later.
