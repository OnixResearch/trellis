## MODIFIED Requirements

### Requirement: Client Proposal Envelope

The system MUST expose a finite client-proposal envelope implementation surface for command identifiers addressed to a modeled Raft node, admitting a proposal only when the target is an admitted leader for the proposal term and rejecting followers, stale terms, unknown commands, or out-of-model nodes with deterministic metadata.

r[domain-primitives.raft-consensus.client-proposal]

#### Scenario: Follower proposal is rejected

- GIVEN a client proposal is addressed to a node not admitted as leader for its current term
- WHEN client-proposal admission is evaluated
- THEN the proposal is rejected with not-leader metadata


#### Scenario: Leader proposal enters append path

- GIVEN a client proposal has an admitted command id, model-valid target, and target node admitted as leader for the proposal term
- WHEN client-proposal admission is evaluated
- THEN the proposal is admitted as eligible for the modeled leader append predicate


#### Scenario: Stale proposal term is rejected

- GIVEN a client proposal carries a term lower than the target leader current term
- WHEN client-proposal admission is evaluated
- THEN the proposal is rejected with stale-proposal-term metadata and no append eligibility
