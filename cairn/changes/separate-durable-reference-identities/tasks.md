## Phase 1: Identity model and baseline

- [ ] [serial] Capture focused durable-reference tests and full-library Verus baseline evidence before changing identity representations. r[newtypes.durable-identities.validation]
- [ ] [serial] Define `ReferenceHash`, `ValueHash`, `TypeHash`, `CodecHash`, `ClosureHash`, and `SchemaVersion` with public raw fields and unchanged all-`u64` value domains. r[newtypes.durable-identities.types] r[newtypes.durable-identities.invalid-inputs]

## Phase 2: Verified migration

- [ ] [depends:durable-identity-types] Migrate `TypedDurableReference`, `ClosureCatalog`, `UnpersistRequest`, and `ConversionWitness` to exact identity domains. r[newtypes.durable-identities.types]
- [ ] [depends:durable-identity-records] Migrate finite metadata, closure membership, missing-closure, unpersist, and conversion-witness specifications without erasing domain distinctions. r[newtypes.durable-identities.preservation]
- [ ] [depends:durable-identity-specs] Migrate executable helpers and proof functions without adding a trusted bridge, assumption, or external body. r[newtypes.durable-identities.preservation] r[newtypes.durable-identities.validation]
- [ ] [depends:durable-identity-proofs] Audit all public signatures and collections so no typed identity position remains raw `u64`. r[newtypes.durable-identities.types]

## Phase 3: Positive and negative evidence

- [ ] [parallel] Add positive tests for admitted references, available closures, matching unpersist requests, and valid conversion witnesses. r[newtypes.durable-identities.evidence]
- [ ] [parallel] Add negative runtime tests for zero-valued identities, missing closures, wrong same-domain values, and incomplete conversion witnesses. r[newtypes.durable-identities.invalid-inputs] r[newtypes.durable-identities.evidence]
- [ ] [depends:durable-identity-types] Add compile-fail evidence for representative type-to-closure, closure-to-reference, and schema-to-hash substitutions. r[newtypes.durable-identities.evidence]

## Phase 4: Traceability and validation

- [ ] [serial] Update `docs/requirements.md`, source and test annotations, public documentation, accepted Cairn text, and explicit hash-strength non-claims. r[newtypes.durable-identities.docs] r[newtypes.durable-identities.non-claims]
- [ ] [depends:durable-identity-evidence] Run focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, direct Tracey uncovered and untested queries, and proof-gap inventory checks. r[newtypes.durable-identities.validation]
- [ ] [depends:durable-identity-validation] Run Cairn validation and proposal, design, and tasks gates. Sync and archive only with fresh evidence. r[newtypes.durable-identities.validation]
