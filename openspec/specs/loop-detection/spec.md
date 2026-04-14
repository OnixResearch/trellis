# Loop Detection Specification

## Purpose

Defines requirements for verified natural loop detection on a CFG together
with reachability and dominator views for that graph. Covers back edge
classification for all CFG edges, exact body/entry/exit helpers over those
views, and body/header/nesting properties for reachable non-self natural
loops.

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

For reachable non-self loops, the loop header MUST dominate every node in
the loop body.

#### Scenario: Header dominates body node

- GIVEN loop with header H and body node N
- WHEN dominance is checked
- THEN H dominates N

#### Scenario: Header dominates back-edge source

- GIVEN back edge (B → H)
- WHEN dominance is checked
- THEN H dominates B (by back-edge definition)

### Requirement: Loop Body Closure

Given a CFG, a reachability bitmap, and a dominance matrix for the same
graph, the loop body for (B → H) MUST be exactly the seeded backward
closure: start from {H, B}, then repeatedly add any reachable node
dominated by H whose successor is already in the current body, excluding
H as an interior successor for non-self loops.

#### Scenario: Seeded nodes included

- GIVEN the loop header H and back-edge source B
- WHEN body construction starts
- THEN both H and B are in the body

#### Scenario: Backward predecessor added

- GIVEN a reachable node N dominated by H with an edge N → M and M already in the current body
- WHEN one closure round runs
- THEN N is in the next body

#### Scenario: Non-self header barrier respected

- GIVEN a non-self loop and an edge X → H
- WHEN X is considered as an interior predecessor step
- THEN X is not added solely by stepping through H in that round

#### Scenario: Path oracle agreement in tests

- GIVEN reachability and dominator views produced by the repo's analyses
- WHEN randomized tests compare the closure result against brute-force path enumeration
- THEN they agree for reachable non-self back edges

### Requirement: Single Entry

Given a reachability bitmap for the CFG entry and a detected non-self loop
body, the loop header MUST be the only reachable entry point to the loop
from outside.

#### Scenario: External edges enter header

- GIVEN an edge from outside the loop body to a node inside the body
- WHEN the target is checked using the reachability bitmap
- THEN the target is the header H

#### Scenario: No reachable side entries

- GIVEN a non-header body node N
- WHEN N's reachable predecessors from outside the loop are checked
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
