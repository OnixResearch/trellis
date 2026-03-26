# Raft Inductive Invariants Specification

## Purpose

Defines the system-level invariants, ghost state, step decomposition,
and linearizability framework that connect the existing leaf-level Raft
primitives into end-to-end safety proofs. Based on the Verdi Raft proof
(Woos, Wilcox et al., CPP 2016) adapted to Verus spec types.

## Requirements

### Ghost Vote History

#### Requirement: Vote Record Structure

A vote record MUST capture (term, candidate_id, voter_log_epoch) at the
time the vote was cast. `VoteRecord` MUST be a pure spec struct with no
runtime representation.

##### Scenario: Record a vote

- GIVEN a voter in term 5 with log epoch (term=4, index=10) voting for candidate 3
- WHEN the vote record is created
- THEN it MUST contain term=5, candidate_id=3, voter_epoch=(4, 10)

#### Requirement: Vote History Accumulation

`append_vote(history, record)` MUST add a new VoteRecord to the history
sequence. The history MUST be append-only — existing records MUST NOT be
modified or removed. `history.len()` MUST increase by 1.

##### Scenario: Two votes in different terms

- GIVEN history with one record for term 3
- WHEN a vote for term 5 is appended
- THEN history.len() == 2 and history[0].term == 3 and history[1].term == 5

#### Requirement: Single Vote Per Term in History

For any term t, the history MUST contain at most one VoteRecord with
`record.term == t`. `single_vote_per_term(history)` MUST return true
when this invariant holds.

##### Scenario: Duplicate term rejected

- GIVEN a history containing a vote for term 5 for candidate A
- WHEN checking single_vote_per_term after adding a vote for term 5 for candidate B
- THEN single_vote_per_term MUST return false

##### Scenario: Different terms allowed

- GIVEN a history with votes for terms 3 and 5
- WHEN checking single_vote_per_term
- THEN MUST return true (different terms, no conflict)

#### Requirement: Vote History Implies Ballot State

If `single_vote_per_term(history)` holds and the history contains a
VoteRecord for term t with candidate c, then `ballot::can_grant_vote_spec`
MUST return false for term t with any candidate != c. The ghost history
is consistent with the ballot module's single-vote guarantee.

##### Scenario: History consistent with ballot

- GIVEN history records a vote for candidate 3 in term 5
- WHEN ballot::can_grant_vote is checked for candidate 7 in term 5
- THEN MUST return false

### Ghost Leader Logs

#### Requirement: Leader Log Snapshot Structure

A leader log snapshot MUST capture (term, log_at_election) — the term in
which the node became leader and the log contents at that moment.
`LeaderLogSnapshot` MUST be a pure spec struct.

##### Scenario: Capture leader log

- GIVEN a node winning election in term 5 with log [(t=1,v=10), (t=2,v=20), (t=5,v=0)]
- WHEN the snapshot is created
- THEN it MUST contain term=5 and log matching those entries

#### Requirement: Leader Log History Accumulation

`append_leader_log(history, snapshot)` MUST add a LeaderLogSnapshot.
Append-only, length increases by 1.

#### Requirement: Leader Log Matches Actual Log

At the moment of election, the leader log snapshot MUST be a prefix of
the node's actual log. `leader_log_prefix_valid(snapshot, actual_log)`
MUST return true when `logs_match_prefix(snapshot.log, actual_log,
snapshot.log.len())`.

##### Scenario: Snapshot matches prefix

- GIVEN a snapshot with log [(t=1), (t=2)] taken at election
- WHEN the actual log is [(t=1), (t=2), (t=5)]
- THEN leader_log_prefix_valid MUST return true

#### Requirement: Leader Log Contains Committed Entries

If an entry is committed before a leader is elected in term T, the
LeaderLogSnapshot for term T MUST contain that entry. This is the ghost
state form of leader completeness.

##### Scenario: Prior committed entry in new leader log

- GIVEN entry committed at index 2 in term 3
- WHEN leader elected in term 5
- THEN snapshot for term 5 MUST have log[2].term == 3

### Ghost Entry Provenance

#### Requirement: Entry Provenance Record

An entry provenance record MUST capture (term_added, entry, node_id) —
the term in which the entry was added to a node's log and which node
held it. `EntryProvenance` MUST be a pure spec struct.

#### Requirement: All Entries Tracking

