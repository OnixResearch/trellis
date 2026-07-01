## Context

Trellis is a verified Rust library, not a hardware construction language. The viable hardware boundary is therefore an explicit finite IR that Trellis can evaluate, verify, and emit, plus a small set of adapters that prove selected public semantics lower into that IR. Directly compiling arbitrary Rust or Verus syntax to RTL would make synthesis rules implicit, pull in unsupported runtime features, and obscure the proof boundary.

## Decisions

### 1. Use an explicit hardware IR instead of Rust-to-HDL compilation

**Choice:** Define a Trellis-owned hardware IR with fixed-width values, booleans, muxes, comparisons, arithmetic/bitwise operators, explicit registers, named module ports, and finite FSM transitions.

**Rationale:** An explicit IR keeps synthesis eligibility inspectable and testable. Unsupported Rust features are rejected before emission instead of being approximated in generated RTL.

### 2. Keep the functional core pure

**Choice:** IR admission, IR evaluation, adapter lowering, and text emission are pure deterministic functions over in-memory data. Filesystem writes, external Verilator/Yosys invocations, and evidence capture stay in thin scripts or CLI shells.

**Rationale:** Trellis proofs and tests can exercise all semantic decisions without standing up external tools. Tool execution remains validation evidence, not the source of truth for semantics.

### 3. Start with synthesizable finite combinational logic plus small FSMs

**Choice:** The first supported subset includes fixed-width combinational expressions and explicitly clocked registers/FSMs with declared reset behavior. Every width, port, state, and transition is explicit in the IR.

**Rationale:** These are the Trellis primitives most likely to become useful hardware IP while staying compatible with Verus proof style and ordinary RTL synthesis constraints.

### 4. Emit deterministic Verilog and SystemVerilog

**Choice:** The emitter produces stable text from canonical IR ordering, with a Verilog compatibility mode and a SystemVerilog mode using `logic`, `always_comb`, and `always_ff` where supported.

**Rationale:** SystemVerilog is clearer for new users and assertions, while plain Verilog keeps compatibility with older synthesis/simulation flows.

### 5. Generate collateral, not just modules

**Choice:** For each emitted module, generate optional simulation vectors and assertion/property snippets that compare RTL behavior to the Rust/IR evaluator on the same finite cases.

**Rationale:** Downstream users need evidence that generated RTL still matches the verified Trellis model after emission and tool parsing.

### 6. Reject unsupported shapes explicitly

**Choice:** Heap allocation, unbounded `Vec`, recursion in generated hardware, network/IO/time behavior, crypto trust-boundary execution, async callbacks, distributed protocols, and spec-only existential predicates are outside the first hardware-output subset.

**Rationale:** These domains either do not map cleanly to finite RTL or would need a separate architecture-specific hardware protocol model before synthesis claims are meaningful.

### 7. Treat literal syslog logging as a separate concern

**Choice:** This change models HDL/SystemVerilog output. Literal syslog/event logging for runtime decisions is not part of the RTL backend and would use a separate audit/logging adapter if needed.

**Rationale:** Mixing operational logs with hardware emission would blur validation requirements and confuse the backend's success criteria.

## Non-Goals

- General Rust, Verus, or MIR-to-HDL compilation.
- Full FIRRTL/CIRCT integration in the first slice.
- Timing closure, placement, routing, area estimates, or vendor-specific FPGA IP integration.
- Automatic hardware generation for every Trellis module.
- Proving that downstream synthesis tools preserve semantics after arbitrary optimization.
- Literal syslog logging or audit-event transport.
- Cryptographic side-channel, constant-time, or physical implementation guarantees.

## Risks / Trade-offs

- A small IR limits initial coverage, but it makes unsupported cases fail closed and keeps proofs local.
- String emission is easy to make ad hoc; the implementation should use canonical document assembly and golden tests to avoid nondeterministic formatting drift.
- External validation tools may be unavailable on some machines; Trellis tests should still validate the pure evaluator/emitter core, while tool-backed checks record best-effort evidence when installed.
