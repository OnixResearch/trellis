# Filesystem Path Primitives Specification

## Purpose

Defines implemented requirements for reusable finite filesystem-path primitives.
These primitives MUST use exact finite path-segment envelopes, path form
metadata, normalization decisions, root/scope containment predicates, relative
traversal rejection metadata, extension/type metadata, path join/split envelopes,
and deterministic path admission/rejection. They MUST avoid OS filesystem
semantics, realpath/canonicalize behavior, symlink resolution, mount points,
permissions, ACLs, durability, sandbox security, runtime enforcement, TOCTOU,
URI/URL semantics, remote object-store behavior, or external standard-conformance
claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Path Segment Envelope [r[domain-primitives.filesystem-path.segment]]
The system MUST provide finite path-segment envelopes that record segment text or
bytes, separator policy metadata, empty/current/parent segment classifiers, and
deterministic segment admission results.

#### Scenario: Rejected segment is explicit [r[domain-primitives.filesystem-path.segment.scenario.rejected-segment-is-explicit]]

- GIVEN finite path-segment admission metadata
- WHEN a candidate segment is empty, forbidden, or outside the declared segment envelope
- THEN the path-segment primitive returns an explicit rejected-segment decision without consulting an OS filesystem

### Requirement: Path Form Envelope [r[domain-primitives.filesystem-path.form]]
The system MUST provide finite path-form envelopes for absolute, relative,
rooted, empty, and scoped path forms over declared segment sequences.

#### Scenario: Path form is admitted before containment [r[domain-primitives.filesystem-path.form.scenario.path-form-is-admitted-before-containment]]

- GIVEN finite path form metadata and a candidate segment sequence
- WHEN the candidate form is not admitted by the envelope
- THEN the path-form primitive rejects it before containment or lookup decisions are composed

### Requirement: Path Normalization Decision [r[domain-primitives.filesystem-path.normalization]]
The system MUST provide deterministic path-normalization decision envelopes that
record accepted, rejected, unchanged, or rewritten finite segment sequences under
declared normalization rules.

#### Scenario: Normalization rejection preserves original segments [r[domain-primitives.filesystem-path.normalization.scenario.normalization-rejection-preserves-original-segments]]

- GIVEN a finite candidate path and normalization metadata
- WHEN normalization is not admitted
- THEN the normalization primitive records rejection metadata while preserving the original segment sequence

### Requirement: Root Scope Containment Predicate [r[domain-primitives.filesystem-path.containment]]
The system MUST provide finite root/scope containment predicates over declared
root segment prefixes and candidate normalized segment sequences.

#### Scenario: Candidate outside root is rejected [r[domain-primitives.filesystem-path.containment.scenario.candidate-outside-root-is-rejected]]

- GIVEN a finite root scope and candidate normalized path segments
- WHEN the candidate does not have the declared root/scope prefix
- THEN the containment primitive returns an explicit outside-scope decision without claiming sandbox security

### Requirement: Relative Traversal Rejection [r[domain-primitives.filesystem-path.traversal]]
The system MUST provide deterministic relative traversal rejection metadata for
finite path segments that represent parent/current traversal under declared
segment classifier rules.

#### Scenario: Parent traversal is rejected by policy [r[domain-primitives.filesystem-path.traversal.scenario.parent-traversal-is-rejected-by-policy]]

- GIVEN finite path segments and a policy rejecting parent traversal segments
- WHEN a parent traversal segment appears before normalization or admission completes
- THEN the traversal primitive returns explicit rejection metadata without claiming exploit prevention beyond the declared finite predicate

### Requirement: Extension Type Metadata [r[domain-primitives.filesystem-path.extension-type]]
The system MUST provide finite extension/type metadata envelopes that classify
admitted final segments by declared suffix, extension, or type tags without
claiming MIME, content-sniffing, or OS file-type correctness.

#### Scenario: Extension metadata does not imply file contents [r[domain-primitives.filesystem-path.extension-type.scenario.extension-metadata-does-not-imply-file-contents]]

- GIVEN a finite path with admitted extension metadata
- WHEN the extension/type primitive classifies the final segment
- THEN the result records only declared segment metadata and does not claim runtime file contents or MIME correctness

### Requirement: Path Join Split Envelope [r[domain-primitives.filesystem-path.join-split]]
The system MUST provide deterministic path join/split shape envelopes over
finite segment sequences and separator metadata.

#### Scenario: Join split preserves admitted segment order [r[domain-primitives.filesystem-path.join-split.scenario.join-split-preserves-admitted-segment-order]]

- GIVEN finite admitted segment sequences and separator metadata
- WHEN a path join or split shape operation is modeled by the primitive
- THEN the resulting envelope records deterministic segment order without consulting host path APIs

### Requirement: Path Admission Envelope [r[domain-primitives.filesystem-path.admission]]
The system MUST provide deterministic path admission/rejection envelopes that
compose segment, form, normalization, containment, traversal, extension/type, and
join/split decisions over finite path metadata.

#### Scenario: Admission requires all finite path checks [r[domain-primitives.filesystem-path.admission.scenario.admission-requires-all-finite-path-checks]]

- GIVEN finite path metadata and a candidate path
- WHEN any segment, form, normalization, containment, traversal, extension/type, or join/split check rejects
- THEN the path-admission primitive returns an explicit rejection decision before any OS filesystem or security claim is made
