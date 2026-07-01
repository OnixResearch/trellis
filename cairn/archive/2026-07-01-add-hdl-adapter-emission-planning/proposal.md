## Why

The HDL adapter layer already checks that supported Trellis primitives refine the hardware IR evaluator, but backend support also needs a fail-closed handoff from a refined adapter to an emitter mode. Without an explicit planning step, a refined adapter could be mistaken for emitted HDL support even when no deterministic Verilog/SystemVerilog fixture exists for that adapter and width.

## What Changes

- Add a hardware-output adapter-emission planning requirement for the currently emitted adapter subset.
- Add a pure planning helper that requires adapter eligibility, finite-domain refinement, emitter-mode support, and successful fail-closed emission before returning an emission request.
- Add positive tests for supported adapter/mode plans and negative tests for unsupported adapter kinds, unsupported modes, and out-of-domain FSM inputs.
- Sync accepted requirements and run focused HDL tests plus Tracey/Cairn validation evidence.

## Impact

- **Files**: `src/hdl_adapters.rs`, `docs/requirements.md`, `cairn/specs/hardware-output/spec.md`, README metrics if required, and this Cairn package.
- **APIs**: adds a pure adapter-emission planning helper and explicit planning error enum.
- **Testing**: focused HDL cargo tests, Tracey uncovered/untested, and Cairn gates for this change.
