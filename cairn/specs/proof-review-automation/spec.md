# Proof Review Automation Specification

## Purpose

Defines the `proof-review-automation` capability.

## Requirements

### Requirement: Specialized VibeThinker Prompt Packets

The proof-review scanner MUST generate deterministic VibeThinker packet guidance that includes a base advisory task, a strict output contract, and rule-specific focus bullets derived from the top scanner findings. These prompts MUST remain advisory-only and MUST NOT replace cargo, Verus, Tracey, Cairn, or repo checker validation.

r[proof-review.specialized-vibethinker-prompts]

#### Scenario: Rule-specific focus is emitted

- GIVEN a proof-review candidate with vacuous-proof or empty-proof findings
- WHEN the scanner writes a VibeThinker packet
- THEN the packet MUST include deterministic rule-specific guidance for those findings and an output contract that asks for concrete proof, spec, or executable-test gaps

### Requirement: Bounded Proof Review Agent Loop

The proof-review automation MUST provide a bounded agent loop that deterministically selects proof-review candidates from the scanner, creates or plans one native Cairn change per iteration, preserves VibeThinker as advisory-only input, captures reviewable evidence, and stops unless configured validation succeeds. The loop MUST default to no push, MUST skip permanent trust-boundary findings unless explicitly configured, and MUST fail closed on dirty unrelated files, active Cairn changes, unsupported apply patterns, attempted trust-boundary additions, validation failures, missing evidence, or archive failures.

r[proof-review.agent-loop]

#### Scenario: Dry-run plans one safe candidate

- GIVEN scanner output containing trusted-boundary findings and actionable non-trust proof findings
- WHEN the proof-review loop runs in dry-run mode with a one-iteration budget
- THEN it MUST select at most one non-trust candidate, report the planned Cairn change and validation plan, and MUST NOT mutate source, Cairn changes, archives, commits, or remotes

#### Scenario: Assisted mode creates reviewable lifecycle artifacts

- GIVEN a clean repository with no active Cairn changes and an actionable proof-review candidate
- WHEN the proof-review loop runs in assisted mode
- THEN it MUST create a native Cairn change package with proposal, design, tasks, spec delta, scanner evidence, and VibeThinker packet references, then stop before source edits unless apply mode is explicitly requested

#### Scenario: Apply mode is validation-gated

- GIVEN an actionable candidate matching a deterministic supported proof-hardening recipe
- WHEN the proof-review loop runs in apply mode
- THEN it MUST apply one candidate patch, run the configured focused and repo-wide validation commands, sync/archive only after successful validation, and produce a loop receipt that links candidate, change, evidence, archive, and commit

#### Scenario: Unsafe or ambiguous work stops the loop

- GIVEN unrelated dirty files, an active Cairn change, a trust-boundary-only candidate, an unsupported patch shape, failed validation, or a push request without an explicit push flag
- WHEN the proof-review loop evaluates the next iteration
- THEN it MUST stop without hiding the blocker and MUST report the exact stop reason in its receipt or dry-run output

### Requirement: Bounded Proof Review Iteration for src/arena.rs

The proof-review iteration for `src/arena.rs` MUST strengthen the selected target without adding trust boundaries, MUST include positive and negative executable tests when the target has runtime behavior, and MUST archive scanner, advisory packet, validation, sync, archive, and final status evidence before claiming completion.

r[proof-review.iteration.proof.review.arena]

#### Scenario: Iteration archives only after validation

- GIVEN `src/arena.rs` has been selected by the proof-review loop
- WHEN the change is prepared for archive
- THEN focused checks, Verus, Tracey, Cairn gates, and repo verification evidence MUST be present and successful

### Requirement: Bounded Proof Review Iteration for src/append_entries_build.rs

The proof-review iteration for `src/append_entries_build.rs` MUST strengthen the selected target without adding trust boundaries, MUST include positive and negative executable tests when the target has runtime behavior, and MUST archive scanner, advisory packet, validation, sync, archive, and final status evidence before claiming completion.

r[proof-review.iteration.proof.review.append.entries.build]

#### Scenario: Iteration archives only after validation

- GIVEN `src/append_entries_build.rs` has been selected by the proof-review loop
- WHEN the change is prepared for archive
- THEN focused checks, Verus, Tracey, Cairn gates, and repo verification evidence MUST be present and successful
