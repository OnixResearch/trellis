## Phase 1: Simple exec loops (bloom, consistent_hash, crdt)

- [ ] Close `bloom::query` — loop with early-return false, spec is universal quantifier
- [ ] Close `bloom::insert_monotonic` — inductive proof: set bit survives further inserts
- [ ] Close `bloom::no_false_negatives` — combines insert_monotonic + query_spec
- [ ] Close `consistent_hash::lookup` — linear scan matching first_ge_index_helper
- [ ] Close `consistent_hash::complete_coverage` — existence: result is in the ring
- [ ] Close `consistent_hash::single_node_all_keys` — unfold spec on length-1 seq
- [ ] Close `crdt::g_counter_value` — sum loop matching recursive spec

## Phase 2: Iterative math (backoff, gossip, merkle)

- [ ] Close `backoff::compute_backoff` — iterative doubling matching pow2 spec
- [ ] Close `gossip::infection_lower_bound` — iterative multiplication with saturation
- [ ] Close `gossip::infection_bounded` — proof: saturating_pow capped at cluster_size
- [ ] Close `merkle::verify_proof` — iterative hash walk matching recursive spec

## Phase 3: Sequence operations (nix_list, nix_attrset open specs)

- [ ] Close `nix_list::concat_lists` — flatten nested vecs
- [ ] Close `nix_attrset::attr_names` — map loop, open spec
- [ ] Close `nix_attrset::attr_values` — map loop, open spec
- [ ] Close `nix_attrset::has_attr` — linear search, open spec (existential)
- [ ] Close `nix_attrset::get_attr` — linear search, open spec (choose + existential)

## Phase 4: Remaining closeable gaps

- [ ] Close `nix_version::parse_drv_name` — byte scan for hyphen-digit
- [ ] Close `nix_string::replace_strings` — byte scanning with pattern match
- [ ] Close `properties::prove_sequential_chunk_continuation` — int↔u64 cast chain

## Intentional axioms (document, don't close)

- chain::hash_deterministic — uninterpreted hash axiom
- chain::compute_entry_hash — wraps blake3 at runtime
- hash_bucket::pow2_mask_is_mod — 64-bit bit_vector SAT exceeds rlimit
- rate_limit::tokens_after_refill — nonlinear division chain
- rate_limit::tokens_to_wait_ms — nonlinear division chain
- rate_limit::wait_enables_consumption — chained nonlinear proof
- gossip::convergence_rounds — depends on closed spec ceil_log_spec
- nix_version::compare_versions — recursive byte-level comparison
- nix_version::compare_versions_antisymmetric — proof over recursive spec
- nix_version::compare_versions_reflexive — proof over recursive spec
- nix_version::compare_versions_range — proof over recursive spec
- nix_attrset::merge — closed spec fn (would need spec redesign)
- nix_attrset::remove_attrs — closed spec fn
- nix_attrset::intersect_attrs — closed spec fn
- nix_attrset::list_to_attrs — closed spec fn
