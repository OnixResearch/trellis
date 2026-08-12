## Phase 1: Requirement-linked test matrix

- [x] [serial] Add a hardware-output test matrix that maps each backend requirement to positive and negative tests or proof artifacts, including explicit Tracey `r[verify ...]` markers. r[hardware-output.testing.matrix]
- [x] [serial] Add fixture conventions for naming source IR, golden HDL, generated vectors, and validation evidence so reviewers can trace each fixture back to one requirement ID. r[hardware-output.testing.fixtures]

## Phase 2: Evaluator oracle tests

- [x] [serial] Add positive evaluator tests for admitted combinational, registered, and FSM modules, deriving expected outputs and next states from pure IR semantics. r[hardware-output.testing.evaluator-oracle]
- [x] [serial] Add negative evaluator tests for missing inputs, width mismatches, invalid current states, and malformed admitted-data boundaries, asserting explicit errors and no panics. r[hardware-output.testing.evaluator-oracle]

## Phase 3: Emitter golden tests

- [x] [serial] Add deterministic Verilog golden-output tests for canonical port ordering, declaration ordering, expression formatting, combinational assignments, and unsupported-node rejection. r[hardware-output.testing.emitter-golden]
- [x] [serial] Add deterministic SystemVerilog golden-output tests for `logic`, `always_comb`, `always_ff`, reset behavior, assertion snippets, reserved identifiers, and unsupported-node rejection. r[hardware-output.testing.emitter-golden]

## Phase 4: Adapter refinement and fail-closed tests

- [x] [serial] Add property tests for bitwise, bitfield, simple arithmetic, and small FSM adapters comparing Trellis helper results with hardware IR evaluator results over finite input domains and edge values. r[hardware-output.testing.adapter-property]
- [x] [serial] Add fail-closed tests proving unsupported adapters, malformed IR, unsupported emitter constructs, evaluator rejections, and invalid validation inputs produce explicit errors and no partial HDL/collateral artifacts. r[hardware-output.testing.fail-closed]

## Phase 5: External-tool evidence and gates

- [x] [serial] Add tool-backed validation tests or scripts that classify Verilator, Yosys, and SymbiYosys outcomes as pass, fail, or skipped with an unavailable-tool reason. r[hardware-output.testing.tool-evidence]
- [x] [serial] Sync `docs/requirements.md`, `cairn/specs/hardware-output/spec.md`, legacy OpenSpec specs if required, README validation notes, and crate docs for the hardware-output testing requirements. r[hardware-output.testing.tracey-gates]
- [x] [serial] Run focused HDL-output tests, generated fixture checks, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn validation/gates, cargo/clippy checks as needed, and archive evidence before completion. r[hardware-output.testing.tracey-gates]
