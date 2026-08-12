# seL4 VSpace Mapping Specification

## Purpose

Defines requirements for verified `sel4_vspace` semantics in this repo. Scope is virtual memory mapping behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: VSpace Mappings Are Aligned And Non-Overlapping

r[sel4-vspace.map]
Installing a mapping into a VSpace MUST respect object alignment and MUST NOT create overlapping live mappings at the same address-space root.

#### Scenario: Frame maps at aligned address

- GIVEN a page-sized frame and aligned virtual address
- WHEN map is invoked
- THEN the mapping is valid

#### Scenario: Overlapping mapping rejected

- GIVEN an existing live mapping covering virtual range `R`
- WHEN another mapping would overlap `R`
- THEN the map operation is invalid

### Requirement: VSpace Unmap Removes Exactly One Mapping

r[sel4-vspace.unmap]
Unmapping MUST remove exactly the targeted mapping and leave all unrelated mappings unchanged.

#### Scenario: Unmap one mapping preserves others

- GIVEN two disjoint live mappings `M1` and `M2`
- WHEN `M1` is unmapped
- THEN `M2` remains live

### Requirement: Mapping Rights Are Restricted By Cap Rights

r[sel4-vspace.rights]
Effective mapping rights MUST be no greater than the rights carried by the capability used to install the mapping.

#### Scenario: Read-only cap yields read-only mapping

- GIVEN a frame capability without write authority
- WHEN it is mapped
- THEN the effective mapping is not writable
