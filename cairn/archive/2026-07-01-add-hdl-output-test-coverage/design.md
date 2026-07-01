## Context

The hardware-output backend has two separate semantic boundaries: Trellis-to-IR lowering and IR-to-HDL text emission. Tests must catch mistakes on both sides. External HDL tools are important validation aids, but they must not become the source of truth for semantics or silently pass when unavailable.

## Decisions

### 1. Drive tests from requirement IDs

**Choice:** Maintain a test matrix that maps every hardware-output requirement to at least one positive test and one negative test, with Tracey `r[verify ...]` markers on the authored tests or proof artifacts.

**Rationale:** Requirement-linked coverage prevents the backend from accumulating untested success paths or untested rejection paths.

### 2. Treat the IR evaluator as the oracle

**Choice:** Generated vectors, testbench expectations, adapter equivalence checks, and simulator comparisons all derive expected values from the pure hardware IR evaluator.

**Rationale:** The evaluator is the small deterministic semantic core Trellis can test and verify. Verilator, Yosys, or SymbiYosys validate emitted artifacts against that core, but they do not define the core semantics.

### 3. Use golden tests for emitter determinism

**Choice:** Verilog and SystemVerilog emitters must have stable golden-output fixtures for representative combinational modules, registered modules, FSM modules, reset behavior, reserved identifiers, and rejected unsupported shapes.

**Rationale:** HDL consumers depend on stable text for review and evidence. Golden fixtures catch nondeterministic ordering and accidental formatting drift.

### 4. Use property tests for adapter refinement

**Choice:** Bitwise, bitfield, simple arithmetic, and FSM adapters must use finite generated inputs to compare the source Trellis helper with the lowered IR evaluator. Edge cases such as zero, maximum values, width boundaries, and invalid states must be included.

**Rationale:** Adapter bugs are likely to hide at width and boundary edges. Property tests complement small unit examples without requiring unbounded hardware claims.

### 5. Test fail-closed behavior as a first-class feature

**Choice:** Unsupported adapter candidates, malformed IR, unsupported emitter nodes, evaluator rejection, missing inputs, and invalid external-tool configurations must be tested as explicit errors that produce no partial RTL or misleading pass evidence.

**Rationale:** For a code generator, refusing to emit invalid output is as important as emitting correct output.

### 6. Record tool outcomes explicitly

**Choice:** Tool-backed validation must record `pass`, `fail`, or `skipped` with the tool name and reason. Missing Verilator, Yosys, or SymbiYosys must be a skipped result, not a successful result.

**Rationale:** Reviewers need to distinguish semantic test success from unavailable infrastructure.

### 7. Keep validation shells thin

**Choice:** Test selection, expected-vector generation, and evidence classification are pure deterministic helpers. Shell scripts or test harnesses only invoke tools, collect stdout/stderr, and write evidence files.

**Rationale:** This preserves Trellis's functional-core / imperative-shell style and keeps the test logic unit-testable without external processes.

## Non-Goals

- Implementing the HDL backend itself; this change specifies and tracks its test/evidence obligations.
- Making external HDL tools mandatory for ordinary unit tests on machines where they are unavailable.
- Exhaustive testing of every Trellis primitive.
- Timing closure, place-and-route evidence, vendor FPGA integration, or synthesis-optimization preservation proofs.
- Treating generated HDL text equality as a substitute for evaluator equivalence.

## Risks / Trade-offs

- Golden fixtures can become noisy if formatting is not canonical; the emitter should use one stable document layout before broad fixture coverage is added.
- Tool-backed checks may be skipped locally, so archive evidence should include at least one environment where available tools actually run before claiming release-quality HDL support.
- Property tests must keep finite domains explicit to avoid implying support for widths or states outside the admitted IR subset.
