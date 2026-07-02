## ADDED Requirements

### Requirement: Outcome Invariant Bundle

The verification-frontier module MUST expose reusable invariant facts for public admission outcomes. For every public admission family, `Proven` MUST imply the modeled safety predicate for that family, `Unknown` MUST remain non-proof evidence, and rejected outcomes MUST NOT be accepted as proof or check-elision evidence.

r[verification-frontier.outcome-invariant-bundle]

#### Scenario: Proven outcome exposes safety fact

- GIVEN a public verification-frontier admission helper returns `Proven`
- WHEN a caller uses the corresponding invariant helper
- THEN the modeled safety predicate for that admission family is available without unfolding the helper implementation

#### Scenario: Unknown outcome is not proof

- GIVEN an admission helper returns `Unknown` because required facts, bounds, or local evidence are missing
- WHEN a caller checks whether the result may justify proof or check elision
- THEN the invariant bundle rejects that use as non-proof evidence

#### Scenario: Rejected outcome stays non-admissible

- GIVEN an admission helper returns `Rejected` with deterministic violation metadata
- WHEN a caller checks whether the result may justify proof or check elision
- THEN the invariant bundle rejects that use and preserves the rejection reason for diagnostics

### Requirement: Admission Metadata Stability

Verification-frontier admission helpers MUST use stable named reason codes and deterministic evidence fields for rejected and unknown outcomes. Evidence fields MUST be derived from modeled inputs or named constants and MUST NOT depend on external benchmarks, wall-clock time, process state, or solver behavior outside the Trellis proof surface.

r[verification-frontier.metadata-stability]

#### Scenario: Unsupported predicate carries stable reason

- GIVEN a contract or slice admission input uses an unsupported predicate or offset kind
- WHEN admission is evaluated
- THEN the result is `Unknown` or `Rejected` with the named unsupported reason code and deterministic input-derived evidence

#### Scenario: Bounds violation carries witness metadata

- GIVEN finite modeled facts expose a lower-bound, upper-bound, backing-range, or budget violation
- WHEN admission is evaluated
- THEN the result is `Rejected` with a stable reason code and evidence identifying the modeled violating value or bound

#### Scenario: External reference evidence is local only

- GIVEN a verification-frontier claim references external Lattice/Salt material
- WHEN external-reference boundary admission is evaluated
- THEN the claim is admitted only when the reference is labeled, local evidence is present, and the external material is not treated as trusted proof or benchmark evidence
