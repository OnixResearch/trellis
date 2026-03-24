## Phase 1: Voter Set Promotion

- [ ] Add cluster membership requirements to docs/requirements.md
- [ ] Implement src/voter_set.rs: promote_spec, promote exec fn
- [ ] Prove learner_always_present: learner_id ∈ result for all inputs
- [ ] Prove replaced_always_absent: replace_id ∉ result when Some
- [ ] Prove others_preserved: ∀v ∈ previous, v ≠ replace_id → v ∈ result
- [ ] Prove no_duplicates: result has unique elements
- [ ] Prove idempotent_when_present: learner already in set → size unchanged
- [ ] Prove quorum_preserved_no_replace: |previous| > 0 → is_quorum(|result|, |result|)
- [ ] Prove quorum_preserved_with_replace: can_remove_safely(|previous|) → quorum holds after replace
- [ ] Proptest: generate random voter sets + learner + replace, check all 5 properties
- [ ] Oracle test: match aspen-cluster-handler test expectations exactly
- [ ] Verus + tracey clean

## Phase 2: Bounded Peer Collection

- [ ] Implement src/bounded_peers.rs: collect_peers_spec, collect_peers exec fn
- [ ] Prove bounded: result.len() <= max_peers for all inputs
- [ ] Prove self_excluded: self_id never appears in result
- [ ] Prove fill_monotonic: candidates added in order, no valid candidate skipped before cap
- [ ] Prove non_empty_guarantee: at least one non-self candidate → result.len() >= 1
- [ ] Prove no_duplicates: result has unique elements when input has unique elements
- [ ] Proptest: generate random candidate lists + self_id + max_peers, check bounds + exclusion
- [ ] Verus + tracey clean

## Phase 3: Node Classification

- [ ] Implement src/node_classify.rs: classify_node spec+exec (single node)
- [ ] Implement classify_nodes spec+exec (batch with bounds)
- [ ] Prove voter_iff_member: is_voter ⟺ node_id ∈ member_ids
- [ ] Prove learner_iff_not_voter: is_learner ⟺ ¬is_voter
- [ ] Prove voter_learner_exclusive: is_voter ⊕ is_learner (exactly one true)
- [ ] Prove leader_implies_voter: is_leader → is_voter
- [ ] Prove at_most_one_leader: count of is_leader across all nodes <= 1
- [ ] Prove bounded_output: classify_nodes result.len() <= MAX_NODES
- [ ] Prove no_duplicate_ids: result has unique node_ids
- [ ] Proptest: generate random member sets + leader + node lists, check classification
- [ ] Verus + tracey clean

## Phase 4: Cross-Module Integration

- [ ] voter_set::promote → quorum::is_quorum (promotion preserves quorum)
- [ ] voter_set::promote → quorum::can_remove_safely (replacement guard)
- [ ] node_classify::at_most_one_leader → quorum::majority (leader uniqueness per term, documented link)
- [ ] Property tests for cross-module relationships
- [ ] Verus + tracey clean
