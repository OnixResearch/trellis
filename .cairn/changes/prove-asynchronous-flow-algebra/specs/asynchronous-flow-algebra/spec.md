# Asynchronous Flow Algebra Specification Delta

## ADDED Requirements

### Requirement: Flow sets have a canonical admitted domain

r[trellis.async_flow.domain] Trellis MUST define a bounded canonical finite integer-set domain with strictly increasing entries, no duplicates, checked lengths, and an executable admission decision for malformed values.

#### Scenario: Canonical set is admitted

r[trellis.async_flow.domain.valid]
- GIVEN a finite integer sequence is strictly increasing, duplicate-free, and within selected bounds
- WHEN domain admission runs
- THEN Trellis MUST admit it as one canonical set value.

#### Scenario: Set is unsorted or contains duplicates

r[trellis.async_flow.domain.invalid]
- GIVEN a finite integer sequence is unsorted, duplicated, oversized, or outside arithmetic bounds
- WHEN domain admission runs
- THEN Trellis MUST reject it before a proof-dependent operation runs.

### Requirement: Set union refines one exact specification

r[trellis.async_flow.set_union] Trellis MUST define one open canonical set-union specification and one executable set-union operation whose postcondition proves exact refinement and canonical output.

#### Scenario: Overlapping sets are united

r[trellis.async_flow.set_union.valid]
- GIVEN two admitted canonical sets overlap
- WHEN executable set union runs
- THEN the result MUST contain exactly every member from either input once
- AND the result MUST remain canonical.

#### Scenario: Input is not admitted

r[trellis.async_flow.set_union.invalid]
- GIVEN either input is not a canonical admitted set
- WHEN the public admission surface is evaluated
- THEN it MUST reject the operation instead of relying on an unstated proof precondition.

### Requirement: Set union has a complete ACI proof profile

r[trellis.async_flow.aci] Verus MUST prove that the production-linked set-union specification is associative, commutative, and idempotent and has the empty set as identity for all admitted values.

#### Scenario: ACI theorem set is validated

r[trellis.async_flow.aci.valid]
- GIVEN arbitrary admitted canonical sets within the selected domain
- WHEN Verus validates the selected theorem set
- THEN associativity, commutativity, idempotence, and empty identity MUST hold without an unrecorded assumption.

#### Scenario: A sampled test is the only evidence

r[trellis.async_flow.aci.sample_only]
- GIVEN unit or property tests pass but the production-linked Verus theorem is missing or stale
- WHEN proof evidence is assembled
- THEN the ACI property MUST remain unsatisfied.

### Requirement: Set union is monotone under subset order

r[trellis.async_flow.monotone] Trellis MUST define the selected canonical subset order and prove reflexivity, antisymmetry, transitivity, union inflation, and union monotonicity over admitted sets.

#### Scenario: Input is below its union

r[trellis.async_flow.monotone.valid]
- GIVEN admitted sets `left` and `right`
- WHEN their union is evaluated
- THEN each input MUST be below the result under the selected subset order.

### Requirement: Finite flow reduction refines set union

r[trellis.async_flow.reduction] Trellis MUST define a finite sequence-of-sets reduction specification and executable reducer whose result is the canonical union of every admitted input set.

#### Scenario: Multiple inputs are reduced

r[trellis.async_flow.reduction.valid]
- GIVEN a bounded finite sequence of admitted canonical sets
- WHEN the executable reducer runs
- THEN its result MUST equal the exact reduction specification.

#### Scenario: Reduction bounds are exceeded

r[trellis.async_flow.reduction.bounds]
- GIVEN the input count, item count, or result size exceeds a selected named bound
- WHEN reduction admission runs
- THEN Trellis MUST reject the reduction before unchecked growth or arithmetic overflow.

### Requirement: Reduction is invariant under admitted permutations

r[trellis.async_flow.permutation] Trellis MUST prove adjacent-swap invariance and supplied-permutation invariance for the finite set-union reduction over admitted input sequences.

#### Scenario: Inputs are reordered

r[trellis.async_flow.permutation.valid]
- GIVEN two admitted input sequences have one validated permutation witness and the same values
- WHEN both sequences are reduced
- THEN their canonical results MUST be equal.

#### Scenario: Permutation witness changes membership

r[trellis.async_flow.permutation.invalid]
- GIVEN a candidate witness drops, adds, or changes one input
- WHEN witness admission runs
- THEN Trellis MUST reject the witness before applying permutation invariance.

### Requirement: Reduction is invariant under duplicate inputs

r[trellis.async_flow.duplicates] Trellis MUST prove that inserting an additional copy of one admitted input set does not change the canonical set-union reduction result.

#### Scenario: One input is duplicated

r[trellis.async_flow.duplicates.valid]
- GIVEN an admitted input sequence and one valid position that duplicates an existing set
- WHEN the original and duplicated sequences are reduced
- THEN their canonical results MUST be equal.

