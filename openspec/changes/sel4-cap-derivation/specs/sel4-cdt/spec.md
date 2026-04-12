# seL4 Capability Derivation Tree Specification

## Purpose

Defines requirements for verified `sel4_cdt` semantics in this repo. Scope is capability ancestry behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Capability Derivation Tree Structure

r[sel4-cdt.structure]
Capability derivation MUST form an acyclic parent-child relation. Copy and mint create children of the source capability, while original capabilities remain roots.

#### Scenario: Copy creates child

- GIVEN capability `P`
- WHEN `P` is copied
- THEN the new capability is a child of `P`

### Requirement: Move Preserves Ancestry

r[sel4-cdt.move]
Moving a capability to a different slot MUST preserve its derivation ancestry and descendant set.

#### Scenario: Move keeps parent relation

- GIVEN capability `C` derived from `P`
- WHEN `C` is moved to a new slot
- THEN `C` still descends from `P`

### Requirement: Revoke Removes Exactly The Target Subtree

r[sel4-cdt.revoke]
Revoking a capability MUST remove descendants of the target subtree while preserving siblings, ancestors, and unrelated branches.

#### Scenario: Sibling preserved

- GIVEN sibling capabilities `A` and `B` under the same parent
- WHEN `A` is revoked
- THEN `B` remains present
