# Verification Gaps Specification

## Purpose

Tracks every trusted or weakened proof boundary in the current tree, not
just a historical subset of `#[verifier::external_body]` items. The goal
is to keep the trust surface explicit: every `external_body`,
`assume(...)` oracle, `@verified-logic:no-ensures` exemption, or
placeholder spec is a place where the proof story is weaker than the
fully verified `exec fn ensures result == spec(...)` pattern.

## Current Status

As of `2026-04-09`, backed by fresh reruns on the current tree, the live verification pipeline is green:

- `./scripts/verify.sh` passes
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (`3191 passed`)
- `./scripts/verus.sh src/lib.rs` passes (`5372 verified, 0 errors`)
- Tracey is green (`1197/1197 covered`, `1197/1197 verified`, `0 uncovered`, `0 untested`)

This green status does **not** mean the trust surface is empty. The
following inventory is the current audited boundary.

## Current Inventory

### `#[verifier::external_body]`

`29` external_body items remain across `11` files.

Each trusted site is listed individually below so the inventory matches the
actual tree and not just per-file totals.

| Site | Classification | Rationale | Test coverage |
|---|---|---|---|
| `src/chain.rs:42 hash_deterministic` | Permanent | Determinism axiom for closed `hash_spec`; this is the cryptographic assumption boundary. | `src/chain.rs` unit tests + proptest (`381-630`) exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:70 compute_entry_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets the closed `hash_spec`. | `src/chain.rs` unit tests + proptest (`381-630`) exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:207 hash_collision_free` | Permanent | Collision-resistance axiom for the closed hash model. | `src/chain.rs` unit tests + proptest (`381-630`) exercise the runtime hash wrapper and chain properties. |
| `src/snapshot_integrity.rs:43 dual_hash_deterministic` | Permanent | Determinism axiom for uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest (`274-356`) exercise round-trip and tamper-detection behavior. |
| `src/snapshot_integrity.rs:54 dual_hash_collision_resistant` | Permanent | Collision-resistance axiom for the dual-hash model. | `src/snapshot_integrity.rs` unit tests + proptest (`274-356`) exercise round-trip and tamper-detection behavior. |
| `src/snapshot_integrity.rs:63 dual_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest (`274-356`) exercise round-trip and tamper-detection behavior. |
| `src/nix_version.rs:207 cv_fuel_antisymmetric` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for antisymmetry. | `src/nix_version.rs` unit tests + proptest corpus (`519-660`) cover version-ordering behavior. |
| `src/nix_version.rs:267 cv_fuel_reflexive` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for reflexivity. | `src/nix_version.rs` unit tests + proptest corpus (`519-660`) cover version-ordering behavior. |
| `src/nix_version.rs:300 cv_fuel_range` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for range reasoning. | `src/nix_version.rs` unit tests + proptest corpus (`519-660`) cover version-ordering behavior. |
| `src/nix_version.rs:365 compare_versions` | Permanent | Iterative exec/spec bridge still hits the same `cv_fuel` unfolding limit. | `src/nix_version.rs` unit tests + proptest corpus (`519-660`) cover version-ordering behavior. |
| `src/log_invariant_inductive.rs:174 leader_append_preserves_lmp` | Permanent | Sound only under the system-level Raft leader-chain invariant, not as a local lemma. | `src/log_invariant_inductive.rs` unit tests + proptest (`387-602`) cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:223 leader_append_preserves_lmp_right` | Permanent | Same system-level leader-chain invariant gap as the left-extension lemma. | `src/log_invariant_inductive.rs` unit tests + proptest (`387-602`) cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:275 leader_invariant_preserves_lmp` | Permanent | Same system-level leader-chain invariant gap as the append lemmas. | `src/log_invariant_inductive.rs` unit tests + proptest (`387-602`) cover valid log-shape behavior. |
| `src/bitwise.rs:197 is_sign_extended` | Closeable | Variable-shift bit-vector reasoning over arbitrary widths is still trusted. | `src/bitwise.rs` unit tests + proptest (`243-392`) cover the runtime checker, zero, all-ones, and width cases. |
| `src/bitwise.rs:224 sign_extended_zero` | Closeable | Z3 still cannot discharge the variable-shift proof for zero at arbitrary widths. | `src/bitwise.rs` unit tests + proptest (`243-392`) cover the runtime checker, zero, all-ones, and width cases. |
| `src/bitwise.rs:234 sign_extended_all_ones` | Closeable | Z3 still cannot discharge the variable-shift proof for all-ones at arbitrary widths. | `src/bitwise.rs` unit tests + proptest (`243-392`) cover the runtime checker, zero, all-ones, and width cases. |
| `src/cron_field.rs:188 first_match_correct` | Closeable | Nonlinear modular-arithmetic reasoning for `first_match_spec` is still trusted. | `src/cron_field.rs` unit tests + proptest (`218-334`) cover valid/sentinel outcomes and step behavior. |
| `src/cron_field.rs:204 first_match_ge_from` | Closeable | Same nonlinear modular-arithmetic gap as `first_match_correct`. | `src/cron_field.rs` unit tests + proptest (`218-334`) cover monotonicity and step behavior. |
| `src/cron_next.rs:106 next_occurrence` | Closeable | Search/rollover loop is still a trusted exec/spec bridge. | `src/cron_next.rs` unit tests + proptest (`363-558`) cover rollover, leap-year, and satisfiable/unsatisfiable schedules. |
| `src/cron_next.rs:297 advance_one_minute` | Closeable | Calendar rollover helper is still trusted runtime code inside the next-occurrence search. | `src/cron_next.rs` unit tests + proptest (`363-558`) cover minute/hour/day/month/year rollover behavior. |
| `src/free_list.rs:87 free_list_push` | Closeable | Exec/spec list-prepend bridge remains trusted. | `src/free_list.rs` unit tests + proptest (`290-375`) cover push/pop/remove and LIFO behavior. |
| `src/free_list.rs:112 free_list_pop` | Closeable | Exec/spec head-removal bridge remains trusted. | `src/free_list.rs` unit tests + proptest (`290-375`) cover push/pop/remove and LIFO behavior. |
| `src/free_list.rs:146 free_list_remove` | Closeable | Exec/spec filtered-removal bridge remains trusted. | `src/free_list.rs` unit tests + proptest (`290-375`) cover push/pop/remove and LIFO behavior. |
| `src/alloc_bitmap.rs:180 bitmap_alloc` | Closeable | First-free search and cloned-state bridge remain trusted. | `src/alloc_bitmap.rs` unit tests + proptest (`414-532`) cover allocation, full maps, generations, and stale handles. |
| `src/alloc_bitmap.rs:237 bitmap_free` | Closeable | Handle validation and updated-state bridge remain trusted. | `src/alloc_bitmap.rs` unit tests + proptest (`414-532`) cover allocation, full maps, generations, and stale handles. |
| `src/pool.rs:54 pool_alloc` | Closeable | Composition over `alloc_bitmap` and `slab` remains trusted runtime glue. | `src/pool.rs` unit tests + proptest (`221-287`) cover alloc/free/full/count behavior. |
| `src/pool.rs:106 pool_free` | Closeable | Free-path composition over `alloc_bitmap` remains trusted runtime glue. | `src/pool.rs` unit tests + proptest (`221-287`) cover alloc/free/full/count behavior. |
| `src/segregated_free.rs:161 seg_alloc` | Closeable | Nested `Vec<Vec<u64>>` to `Seq<Seq<u64>>` bridge is still trusted. | `src/segregated_free.rs` unit tests + proptest (`268-344`) cover class selection, alloc/free, and free-count behavior. |
| `src/segregated_free.rs:196 seg_free` | Closeable | Nested `Vec<Vec<u64>>` mutation still lacks a proved exec/spec bridge. | `src/segregated_free.rs` unit tests + proptest (`268-344`) cover class selection, alloc/free, and free-count behavior. |

