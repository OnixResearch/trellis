# Property Specification Index

This document is the discoverable catalog of trellis's executable property
specifications. It indexes the property groups, their source modules, and their
`proof fn` / proptest counterparts so consumers can find and run properties
without reading the entire source tree.

This is the Shoals `properties/` surface (chelis.ch/docs/shoals/properties/)
applied to trellis. Shoals puts properties in a `properties/` directory; trellis
puts them in `src/` as `proof fn` and `proptest!` blocks, and this index makes
them discoverable.

## Convention

Every property group entry has:

- **Group**: the module group name
- **Source modules**: the `.rs` files containing the properties
- **Property kinds**: `proof fn` (mechanically checked by Verus), `proptest`
  (randomized property test), `#[test]` (unit test)
- **Count**: approximate number of properties in the group

## Arithmetic and data structures

**Group**: arithmetic
**Source modules**: `src/arithmetic.rs`, `src/bitwise.rs`, `src/bitfield.rs`,
`src/bitmap.rs`, `src/bounds.rs`, `src/collections.rs`, `src/conversion.rs`,
`src/integer.rs`, `src/interval.rs`, `src/option.rs`, `src/ordering.rs`,
`src/result.rs`, `src/ring_buffer.rs`, `src/sequence.rs`, `src/slice.rs`
**Property kinds**: `proof fn`, `proptest`, `#[test]`
**Count**: ~150

## Raft consensus

**Group**: raft
**Source modules**: `src/election.rs`, `src/election_init.rs`,
`src/election_safety_proof.rs`, `src/pre_vote.rs`, `src/pre_vote_tally.rs`,
`src/vote_tally.rs`, `src/quorum.rs`, `src/log.rs`, `src/log_compaction.rs`,
`src/log_invariant_inductive.rs`, `src/commit_advance.rs`, `src/match_index.rs`,
`src/append_entries_check.rs`, `src/fast_backtrack.rs`, `src/linearizability.rs`,
`src/leader_completeness.rs`, `src/leader_completeness_ghost.rs`,
`src/trace_relation.rs`, `src/consensus_path.rs`, `src/properties.rs`
(cross-module Raft proofs)
**Property kinds**: `proof fn`, `proptest`
**Count**: ~300

## Consensus acceleration

**Group**: consensus-fast-path
**Source modules**: `src/consensus_fast_path/identity.rs`,
`src/consensus_fast_path/vocabulary.rs`,
`src/consensus_fast_path/membership.rs`,
`src/consensus_fast_path/agreement.rs`,
`src/consensus_fast_path/barrier.rs`,
`src/consensus_fast_path/witness.rs`, and
`src/consensus_fast_path/proofs.rs`
**Property kinds**: `proof fn`, `proptest`, `#[test]`
**Count**: 17 direct proofs, 5 randomized properties, and 24 unit tests

## WebAssembly runtime bounds

**Group**: wasm-runtime-bounds
**Source modules**: `src/wasm_runtime_bounds/identity.rs`,
`src/wasm_runtime_bounds/machine.rs`,
`src/wasm_runtime_bounds/execution.rs`,
`src/wasm_runtime_bounds/binding.rs`,
`src/wasm_runtime_bounds/resources.rs`,
`src/wasm_runtime_bounds/streaming.rs`, and
`src/wasm_runtime_bounds/proofs.rs`
**Property kinds**: `proof fn`, `proptest`, `#[test]`
**Count**: 20 direct proofs, 3 randomized properties, and 24 unit tests

## Scheme logic

**Group**: scheme-logic
**Source modules**: `src/scheme_logic/identity.rs`,
`src/scheme_logic/syntax.rs`, `src/scheme_logic/registry.rs`,
`src/scheme_logic/linkage.rs`, `src/scheme_logic/boundary.rs`,
`src/scheme_logic/vm.rs`, `src/scheme_logic/heap.rs`, and
`src/scheme_logic/proofs/mod.rs`
**Property kinds**: `proof fn`, `proptest`, `#[test]`
**Count**: 27 direct proofs, 4 randomized properties, and 27 unit tests

