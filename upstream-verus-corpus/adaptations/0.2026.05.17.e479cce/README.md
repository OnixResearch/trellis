# Verus 0.2026.05.17 corpus adaptations

These patches are reviewable transformations of exact immutable corpus inputs. They do not alter the catalog revisions, certify the unmodified revisions, or transfer proof authority. A passing verifier run is reported as an **adapted-source** result and is bound to both the upstream fixed-output identity and the patch BLAKE3 identity.

## verified-ironkv

- Upstream revision: `6b8830a47a2dd5a8ce141438ebb2decb6550298a`
- Patch: `verified-ironkv.patch`
- Patch BLAKE3: `cedf509e48f6e83431f4884521fad46a8cd6be7908f28ef8de9b493a77f57937`
- Scope: nine files below `ironsht/src/`

The pinned upstream commit is itself titled “Adapt to finite sets and maps” and follows Verus PR [#2486](https://github.com/verus-lang/verus/pull/2486). That PR states that the new `ISet` and `IMap` names denote the old possibly-infinite `Set` and `Map` semantics. The May 17 candidate predates that June API split: its `Set` is predicate-backed, its `Map` supports total and predicate-defined domains, and `=~=` is extensional equality.

The patch therefore maps the post-split infinite collection spelling back to the candidate's pre-split generalized collections. Domain-preserving `mk_map` calls replace finite-domain `Map::new` calls; `.to_iset()` is removed where the candidate's `Set` already has the generalized semantics; and `congruent`/assertion macros are replaced by the candidate's extensional-equality forms. It adds no assumption, trusted-body, external-body, or trigger marker.

This is a bounded API-semantic compatibility argument, not a proof that the two Vstd implementations are globally equivalent.

## verified-memory-allocator

- Upstream revision: `847354efbb51a7999deb3eef0ed2bfc199bccac1`
- Patch: `verified-memory-allocator.patch`
- Patch BLAKE3: `399027aba3425050a667a60194767c1bd5e84af7f6c93bc5941b6a34f47b3f5b`
- Scope: `verus-mimalloc/layout.rs` and `verus-mimalloc/os_alloc.rs`

The published candidate package does not enable the optional Singular-backed `integer_ring` feature. The patch preserves all three arithmetic lemma signatures and contracts, replacing the unavailable attribute with explicit division, multiplication, and nonlinear proofs. It also supplies an explicit proof of the existing `os_good_alloc_size` segment-size postcondition and names the already-present segment threshold/alignment expressions. Executable behavior, assumptions, triggers, and trusted boundaries are unchanged.

## Required checks

The trial must fail closed unless it:

1. verifies each patch BLAKE3 before application;
2. applies each patch to the exact fixed-output source and records an adapted-tree BLAKE3;
3. confirms assumption, external-body, and trusted-marker counts do not increase;
4. completes native Verus execution and linking with exit code zero;
5. records the adapted result separately from the unmodified upstream identity.
