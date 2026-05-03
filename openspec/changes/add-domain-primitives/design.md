## Context

The repository already grows verified coverage by adding small primitive modules,
explicit validity boundaries, and Tracey/OpenSpec coverage before composing
higher-level facades. Domain-heavy systems need the same pattern for spatial,
numerical, physical, and perception data: most bugs are boundary bugs around
shape, range, indexing, and stale/invalid decisions.

## Goals / Non-Goals

**Goals:**

- define a first suite of Verus-friendly domain primitives for GIS, ML, physics,
  and machine vision
- keep each primitive small enough to prove with local invariants, bounded
  arithmetic, and deterministic state transitions
- prefer checked constructors/checkers for public runtime surfaces accepting
  external input
- make every validity boundary, rejection semantics, and proof/test obligation
  explicit before implementation

**Non-Goals:**

- proving floating-point numerical accuracy or model quality
- binding to large external engines/libraries
- adding application-level composite facades in this change
- weakening existing repo verification gates or adding runtime-only exemptions

## Decisions

### 1. Treat domain primitives as foundations, not composites

**Choice:** Add a new `domain-primitives` spec domain instead of extending the
existing `composite-primitives` suite.

**Rationale:** GIS/ML/physics/vision helpers are source primitives. They should
establish local facts such as bounded coordinates, valid shapes, safe windows,
and conservative envelopes before any future composite facade consumes them.

**Alternative:** Add them as another batch of composite primitives. Rejected
because there are no stable source modules to compose yet.

### 2. Use explicit validity-boundary classes

**Choice:** Each primitive must choose one of three boundaries:

- checked runtime constructor/checker returns `Option`/decision for invalid input
- proof-only helper with explicit `requires`
- hybrid public exec wrapper around a proof-only core

**Rationale:** Reviewers can then distinguish caller-supplied invalid data from
logic-internal facts, and rejection/no-mutation behavior can be tested.

### 3. Avoid claims about numerical accuracy beyond discrete envelopes

**Choice:** Specs focus on bounded integer/rational/discrete representations,
finite-state checks, ordering, containment, and conservative admission.

**Rationale:** Verus proofs are strongest for exact logical invariants. Floating
point error bounds, geodesic accuracy, and ML quality claims require separate
numeric-model work.

### 4. Group implementation by proof shape

**Choice:** Implement in four families but prove one module at a time:

1. bounded ranges and rectangular/shape layout,
2. safe index/window traversal,
3. conservative accept/reject state transitions,
4. derived metrics with denominator/overflow checks.

**Rationale:** This keeps proof obligations local and reusable. It also lets
future composites depend on a stable subset if one family lands earlier.

## Risks / Trade-offs

**Over-broad primitives** → Keep each requirement tied to one module and one
primary exported safety predicate.

**Numeric ambiguity** → State scaled-integer or discrete model boundaries in
requirements; leave real-valued accuracy proofs out of scope.

**Tracey drift** → Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and
`r[verify]` tags in the same implementation slice as each module.

**Runtime regression** → Prefer iterative exec paths for scans/window traversal;
use recursive specs/proofs only for mathematical models.

## Validation Plan

For each module:

- add requirement text and Tracey anchors
- add public exec or proof-only API per the requirement boundary
- add positive and negative tests for runtime admission/rejection
- run focused cargo tests for the module
- run `./scripts/verus.sh src/lib.rs` after the proof slice
- capture final `cargo test --quiet`, Verus, Tracey uncovered, and Tracey
  untested evidence before archive
