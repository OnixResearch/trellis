## Coverage Index

| Requirement ID | Future module |
|---|---|
| `domain-primitives.state-machine-workflow.state` | `src/state_machine_state.rs` |
| `domain-primitives.state-machine-workflow.transition` | `src/state_machine_transition.rs` |
| `domain-primitives.state-machine-workflow.event` | `src/state_machine_event.rs` |
| `domain-primitives.state-machine-workflow.guard` | `src/state_machine_guard.rs` |
| `domain-primitives.state-machine-workflow.admission` | `src/state_machine_admission.rs` |
| `domain-primitives.state-machine-workflow.step` | `src/workflow_step.rs` |
| `domain-primitives.state-machine-workflow.dependency` | `src/workflow_dependency.rs` |
| `domain-primitives.state-machine-workflow.reachability` | `src/state_machine_reachability.rs` |
| `domain-primitives.state-machine-workflow.terminal` | `src/state_machine_terminal.rs` |
| `domain-primitives.state-machine-workflow.trace` | `src/state_machine_trace.rs` |

## Phase 1: Spec foundation

- [x] Define finite state-machine/workflow primitive requirements and exact-model non-goals.

## Phase 2: Future implementation slices

- [x] Implement finite state-set admission with initial and terminal metadata. `[covers=domain-primitives.state-machine-workflow.state]`
- [x] Implement finite transition relation envelopes with source/target membership checks. `[covers=domain-primitives.state-machine-workflow.transition]`
- [x] Implement finite event metadata and event-set admission. `[covers=domain-primitives.state-machine-workflow.event]`
- [x] Implement deterministic guard predicate envelopes over admitted transition context. `[covers=domain-primitives.state-machine-workflow.guard]`
- [x] Implement deterministic transition admission/rejection metadata for public exec surfaces. `[covers=domain-primitives.state-machine-workflow.admission]`
- [x] Implement finite workflow step metadata with declared state/event references. `[covers=domain-primitives.state-machine-workflow.step]`
- [x] Implement finite workflow dependency envelopes with dependency-shape admission. `[covers=domain-primitives.state-machine-workflow.dependency]`
- [x] Implement bounded reachability/path envelopes for admitted transition systems. `[covers=domain-primitives.state-machine-workflow.reachability]`
- [x] Implement terminal-state predicate envelopes. `[covers=domain-primitives.state-machine-workflow.terminal]`
- [x] Implement finite transition trace envelopes with stepwise transition validation. `[covers=domain-primitives.state-machine-workflow.trace]`

## Phase 3: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for each implemented slice.
- [x] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final validation.
- [x] Sync and archive the change once all implementation/evidence tasks are complete.
