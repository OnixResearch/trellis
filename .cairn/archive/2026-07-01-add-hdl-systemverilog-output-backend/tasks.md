## Phase 1: Hardware IR and semantics

- [x] [serial] Add a finite hardware IR with explicit fixed-width values, booleans, module ports, combinational expressions, registers, clocks, resets, and FSM states, with positive and negative tests for admitted and rejected shapes. r[hardware-output.ir]
- [x] [serial] Add pure IR evaluation semantics that deterministically evaluate admitted combinational modules and single-step admitted registered/FSM modules, with tests for width mismatches and invalid state rejection. r[hardware-output.evaluator]

## Phase 2: Trellis adapter subset

- [x] [serial] Add checked adapter metadata for the first supported Trellis primitives, covering bitwise, bitfield, simple arithmetic, and small FSM-style admission helpers while rejecting unsupported heap, IO, async, crypto, and distributed shapes. r[hardware-output.adapter-subset]
- [x] [serial] Prove or test that each supported adapter's IR evaluator result matches the source Trellis spec/exec behavior for the admitted finite inputs, including negative tests for rejected adapter candidates. r[hardware-output.adapter-refinement]

## Phase 3: Verilog/SystemVerilog emission

- [x] [serial] Add deterministic Verilog emission from canonical admitted IR, with golden-output tests and negative tests that unsupported IR nodes do not emit partial RTL. r[hardware-output.verilog-emission]
- [x] [serial] Add deterministic SystemVerilog emission with explicit `logic`, `always_comb`, `always_ff`, reset, and optional assertion snippets, with golden-output tests covering combinational and registered modules. r[hardware-output.systemverilog-emission]

## Phase 4: Generated collateral and tool validation

- [x] [serial] Generate testbench/vector collateral from the IR evaluator so emitted RTL can be simulated against the same finite cases as Rust tests. r[hardware-output.testbench]
- [x] [serial] Add validation scripts for Verilator syntax/simulation and Yosys or SymbiYosys syntax/formal checks when installed, recording unavailable-tool results as explicit skipped evidence instead of passing silently. r[hardware-output.validation]

## Phase 5: Spec, docs, and archive

- [x] [serial] Sync `docs/requirements.md`, `cairn/specs/hardware-output/spec.md`, legacy OpenSpec specs if required, README, and crate docs for the implemented hardware-output surface. r[hardware-output.backend-admission]
- [x] [serial] Run focused HDL-output tests, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn validation/gates, cargo/clippy checks as needed, and archive evidence before completion. r[hardware-output.backend-admission]
