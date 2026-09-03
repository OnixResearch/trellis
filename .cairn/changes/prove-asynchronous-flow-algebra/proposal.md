## Why

Choregraph needs exact evidence before it admits unordered or duplicate-bearing asynchronous edges. Trellis has reusable algebraic predicates, sequence and set primitives, CRDT examples, and proof-evidence export.

The current algebraic predicates focus on `u64` operations. They do not provide one consumer-ready proof profile for canonical finite set union, permutation invariance, duplicate invariance, or monotone prefixes.

Hydro demonstrates the value of algebraic constraints, but it relies on manual declarations for key laws. Trellis can provide the mechanically checked source of those laws for the first Onix flow profile.

## What Changes

- Add a narrow verified asynchronous-flow algebra module over bounded canonical finite integer sets.
- Define canonical set admission and an executable set-union operation with exact specification refinement.
- Prove associativity, commutativity, idempotence, identity, canonical-output preservation, and monotonicity.
- Prove that set-union reduction is invariant under admitted input permutation and duplicate insertion.
- Prove that every admitted reduction prefix is below the final result under the selected subset order.
- Export exact proof-artifact rows for Choregraph and Lattice consumption.
- Add a negative LWW fixture that demonstrates why distinct-timestamp commutativity cannot establish general ACI behavior.
- Record the dissertation as a design reference without importing Hydro, Katara, or Stageleft code.
- Add positive and negative runtime, property, Verus, manifest, and claim-boundary evidence.

## Success Criteria

- The runtime set-union operation refines one exact specification over canonical admitted sets.
- Verus proves every selected algebraic and flow-order property against that operation.
- Permuting or duplicating admitted input sets does not change the canonical final union.
- Every admitted reduction prefix is a subset of the final union.
- Proof artifacts bind source, operation, domain, property, verifier, assumptions, and non-claims.
- Choregraph can reject a stale or wrong-domain proof reference without importing Trellis proof internals.

## False Completion

A proptest alone is not completion. A generic `Commutative` marker trait is not completion.

The existing LWW distinct-timestamp theorem is not a general commutativity proof. A bounded example run is not a proof for arbitrary admitted inputs.

## Impact

- **Source**: new verified flow-algebra and canonical set-union surfaces with narrow exports.
- **Requirements**: add canonical requirement and Tracey anchors to `docs/requirements.md`.
- **Evidence**: extend the proof-artifact manifest and existing consumer profile.
- **Testing**: positive unit and property tests plus malformed, non-canonical, stale-evidence, wrong-domain, overflow, and false-law cases.
- **Consumers**: Choregraph first, then Lattice through Choregraph-owned compatibility artifacts.

## Dependencies

- Existing `algebraic`, `seq_set`, `seq_fold`, `pointwise`, proof-evidence, proof-gap, and consumer-profile boundaries remain authoritative.
- Choregraph change `add-asynchronous-flow-profile` consumes the published result after Trellis archives and publishes it.
- No direct dependency on Choregraph, Lattice, Hydro, Katara, or a stream runtime is required.

## Out of Scope

- Proving laws for arbitrary user functions or Rust trait implementations.
- Transport delivery, retries, network ordering, process recovery, or effect safety.
- Repairing every existing CRDT or defining a general CRDT synthesis engine.
- Katara-style candidate synthesis.
- Equality-saturation optimization.
- Production readiness or release eligibility.

## Affected Specs

- `asynchronous-flow-algebra`: canonical set union, algebraic laws, permutation, duplicates, prefixes, evidence, validation, and non-claims.
