## Phase 1: Attribute and call metadata

- [ ] [serial] Define finite function metadata for ABI kind, closure/naked status, trait/default relationship, track-caller source, and caller-location identifiers. r[track-caller.attribute-validity] r[track-caller.location-metadata]
- [ ] [serial] Define call-edge metadata for tracked and untracked direct calls plus tracked function-pointer reification. r[track-caller.direct-call] r[track-caller.reify-shim]
- [ ] [serial] Define trait method inheritance specs for prototype annotations, default implementations, and overrides. r[track-caller.trait-inheritance]

## Phase 2: Propagation helpers and proofs

- [ ] [serial] Implement checked exec helpers for attribute admission, call-edge admission, caller-location selection, tracked-chain forwarding, and shim requirement checks. r[track-caller.admission]
- [ ] [serial] Prove helper facts that admitted tracked chains forward the nearest untracked caller location and that indirect tracked function pointers require a shim. r[track-caller.propagation] r[track-caller.reify-shim]
- [ ] [serial] Define finite const/codegen trace agreement checks. r[track-caller.const-codegen-agreement]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for valid direct tracked calls, tracked chains, untracked boundaries, trait/default inheritance, and reify shims. r[track-caller.evidence]
- [ ] [serial] Add negative tests for invalid ABI targets, closure/naked targets, missing shim, wrong propagated location, missing trait inheritance, and const/codegen disagreement. r[track-caller.evidence]
- [ ] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[track-caller.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for track-caller helpers and capture evidence. r[track-caller.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[track-caller.final-validation]
