## Phase 1: Election Jitter + Raft Timing

- [ ] Add cluster-coordination requirements to docs/requirements.md
- [ ] Implement src/election_jitter.rs: jitter_spec, compute_jitter exec, boundedness proof
- [ ] Prove jitter <= jitter_range for all node_id values
- [ ] Prove zero-range yields zero jitter
- [ ] Prove determinism (same inputs → same output, trivial but document)
- [ ] Implement src/raft_timing.rs: validate_heartbeat, validate_election_range, validate_order
- [ ] Implement validate_all with fail-fast composition
- [ ] Prove: validate_all Ok implies each individual constraint holds
- [ ] Prove: ordering failure when max <= min
- [ ] Unit tests + proptest for both modules
- [ ] Verus + tracey clean

## Phase 2: Trust Level + Peer Spec

- [ ] Implement src/trust_level.rs: TrustLevel enum, resolve_trust spec+exec
- [ ] Implement can_access spec+exec
- [ ] Prove block precedence: in_blocked → result is Blocked
- [ ] Prove access control matrix: Blocked→deny, Trusted→allow, Public→conditional
- [ ] Implement capacity predicates: trusted_bounded, blocked_bounded, pending_bounded
- [ ] Prove add_trusted_maintains_bound, block_maintains_bound
- [ ] Prove mutual exclusivity: add_trusted removes block, block removes trust
- [ ] Implement src/peer_spec.rs: find_separator, parse_peer_spec spec+exec
- [ ] Implement is_json_endpoint spec+exec
- [ ] Prove error completeness: every invalid input maps to exactly one error variant
- [ ] Prove roundtrip: format then parse yields original values
- [ ] Unit tests + proptest for both modules
- [ ] Verus + tracey clean

## Phase 3: Tiered Rate Limit

- [ ] Implement src/tiered_rate_limit.rs: TieredResult enum, check_tiered spec+exec
- [ ] Prove global-first ordering: global denied → result is GlobalDenied without per-peer eval
- [ ] Implement peer_map_bounded predicate
- [ ] Prove capacity invariant: after eviction, count < max
- [ ] Prove global_dominates: global_rate > per_peer_rate as precondition
- [ ] Implement evict_reduces_count spec+proof
- [ ] Cross-module: link check_tiered to existing rate_limit::can_consume
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 4: Drain Protocol

- [ ] Implement src/drain_protocol.rs: try_start_with_recheck spec+exec
- [ ] Implement finish_with_notify spec+exec
- [ ] Implement is_fully_drained spec+exec
- [ ] Prove double-check: if recheck finds draining, count is decremented
- [ ] Prove notification condition: should_notify iff in_flight==0 && is_draining
- [ ] Prove drain completeness: is_draining && in_flight==0 → is_fully_drained
- [ ] Cross-module: link to existing drain::can_start_op_spec
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 5: Node Lifecycle

- [ ] Implement src/node_lifecycle.rs: NodeStatus enum, is_valid_transition spec+exec
- [ ] Implement is_operational spec+exec
- [ ] Prove terminal: Offline has no valid outgoing transitions
- [ ] Prove reachability: Offline reachable from all non-Offline states
- [ ] Prove operational correctness: Online and Degraded only
- [ ] Prove transition exhaustiveness: every (from, to) pair classified
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 6: Cross-Module Properties + Integration

- [ ] election_jitter bounded → backoff bounded (both use 2^k with cap pattern)
- [ ] tiered_rate_limit::check_tiered → rate_limit::can_consume (shared predicate)
- [ ] drain_protocol::try_start → drain::can_start_op (refinement)
- [ ] node_lifecycle::is_operational → pressure::classify_level (operational + pressure = scheduling)
- [ ] trust_level::can_access exhaustive over all TrustLevel × is_public combinations
- [ ] Property tests for all cross-module relationships
- [ ] Verus + tracey clean
