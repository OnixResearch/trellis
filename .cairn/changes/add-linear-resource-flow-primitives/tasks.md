# Tasks: Add linear resource-flow primitives

## Reference and baseline

- [ ] [serial] Record the exact Mirth revision, reviewed source paths, license text, build result, golden-suite result, open maturity gaps, and adaptation non-claims. r[trellis.linear_resource_flow.reference]
- [ ] [serial] Record focused baselines for `type_unification`, `algebraic_effect`, `ability_effect`, `remote_computation`, proof gaps, Tracey, and the full verification rail. r[trellis.linear_resource_flow.evidence]
- [ ] [parallel] Audit existing Trellis modules for reusable terms, substitutions, set operations, and partition proofs before new implementation. r[trellis.linear_resource_flow.model]

## Model and executable decisions

- [ ] [depends:trellis.linear_resource_flow.model] Define normalized identity, resource entry, row expression, closed row, substitution, arrow, witness, and decision types. r[trellis.linear_resource_flow.model]
- [ ] [depends:trellis.linear_resource_flow.model] Implement well-formed closed-row and unique-occurrence specifications and executable decisions. r[trellis.linear_resource_flow.well_formed]
- [ ] [depends:trellis.linear_resource_flow.model] Implement bounded row substitution and unification with an occurs check. r[trellis.linear_resource_flow.unification]
- [ ] [depends:trellis.linear_resource_flow.unification] Implement sequential arrow composition over admitted substitutions. r[trellis.linear_resource_flow.sequence]
- [ ] [depends:trellis.linear_resource_flow.well_formed] Implement closed-row fork partition, conditional join, and terminal discharge decisions. r[trellis.linear_resource_flow.graph]
- [ ] [depends:trellis.linear_resource_flow.well_formed] Implement exact duplication and disposal witness admission. r[trellis.linear_resource_flow.witness]
- [ ] [parallel] Keep the complete module deterministic, `no_std + alloc`, host-I/O free, serialization free, and authority free. r[trellis.linear_resource_flow.boundary]

## Proofs

- [ ] [depends:trellis.linear_resource_flow.unification] Prove successful unification soundness and recursive-substitution rejection for the stated solver scope. r[trellis.linear_resource_flow.unification]
- [ ] [depends:trellis.linear_resource_flow.sequence] Prove sequential compatibility and outer-row preservation after substitution. r[trellis.linear_resource_flow.sequence]
- [ ] [depends:trellis.linear_resource_flow.graph] Prove fork completeness, pairwise disjointness, conditional join agreement, and terminal non-loss. r[trellis.linear_resource_flow.graph]
- [ ] [depends:trellis.linear_resource_flow.witness] Prove that duplication and disposal require exact admitted witnesses. r[trellis.linear_resource_flow.witness]
- [ ] [depends:trellis.linear_resource_flow.sequence] Prove rejection preservation and executable equality with every public specification. r[trellis.linear_resource_flow.rejection]

## Tests and consumer evidence

- [ ] [parallel] Add positive unit and property tests for closed rows, tail substitution, sequence, fork, join, witnesses, and terminal discharge. r[trellis.linear_resource_flow.evidence]
- [ ] [parallel] Add negative tests for duplicate occurrences, occurs-check failures, mismatch, implicit reorder, fork copy, fork loss, branch mismatch, wrong witnesses, and terminal loss. r[trellis.linear_resource_flow.evidence]
- [ ] [depends:trellis.linear_resource_flow.rejection] Publish a Choregraph consumer profile with exact model, bounds, function, proof, assumption, and non-claim identities. r[trellis.linear_resource_flow.consumer]
- [ ] [depends:trellis.linear_resource_flow.consumer] Add positive and negative consumer-profile fixtures, including stale revision and projection mismatch cases. r[trellis.linear_resource_flow.consumer]

## Documentation and closeout

- [ ] [serial] Add requirement and Tracey anchors to `docs/requirements.md`, source, tests, and proof functions. r[trellis.linear_resource_flow.evidence]
- [ ] [parallel] Update `src/lib.rs`, the README module catalog and reference list, module guides, module indexes, and proof-artifact manifests. r[trellis.linear_resource_flow.reference] r[trellis.linear_resource_flow.consumer]
- [ ] [serial] Run focused tests, full workspace tests, Clippy with warnings denied, full-library Verus, Tracey, Octet, policy freshness, and repository guards. r[trellis.linear_resource_flow.evidence]
- [ ] [serial] Validate Cairn proposal, design, tasks, and repository state. Then sync accepted requirements and archive with fresh evidence. r[trellis.linear_resource_flow.evidence]
