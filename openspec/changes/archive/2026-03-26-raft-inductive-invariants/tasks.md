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

- [x] Create src/raft_step.rs: RaftState spec struct (term, voted_term, voted_for, log, commit_index, node_type, match/next_indices, cluster_size, ghost fields) ✅
- [x] Define initial_raft_state_spec fn ✅
- [x] Define step_preserves_spec fn: inv(pre) ==> inv(post) ✅
- [x] Define handler_preserves_spec: three sub-steps chain into full handler preservation ✅
- [x] Define is_initial_state_spec predicate (intermediate reachability foundation) ✅
- [x] Prove handler_decomposition_sound: chaining three preserving steps preserves end-to-end ✅
- [x] Prove invariant_conjunction: if P and Q are both preserved, P && Q is preserved ✅
- [x] Prove term_monotonic_transitive and commit_monotonic_transitive ✅
- [x] State predicates: is_leader/follower/candidate_state_spec, is_valid_node_type_spec ✅
- [x] Add raft_step to lib.rs module list ✅
- [x] Create src/log_invariant_inductive.rs: log_matching_property_spec ✅
- [x] Prove log_matching_initial: empty logs satisfy LMP (+ empty left/right) ✅
- [x] Prove consistency_check_implies_prefix: check_append_entries → logs_match_prefix ✅
- [x] Prove resolve_preserves_matching: conflict resolution maintains covered entries ✅
- [x] Prove leader_append_preserves_lmp: appending to one log preserves LMP ✅ (+ symmetric right variant)
- [x] Prove election_preserves_lmp: elections don't modify logs ✅
- [x] Prove leader_invariant_preserves_lmp: leader_log_invariant + LMP → LMP preserved ✅
- [x] Cross-module: check_establishes_agreement links check_append_entries to agree_at ✅
- [x] Cross-module: leader_append_only_chain links append_satisfies_invariant to LMP ✅
- [x] Prove ae_exchange_preserves_lmp: full AppendEntries exchange preserves LMP ✅
- [x] Unit tests: multi-step simulations with LMP checked after each step ✅ (7 tests)
- [x] Proptest: append preserves, AE exchange preserves, LMP model oracle ✅ (3 proptests)
- [x] Verus clean ✅ (0 errors in new modules)

## Phase 3: Close Verification Gaps

- [x] Remove #[verifier::external_body] from log_conflict::resolve_conflict ✅
- [x] Prove truncation branch: after truncate(follower, start_index + offset), follower == resolve_start_state(...) ✅
- [x] Prove no-conflict branch: after computing overlap, follower state == resolve_start_state(...) ✅
- [x] Prove append loop invariant: inverted form — append_entries_spec(follower@, entries, j) == full_result ✅
- [x] Prove loop base: at j == append_from, append_entries_spec(rss, entries, append_from) == full_result ✅ (by definition)
- [x] Prove loop step: push(leader_entries[j]) advances via append_entries_step ✅
- [x] Prove loop termination: at j == entries.len(), append_entries_spec(follower@, entries, len) == follower@ ✅ (base case)
- [x] Verify the full resolve_conflict function without external_body ✅ (2949 verified, 0 errors)
- [x] Key fix: original loop invariant was WRONG direction — used "remaining work" form instead ✅
- [x] Close leader_completeness precondition: create leader_completeness_ghost.rs ✅
- [x] Proof: prove_shared_voter_exists — quorum intersection gives overlap ✅
- [x] Proof: voter_has_committed_epoch_spec — shared voter has committed entry ✅
- [x] Proof: leader_epoch_ge_voter_spec — vote granting requires leader >= voter ✅
- [x] Proof: leader_completeness_from_ghost — full chain with transitivity of epoch_ge ✅
- [x] Proof: vote_record_provides_epoch — connects VoteRecord to epoch predicates ✅
- [x] Prove five_node_overlap and higher_term_trivial concrete scenarios ✅
- [x] All existing tests still pass ✅ (1844 total)
- [x] Verus clean ✅ (0 errors in new modules)

## Phase 4: Trace Relations + Linearizability

- [x] Create src/trace_relation.rs: TraceEvent struct (event_type, uid, command, result) ✅
- [x] Define client_visible_spec: filter trace to client events ✅
- [x] Define forward trace relation: r_stable, t_initially_false, t_implies_r ✅
- [x] Prove trace_relation_valid_step: induction step (T after → R after) ✅
- [x] Define inverse trace relation: r_initially_false, t_stable, r_implies_t ✅
- [x] Prove inverse_trace_relation_valid_step: symmetric induction step ✅
- [x] Instantiate: has_response_for (T) + command_executed (R) → response implies execution ✅
- [x] Prove response_stability: R stable because commit_index never decreases ✅
- [x] Prove no_response_initially: T false on empty trace ✅
- [x] Prove response_added: adding response event makes T true ✅
- [x] Add trace_relation to lib.rs module list ✅
- [x] Create src/linearizability.rs: is_sequential_spec (alternating req/resp with matching uids) ✅
- [x] Define agrees_with_spec: sequential trace matches command sequence σ ✅
- [x] Define respects_ordering_spec: acknowledged ops not reordered past later requests ✅
- [x] Define linearizes_to_spec: exists reordering τ' satisfying all three conditions ✅
- [x] Define is_linearizable_spec: exists σ such that τ linearizes to σ ✅
- [x] State safety_implies_linearizability_spec: SMS → linearizability ✅
- [x] Prove safety_implies_linearizability_structure: given σ, conditions satisfiable ✅
- [x] Prove single_command_linearizable: one acknowledged command is linearizable ✅
- [x] Prove empty_trace_linearizable and pair_is_sequential ✅
- [x] Add linearizability to lib.rs module list ✅
- [x] Unit tests: concrete trace scenarios checked against sequential/agrees/linearizable ✅ (9 trace_relation + 8 linearizability)
- [x] Proptest: random command sequences, verify sequential and agrees-with ✅ (2 + 3 proptests)
- [x] Full cargo test clean across all new modules ✅ (1864 total, 0 failures)
