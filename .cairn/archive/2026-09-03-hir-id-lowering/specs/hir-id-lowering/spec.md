# HIR ID Lowering Delta

## Purpose

Defines finite HIR ID-lowering primitives for Trellis. The requirements cover owner scopes, source `NodeId` to `HirId` maps, created-ID use, generated-ID freshness, per-owner uniqueness, definition-correspondence metadata, evidence, documentation, validation, and non-claim boundaries.

## ADDED Requirements

### Requirement: HIR Owner Scope
r[hir-id.owner-scope] The system MUST model HIR owner scopes explicitly and require every admitted HIR identifier to belong to the owner scope in which it is lowered or generated.

#### Scenario: ID in current owner is admitted
r[hir-id.owner-scope.valid]
- GIVEN a HIR identifier whose owner equals the current lowering owner
- WHEN owner-scope admission is evaluated
- THEN the identifier is accepted for that owner.

#### Scenario: Wrong-owner ID is rejected
r[hir-id.owner-scope.wrong-owner]
- GIVEN a HIR identifier created while a different owner is active
- WHEN owner-scope admission is evaluated
- THEN the identifier is rejected with deterministic owner metadata.

### Requirement: NodeId Lowering Map
r[hir-id.lowering-map] Every AST `NodeId` that is placed into an admitted HIR structure MUST appear in the finite lowering map and map to an admitted `HirId` in the owning scope.

#### Scenario: Lowered source node is accepted
r[hir-id.lowering-map.valid]
- GIVEN a source `NodeId` with a lowering-map entry to an in-scope `HirId`
- WHEN source-node admission is evaluated
- THEN the source node is accepted as lowered.

#### Scenario: Unlowered source node is rejected
r[hir-id.lowering-map.missing]
- GIVEN a source `NodeId` referenced by a HIR structure but absent from the lowering map
- WHEN source-node admission is evaluated
- THEN admission is rejected with deterministic missing-lowering metadata.

### Requirement: Created ID Use
r[hir-id.admission] Every created HIR identifier that is intended for a HIR structure MUST be used exactly according to the declared created/used metadata.

#### Scenario: Created ID is used
r[hir-id.admission.used]
- GIVEN a created HIR identifier that appears in the used-ID set for its owner
- WHEN created-ID admission is evaluated
- THEN the identifier satisfies the use requirement.

#### Scenario: Unused created ID is rejected
r[hir-id.admission.unused]
- GIVEN a created HIR identifier absent from the used-ID set and not marked by explicit discard metadata
- WHEN created-ID admission is evaluated
- THEN the lowering record is rejected.

### Requirement: Generated ID Freshness
r[hir-id.fresh-generated] Synthetic HIR nodes introduced during lowering MUST receive fresh generated identifiers that do not alias source-lowered identifiers or other generated identifiers in the same owner.

#### Scenario: Fresh generated node is admitted
r[hir-id.fresh-generated.valid]
- GIVEN a generated HIR identifier absent from the source-lowered range and existing generated-ID set
- WHEN generated-ID admission is evaluated
- THEN the identifier is accepted as fresh.

#### Scenario: Generated ID aliases source ID
r[hir-id.fresh-generated.alias-source]
- GIVEN a generated HIR identifier equal to a source-lowered identifier in the same owner
- WHEN generated-ID admission is evaluated
- THEN the generated node is rejected with deterministic freshness metadata.

### Requirement: Per-Owner Uniqueness
r[hir-id.uniqueness] The admitted lowering record MUST contain no duplicate HIR identifiers within the same owner unless the model declares an explicit shared debug/provenance alias outside HIR structure identity.

#### Scenario: Unique IDs are accepted
r[hir-id.uniqueness.valid]
- GIVEN a lowering record whose HIR structure identifiers are unique within each owner
- WHEN uniqueness admission is evaluated
- THEN the record is accepted.

#### Scenario: Duplicate ID is rejected
r[hir-id.uniqueness.duplicate]
- GIVEN two distinct HIR structure nodes with the same identifier in one owner
- WHEN uniqueness admission is evaluated
- THEN the record is rejected with deterministic duplicate-ID metadata.

### Requirement: Definition Correspondence Metadata
r[hir-id.def-correspondence] If the model declares that a source node introduces a definition, the admitted lowering record MUST expose deterministic source-node to definition metadata for that node.

#### Scenario: Definition source has correspondence
r[hir-id.def-correspondence.valid]
- GIVEN a source node marked as definition-introducing and a matching definition metadata entry
- WHEN definition correspondence is evaluated
- THEN the correspondence is accepted.

#### Scenario: Missing definition correspondence is rejected
r[hir-id.def-correspondence.missing]
- GIVEN a definition-introducing source node without definition metadata
- WHEN definition correspondence is evaluated
- THEN the lowering record is rejected.

### Requirement: HIR ID Evidence Coverage
r[hir-id.evidence] The implementation MUST include positive and negative evidence for owner scope, lowering maps, created-ID use, generated freshness, uniqueness, and definition metadata.

#### Scenario: Positive fixtures cover accepted ID behavior
r[hir-id.evidence.positive.fixtures]
- GIVEN the HIR ID-lowering test suite
- WHEN positive fixtures run
- THEN valid source lowering, generated nodes, owner scopes, and definition metadata are covered.

#### Scenario: Negative fixtures cover invalid ID behavior
r[hir-id.evidence.negative.fixtures]
- GIVEN the HIR ID-lowering test suite
- WHEN negative fixtures run
- THEN wrong-owner, duplicate, unused, unlowered, stale, and missing-definition cases are rejected deterministically.

### Requirement: HIR ID Documentation Sync
r[hir-id.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public HIR ID-lowering helpers.

#### Scenario: Requirement IDs are traceable
r[hir-id.docs.traceable]
- GIVEN a public HIR ID helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: HIR ID Validation Evidence
r[hir-id.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[hir-id.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: HIR ID Non-Claim Boundary
r[hir-id.non-claims] The HIR ID-lowering primitive MUST NOT claim full AST lowering correctness, parser correctness, macro expansion correctness, name resolution, HIR semantic correctness, THIR construction, Rust type checking, or whole-frontend correctness.

#### Scenario: ID evidence is not lowering semantics
r[hir-id.non-claims.id-only]
- GIVEN a consumer cites HIR ID-lowering evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite ID and owner invariants, not semantic equivalence between AST and HIR.
