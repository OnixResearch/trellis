## Why

Trellis already has verified Paxos, Raft, quorum, and distributed-systems building blocks, but it does not yet model the QuePaxa ideas from Kokoris-Kogias et al., “QuePaxa: Escaping the Tyranny of Timeouts in Consensus” (SOSP 2023, https://bford.info/pub/os/quepaxa/quepaxa.pdf). QuePaxa is interesting for Trellis because it separates timeout-free asynchronous safety machinery from implementation-dependent performance and liveness claims: interval summary registers, prioritized proposals, quorum propagation, phase/round clocks, fast-path admission, and hedging schedules can all be represented as finite deterministic verified-logic primitives.

This change adds a bounded QuePaxa primitive slice that captures those local invariants without claiming a production SMR implementation, real network liveness, cryptographic randomness, multi-armed-bandit optimality, or the paper’s benchmark results.

## What Changes

- Add QuePaxa proposal and priority-order primitives over finite proposer/value metadata.
- Add interval summary register (ISR) primitives that model monotone logical steps, first-value capture, max aggregation, stale-step discard, and skipped-step nil aggregation.
- Add abstract quorum/tcast-style propagation predicates that expose the existent/common/universal set relationship used by the paper’s safety argument.
- Add abstract-round decision predicates for validity, agreement, integrity, and next-candidate carry-forward in the finite modeled state.
- Add concrete phase/round admission helpers for the four-phase proposer path, catch-up behavior, asynchronous phase-2 decision guard, and first-round leader fast path.
- Add finite hedging-schedule admission for non-destructive proposer activation ordering, while keeping clocks, real delays, and adaptive MAB tuning out of the verified claim.
- Add positive and negative tests plus proof lemmas, Tracey anchors, README/module-index updates, and synced requirements.

## Impact

- **Files**: expected source in a new `src/quepaxa.rs` module or a small `src/quepaxa_*` family, `src/lib.rs`, `docs/requirements.md`, `cairn/specs/consensus/spec.md`, README/module catalog docs, and this Cairn package.
- **APIs**: finite verified helper surfaces only; no network server, storage engine, random-number generator, SMR runtime, or production QuePaxa compatibility layer.
- **Testing**: positive and negative Rust tests for ISR transitions, priority/tie policy, quorum propagation predicates, decision guards, catch-up, fast-path rejection, and hedging schedules; Verus proof checks; Tracey coverage; Cairn validation/gates.
