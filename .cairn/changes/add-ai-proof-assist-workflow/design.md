## Context

Trellis produces and audits Verus proof artifacts. Today an agent or engineer writes proof obligations by hand, runs the verifier, and records nothing structured about what was proposed, which verifier ran, or what passed and failed. Nothing stops a generated proof from being accepted on formatting confidence instead of a machine verdict.

MoonBit 0.9 demonstrates the counter-model: the agent proposes, the prover checks, and failed goals become the iteration evidence. The design below binds that model to Trellis and octet evidence conventions.

## Decisions

### Decision: Prover success is the only acceptance authority

**Choice:** accept a candidate only on an explicit pinned-verifier success verdict over recorded inputs.

**Rationale:** generation and formatting confidence cannot authenticate math. A machine verdict is the existing Trellis and octet trust point.

### Decision: Candidate provenance is recorded before checking

**Choice:** every candidate binds proposing run, generator identity, and source hashes at admission time.

**Rationale:** stale or unprovenanced candidates make verdicts unreplayable. The requirement also keeps receipts bounded and deterministic.

### Decision: Core stays pure

**Choice:** obligation admission, provenance binding, verdict normalization, blocker classification, and receipt payload construction are pure deterministic logic in a focused family.

**Rationale:** this preserves the functional-core boundary already enforced in Trellis and octet. Prover invocation and process exit mapping live in the shell.

## Risks / Trade-offs

- Verifier versions drift and invalidate old verdicts; receipts bind verifier and version so staleness fails closed.
- Full solver transcripts are large; receipts exclude them beyond declared bounds.
- Automated loop-invariant search can burn time; campaign bounds cap prover runs per goal.
