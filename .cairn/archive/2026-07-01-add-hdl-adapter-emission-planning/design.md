## Context

Adapter refinement and HDL emission are intentionally separate pure cores. Refinement compares Trellis source semantics with hardware IR evaluator semantics, while emission exposes deterministic HDL text only for a small set of example shapes. A backend support claim needs both facts: the adapter must refine the evaluator, and the requested emitter mode must actually support the adapter's emitted shape.

## Decisions

### 1. Add a pure planning core

**Choice:** Introduce an adapter-emission planning helper that takes a candidate, finite adapter input, and requested emitter mode, and returns an `HdlEmitRequest` only after eligibility, refinement, width support, and emitter support have all passed.

**Rationale:** This keeps the side-effect-free admission decision testable without filesystem writes, subprocesses, or external HDL tools.

### 2. Treat emitted support as narrower than refined support

**Choice:** The planning helper accepts only adapter/mode pairs that the current deterministic emitters can represent: fixed-width bitwise-and examples in Verilog or SystemVerilog, and the small FSM toggle example in SystemVerilog.

**Rationale:** Bitfield and wrapping-add adapters may refine the IR evaluator today, but the current emitter fixtures do not expose HDL text for those adapter shapes. They must fail closed rather than inherit a broader support claim.

### 3. Keep errors explicit

**Choice:** Planning distinguishes unsupported adapter shape, out-of-domain adapter input, unsupported emitted adapter/mode pair, evaluator rejection, and emitter rejection.

**Rationale:** Review evidence should show whether failure came from adapter admission/refinement or from HDL emission support.

## Non-Goals

- Adding new Verilog/SystemVerilog templates for every refined adapter.
- Running external synthesis/simulation tools as part of the pure planning helper.
- Claiming timing closure, synthesis preservation, or vendor implementation quality.