`all_entries(node_id)` MUST contain every entry that has ever been present
in the node's log, tagged with the term in which it was observed.
Entries are added when log_append or resolve_conflict introduces them.
Entries are never removed from all_entries even when the log is truncated.

##### Scenario: Entry survives truncation in provenance

- GIVEN a node whose log had entry E at index 3
- WHEN the log is truncated to length 2
- THEN all_entries MUST still contain E

#### Requirement: Provenance Affinity

Any property P that holds for every entry in a node's current log MUST
also hold for every entry in all_entries at the time it was added. This
is the higher-order affinity lemma from the Verdi proof.

##### Scenario: Log sorted indices

- GIVEN that all entries in the current log have positive indices
- WHEN checking all_entries
- THEN every entry in all_entries had a positive index when it was live

### Election Safety (System Level)

#### Requirement: At Most One Leader Per Term

For any term t, at most one node MUST be leader in term t. If node A
is leader in term t and node B is leader in term t, then A == B.
`election_safe(leaders, term)` MUST return true when the count of
leaders for term t is <= 1.

##### Scenario: Single leader

- GIVEN a 5-node cluster where node 2 won election in term 5
- WHEN checking election safety for term 5
- THEN MUST return true (exactly 1 leader)

##### Scenario: No leader (election failed)

- GIVEN a 5-node cluster where no candidate got a majority in term 5
- WHEN checking election safety for term 5
- THEN MUST return true (0 leaders <= 1)

#### Requirement: Election Safety Proof Chain

The proof MUST chain:
1. `ballot::can_grant_vote_spec` → single vote per term per node
2. `ghost_history::single_vote_per_term` → at most one candidate
   receives each voter's vote for a given term
3. `quorum::prove_quorum_intersection` → two quorums of the same
   cluster share a member
4. Contradiction: the shared member voted for both candidates,
   violating single_vote_per_term

##### Scenario: Two leaders contradicted

- GIVEN hypothetical leaders A and B both in term 5 of a 5-node cluster
- WHEN A has quorum {1,2,3} and B has quorum {3,4,5}
- THEN node 3 voted for both A and B in term 5
- AND this violates single_vote_per_term
- AND therefore at most one of A, B is leader

#### Requirement: Election Safety Uses Ghost State

The proof MUST use `ghost_history::VoteRecord` to track that a vote was
cast, not just that `ballot::can_grant_vote_spec` would reject a second
vote. The ghost history bridges the gap between the per-node ballot check
and the system-wide uniqueness claim.

### Raft Step Decomposition

#### Requirement: Raft State Spec

`RaftState` MUST be a spec struct capturing the verifiable portion of a
Raft node's state: term, vote_state, log, commit_index, node_type,
match_indices, next_indices, and ghost state (vote_history,
leader_log_history, all_entries).

##### Scenario: Initial state

- GIVEN a fresh node
- WHEN RaftState is constructed
- THEN term=0, node_type=Follower, log=empty, commit_index=0,
  vote_history=empty, leader_log_history=empty, all_entries=empty

#### Requirement: Invariant Predicate Type

An invariant MUST be a `spec fn(RaftState) -> bool`. The framework MUST
support composing invariants with conjunction: if `inv_a` and `inv_b`
are invariants, `|s| inv_a(s) && inv_b(s)` is an invariant.

#### Requirement: Handler Decomposition

Each top-level handler (handle_message, handle_input, handle_timeout)
MUST decompose into:
1. Event-specific code (message dispatch)
2. `leader_heartbeat` (send AppendEntries, update commit)
3. `execute_entries` (apply committed entries to state machine)

An invariant P MUST be provably preserved by the full handler if it is
preserved by each sub-step independently.

##### Scenario: Reordering tolerance

- GIVEN an invariant P preserved by each sub-step
- WHEN leader_heartbeat and execute_entries are reordered
- THEN P is still preserved (sub-step independence)

#### Requirement: Intermediate Reachability

`intermediate_reachable(state)` MUST be a spec predicate that is a
superset of actual reachability. A state is intermediate-reachable if
it is the initial state, reachable via a network step from an
intermediate-reachable state, or reachable via a single sub-step
(leader_heartbeat or execute_entries) from an intermediate-reachable
state.

##### Scenario: Overapproximation

- GIVEN a state reachable only after leader_heartbeat but before
  execute_entries
- WHEN checking intermediate_reachable
- THEN MUST return true (intermediate states are included)

##### Scenario: Sound overapproximation