## Concurrency primitives

**Group**: concurrency
**Source modules**: `src/lock_ordering.rs`, `src/epoch_reclaim.rs`,
`src/hazard_pointer.rs`, `src/seqlock.rs`, `src/ref_count.rs`,
`src/work_stealing.rs`, `src/barrier.rs`, `src/backoff.rs`, `src/rcu.rs`,
`src/cas.rs`, `src/cas_loop.rs`, `src/ticket_lock.rs`, `src/futex_state.rs`,
`src/memory_order.rs`, `src/cache_hierarchy.rs`, `src/directory_coherence.rs`,
`src/cache_prefetch.rs`, `src/store_buffer.rs`, `src/tlb_shootdown.rs`,
`src/properties.rs` (cross-module concurrency proofs)
**Property kinds**: `proof fn`, `proptest`
**Count**: ~250

## Financial accounting

**Group**: finance
**Source modules**: `src/balance.rs`, `src/double_entry.rs`, `src/escrow.rs`,
`src/fee.rs`, `src/journal.rs`, `src/ledger.rs`, `src/settlement.rs`,
`src/transfer.rs`, `src/properties.rs` (cross-module finance proofs)
**Property kinds**: `proof fn`, `proptest`
**Count**: ~80

## Memory management and allocator

**Group**: memory
**Source modules**: `src/buddy.rs`, `src/page.rs`, `src/region.rs`,
`src/slab.rs`, `src/watermark.rs`, `src/heap_index.rs`, `src/alloc_token.rs`,
`src/size_class.rs`, `src/free_list.rs`, `src/split_coalesce.rs`,
`src/alloc_bitmap.rs`, `src/arena.rs`, `src/pool.rs`, `src/segregated_free.rs`,
`src/coalesce_policy.rs`, `src/fragmentation.rs`
**Property kinds**: `proof fn`, `proptest`
**Count**: ~120

## Storage engine primitives

**Group**: storage
**Source modules**: `src/btree_balance.rs`, `src/btree_insert.rs`,
`src/btree_delete.rs`, `src/lsm_level.rs`, `src/wal_segment.rs`,
`src/wal_recovery.rs`, `src/compaction_fence.rs`, `src/page_checksum.rs`,
`src/free_space_map.rs`, `src/write_buffer.rs`, `src/write_policy.rs`,
`src/block_cache.rs`, `src/set_associative_lookup.rs`
**Property kinds**: `proof fn`, `proptest`
**Count**: ~100

## Cross-module properties

**Group**: cross-module
**Source modules**: `src/properties.rs`, `src/concurrent_composition.rs`,
`src/consensus_path.rs`
**Property kinds**: `proof fn`, `proptest`
**Count**: 200+ cross-module proofs covering signal-bitiwise, permission
independence, escrow-balance, ledger-transfer, fee conservation,
election-quorum, fencing-CAS, vector clock-CRDT, mpmc-mpsc occupancy,
spin-CAS retry, seqlock-memory order, RCU-epoch reclaim, ref count-hazard
pointer, lock ordering-ticket lock, barrier-quorum, futex-notify roundtrip,
and concurrency protocol-data structure composition.

## Circuit primitives (R1CS)

**Group**: r1cs
**Source modules**: `src/r1cs_constraint.rs`, `src/r1cs_bit_decomposition.rs`,
`src/r1cs_lookup.rs`, `src/r1cs_boolean.rs`, `src/r1cs_gates.rs`,
`src/r1cs_system.rs`, `src/r1cs_public_inputs.rs`
**Property kinds**: `proof fn`, `#[test]`
**Count**: ~40

## Non-claims

This index is a discoverability surface, not a completeness claim. It does not
claim to list every property, every module, or every proof. Properties listed
here are mechanically checked (Verus) or randomized (proptest); they are not
exhaustive correctness proofs. See `docs/module-scope.md` for per-group scope
and limitations, and `docs/proof-gap-sites.tsv` for trusted boundaries.
