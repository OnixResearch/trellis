## ADDED Requirements

### Requirement: Hardware Output Test Matrix

r[hardware-output.testing.matrix]
The system MUST maintain a requirement-linked hardware-output test matrix. Each hardware-output backend requirement MUST map to at least one positive test or proof artifact and at least one negative test or proof artifact, unless the requirement explicitly documents why one side is inapplicable. Each mapped artifact MUST carry a Tracey `r[verify ...]` marker for the covered requirement.

#### Scenario: Every requirement has positive and negative coverage

- GIVEN the set of hardware-output backend requirement IDs
- WHEN the test matrix is checked
- THEN each requirement has at least one positive and one negative mapped artifact or an explicit inapplicability rationale

#### Scenario: Missing negative coverage is rejected

- GIVEN a hardware-output requirement with only happy-path tests
- WHEN the test matrix is checked
- THEN the matrix check fails and names the missing negative coverage

### Requirement: Hardware Output Fixture Traceability

r[hardware-output.testing.fixtures]
The system MUST use deterministic fixture naming and metadata for hardware-output tests. Source IR fixtures, golden HDL fixtures, generated-vector fixtures, and validation evidence MUST identify the requirement ID, emitted module name, emitter mode, and expected oracle source so reviewers can trace each artifact back to the requirement it verifies.

#### Scenario: Fixture identifies requirement and mode

- GIVEN a generated HDL golden fixture
- WHEN fixture metadata is inspected
- THEN it names the requirement ID, module under test, and Verilog or SystemVerilog mode

#### Scenario: Untraceable fixture is rejected

- GIVEN a hardware-output fixture without a requirement ID or emitter mode
- WHEN fixture validation runs
- THEN the fixture is rejected as untraceable evidence

### Requirement: Evaluator Oracle Tests

r[hardware-output.testing.evaluator-oracle]
The system MUST test the pure hardware IR evaluator as the semantic oracle for backend validation. Positive tests MUST cover admitted combinational modules, registered modules, and FSM steps. Negative tests MUST cover missing inputs, incompatible widths, invalid current states, malformed admitted-data boundaries, and any evaluator rejection that should block generated collateral.

#### Scenario: Evaluator oracle covers admitted modules

- GIVEN admitted combinational, registered, and FSM hardware IR fixtures
- WHEN evaluator oracle tests run
- THEN each fixture produces deterministic expected outputs or next states

#### Scenario: Evaluator rejection blocks downstream evidence

- GIVEN an evaluator input with a missing port assignment or invalid FSM state
- WHEN evaluator oracle tests run
- THEN the evaluator returns explicit rejection and no expected-output fixture is accepted for that case

### Requirement: Emitter Golden Tests

r[hardware-output.testing.emitter-golden]
The system MUST test Verilog and SystemVerilog emitters with deterministic golden-output fixtures. Golden tests MUST cover canonical module ordering, port ordering, declaration ordering, expression formatting, combinational assignments, registered update blocks, reset behavior, reserved identifier escaping or rejection, and unsupported-node rejection. Repeated emission of the same admitted IR MUST produce byte-identical output.

#### Scenario: Repeated emission matches golden text

- GIVEN an admitted hardware IR fixture and its checked-in golden HDL output
- WHEN the emitter runs repeatedly in the fixture's mode
- THEN every emitted output is byte-identical to the golden fixture

#### Scenario: Unsupported emitter node produces no partial golden output

- GIVEN a hardware IR fixture containing a node unsupported by the requested emitter mode
- WHEN the emitter test runs
- THEN it observes an explicit unsupported-node error and no partial HDL text is accepted

### Requirement: Adapter Property Tests

r[hardware-output.testing.adapter-property]
The system MUST property-test each supported Trellis-to-hardware adapter over finite admitted input domains. Adapter property tests MUST compare the source Trellis helper result with the lowered hardware IR evaluator result, and MUST include edge values for zero, maximum representable values, width boundaries, state boundaries, and rejected out-of-domain candidates.

#### Scenario: Adapter agrees with source helper

- GIVEN a supported adapter and a generated finite input inside its admitted domain
- WHEN the source helper and lowered hardware IR evaluator both run
- THEN their observable outputs are equal

#### Scenario: Out-of-domain adapter input is rejected

- GIVEN a generated input outside an adapter's admitted finite domain
- WHEN adapter property tests attempt lowering or evaluation
- THEN the adapter returns explicit rejection instead of producing an IR result

### Requirement: Fail-Closed Backend Tests

r[hardware-output.testing.fail-closed]
The system MUST test fail-closed behavior for the hardware-output backend. Unsupported adapters, malformed IR, unsupported emitter constructs, evaluator rejections, invalid validation inputs, and unavailable output destinations MUST produce explicit errors and MUST NOT write partial HDL, partial testbenches, partial vectors, or misleading success evidence.

#### Scenario: Unsupported adapter writes no HDL

- GIVEN a Trellis helper outside the supported hardware adapter subset
- WHEN hardware-output generation is requested
- THEN the request returns explicit rejection and no HDL or collateral artifact is produced

#### Scenario: Invalid validation input records failure

- GIVEN generated HDL collateral whose expected-vector metadata is malformed
- WHEN validation evidence generation runs
- THEN the result is an explicit validation failure and not a pass or skipped result

### Requirement: External Tool Evidence Classification

r[hardware-output.testing.tool-evidence]
The system MUST classify external HDL tool validation outcomes as pass, fail, or skipped. Tool evidence MUST name the tool, command or invocation class, input artifact, result classification, and reason for failure or skip. Missing Verilator, Yosys, or SymbiYosys MUST be recorded as skipped with an unavailable-tool reason and MUST NOT be reported as success.

#### Scenario: Available tool records pass or fail

- GIVEN generated HDL and an installed supported HDL validation tool
- WHEN the validation wrapper runs that tool
- THEN evidence records a pass or fail result tied to the input artifact and tool name

#### Scenario: Missing tool records skipped

- GIVEN generated HDL and no installed supported HDL validation tool
- WHEN the validation wrapper attempts that tool class
- THEN evidence records a skipped result with an unavailable-tool reason

### Requirement: Hardware Output Tracey and Cairn Gates

r[hardware-output.testing.tracey-gates]
The system MUST require Tracey and Cairn gate evidence before claiming hardware-output test coverage is complete. Hardware-output test completion MUST include focused backend tests, generated fixture checks, Verus proof checks for touched verified modules, Tracey uncovered and untested queries, Cairn validation, Cairn proposal/design/tasks gates, and cargo/clippy checks as applicable to touched Rust code.

#### Scenario: Tracey untested coverage blocks completion

- GIVEN a hardware-output requirement without a corresponding `r[verify ...]` artifact
- WHEN Tracey untested coverage is checked
- THEN the change is not considered complete

#### Scenario: Cairn gates pass before archive

- GIVEN completed hardware-output testing artifacts and evidence
- WHEN Cairn validation and proposal/design/tasks gates run
- THEN archive readiness requires those gates to pass for the current tree
