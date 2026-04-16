## Context

This change adds a small verified kernel for choreographic programming, not a
full language implementation. The kernel is intentionally finite and
first-order: explicit roles, explicit communication labels, explicit payload
symbols, a finite global syntax, a finite local syntax, an exact merge
operation, endpoint projection, and one-step semantics.

The design target is the same as the rest of `verified-logic`: small pure
modules with exact executable behavior, matching spec functions, named proof
artifacts, and direct Tracey anchors.

## Goals / Non-Goals

**Goals**
- Represent finite global choreographies with exact communication and choice
  structure.
- Represent finite local choreographies with enough structure to state and
  prove endpoint projection.
- Make projectability exact and checkable: no vague “probably mergeable”
  contract.
- Prove the core projection facts that make the choreography useful to later
  changes: communication duality, branch agreement, and one-step projection /
  reduction correspondence.
- Keep the proof surface compatible with repo norms: explicit `r[impl]` /
  `r[verify]` anchors, small modules, Verus-friendly value types.

**Non-Goals**
- Full endpoint code generation
- Recursive or higher-order choreographies
- Session-type environments, channel implementations, or runtime transports
- Failure-aware semantics or progress results for unbounded executions
- Data-dependent local computation beyond symbolic labels and payload tags

## Decisions

### 1. Use finite ASTs over exact scalar identifiers

**Choice:** represent roles, branch labels, and payload classes as exact `u32`
identifiers. The global syntax is:

```rust
pub type RoleId = u32;
pub type LabelId = u32;
pub type PayloadTag = u32;

pub enum GlobalChoreo {
    End,
    Comm {
        from: RoleId,
        to: RoleId,
        label: LabelId,
        payload_tag: PayloadTag,
        next: Box<GlobalChoreo>,
    },
    Choice {
        decider: RoleId,
        branches: Vec<(LabelId, GlobalChoreo)>,
    },
}
```

**Rationale:** this keeps the kernel concrete and Verus-friendly. It avoids a
large generic/typeclass surface and keeps proof obligations about order,
branch labels, and role equality purely first-order. The `Vec` notation in
this design is semantic shorthand for an exact stored branch order; the
implementation may use ordinary Verus-compatible vectors plus view proofs or a
thin wrapper, but the ordered branch sequence above is the contract.

### 2. Abstract payload semantics to `payload_tag`

**Choice:** the choreography kernel tracks the exact symbolic message metadata
`(label, payload_tag)` but does not model evaluated expressions, runtime
payload values, or serialization.

**Rationale:** endpoint projection and branch agreement depend on message shape
and control flow, not on user expression semantics. Symbolic payload tags give
a useful exact contract without forcing a much larger data-language proof.

### 3. Separate local syntax from global syntax

**Choice:** use a distinct local AST:

```rust
pub enum LocalChoreo {
    End,
    Send {
        peer: RoleId,
        label: LabelId,
        payload_tag: PayloadTag,
        next: Box<LocalChoreo>,
    },
    Recv {
        peer: RoleId,
        label: LabelId,
        payload_tag: PayloadTag,
        next: Box<LocalChoreo>,
    },
    InternalChoice {
        branches: Vec<(LabelId, LocalChoreo)>,
    },
    Offer {
        from: RoleId,
        branches: Vec<(LabelId, LocalChoreo)>,
    },
}
```

`InternalChoice` is the decider-side local form of a global `Choice`. `Offer`
is the non-decider local form when branch projections differ by branch label.
Both `InternalChoice` and `Offer` use branch vectors stored in strict
increasing `LabelId` order.

**Rationale:** projection is clearer when global and local syntax are distinct.
It also gives the change an exact place to state merge rules and duality.

### 4. Compute participants from syntax, not from a separate declaration list

**Choice:** add `participants_spec(choreo)` and executable helpers that compute
exactly the roles mentioned in communication endpoints or as a choice decider.
Roles outside that set are observationally irrelevant and project to `End`.

**Rationale:** this removes a second source of truth and keeps the kernel local
to the choreography value itself.

### 5. Make well-formedness exact before projection

**Choice:** a global choreography is well-formed only if:
- every communication satisfies `from != to`
- every choice has a non-empty branch list
- branch labels are pairwise distinct within one choice and stored in strict
  increasing `LabelId` order
- every branch subtree is itself well-formed

This is encoded in `well_formed_global_spec(...)` and mirrored by executable
validators.

**Rationale:** review will reject a projection contract that hides malformed
cases behind vague prose. These checks define the admitted syntax surface.

