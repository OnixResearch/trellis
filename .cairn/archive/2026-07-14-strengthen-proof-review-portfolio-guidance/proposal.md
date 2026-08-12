## Why

The proof-review scanner already asks VibeThinker for concrete postconditions, executable tests, specification gaps, and patch risks. It does not yet distinguish materially different proof mechanisms from paraphrases, identify helper lemmas that are equivalent or stronger than the target, or require a systematic adversarial audit of proof-soundness boundaries. A reviewer can therefore return elegant reduction theater or correlated suggestions that look like progress without reducing the actual proof obligation.

## What Changes

- Strengthen generated proof-review packets so they do not assume the selected target or patch is correct. r[proof-review.specialized-vibethinker-prompts]
- Require proposed proof routes to be grouped by mechanism rather than wording. r[proof-review.specialized-vibethinker-prompts.portfolio]
- Require each proposed helper lemma or reduction to be classified as simpler, equivalent, stronger, or unknown relative to the target; equivalent or stronger gaps remain blocked. r[proof-review.specialized-vibethinker-prompts.gap-strength]
- Require concrete counterexamples or failed proof obligations for rejected sublemmas when feasible. r[proof-review.specialized-vibethinker-prompts.adversarial]
- Add an explicit audit for satisfiable preconditions, vacuity, quantifier domains, arithmetic and index bounds, decreases, exec/spec correspondence, and trust-boundary expansion. r[proof-review.specialized-vibethinker-prompts.adversarial]
- Extend deterministic self-test assertions for the new positive and negative prompt contract. r[proof-review.specialized-vibethinker-prompts.verification]

## Impact

- **Files**: `scripts/proof-review-candidates.rs`, accepted proof-review automation specification, and this Cairn change package.
- **Behavior**: generated Markdown packets become more discriminating; VibeThinker remains advisory and gains no repository or validation authority.
- **Non-claims**: mechanism labels and gap-strength classifications are requested review output, not mechanically proved semantic equivalence. Cargo, Verus, Tracey, Cairn, and repository checkers remain authoritative.
