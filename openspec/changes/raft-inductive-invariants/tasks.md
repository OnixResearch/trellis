## Phase 1: Ghost State Types + Election Safety

- [x] Create src/ghost_history.rs: VoteRecord spec struct (term, candidate_id, voter_last_term, voter_last_index) ✅
- [x] Implement append_vote spec fn on Seq<VoteRecord> ✅
- [x] Implement single_vote_per_term spec fn: forall i,j if history[i].term == history[j].term then i == j ✅
- [x] Prove append preserves single_vote_per_term when new record's term is fresh ✅ (append_fresh_preserves_single_vote + append_revote_preserves_single_vote)
- [x] Prove vote_history_implies_ballot: single_vote_per_term + history contains (t, c) → ballot rejects (t, c') for c' ≠ c ✅ (vote_excludes_other_candidate)
- [x] Create LeaderLogSnapshot spec struct (term, log: Seq<LogEntry>) ✅
- [x] Implement append_leader_log spec fn on Seq<LeaderLogSnapshot> ✅
- [x] Implement leader_log_prefix_valid spec fn: logs_match_prefix(snapshot.log, actual, snapshot.log.len()) ✅
- [x] Create EntryProvenance spec struct (term_observed, entry: LogEntry, node_id: nat) ✅
- [x] Implement record_entries spec fn: entry tracking with append-only provenance ✅
- [x] Prove affinity: record_full_log_covers shows all log entries have provenance records ✅
- [x] Add ghost_history to lib.rs module list ✅
- [x] Create src/election_safety_proof.rs: election_safe spec fn (cronies-based quorum check) ✅
- [x] Prove election_safety_theorem: single_vote_per_term + cronies_exclusive → at most one leader ✅
- [x] Proof step: assume two candidates A ≠ B both have quorums ✅
- [x] Proof step: cronies_exclusive shows one has zero cronies ✅
- [x] Proof step: zero cronies < majority (majority >= 1) → contradiction ✅
- [x] Proof step: therefore A == B ✅
- [x] Prove quorum_winner_unique: winner has quorum → challenger does not ✅
- [x] Prove quorum_intersection_contradicts_two_winners: votes overlap ✅
- [x] Prove empty_history_safe + fresh_vote_preserves_other_terms ✅
- [x] Add election_safety_proof to lib.rs module list ✅
- [x] Unit tests: concrete 3/5/7-node clusters with simulated vote histories ✅ (10 tests)
- [x] Proptest: random cluster sizes, random vote assignments, verify election_safe predicate ✅ (3 proptests ghost_history + 3 proptests election_safety)
- [x] Verus clean ✅ (pre-existing check_quorum type error unrelated; new modules have 0 errors)

## Phase 2: Step Decomposition + Log Matching Inductive

- [ ] Create src/raft_step.rs: RaftState spec struct (term, vote_state, log, commit_index, node_type, match_indices, next_indices, ghost fields)
- [ ] Define initial_raft_state spec fn
- [ ] Define step_preserves spec fn: inv(pre) ==> inv(post)
- [ ] Define handler_decomposition spec: handler = specific_code ; leader_heartbeat ; execute_entries
- [ ] Define intermediate_reachable spec predicate (superset of actual reachability)
- [ ] Prove decomposition_soundness: if P holds for all intermediate-reachable states then P holds for all reachable states
- [ ] Prove invariant_conjunction: if P and Q are both preserved by each sub-step, P && Q is preserved
- [ ] Add raft_step to lib.rs module list
- [ ] Create src/log_invariant_inductive.rs: log_matching_invariant spec fn
- [ ] Prove log_matching_initial: log_matching_invariant holds for initial state (empty logs)
- [ ] Prove log_matching_append_entries: if invariant holds pre-AE, check_append_entries + resolve_conflict preserves it
- [ ] Sub-proof: check_append_entries validates prefix agreement
- [ ] Sub-proof: resolve_conflict preserves matching prefix before conflict point
- [ ] Sub-proof: entries after conflict come from leader (match leader's log)
- [ ] Prove log_matching_leader_append: leader_append_entry preserves log_matching_invariant
- [ ] Prove log_matching_election: election transitions don't modify logs, invariant trivially preserved
- [ ] Prove log_matching_heartbeat: heartbeat sends AE messages but doesn't modify leader log, preserved
- [ ] Cross-module: chain append_entries_check::consistency_implies_agreement → log::logs_match_prefix
- [ ] Cross-module: chain leader_append_only::append_satisfies_invariant → log extension preserves matching
- [ ] Unit tests: multi-step simulations with log matching checked after each step
- [ ] Proptest: random interleaving of append, truncate, conflict resolution on 3-node simulated cluster
- [ ] Verus clean

## Phase 3: Close Verification Gaps

- [ ] Remove #[verifier::external_body] from log_conflict::resolve_conflict
- [ ] Prove truncation branch: after truncate(follower, start_index + offset), follower == resolve_start_state(...)
- [ ] Prove no-conflict branch: after computing overlap, follower state == resolve_start_state(...)
- [ ] Prove append loop invariant: at iteration j, follower@ == append_entries_spec(resolve_start_state(...), leader_entries@, j)
- [ ] Prove loop base: at j == append_from, follower equals resolve_start_state
- [ ] Prove loop step: push(leader_entries[j]) advances append_entries_spec from j to j+1
- [ ] Prove loop termination: at j == leader_entries.len(), append_entries_spec returns the prefix (identity)
- [ ] Verify the full resolve_conflict function without external_body
- [ ] Update verification-gaps spec to remove resolve_conflict entry
- [ ] Close leader_completeness precondition: prove shared voter existence from ghost_history
- [ ] Proof: ghost vote_history gives quorum Q2 (voters for new leader)
- [ ] Proof: commitment implies quorum Q1 (replication set)
- [ ] Proof: quorum_intersection gives node n ∈ Q1 ∩ Q2
- [ ] Proof: n ∈ Q1 means n has the committed entry → n's epoch >= committed epoch
- [ ] Proof: n ∈ Q2 means n voted for leader → leader's epoch >= n's epoch (ballot::can_grant_vote_spec)
- [ ] Proof: transitivity of epoch_ge → leader has committed entry
- [ ] Strengthen leader_completeness_theorem to derive (not assume) the shared voter's epoch
- [ ] All existing tests still pass
- [ ] Verus clean

## Phase 4: Trace Relations + Linearizability

- [ ] Create src/trace_relation.rs: TraceEvent spec enum (ClientRequest, ClientResponse, Internal)
- [ ] Define trace_property spec type: spec fn(Seq<TraceEvent>) -> bool
- [ ] Define state_property spec type: spec fn(RaftState) -> bool
- [ ] Define TraceRelation spec struct: (T, R, R_stable, T_initially_false, T_implies_R)
- [ ] Prove trace_relation_valid: forall execution satisfying T, R holds on final state
- [ ] Proof: induction on step sequence; base case uses T_initially_false; inductive uses R_stable + T_implies_R
- [ ] Define InverseTraceRelation spec struct: (R, T) with R_initially_false, T_stable, R_implies_T
- [ ] Prove inverse_trace_relation_valid: analogous to forward direction
- [ ] Instantiate: T = "response for cmd c in trace", R = "c executed on state machine" → proves response implies execution
- [ ] Add trace_relation to lib.rs module list
- [ ] Create src/linearizability.rs: is_sequential spec fn (alternating req/resp with matching uids)
- [ ] Define agrees_with spec fn: sequential trace matches command sequence σ on commands and outputs
- [ ] Define respects_ordering spec fn: acknowledged ops not reordered past later requests
- [ ] Define linearizes_to spec fn: exists reordering τ' satisfying all three conditions
- [ ] Define is_linearizable spec fn: exists σ such that τ linearizes to σ
- [ ] State theorem: state_machine_safety_implies_linearizability
- [ ] Proof sketch: construct σ from committed command sequence, construct τ' by commit-index ordering
- [ ] Prove condition 1: τ' is sequential (commands ordered by commit index, responses follow requests)
- [ ] Prove condition 2: τ' agrees with σ (same commands, same outputs from deterministic state machine)
- [ ] Prove condition 3: ordering respected (acknowledged commands committed before later requests)
- [ ] Cross-module: cite state_machine_safety::committed_entries_agree for σ well-definedness
- [ ] Cross-module: cite commit_advance::advance_monotonic for ordering
- [ ] Add linearizability to lib.rs module list
- [ ] Unit tests: concrete trace scenarios checked against linearizability predicate
- [ ] Proptest: random command sequences on simulated single-node state machine, verify linearizable
- [ ] Full verus + tracey clean across all new modules