#### Scenario: Duplicate-sensitive operation is substituted

r[trellis.async_flow.duplicates.invalid]
- GIVEN a consumer substitutes an operation without exact idempotence evidence
- WHEN duplicate-invariance evidence is requested
- THEN the set-union theorem MUST NOT satisfy that operation.

### Requirement: Reduction prefixes are monotone

r[trellis.async_flow.prefix] Trellis MUST prove that extending an admitted input prefix cannot remove a result member and that every admitted prefix result is below the complete reduction.

#### Scenario: A prefix grows

r[trellis.async_flow.prefix.valid]
- GIVEN one admitted input sequence and two valid prefix lengths in nondecreasing order
- WHEN both prefixes are reduced
- THEN the earlier result MUST be below the later result under subset order.

#### Scenario: Prefix result is used as effect authority

r[trellis.async_flow.prefix.effect]
- GIVEN prefix monotonicity is proven
- WHEN a consumer presents that theorem as authorization for an irreversible effect
- THEN Trellis evidence MUST reject that claim as outside the theorem scope.

### Requirement: Invalid law evidence remains explicit

r[trellis.async_flow.negative_laws] Trellis MUST retain negative fixtures for convincing but invalid law claims, including equal-timestamp LWW values with distinct payloads, wrong operations, wrong domains, stale sources, and missing assumptions.

#### Scenario: Equal-timestamp LWW is presented as commutative

r[trellis.async_flow.negative_laws.lww]
- GIVEN two LWW values have equal timestamps and distinct payloads under the current second-input tie rule
- WHEN their merge order is reversed
- THEN the results differ
- AND the distinct-timestamp theorem MUST NOT establish general commutativity.

### Requirement: Proof artifacts bind exact meaning

r[trellis.async_flow.evidence] Trellis MUST export proof-artifact rows that bind each flow property to exact requirement, operation, domain, source, revision, verifier, assumptions, result, evidence, and non-claim identities.

#### Scenario: Current proof row is exported

r[trellis.async_flow.evidence.valid]
- GIVEN the production source and selected Verus theorem pass under the pinned verifier cohort
- WHEN proof artifacts are generated
- THEN the exported row MUST bind every exact input needed by a consumer.

#### Scenario: Source changes after proof

r[trellis.async_flow.evidence.stale]
- GIVEN the operation, domain, proof, assumption, or verifier identity changes
- WHEN the prior artifact is validated
- THEN it MUST fail as stale.

### Requirement: Consumer handoff is narrow and immutable

r[trellis.async_flow.consumer] Trellis MUST publish a narrow consumer profile for Choregraph that identifies the exact runtime API, proof rows, source revision, assumptions, license, and non-claims.

#### Scenario: Choregraph selects the exact profile

r[trellis.async_flow.consumer.valid]
- GIVEN Choregraph pins the immutable Trellis revision and complete flow-algebra profile
- WHEN consumer compatibility is evaluated
- THEN it MAY cite the selected local laws after its own linkage validation.

#### Scenario: Consumer cites a theorem name only

r[trellis.async_flow.consumer.unbound]
- GIVEN a consumer cites a law by display name without the selected source and evidence identities
- WHEN compatibility is evaluated
- THEN the reference MUST fail.

### Requirement: Dissertation use is selective and recorded

r[trellis.async_flow.reference] Trellis MUST record the dissertation URL, selected algebraic stream-admission idea, rejected mechanisms, and non-transferred assumptions or claims.

#### Scenario: Reference scope is reviewed

r[trellis.async_flow.reference.valid]
- GIVEN the flow-algebra design cites the dissertation
- WHEN provenance is reviewed
- THEN Trellis MUST identify the selected concept
- AND Hydro, Stageleft, Katara, and optimizer code MUST NOT enter the trusted base.

### Requirement: Proof authority remains bounded

r[trellis.async_flow.boundary] Passing flow-algebra proofs MUST NOT claim stream closure, transport order, delivery, deduplication storage, exactly-once effects, effect safety, runtime convergence under faults, verifier soundness, production readiness, or release eligibility.

#### Scenario: ACI proof is presented as transport proof

r[trellis.async_flow.boundary.delivery]
- GIVEN all set-union proofs and tests pass
- WHEN evidence is presented as proof of network delivery or runtime deduplication
- THEN Trellis MUST reject that claim as outside proof scope.

### Requirement: Flow-algebra validation is maintained

r[trellis.async_flow.validation] Maintained validation MUST cover positive and negative domain, refinement, ACI, monotonicity, reduction, permutation, duplicates, prefixes, invalid-law, evidence, consumer, reference, and boundary cases.

#### Scenario: A proof or fixture regresses

r[trellis.async_flow.validation.regression]
- GIVEN one theorem, refinement, manifest row, consumer profile, Tracey marker, or negative fixture is missing or stale
- WHEN maintained validation runs
- THEN the validation rail MUST fail before sync or archive.
