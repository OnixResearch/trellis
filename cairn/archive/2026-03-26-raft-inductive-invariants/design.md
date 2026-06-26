## Context

The library has leaf-level Raft verification across 13+ modules. Each
module proves its functions correct against local specs. The proofs stop
at module boundaries — `leader_completeness.rs` takes the shared voter's
existence as a precondition, `state_machine_safety.rs` takes log prefix
matching as a precondition. The Verdi Raft proof (CPP 2016) shows that
closing these preconditions requires ghost state, an inductive step
decomposition, and ~90 invariants proved about every reachable state.

The library currently has:
- `quorum` (majority, intersection, fault tolerance)
- `ballot` (vote granting, single-vote-per-term, election safety)
- `election` (state machine, fencing tokens, timing)
- `epoch` (lexicographic comparison)
- `log` (agree_at, prefix matching, append, truncate)
- `log_conflict` (find_conflict verified, resolve_conflict external_body)
- `match_index` (per-follower tracking with monotonicity)
- `commit_advance` (§5.4.2 current-term guard)
- `term_authority` (step-down on higher term)
- `leader_append_only` (leader log immutability)
- `leader_completeness` (quorum overlap + epoch transitivity)
- `state_machine_safety` (committed entries agree)
- `pre_vote`, `read_index`, `joint_consensus`, `install_snapshot`

The gap is the system-level layer above these leaf modules.

## Goals / Non-Goals

**Goals:**
- Ghost spec types for vote history, leader log snapshots, entry provenance
- System-level election safety proof (one leader per term)
- Intermediate reachability decomposition for modular invariant proofs
- Inductive proof that the Log Matching Property holds across all transitions
- Trace relation framework connecting state invariants to client behavior
- Linearizability spec and the top-level safety-implies-linearizability theorem
- Close the resolve_conflict external_body
- Close the leader_completeness precondition gap with ghost state

**Non-Goals:**
- Full protocol state machine with message routing (stays out of scope)
- Network semantics or message reordering model (Verdi-specific)
- Liveness proofs (depend on timing assumptions and fairness)
- OCaml/runtime extraction (Verdi-specific; we target Rust)
- All 90 Verdi invariants (we prove the structural skeleton; many of
  their invariants are Coq-specific bookkeeping)

## Decisions

### 1. Ghost state as Verus spec structs, not runtime types

**Choice:** `VoteRecord`, `LeaderLogSnapshot`, `EntryProvenance` are
defined inside `verus!` blocks as `pub struct` with `#[verifier::spec]`
fields. They exist only in the proof world. Spec functions operate on
`Seq<VoteRecord>` etc.

**Rationale:** The Verdi proof uses Coq Inductive types for ghost state,
extracted to nothing at runtime. Verus spec structs serve the same role —
they're erased by vstd during normal compilation. No runtime overhead,
no API surface for users who aren't writing proofs.

**Alternative:** Runtime types with a `#[cfg(debug_assertions)]` guard.
Rejected — mixes proof obligations with runtime behavior and complicates
the trusted computing base.

### 2. Election safety as a standalone proof module, not embedded in ballot

**Choice:** New `election_safety_proof.rs` module that imports from
`ballot`, `quorum`, and `ghost_history`. Proves the system-level theorem
separately from the per-node ballot logic.

**Rationale:** `ballot.rs` proves that a single node votes at most once
per term. The system-level claim "at most one leader per term" requires
reasoning about quorum intersection across nodes — a fundamentally
different proof obligation. The Verdi proof separates these
(OneLeaderPerTermProof.v vs. Raft.v). Keeping them separate in our
codebase matches the proof structure and avoids coupling local voting
logic with system-wide reasoning.

**Alternative:** Add the system-level proof to ballot.rs. Rejected —
ballot.rs is about a single node's voting decision. System-level claims
belong in a system-level module.

### 3. Step decomposition as spec predicates, not an execution framework

**Choice:** `raft_step.rs` defines `RaftState` as a spec struct and
`step_preserves` as a spec function. No runtime Raft execution loop.
The decomposition is purely for structuring proofs.

**Rationale:** We're not building a Raft runtime. We're building verified
decision logic. The step decomposition exists to organize proof
obligations: "prove that leader_heartbeat preserves invariant P" is a
spec-level lemma, not a runtime test. The Verdi framework includes a
full network semantics because they extract to OCaml; we don't need
that. We need the proof architecture without the execution model.

**Alternative:** Build a minimal Raft event loop in Verus. Rejected —
massively increases scope. The leaf-level functions are the verification
targets; the step decomposition just organizes their proofs.