### 6. Make merge rules exact and narrow

**Choice:** projection for non-decider roles uses an exact merge helper
`merge_local_spec(decider, left, right)` with these rules:
- `End` merges only with `End`
- identical `Send` heads merge only when `peer`, `label`, and `payload_tag`
  are all equal; the result keeps that exact `Send` head and recursively merges
  continuations
- identical `Recv` heads merge only when `peer`, `label`, and `payload_tag`
  are all equal; the result keeps that exact `Recv` head and recursively merges
  continuations
- identical `InternalChoice` heads merge only when they carry the same
  normalized label set; the result is an exact `InternalChoice` with that same
  ordered label set and recursively merged per-label continuations
- `Recv { peer = decider, label = l1, ... }` and
  `Recv { peer = decider, label = l2, ... }` with `l1 != l2` merge into one
  normalized `Offer { from = decider, branches = [(l1, ...), (l2, ...)] }`
- `Offer { from = decider, ... }` merges with another such `Offer` by normalized
  label-set union: shared labels recursively merge, labels present in only one
  input pass through unchanged, and the result keeps strict increasing label
  order
- every other head conflict fails the merge, including any case where `peer`
  and `label` match but `payload_tag` differs or two `Offer` heads name
  different `from` roles

The executable helper returns `Option<LocalChoreo>` and is total over all
inputs, but `projectable_spec(...)` requires successful merges where needed.
For non-decider choice projection, the merge is projection-specific rather than
a raw left fold on unlabeled locals: it preserves the stored outer branch labels
while stripping any shared local `Send` / `Recv` prefix common to every branch.
When behavior first diverges, it exposes one `Offer` from `decider` whose
branch labels are exactly those stored outer branch labels, and whose branch
bodies are the remaining branch-local continuations. The verified recursion
decreases structurally on the sum of remaining continuation sizes.

**Rationale:** this is strong enough to model common-prefix merging and
choice-derived offers without pulling in a larger session-type environment.
The failure cases are also explicit, which matters for OpenSpec review.

### 7. Define projectability in terms of branchwise local merge

**Choice:** `projectable_spec(global)` means:
- `global` is well-formed
- every branch subtree is projectable
- for each `Choice { decider, branches }` and each non-decider role appearing in
  any branch participant set, the branch projections succeed under the exact
  projection-side merge that preserves outer branch labels through shared
  prefixes and exposes them at the first branch-dependent `Offer`

Projection is exact only over that surface.

**Rationale:** this pins the contract reviewers usually ask for: not just what
projection returns when it works, but exactly when it is allowed to work.

### 8. Keep projection exact and role-directed

**Choice:** `project_endpoint_spec(role, global)` follows exact structural
rules:
- `End -> End`
- `Comm { from, to, label, payload_tag, next }`
  - if `role == from`, project to `Send { peer: to, label, payload_tag, next: project(role, next) }`
  - if `role == to`, project to `Recv { peer: from, label, payload_tag, next: project(role, next) }`
  - otherwise project to `project(role, next)`
- `Choice { decider, branches }`
  - if `role == decider`, project to a normalized `InternalChoice` over
    per-branch projections in branch-label order
  - otherwise project to the exact outer-label-preserving merge of per-branch
    projections in branch-label order, preserving any shared local prefix and
    introducing an `Offer` from `decider` whose branch labels are exactly the
    stored outer branch labels at the first branch-dependent point

Executable `project_endpoint(...) -> LocalChoreo` is preconditioned-total:
it uses `requires projectable_spec(global)` rather than returning `Option`.

**Rationale:** the user-facing value of the change is the exact global-to-local
mapping. This rule set is the core of the change.

### 9. Prove only the finite one-step kernel in this change

**Choice:** expose total `Option`-returning one-step helpers:
- `global_comm_step(global) -> Option<GlobalChoreo>` returns `Some(next)`
  exactly when `global` has a `Comm` head, and `None` otherwise
- `global_choice_step(global, label) -> Option<GlobalChoreo>` returns
  `Some(branch_l)` exactly when `global` has a `Choice` head with branch label
  `label`, and `None` otherwise
- `local_send_step(local) -> Option<LocalChoreo>` and
  `local_recv_step(local) -> Option<LocalChoreo>` return `Some(next)` exactly
  for matching `Send` / `Recv` heads, and `None` otherwise
