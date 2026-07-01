# Trellis Module Guide

Use this guide when you know the job you want to model but not the module name.
Every module reference below links to the local source file so `scripts/check-module-guide.rs`
can reject stale links. Runnable examples are being added by the separate
`add-runnable-examples` Cairn change; entries without example links intentionally
point to source and requirements first.

## Bounded arithmetic and data-shape checks

Goal labels: bounded arithmetic, range admission, checked conversion.

Start with [`arithmetic`](../src/arithmetic.rs) for absolute difference,
saturating arithmetic, checked multiplication, GCD, and ceiling division. Use
[`bounds`](../src/bounds.rs) for clamping/range predicates and
[`conversion`](../src/conversion.rs) for checked narrowing between integer
widths. Follow up with [`integer`](../src/integer.rs) for parity, alignment,
log2, and next-power-of-two helpers.

Use these modules when you need a small deterministic helper with a direct
`result == *_spec(...)` contract. Do not treat them as arbitrary-precision math
or side-channel claims.

## Quorum admission and consensus safety

Goal labels: quorum admission, leader safety, log admission.

Start with [`quorum`](../src/quorum.rs) for majority thresholds and quorum
intersection facts. For leader-election modeling, use [`election`](../src/election.rs),
[`vote_tally`](../src/vote_tally.rs), and [`election_safety_proof`](../src/election_safety_proof.rs).
For Raft-shaped finite metadata admission, start with
[`raft_consensus_core`](../src/raft_consensus_core.rs), then move to
[`append_entries_check`](../src/append_entries_check.rs),
[`commit_advance`](../src/commit_advance.rs), or
[`install_snapshot`](../src/install_snapshot.rs) for narrower paths.

Caveat: these are finite verified models, not network services. Timing, storage
fsync, and external OpenRaft conformance require separate adapter evidence.

## Authorization and trust decisions

Goal labels: authorization, UCAN admission, web-of-trust policy.

Start with [`authorize`](../src/authorize.rs), [`delegation`](../src/delegation.rs),
[`scope`](../src/scope.rs), and [`containment`](../src/containment.rs) for small
capability decisions. For UCAN-shaped checks, use [`ucan_token`](../src/ucan_token.rs),
[`ucan_caveat`](../src/ucan_caveat.rs), [`ucan_chain`](../src/ucan_chain.rs), and
[`ucan_invocation`](../src/ucan_invocation.rs). For rooted trust policies, use
[`web_of_trust_policy`](../src/web_of_trust_policy.rs),
[`web_of_trust_chain`](../src/web_of_trust_chain.rs), and
[`web_of_trust_resolution`](../src/web_of_trust_resolution.rs).

Caveat: crypto and codec surfaces near [`ucan_did`](../src/ucan_did.rs),
[`ucan_signature`](../src/ucan_signature.rs), and
[`web_of_trust_principal`](../src/web_of_trust_principal.rs) are
trusted-boundary-adjacent. Check `docs/proof-gap-sites.tsv` before relying on
those APIs for more than their stated structural contracts.

## Serialization and codec shapes

Start with [`serialize_inj`](../src/serialize_inj.rs) for canonical byte encoding
injectivity patterns and [`serialization_codec`](../src/serialization_codec.rs)
for finite codec envelope admission. Use [`ucan_cid`](../src/ucan_cid.rs),
[`ucan_base64url`](../src/ucan_base64url.rs), and [`ucan_json`](../src/ucan_json.rs)
when the data is UCAN-shaped.

Caveat: codec modules may deliberately separate structural byte-shape checks
from trusted third-party codec behavior. Look for proof-gap inventory entries
before claiming full external codec correctness.

## Memory layout and allocator modeling

Goal labels: memory layout, allocation admission, stack layout.

Start with [`region`](../src/region.rs), [`page`](../src/page.rs), and
[`buddy`](../src/buddy.rs) for address ranges, page decomposition, and buddy
allocation arithmetic. Use [`slab`](../src/slab.rs), [`free_list`](../src/free_list.rs),
[`pool`](../src/pool.rs), and [`arena`](../src/arena.rs) for allocator-shaped
state transitions. For compiler backend memory layout, use
[`stack_frame_layout`](../src/stack_frame_layout.rs) and
[`register_allocation`](../src/register_allocation.rs).

Caveat: allocator modules model finite metadata/state transitions. They do not
allocate process memory or replace a runtime allocator.

## Compiler analysis and IR transforms

Goal labels: compiler analysis, CFG modeling, SSA, optimization.

Start with [`control_flow_graph`](../src/control_flow_graph.rs) for block/edge
shape, then [`dominator_tree`](../src/dominator_tree.rs),
[`loop_detection`](../src/loop_detection.rs), and [`liveness_analysis`](../src/liveness_analysis.rs)
for graph analyses. Use [`ssa_construction`](../src/ssa_construction.rs),
[`constant_folding`](../src/constant_folding.rs), and
[`dead_code_elimination`](../src/dead_code_elimination.rs) for transform-focused
models.

Caveat: several compiler modules expose verified finite analyses over the shared
raw IR rather than a full compiler frontend/backend. Check module docs for the
exact execution semantics.

## Verification and contribution workflow

For contribution mechanics, start with [CONTRIBUTING.md](../CONTRIBUTING.md) and
[`docs/module-template.md`](module-template.md). For proof-gap adjacency, check
`docs/proof-gap-sites.tsv` and run `./scripts/check-proof-gap-inventory.rs`.
For module catalog drift, run `./scripts/check-readme-module-catalog.rs` and
`./scripts/check-module-guide.rs`.