### 4. Linearizability as an abstract spec with manual witness construction

**Choice:** Define `linearizes_to(trace, sigma)` as a spec predicate.
The top-level theorem states that state machine safety implies
linearizability. The proof constructs the witness σ from the committed
log and shows the three conditions hold.

**Rationale:** Linearizability in the Verdi proof is expressed as a
property of traces. We can state the same property as a Verus spec
without modeling the full network. The key insight from the paper is
that σ is just "the committed command sequence" and the reordering
τ' is constructed by putting commands in commit-index order. This
construction is purely spec-level — no runtime code.

**Alternative:** Model traces as runtime Vec<Event> and check
linearizability at test time. Complementary but doesn't give the
spec-level guarantee. We should do both.

### 5. Close resolve_conflict by strengthening the append loop invariant

**Choice:** Remove `#[verifier::external_body]` from `resolve_conflict`.
Prove the append loop invariant: at iteration j, the follower equals
`append_entries_spec(resolve_start_state(...), leader_entries, j)`.

**Rationale:** The `resolve_start_state` helper spec already exists in
`log_conflict.rs`. The loop pushes `leader_entries[j]` each iteration.
The spec `append_entries_spec` is defined recursively as
`prefix.push(leader_entries[from])` for each step. The loop and the
spec mirror each other. The gap is connecting the truncation step
(or no-truncation step) to `resolve_start_state` — proving that after
the match/truncation logic, the actual Vec state equals the spec's
starting state.

**Alternative:** Leave it as external_body and add more proptest coverage.
Rejected — this is the most concrete verification gap we can close, and
the Verdi proof specifically goes through conflict resolution for log
matching.

### 6. Affinity lemmas as higher-order proof functions

**Choice:** Write proof functions like:
```rust
proof fn log_property_affinity<P>(history: Seq<EntryProvenance>, log: Seq<LogEntry>)
    requires forall|e| log.contains(e) ==> P(e)
    ensures forall|p| history.contains(p) ==> P(p.entry)
```

**Rationale:** The Verdi proof's affinity lemmas (Section 6) are
higher-order: they quantify over arbitrary properties P. Verus supports
`FnSpec` and universal quantification over spec closures. This lets us
prove "any property of the current log holds for the ghost entry history"
once, then instantiate it for each specific property (sorted indices,
positive terms, etc.).

**Alternative:** Prove each specific property for ghost state separately.
Rejected — duplicates proof effort and misses the paper's key insight
that affinity is a general principle.

### 7. Phased implementation: ghost first, then proofs, then linearizability

**Choice:** Four phases:
1. Ghost state types + election safety proof
2. Step decomposition + log matching inductive proof
3. Close resolve_conflict + close leader completeness preconditions
4. Trace relations + linearizability

**Rationale:** Ghost state is foundational — election safety and leader
completeness both need it. Step decomposition organizes all subsequent
inductive proofs. Closing the verification gaps (resolve_conflict,
leader completeness preconditions) demonstrates the ghost state is
sufficient. Linearizability is the capstone that connects everything.

## Risks / Trade-offs

**[Verus spec quantifier limits]** → The election safety proof requires
`forall|node_id| ...` over cluster members. Z3 may struggle with
universally quantified ghost state assertions across unbounded node sets.
Mitigation: bound the cluster size in the spec (use `Seq` indexed by
bounded `nat`). The Verdi proof also works with finite node sets.

**[resolve_conflict loop invariant complexity]** → The recursive spec
`append_entries_spec` must align with an imperative loop. Each unrolling
of the spec must match one loop iteration. Mitigation: add intermediate
`assert` statements at each iteration connecting the Vec state to the
spec, similar to the existing `find_conflict` loop which already works.

**[Ghost state not exercised at runtime]** → Ghost spec types have no
runtime representation, so proptest can't directly test them. Mitigation:
write proptest "model" tests that simulate ghost state updates alongside
real state updates, checking that ghost invariants hold after each
simulated step.

**[Linearizability proof may need nonlinear quantifier instantiation]** →
The witness construction quantifies over all commands and their positions
in the trace. Z3 may need manual triggers. Mitigation: structure the
proof with explicit `assert forall ... by { ... }` blocks and trigger
annotations on the quantified variables.

**[Scope creep toward full protocol verification]** → Adding ghost state
and step decomposition moves toward modeling the full protocol. Risk of
expanding into message routing, timer logic, etc. Mitigation: strict
non-goal boundary — we model state transitions as spec predicates, not
as an executable Raft implementation.
