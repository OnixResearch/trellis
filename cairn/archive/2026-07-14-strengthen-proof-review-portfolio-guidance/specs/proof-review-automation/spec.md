# Proof Review Automation Specification Delta

## MODIFIED Requirements

### Requirement: Specialized VibeThinker Prompt Packets

The proof-review scanner MUST generate deterministic VibeThinker packet guidance that includes a base advisory task, a strict output contract, and rule-specific focus bullets derived from the top scanner findings. These prompts MUST remain advisory-only and MUST NOT replace cargo, Verus, Tracey, Cairn, or repo checker validation. Packet guidance MUST ask for feasible executable tests through public/runtime APIs, MUST reject suggestions that depend on violating proof `requires` clauses or infeasible stress cases, MUST separate accepted actionable items from rejected non-actionable advisory items, MUST group materially different proof proposals by mechanism, and MUST classify proposed helper lemmas or reductions as simpler, equivalent, stronger, or unknown relative to the selected target. Equivalent or stronger missing steps MUST be reported as blocked rather than proof progress. The packet MUST NOT assume that the selected target or current patch is correct.

r[proof-review.specialized-vibethinker-prompts]

#### Scenario: Rule-specific focus is emitted

- GIVEN a proof-review candidate with vacuous-proof or empty-proof findings
- WHEN the scanner writes a VibeThinker packet
- THEN the packet MUST include deterministic rule-specific guidance for those findings and an output contract that asks for concrete proof, spec, or feasible executable-test gaps.

#### Scenario: Mechanism families expose duplicate routes

r[proof-review.specialized-vibethinker-prompts.portfolio]
- GIVEN multiple proposed proof routes differ only in wording or local notation
- WHEN the advisory reviewer organizes its response
- THEN it MUST group those routes under one mechanism family
- AND materially different mechanisms MUST remain separate.

#### Scenario: Equivalent-strength reduction stays blocked

r[proof-review.specialized-vibethinker-prompts.gap-strength]
- GIVEN a proposed helper lemma or reduction leaves a missing statement equivalent to or stronger than the selected target
- WHEN gap strength is classified
- THEN the route MUST be reported as blocked rather than near-complete
- AND the response MUST name the smallest discriminating proof or validation step.

#### Scenario: Adversarial proof boundaries are audited

r[proof-review.specialized-vibethinker-prompts.adversarial]
- GIVEN a selected proof target or changed patch
- WHEN focused advisory review runs
- THEN the review MUST examine precondition satisfiability, vacuity, quantifier domains, arithmetic and index bounds, decreases, exec/spec correspondence, and trust-boundary expansion
- AND a rejected sublemma SHOULD include a concrete counterexample or failed proof obligation when feasible.

#### Scenario: Prompt contract is regression tested

r[proof-review.specialized-vibethinker-prompts.verification]
- GIVEN the proof-review script self-test fixtures
- WHEN packet rendering is exercised
- THEN positive assertions MUST require mechanism grouping, gap-strength classification, and adversarial audit text
- AND negative assertions MUST require equivalent or stronger gaps to remain blocked and prohibit assuming target correctness.
