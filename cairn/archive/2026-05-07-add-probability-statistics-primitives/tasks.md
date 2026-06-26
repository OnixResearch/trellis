## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.stats-count-table` | `stats_count_table` |
| `domain-primitives.stats-frequency-table` | `stats_frequency_table` |
| `domain-primitives.stats-discrete-distribution` | `stats_discrete_distribution` |
| `domain-primitives.stats-cdf-quantile` | `stats_cdf_quantile` |
| `domain-primitives.stats-bounded-moments` | `stats_bounded_moments` |
| `domain-primitives.stats-deterministic-sampler` | `stats_deterministic_sampler` |
| `domain-primitives.stats-confidence-interval-envelope` | `stats_confidence_interval_envelope` |

## Phase 1: Spec foundation

- [x] Create focused probability/statistics primitive OpenSpec with finite exact models, validity-boundary, and evidence expectations.

## Phase 2: Count, frequency, and distribution primitives

- [x] Implement and prove `stats_count_table`; cover category admission, bounded count updates, single-bucket mutation, and overflow rejection. `[covers=domain-primitives.stats-count-table]`
- [x] Implement and prove `stats_frequency_table`; cover denominator-safe count-to-frequency derivation, total-count bounds, and zero-total rejection/empty result. `[covers=domain-primitives.stats-frequency-table]`
- [x] Implement and prove `stats_discrete_distribution`; cover non-negative finite weights, total-weight admission, normalization envelope, and invalid-weight rejection. `[covers=domain-primitives.stats-discrete-distribution]`

## Phase 3: CDF, moment, sampling, and interval primitives

- [x] Implement and prove `stats_cdf_quantile`; cover monotone cumulative weights, partition lookup, deterministic tie policy, and invalid-CDF rejection. `[covers=domain-primitives.stats-cdf-quantile]`
- [x] Implement and prove `stats_bounded_moments`; cover count/sum/mean/variance-envelope derivation, checked accumulators, and overflow/zero-count rejection. `[covers=domain-primitives.stats-bounded-moments]`
- [x] Implement and prove `stats_deterministic_sampler`; cover admitted threshold/index mapping to exactly one bucket and out-of-range threshold rejection. `[covers=domain-primitives.stats-deterministic-sampler]`
- [x] Implement and prove `stats_confidence_interval_envelope`; cover ordered interval endpoints, confidence metadata bounds, and malformed-interval rejection without coverage-probability claims. `[covers=domain-primitives.stats-confidence-interval-envelope]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
