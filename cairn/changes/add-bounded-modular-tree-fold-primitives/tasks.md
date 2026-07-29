## Phase 1: Boundary and profile

- [ ] [serial] Record the exact Bend and HVM2 reference revisions, versions, licenses, and non-claim boundary in README and proof-facing documentation without importing external source or claims. r[bounded-modular-tree-fold.boundary]
- [ ] [serial] Define the versioned balanced `u24` sum profile, admitted input shape, typed rejection reasons, caller-provided leaf limit, executable oracle symbol, and downstream handoff fields before implementation. r[bounded-modular-tree-fold.admission] r[bounded-modular-tree-fold.evidence]

## Phase 2: `u24` domain

- [ ] [serial] Add a verified `u24` module with named width, modulus, maximum, mask, and conversion constants plus validity and normalization specifications. r[bounded-modular-tree-fold.u24-domain]
- [ ] [depends:u24-domain] Implement normalization and wrapping addition with exact spec postconditions and no hidden widening or host-state dependency. r[bounded-modular-tree-fold.u24-domain] r[bounded-modular-tree-fold.u24-add]
- [ ] [depends:u24-add] Prove normalization closure, valid-input identity, normalization idempotence, wrapping-add closure, zero identity, commutativity, and associativity. Connect concrete laws to generic algebraic predicates where useful. r[bounded-modular-tree-fold.u24-domain] r[bounded-modular-tree-fold.algebra]
- [ ] [parallel] Add positive and negative unit and property tests for zero, maximum, wrap, normalization, out-of-domain input, and the concrete algebraic laws. r[bounded-modular-tree-fold.fixtures]

## Phase 3: Balanced reduction and oracle

- [ ] [depends:balanced-u24-profile] Add pure admission for non-empty power-of-two leaf vectors under a nonzero caller-provided limit. Reject out-of-domain leaves and unrepresentable accounting. r[bounded-modular-tree-fold.admission]
- [ ] [depends:u24-add] Implement the optimized iterative sequential sum oracle with a postcondition equal to the ordered modular-sum specification. r[bounded-modular-tree-fold.refinement]
- [ ] [depends:balanced-admission] Implement one adjacent-pair round and repeated balanced reduction without recursive Rust stack growth. Refine each executable helper to its declared spec. r[bounded-modular-tree-fold.refinement]
- [ ] [depends:balanced-reducers] Prove one round preserves ordered modular sum and prove the accepted balanced result equals the sequential oracle result. r[bounded-modular-tree-fold.equivalence]
- [ ] [depends:balanced-reducers] Prove exact abstract combine counts, exact halving-round counts, monotone vector shrinkage, and termination for admitted inputs. r[bounded-modular-tree-fold.work]
- [ ] [parallel] Add positive fixtures for singleton and multi-round inputs. Add negative fixtures for empty input, zero limit, over-limit input, non-power-of-two length, invalid leaves, stale profile identity, and accounting boundary failure. r[bounded-modular-tree-fold.fixtures]
- [ ] [depends:balanced-equivalence] Add property tests that compare the optimized oracle, balanced reference reducer, and an independent test-only mathematical oracle across generated admitted inputs. r[bounded-modular-tree-fold.fixtures]

## Phase 4: Traceability and proof handoff

- [ ] [serial] Add canonical requirements, `r[impl]` and `r[verify]` links, public exports, module documentation, README catalog text, examples, and explicit non-claims. r[bounded-modular-tree-fold.docs] r[bounded-modular-tree-fold.non-claims]
- [ ] [depends:balanced-equivalence] Add a proof-evidence bundle row that binds source, requirements, proof functions, verifier, proof gaps, algorithm profile, executable oracle symbol, BLAKE3 identities, and downstream non-claims. r[bounded-modular-tree-fold.evidence]
- [ ] [depends:proof-handoff] Add positive and negative proof-artifact fixtures for exact pinning, stale identity, missing verifier receipt, wrong algorithm profile, unsupported verified role, and downstream overclaim. r[bounded-modular-tree-fold.evidence] r[bounded-modular-tree-fold.fixtures]

## Phase 5: Validation and archive

- [ ] [serial] Run focused Cargo tests and property tests for `u24` and balanced reduction. Capture exact commands and positive and negative results. r[bounded-modular-tree-fold.validation]
- [ ] [serial] Run full-library Verus verification and reconcile every promised theorem with a concrete verified proof function or exact public ensures refinement. r[bounded-modular-tree-fold.validation]
- [ ] [serial] Run Tracey uncovered and untested queries, proof-evidence validation, module and README drift checks, proof-gap inventory, Clippy, Octet policy, and relevant Nix checks. r[bounded-modular-tree-fold.validation]
- [ ] [serial] Run Cairn validation and proposal, design, and tasks gates. Sync the accepted spec and archive only with fresh proof, test, traceability, and non-claim evidence. r[bounded-modular-tree-fold.validation]
