## Phase 1: Fencing Tokens

- [x] Add fencing requirements to docs/requirements.md (monotonic issuance, stale rejection, no false rejection) ✅
- [x] Implement src/fencing.rs: issue_token spec+exec, is_valid_write spec+exec, is_stale spec+exec ✅
- [x] Prove issuance_strictly_increasing: issue_token(t) > t ✅
- [x] Prove valid_write_not_stale: is_valid_write(t, fence) == !is_stale(t, fence) ✅
- [x] Prove no_false_rejection: issued token is always valid against the fence it was issued from ✅
- [x] Cross-module: fencing::is_stale ↔ monotonic::is_strict_advance ✅
- [x] Unit tests + proptest ✅
- [x] Verus + tracey clean ✅

## Phase 2: Ballot / Vote Tracking

- [ ] Add ballot requirements to docs/requirements.md (single-vote-per-term, election safety, log check)
- [ ] Implement src/ballot.rs: VoteState struct, can_grant_vote spec+exec, grant_vote spec+exec
- [ ] Implement has_won_election spec+exec using quorum::is_quorum
- [ ] Prove single_vote_per_term: grant_vote for same term returns same voted_for
- [ ] Prove election_safety: two quorum winners in same term must be same candidate
- [ ] Prove stale_candidate_rejected: lower epoch candidate is rejected
- [ ] Cross-module: ballot::election_safety → quorum::quorum_intersection
- [ ] Cross-module: ballot::can_grant_vote → epoch::epoch_ge (log comparison)
- [ ] Unit tests + proptest (random vote sequences, concurrent elections)
- [ ] Verus + tracey clean

## Phase 3: Vector Clocks

- [ ] Add vector_clock requirements to docs/requirements.md (partial order, merge semilattice, increment)
- [ ] Implement src/vector_clock.rs: VectorClock<N> struct with [u64; N] entries
- [ ] Implement happens_before spec+exec, concurrent spec+exec, merge spec+exec, increment spec+exec
- [ ] Prove partial_order: irreflexive, asymmetric, transitive for happens_before
- [ ] Prove merge_commutative: merge(a, b) == merge(b, a)
- [ ] Prove merge_associative: merge(merge(a, b), c) == merge(a, merge(b, c))
- [ ] Prove merge_idempotent: merge(v, v) == v
- [ ] Prove increment_advances: happens_before(v, increment(v, i))
- [ ] Cross-module: vector_clock::merge component ↔ ordering::max
- [ ] Unit tests + proptest (random clock pairs, concurrent detection)
- [ ] Verus + tracey clean

## Phase 4: CRDTs

- [ ] Add CRDT requirements to docs/requirements.md (G-Counter, PN-Counter, LWW-Register merge laws)
- [ ] Implement src/crdt.rs: GCounter<N> struct, merge spec+exec, value spec+exec, increment spec+exec
- [ ] Prove g_counter_merge_commutative, g_counter_merge_associative, g_counter_merge_idempotent
- [ ] Prove g_counter_value_monotonic: increment increases value by 1
- [ ] Prove g_counter_merge_dominates: value(merge(a, b)) >= value(a)
- [ ] Implement PNCounter<N>: increment spec+exec, decrement spec+exec, merge spec+exec, value spec+exec
- [ ] Prove pn_counter_merge_laws (commutativity, associativity, idempotency)
- [ ] Prove pn_counter_increment_decrement_inverse
- [ ] Implement LWWRegister: merge spec+exec based on timestamp comparison
- [ ] Prove lww_merge_commutative, lww_merge_associative, lww_merge_idempotent
- [ ] Prove lww_higher_timestamp_wins
- [ ] Unit tests + proptest for all three types
- [ ] Verus + tracey clean

## Phase 5: Lease Disjointness

- [ ] Add lease requirements to docs/requirements.md (disjointness, renewal, revocation)
- [ ] Implement src/lease.rs: Lease struct, is_valid spec+exec, grant spec+exec, renew spec+exec, revoke spec+exec
- [ ] Prove lease_disjointness: sequential leases cannot both be valid at any time t
- [ ] Prove renewal_continuous: renew extends validity without gap
- [ ] Prove revocation_immediate: revoked lease is invalid at and after revocation time
- [ ] Cross-module: lease::is_valid → interval::contains, lease::disjoint → interval::overlaps
- [ ] Unit tests + proptest (random lease sequences, concurrent validity checks)
- [ ] Verus + tracey clean

## Phase 6: Two-Phase Commit

- [ ] Add two_phase requirements to docs/requirements.md (unanimity, abort safety, decision uniqueness)
- [ ] Implement src/two_phase.rs: Decision enum (int-encoded), can_commit spec+exec, decide spec+exec
- [ ] Prove commit_requires_unanimity: can_commit ==> all votes are Some(true)
- [ ] Prove abort_always_safe: abort never violates safety
- [ ] Prove decision_uniqueness: can_commit and must_abort are mutually exclusive
- [ ] Cross-module: two_phase::can_commit ↔ collections::find_first (first no-vote)
- [ ] Unit tests + proptest (random vote arrays, edge cases with None)
- [ ] Verus + tracey clean

## Phase 7: Circuit Breaker

