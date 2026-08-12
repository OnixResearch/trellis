## Why

Trellis already verifies many finite bit-level, arithmetic, control, and circuit-adjacent primitives, but those verified decisions currently stay inside Rust APIs. Hardware users cannot reuse the same checked semantics as small FPGA/ASIC blocks without hand-translating them into RTL and manually preserving the proof boundary.

This change adds a narrow hardware-output slice: a finite Trellis hardware IR, deterministic Verilog/SystemVerilog emission, generated simulation/formal collateral, and validation evidence that selected Trellis primitives lower only through the admitted IR subset. The goal is reusable verified micro-IP for bit operations and small finite state machines, not a general Rust-to-HDL compiler.

## What Changes

- Add a finite hardware IR for synthesizable combinational expressions, explicit registers, clocks, resets, and small FSM transitions.
- Add pure IR evaluation semantics and checked adapters from selected Trellis primitives into that IR.
- Add deterministic Verilog and SystemVerilog emitters whose output is derived only from admitted IR nodes.
- Add generated testbench/property collateral for equivalence against the Rust/IR evaluator.
- Add validation tasks for Verilator simulation and Yosys/SymbiYosys-style syntax/formal checks when those tools are available.
- Start with bitwise, bitfield, simple arithmetic, and small FSM-style admission helpers; defer heap-backed collections, distributed protocols, crypto wrappers, async runtime behavior, and unbounded vectors.

## Impact

- **Files**: expected source modules such as `src/hdl_ir.rs`, `src/hdl_emit.rs`, `src/hdl_adapters.rs`, `docs/requirements.md`, `cairn/specs/hardware-output/spec.md`, legacy OpenSpec sync if still required, README, crate docs, validation scripts, and this Cairn package.
- **APIs**: new finite hardware-output helpers and explicit adapter entry points rather than blanket HDL emission for every public Trellis function.
- **Testing**: positive and negative unit/property tests for IR admission, evaluator semantics, emitter determinism, rejected unsupported shapes, generated testbench fixtures, and external-tool validation where available; Verus proof checks; Tracey coverage; Cairn validation/gates.
