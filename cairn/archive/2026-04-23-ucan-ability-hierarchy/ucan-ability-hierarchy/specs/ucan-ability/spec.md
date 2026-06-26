# UCAN Ability Hierarchy Specification

## Purpose

Defines the landed string-based hierarchical ability model used by the
UCAN ability module. Abilities are represented as slash-separated path
segments, with a distinguished top ability `*` and single-level `*`
matching inside multi-segment patterns.

## Definitions

- **Ability**: A sequence of byte-string path segments such as
  `["crud", "read"]`.
- **Top ability**: The distinguished pattern `["*"]`, which subsumes any
  ability regardless of depth.
- **Glob segment**: Within a non-top multi-segment pattern, `"*"` matches
  exactly one segment.
- **Subsumption**: `ability_subsumes_spec(pattern, ability)`.
- **Containment**: In the landed module,
  `ability_contains_spec(parent, child) == ability_subsumes_spec(parent, child)`.
- **Namespace**: The first segment of an ability path.

## Requirements

### Subsumption Properties

#### Requirement: Ability Subsumption Is Reflexive
`r[ucan-ability.subsumption-reflexive]`

A concrete non-glob ability MUST subsume itself.

##### Scenario: Exact match
`r[ucan-ability.subsumption-reflexive.scenario-exact-match]`

- GIVEN ability `["crud", "read"]`
- WHEN checking `ability_subsumes_spec(["crud", "read"], ["crud", "read"])`
- THEN the result is true

#### Requirement: Ability Subsumption Is Transitive
`r[ucan-ability.subsumption-transitive]`

If pattern `A` subsumes pattern `B` and `B` subsumes ability `C`, then
`A` MUST subsume `C`.

##### Scenario: Top and namespace glob compose transitively
`r[ucan-ability.subsumption-transitive.scenario-top-and-glob]`

- GIVEN `ability_subsumes_spec(["*"], ["crud", "*"])`
- AND `ability_subsumes_spec(["crud", "*"], ["crud", "read"])`
- WHEN transitivity is applied
- THEN `ability_subsumes_spec(["*"], ["crud", "read"])` holds

#### Requirement: Top Ability Subsumes All
`r[ucan-ability.top-subsumes-all]`

The distinguished top ability `["*"]` MUST subsume every ability,
regardless of path depth.

##### Scenario: Star matches any ability
`r[ucan-ability.top-subsumes-all.scenario-star-matches-any-ability]`

- GIVEN any ability path
- WHEN checking subsumption against the top ability `["*"]`
- THEN the result is true

### Namespace Isolation

#### Requirement: Cross-Namespace Never Subsumes
`r[ucan-ability.namespace-isolation]`

Two non-top abilities in different top-level namespaces MUST NOT subsume
each other.

##### Scenario: msg vs crud
`r[ucan-ability.namespace-isolation.scenario-msg-vs-crud]`

- GIVEN `["msg", "send"]` and `["crud", "read"]`
- WHEN checking subsumption in either direction
- THEN the result is false

### Glob Matching

#### Requirement: Single Glob Matches One Level
`r[ucan-ability.glob-one-level]`

Inside a non-top multi-segment pattern, a single `"*"` MUST match
exactly one path segment and MUST NOT match deeper nesting.

##### Scenario: One level matches
`r[ucan-ability.glob-one-level.scenario-one-level-match]`

- GIVEN pattern `["crud", "*"]` and ability `["crud", "read"]`
- WHEN checking subsumption
- THEN the result is true

##### Scenario: Nested rejected
`r[ucan-ability.glob-one-level.scenario-nested-rejected]`

- GIVEN pattern `["crud", "*"]` and ability `["crud", "nested", "deep"]`
- WHEN checking subsumption
- THEN the result is false

### Containment

#### Requirement: Ability Containment Is Monotone
`r[ucan-ability.containment-monotone]`

If parent ability contains child ability, then everything the child
subsumes is also subsumed by the parent.

##### Scenario: Delegation narrows ability
`r[ucan-ability.containment-monotone.scenario-delegation-narrows-ability]`

- GIVEN parent ability `["crud", "*"]` and child ability `["crud", "read"]`
- WHEN the child subsumes `["crud", "read"]`
- THEN the parent also subsumes `["crud", "read"]`