- `local_internal_choice_step(local, label) -> Option<LocalChoreo>` and
  `local_offer_step(local, label) -> Option<LocalChoreo>` return
  `Some(branch_l)` exactly when the head is `InternalChoice` / `Offer` with
  branch label `label`, and `None` otherwise

For communication steps, uninvolved roles satisfy exact projection equality and
do not fabricate a local head step. For choice steps, the decider and any
currently exposed `Offer` heads step on the chosen label, while roles whose
merge preserved a shared prefix stay aligned with the selected-branch
projection under that same prefix. Named proofs will also establish that every
successful global head step stays on the projectable surface, so
`project_endpoint(_, g')` remains admissible for the post-step choreography.

The key proof target is one-step projection / reduction correspondence, not a
full theorem over arbitrary execution lengths.

**Rationale:** this captures the useful “matching peer exists” kernel without
opening the much larger proof space of unbounded progress and deadlock freedom.

### 10. Keep cross-module theorems in `src/properties.rs`

**Choice:** land three named proof artifacts in `src/properties.rs`:
- `choreography_projection_duality`
- `choreography_choice_label_agreement`
- `choreography_step_projection_correspondence`

Those are the concrete `r[verify]` artifacts for the `properties.*` rules.

**Rationale:** this repo already uses `src/properties.rs` for cross-module
facts. Reusing that pattern keeps the change consistent with existing review
expectations.

## Module Plan

- `src/choreography_global.rs`
  - `GlobalChoreo`
  - `participants_spec`, `participants`
  - `distinct_branch_labels_spec`, `well_formed_global_spec`
  - executable validators and simple structural lemmas
- `src/choreography_local.rs`
  - `LocalChoreo`
  - `merge_local_spec`, `merge_local`
  - label-set helpers for `Offer`
  - named proofs for merge soundness and merge-failure conditions
- `src/choreography_projection.rs`
  - `projectable_spec`, `projectable`
  - `project_endpoint_spec`, `project_endpoint`
  - named proofs for sender/receiver/decider/nonparticipant projection shape
- `src/choreography_step.rs`
  - total `Option`-returning `global_comm_step`, `global_choice_step`
  - total `Option`-returning `local_send_step`, `local_recv_step`
  - total `Option`-returning `local_internal_choice_step`, `local_offer_step`
  - named proofs for exact `Some` / `None` contracts, head-step continuation
    shape, and unchanged-role equality
- `src/properties.rs`
  - the three cross-module theorems above

## Traceability Matrix

- `r[choreography.participants]` -> `src/choreography_global.rs` participant /
  well-formedness helpers + named proofs + tests
- `r[choreography.communication]` -> `src/choreography_global.rs` exact global
  communication-prefix constructors/helpers + tests
- `r[choreography.merge]` -> `src/choreography_local.rs` merge helpers + named
  merge proofs + tests
- `r[choreography.projectability]` -> `src/choreography_projection.rs`
  `projectable` helpers + failure/success proofs + tests
- `r[choreography.projection]` -> `src/choreography_projection.rs`
  `project_endpoint` helpers + shape proofs + tests
- `r[choreography.steps]` -> `src/choreography_step.rs` total `Option`-returning
  one-step helpers + exact `Some` / `None` proofs + continuation / unchanged
  projection proofs + tests
- `r[properties.choreography-duality]` -> `src/properties.rs::choreography_projection_duality` (`r[impl]` and `r[verify]`) + tests
- `r[properties.choreography-choice-agreement]` -> `src/properties.rs::choreography_choice_label_agreement` (`r[impl]` and `r[verify]`) + tests over exact label-set agreement through common-prefix merges
- `r[properties.choreography-step-correspondence]` -> `src/properties.rs::choreography_step_projection_correspondence` (`r[impl]` and `r[verify]`) + tests

## Migration Plan

1. Confirm repo-sync against `src/lib.rs`, `docs/requirements.md`, `README.md`,
   and `openspec/specs/` before landing code.
2. Land `choreography_global` and `choreography_local` first so well-formedness
   and merge are stable before projection.
3. Land `choreography_projection` next, then `choreography_step`.
4. Land `src/properties.rs` theorems after the lower-level modules are stable.
5. Add `docs/requirements.md` anchors in the same phase as the defining code,
   not at the end.
6. Validate with `check-ensures`, tests, clippy, Verus, Tracey, and
   `./scripts/verify.sh`.
7. If `choreography_step_projection_correspondence` cannot be closed without a
   new trust boundary or forces an oversized proof module, stop and split a
   follow-up OpenSpec change instead of weakening this change with assumptions.
