## Phase 1: Adapter-emission planning core

- [x] [serial] Add a pure adapter-emission planning helper that returns an HDL emission request only when adapter eligibility, finite-domain refinement, emitted-width support, and emitter-mode support all pass. r[hardware-output.adapter-emission]
- [x] [serial] Add explicit planning errors for unsupported adapter shapes, out-of-domain adapter inputs, unsupported emitted adapter/mode pairs, evaluator rejection, and emitter rejection. r[hardware-output.adapter-emission]

## Phase 2: Positive and negative coverage

- [x] [serial] Add positive tests for emitted bitwise adapter plans in Verilog/SystemVerilog-compatible mode and small-FSM adapter plans in SystemVerilog mode. r[hardware-output.adapter-emission]
- [x] [serial] Add negative tests for refined-but-not-emitted adapters, unsupported emitter modes, unsupported candidates, and invalid FSM states producing explicit rejection and no emission request. r[hardware-output.adapter-emission]

## Phase 3: Specs and validation

- [x] [serial] Sync `docs/requirements.md` and `cairn/specs/hardware-output/spec.md` with the adapter-emission planning requirement. r[hardware-output.adapter-emission]
- [x] [serial] Run focused HDL tests, Tracey uncovered/untested, Cairn validation/gates, and archive the completed change with evidence. r[hardware-output.adapter-emission]
