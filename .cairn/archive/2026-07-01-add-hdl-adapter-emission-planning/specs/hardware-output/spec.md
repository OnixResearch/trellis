## ADDED Requirements

### Requirement: Adapter Emission Planning

r[hardware-output.adapter-emission]
The system MUST expose a pure adapter-emission planning decision that returns an HDL emission request only when a Trellis adapter candidate is eligible, its finite input refines the source helper through the hardware IR evaluator, the adapter shape is in the currently emitted subset, and the requested Verilog/SystemVerilog mode supports that emitted shape. Planning MUST fail closed for refined adapters without emitted HDL support, unsupported emitter modes, unsupported adapter candidates, out-of-domain adapter inputs, evaluator rejection, and emitter rejection, and MUST NOT expose a partial HDL request on rejection.

#### Scenario: Supported adapter produces emission request

- GIVEN an eligible Trellis adapter candidate, an admitted finite adapter input, and an emitter mode that supports the adapter's emitted shape
- WHEN adapter-emission planning is evaluated
- THEN the result contains a deterministic HDL emission request and the expected observable value derived from the hardware IR evaluator

#### Scenario: Refined adapter without emitted support is rejected

- GIVEN a Trellis adapter candidate that refines the hardware IR evaluator but has no HDL template in the requested emitter subset
- WHEN adapter-emission planning is evaluated
- THEN planning returns an explicit unsupported-emission rejection and no HDL request

#### Scenario: Out-of-domain adapter input is rejected before emission

- GIVEN an otherwise supported adapter candidate and a finite input outside that adapter's admitted state domain
- WHEN adapter-emission planning is evaluated
- THEN planning returns an explicit out-of-domain rejection and no HDL request
