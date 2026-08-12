## MODIFIED Requirements

### Requirement: Node State Envelope

The system MUST expose a finite node-state envelope implementation surface for follower, candidate, and leader states whose role, node identity, current term, voted-for metadata, last-log metadata, commit index, and last-applied index are admitted by the finite Raft model and by role-specific consistency predicates.

r[domain-primitives.raft-consensus.node-state]

#### Scenario: Leader state belongs to current term

- GIVEN a node state is marked leader for a term distinct from its current term
- WHEN node-state admission is evaluated
- THEN the state is rejected with inconsistent-leader-term metadata


#### Scenario: Applied index cannot exceed commit or local log

- GIVEN a node state has last-applied greater than commit index or local last-log index
- WHEN node-state admission is evaluated
- THEN the state is rejected with applied-after-commit or applied-after-log metadata


#### Scenario: Candidate vote metadata is self-consistent

- GIVEN a candidate state records a vote for a different node in the candidate term
- WHEN node-state admission is evaluated
- THEN the state is rejected with inconsistent-candidate-vote metadata
