# State Machine Workflow Domain Primitive Specification

## Purpose

Defines spec-only requirements for reusable finite state-machine/workflow
primitives. These primitives MUST use exact finite state sets, transition
relations, events, guard predicates, workflow steps, dependencies, reachability
envelopes, terminal-state predicates, transition traces, and deterministic
transition admission results. They MUST avoid runtime workflow-engine semantics,
distributed orchestration, external queue/worker behavior, wall-clock scheduling,
liveness/fairness, exactly-once execution, retry behavior, external storage
durability, business-process completeness, saga/transaction correctness, or
external system correctness unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: State Set Envelope [r[domain-primitives.state-machine-workflow.state]]
The system MUST define finite state-set envelopes that admit declared states and
record initial and terminal-state metadata before transitions or traces reference
them.

#### Scenario: Initial state must be declared [r[domain-primitives.state-machine-workflow.state.scenario.initial-state-must-be-declared]]

- GIVEN a finite state-machine declaration with an initial state
- WHEN state-set admission is evaluated
- THEN the declaration is admitted only if the initial state belongs to the finite declared state set

### Requirement: Transition Relation Envelope [r[domain-primitives.state-machine-workflow.transition]]
The system MUST define transition relation envelopes over finite source state,
event, target state, and optional guard metadata.

#### Scenario: Transition endpoints must be declared states [r[domain-primitives.state-machine-workflow.transition.scenario.endpoints-must-be-declared-states]]

- GIVEN a transition whose source or target state is absent from the declared state set
- WHEN transition admission is evaluated
- THEN the transition is rejected with endpoint metadata

### Requirement: Event Envelope [r[domain-primitives.state-machine-workflow.event]]
The system MUST define finite event envelopes that admit declared event symbols
before transitions or workflow steps reference them.

#### Scenario: Unknown event is rejected [r[domain-primitives.state-machine-workflow.event.scenario.unknown-event-rejected]]

- GIVEN a transition referencing an event absent from the finite event set
- WHEN event admission is evaluated
- THEN the event reference is rejected with unknown-event metadata

### Requirement: Guard Predicate Envelope [r[domain-primitives.state-machine-workflow.guard]]
The system MUST define deterministic guard predicate envelopes over admitted
transition context without claiming external side effects or runtime execution.

#### Scenario: Failed guard blocks transition admission [r[domain-primitives.state-machine-workflow.guard.scenario.failed-guard-blocks-transition-admission]]

- GIVEN an admitted source state, event, target state, and guard predicate metadata
- WHEN the guard predicate evaluates to false in the finite transition context
- THEN transition admission rejects the guarded transition with failed-guard metadata

### Requirement: Transition Admission Envelope [r[domain-primitives.state-machine-workflow.admission]]
The system MUST define deterministic transition admission and rejection envelopes
for public exec surfaces that combine state, transition, event, and guard checks.

#### Scenario: Undeclared transition is rejected deterministically [r[domain-primitives.state-machine-workflow.admission.scenario.undeclared-transition-rejected-deterministically]]

- GIVEN a transition request that is absent from the finite transition relation
- WHEN transition admission is evaluated
- THEN the request is rejected with undeclared-transition metadata

### Requirement: Workflow Step Envelope [r[domain-primitives.state-machine-workflow.step]]
The system MUST define finite workflow step envelopes that reference declared
states, events, and transition metadata without claiming runtime step execution.

#### Scenario: Workflow step references declared transition metadata [r[domain-primitives.state-machine-workflow.step.scenario.references-declared-transition-metadata]]

- GIVEN a workflow step with source state, event, and target state references
- WHEN workflow step admission is evaluated
- THEN the step is admitted only if those references match declared transition metadata

### Requirement: Workflow Dependency Envelope [r[domain-primitives.state-machine-workflow.dependency]]
The system MUST define finite workflow dependency envelopes over declared steps
with deterministic dependency-shape admission metadata.

#### Scenario: Dependency references declared workflow steps [r[domain-primitives.state-machine-workflow.dependency.scenario.references-declared-workflow-steps]]

- GIVEN a workflow dependency edge between two step identifiers
- WHEN dependency admission is evaluated
- THEN the dependency is admitted only if both step identifiers are declared in the finite workflow step set

### Requirement: Reachability Path Envelope [r[domain-primitives.state-machine-workflow.reachability]]
The system MUST define bounded reachability/path envelopes over finite admitted
transition systems and explicit path-witness metadata.

#### Scenario: Reachability witness follows admitted transitions [r[domain-primitives.state-machine-workflow.reachability.scenario.witness-follows-admitted-transitions]]

- GIVEN a finite path witness from one state to another
- WHEN reachability admission is evaluated
- THEN each adjacent state pair in the witness follows an admitted transition relation entry

### Requirement: Terminal State Predicate Envelope [r[domain-primitives.state-machine-workflow.terminal]]
The system MUST define terminal-state predicate envelopes over finite declared
states without claiming liveness, eventual completion, or runtime scheduling.

#### Scenario: Terminal predicate only accepts declared terminal state [r[domain-primitives.state-machine-workflow.terminal.scenario.only-accepts-declared-terminal-state]]

- GIVEN a state candidate
- WHEN terminal-state predicate evaluation is performed
- THEN the predicate matches only if the candidate is declared as terminal in the finite state metadata

### Requirement: Transition Trace Envelope [r[domain-primitives.state-machine-workflow.trace]]
The system MUST define finite transition trace envelopes whose consecutive trace
items are stepwise admitted by the transition relation and guard metadata.

#### Scenario: Trace rejects invalid step [r[domain-primitives.state-machine-workflow.trace.scenario.rejects-invalid-step]]

- GIVEN a finite transition trace containing one step absent from the admitted transition relation
- WHEN trace admission is evaluated
- THEN the trace is rejected with the invalid step index and transition metadata
