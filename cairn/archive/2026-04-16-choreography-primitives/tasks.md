## Phase 0: Repo Sync and Scaffolding

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`; confirm no existing module name starts with `choreography_` and no shipped requirement ID starts with `choreography.` or `properties.choreography-`
- [x] Create `openspec/changes/choreography-primitives/evidence/` and scaffold the evidence file set used by this change before later phases write validation output there
- [x] Record the repo-sync result in `openspec/changes/choreography-primitives/evidence/repo-sync.md`
- [x] Confirm or add `pub mod properties;` in `src/lib.rs` before new cross-module proofs depend on it
- [x] For each new choreography module, add the matching `pub mod choreography_*;` line to `src/lib.rs` with that module's first implementation task

## Phase 1: Global and Local Core

- [x] `src/choreography_global.rs`: land `RoleId`, `LabelId`, `PayloadTag`, `GlobalChoreo`, and `pub mod choreography_global;` in `src/lib.rs` with that first implementation (`r[choreography.participants]`)
- [x] `src/choreography_global.rs`: implement exact `Comm` field / continuation helpers and prove they expose stored `(from, to, label, payload_tag, next)` unchanged (`r[choreography.communication]`)
- [x] `src/choreography_global.rs`: implement and prove exact participant extraction over communications and deciders (`r[choreography.participants]`)
- [x] `src/choreography_global.rs`: implement and prove global well-formedness rules for self-send rejection, non-empty choices, distinct branch labels, and strict increasing branch-label order (`r[choreography.participants]`)
- [x] `src/choreography_local.rs`: land `LocalChoreo` and `pub mod choreography_local;` in `src/lib.rs` with that first implementation (`r[choreography.merge]`)
- [x] `src/choreography_local.rs`: implement exact merge success for identical local heads by recursive continuation merge, with an explicit Verus `decreases` argument over local syntax size (`r[choreography.merge]`)
- [x] `src/choreography_local.rs`: implement exact `Recv from decider` -> `Offer from decider` merge rule for distinct labels, preserving strict increasing label order (`r[choreography.merge]`)
- [x] `src/choreography_local.rs`: implement exact `Offer`/`Offer` merge-by-label-set rule, prove conflicting heads fail merge, and prove merge output satisfies local branch-order / local well-formedness invariants whenever the inputs do (`r[choreography.merge]`)
- [x] Add named merge proof artifacts in `src/choreography_local.rs` for identical-prefix merge, normalized offer construction, merge output local well-formedness, explicit termination / decreases, and merge-failure conditions (`r[choreography.merge]`)
- [x] Add `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `choreography.participants`, `choreography.communication`, and `choreography.merge`
- [x] Add unit/property tests covering self-send rejection, duplicate-label rejection, identical-prefix merge, decider-offer merge, and merge-failure cases

## Phase 2: Projectability and Endpoint Projection

