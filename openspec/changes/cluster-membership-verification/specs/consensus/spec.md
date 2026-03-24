# Cluster Membership Verification — Delta Spec

## ADDED Requirements

### Requirement: Voter Set Promotion

Promoting a learner to voter MUST add the learner's ID to the voter set.
If the learner is already a voter, the set MUST be unchanged (idempotent).
The result MUST contain no duplicate IDs.

#### Scenario: Learner not yet a voter

- GIVEN voters [1, 2, 3] and learner_id 4
- WHEN promoting without replacement
- THEN result MUST be [1, 2, 3, 4]

#### Scenario: Learner already a voter

- GIVEN voters [1, 2, 3] and learner_id 2
- WHEN promoting without replacement
- THEN result MUST be [1, 2, 3] (unchanged, no duplicate)

### Requirement: Voter Set Replacement

When promoting a learner with a replacement target, the replaced node
MUST be absent from the result. All voters other than the replaced node
MUST be preserved. The learner MUST be present in the result.

#### Scenario: Replace existing voter

- GIVEN voters [1, 2, 3], learner_id 4, replace_node Some(2)
- WHEN promoting with replacement
- THEN result MUST be [1, 3, 4]
- AND 2 MUST NOT appear in result
- AND 1 and 3 MUST appear in result

#### Scenario: Replace non-existent node

- GIVEN voters [1, 2, 3], learner_id 4, replace_node Some(99)
- WHEN promoting with replacement
- THEN result MUST be [1, 2, 3, 4]
- AND retain behaves as no-op for absent ID

#### Scenario: Replace with learner already present

- GIVEN voters [1, 2, 3], learner_id 3, replace_node Some(1)
- WHEN promoting with replacement
- THEN result MUST be [2, 3]
- AND 1 MUST NOT appear in result

### Requirement: Voter Set No Duplicates

The result of any promotion operation MUST contain no duplicate node IDs.
This MUST hold regardless of whether the learner was already in the voter
set.

#### Scenario: Duplicate prevention

- GIVEN voters [1, 2, 3] and learner_id 2
- WHEN promoting
- THEN result.len() MUST equal 3 (not 4)

### Requirement: Voter Set Quorum Preservation

If the previous voter set formed a quorum (size > 0), the result of
promotion without replacement MUST also form a quorum. Promotion with
replacement MUST preserve quorum if the previous size satisfies
`can_remove_safely` from the quorum module.

#### Scenario: Promotion grows the set

- GIVEN voters [1, 2, 3] (size 3, majority 2)
- WHEN promoting learner 4 without replacement
- THEN result size is 4, majority is 3, quorum still holds

#### Scenario: Replacement preserves quorum

- GIVEN voters [1, 2, 3, 4, 5] (size 5, can_remove_safely is true)
- WHEN promoting learner 6 replacing node 2
- THEN result size is 5, quorum still holds

#### Scenario: Replacement on 2-node cluster

- GIVEN voters [1, 2] (size 2, can_remove_safely is false)
- WHEN promoting learner 3 replacing node 1
- THEN result size is 2, which still has majority 2

### Requirement: Bounded Peer Collection

Building a bootstrap peer list from candidates MUST enforce a hard
capacity bound. The result size MUST never exceed MAX_PEERS. Candidates
MUST be added in order until the cap is reached.

#### Scenario: Under capacity

- GIVEN MAX_PEERS=16 and 5 candidates
- WHEN building peer list
- THEN result contains all 5 candidates

#### Scenario: At capacity

- GIVEN MAX_PEERS=16 and 20 candidates
- WHEN building peer list
- THEN result contains exactly 16 candidates
- AND they are the first 16 from the input order

### Requirement: Peer Self-Exclusion

The node's own endpoint ID MUST NOT appear in the bootstrap peer list.
Self-exclusion MUST be checked for every candidate, not just the first.

#### Scenario: Self in candidates

- GIVEN self_id=X and candidates [A, X, B, C]
- WHEN building peer list
- THEN result MUST be [A, B, C]
- AND X MUST NOT appear

#### Scenario: Self as only candidate

- GIVEN self_id=X and candidates [X]
- WHEN building peer list
- THEN result MUST be empty

### Requirement: Peer List Non-Empty Guarantee

When at least one non-self candidate exists, the result MUST contain
at least one peer.

#### Scenario: Mixed self and non-self

- GIVEN self_id=X and candidates [X, A]
- WHEN building peer list
- THEN result MUST be [A] (length >= 1)

### Requirement: Node Classification Voter Equivalence

A node MUST be classified as a voter if and only if its ID is in the
member set. A node MUST be classified as a learner if and only if it is
not a voter. `is_voter` and `is_learner` MUST be mutually exclusive and
exhaustive.

#### Scenario: Voter node

- GIVEN member_ids={1, 2, 3} and node_id=2
- WHEN classifying
- THEN is_voter MUST be true AND is_learner MUST be false

#### Scenario: Learner node

- GIVEN member_ids={1, 2, 3} and node_id=5
- WHEN classifying
- THEN is_voter MUST be false AND is_learner MUST be true

### Requirement: At Most One Leader

At most one node in the classification output MUST have `is_leader` set
to true. If `leader_id` is None, zero nodes MUST be marked as leader.

#### Scenario: Leader present

- GIVEN leader_id=Some(2) and nodes [1, 2, 3]
- WHEN classifying all nodes
- THEN exactly node 2 has is_leader=true, nodes 1 and 3 have is_leader=false

#### Scenario: No leader

- GIVEN leader_id=None and nodes [1, 2, 3]
- WHEN classifying all nodes
- THEN all nodes have is_leader=false

### Requirement: Leader Is Voter

If a node is classified as leader, it MUST also be classified as voter.
A learner MUST NOT be the leader.

#### Scenario: Leader in member set

- GIVEN member_ids={1, 2, 3}, leader_id=Some(2)
- WHEN classifying node 2
- THEN is_leader=true AND is_voter=true

### Requirement: Classification Bounded Output

The classification output MUST contain at most MAX_NODES entries.
Input lists exceeding this bound MUST be truncated.

#### Scenario: Truncation

- GIVEN MAX_NODES=100 and 150 input nodes
- WHEN classifying
- THEN result MUST contain exactly 100 entries

### Requirement: Classification No Duplicates

The classification output MUST NOT contain duplicate node IDs. If the
same node ID appears in both the voters list and the learners list in
the input, it MUST appear exactly once in the output.

#### Scenario: Deduplicated

- GIVEN node 5 appears in both voters and learners input
- WHEN classifying
- THEN node 5 appears exactly once in output
