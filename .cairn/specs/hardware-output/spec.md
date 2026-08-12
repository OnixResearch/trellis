# Hardware Output Specification

## Purpose

Defines the `hardware-output` capability.

## Requirements

### Requirement: Hardware IR Admission

r[hardware-output.ir]
The system MUST provide a finite hardware IR for synthesizable module shapes. The IR MUST make every fixed-width value, boolean, module port, combinational expression, register, clock, reset, and FSM state explicit. IR admission MUST reject unsupported or malformed shapes before evaluation or emission, and rejection MUST NOT expose partial RTL output.

#### Scenario: Valid combinational IR is admitted

- GIVEN a hardware IR module with declared input ports, output ports, fixed-width combinational expressions, and no missing width metadata
- WHEN IR admission is evaluated
- THEN the module is admitted for evaluator and emitter use

#### Scenario: Width mismatch is rejected

- GIVEN a hardware IR expression that combines operands with incompatible declared widths and no explicit extension or truncation node
- WHEN IR admission is evaluated
- THEN the module is rejected before evaluation or emission

### Requirement: Hardware IR Evaluator

r[hardware-output.evaluator]
The system MUST provide pure deterministic evaluation semantics for admitted hardware IR modules. The evaluator MUST compute combinational outputs from explicit inputs and MUST compute the next registered or FSM state from explicit current state and input values. Evaluation MUST reject missing inputs, invalid states, and malformed admitted-data boundaries without panicking.

#### Scenario: Combinational evaluation is deterministic

- GIVEN an admitted combinational hardware IR module and a complete input assignment
- WHEN the evaluator is run repeatedly on the same inputs
- THEN it returns the same output assignment each time

#### Scenario: Invalid FSM state is rejected

- GIVEN an admitted registered hardware IR module and a current state value outside the declared FSM state set
- WHEN single-step evaluation is requested
- THEN the evaluator returns an explicit invalid-state rejection

### Requirement: Adapter Subset Admission

r[hardware-output.adapter-subset]
The system MUST expose explicit adapter eligibility metadata for Trellis primitives that may lower to hardware IR. The first supported adapter subset MUST include finite bitwise operations, bitfield operations, simple arithmetic operations, and small FSM-style admission helpers. The adapter layer MUST reject heap allocation, unbounded vectors, recursion intended as generated hardware, network or file IO, async callbacks, distributed protocol execution, crypto trust-boundary execution, and spec-only existential predicates.

#### Scenario: Bitfield adapter is eligible

- GIVEN a Trellis bitfield helper whose inputs and outputs have explicit finite widths
- WHEN adapter eligibility is checked
- THEN the helper is accepted as a candidate for hardware IR lowering

#### Scenario: Heap-backed helper is rejected

- GIVEN a Trellis helper whose runtime behavior depends on heap allocation or unbounded vector length
- WHEN adapter eligibility is checked
- THEN the helper is rejected before IR lowering

### Requirement: Adapter Refinement

r[hardware-output.adapter-refinement]
The system MUST ensure each supported Trellis-to-hardware adapter refines the source Trellis semantics over its admitted finite input space. For every emitted adapter, the hardware IR evaluator result MUST match the source spec or executable helper result for the same admitted inputs, and rejected adapter candidates MUST leave the source primitive unchanged.

#### Scenario: Hardware evaluator matches source helper

- GIVEN an admitted Trellis primitive input and its lowered hardware IR adapter
- WHEN the source helper and hardware IR evaluator are both evaluated on that input
- THEN their observable output values are equal

#### Scenario: Rejected adapter candidate emits nothing

- GIVEN a Trellis primitive outside the supported adapter subset
- WHEN adapter lowering is requested
- THEN lowering returns explicit rejection and no Verilog or SystemVerilog module is produced

### Requirement: Adapter Emission Planning

r[hardware-output.adapter-emission]
The system MUST expose a pure adapter-emission planning decision that returns an HDL emission request only when a Trellis adapter candidate is eligible, its finite input refines the source helper through the hardware IR evaluator, the adapter shape is in the currently emitted subset, and the requested Verilog/SystemVerilog mode supports that emitted shape. Planning MUST fail closed for refined adapters without emitted HDL support, unsupported emitter modes, unsupported adapter candidates, out-of-domain adapter inputs, evaluator rejection, and emitter rejection, and MUST NOT expose a partial HDL request on rejection.

#### Scenario: Supported adapter produces emission request

- GIVEN an eligible Trellis adapter candidate, an admitted finite adapter input, and an emitter mode that supports the adapter's emitted shape
- WHEN adapter-emission planning is evaluated
- THEN the result contains a deterministic HDL emission request and the expected observable value derived from the hardware IR evaluator

#### Scenario: Refined adapter without emitted support is rejected

