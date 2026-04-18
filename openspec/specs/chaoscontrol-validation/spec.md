# ChaosControl Validation Specification

## Purpose

Defines deterministic validation requirements that are part of this change's
contract, not optional review notes.

## Requirements

### Requirement: Scheduler and Fault-Schedule Exec Paths Stay Acyclic

r[chaoscontrol-validation.iterative-exec]
The exec helpers in `src/chaoscontrol_scheduler.rs` and
`src/chaoscontrol_fault_schedule.rs` MUST remain acyclic. Validation for this
change uses deterministic command
`python3 scripts/check-iterative-exec.py src/chaoscontrol_scheduler.rs src/chaoscontrol_fault_schedule.rs`.
That audit collects functions in those two files that are not `spec fn` or
`proof fn`, requires intra-module exec helper calls to use bare local function
names, builds the module-local exec call graph from those bare local calls, and
rejects any self-edge or cycle. The audit result MUST be recorded in
`openspec/changes/chaoscontrol-primitives/evidence/iterative-exec-audit.md`.

#### Scenario: Direct or mutual exec recursion is rejected

- GIVEN either target module contains an exec helper that calls itself or participates in an exec-call cycle
- WHEN the deterministic iterative-exec audit is run
- THEN the audit fails

#### Scenario: Acyclic exec helpers pass the audit

- GIVEN both target modules contain only acyclic exec-helper call graphs
- WHEN the deterministic iterative-exec audit is run
- THEN the audit succeeds and records its result
