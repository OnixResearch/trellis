# Verification Gaps Specification

## Purpose

Tracks every trusted or weakened proof boundary in the current tree, not
just a historical subset of `#[verifier::external_body]` items. The goal
is to keep the trust surface explicit: every `external_body`,
`assume(...)` oracle, `@verified-logic:no-ensures` exemption, or
placeholder spec is a place where the proof story is weaker than the
fully verified `exec fn ensures result == spec(...)` pattern.

## Current Status

As of `2026-04-08`, the live verification pipeline is green:

- `./scripts/verify.sh` passes
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (`3162 passed`)
- `./scripts/verus.sh src/lib.rs` passes (`5053 verified, 0 errors`)
- Tracey is green (`1197/1197 covered`, `1197/1197 verified`, `0 uncovered`, `0 untested`)

This green status does **not** mean the trust surface is empty. The
following inventory is the current audited boundary.

## Current Inventory

### `#[verifier::external_body]`

`36` external_body items remain across `13` files.

| File | Count | Classification | Rationale |
|---|---:|---|---|
| `src/chain.rs` | 3 | Permanent | Cryptographic hash axioms / closed-spec boundary |
| `src/snapshot_integrity.rs` | 3 | Permanent | Cryptographic hash axioms / closed-spec boundary |
| `src/nix_version.rs` | 4 | Permanent | Deep recursive version-comparison proofs still exceed solver limits |
| `src/log_invariant_inductive.rs` | 3 | Permanent | Sound only under a system-level invariant, not a local lemma |
| `src/bitwise.rs` | 3 | Closeable | Variable-shift proofs are still trusted solver workarounds |
| `src/cron_field.rs` | 2 | Closeable | Nonlinear modular-arithmetic lemmas still missing |
| `src/cron_next.rs` | 2 | Closeable | Search/rollover execs remain trusted runtime code |
| `src/free_list.rs` | 3 | Closeable | Exec/spec correspondence for list mutation remains trusted |
| `src/alloc_bitmap.rs` | 2 | Closeable | Allocation/free execs remain trusted wrappers |
| `src/pool.rs` | 2 | Closeable | Composition over bitmap/slab is still trusted runtime glue |
| `src/segregated_free.rs` | 2 | Closeable | Nested `Vec<Vec<u64>>` to `Seq<Seq<u64>>` bridge remains trusted |
| `src/mesi.rs` | 6 | Closeable | Composite MESI checker/transition proofs were externalized to restore a green pipeline |
| `src/page.rs` | 1 | Closeable | `radix_index_bounded` remains trusted while the computed-offset bit-vector proof is tightened |

Summary:

- Permanent external-body items: `13`
- Closeable external-body items: `23`

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
