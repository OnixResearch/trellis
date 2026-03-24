## Why

aspen-coordination has a `verified/` module with ~9,500 lines of pure
functions implementing distributed coordination primitives: leader
election state machines, distributed locks, sequence generators, barriers,
read-write locks, counters, semaphores, queues, and cross-primitive
fencing. These functions follow Tiger Style (saturating arithmetic,
explicit types, no I/O, time as parameter) and have unit tests. But the
correctness arguments are informal — comments, naming conventions, and
separate verus spec files that use `external_body` everywhere.

verified-logic already has building blocks these primitives rest on:
`fencing` (token issuance/staleness), `rate_limit` (token bucket),
`expiration` (TTL/is_expired), `lease` (grant/renew/revoke), `backoff`
(exponential with cap), `quorum` (majority threshold), `cas` (CAS
conditions), `two_phase` (unanimity/abort safety), `drain` (admission
guards), `circuit_breaker` (three-state FSM). What's missing is the
coordination-specific logic that composes these into leader election
transitions, distributed lock protocols, rwlock mutual exclusion, sequence
monotonicity, barrier phase machines, and cross-primitive fencing
validation.

These are the functions where bugs cause split-brain, lost locks, duplicate
IDs, and deadlocks. The election state machine alone has 7 transitions and
5 preconditions — a wrong guard means two leaders. The rwlock module has a
mutual exclusion invariant that's asserted in comments but never proved.
Formal verification pays for itself here.

## What Changes

Four new modules covering the core coordination decision logic extracted
from aspen-coordination's verified/. Each module is self-contained with
Verus spec+exec functions, proofs, and property tests.

- **`election`** — Leader election state machine. Three states
  (Follower/Transitioning/Leader), five transition preconditions, token
  progression, renewal timing, step-down logic. Properties: only valid
  transitions succeed, token monotonicity on win, well-formedness invariant,
  step-down always reaches Follower.

- **`distributed_lock`** — Distributed lock protocol logic. Token
  progression (always >= 1, monotonic), deadline computation, expiry checks,
  backoff with jitter for retry. Composes existing `fencing`, `expiration`,
  and `backoff` modules. Properties: token never zero, token monotonic,
  expired locks detected, backoff bounded by max.

- **`sequence`** — Monotonic ID generation with batch reservation. Batch
  state (remaining count, refill decision), range computation, overflow
  protection, CAS expected-value derivation. Properties: sequence values
  never decrease, batch_end >= next after refill, overflow returns None.

- **`rwlock`** — Read-write lock mode transitions and mutual exclusion.
  Three modes (Free/Read/Write), reader/writer expiry, acquisition
  preconditions, mode transitions, fencing tokens for writers. Properties:
  mutual exclusion (never Read+Write simultaneously), reader count
  monotonicity on acquire/release, mode transition validity, writer token
  monotonic.

## Capabilities

### New Capabilities
- `election-state-machine`: Verified leader election with transition guards
  and token progression proofs
- `distributed-lock-protocol`: Verified lock token monotonicity and expiry
  composing fencing + expiration + backoff
- `sequence-monotonicity`: Verified ID generation that never goes backwards,
  batch reservation with overflow protection
- `rwlock-mutual-exclusion`: Verified reader/writer mode transitions with
  mutual exclusion proof

### Modified Capabilities
- `cross-module-composition`: Properties linking election to existing
  `fencing::issue_token`, lock to `expiration::is_expired` +
  `backoff::compute_backoff`, rwlock to `fencing::is_valid_write`

## Impact

- **Files**: 4 new src/ modules, src/lib.rs additions, docs/requirements.md
  additions, src/properties.rs cross-module properties
- **APIs**: ~45 new pub fns across 4 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests + model oracles
  against aspen-coordination's existing unit tests as reference