- GIVEN a state that is actually reachable via the network semantics
- WHEN checking intermediate_reachable
- THEN MUST return true (actual reachability ⊆ intermediate reachability)

#### Requirement: Decomposition Soundness

If `P(state)` holds for all intermediate-reachable states, then `P(state)`
holds for all actually reachable states. `intermediate_implies_actual(P)`
MUST be provable from the definition of intermediate_reachable.

### Log Matching Inductive Proof

#### Requirement: Log Matching Invariant

`log_matching_invariant(state_a, state_b)` MUST hold for any two nodes
in any reachable state: if both logs have an entry at index i with the
same term, then all entries at indices [0, i) also have the same term.
This is `log::logs_match_prefix` universally quantified over all pairs
of nodes.

##### Scenario: Two followers agree

- GIVEN nodes A and B both received entries from the same leader
- WHEN both have entry at index 5 with term 3
- THEN log_a[0..5] and log_b[0..5] have identical terms

#### Requirement: AppendEntries Preserves Log Matching

If `log_matching_invariant` holds before processing an AppendEntries,
it MUST hold after. The proof MUST chain:
1. `append_entries_check::check_append_entries` validates
   prev_log_index/prev_log_term
2. The consistency check guarantees the follower's log matches the
   leader's through prev_log_index
3. `log_conflict::resolve_conflict` truncates at the divergence point
   and appends leader entries
4. The resulting log agrees with the leader's log at all covered indices
5. Therefore log_matching_invariant is preserved

##### Scenario: Follower accepts matching entries

- GIVEN log matching holds and leader sends AE with prev=(3, term=2)
- WHEN follower checks log[2].term == 2 (pass) and appends entries
- THEN log matching still holds between follower and all other nodes

##### Scenario: Follower truncates conflicting entries

- GIVEN log matching holds and leader sends AE with prev=(3, term=2)
- WHEN follower has log[2].term == 5 (fail) and backtrack occurs
- THEN after conflict resolution, log matching is restored

#### Requirement: Leader Append Preserves Log Matching

When a leader appends a new entry (via `leader_append_only::leader_append_entry`),
log_matching_invariant is preserved because the new entry extends the
log without modifying existing entries.

#### Requirement: Election Preserves Log Matching

A new leader election does not modify any log. Therefore
log_matching_invariant is trivially preserved across election transitions.

### Trace Relations

#### Requirement: Trace Event Type

A trace event MUST be one of:
- `ClientRequest(command, uid)` — input from a client
- `ClientResponse(uid, result)` — output to a client
- `Internal` — not visible to clients (filtered from traces)

##### Scenario: Trace filtering

- GIVEN events [ClientRequest, Internal, Internal, ClientResponse]
- WHEN filtering to client-visible events
- THEN result is [ClientRequest, ClientResponse]

#### Requirement: Trace Relation Definition

A trace relation (T, R) MUST consist of:
1. A trace property `T: spec fn(Seq<TraceEvent>) -> bool`
2. A state property `R: spec fn(RaftState) -> bool`
3. `R` is stable: `R(state) && step(state, state') ==> R(state')`
4. `T` is initially false: `!T(empty_trace)`
5. `T` is decidable (constructive)
6. When `T` becomes true on a step, `R` holds on the post-state

##### Scenario: Response implies execution

- GIVEN T = "response for command c is in the trace"
- GIVEN R = "command c has been executed on at least one state machine"
- WHEN a node outputs a ClientResponse for c
- THEN R holds (node executed c before responding)

#### Requirement: Trace Relation Validity

`trace_relation_valid(T, R)` MUST be a proof that for any execution
satisfying T, R holds on the final state. The proof proceeds by
induction on the step sequence, using T_initially_false for the base
case and R_stable + T_implies_R for the inductive case.

#### Requirement: Inverse Trace Relation

An inverse trace relation (R, T) MUST have:
1. `R` is initially false
2. `T` is stable
3. When R becomes true, T holds on the resulting trace

This proves "if a state property holds, a trace property holds" — the
opposite direction from a trace relation.

##### Scenario: Execution implies response

- GIVEN R = "command c has been executed"
- GIVEN T = "response for c is in the trace (or will be)"
- WHEN c is executed
- THEN R becoming true guarantees T holds

### Linearizability

#### Requirement: Sequential Trace

A trace MUST be sequential if it consists of alternating
ClientRequest/ClientResponse pairs with matching uids. No two requests
share a uid. Every response follows its corresponding request.

##### Scenario: Valid sequential trace

