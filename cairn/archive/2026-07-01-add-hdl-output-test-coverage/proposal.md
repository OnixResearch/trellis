## Why

The HDL/SystemVerilog backend will only be useful if rejected shapes fail closed and emitted RTL can be checked against the same Trellis semantics that Verus proves. A backend that emits plausible-looking Verilog without requirement-linked positive and negative tests would create a new trust boundary instead of extending Trellis's verified-logic workflow.

This change codifies the test and validation strategy for the hardware-output backend. It requires a requirement-driven matrix, evaluator-as-oracle checks, golden emitter fixtures, adapter property tests, fail-closed rejection tests, external-tool evidence, and Tracey/Cairn coverage before any HDL support claim is accepted.

## What Changes

- Add explicit hardware-output testing requirements for positive and negative coverage per backend requirement.
- Require the pure hardware IR evaluator to be the semantic oracle for generated vectors and RTL validation.
- Require deterministic golden tests for Verilog and SystemVerilog emitters.
- Require adapter property tests that compare Trellis source helpers to lowered IR over finite input spaces.
- Require fail-closed tests for unsupported IR nodes, unsupported adapter candidates, malformed inputs, and unavailable external tools.
- Require validation evidence that distinguishes pass, fail, and skipped-tool outcomes.
- Require Tracey `r[verify ...]` markers and Cairn gate evidence for the authored test suite.

## Impact

- **Files**: expected test modules, golden HDL fixtures, generated-vector fixtures, validation scripts or harnesses, `docs/requirements.md`, `cairn/specs/hardware-output/spec.md`, legacy OpenSpec sync if still required, README validation notes, and this Cairn package.
- **APIs**: no new public HDL generation API is required by this change; it constrains the test and evidence surfaces for the backend change.
- **Testing**: adds required positive/negative test coverage, property tests, golden-output tests, external-tool validation evidence, and Tracey/Cairn gate checks for hardware-output work.
