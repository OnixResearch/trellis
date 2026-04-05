# Loop Detection Specification

## Purpose

Defines requirements for verified natural loop detection on a CFG with
dominator information. Covers back edge classification, loop body
computation, header dominance, single entry, nesting structure, and
exit edges.

## Requirements

### Requirement: Back Edge Classification

An edge MUST be classified as a back edge iff the target dominates the
source.

#### Scenario: Back edge

- GIVEN edge (B → H) where H dominates B
- WHEN edge classification runs
- THEN the edge is classified as a back edge

#### Scenario: Forward edge

- GIVEN edge (A → B) where A dominates B
- WHEN edge classification runs
- THEN the edge is not a back edge

#### Scenario: Cross edge

- GIVEN edge (A → B) where neither dominates the other
- WHEN edge classification runs
- THEN the edge is not a back edge

### Requirement: Header Dominance

The loop header MUST dominate every node in the loop body.

#### Scenario: Header dominates body node

- GIVEN loop with header H and body node N
- WHEN dominance is checked
- THEN H dominates N

#### Scenario: Header dominates back-edge source

- GIVEN back edge (B → H)
- WHEN dominance is checked
- THEN H dominates B (by back-edge definition)

### Requirement: Loop Body Correctness

The loop body MUST be exactly the header plus all nodes that can reach
the back-edge source without passing through the header.

#### Scenario: Body membership

- GIVEN node N that can reach the back-edge source B without going through H
- WHEN body membership is checked
- THEN N is in the loop body

#### Scenario: Non-membership

- GIVEN node M where every path to B passes through H (and M ≠ H)
- WHEN body membership is checked
- THEN M is not in the loop body

#### Scenario: Header included

- GIVEN the loop header H
- WHEN body membership is checked
- THEN H is in the loop body

### Requirement: Single Entry

The loop header MUST be the only entry point to the loop from outside.

#### Scenario: External edges enter header

- GIVEN an edge from outside the loop body to a node inside the body
- WHEN the target is checked
- THEN the target is the header H

#### Scenario: No side entries

- GIVEN a non-header body node N
- WHEN N's predecessors from outside the loop are listed
- THEN the list is empty

### Requirement: Loop Nesting

If loop L1 is nested inside loop L2, L1's body MUST be a strict subset
of L2's body.

#### Scenario: Nested body subset

- GIVEN inner loop L1 nested in outer loop L2
- WHEN bodies are compared
- THEN L1.body ⊂ L2.body

#### Scenario: Inner header in outer body

- GIVEN inner loop L1 nested in outer loop L2
- WHEN L1.header is checked against L2.body
- THEN L1.header ∈ L2.body

### Requirement: Nesting Forest

The loop nesting relation MUST form a forest.

#### Scenario: No nesting cycles

- GIVEN loops L1 and L2
- WHEN L1 is nested in L2 and L2 is nested in L1
- THEN this situation is impossible (L1 == L2)

#### Scenario: Unique nesting parent

- GIVEN a loop L nested in two loops L1 and L2
- WHEN L1 and L2 are compared
- THEN one is nested in the other (strict nesting hierarchy)

### Requirement: Exit Edges

All edges from loop body nodes to non-body nodes MUST be identified as
exit edges.

#### Scenario: Exit edge detected

- GIVEN edge (S → T) where S is in the loop body and T is not
- WHEN exit edges are listed
- THEN this edge is included

#### Scenario: Internal edge excluded

- GIVEN edge (S → T) where both S and T are in the loop body
- WHEN exit edges are listed
- THEN this edge is not included
