# Choreography Cross-Module Properties Specification

## Purpose

Defines the proof-level theorems that tie together finite choreography syntax,
merge, projection, and one-step semantics.

## Requirements

### Requirement: Communication Projection Is Dual

r[properties.choreography-duality]
Depends on `r[choreography.communication]` and `r[choreography.projection]`.
For any projectable communication prefix, projection to the sender and receiver
MUST yield exact dual local heads: the sender sees `Send { peer = receiver, ... }`
and the receiver sees `Recv { peer = sender, ... }` with the same exact
`label` and `payload_tag`.

#### Scenario: Projected communication heads are exact duals

- GIVEN a projectable communication prefix from `A` to `B`
- WHEN projecting to `A` and `B`
- THEN one local head MUST be `Send` to `B`
- AND the other MUST be `Recv` from `A`
- AND both MUST carry the same exact `label` and `payload_tag`

### Requirement: Choice Labels Agree Across Projected Roles

r[properties.choreography-choice-agreement]
Depends on `r[choreography.merge]`, `r[choreography.projectability]`, and
`r[choreography.projection]`.
For any projectable choice with decider `D` and global branch-label set `L`,
the decider's local projection MUST expose exactly `L` as a normalized
`InternalChoice`. Every non-decider role whose branch behavior is identical
across all branches MUST collapse to that exact common local behavior. Every
other non-decider role MUST preserve any exact shared local prefix produced by
the projection-side merge and, at the first branch-dependent point if an
`Offer` from `D` is exposed, MUST expose exactly `L` as a normalized `Offer`
from `D`.

#### Scenario: Decider and first exposed offer share the same label set

- GIVEN a projectable choice with decider `D` and branch labels `L`
- WHEN projecting to `D` and to a non-decider role whose merge eventually
  exposes an `Offer`
- THEN `D`'s local view MUST expose exactly `L`
- AND the first exposed `Offer` from `D` MUST expose exactly `L`

#### Scenario: Shared prefix remains before the first offer

- GIVEN a projectable choice with branch labels `L` whose projection for role
  `R` keeps a common local prefix before branch-dependent receives from `D`
- WHEN projecting to `R`
- THEN the merged local view MUST keep that exact prefix
- AND the first exposed `Offer` from `D`, if any, MUST expose exactly `L`

#### Scenario: Common behavior does not force a useless offer

- GIVEN a projectable choice whose branches are locally identical for role `R`
- WHEN projecting to `R`
- THEN the result MUST collapse to that common local behavior instead of an
  explicit `Offer`

### Requirement: Projected Head Steps Preserve Global Reduction

r[properties.choreography-step-correspondence]
Depends on `r[choreography.projection]` and `r[choreography.steps]`.
If a projectable choreography takes one global head step to `g'`, then the
projected local views MUST stay exactly aligned with `project_endpoint(_, g')`.
Sender / receiver heads and any currently exposed `InternalChoice` / `Offer`
heads MUST take one local head step to those exact endpoint projections. Roles
whose projection already collapses the global head behind a shared prefix or
identical behavior MUST already equal the corresponding endpoint projection
without fabricating a spurious extra local head step. This is the finite
deadlock-by-design kernel promised by this change: projection MUST NOT produce
a first local step that lacks its matching peer or branch label, and it MUST
NOT require a fake local step where merge has already preserved a shared
prefix.

#### Scenario: Communication reduction preserves projected continuations

- GIVEN a projectable communication head that reduces to `g'`
- WHEN stepping the sender and receiver local heads
- THEN they MUST land on exactly the endpoint projections of `g'`

#### Scenario: Branch selection preserves projected branch continuations

- GIVEN a projectable choice that selects branch label `l` and reduces to `g'`
- WHEN stepping the decider and every projected non-decider head that already
  exposes branch label `l`
- THEN those stepped heads MUST land on exactly the endpoint projections of
  `g'`
- AND every remaining projected non-decider view MUST already equal its
  endpoint projection of `g'` because merge preserved a shared prefix or
  identical behavior
