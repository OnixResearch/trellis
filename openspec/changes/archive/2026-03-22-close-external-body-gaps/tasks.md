## Phase 1: Simple exec loops (bloom, consistent_hash, crdt)

- [x] Close `bloom::query` — while loop with ghost map_values tracking ✅
- [x] Close `bloom::insert_monotonic` — inductive proof with insert_preserves_len helper ✅
- [x] Close `bloom::no_false_negatives` — inductive proof via no_false_negatives_helper ✅
- [x] Close `consistent_hash::lookup` — while loop with first_ge_skip helper ✅
- [x] Close `consistent_hash::complete_coverage` — first_ge_index_helper_bounds + case split ✅
- [x] Close `consistent_hash::single_node_all_keys` — unfold spec on length-1 seq ✅
- [x] Close `crdt::g_counter_value` — sum loop with take_lemma/nonneg/prefix_le helpers ✅

## Phase 2: Iterative math (backoff, gossip, merkle)

- [x] Close `backoff::compute_backoff` — while loop with pow2 invariant + vstd mul lemmas ✅
- [x] Close `gossip::infection_lower_bound` — iterative saturating_pow with overflow flag ✅
- [x] Close `gossip::infection_bounded` — saturating_pow_nonneg helper + case split ✅
- [x] Close `merkle::verify_proof` — iterative loop with verify_proof_unfold helper ✅

## Phase 3: Sequence operations (nix_list, nix_attrset open specs)

- [x] Close `nix_list::concat_lists` — nested loops with concat_lists_take_step lemma ✅
- [x] Close `nix_attrset::attr_names` — while loop with extensional equality ✅
- [x] Close `nix_attrset::attr_values` — same pattern as attr_names ✅
- [x] Close `nix_attrset::has_attr` — linear scan with witness/negative invariant ✅
- [x] Close `nix_attrset::get_attr` — added is_sorted_keys requires for choose uniqueness ✅

## Phase 4: Remaining closeable gaps

- [x] Close `properties::prove_sequential_chunk_continuation` — nonlinear_arith for cast chain ✅
- nix_version::parse_drv_name — closed spec, can't close
- nix_string::replace_strings — closed spec, can't close

## Intentional axioms (documented, not closeable)

### Genuine axioms
- chain::hash_deterministic — uninterpreted hash function axiom
- chain::compute_entry_hash — wraps blake3 at runtime

### Z3 solver limits
- hash_bucket::pow2_mask_is_mod — 64-bit bit_vector SAT exceeds rlimit
- rate_limit::tokens_after_refill — nonlinear division/multiplication chain
- rate_limit::tokens_to_wait_ms — nonlinear division chain
- rate_limit::wait_enables_consumption — chained nonlinear proof across two specs

### Closed spec functions (would need spec redesign to open)
- gossip::convergence_rounds — depends on closed spec ceil_log_spec
- nix_version::compare_versions — closed spec compare_versions_spec
- nix_version::compare_versions_antisymmetric — proof over closed spec
- nix_version::compare_versions_reflexive — proof over closed spec
- nix_version::compare_versions_range — proof over closed spec
- nix_version::parse_drv_name — closed spec parse_drv_name_spec
- nix_string::replace_strings — closed spec replace_strings_spec
- nix_attrset::merge — closed spec merge_spec
- nix_attrset::remove_attrs — closed spec remove_attrs_spec
- nix_attrset::intersect_attrs — closed spec intersect_attrs_spec
- nix_attrset::list_to_attrs — closed spec list_to_attrs_spec
