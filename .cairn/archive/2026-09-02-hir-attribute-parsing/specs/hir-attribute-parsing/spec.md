# HIR Attribute Parsing Delta

## ADDED Requirements

### Requirement: Attribute Parser Envelope
r[hir-attr.parser-envelope] The system MUST define finite parser descriptors for inert attributes, including attribute name, parser class, target policy, duplicate policy, safety policy, stability gate, and argument template metadata.

#### Scenario: Registered parser is admitted
r[hir-attr.parser-envelope.valid]
- GIVEN a parser descriptor with a declared inert attribute name and complete policy metadata
- WHEN parser-descriptor admission is evaluated
- THEN the descriptor is accepted.

#### Scenario: Incomplete parser descriptor is rejected
r[hir-attr.parser-envelope.incomplete]
- GIVEN a parser descriptor missing required policy or template metadata
- WHEN parser-descriptor admission is evaluated
- THEN the descriptor is rejected with deterministic parser metadata.

### Requirement: Attribute Parser Classes
r[hir-attr.parser-classes] The system MUST distinguish no-args, single, combine, and generic parser classes and enforce each class's occurrence and argument-shape rules.

#### Scenario: No-args parser accepts empty attribute
r[hir-attr.parser-classes.no-args]
- GIVEN a no-args parser descriptor and an occurrence with no arguments
- WHEN parser admission is evaluated
- THEN the occurrence is accepted.

#### Scenario: Single parser rejects duplicate
r[hir-attr.parser-classes.single-duplicate]
- GIVEN a single-attribute parser descriptor and two occurrences on the same target
- WHEN parser admission is evaluated
- THEN duplicate handling rejects the input unless the descriptor explicitly permits replacement.

#### Scenario: Combine parser merges occurrences
r[hir-attr.parser-classes.combine]
- GIVEN a combine parser descriptor and multiple admitted occurrences
- WHEN parser admission is evaluated
- THEN the occurrences are combined into one deterministic parsed attribute metadata value.

### Requirement: Attribute Policy Checks
r[hir-attr.policy] The system MUST enforce allowed target, duplicate, safety wrapper, and stability policy for every admitted parsed attribute.

#### Scenario: Allowed target is accepted
r[hir-attr.policy.allowed-target]
- GIVEN an attribute occurrence on a target admitted by its parser descriptor
- WHEN target policy is evaluated
- THEN the target is accepted.

#### Scenario: Disallowed target is rejected
r[hir-attr.policy.bad-target]
- GIVEN an attribute occurrence on a target outside the descriptor's allowed target set
- WHEN target policy is evaluated
- THEN the occurrence is rejected with deterministic target metadata.

#### Scenario: Missing unsafe wrapper is rejected
r[hir-attr.policy.unsafe-wrapper]
- GIVEN an attribute whose safety policy requires an unsafe wrapper and an occurrence without that wrapper
- WHEN safety policy is evaluated
- THEN the occurrence is rejected.

#### Scenario: Unstable attribute requires gate
r[hir-attr.policy.stability]
- GIVEN an unstable attribute occurrence and no matching feature gate metadata
- WHEN stability policy is evaluated
- THEN the occurrence is rejected.

### Requirement: Attribute Admission Helpers
r[hir-attr.admission] Runtime helpers MUST reject unknown parsers, malformed arguments, invalid targets, duplicate policy violations, missing safety wrappers, and stability violations with deterministic reason codes.

#### Scenario: Malformed arguments are rejected
r[hir-attr.admission.malformed-args]
- GIVEN an occurrence whose argument shape does not match the parser template
- WHEN parser admission is evaluated
- THEN the occurrence is rejected with deterministic argument metadata.

#### Scenario: Unknown parser is rejected
r[hir-attr.admission.unknown]
- GIVEN an inert attribute name absent from the finite parser registry
- WHEN parser admission is evaluated
- THEN the occurrence is rejected or classified as unsupported according to the declared policy.

### Requirement: Early and Late Parse Agreement
r[hir-attr.early-late-agreement] When both limited early parsing and later HIR parsing are declared for an attribute, successful parsed metadata from both phases MUST agree.

#### Scenario: Matching early and late parses are accepted
r[hir-attr.early-late-agreement.match]
- GIVEN early and late parse metadata that both succeed with the same parsed value
- WHEN agreement is evaluated
- THEN the pair is accepted.

#### Scenario: Divergent early and late parses are rejected
r[hir-attr.early-late-agreement.diverge]
- GIVEN early and late parse metadata that both succeed but produce different parsed values
- WHEN agreement is evaluated
- THEN the pair is rejected.

### Requirement: Attribute Evidence Coverage
r[hir-attr.evidence] The implementation MUST include positive and negative evidence for parser descriptors, parser classes, policy checks, admission failures, early/late agreement, and deterministic diagnostics.

#### Scenario: Positive fixtures cover parser behavior
r[hir-attr.evidence.positive.fixtures]
- GIVEN the HIR attribute parsing test suite
- WHEN positive fixtures run
- THEN no-args, single, combine, generic, target, stability, safety, and early/late agreement cases are covered.

#### Scenario: Negative fixtures cover parser failures
r[hir-attr.evidence.negative.fixtures]
- GIVEN the HIR attribute parsing test suite
- WHEN negative fixtures run
- THEN bad target, duplicates, missing unsafe wrapper, missing feature gate, malformed arguments, unknown parser, and early/late disagreement are rejected deterministically.

### Requirement: Attribute Documentation Sync
r[hir-attr.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public HIR attribute parsing helpers.

#### Scenario: Requirement IDs are traceable
r[hir-attr.docs.traceable]
- GIVEN a public attribute parsing helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Attribute Validation Evidence
r[hir-attr.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[hir-attr.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Attribute Non-Claim Boundary
r[hir-attr.non-claims] The HIR attribute parsing primitive MUST NOT claim active attribute or macro expansion correctness, proc-macro behavior, full token parsing, rustc diagnostic rendering, Rust feature stabilization policy, or whole-frontend correctness.

#### Scenario: Parser evidence is not macro evidence
r[hir-attr.non-claims.no-macro]
- GIVEN a consumer cites attribute parsing evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite inert parsed attribute admission and not to active macro expansion.