- [ ] Add circuit_breaker requirements to docs/requirements.md (transitions, open blocks, half-open probe)
- [ ] Implement src/circuit_breaker.rs: state constants, next_state spec+exec, allow_request spec+exec
- [ ] Prove valid_state_output: next_state always returns CLOSED, OPEN, or HALF_OPEN
- [ ] Prove transition_legality: only 4 valid transitions can occur
- [ ] Prove open_blocks_all: state == OPEN ==> allow_request == false
- [ ] Prove half_open_admits_one: state == HALF_OPEN && !probe_active ==> allow_request == true
- [ ] Unit tests + proptest (state machine walk, random transition sequences)
- [ ] Verus + tracey clean

## Phase 8: Consistent Hashing

- [ ] Add consistent_hash requirements to docs/requirements.md (complete coverage, minimal disruption)
- [ ] Implement src/consistent_hash.rs: lookup spec+exec over sorted ring array
- [ ] Prove complete_coverage: non-empty ring ==> lookup always returns a valid node
- [ ] Prove minimal_disruption_add: adding a node changes at most one key's assignment
- [ ] Prove minimal_disruption_remove: removing a node reassigns keys to successor only
- [ ] Cross-module: consistent_hash lookup → collections::binary_search
- [ ] Unit tests + proptest (random rings, add/remove stability)
- [ ] Verus + tracey clean

## Phase 9: Backoff

- [ ] Add backoff requirements to docs/requirements.md (exponential growth, cap, jitter bounds)
- [ ] Implement src/backoff.rs: compute_backoff spec+exec, jitter_range spec+exec
- [ ] Prove growth_bounded: compute_backoff <= cap for all inputs
- [ ] Prove no_overflow: compute_backoff never wraps (uses saturating shift)
- [ ] Prove jitter_in_range: jitter_range returns [0, backoff]
- [ ] Cross-module: backoff exponent ↔ integer::next_power_of_two
- [ ] Unit tests + proptest (edge cases at u64 boundary, attempt=63/64)
- [ ] Verus + tracey clean

## Phase 10: Bloom Filter

- [ ] Add bloom requirements to docs/requirements.md (no false negatives, monotonic membership)
- [ ] Implement src/bloom.rs: BloomFilter struct with bitmap, insert spec+exec, query spec+exec
- [ ] Prove no_false_negatives: insert(x) ==> query(x) == true
- [ ] Prove monotonic_membership: insert(y) preserves query(x) for previously inserted x
- [ ] Cross-module: bloom bit operations → bitmap::get, bitmap::set
- [ ] Unit tests + proptest (random insert/query sequences)
- [ ] Verus + tracey clean

## Phase 11: Merkle Proofs

- [ ] Add merkle requirements to docs/requirements.md (inclusion verification, proof length)
- [ ] Implement src/merkle.rs: verify_proof spec+exec using chain module hash axiom
- [ ] Prove valid_proof_accepted: correct leaf + correct siblings → root matches
- [ ] Prove proof_length: siblings.len() == ceil_log2(tree_size)
- [ ] Cross-module: merkle hash → chain::hash_spec, proof_length → integer::log2
- [ ] Unit tests + proptest (random trees, tampered proofs)
- [ ] Verus + tracey clean

## Phase 12: Gossip Bounds

- [ ] Add gossip requirements to docs/requirements.md (infection lower bound, convergence)
- [ ] Implement src/gossip.rs: infection_lower_bound spec+exec, convergence_rounds spec+exec
- [ ] Prove infection_monotonic: more rounds ==> more infected (or saturated at n)
- [ ] Prove convergence_bound: ceil_log_f(n) rounds suffice
- [ ] Unit tests + proptest (various fanout/cluster sizes)
- [ ] Verus + tracey clean

## Phase 13: Causal Order and Snapshots

- [ ] Add causal_order requirements to docs/requirements.md (causal delivery, concurrent independence)
- [ ] Implement src/causal_order.rs: is_causally_ready spec+exec, can_deliver spec+exec
- [ ] Prove causal_delivery: happens_before(m1, m2) ==> m1 delivered before m2
- [ ] Cross-module: causal_order → vector_clock::happens_before
- [ ] Add snapshot requirements to docs/requirements.md (consistent cut, cut lattice)
- [ ] Implement src/snapshot.rs: is_consistent_cut spec+exec, cut_join spec+exec
- [ ] Prove consistent_cut_join: join of two consistent cuts is consistent
- [ ] Cross-module: snapshot cut → vector_clock (component-wise max)
- [ ] Unit tests + proptest for both modules
- [ ] Verus + tracey clean

## Phase 14: Cross-Module Properties

- [ ] vector_clock::merge component ↔ ordering::max (pointwise)
- [ ] ballot::winner → quorum::is_quorum (election safety)
- [ ] fencing::is_valid → monotonic::is_advance (token ordering)
- [ ] lease::disjoint → interval::overlaps (time intervals)
- [ ] two_phase::can_commit ↔ collections::find_first (first no-vote)
- [ ] backoff::compute ↔ integer::next_power_of_two (exponential)
- [ ] merkle::proof_length ↔ integer::log2 (tree depth)
- [ ] bloom::no_false_negatives → bitmap::get (bit membership)
- [ ] snapshot::cut_join → vector_clock::merge (lattice = merge)
- [ ] All property tests passing
- [ ] Verus + tracey clean
