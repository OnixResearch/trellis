## Phase 1: Baseline and domain

- [ ] [serial] Record current algebraic, sequence-set, fold, pointwise, CRDT, proof-evidence, proof-gap, consumer-profile, Tracey, Verus, test, Clippy, and Octet baselines. r[trellis.async_flow.validation]
- [ ] [serial] Record the dissertation URL, selected algebraic-stream concept, rejected mechanisms, and claim boundary. r[trellis.async_flow.reference]
- [ ] [serial] Define the canonical bounded finite integer-set profile with strict order, duplicate exclusion, checked lengths, and explicit malformed-input admission. r[trellis.async_flow.domain]
- [ ] [parallel] Add positive canonical-set tests and negative unsorted, duplicate-entry, oversized, and arithmetic-bound tests. r[trellis.async_flow.domain] r[trellis.async_flow.validation]

## Phase 2: Production-linked set union

- [ ] [serial] Define the open canonical set-union specification and one executable operation with an exact refinement postcondition. r[trellis.async_flow.set_union]
- [ ] [serial] Prove output length bounds, canonical shape, membership union, and empty identity. r[trellis.async_flow.set_union] r[trellis.async_flow.aci]
- [ ] [parallel] Add empty, singleton, disjoint, overlapping, equal, nested, and boundary unit and property tests. r[trellis.async_flow.set_union] r[trellis.async_flow.validation]

## Phase 3: Algebraic and flow-order proofs

- [ ] [serial] Prove set-union associativity, commutativity, and idempotence against the production-linked specification. r[trellis.async_flow.aci]
- [ ] [serial] Prove subset-order reflexivity, antisymmetry, transitivity, union inflation, and union monotonicity for admitted canonical sets. r[trellis.async_flow.monotone]
- [ ] [serial] Define the finite sequence-of-sets reduction specification and executable reducer with exact refinement. r[trellis.async_flow.reduction]
- [ ] [serial] Prove admitted adjacent-swap invariance and supplied-permutation invariance for reduction. r[trellis.async_flow.permutation]
- [ ] [serial] Prove admitted duplicate-input insertion invariance for reduction. r[trellis.async_flow.duplicates]
- [ ] [serial] Prove prefix monotonicity and that every admitted prefix result is below the complete reduction. r[trellis.async_flow.prefix]
- [ ] [parallel] Add property tests for ACI, permutation witnesses, duplicate insertion, and prefix monotonicity. r[trellis.async_flow.aci] r[trellis.async_flow.permutation] r[trellis.async_flow.duplicates] r[trellis.async_flow.prefix] r[trellis.async_flow.validation]

## Phase 4: Negative law boundary

- [ ] [serial] Add a maintained equal-timestamp distinct-value LWW fixture that fails general commutativity while preserving the existing distinct-timestamp theorem scope. r[trellis.async_flow.negative_laws]
- [ ] [parallel] Add wrong-operation, wrong-domain, stale-source, missing-assumption, forged-property, and non-canonical-input evidence fixtures. r[trellis.async_flow.negative_laws] r[trellis.async_flow.evidence] r[trellis.async_flow.validation]
- [ ] [serial] Audit every proof precondition so malformed runtime input is rejected by an executable admission surface before theorem use. r[trellis.async_flow.domain] r[trellis.async_flow.boundary]

## Phase 5: Evidence and consumer handoff

- [ ] [serial] Add canonical requirement text and Tracey implementation and verification markers for every new public surface. r[trellis.async_flow.validation]
- [ ] [serial] Extend the proof-artifact manifest with exact operation, domain, property, source, verifier, assumption, result, and non-claim rows. r[trellis.async_flow.evidence]
- [ ] [serial] Extend the verified-logic consumer profile with the narrow Choregraph API and immutable evidence handoff. r[trellis.async_flow.consumer]
- [ ] [parallel] Add manifest and consumer-profile self-tests that reject stale, missing, duplicate, wrong-domain, wrong-operation, and overclaim rows. r[trellis.async_flow.evidence] r[trellis.async_flow.consumer] r[trellis.async_flow.validation]

## Phase 6: Documentation and closeout

- [ ] [parallel] Document the domain, runtime API, proof laws, permutation witness, duplicate invariance, prefix theorem, evidence handoff, and non-claims. r[trellis.async_flow.boundary]
- [ ] [serial] Add the dissertation to README references with the selected algebraic scope and rejected claim transfers. r[trellis.async_flow.reference]
- [ ] [serial] Run focused tests, property tests, `./scripts/verus.sh src/lib.rs`, Tracey coverage, proof manifests, consumer profiles, formatting, Clippy, Octet, Cairn, and `./scripts/verify.rs`. r[trellis.async_flow.validation]
- [ ] [serial] Run the adversarial audit and block archive for refinement drift, non-canonical output, false ACI, stale evidence, invalid preconditions, LWW scope promotion, or runtime overclaim. r[trellis.async_flow.validation] r[trellis.async_flow.boundary]
- [ ] [serial] Review sync and archive plans and archive only with complete Verus, test, Tracey, manifest, and consumer evidence. r[trellis.async_flow.validation]
