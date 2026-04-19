# UCAN Ability Hierarchy Specification

## Purpose

Defines correctness properties for string-based hierarchical ability
namespacing as used by UCAN capabilities. Abilities are slash-separated
paths (e.g., `msg/send`, `crud/read`) with glob matching for delegation.
This is the UCAN-native subsumption surface, parallel to the existing
integer-variant subsumption model.

## Definitions

- **Ability**: A slash-separated path of namespace segments
  (e.g., `"crud/read"`, `"msg/send"`).
- **Ability pattern**: An ability that may contain glob segments
  (`"*"` matches one segment, `"**"` matches zero or more).
- **Top ability**: The pattern `"*"` that subsumes all abilities.
- **Namespace**: The first segment of an ability path
  (e.g., `"crud"` in `"crud/read"`).
- **Subsumption**: Pattern P subsumes ability A if A falls within
  the set of abilities matched by P.

## Requirements

### Subsumption Properties

#### Requirement: Ability Subsumption Is Reflexive

A non-glob ability MUST subsume itself.

`ability_subsumes(a, a) == true` for any concrete ability a.

##### Scenario: Exact match

- GIVEN ability "crud/read"
- WHEN checking ability_subsumes("crud/read", "crud/read")
- THEN true

#### Requirement: Ability Subsumption Is Transitive

If pattern A subsumes pattern B and B subsumes ability C,
then A subsumes C.

`ability_subsumes(a, b) && ability_subsumes(b, c) ==> ability_subsumes(a, c)`

##### Scenario: Star subsumes namespace, namespace subsumes action

- GIVEN ability_subsumes("*", "crud/read") and ability_subsumes("crud/read", "crud/read")
- THEN ability_subsumes("*", "crud/read")

#### Requirement: Top Ability Subsumes All

The top ability pattern `"*"` MUST subsume every ability.

`ability_subsumes(top_ability(), a) == true` for all a.

##### Scenario: Star matches any ability

- GIVEN any ability string
- WHEN checking subsumption against "*"
- THEN true

### Namespace Isolation

#### Requirement: Cross-Namespace Never Subsumes

Abilities in different top-level namespaces MUST NOT subsume
each other (unless one is the top pattern).

`namespace(a) != namespace(b) && a != "*" ==> !ability_subsumes(a, b)`

##### Scenario: msg vs crud

- GIVEN "msg/send" and "crud/read"
- WHEN checking ability_subsumes("msg/send", "crud/read")
- THEN false

### Glob Matching

#### Requirement: Single Glob Matches One Level

A single `*` in an ability pattern MUST match exactly one
path segment, not deeper nesting.

`ability_subsumes("crud/*", "crud/read") == true`
`ability_subsumes("crud/*", "crud/nested/deep") == false`

##### Scenario: One level matches

- GIVEN pattern "crud/*" and ability "crud/read"
- THEN subsumes

##### Scenario: Nested rejected

- GIVEN pattern "crud/*" and ability "crud/nested/deep"
- THEN does not subsume

### Containment

#### Requirement: Ability Containment Is Monotone

If parent ability contains child ability, then everything the
child subsumes is also subsumed by the parent.

`ability_contains(p, c) ==> ∀ op: ability_subsumes(c, op) ==> ability_subsumes(p, op)`

##### Scenario: Delegation narrows ability

- GIVEN parent ability "crud/*" and child ability "crud/read"
- WHEN child subsumes "crud/read"
- THEN parent also subsumes "crud/read"

### Authorization Integration

#### Requirement: String Abilities Satisfy Authorization Decomposition

When using string-based abilities, the authorization check MUST
decompose as `authorizes = ability_subsumes && covers && mode_permits`,
satisfying the same structural contract as integer-variant subsumption.
