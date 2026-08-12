## Context

aspen-cluster's `verified/` module has four submodules of pure functions:

- `gossip.rs` — topic ID derivation (blake3), backoff calculation
- `rate_limiter.rs` — token replenishment (f64), consumption, LRU eviction
- `peer.rs` — `node_id@endpoint_spec` parsing
- `message.rs` — size/version comparison

The crate also has non-verified pure logic scattered across:

- `validation.rs` — Raft timing, cookie, secret key validators
- `upgrade/types.rs` — DrainState with try_start_op double-check
- `upgrade/drain.rs` — drain execution and reset
- `gossip/rate_limiter.rs` — two-tier GossipRateLimiter (per-peer + global)
- `gossip/constants.rs` — compile-time assertions on rate limits
- `memory_watcher.rs` — pressure level classification
- `metadata.rs` — NodeStatus enum (Starting/Online/Degraded/Offline)
- `lib.rs` tests — election jitter via Knuth multiplicative hash
- `verus/trust_spec.rs` — trust manager specs (no exec functions)

verified-logic already has overlapping foundations: `rate_limit` (integer
token bucket), `drain` (admission predicates), `pressure` (threshold
classification), `backoff` (exponential with cap), `gossip` (infection
bounds), `process_state` (Unix process FSM), `circuit_breaker` (three-state).

The gap is the cluster-specific composition: how these pieces combine
into election timing, tiered rate limiting, trust decisions, and the
concurrent drain protocol.

## Goals / Non-Goals

**Goals:**
- Verified election jitter with boundedness and determinism proofs
- Verified Raft timing validators that compose with fail-fast ordering
- Verified two-tier rate limit decision with capacity invariant
- Verified trust level precedence with access control proofs
- Verified peer spec parsing with roundtrip and error completeness
- Verified drain double-check extending the existing drain module
- Verified node lifecycle state machine with transition guards
- All modules follow existing pattern: spec fn + exec fn + proof fn + tests
- Cross-module properties linking to existing library

**Non-Goals:**
- Reimplementing aspen-cluster's imperative shell (async, I/O, networking)
- Verifying blake3 hashing or Ed25519 signatures (crypto axioms)
- Full gossip protocol state machine (protocol-level, not a primitive)
- Verifying f64 token bucket arithmetic (verified-logic uses integer
  millitokens; the f64 version in aspen-cluster stays as-is)
- Liveness proofs requiring timing models

## Decisions

### 1. Election jitter as integer hash, not floating point

**Choice:** Model jitter as `node_id.wrapping_mul(PHI_KNUTH) % jitter_range`
where `jitter_range = (max - min) / 3`. Use u64 throughout.

**Rationale:** This mirrors the exact computation in aspen-cluster's
`compute_election_jitter`. The Knuth multiplicative hash (2,654,435,761)
produces good distribution for small sequential node IDs. Verus can reason
about modular arithmetic on u64.

**Alternative:** Use a more sophisticated hash. Rejected — the existing
hash works and is deployed. We verify what's running, not a theoretical
improvement.

### 2. Raft timing as composable validators returning Result

**Choice:** Individual validator functions (`validate_heartbeat`,
`validate_election_range`, `validate_timing_order`) each return
`Result<(), u8>` where the error code identifies the failure. A
`validate_all` composes them with fail-fast semantics.

**Rationale:** Matches aspen-cluster's `validation.rs` pattern. The error
code approach avoids string allocation in verified code while preserving
failure identification. Composition proof: if `validate_all` succeeds,
each individual invariant holds.

**Alternative:** Return a bitfield of failures. Rejected — fail-fast is
the deployed behavior and easier to verify (short-circuit evaluation).

### 3. Tiered rate limit as two separate decision functions

**Choice:** `check_global(global_tokens) -> bool` and
`check_per_peer(peer_tokens) -> bool` as independent functions, plus
`check_tiered(global_tokens, peer_tokens) -> TieredResult` that
composes them with global-first ordering. Capacity invariant as a
separate predicate `peer_map_bounded(len, max)`.

**Rationale:** Separating the decision from the data structure lets us
verify the logic without modeling HashMap. The composition function
proves that global-first ordering means a global denial skips the
per-peer check (cheaper path). The capacity invariant proves that
after eviction, the map stays within bounds.

**Alternative:** Model the full HashMap + LRU. Rejected — Verus doesn't
have HashMap support, and the decision logic is the interesting part.

### 4. Trust level as enum with explicit precedence function

**Choice:** `TrustLevel` enum (Blocked/Trusted/Public) with
`resolve_trust(in_blocked, in_trusted) -> TrustLevel` that checks
blocked first. Access control as `can_access(level, is_public) -> bool`.

**Rationale:** Matches the trust_spec.rs pattern but as verified exec
functions instead of external_body specs. The precedence rule (blocked
overrides trusted) is the critical safety property — a bug here means
a blocked cluster gets access.

### 5. Peer spec parsing as index-based, not regex

**Choice:** Find first `@` by index scan, split, validate parts. Return
a result enum with specific error variants.

**Rationale:** Verus can reason about character-level string operations.
The roundtrip property (`format → parse → same values`) is provable
with index arithmetic. Regex is opaque to verification.

### 6. Drain protocol extends existing drain module

**Choice:** Add `try_start_with_recheck` and `finish_with_notify` to
a new `drain_protocol` module that imports specs from `drain`. The
double-check invariant is: if `try_start_with_recheck` returns true,
then at the moment of the second check, draining was false.

**Rationale:** The existing `drain` module has the basic predicates.
The protocol module adds the recheck step that prevents the race
condition in aspen-cluster's `DrainState::try_start_op`. Keeping them
separate avoids modifying proven code.

### 7. Node lifecycle distinct from process_state

**Choice:** New `node_lifecycle` module with `NodeStatus` enum
(Starting/Online/Degraded/Offline) and `is_valid_transition` predicate.
Does not reuse `process_state`.

**Rationale:** The transitions are different. Unix processes have
Created→Ready→Running→Blocked→Terminated. Cluster nodes have
Starting→Online→Degraded→Offline with bidirectional recovery
(Degraded→Online). Forcing them into the same model would require
awkward mapping.

### 8. Phased implementation matching dependency order

**Choice:** Build in this order:
1. election_jitter + raft_timing (independent, no cross-deps)
2. trust_level + peer_spec (independent, no cross-deps)
3. tiered_rate_limit (builds on existing rate_limit)
4. drain_protocol (builds on existing drain)
5. node_lifecycle (independent)
6. cross-module properties

**Rationale:** Phases 1-2 can run in parallel (no dependency). Phase 3-4
depend on existing modules. Phase 5 is standalone. Phase 6 ties
everything together.

## Risks / Trade-offs

**[Wrapping multiplication in Verus]** → `node_id.wrapping_mul(K)` uses
modular arithmetic. Verus supports this via `u64::wrapping_mul` but the
spec needs `(node_id * K) % 2^64`. Z3 handles modular arithmetic but
may be slow on large constants. Mitigation: use `assume` for the Knuth
constant's distribution properties and verify only the boundedness.

**[String operations in Verus for peer parsing]** → Verus has limited
string support. The `find('@')` operation needs modeling as an index
scan. Mitigation: work with `&[u8]` and byte-level operations, with an
axiom that the Rust `str::find` matches the byte scan.

**[Scope creep into protocol verification]** → The tiered rate limit
and drain protocol are close to protocol-level behavior. Guard rail:
no async, no channels, no time sources. If a function needs `Instant`
or `tokio`, it's out of scope. Time is always an explicit parameter.
