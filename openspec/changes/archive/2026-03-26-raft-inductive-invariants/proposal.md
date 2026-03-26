## Why

The library has 13 verified Raft modules covering leaf-level primitives:
log operations, conflict resolution, matchIndex tracking, commit
advancement, ballot voting, election state machines, term authority,
leader append-only, leader completeness, state machine safety, pre-vote,
read index, joint consensus, and install snapshot. Each module proves
its own spec/exec/proof functions correct in isolation.

What's missing is the connective tissue. The Verdi Raft proof (Woos,
Wilcox et al., CPP 2016) needed 90 system invariants and 50,000 lines
of Coq to prove state machine safety → linearizability. Their core
insight: you cannot prove these properties hold in every reachable state
without (1) ghost state that tracks history — votes cast, leader log
snapshots, entry provenance — and (2) an inductive step decomposition
that factors the proof across handler functions.

Our `leader_completeness.rs` proves that *if* a shared voter exists with
the right epoch, *then* the new leader has the committed entry. But the
preconditions are taken as `requires` clauses — nothing proves those
preconditions actually hold in every reachable state. The Verdi proof
fills this gap with ghost variables (`votes`, `cronies`, `leaderLogs`,
`allEntries`) that accumulate history and enable inductive arguments.

Similarly, our `state_machine_safety.rs` proves that committed entries
agree when logs match on a prefix. But the premise "logs match on a
prefix" requires the Log Matching Property to hold inductively through
every AppendEntries exchange and conflict resolution. That inductive
argument is absent.

The paper also identifies three proof architecture patterns that reduce
rework:
- **Intermediate reachability**: factor handlers into sub-steps, prove
  each preserves invariants independently, compose.
- **Affinity lemmas**: any property of the main log propagates to log
  copies in ghost state and message payloads.
- **Trace relations**: connect internal state invariants to client-visible
  linearizability.

Without these, the existing leaf proofs are islands. They verify
individual functions but don't compose into a system-level guarantee.

## What Changes

Six new modules and one new spec domain covering the inductive invariant
infrastructure for Raft safety:

**Ghost State Infrastructure:**

- **`ghost_history`** — Ghost variable specs for vote history, leader log
  snapshots, and entry provenance. Pure spec types that don't affect
  runtime code. These are the `votes`, `cronies`, `leaderLogs`, and
  `allEntries` from the Verdi proof, adapted to Verus spec types.

- **`election_safety_proof`** — System-level election safety: at most
  one leader per term. Chains single-vote-per-term (ballot) + quorum
  intersection (quorum) + vote history (ghost_history) into a
  contradiction proof.

**Inductive Step Decomposition:**

- **`raft_step`** — Intermediate reachability predicate. Defines a
  `RaftState` spec struct and an `invariant_preserved_by` spec that
  factors the proof obligation across handler sub-steps. If each of
  {handle_message_specific, leader_heartbeat, execute_entries} preserves
  an invariant, the full handler preserves it.

- **`log_invariant_inductive`** — Inductive proof that the Log Matching
  Property holds through every state transition. Connects
  append_entries_check (consistency guard) + log_conflict (resolution) +
  leader_append_only (leader logs grow monotonically) to show that
  `logs_match_prefix` holds in every reachable state.

**Trace Relations and Linearizability:**

- **`trace_relation`** — Trace relation framework. Defines the
  "if trace property T becomes true, state property R is guaranteed"
  pattern from the Verdi proof. Inverse trace relations for the
  opposite direction.

- **`linearizability`** — Linearizability spec. Defines the
  linearizes-to relation (Definition 1 from the paper) and states the
  top-level theorem: state machine safety implies linearizability.

## Capabilities

### New Capabilities
- `ghost-vote-history`: Spec-level vote history tracking for inductive proofs
- `ghost-leader-logs`: Spec-level leader log snapshots at election time
- `ghost-entry-provenance`: Spec-level tracking of entry origin
- `election-safety-system`: System-level one-leader-per-term proof
- `raft-step-decomposition`: Handler decomposition for modular invariant proofs
- `log-matching-inductive`: Inductive proof of Log Matching Property
- `trace-relation-framework`: Generic trace-to-state proof pattern
- `linearizability-spec`: Formal linearizability definition and top-level theorem

### Modified Capabilities
- `leader-completeness`: Close the precondition gap with ghost state
- `state-machine-safety`: Connect to inductive Log Matching proof
- `log-conflict-resolution`: Close resolve_conflict external_body

## Impact

- **Files**: 6 new src/ modules, 1 new spec domain (raft-invariants),
  src/lib.rs additions, docs/requirements.md additions
- **APIs**: ~40 new spec/proof functions across 6 modules (mostly
  ghost/spec — minimal runtime surface)
- **Dependencies**: None — uses existing library modules and vstd
- **Testing**: Verus verification + proptest model checking of invariant
  predicates against simulated Raft traces
