## Context

Trellis already defines algebraic predicates for commutativity, associativity, idempotence, semilattices, monotonicity, and partial orders. It also has sequence-set operations, folds, pointwise lifting, and CRDT examples.

The first Choregraph asynchronous-flow pilot needs a concrete operation with a complete proof chain. Its worker inputs can arrive in another order or appear more than once. The reducer must produce one canonical result.

The dissertation `Programming Models for Correct and Modular Distributed Systems` is a design reference for algebraic stream admission. The Trellis result remains independent and mechanically checked against local production code.

## Success Contract

The accepted result is one narrow operation profile over canonical bounded finite integer sets. Runtime code refines one specification. Verus proves ACI, identity, canonical output, permutation invariance, duplicate invariance, and monotone-prefix results.

The exported proof manifest lets a consumer bind an exact operation and domain without treating a theorem name as ambient authority.

## Approach Registry

| Family | Mechanism | State |
| --- | --- | --- |
| Concrete verified set-union profile | Spec, exec refinement, proofs, tests, evidence manifest | selected |
| Empty algebraic marker traits | Let any crate assert law membership | rejected |
| Proptest-only evidence | Sample values and orders | rejected |
| General higher-kinded algebra library | Abstract every Rust collection and operation | deferred |
| CRDT synthesis | Search implementation candidates from sequential specs | deferred |
| Reuse current LWW as ACI | Ignore equal-timestamp non-commutativity | rejected |

## Decisions

### Decision: prove one concrete value domain

**Choice:** The first profile uses bounded canonical finite sets represented by strictly increasing duplicate-free integer sequences.

Admission validates the canonical shape before union. Runtime output preserves that shape and uses checked index and length arithmetic.

**Rationale:** A concrete finite domain gives Choregraph one usable proof artifact without introducing a broad abstraction layer.

### Decision: use one production-linked operation

**Choice:** Define one open set-union specification and one executable union function whose `ensures` clause matches the specification.

All flow laws refer to this exact operation. A second test-only union implementation is not the proof target.

**Rationale:** Production and proof paths must not drift.

### Decision: prove the complete ACI profile

**Choice:** Prove associativity, commutativity, idempotence, empty identity, canonical-output preservation, and subset monotonicity.

Reuse existing `algebraic` and `seq_set` primitives when their exact types and claims fit. Add narrow bridge lemmas instead of copying proof bodies.

**Rationale:** Unordered duplicate-bearing reduction requires more than one isolated algebraic fact.

### Decision: prove permutation and duplicate invariance at the reduction level

**Choice:** Define a bounded sequence-of-sets reduction specification. Prove that an admitted adjacent swap preserves the result and that repeated swaps cover a supplied permutation witness.

Prove that inserting a duplicate admitted input set preserves the final result. The proof can compose associativity, commutativity, and idempotence with explicit sequence lemmas.

**Rationale:** Choregraph needs properties that match runtime edge behavior, not only binary-operation equations.

### Decision: prove monotone prefixes without effect claims

**Choice:** Under canonical subset order, prove that reducing a longer admitted prefix cannot remove an element and that every admitted prefix is below the complete reduction.

The theorem says nothing about authorization or irreversible effect timing.

**Rationale:** Mathematical progress and safe external effects are separate contracts.

### Decision: keep closure outside the algebra

**Choice:** The proof accepts an explicit finite input sequence or prefix. It does not decide whether a runtime stream is closed, complete, or live.

**Rationale:** Choregraph declares logical closure. Lattice observes and persists runtime closure facts.

### Decision: export exact proof identities

**Choice:** Extend the existing proof-artifact manifest with rows that bind requirement ID, operation symbol, domain profile, source files, verifier cohort, assumptions, proof status, evidence refs, and non-claims.

The consumer profile lists the exact API and immutable Trellis revision. Choregraph must revalidate or bind its own receipt before making a stronger claim.

**Rationale:** Proof by name or citation does not transfer.

### Decision: retain negative law evidence

**Choice:** Add a negative fixture for an equal-timestamp LWW merge with distinct values. It demonstrates that the current distinct-timestamp theorem does not prove general commutativity.

Add wrong-operation, wrong-domain, stale-source, missing-assumption, non-canonical-set, and forged-manifest cases.

**Rationale:** The consumer boundary must fail for convincing but invalid evidence.

## Functional Core and Imperative Shell

The verified module owns pure admission, set union, reduction, subset order, and theorems. It performs no file, process, clock, network, or environment access.

Existing Rust tools and scripts own manifest generation, file comparison, verifier invocation, transcript capture, and evidence publication.

## Validation Strategy

Positive unit tests cover empty, singleton, disjoint, overlapping, equal, and nested sets. Property tests cover canonical union, ACI, adjacent swaps, duplicate insertion, and prefix monotonicity.

Negative tests cover unsorted input, duplicate entries, bound failures, arithmetic limits, wrong domain, stale proof rows, and the equal-timestamp LWW counterexample.

Verus validation runs against `src/lib.rs`. Tracey must report no uncovered or untested new requirement.

## Adversarial Audit

The audit must try to show that:

- runtime union differs from its specification;
- output loses canonical shape;
- permutation or duplication changes the result;
- a prefix contains an element absent from the final result;
- an invalid set reaches a proof precondition silently;
- an LWW distinct-timestamp proof satisfies general ACI;
- a stale manifest row remains accepted; or
- a proof result claims transport or effect safety.

Any successful counterexample blocks archive.

## Risks and Trade-offs

- Sequence-set proofs can require explicit bridge lemmas and trigger control.
- A concrete integer-set profile is less general than a trait-based algebra.
- Exact manifest identities create update work after source or verifier changes.
- Reduction permutation proofs need a bounded witness representation.
- The result does not cover payload schemas beyond the selected canonical domain.

## Source Boundary

The dissertation source is `https://www.shadaj.me/papers/dissertation.pdf`. Trellis adapts only the algebraic admission idea.

No Hydro runtime, Stageleft staging mechanism, Katara synthesis code, or dissertation proof artifact enters the trusted base.

## Non-Claims

This design does not prove stream closure, transport order, delivery, deduplication storage, exactly-once effects, effect safety, runtime convergence under faults, verifier soundness, whole-system correctness, production readiness, or release eligibility.
