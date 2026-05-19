## MODIFIED Requirements

### Requirement: Finite Raft Model Envelope

The system MUST expose a focused finite Raft model envelope implementation surface containing node-universe bounds, term bounds, log-index bounds, command identifier bounds, message-kind admission, configuration-epoch bounds, and deterministic rejection metadata before higher-level Raft transition predicates rely on those values.

r[domain-primitives.raft-consensus.model]

#### Scenario: Admitted references share one model boundary

- GIVEN a Raft primitive references node, term, log-index, command, message-kind, and configuration-epoch metadata
- WHEN model admission is evaluated
- THEN each value is accepted only when it falls inside the finite model envelope


#### Scenario: Unknown node reports model metadata

- GIVEN a Raft state or message references node `0` or a node above the finite model bound
- WHEN model admission is evaluated
- THEN the reference is rejected with unknown-node metadata and no higher-level transition is admitted


#### Scenario: Out-of-bound term or index is rejected

- GIVEN a Raft primitive carries a term or log index beyond the finite model bound
- WHEN model admission is evaluated
- THEN the candidate is rejected with bound metadata before transition-specific predicates run
