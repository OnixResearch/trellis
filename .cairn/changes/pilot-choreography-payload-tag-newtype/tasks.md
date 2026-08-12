## Phase 1: Baseline and type boundary

- [ ] [serial] Capture focused choreography tests and full-library Verus baseline evidence before changing the type. r[newtypes.choreography-payload.validation]
- [ ] [serial] Define the exact `PayloadTag` representation, copy/equality behavior, raw field boundary, and unchanged all-`u32` value domain. r[newtypes.choreography-payload.type]

## Phase 2: Verified migration

- [ ] [depends:payload-tag-type] Migrate global and local choreography syntax so every payload position uses `PayloadTag` while role and label aliases remain unchanged. r[newtypes.choreography-payload.type] r[newtypes.choreography-payload.scope]
- [ ] [depends:payload-tag-syntax] Migrate projection, merge, and global/local step helpers without erasing typed payload equality inside semantic predicates. r[newtypes.choreography-payload.preservation]
- [ ] [depends:payload-tag-semantics] Repair choreography and cross-module proof functions without adding an external body, assumption, or new proof-gap bridge. r[newtypes.choreography-payload.preservation]
- [ ] [depends:payload-tag-proofs] Audit public choreography functions and requirement links for complete typed payload coverage. r[newtypes.choreography-payload.docs]

## Phase 3: Positive and negative evidence

- [ ] [parallel] Add positive tests for typed payload construction, global-to-local preservation, merge equality, duality, and step preservation. r[newtypes.choreography-payload.evidence]
- [ ] [depends:payload-tag-type] Add compile-fail evidence for supplying `LabelId` or raw `u32` where `PayloadTag` is required. r[newtypes.choreography-payload.evidence]
- [ ] [parallel] Keep negative runtime tests for mismatched payload tags, invalid merge heads, missing branches, and wrong-head steps. r[newtypes.choreography-payload.evidence]

## Phase 4: Decision and validation

- [ ] [serial] Update `docs/requirements.md`, source and test annotations, API documentation, and accepted choreography Cairn text. r[newtypes.choreography-payload.docs]
- [ ] [depends:payload-tag-evidence] Run focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, direct Tracey uncovered and untested queries, and proof-gap inventory checks. r[newtypes.choreography-payload.validation]
- [ ] [depends:payload-tag-validation] Record the proof repairs and a bounded stop or continue decision. Do not migrate roles or labels in this change. r[newtypes.choreography-payload.scope] r[newtypes.choreography-payload.validation]
- [ ] [depends:payload-tag-decision] Run Cairn validation and proposal, design, and tasks gates. Sync and archive only with fresh evidence. r[newtypes.choreography-payload.validation]
