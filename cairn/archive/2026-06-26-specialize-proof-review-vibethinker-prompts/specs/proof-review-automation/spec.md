## ADDED Requirements

### Requirement: Specialized VibeThinker Prompt Packets

The proof-review scanner MUST generate deterministic VibeThinker packet guidance that includes a base advisory task, a strict output contract, and rule-specific focus bullets derived from the top scanner findings. These prompts MUST remain advisory-only and MUST NOT replace cargo, Verus, Tracey, Cairn, or repo checker validation.

r[proof-review.specialized-vibethinker-prompts]

#### Scenario: Rule-specific focus is emitted

- GIVEN a proof-review candidate with vacuous-proof or empty-proof findings
- WHEN the scanner writes a VibeThinker packet
- THEN the packet MUST include deterministic rule-specific guidance for those findings and an output contract that asks for concrete proof, spec, or executable-test gaps
