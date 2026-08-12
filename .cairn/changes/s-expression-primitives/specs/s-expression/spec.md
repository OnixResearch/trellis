# S-expression Specification

## Purpose

Defines finite canonical S-expression primitives for Trellis Verus proofs. The requirements cover AST shape, atom admission, canonical serialization, bounded parse/admission behavior, roundtrip facts, explicit size/depth budgets, evidence coverage, documentation sync, validation, and non-claim boundaries.

## Requirements

### Requirement: S-expression AST Shape
r[s-expression.ast] The system MUST define a finite S-expression AST whose nodes are either admitted atoms or finite lists of admitted S-expressions.

#### Scenario: Atom node is admitted
r[s-expression.ast.atom]
- GIVEN an atom that satisfies the declared atom admission predicate
- WHEN S-expression AST admission is evaluated
- THEN the atom node is accepted as a finite S-expression.

#### Scenario: List node is admitted
r[s-expression.ast.list]
- GIVEN a finite list whose children are admitted S-expressions
- WHEN S-expression AST admission is evaluated
- THEN the list node is accepted as a finite S-expression and its children remain ordered.

#### Scenario: Malformed child is rejected
r[s-expression.ast.reject-child]
- GIVEN a list containing a child that does not satisfy S-expression admission
- WHEN S-expression AST admission is evaluated
- THEN the list node is rejected with deterministic child-admission metadata.

### Requirement: S-expression Atom Admission
r[s-expression.atom-admission] The system MUST define dialect-neutral atom admission rules for canonical atoms, including explicit handling for empty atoms, reserved delimiter characters, whitespace, and escape sequences.

#### Scenario: Canonical atom is accepted
r[s-expression.atom-admission.valid]
- GIVEN an atom whose bytes or characters satisfy the declared canonical atom grammar
- WHEN atom admission is evaluated
- THEN the atom is accepted and can be used in S-expression AST nodes.

#### Scenario: Reserved atom syntax is rejected
r[s-expression.atom-admission.reserved]
- GIVEN an atom containing unescaped whitespace, list delimiters, or another reserved character declared by the atom grammar
- WHEN atom admission is evaluated
- THEN the atom is rejected with deterministic atom-syntax metadata.

#### Scenario: Invalid escape is rejected
r[s-expression.atom-admission.escape]
- GIVEN an atom containing an unterminated escape, unknown escape, or escape sequence outside the declared canonical grammar
- WHEN atom admission is evaluated
- THEN the atom is rejected with deterministic escape metadata.

### Requirement: S-expression Canonical Serialization
r[s-expression.canonical-serialization] The system MUST define deterministic canonical serialization for admitted S-expressions, including atom rendering, list delimiters, child separators, and escape normalization.

#### Scenario: Atom serialization is deterministic
r[s-expression.canonical-serialization.atom]
- GIVEN an admitted atom
- WHEN canonical serialization is evaluated repeatedly
- THEN the same canonical representation is produced each time.

#### Scenario: List serialization preserves child order
r[s-expression.canonical-serialization.list]
- GIVEN an admitted list of S-expression children
- WHEN canonical serialization is evaluated
- THEN the result contains the canonical child serializations in the same order, enclosed by canonical list delimiters and separated only by the declared canonical separator.

#### Scenario: Equivalent non-canonical spellings normalize or reject
r[s-expression.canonical-serialization.normalization]
- GIVEN input text that uses a spelling outside the canonical rendering rules
- WHEN parse/admission is evaluated
- THEN the input is either rejected with deterministic non-canonical metadata or normalized to the unique canonical serialization declared by this spec.

### Requirement: S-expression Parse Admission
r[s-expression.parse-admission] The system MUST define bounded parse/admission behavior for the canonical S-expression grammar with deterministic success and failure metadata.

#### Scenario: Canonical text parses to an AST
r[s-expression.parse-admission.valid]
- GIVEN canonical text for an admitted S-expression within the declared budgets
- WHEN parse admission is evaluated
- THEN parsing succeeds and returns the corresponding admitted AST.

#### Scenario: Unbalanced delimiters are rejected
r[s-expression.parse-admission.unbalanced]
- GIVEN text with an opening delimiter without a matching close delimiter, a close delimiter without a matching open delimiter, or mismatched nesting
- WHEN parse admission is evaluated
- THEN parsing fails with deterministic delimiter metadata.

#### Scenario: Trailing input is rejected
r[s-expression.parse-admission.trailing]
- GIVEN text that contains a complete S-expression followed by additional non-separator input outside the declared grammar
- WHEN parse admission is evaluated
- THEN parsing fails with deterministic trailing-input metadata.

### Requirement: S-expression Roundtrip
r[s-expression.roundtrip] The system MUST prove or mechanically check that every admitted finite S-expression roundtrips through canonical serialization and canonical parse admission.

