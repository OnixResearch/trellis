# Recommendation Scoring Change Spec

## Purpose

Define deterministic integer scoring primitives for recommendation candidates.

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

Crate-public rule: `src/lib.rs` MUST export the `recommendation_scoring`
module so `RECOMMENDATION_WEIGHT_SCALE`, `RECOMMENDATION_MAX_FEATURES`,
`weights_valid`, and `weighted_score` are public crate-surface items.

Preflight audit rule: before implementation or sync-only closure continues,
this change MUST record a pinned pre-change baseline audit against `src/lib.rs`,
`README.md`, `docs/requirements.md`, and `openspec/specs/`, then either
continue as net-new work or narrow to metadata-sync-plus-archive scope if the
requested recommendation surface is already shipped.

Validation-evidence rule: when this change lands, it MUST capture fresh output
for `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`,
`./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`,
`tracey query uncovered`, `tracey query untested`, and `./scripts/verify.sh`.

### Requirement: Weighted Recommendation Scoring

ID: recommendation.scoring
r[recommendation.scoring]
`src/recommendation_scoring.rs` MUST expose public constants
`RECOMMENDATION_WEIGHT_SCALE = 10_000` and
`RECOMMENDATION_MAX_FEATURES = 1024`, executable validator
`weights_valid(weights: Vec<u16>) -> bool`, and scorer
`weighted_score(signals: Vec<u32>, weights: Vec<u16>) -> u128`. Bounds are the
declared scalar types plus the named feature-count limit: `signals` use `u32`,
`weights` use `u16`, and admitted calls satisfy
`signals.len() == weights.len() <= RECOMMENDATION_MAX_FEATURES`. The scoring
model is exact integer arithmetic on that admitted bounded domain:
`weights_valid` widens each weight to `u128` and is true exactly when
`weights` is non-empty, `weights.len() <= RECOMMENDATION_MAX_FEATURES`, and
the widened exact sum is `RECOMMENDATION_WEIGHT_SCALE`, and `weighted_score`
requires equal vector lengths plus valid weights and widens each term to
`u128` before multiply/add so the result is the exact dot product
`Σ signals[i] * weights[i]`. Higher scores rank ahead of lower scores;
floating-point scoring is out of scope.

#### Scenario: Weights are valid exactly at the named scale

ID: recommendation.scoring.valid-weight-scale
- GIVEN a weight vector `weights`
- WHEN `weights_valid(weights)` is computed
- THEN it returns true if and only if `0 < weights.len() <= RECOMMENDATION_MAX_FEATURES = 1024`
- AND the widened `u128` sum of the weights is exactly `RECOMMENDATION_WEIGHT_SCALE = 10_000`

#### Scenario: Weighted score is the exact dot product

ID: recommendation.scoring.exact-dot-product
- GIVEN signal vector `signals` and weight vector `weights`
- WHEN their lengths are equal and `weights_valid(weights)` holds
- THEN `weighted_score(signals, weights)` equals the exact widened `u128` dot product `Σ signals[i] * weights[i]`

#### Scenario: Increasing a positively weighted signal cannot lower score

ID: recommendation.scoring.monotone-signal
- GIVEN valid `signals`, valid `weights`, and index `i` on the admitted `RECOMMENDATION_MAX_FEATURES` domain
- WHEN `weights[i] > 0` and only `signals[i]` increases
- THEN `weighted_score` does not decrease

#### Scenario: Mismatched scoring vectors are rejected

ID: recommendation.scoring.reject-length-mismatch
- GIVEN `signals.len() != weights.len()`
- WHEN `weighted_score(signals, weights)` is invoked
- THEN the call is rejected by an explicit public-helper precondition

#### Scenario: Overlong signal vectors are rejected

ID: recommendation.scoring.reject-overlong-signals
- GIVEN `signals.len() > RECOMMENDATION_MAX_FEATURES = 1024`
- WHEN `weighted_score(signals, weights)` is invoked
- THEN the call is rejected by an explicit public-helper precondition

#### Scenario: Invalid weights are rejected

ID: recommendation.scoring.reject-invalid-weights
- GIVEN an empty weight vector, a weight vector longer than `RECOMMENDATION_MAX_FEATURES = 1024`, or a weight vector whose exact sum is not `RECOMMENDATION_WEIGHT_SCALE = 10_000`
- WHEN `weighted_score(signals, weights)` is invoked
- THEN the call is rejected by an explicit public-helper precondition
- AND `weights_valid(weights)` returns false
