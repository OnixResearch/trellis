# seL4 Capability Derivation Tree Specification

## Purpose

Defines requirements for verified `sel4_cdt` semantics in this repo. Scope is capability ancestry behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Capability Derivation Tree Structure

r[sel4-cdt.structure]
Capability derivation MUST form an acyclic parent-child relation over live capabilities. Copy and mint create a new live child of the source capability, the source capability remains live, and every modeled transition MUST preserve the remaining live relation's acyclic parent-child shape.

#### Scenario: Copy creates child

- GIVEN live capability `P`
- WHEN `P` is copied
- THEN the new capability is a live child of `P`

#### Scenario: Mint creates child

- GIVEN live capability `P`
- WHEN `P` is minted
- THEN the new capability is a live child of `P`

#### Scenario: Copy and mint are structurally identical here

- GIVEN live capability `P`
- WHEN copy or mint creates a derived capability
- THEN this model distinguishes only the new parent-child relation, not rights attenuation

### Requirement: Move Preserves Ancestry

r[sel4-cdt.move]
Moving a capability to a different slot MUST preserve its derivation ancestry and descendant set.

#### Scenario: Move keeps parent relation

- GIVEN capability `C` derived from `P`
- WHEN `C` is moved
- THEN `C` still descends from `P`

#### Scenario: Move preserves descendants

- GIVEN capability `P` with descendant `D`
- WHEN `P` is moved
- THEN `D` remains in the descendant set of `P`

### Requirement: Revoke Removes Exactly The Target Subtree

r[sel4-cdt.revoke]
Revoking a capability MUST remove exactly the live target subtree, including the target root itself, while preserving siblings, ancestors outside that subtree, and unrelated branches.

#### Scenario: Sibling preserved

- GIVEN sibling capabilities `A` and `B` under the same parent
- WHEN `A` is revoked
- THEN `B` remains present

#### Scenario: Descendants removed

- GIVEN capability `P` with live descendants
- WHEN `P` is revoked
- THEN `P` and every live descendant of `P` are removed

### Requirement: Recycle Clears Only A Live Leaf

r[sel4-cdt.recycle]
Recycling a capability MUST succeed only for a live leaf capability and MUST clear exactly that capability while preserving all other live nodes.

#### Scenario: Leaf recycle clears one node

- GIVEN live leaf capability `L`
- WHEN `L` is recycled
- THEN only `L` becomes absent

#### Scenario: Non-leaf recycle fails

- GIVEN live capability `P` with a live child
- WHEN `P` is recycled
- THEN recycle fails and the tree remains unchanged
