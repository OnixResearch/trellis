## Phase 1: Baseline and implementation

- [x] [serial] Run the current proof-review script self-test and focused scanner baseline. r[proof-review.specialized-vibethinker-prompts]
- [x] [serial] Add deterministic rule-specific VibeThinker prompt guidance to `scripts/proof-review-candidates.rs`. r[proof-review.specialized-vibethinker-prompts]
- [x] [serial] Extend self-test coverage for specialized packet prompts. r[proof-review.specialized-vibethinker-prompts]

## Phase 2: Validation and archive

- [x] [serial] Run script self-test, focused scanner with policy check, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[proof-review.specialized-vibethinker-prompts]
- [x] [serial] Sync/archive the change and commit the completed evidence. r[proof-review.specialized-vibethinker-prompts]