Summary:

- Permanent external-body items: `13`
- Closeable external-body items: `16`

### `assume(...)` soundness oracles

`2` soundness oracles remain in verified code.

| Site | Classification | Rationale |
|---|---|---|
| `src/topo_sort.rs:265` | Closeable | Checker soundness currently ends in an `assume(is_topo_order_spec(...))` oracle |
| `src/reachability.rs:286` | Closeable | Completeness is still partially delegated to an `assume(reachable_fuel(...))` oracle |

### `@verified-logic:no-ensures` and placeholder surfaces

`5` weakened exec/spec boundaries remain.

| Site | Classification | Rationale |
|---|---|---|
| `src/index_select.rs:99` | Closeable | `best_index_spec` is still a placeholder and the exec fn is exempted |
| `src/nix_closure.rs:61` | Closeable | `generic_closure_spec` still returns `start`; the model is intentionally incomplete |
| `src/settlement.rs:204` | Closeable | Exec/spec bridge is justified by a type conversion argument, not a verified ensures clause |
| `src/segregated_free.rs:157` | Closeable | Nested `Vec<Vec<u64>>` vs `Seq<Seq<u64>>` bridge remains exempted |
| `src/cycle_detect.rs:62` | Closeable | Kahn-cycle runtime checker is tested but not yet connected to `has_cycle_spec` |

