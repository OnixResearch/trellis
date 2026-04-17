## Why

`verified-logic` already has useful distributed kernels such as vector clocks,
causal-delivery checks, quorums, and snapshots, but it does not yet have a
choreography layer: a finite global protocol model, a projectability check,
and a verified endpoint projection that turns one global interaction program
into per-role local views.

That gap matters because choreographic programming is exactly about moving from
one global communication description to compatible local behavior. The repo is
a good fit for the pure kernels underneath that idea: finite interaction ASTs,
branch-merge rules, projectability predicates, endpoint projection, and
local/global head-step correspondence. Those kernels are reusable outside any
single language implementation.

This change should stay at the verified-logic level. It should not try to land
full code generation, async runtimes, channels, recursive global types, or a
whole choreographic compiler in one pass.

## What Changes

- Add verified finite choreography syntax for point-to-point communication,
  single-decider branching, and termination.
- Add exact participant-set and well-formedness helpers, including self-send
  rejection plus normalized strictly increasing branch-label order.
- Add verified local choreography syntax plus exact merge rules used when
  projecting non-decider roles across branches.
- Add a verified `projectable` predicate and a preconditioned-total verified
  `project_endpoint` helper that returns the exact local behavior for one role.
- Add verified one-step global and local semantics for communication and branch
  selection, exposed as total `Option`-returning head-step helpers with exact
  `Some` / `None` contracts.
- Add named `proof fn` cross-module proof artifacts for projected
  communication duality, exact branch-label agreement through common-prefix
  merges, and one-step projection/reduction correspondence.
- Add `docs/requirements.md` anchors, unit tests, and property tests for the
  new choreography surface.

## Capabilities

### New Capabilities
- `choreography.participants`: Verified participant extraction and branch
  well-formedness for finite choreographies.
- `choreography.communication`: Verified point-to-point communication prefix
  semantics.
- `choreography.merge`: Verified local branch-merge semantics for non-decider
  roles.
- `choreography.projectability`: Verified exact conditions under which a
  finite choreography can be projected, including the exact outer-label-
  preserving merge for non-decider branches.
- `choreography.projection`: Verified endpoint projection from a global
  choreography to one role's local view.
- `choreography.steps`: Verified one-step global and local semantics for
  communication and branch selection.
- `properties.choreography-duality`: Verified sender/receiver duality after
  projection of a communication prefix.
- `properties.choreography-choice-agreement`: Verified label agreement between
  the decider's internal choice and the first branch-dependent non-decider
  offers produced through common-prefix merges.
- `properties.choreography-step-correspondence`: Verified one-step projection /
  reduction correspondence for projectable finite choreographies.

## Impact

- **Files**: new `src/choreography_global.rs`, `src/choreography_local.rs`,
  `src/choreography_projection.rs`, `src/choreography_step.rs`,
  `src/properties.rs` additions, `docs/requirements.md` additions, and this
  OpenSpec change
- **APIs**: intended public surface is a finite global choreography AST, a
  finite local choreography AST, well-formedness helpers, branch-merge helpers,
  `projectable`, preconditioned-total `project_endpoint`, total
  `Option`-returning one-step head helpers, and proof-only cross-module
  lemmas. Payload values stay abstracted to exact `u32` `payload_tag` symbols;
  this change does not model user expressions or runtime transport.
- **Dependencies**: no new runtime dependencies expected; existing sequence /
  set helpers may be reused where they fit
- **Testing**: implementation-completion validation should include fresh
  `./scripts/check-ensures.sh`, `cargo test`,
  `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`,
  `tracey query uncovered`, `tracey query untested`, and `./scripts/verify.sh`
- **Traceability**: `docs/requirements.md` should mirror
  `choreography.participants`, `choreography.communication`,
  `choreography.merge`, `choreography.projectability`,
  `choreography.projection`, `choreography.steps`,
  `properties.choreography-duality`,
  `properties.choreography-choice-agreement`, and
  `properties.choreography-step-correspondence`. Each rule needs a concrete
  `r[impl]` anchor, and proof-style rules also need named `proof fn`
  `r[verify]` artifacts.

## Out of Scope

- Code generation to Rust, Jolie, Java, or any other target language
- Runtime channels, async executors, or network/failure semantics
- Recursion, loops, higher-order choreographies, or parallel composition
- Broadcast, multicast, or dynamic participant creation
- Expression evaluation, payload serialization, or user data semantics beyond
  exact `payload_tag` symbols
- Full deadlock-freedom theorems for unbounded executions; this change only
  commits to the finite one-step projection / reduction kernel
