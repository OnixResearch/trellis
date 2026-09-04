## Why

Trellis already verifies majority quorum arithmetic, Raft log and execution order, Paxos promises, view-number monotonicity, no-op commitment, client-session deduplication, object-history linearizability, and a bounded QuePaxa fast path. It does not yet provide the distinct proof obligations needed when a client-visible fast path runs beside an existing consensus protocol.

The Jetpack OSDI 2026 paper identifies the central gap: a fast acknowledgement made in one stable view can become unsafe after proposer replacement unless the command remains recoverable and is ordered before conflicting new-view work. Its design also depends on stronger superquorum intersection, same-view certificates, receive-to-propose-to-execute ordering, a sound application conflict contract, and at-most-once convergence between fast and original paths.

These are reusable Trellis proof primitives. They should be proved independently of the Jetpack runtime and exposed with explicit assumptions and non-claims rather than treating external TLA+ checks or benchmarks as transferred proof.

## What Changes

- Add finite crash-fault fast-quorum arithmetic and actual certificate-intersection proofs for `n = 2f + 1` replicas.
- Prove the recovery-support threshold, its sharpness, and the reduced fast-path failure envelope for three-replica and five-replica profiles.
- Add same-command, same-acceleration-view, same-base-view certificates with unique replica acknowledgements and complete active-proposer promises.
- Prove the receive-order → proposal-order → execution-order compatibility chain and reject unsupported buffering or execution reordering.
- Add a generic state-and-response commutativity contract, conservative conflict admission, and a concrete finite key/value refinement example.
- Add recovery-set, accepted-set carry-forward, recovery/no-op marker, and recovery-before-new-view order proofs.
- Compose canonical operation identity with existing client-session logic to prove fast/original convergence cannot apply one operation twice.
- Prove a bounded two-operation linearization theorem over `consistency_history` and `consistency_relations`, while retaining explicit whole-system non-claims.

## Impact

- **Files**: a `consensus_fast_path` module family, `src/lib.rs`, `src/properties.rs` or `src/consensus_path.rs`, `docs/requirements.md`, `cairn/specs/consensus/spec.md`, README/module catalogs, proof-evidence manifests, and this Cairn package.
- **Testing**: positive and negative certificate, quorum, ordering, conflict, recovery, deduplication, and history fixtures; property tests; direct Verus proof functions; Tracey coverage; proof-evidence export.
- **Safety**: the change proves finite local arithmetic, transition, refinement, and composition lemmas under named assumptions. It does not prove the external Jetpack implementation, arbitrary application conflict classifiers, network liveness, storage durability, live Raft, dynamic membership, Byzantine faults, interactive transactions, latency, throughput, or production readiness.
