## Phase 1: Voter Set Promotion

- [x] Add cluster membership requirements to docs/requirements.md ✅
- [x] Implement src/voter_set.rs: promote_spec, promote exec fn ✅
- [x] Prove learner_always_present: learner_id ∈ result for all inputs ✅
- [x] Prove replaced_always_absent: replace_id ∉ result when Some ✅
- [x] Prove others_preserved: ∀v ∈ previous, v ≠ replace_id → v ∈ result ✅
- [x] Prove no_duplicates: result has unique elements ✅
- [x] Prove idempotent_when_present: learner already in set → size unchanged ✅
- [x] Prove quorum_preserved_no_replace: result.len() >= voters.len() ✅
- [x] Prove quorum_preserved_with_replace: result.len() >= voters.len() - 1 ✅
- [x] Proptest: generate random voter sets + learner + replace, check all 5 properties ✅
- [x] Oracle test: match aspen-cluster-handler test expectations exactly ✅
- [x] Verus + tracey clean ✅

## Phase 2: Bounded Peer Collection

- [x] Implement src/bounded_peers.rs: collect_peers_spec, collect_peers exec fn ✅
- [x] Prove bounded: result.len() <= max_peers for all inputs ✅
- [x] Prove self_excluded: self_id never appears in result ✅
- [x] Prove fill_monotonic: candidates added in order, no valid candidate skipped before cap ✅
- [x] Prove non_empty_guarantee: at least one non-self candidate → result.len() >= 1 ✅
- [x] Prove no_duplicates: result has unique elements when input has unique elements ✅
- [x] Proptest: generate random candidate lists + self_id + max_peers, check bounds + exclusion ✅
- [x] Verus + tracey clean ✅

## Phase 3: Node Classification

- [x] Implement src/node_classify.rs: classify_node spec+exec (single node) ✅
- [x] Implement classify_nodes spec+exec (batch with bounds) ✅
- [x] Prove voter_iff_member: is_voter ⟺ node_id ∈ member_ids ✅
- [x] Prove learner_iff_not_voter: is_learner ⟺ ¬is_voter ✅
- [x] Prove voter_learner_exclusive: is_voter ⊕ is_learner (exactly one true) ✅
- [x] Prove leader_implies_voter: is_leader → is_voter ✅
- [x] Prove at_most_one_leader: count of is_leader across all nodes <= 1 ✅
- [x] Prove bounded_output: classify_nodes result.len() <= MAX_NODES ✅
- [x] Prove no_duplicate_ids: result has unique node_ids ✅
- [x] Proptest: generate random member sets + leader + node lists, check classification ✅
- [x] Verus + tracey clean ✅

## Phase 4: Cross-Module Integration

- [x] voter_set::promote → quorum::is_quorum (proptest: full set is always quorum) ✅
- [x] voter_set::promote → quorum::can_remove_safely (proptest: replacement guard) ✅
- [x] node_classify::at_most_one_leader → quorum::majority (proptest: leader is quorum voter) ✅
- [x] Property tests for cross-module relationships ✅
- [x] Verus + tracey clean ✅
