## Phase 1: OpenSpec Scaffolding

- [x] Create the `consistency-models` OpenSpec change with proposal, design,
  tasks, and a dedicated spec domain
- [x] Capture the Jepsen implication graph and availability envelopes in the
  spec text
- [x] Cover all 16 model pages under `https://jepsen.io/consistency/models`

## Phase 2: Shared History Vocabulary

- [x] Define shared history concepts for future implementation: operations,
  transactions, objects, sessions, visibility, arbitration, and real-time
  order
- [x] Decide which common relations belong in a reusable module versus inside
  model-specific specs
- [x] Add Tracey requirement IDs and source-level implementation plan for the
  shared vocabulary

## Phase 3: Transactional Models

- [x] Add formal predicates and proof skeletons for strict serializability,
  serializability, repeatable read, snapshot isolation, monotonic atomic view,
  cursor stability, read committed, and read uncommitted
- [x] Model transaction-level conflicts, predicate reads, and first-committer-
  wins behavior needed by snapshot isolation
- [x] Add example histories and regression cases for allowed and forbidden
  transactional anomalies

## Phase 4: Object and Session Models

- [x] Add formal predicates and proof skeletons for linearizability,
  sequential consistency, causal consistency, PRAM, writes follow reads,
  monotonic reads, monotonic writes, and read your writes
- [x] Reuse existing vector-clock and causal-order machinery where it matches
  the history-level definitions
- [x] Add example histories that distinguish linearizability vs sequential
  consistency and causal consistency vs PRAM

## Phase 5: Refinement Proofs and Validation

- [x] Prove the implication edges captured in the Jepsen model tree
- [x] Encode availability-envelope metadata for each model family in the main
  spec or implementation docs
- [x] Add model-level tests or checkers for representative legal and illegal
  histories
- [x] Run Verus, cargo test, and Tracey for the eventual implementation
