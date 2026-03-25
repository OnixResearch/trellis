## Phase 1: Election Jitter + Raft Timing

- [x] Add cluster-coordination requirements to docs/requirements.md ✅
- [x] Implement src/election_jitter.rs: jitter_spec, compute_jitter exec, boundedness proof ✅
- [x] Prove jitter <= jitter_range for all node_id values ✅
- [x] Prove zero-range yields zero jitter ✅
- [x] Prove determinism (same inputs → same output, trivial but document) ✅
- [x] Implement src/raft_timing.rs: validate_heartbeat, validate_election_range, validate_order ✅
- [x] Implement validate_all with fail-fast composition ✅
- [x] Prove: validate_all Ok implies each individual constraint holds ✅
- [x] Prove: ordering failure when max <= min ✅
- [x] Unit tests + proptest for both modules ✅ (13 tests)
- [x] Verus + tracey clean ✅

## Phase 2: Trust Level + Peer Spec

- [x] Implement src/trust_level.rs: TrustLevel as u8 constants, resolve_trust spec+exec ✅
- [x] Implement can_access spec+exec ✅
- [x] Prove block precedence: in_blocked → result is Blocked ✅
- [x] Prove access control matrix: Blocked→deny, Trusted→allow, Public→conditional ✅
- [x] Implement capacity predicates: trusted_bounded, blocked_bounded, pending_bounded ✅
- [x] Prove add_trusted_maintains_bound, block_maintains_bound ✅
- [x] Prove mutual exclusivity: add_trusted removes block, block removes trust ✅
- [x] Implement src/peer_spec.rs: find_separator, parse_peer_spec spec+exec ✅
- [x] Implement is_all_digits spec (is_json_endpoint deferred — string ops too complex for Verus) ✅
- [x] Prove error completeness: every invalid input maps to exactly one error variant ✅
- [x] Prove roundtrip: format then parse yields original values ✅ (proptest)
- [x] Unit tests + proptest for both modules ✅ (15 tests)
- [x] Verus + tracey clean ✅

## Phase 3: Tiered Rate Limit

- [x] Implement src/tiered_rate_limit.rs: TieredResult as u8 constants, check_tiered spec+exec ✅
- [x] Prove global-first ordering: global denied → result is GlobalDenied without per-peer eval ✅
- [x] Implement peer_map_bounded predicate ✅
- [x] Prove capacity invariant: after eviction, count < max ✅
- [x] Prove global_dominates: zero cost always allowed ✅
- [x] Implement evict_reduces_count spec+proof ✅
- [x] Cross-module: link check_tiered to existing rate_limit::try_consume ✅
- [x] Unit tests + proptest ✅ (9 tests)
- [x] Verus + tracey clean ✅

## Phase 4: Drain Protocol

- [x] Implement src/drain_protocol.rs: try_start_with_recheck spec+exec ✅
- [x] Implement finish_with_notify spec+exec ✅
- [x] Implement is_fully_drained spec+exec ✅
- [x] Prove double-check: if recheck finds draining, count is decremented ✅
- [x] Prove notification condition: should_notify iff in_flight==0 && is_draining ✅
- [x] Prove drain completeness: is_draining && in_flight==0 → is_fully_drained ✅
- [x] Cross-module: link to existing drain::can_start_op_spec ✅
- [x] Unit tests + proptest ✅ (7 tests)
- [x] Verus + tracey clean ✅

## Phase 5: Node Lifecycle

- [x] Implement src/node_lifecycle.rs: NodeStatus as u8 constants, is_valid_transition spec+exec ✅
- [x] Implement is_operational spec+exec ✅
- [x] Prove terminal: Offline has no valid outgoing transitions ✅
- [x] Prove reachability: Offline reachable from all non-Offline states ✅
- [x] Prove operational correctness: Online and Degraded only ✅
- [x] Prove transition exhaustiveness: every (from, to) pair classified ✅
- [x] Unit tests + proptest ✅ (6 tests)
- [x] Verus + tracey clean ✅

## Phase 6: Cross-Module Properties + Integration

- [x] election_jitter bounded → backoff bounded (both use cap pattern) ✅
- [x] tiered_rate_limit::check_tiered → rate_limit::try_consume (shared predicate) ✅
- [x] drain_protocol::try_start → drain::can_start_op (refinement) ✅
- [x] node_lifecycle::is_operational → operational predicate ✅
- [x] trust_level::can_access exhaustive over all TrustLevel × is_public combinations ✅
- [x] Property tests for all cross-module relationships ✅ (5 tests)
- [x] Verus + tracey clean ✅
