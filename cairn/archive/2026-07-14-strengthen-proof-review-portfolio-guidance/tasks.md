## Phase 1: Deterministic prompt contract

- [x] [serial] Strengthen the base advisory task so target and patch correctness are not assumed. r[proof-review.specialized-vibethinker-prompts]
- [x] [serial] Add mechanism-family grouping and simpler/equivalent/stronger/unknown gap-strength output requirements. r[proof-review.specialized-vibethinker-prompts.portfolio] r[proof-review.specialized-vibethinker-prompts.gap-strength]
- [x] [serial] Add concrete counterexample or failed-obligation guidance plus the Trellis adversarial proof-boundary checklist. r[proof-review.specialized-vibethinker-prompts.adversarial]

## Phase 2: Positive and negative regression evidence

- [x] [parallel] Extend positive self-test assertions for mechanism grouping, gap-strength classification, and adversarial audit guidance. r[proof-review.specialized-vibethinker-prompts.verification]
- [x] [parallel] Extend negative self-test assertions so equivalent or stronger gaps remain blocked and the prompt does not assume target correctness. r[proof-review.specialized-vibethinker-prompts.verification]

## Phase 3: Validation and lifecycle closeout

- [x] [serial] Run the proof-review candidate self-test before and after the source change and record the baseline/final outcomes. r[proof-review.specialized-vibethinker-prompts.verification]
  - Evidence: both runs pass; see `verification.md`.
- [x] [serial] Run canonical Cairn validation and proposal, design, and tasks gates for this change. r[proof-review.specialized-vibethinker-prompts.verification]
  - Evidence: the canonical command was run and exposed pre-existing generated-policy drift; validation and all gates pass with current Cairn's explicit policy. The distinction is recorded in `verification.md`.
- [x] [serial] Sync the modified accepted requirement, dry-run archive, verify the accepted text, and archive only after all focused checks pass. r[proof-review.specialized-vibethinker-prompts.verification]
  - Evidence: the Trellis Nickel policy contract and generated policy now include the current optional function-address gate; policy freshness, canonical validation, all three gates, and the focused self-test pass. The accepted requirement is present in `cairn/specs/proof-review-automation/spec.md`.
