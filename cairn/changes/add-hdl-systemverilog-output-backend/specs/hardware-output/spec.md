## ADDED Requirements

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
