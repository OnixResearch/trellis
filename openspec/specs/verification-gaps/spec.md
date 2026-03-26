# Verification Gaps Specification

## Purpose

Tracks every `#[verifier::external_body]` item and `closed spec fn` in
the codebase. Each gap is classified as closeable or permanent, with a
rationale. The goal is to minimize trusted code — every external_body is
an unverified assertion that Verus accepts on faith.

## Current Inventory

13 external_body items remain across 4 files:

| File | Count | Classification |
|---|---|---|
| `chain.rs` | 3 | Permanent — crypto axioms |
| `snapshot_integrity.rs` | 3 | Permanent — crypto axioms |
| `nix_version.rs` | 4 | Permanent — Z3 rlimit on nested spec |
| `log_invariant_inductive.rs` | 3 | Permanent — requires system-level invariant |

## Requirements

### Cryptographic Hash Axioms

#### Requirement: Hash Functions Are Uninterpreted

`chain::hash_spec` and `snapshot_integrity::dual_hash_spec` MUST remain
`closed spec fn` with no definition body. They model cryptographic hash
functions (Blake3) whose internals are deliberately opaque to the prover.

The axioms `hash_deterministic` and `dual_hash_deterministic` assert
that equal inputs produce equal outputs. The axioms `hash_collision_free`
and `dual_hash_collision_resistant` assert the converse. These are the
standard cryptographic assumptions and cannot be proved from first
principles.

The exec wrappers `compute_entry_hash` and `dual_hash` MUST remain
external_body because their ensures clauses reference closed specs
that Z3 cannot inspect.

##### Classification: Permanent (6 items across 2 files)

These items (2 closed specs, 4 proof axioms, 2 exec fns) are
permanent. They encode the irreducible trust boundary between formal
verification and cryptographic assumptions.

### Version Comparison Unfolding

#### Requirement: Version Comparison Properties

`compare_versions` compares Nix version strings byte-by-byte with
numeric and lexicographic sub-comparisons. The spec `cv_fuel` uses
fuel-bounded recursion with 8+ levels of nested let/if/else.

The proofs `cv_fuel_antisymmetric`, `cv_fuel_reflexive`, and
`cv_fuel_range` have correct inductive structure but Z3 cannot unfold
the spec deeply enough even at rlimit(2000). At each recursion level,
Z3 must case-split on separator detection, digit detection, numeric
parsing, and lexicographic comparison — producing combinatorial
explosion in quantifier instantiation.

The exec fn `compare_versions` is marked external_body because
connecting the iterative loop to the recursive `cv_fuel` spec has
the same unfolding depth problem.

##### Classification: Permanent (4 items)

The spec is open and the proof bodies are preserved as documentation.
The limitation is Z3's quantifier instantiation budget, not a logical
gap. Comprehensive proptest oracles cover runtime correctness
(reflexivity, antisymmetry, range).

### Log Matching Inductive Invariant

#### Requirement: Leader Append Preserves LMP

`leader_append_preserves_lmp`, `leader_append_preserves_lmp_right`,
and `leader_invariant_preserves_lmp` assert that extending a log
preserves the Log Matching Property. The ensures clauses are correct
only under the SYSTEM-LEVEL invariant that all logs were built from
the same leader chain.

Counterexample without system-level invariant:
- log_a = [term=1, term=3], log_b = [term=1, term=2, term=3]
- LMP(log_a, log_b) holds (disagreement at index 1 means implication
  is vacuously true)
- Push term=3 to log_a: new_a = [term=1, term=3, term=3]
- At i=2: new_a[2].term=3 == log_b[2].term=3, but new_a[1].term=3
  != log_b[1].term=2 — prefix match fails
- LMP(new_a, log_b) is FALSE

The proof bodies explain why: the `i == log_a.len()` case (new entry)
requires knowing that all prior entries match, which only follows from
the system-level invariant that entries with the same term came from
the same leader.

##### Classification: Permanent (3 items)

These are trust boundaries encoding the system-level Raft safety
argument, not proof gaps. The external_body hides an ensures clause
that is only sound under global assumptions. Removing it would expose
the unsoundness. Proptest covers the runtime behavior for valid
Raft log sequences.

## Closed Gaps (Historical)

The following gaps were closed in prior sessions:

- **buddy::order_for_size** (1 item) — 64-way case split on concrete
  order values bridges bitvector `>>` to spec-level `<<`. Same
  technique as hash_bucket::pow2_mask_is_mod.
- **rate_limit::tokens_after_refill, tokens_to_wait_ms** (2 items) —
  u128 intermediate arithmetic eliminates overflow-splitting exec code.
- **snapshot_transfer::transfer_progress** (1 item) — u128 for
  received_bytes * 100.
- **hash_bucket::pow2_mask_is_mod** (1 item) — 64-way case split on
  power-of-two values.
- **bitfield round_trip, non_interference, clear_then_extract,
  insert_idempotent** (4 items) — decomposed into sub-lemmas with
  fewer shift operations per by(bit_vector) assertion.
- **ghost_history::record_full_log_covers, cronies_exclusive** (2 items)
  — helper lemmas for existential witnesses and zero-count reasoning.
- **Various exec fns** (bloom::insert, bloom::query, consistent_hash::lookup,
  backoff::compute_backoff, merkle::verify_proof, delegation::chain_valid,
  markov::confidence_meets, markov::best_prediction_index,
  pattern::pattern_contains, bounded_peers::collect_peers,
  voter_set::promote_voters, peer_spec::find_separator,
  node_classify::classify_node, node_classify::classify_nodes,
  resolve_conflict, etc.) — loop invariants with ghost tracking.
- **glob::glob_match** — spec opened with segments_match_spec.
- **nix_string::replace_strings** — spec opened with recursive
  replace_at_spec.
