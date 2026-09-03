# Mir Drop Elaboration Specification

## Purpose

Defines the `mir-drop-elaboration` capability.

## Requirements

### Requirement: Drop Obligations
r[mir-drop.obligations] The system MUST define finite drop obligations over structural paths and declared type/drop metadata.

#### Scenario: Initialization establishes obligation
r[mir-drop.obligations.establish]
- GIVEN a structural path whose declared type metadata requires drop glue
- WHEN initialization admission is evaluated
- THEN the corresponding drop obligation is established for that path or its declared droppable descendants.

#### Scenario: Move releases obligation
r[mir-drop.obligations.release]
- GIVEN an established drop obligation for a path or its descendant
- WHEN a move from that path is admitted
- THEN the obligation for the moved path and applicable descendants is released.

### Requirement: Initializedness Transitions
r[mir-drop.initializedness] The system MUST model finite initializedness transitions for initialization, move, overwrite, and descendant-path effects used by drop classification.

#### Scenario: Write marks path initialized
r[mir-drop.initializedness.write]
- GIVEN a path admitted for initialization
- WHEN write initialization is evaluated
- THEN the path is recorded as initialized in the finite initializedness state.

#### Scenario: Move marks path uninitialized
r[mir-drop.initializedness.move]
- GIVEN an initialized path admitted for move
- WHEN move admission is evaluated
- THEN the path is recorded as uninitialized for later drop decisions.

### Requirement: Drop Classification
r[mir-drop.classification] The system MUST classify each admitted drop target as `Static`, `Dead`, `Conditional`, or `Open` using finite maybe-initialized and maybe-uninitialized facts.

#### Scenario: Definitely initialized target is static
r[mir-drop.classification.static]
- GIVEN a drop target present in maybe-initialized facts and absent from maybe-uninitialized facts
- WHEN classification is evaluated
- THEN the drop target is classified as `Static`.

#### Scenario: Definitely uninitialized target is dead
r[mir-drop.classification.dead]
- GIVEN a drop target absent from maybe-initialized facts and present in maybe-uninitialized facts
- WHEN classification is evaluated
- THEN the drop target is classified as `Dead`.

#### Scenario: Partly initialized target is open
r[mir-drop.classification.open]
- GIVEN a drop target whose descendants may differ in initializedness
- WHEN classification is evaluated
- THEN the drop target is classified as `Open` unless the declared metadata permits a whole-target conditional classification.

### Requirement: Drop Admission Metadata
r[mir-drop.admission] Runtime helpers MUST reject malformed drop-elaboration inputs with deterministic metadata rather than panicking or silently assuming obligations.

#### Scenario: Malformed structural path is rejected
r[mir-drop.admission.bad-path]
- GIVEN an obligation path whose base or projection is not admitted by MIR core
- WHEN drop-obligation admission is evaluated
- THEN the input is rejected with deterministic path metadata.

### Requirement: Drop Flags
r[mir-drop.drop-flags] The system MUST model drop flags for conditional drops whose whole-target initializedness cannot be resolved statically but does not require open-field expansion.

#### Scenario: Conditional drop is guarded
r[mir-drop.drop-flags.conditional]
- GIVEN a target classified as `Conditional`
- WHEN elaboration metadata is produced
- THEN the drop is guarded by a finite flag representing target initializedness.

#### Scenario: Dead drop has no call
r[mir-drop.drop-flags.dead]
- GIVEN a target classified as `Dead`
- WHEN elaboration metadata is produced
- THEN no drop-call metadata is emitted for that target.

### Requirement: Open Drop Expansion
r[mir-drop.open-drop] The system MUST expand `Open` drops through finite field or enum-variant obligations until each emitted subdrop is static, dead, or conditional, bounded by explicit path-depth or fuel metadata.

#### Scenario: Open struct drops initialized fields
r[mir-drop.open-drop.struct]
- GIVEN an open drop over a struct-like target with initialized and uninitialized droppable fields
- WHEN open-drop expansion is evaluated
- THEN only admitted initialized or conditional droppable fields produce subdrop metadata.

#### Scenario: Inactive enum variant is skipped
r[mir-drop.open-drop.enum]
- GIVEN enum metadata with an active-variant fact
- WHEN open-drop expansion is evaluated
- THEN obligations for inactive variants are skipped or rejected according to the declared active-variant metadata.

#### Scenario: Expansion budget exhaustion rejects claim
r[mir-drop.open-drop.budget]
- GIVEN an open-drop target whose recursive expansion exceeds the declared budget
- WHEN elaboration is evaluated
- THEN the result is rejected with deterministic budget metadata.

### Requirement: DropAndReplace Splitting
r[mir-drop.drop-and-replace] The system MUST split each admitted `DropAndReplace` candidate into deterministic old-target drop behavior followed by replacement write/initialization metadata.

#### Scenario: Replacement follows old drop
r[mir-drop.drop-and-replace.order]
- GIVEN an admitted `DropAndReplace` target and replacement value
- WHEN splitting is evaluated
- THEN old-target drop behavior precedes replacement initialization in the emitted metadata.

#### Scenario: Invalid replacement target is rejected
r[mir-drop.drop-and-replace.invalid]
- GIVEN a `DropAndReplace` target not admitted as a MIR place
- WHEN splitting is evaluated
- THEN the candidate is rejected before replacement metadata is emitted.

### Requirement: Drop-Elaboration Evidence Coverage
r[mir-drop.evidence] The implementation MUST include positive and negative evidence for obligation, initializedness, classification, drop-flag, open-drop, and drop-and-replace behavior.

#### Scenario: Positive fixtures cover elaboration families
r[mir-drop.evidence.positive.fixtures]
- GIVEN the MIR drop-elaboration test suite
- WHEN positive fixtures run
- THEN static, dead, conditional, open, enum, partial-move, and drop-and-replace cases are covered.

#### Scenario: Negative fixtures cover malformed metadata
r[mir-drop.evidence.negative.fixtures]
- GIVEN the MIR drop-elaboration test suite
- WHEN negative fixtures run
- THEN malformed paths, unsupported metadata, inconsistent facts, and budget failures are rejected deterministically.

### Requirement: Drop-Elaboration Documentation Sync
r[mir-drop.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public drop-elaboration helpers.

#### Scenario: Requirement IDs are traceable
r[mir-drop.docs.traceable]
- GIVEN a public drop-elaboration helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Drop-Elaboration Validation Evidence
r[mir-drop.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mir-drop.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Drop-Elaboration Non-Claim Boundary
r[mir-drop.non-claims] The drop-elaboration primitive MUST NOT claim actual destructor side-effect correctness, panic or unwind cleanup correctness, full rustc drop-shim generation, const-eval policy correctness, unsafe-code soundness, or whole-program memory safety.

#### Scenario: Drop metadata is not destructor semantics
r[mir-drop.non-claims.no-destructor]
- GIVEN a consumer cites drop-elaboration evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite drop-decision metadata and not to behavior of user `Drop` implementations.