- GIVEN [Request(cmd=Put "x" 1, uid=1), Response(uid=1, result=""),
  Request(cmd=Get "x", uid=2), Response(uid=2, result="1")]
- WHEN checking sequential
- THEN MUST return true

##### Scenario: Out-of-order trace

- GIVEN [Response(uid=1, ...), Request(uid=1, ...)]
- WHEN checking sequential
- THEN MUST return false (response before request)

#### Requirement: Linearizes-To Relation

A trace τ linearizes to a command sequence σ if the events of τ can be
reordered into a sequential trace τ' such that:
1. τ' is sequential
2. τ' agrees with σ (same commands in same order, matching outputs)
3. If an output o appears before an input i in τ, then o also appears
   before i in τ' (acknowledged operations are not reordered before
   later requests)

##### Scenario: Concurrent requests reorderable

- GIVEN two concurrent requests (neither acknowledged before the other
  is sent)
- WHEN linearizing
- THEN either order is valid in τ'

##### Scenario: Sequential requests ordered

- GIVEN request A acknowledged, then request B sent
- WHEN linearizing
- THEN A MUST appear before B in τ'

#### Requirement: Linearizability Definition

A trace τ is linearizable if there EXISTS a command sequence σ such that
τ linearizes to σ. `is_linearizable(trace)` MUST return true when such
a σ exists.

#### Requirement: State Machine Safety Implies Linearizability

If state machine safety holds (committed entries agree across all nodes),
then every trace produced by the Raft system is linearizable. The proof
constructs σ from the committed command sequence and shows the three
conditions of linearizes-to hold.

##### Scenario: Safe system is linearizable

- GIVEN a Raft cluster where state_machine_safety holds in every
  reachable state
- WHEN any execution trace τ is produced
- THEN is_linearizable(τ) MUST return true

#### Requirement: Linearization Witness Construction

The witness σ MUST be the sequence of commands executed by the nodes on
their local state machines, ordered by commit index. State machine safety
guarantees all nodes agree on this sequence, so σ is well-defined.

##### Scenario: Witness from committed log

- GIVEN committed entries [(t=1, cmd=Put "x" 1), (t=2, cmd=Put "y" 2)]
- WHEN constructing σ
- THEN σ = [Put "x" 1, Put "y" 2]

### Resolve Conflict Verification Closure

#### Requirement: Close resolve_conflict external_body

`log_conflict::resolve_conflict` MUST have its `#[verifier::external_body]`
removed. The exec function MUST be verified against `resolve_conflict_spec`.

The append loop invariant MUST connect the loop state to
`append_entries_spec` via `resolve_start_state`. At each iteration j,
the follower's contents MUST equal
`append_entries_spec(resolve_start_state(...), leader_entries, j)`.

##### Scenario: Loop invariant holds at each iteration

- GIVEN the loop at iteration j with 3 leader entries remaining
- WHEN one entry is pushed
- THEN the follower state matches append_entries_spec at j+1

#### Requirement: Resolve Start State Correctness

`resolve_start_state` MUST be proved equivalent to the truncation step
of `resolve_conflict_spec`. After truncation (or no truncation when
there's no conflict), the follower's actual state MUST match what
`resolve_start_state` computes.

### Leader Completeness Precondition Closure

#### Requirement: Shared Voter Existence

The proof MUST establish that in every reachable state where an entry is
committed and a new leader is elected, there EXISTS a node that is in
both the commitment quorum and the election quorum. This follows from
`quorum::prove_quorum_intersection` applied to the ghost state.

##### Scenario: Existence from ghost state

- GIVEN ghost vote_history shows quorum Q2 voted for leader in term 5
- GIVEN ghost shows quorum Q1 replicated entry committed in term 3
- WHEN Q1 and Q2 are both quorums of the same cluster
- THEN there exists a node in Q1 ∩ Q2

#### Requirement: Shared Voter Log Freshness

The shared voter's log epoch at vote time MUST be >= the committed
entry's epoch. This follows from:
1. The voter had the committed entry (it was in Q1)
2. `ballot::can_grant_vote_spec` requires the candidate's epoch >=
   voter's epoch
3. The voter's epoch >= committed entry's epoch (the voter has the entry)

##### Scenario: Voter epoch chain

- GIVEN voter had entry committed at (term=3, index=5)
- WHEN voter's log epoch at vote time is (term=4, index=8)
- THEN voter epoch (4,8) >= committed epoch (3,5) ✓
