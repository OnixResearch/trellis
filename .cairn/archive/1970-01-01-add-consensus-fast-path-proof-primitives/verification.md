# Consensus Fast-Path Proof Verification

## Result

The finite crash-fault proof component is implemented and validated. Cairn
synced the accepted consensus specification and archived the completed change
as `1970-01-01-add-consensus-fast-path-proof-primitives`.

The component exposes pure arithmetic, certificate admission, ordering,
conflict, recovery, deduplication, and bounded-history composition primitives.

The component has seven top-level source files. Each production file has fewer
than 300 lines. The component reads no files, environment variables, clocks,
networks, processes, or persistent state.

## Checked boundary

The initial scope admits only static membership, crash faults, and one-command
operations. Byzantine faults, dynamic membership, interactive transactions,
and cross-group atomicity reject.

The Jetpack paper and `stonysystems/jetpack` revision
`c03e318ec355b11edd42aac56c68d0765f88d1d2` remain external references. Their
TLA+, benchmark, implementation, network, storage, liveness, and performance
claims do not transfer to Trellis.

## Implemented facts

The implementation provides:

- overflow-safe `n = 2f + 1`, majority, accelerated quorum, recovery support,
  and failure-tolerance helpers;
- exact finite certificate size, unique in-range replica identities, one
  command, one acceleration view, one base view, equal cross-path views, current
  view binding, and complete unique proposer coverage;
- actual overlap measurement against a unique, in-range recovery majority;
- receive, proposal, log, and execution order composition;
- a generic state-and-response commutativity law and a key/value refinement;
- canonical key ordering, disjoint-write commutativity, same-key conflict, and
  fail-closed unsupported effects;
- bounded possible and accepted recovery sets, accepted-set carry-forward,
  explicit empty markers, marker-before-normal admission, and recovered-order
  checks;
- exact session, sequence, and command identity with a retention horizon and
  existing `client_session` duplicate and cached-response checks; and
- a bounded composition checker over `consistency_history::HistoryOp` and
  `consistency_relations::realtime_order`.

## Proof portfolio

Seventeen direct `proof fn` artifacts cover:

- quotient and half-partition arithmetic;
- quorum bounds and failure tolerance;
- quorum/recovery intersection and threshold sharpness;
- three-replica and five-replica corollaries;
- conflicting same-view certificate impossibility from finite overlap and the
  no-double-ack rule;
- receive-to-proposal-to-log-to-execution composition;
- state and both response equalities under the commutativity contract;
- accepted-set carry-forward and marker gating;
- retained duplicate classification; and
- bounded real-time order and unsupported-scope rejection.

The focused Verus run reports 20 verified obligations and zero errors. The full
library metric reports 11,012 verified obligations.

## Runtime and property evidence

Twenty-nine focused tests pass. They include five randomized properties and
positive and negative cases for:

- three-node and five-node formulas;
- arbitrary rotated finite membership intersections;
- one-fewer-acknowledgement rejection;
- duplicate and out-of-range identities;
- mixed command and view hazards;
- proposer omissions and duplicates;
- base reorder hazards;
- disjoint writes, same-key conflicts, unsupported effects, and response drift;
- interrupted, empty, carried-forward, and stale-predecessor recovery;
- retained, expired, and mismatched duplicate records;
- stable-view and recovered-view history composition; and
- unsupported capability scopes.

## Final evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 29 passed |
| Focused Verus | 20 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 11,012 proofs; 5,147 tests; 2,215 requirements; 620 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,215 |
| Tracey untested | 0 of 2,215 |
| Locked Octet direct check | 3,867 accepted; 0 new; 0 stale |
| Focused Nix Octet check | passed |
| Proof evidence bundle | 4 records validate |
| Proof evidence IR chain | 4 records validate |
| Proof artifact manifest | 5 primitives and 1 Preserves envelope validate |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures pass locally |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |
| Cairn accepted-spec sync | passed |
| Cairn archive execution | passed |

Evidence is under `evidence/final-2026-09-03/` and `evidence/octet/`.

The first wrapper run selected Cargo 1.76 from the Rust-script shell. The second
run used Cargo 1.96 but could not discover its sibling Clippy binary. A later
run found an unusable unpacked Verus path after the broad Nix attempt.

All failure transcripts are retained. The final run bound the modern Cargo
path, kept Bash discoverable, and resolved the repository's pinned Verus
package. It passed every wrapper check. `scripts/verus.sh` now normalizes the
Verus library root, and the proof-example rail no longer scans arbitrary Nix
store paths.

## Broad Nix observation

The focused Nix Octet check passes. The broad `nix flake check` remains blocked
by two repository-wide conditions:

1. the formatter check reports pre-existing rustfmt drift outside this change;
2. the immutable `trellis-consumer-release-src` input still points to revision
   `1f03111b92cc36aade07359c96773e086908c95f`, so its measured crate root and
   proof manifest do not match the evolving local profile.

The first Nix attempt also wrote its log inside the source tree and triggered a
source-mutation hash guard. That transcript is retained, and the rerun wrote to
`/tmp` before copying the completed log into evidence.

These broad failures do not replace or weaken the passing locked Octet gate.
They do not authorize a format baseline, warning budget, finding suppression,
consumer-pin claim, or manual `flake.lock` edit.

## Octet ratchet

The first check reported 26 new findings. The component was reorganized into
seven top-level files, and all 25 component findings were removed. The final
remaining change was the existing `src/lib.rs` file-length identity.

The guarded migration replaced `b3:8615a1197127a747` with
`b3:9eb4e48841031554`. It preserved all other accepted records and admitted no
finding under `src/consensus_fast_path/`.

## Generated artifacts

The change refreshes:

- `docs/module-index.md`;
- `docs/invariant-surface-catalog.md`;
- `docs/property-spec-index.md`;
- `docs/proof-artifact-manifest.json` and its Preserves carrier;
- `docs/proof-evidence-bundle.json`;
- `docs/proof-evidence-ir-chain.json`; and
- `consumer-profile/generated/profile.json` from its Nickel source.

## Non-claims

This evidence does not prove:

- Jetpack or another consensus implementation;
- arbitrary application conflict classifiers;
- live network, storage, clock, failure-detector, or process behavior;
- liveness, latency, throughput, or production readiness;
- dynamic membership, Byzantine tolerance, interactive transactions, or
  cross-group atomicity;
- whole-system linearizability; or
- verifier soundness.
