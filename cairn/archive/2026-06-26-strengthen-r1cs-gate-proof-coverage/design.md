## Context

`r1cs_gates` models a deliberately finite adapter from multiplication, addition, and constant gates into the existing bounded single-row R1CS surface. The runtime helpers already prove spec/exec equality. The remaining weakness is reviewability: the scanner correctly flags the catch-all coverage proof because `ensures true` does not connect the proof artifact to the gate behaviours named by the requirement.

## Decisions

### 1. Strengthen coverage in proof postconditions, not trust

**Choice:** Replace the catch-all `ensures true` coverage proof with postconditions that quantify over gates and witnesses and restate the concrete rejection implications already provided by the local proof helpers.

**Rationale:** This keeps the proof local, deterministic, and Verus-checkable while giving reviewers an observable non-vacuous coverage artifact. No new external-body, assume-specification, or runtime-exempt surface is needed.

### 2. Add finite negative tests for missing runtime boundaries

**Choice:** Add focused cargo tests for unsupported gate kind rejection, witness length mismatch rejection, malformed witness rejection, and valid-but-unsatisfied encoded-row rejection.

**Rationale:** The existing tests cover happy-path multiplication/addition/constant gates and several malformed gates, but these finite negatives make the requirement coverage explicit and guard the exact cases named by the proof-review advisory.

## Risks / Trade-offs

- Quantified proof postconditions can become brittle if they inline too much row arithmetic. The mitigation is to phrase them as implications over existing local specs and let small helper proof functions establish each implication.
- This does not widen gate semantics or add practical proving-system features; scope remains the bounded R1CS adapter only.