## Requirements

### Requirement: Proof Audit Inventory Covers All Trusted Boundaries

The proof-gap inventory MUST track every trusted or weakened proof
boundary in the current tree, not only a historical subset of
`#[verifier::external_body]` items.

Tracked categories MUST include:

- `#[verifier::external_body]`
- `assume(...)` soundness oracles
- public functions exempted with `@verified-logic:no-ensures`
- public verified functions that actually lack an `ensures` clause
- placeholder or simplified specs that weaken exec/spec correspondence

#### Scenario: Mixed trust surfaces exist

- GIVEN the tree contains `external_body`, `assume(...)`, and
  `@verified-logic:no-ensures` sites
- WHEN the proof-gap inventory is refreshed
- THEN every site appears in the inventory with file path,
  classification, and rationale

#### Scenario: Inventory totals are published

- GIVEN the inventory has been regenerated from the current tree
- WHEN `README.md` or the main `verification-gaps` spec reports totals
- THEN the published totals match the regenerated inventory

### Requirement: Proof Status Reports Are Evidence Backed

The project MUST NOT claim a fully green proof posture unless the live
verification commands support that claim.

#### Scenario: Verification pipeline is failing

- GIVEN `./scripts/verify.sh` reports failing Clippy, Verus, or Tracey gates
- WHEN the proof status is documented
- THEN the documentation reports the failing gates and affected modules
  instead of claiming all checks are green

#### Scenario: Verification pipeline is green

- GIVEN `./scripts/verify.sh` passes and Tracey reports zero uncovered
  and zero untested rules
- WHEN the proof status is documented
- THEN the documentation may describe the proof posture as green while
  still reporting the audited trust surface

### Requirement: Proof Audit Tracks Tracey Coverage Drift

Tracey uncovered and untested rules discovered during a proof audit MUST
be treated as remediation items until cleared.

#### Scenario: Uncovered rules are present

- GIVEN `tracey query uncovered` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  implementation references to add

#### Scenario: Untested rules are present

- GIVEN `tracey query untested` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  verification references to add

#### Scenario: Tracey is green again

- GIVEN the missing `r[impl]` and `r[verify]` references have been added
- WHEN Tracey is re-run
- THEN the audit records `0 uncovered` and `0 untested`

### Requirement: Ensure Checker Distinguishes Real Gaps From Parser Limits

The contract-checking tooling MUST report real missing postconditions
without flagging long valid signatures as missing `ensures`.

#### Scenario: Long verified signature

- GIVEN a public verified function whose `ensures` clause appears many
  lines after the `pub fn`
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is accepted as having an `ensures` clause

#### Scenario: Real missing ensures

- GIVEN a public verified function with no `ensures` clause and no
  exemption marker
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is reported as a real contract gap

### Requirement: Permanent Trust Boundaries Stay Separable From Closeable Ones

Permanent architectural trust boundaries MUST be distinguished from
closeable proof debt.

#### Scenario: Permanent item is inventoried

- GIVEN an item exists because of cryptographic assumptions, solver
  limits, or a system-level invariant that is intentionally modeled as trusted
- WHEN it is listed in the gap inventory
- THEN it is classified as permanent with a rationale

#### Scenario: Closeable item is inventoried

- GIVEN an item exists because a proof is incomplete, a loop/spec bridge
  is still trusted, or a placeholder spec remains in the tree
- WHEN it is listed in the gap inventory
- THEN it is classified as closeable with a rationale
