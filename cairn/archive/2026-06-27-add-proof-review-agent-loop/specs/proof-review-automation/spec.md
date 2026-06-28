## ADDED Requirements

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