#### Scenario: Canonical serialize then parse returns original value
r[s-expression.roundtrip.serialize-parse]
- GIVEN an admitted finite S-expression within the declared parse and serialization budgets
- WHEN the value is canonically serialized and the result is parsed by canonical parse admission
- THEN parsing succeeds with the original S-expression value.

#### Scenario: Re-serialization is stable
r[s-expression.roundtrip.stable-render]
- GIVEN canonical text that parses to an admitted S-expression
- WHEN that parsed S-expression is serialized canonically
- THEN the output equals the unique canonical representation for that value.

### Requirement: S-expression Size and Depth Budgets
r[s-expression.budgets] The system MUST expose explicit finite size and depth budgets for recursive S-expression traversals, parsing, serialization proofs, and rejection of under-budget inputs.

#### Scenario: Budget-admitted value is accepted
r[s-expression.budgets.accept]
- GIVEN an admitted S-expression whose node count and nesting depth are within the declared budgets
- WHEN budget admission is evaluated
- THEN the value is accepted for bounded traversal and parsing claims.

#### Scenario: Size budget exhaustion is rejected
r[s-expression.budgets.size-reject]
- GIVEN an S-expression or input text whose admitted node count would exceed the declared size budget
- WHEN budget admission or parse admission is evaluated
- THEN the candidate is rejected with deterministic size-budget metadata.

#### Scenario: Depth budget exhaustion is rejected
r[s-expression.budgets.depth-reject]
- GIVEN an S-expression or input text whose nesting depth would exceed the declared depth budget
- WHEN budget admission or parse admission is evaluated
- THEN the candidate is rejected with deterministic depth-budget metadata.

### Requirement: S-expression Positive Evidence
r[s-expression.evidence.positive] The implementation MUST include positive evidence for atom admission, nested list admission, empty list admission, canonical serialization, escaped atom handling, budget-admitted traversal, and canonical parse/serialize roundtrips.

#### Scenario: Positive fixtures cover accepted behavior
r[s-expression.evidence.positive.fixtures]
- GIVEN the S-expression test suite
- WHEN positive fixtures run
- THEN accepted examples cover every public S-expression primitive family.

### Requirement: S-expression Negative Evidence
r[s-expression.evidence.negative] The implementation MUST include negative evidence for invalid atoms, invalid escapes, unbalanced delimiters, trailing input, malformed nesting, non-canonical input when rejected by the chosen surface, size-budget exhaustion, and depth-budget exhaustion.

#### Scenario: Negative fixtures cover failure behavior
r[s-expression.evidence.negative.fixtures]
- GIVEN the S-expression test suite
- WHEN negative fixtures run
- THEN malformed examples are rejected with deterministic metadata rather than panicking, looping, silently accepting unsupported syntax, or exporting an unsupported proof claim.

### Requirement: S-expression Documentation Sync
r[s-expression.docs] The implementation MUST update Trellis `docs/requirements.md`, source annotations, test annotations, README/module-catalog text, and accepted Cairn spec documentation for any public S-expression module or requirement IDs that land in source.

#### Scenario: Requirement IDs are traceable
r[s-expression.docs.traceable]
- GIVEN an S-expression public module lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in `docs/requirements.md`, source annotations, tests, and accepted Cairn spec text.

### Requirement: S-expression Validation Evidence
r[s-expression.final-validation] The change MUST capture focused validation evidence for Cargo tests, Verus verification, Tracey uncovered/untested checks, and Cairn validation/gates before archive.

#### Scenario: Final validation names commands and outcomes
r[s-expression.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final validation evidence is inspected
- THEN it names the commands run, their outcomes, and any scoped limitations or blockers.

### Requirement: S-expression Non-Claim Boundary
r[s-expression.non-claims] The S-expression primitive MUST state that it does not prove evaluator semantics, macro expansion, symbol hygiene, interning, module loading, bytecode safety, filesystem behavior, network behavior, process behavior, whole-consumer correctness, Steel compatibility, Scheme compatibility, Common Lisp compatibility, SMT-LIB compatibility, or parser completeness for any external dialect.

#### Scenario: Tree identity does not imply evaluation semantics
r[s-expression.non-claims.no-eval]
- GIVEN a consumer cites S-expression canonicalization evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite tree admission, canonical serialization, bounded parse admission, and roundtrip facts, and does not imply any expression evaluates correctly.

#### Scenario: External dialect syntax stays out of scope
r[s-expression.non-claims.no-dialect]
- GIVEN input text accepted by an external S-expression dialect but outside the Trellis canonical grammar
- WHEN Trellis S-expression evidence is reviewed
- THEN the evidence does not claim that input is accepted, normalized, or semantically equivalent unless this spec explicitly admits it.
