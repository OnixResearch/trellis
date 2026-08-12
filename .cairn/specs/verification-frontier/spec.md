# Verification Frontier Specification

## Purpose

Defines the `verification-frontier` capability.

## Requirements

### Requirement: Contract Implication Admission

The system MUST provide a finite contract-implication model that classifies a callee precondition under caller facts, path facts, and type-bound facts as `Proven`, `Rejected`, or `Unknown`. `Proven` MUST mean every modeled assignment satisfying the available facts satisfies the callee precondition. `Rejected` MUST mean the modeled facts expose a violating assignment or a directly false precondition. `Unknown` MUST remain non-proof evidence and MUST NOT be treated as check-elision evidence by core helpers.

r[verification-frontier.contract-implication]

#### Scenario: Caller facts imply callee precondition

- GIVEN caller facts that strictly imply a callee precondition in the finite model
- WHEN contract implication admission is evaluated
- THEN the outcome is `Proven`

#### Scenario: Counterexample rejects admission

- GIVEN caller facts that admit an assignment violating the callee precondition
- WHEN contract implication admission is evaluated
- THEN the outcome is `Rejected` with deterministic violation metadata

#### Scenario: Missing facts are not proof

- GIVEN a callee precondition that depends on facts absent from the finite model
- WHEN contract implication admission is evaluated
- THEN the outcome is `Unknown` and no proof/elision claim is emitted

### Requirement: Type-Bound Fact Extraction

The system MUST expose deterministic type-bound fact extraction for modeled integer and boolean domains. Extracted facts MUST use named domain constants, MUST distinguish signed and unsigned domains, and MUST compose with caller facts without widening a caller-provided stricter bound.

r[verification-frontier.type-bound-facts]

#### Scenario: Unsigned type contributes nonnegative lower bound

- GIVEN a modeled unsigned integer parameter
- WHEN type-bound facts are extracted
- THEN the facts include a nonnegative lower bound for that parameter

#### Scenario: Narrow type contributes finite upper bound

- GIVEN a modeled finite-width integer or boolean parameter with a known representable maximum
- WHEN type-bound facts are extracted
- THEN the facts include the named finite upper bound for that domain

#### Scenario: Stricter caller bound is preserved

- GIVEN type-bound facts and a stricter caller-supplied range for the same parameter
- WHEN the fact set is normalized
- THEN the normalized facts preserve the stricter range rather than widening it

### Requirement: Slice Access Certificate

The system MUST provide a finite slice-access certificate model for a backing length, slice start, slice end, and fixed or bounded dynamic access offset. Admission MUST accept only certificates that prove every modeled access lies within the slice and the slice lies within the backing length. Inverted ranges, out-of-backing ranges, and unbounded dynamic offsets MUST be rejected or classified as `Unknown` rather than accepted.

r[verification-frontier.slice-access-certificate]

#### Scenario: Fixed in-slice offset is admitted

- GIVEN a backing length, a slice range within that backing length, and a fixed access offset inside the slice range
- WHEN slice-access admission is evaluated
- THEN the certificate is accepted

#### Scenario: Inverted slice range is rejected

- GIVEN a slice start greater than the slice end
- WHEN slice-access admission is evaluated
- THEN the certificate is rejected with deterministic range metadata

#### Scenario: Dynamic offset needs a bound

- GIVEN a dynamic access offset without a modeled upper bound
- WHEN slice-access admission is evaluated
- THEN the outcome is `Unknown` or rejected, and it is not accepted as safe access evidence

### Requirement: Arena Scope Provenance

The system MUST provide an arena scope-provenance model where arenas, allocations, allocator wrappers, and containers carry a scope depth or explicit untracked provenance. A value allocated from a deeper local scope MUST NOT be returned from a shallower boundary or stored into a shallower-lived destination. Values allocated from caller-provided arenas MAY be returned when their provenance is no deeper than the caller boundary. Untracked heap-backed values MUST NOT be treated as arena-tainted unless an explicit provenance fact says so.

r[verification-frontier.arena-scope-provenance]

#### Scenario: Local arena allocation cannot escape by return

- GIVEN an allocation whose provenance depth is deeper than the function boundary
- WHEN return-escape admission is evaluated
- THEN the escape is rejected

#### Scenario: Caller arena allocation can be returned

- GIVEN an allocation whose provenance depth is no deeper than the caller boundary
- WHEN return-escape admission is evaluated
- THEN the escape is admitted

#### Scenario: Container inherits arena allocator depth

- GIVEN a container constructed from an allocator wrapper that inherits an arena's provenance depth
- WHEN provenance is computed for the container
- THEN the container carries the inherited arena depth

#### Scenario: Deeper value cannot be stored into shallower destination

- GIVEN a source value with deeper provenance than the destination
- WHEN store-escape admission is evaluated
- THEN the store is rejected

### Requirement: Progress Budget Admission

The system MUST provide a finite progress-budget admission model over task or control-flow summaries. The model MUST use named operation-cost constants and named budget values. A summary MUST be accepted only when every modeled acyclic path reaches yield or return within the configured budget and no modeled cycle can avoid yield or return indefinitely.

r[verification-frontier.progress-budget]

#### Scenario: All paths stay within budget

- GIVEN a finite task summary where every modeled path to yield or return has cost within the configured budget
- WHEN progress-budget admission is evaluated
- THEN the summary is accepted

#### Scenario: Over-budget path is rejected

- GIVEN a finite task summary with at least one modeled path whose cost exceeds the configured budget
- WHEN progress-budget admission is evaluated
- THEN the summary is rejected with deterministic path metadata

#### Scenario: No-yield cycle is rejected

- GIVEN a finite task summary with a modeled cycle that can avoid yield or return indefinitely
- WHEN progress-budget admission is evaluated
- THEN the summary is rejected

### Requirement: External Reference Boundary

The system MUST document Lattice/Salt as an external design reference only when explaining these adapted verification-frontier patterns. Documentation and evidence MUST NOT treat Lattice code, benchmarks, or solver behavior as trusted validation for Trellis. Trellis claims MUST be backed by repo-local specs, proofs, tests, Tracey coverage, and validation logs.

r[verification-frontier.external-reference-boundary]

#### Scenario: Reference is labeled as inspiration

- GIVEN documentation mentions Lattice/Salt in this adaptation
- WHEN the documentation is reviewed
- THEN it labels the repository as an external design reference or inspiration, not as trusted proof evidence

#### Scenario: Local evidence backs Trellis claims

- GIVEN a Trellis claim about an adapted verification-frontier primitive
- WHEN validation evidence is reviewed
- THEN the claim is backed by Trellis-local tests, proofs, coverage, or validation logs

#### Scenario: External benchmark is not copied as guarantee

- GIVEN Lattice/Salt publishes a benchmark or runtime-cost claim
- WHEN Trellis documentation describes the adapted primitive
- THEN the external measurement is not copied as a Trellis guarantee

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
