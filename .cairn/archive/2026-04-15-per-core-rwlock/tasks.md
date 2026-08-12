## Phase 1: Aggregation Foundation

- [x] Define `sum_readers` spec functions over `Seq<u32>`. ✅
- [x] Prove empty and single-core aggregation identities. ✅
- [x] Prove a single-core increment raises the mathematical sum by 1. ✅
- [x] Write exec-level `aggregate_reader_count` with `u64` saturation. ✅

## Phase 2: Per-Core Read Operations

- [x] Define `read_acquire_core` and `read_release_core` specs that update exactly one core. ✅
- [x] Prove per-core independence: for `j != i`, other core counts stay unchanged. ✅
- [x] Write exec-level acquire/release helpers over borrowed slices returning updated vectors. ✅

## Phase 3: Writer Protocol

- [x] Define the read-acquire gate and the drain-complete predicate. ✅
- [x] Prove the write flag blocks new readers. ✅
- [x] Prove one active reader release decreases the mathematical aggregate by exactly 1. ✅
- [x] Write exec-level writer flag and drain-check helpers. ✅

## Phase 4: Mutual Exclusion and Mode Equivalence

- [x] Define `per_core_mode` over `(write_held, counts)`. ✅
- [x] Prove aggregated mutual exclusion matches the global rwlock surface. ✅
- [x] Prove mode equivalence with global `check_mutual_exclusion_spec`. ✅
- [x] Prove writer release returns the drained lock to Free mode. ✅

## Phase 5: Tests and Validation

- [x] Add unit tests for aggregation, per-core acquire/release, writer protocol, and mode computation. ✅
- [x] Add proptests for independence, saturated aggregation, and mode equivalence with global rwlock. ✅
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean. ✅
