## Why

The aspen-cluster crate has a `verified/` module containing pure functions
for gossip, rate limiting, peer parsing, and message validation. These
functions follow Tiger Style (saturating arithmetic, explicit types, no I/O)
and have unit tests plus bolero property tests. But they lack machine-checked
proofs. The correctness arguments live in comments and separate verus spec
files that use `external_body` everywhere — the proofs don't touch the
actual implementations.

Meanwhile, verified-logic already covers the mathematical foundations these
functions rely on: token bucket refill (`rate_limit`), drain admission
(`drain`), pressure classification (`pressure`), exponential backoff
(`backoff`), gossip infection bounds (`gossip`). What's missing is the
cluster-specific coordination logic that ties these together: election
jitter for Raft liveness, Raft timing validation, two-tier rate limit
decisions, trust manager state, peer spec parsing, the drain double-check
protocol, and node lifecycle state machines.

These are the functions where a bug causes real outages — election thundering
herd, split-brain from bad timeout ordering, rate limiter bypass, trust
escalation. Formal verification pays for itself here.

## What Changes

Seven new modules covering the pure decision logic extracted from
aspen-cluster. Each module is self-contained with spec fns, exec fns,
proofs, and property tests.

- **`election_jitter`** — Knuth multiplicative hash for deterministic
  per-node election timeout jitter. Properties: bounded by range/3,
  zero-range yields zero, deterministic.

- **`raft_timing`** — Validation of Raft timeout configuration:
  all timeouts positive, max > min for randomization, heartbeat < election
  timeout for correctness. Composable fail-fast validators.

- **`tiered_rate_limit`** — Two-tier rate limit decision logic: check
  global bucket first (cheaper), then per-peer. Capacity invariant for
  the peer map with LRU eviction. Global rate dominates per-peer rate.

- **`trust_level`** — Trust manager decision logic: block precedence
  (blocked overrides trusted), capacity bounds on all sets, mutual
  exclusivity after operations, access control matrix.

- **`peer_spec`** — Peer specification parsing: `node_id@endpoint_spec`
  format with first-@ splitting, empty-part rejection, u64 validation,
  roundtrip property.

- **`drain_protocol`** — The atomic double-check admission protocol:
  check draining → increment in-flight → recheck draining. Extends the
  existing `drain` module with the recheck invariant and notification
  condition (notify when count hits zero during drain).

- **`node_lifecycle`** — Node status state machine: Starting, Online,
  Degraded, Offline with valid transition guards. Different from
  `process_state` which models Unix process lifecycle.

## Capabilities

### New Capabilities
- `election-jitter`: Verified deterministic jitter with boundedness proof
- `raft-timing-validation`: Verified timeout ordering constraints
- `tiered-rate-limit`: Verified two-tier decision with capacity invariant
- `trust-level`: Verified trust precedence and access control
- `peer-spec-parsing`: Verified string parsing with roundtrip proof
- `drain-double-check`: Verified atomic admission protocol
- `node-lifecycle-fsm`: Verified node status transitions

### Modified Capabilities
- `cross-module-properties`: Properties linking new modules to existing
  `rate_limit`, `drain`, `pressure`, `backoff`, `process_state`

## Impact

- **Files**: 7 new src/ modules, src/lib.rs additions, docs/requirements.md
  additions, src/properties.rs additions
- **APIs**: ~40 new pub fns across 7 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests + model oracles
  against aspen-cluster's existing unit tests as reference
