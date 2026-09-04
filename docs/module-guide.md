# Trellis Module Guide

Use this guide when you know the job you want to model but not the module name.
Every module reference below links to the local source file so `scripts/check-module-guide.rs`
can reject stale links. Use [the module maturity index](module-index.md) when you need a generated row for every public module, including examples, Tracey markers, and proof-gap adjacency. Runnable example links and commands are checked by
`scripts/check-examples.rs`.

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
intersection facts. For a client-visible fast path beside a base consensus
protocol, use
[`consensus_fast_path`](../src/consensus_fast_path/identity.rs) for crash-fault
fast-quorum arithmetic, same-view certificates, ordering prerequisites,
commutativity, recovery ordering, and dual-path convergence. For
leader-election modeling, use [`election`](../src/election.rs),
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
for finite codec envelope admission. Use
[`fenced_attempt_admission`](../src/fenced_attempt/identity.rs) for fenced
durable-attempt phases, epoch safety, event idempotence, completion linkage,
and exact rejection preservation. Use
[`s_expression`](../src/paren_tree/identity.rs) for bounded canonical atom/list
trees, deterministic UTF-8 rendering, canonical-only parsing, and roundtrips.
Use [`scheme_logic`](../src/scheme_logic/identity.rs) for finite reader spans,
explicit interning, module linkage, contract/blame metadata, bytecode admission,
cycle-safe value graphs, and Scheme collection shapes. This module does not
prove Steel compatibility or interpreter correctness. Use [`total_config`](../src/normal_input/identity.rs) for finite typed graphs,
canonical normal forms, digest-input selection, frozen imports, and refactors.
Use [`ucan_cid`](../src/ucan_cid.rs),
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

For bounded WebAssembly runtime control, use
[`wasm_runtime_bounds`](../src/wasm_runtime_bounds/identity.rs) for finite
transition admission, fuel, segmentation, continuation, initialization-only
page allocation, explicit growth policy, and exact chunk schedules. These laws
do not verify an interpreter, unsafe allocator, host scheduler, or sandbox.

## Compiler analysis and IR transforms

Goal labels: compiler analysis, HIR metadata, library metadata, CFG modeling, SSA, optimization, ABI metadata.

Use [`hir_attribute_parsing`](../src/hir/attribute/parsing.rs) for finite inert
parser and policy admission. Use [`hir_id_lowering`](../src/frontend_lower/identity.rs)
for owner-scoped source mappings, generated-ID freshness, created-ID use, and
definition correspondence. Use [`mir_core`](../src/mir_core/identity.rs) for
finite MIR body, local, place, shallow-rvalue, terminator, and CFG admission. Use
[`mir_dataflow_framework`](../src/finite_flow/identity.rs) for finite lattice
laws, ordered effects, bounded fixed points, and pure result cursors. Use
[`mir_destruct`](../src/destruct_paths/identity.rs) for structural obligations,
initializedness, guarded/open decisions, and replacement ordering. Use
[`mir_visitor_traversal`](../src/mir_walk/identity.rs) for explicit visit
locations, exactly-once coverage, deterministic block orders, and
shape-preserving mutation admission. Use
[`mir_pass_pipeline`](../src/mir_pass_pipeline/identity.rs) for finite MIR phase,
pass, validation, optimization-gate, and artifact-state trace admission. Use
[`backend_codegen_trace`](../src/builder_lower/identity.rs) for typed builder
handles, block mappings, source-tied operations, and control-flow admission. Use
[`rustc_monomorphization`](../src/instance_partition/identity.rs) for finite
concrete-item reachability, deduplication, stability classes, and disjoint unit
partitions. Use [`rustc_metadata_identity`](../src/rustc/metadata/identity.rs) for declared
crate-artifact compatibility and bounded dependency selection. Use
[`track_caller_abi`](../src/track/caller/abi.rs) for caller-location,
trait-inheritance, direct-call, and reify-shim admission.
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

## Runnable examples

Run runtime examples through normal Cargo; no Verus invocation is required. For proof-focused examples that intentionally invoke Verus, see [docs/proof-examples.md](proof-examples.md):

- [`core_basics`](../examples/core_basics.rs): `cargo run --example core_basics`
- [`consensus_admission`](../examples/consensus_admission.rs): `cargo run --example consensus_admission`
- [`ucan_web_of_trust`](../examples/ucan_web_of_trust.rs): `cargo run --example ucan_web_of_trust`
- [`r1cs_single_row`](../examples/r1cs_single_row.rs): `cargo run --example r1cs_single_row`
- [`storage_allocator`](../examples/storage_allocator.rs): `cargo run --example storage_allocator`
- [`integration_adapters`](../examples/integration_adapters.rs): `cargo run --example integration_adapters`

## Integration adapters

Use [`integration_adapters`](../src/integration_adapters.rs) and [the integration adapter guide](integration-adapters.md) when you need domain-shaped admission APIs over existing verified consensus, UCAN, Nix/Nickel, or WAL primitives. The adapters are finite deterministic shells and do not claim full external ecosystem conformance.

## Verification and contribution workflow

For contribution mechanics, start with [CONTRIBUTING.md](../CONTRIBUTING.md),
[`docs/module-template.md`](module-template.md), and [the cookbook](cookbook.md).
For proof-gap adjacency, check
`docs/proof-gap-sites.tsv` and run `./scripts/check-proof-gap-inventory.rs`.
For module catalog drift, run `./scripts/check-readme-module-catalog.rs`,
`./scripts/check-module-guide.rs`, and `./scripts/check-module-index.rs`.