- [x] `src/choreography_projection.rs`: land `projectable_spec`, `projectable`, and `pub mod choreography_projection;` in `src/lib.rs` with that first implementation, including the recursive "every branch subtree is projectable" case explicitly (`r[choreography.projectability]`)
- [x] `src/choreography_projection.rs`: land preconditioned-total `project_endpoint_spec` / `project_endpoint` over `requires projectable_spec(global)` (`r[choreography.projection]`)
- [x] `src/choreography_projection.rs`: prove communication-prefix projection shape for sender, receiver, and uninvolved roles (`r[choreography.projection]`)
- [x] `src/choreography_projection.rs`: prove decider-side `Choice` projection becomes exact `InternalChoice` over branch projections (`r[choreography.projection]`)
- [x] `src/choreography_projection.rs`: prove non-decider-side `Choice` projection uses the exact outer-label-preserving projection merge, preserves any shared local prefix before the first branch-dependent `Offer`, and exposes stored global branch labels at that first `Offer`; projectability still fails on non-mergeable branches (`r[choreography.projectability]`, `r[choreography.projection]`)
- [x] `src/choreography_projection.rs`: prove roles outside `participants(global)` project to `End` (`r[choreography.participants]`, `r[choreography.projection]`)
- [x] Add named projection proof artifacts in `src/choreography_projection.rs`
- [x] Add `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `choreography.projectability` and `choreography.projection`
- [x] Add unit/property tests covering sender projection, receiver projection, decider projection, nonparticipant projection, projectable branch merge, and non-projectable branch conflict

## Phase 3: One-Step Semantics

- [x] `src/choreography_step.rs`: land total `Option`-returning `global_comm_step`, `global_choice_step`, `local_send_step`, `local_recv_step`, `local_internal_choice_step`, `local_offer_step`, and `pub mod choreography_step;` in `src/lib.rs` with that first implementation (`r[choreography.steps]`)
- [x] `src/choreography_step.rs`: prove exact `Some(...)` results on matching heads / labels and exact `None` results on nonmatching heads or missing labels (`r[choreography.steps]`)
- [x] `src/choreography_step.rs`: prove every successful global head step stays on the projectable surface so post-step `project_endpoint(_, g')` calls are admissible; land this as a standalone lemma reused by the Phase 4 correspondence proof instead of re-deriving it there (`r[choreography.projectability]`, `r[choreography.steps]`)
- [x] `src/choreography_step.rs`: prove `Comm` head-step continuation shape, exact local `Send` / `Recv` head-step continuation shape, and projection equality for uninvolved roles (`r[choreography.steps]`)
- [x] `src/choreography_step.rs`: prove `Choice` head-step selection shape: decider `InternalChoice` heads and currently exposed `Offer` heads step on the exact chosen label, while shared-prefix / identical non-decider merges stay aligned with the selected-branch projection without fabricating a spurious local head step (`r[choreography.steps]`)
- [x] Add named step proof artifacts in `src/choreography_step.rs` and add `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `choreography.steps`
- [x] Add unit/property tests covering communication reduction, branch selection, missing-label / wrong-head `None` cases, and unchanged uninvolved roles

## Phase 4: Cross-Module Properties

- [x] `src/properties.rs`: add `proof fn choreography_projection_duality(...)` as the defining artifact for `r[properties.choreography-duality]`, with explicit code-side `r[impl]` / `r[verify]` anchors and tests
- [x] `src/properties.rs`: add `proof fn choreography_choice_label_agreement(...)` as the defining artifact for `r[properties.choreography-choice-agreement]`, proving exact decider label-set agreement and exact first-exposed-offer label-set agreement through common-prefix merges, with explicit code-side `r[impl]` / `r[verify]` anchors and tests
- [x] `src/properties.rs`: add `proof fn choreography_step_projection_correspondence(...)` as the defining artifact for `r[properties.choreography-step-correspondence]`, proving exposed local heads step when present and shared-prefix / identical non-decider views already equal the selected continuation projection otherwise, with explicit code-side `r[impl]` / `r[verify]` anchors and tests
- [x] Add `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `properties.choreography-duality`, `properties.choreography-choice-agreement`, and `properties.choreography-step-correspondence`
- [x] Add focused example tests for a two-role ping choreography, a three-role branching choreography, and a common-prefix-before-offer choreography; each test must carry the needed `r[impl]` / `r[verify]` anchors and must assert structural projection equality and one-step continuation equality, not just that a validator accepts the input

## Phase 5: Validation

- [x] Run `./scripts/check-ensures.sh` and capture output in `openspec/changes/choreography-primitives/evidence/validation-check-ensures.txt`
- [x] Run `cargo test` and capture output in `openspec/changes/choreography-primitives/evidence/validation-cargo-test.txt`
- [x] Run `cargo clippy --all-targets -- -D warnings` and capture output in `openspec/changes/choreography-primitives/evidence/validation-clippy.txt`
- [x] Run `./scripts/verus.sh src/lib.rs` and capture output in `openspec/changes/choreography-primitives/evidence/validation-verus.txt`
- [x] Run `tracey query uncovered` and capture output in `openspec/changes/choreography-primitives/evidence/validation-tracey-uncovered.txt`
- [x] Run `tracey query untested` and capture output in `openspec/changes/choreography-primitives/evidence/validation-tracey-untested.txt`
- [x] Run `./scripts/verify.sh` and capture output in `openspec/changes/choreography-primitives/evidence/validation-verify.txt`
- [x] Require empty uncovered / untested Tracey output before completion unless a reviewer-approved disposition is recorded in `openspec/changes/choreography-primitives/evidence/tracey-disposition.md`
- [x] If `choreography_step_projection_correspondence` cannot be closed without a new trust boundary, stop and spin a follow-up OpenSpec change instead of weakening this one with assumptions