- GIVEN a Trellis adapter candidate that refines the hardware IR evaluator but has no HDL template in the requested emitter subset
- WHEN adapter-emission planning is evaluated
- THEN planning returns an explicit unsupported-emission rejection and no HDL request

#### Scenario: Out-of-domain adapter input is rejected before emission

- GIVEN an otherwise supported adapter candidate and a finite input outside that adapter's admitted state domain
- WHEN adapter-emission planning is evaluated
- THEN planning returns an explicit out-of-domain rejection and no HDL request

### Requirement: Verilog Emission

r[hardware-output.verilog-emission]
The system MUST emit deterministic Verilog text only from admitted hardware IR modules. Verilog emission MUST use canonical module, port, declaration, assignment, and process ordering so repeated emission of the same IR produces byte-identical output. Verilog emission MUST fail closed for unsupported IR constructs.

#### Scenario: Verilog emission is stable

- GIVEN an admitted hardware IR module
- WHEN Verilog emission is run repeatedly
- THEN every run produces byte-identical Verilog text

#### Scenario: Unsupported construct fails closed

- GIVEN hardware IR containing a construct that the Verilog emitter does not support
- WHEN Verilog emission is requested
- THEN emission returns an explicit unsupported-construct error and no partial module text is exposed

### Requirement: SystemVerilog Emission

r[hardware-output.systemverilog-emission]
The system MUST emit deterministic SystemVerilog text from admitted hardware IR modules. SystemVerilog emission SHOULD use `logic`, `always_comb`, and `always_ff` for supported combinational and registered modules, and MAY emit assertion snippets for admitted safety properties. SystemVerilog emission MUST preserve the same IR semantics as the Verilog emitter for shared supported constructs.

#### Scenario: Registered module emits sequential process

- GIVEN an admitted hardware IR module with explicit clocked registers and reset behavior
- WHEN SystemVerilog emission is requested
- THEN the output contains a deterministic sequential process representing the declared register update and reset behavior

#### Scenario: Shared construct semantics match Verilog mode

- GIVEN an admitted hardware IR module supported by both Verilog and SystemVerilog emitters
- WHEN both emitters run and their outputs are simulated against the same generated vectors
- THEN both outputs match the hardware IR evaluator results

### Requirement: Generated Testbench Collateral

r[hardware-output.testbench]
The system MUST generate optional testbench or vector collateral from the hardware IR evaluator for admitted modules. Generated collateral MUST identify the source module, input assignments, expected outputs or next states, and the emitter mode under test. Collateral generation MUST be deterministic and MUST NOT depend on clocks, filesystem order, random seeds, or external tool output.

#### Scenario: Test vector records expected output

- GIVEN an admitted combinational hardware IR module and a finite input assignment selected for validation
- WHEN testbench collateral generation runs
- THEN the collateral records the input assignment and the expected output from the hardware IR evaluator

#### Scenario: Missing evaluator result blocks collateral

- GIVEN a hardware IR module and validation input that the evaluator rejects
- WHEN testbench collateral generation is requested
- THEN collateral generation returns explicit rejection instead of writing an expected-output fixture

### Requirement: External HDL Validation

r[hardware-output.validation]
The system MUST provide validation scripts or commands that can run Verilator syntax or simulation checks and Yosys or SymbiYosys syntax/formal checks for generated HDL when those tools are available. If a tool is unavailable, validation evidence MUST record an explicit skipped-tool result; it MUST NOT report success for a check that did not run.

#### Scenario: Available simulator checks generated RTL

- GIVEN generated HDL, generated testbench collateral, and an available simulator tool
- WHEN HDL validation runs
- THEN validation records whether simulator execution matched the expected IR evaluator results

#### Scenario: Missing tool is recorded as skipped

- GIVEN generated HDL and no installed supported simulator or synthesis checker
- WHEN HDL validation runs
- THEN validation records the tool as skipped with an unavailable-tool reason

### Requirement: Hardware Backend Admission Composition

r[hardware-output.backend-admission]
The system MUST expose a hardware-output backend admission decision that requires IR admission, adapter subset admission, adapter refinement evidence, deterministic emitter support, generated collateral eligibility, and validation-result recording before claiming that a Trellis primitive has HDL output support. Backend admission MUST NOT imply timing closure, vendor implementation quality, downstream synthesis preservation, or support for modules outside the admitted adapter subset.

#### Scenario: Backend admission requires every check

- GIVEN a Trellis primitive candidate and requested HDL output mode
- WHEN any IR admission, adapter subset, refinement, emitter, collateral, or validation-recording check fails
- THEN backend admission returns explicit rejection and does not claim HDL support

#### Scenario: Accepted backend names its limits

- GIVEN a Trellis primitive that passes every hardware-output backend admission check
- WHEN backend admission succeeds
- THEN the result identifies the emitted HDL mode, supported adapter subset, validation evidence, and explicit non-claims for timing closure and downstream synthesis preservation

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
