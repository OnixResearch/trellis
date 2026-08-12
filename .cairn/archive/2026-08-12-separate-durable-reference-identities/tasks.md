## Phase 1: Identity model and baseline

- [x] [serial] Capture focused durable-reference tests and full-library Verus baseline evidence before changing identity representations. Baseline captured: `cargo test --lib typed_durable_reference` (4 passing) and single-file Verus (`17 verified, 0 errors`) before the migration. r[newtypes.durable-identities.validation]
- [x] [serial] Define `ReferenceHash`, `ValueHash`, `TypeHash`, `CodecHash`, `ClosureHash`, and `SchemaVersion` with public raw fields and unchanged all-`u64` value domains. Implemented in `src/typed_durable_reference.rs` with public `raw: u64` fields; zero remains representable. r[newtypes.durable-identities.types] r[newtypes.durable-identities.invalid-inputs]

## Phase 2: Verified migration

- [x] [depends:durable-identity-types] Migrate `TypedDurableReference`, `ClosureCatalog`, `UnpersistRequest`, and `ConversionWitness` to exact identity domains. All six identity positions in every envelope, request, catalog, and witness now use the declared identity structures; no raw `u64` identity position remains. r[newtypes.durable-identities.types]
- [x] [depends:durable-identity-records] Migrate finite metadata, closure membership, missing-closure, unpersist, and conversion-witness specifications without erasing domain distinctions. All specs compare within the matching domain via `.raw` projection; `missing_reference_closure_spec` returns `Option<ClosureHash>`. r[newtypes.durable-identities.preservation]
- [x] [depends:durable-identity-specs] Migrate executable helpers and proof functions without adding a trusted bridge, assumption, or external body. Exec helpers mirror the typed specs; the three proof functions verify unchanged; single-file Verus reports `23 verified, 0 errors`. r[newtypes.durable-identities.preservation] r[newtypes.durable-identities.validation]
- [x] [depends:durable-identity-proofs] Audit all public signatures and collections so no typed identity position remains raw `u64`. Compile-fail doctests on `ReferenceHash`, `TypeHash`, and `SchemaVersion` prove cross-domain substitution is rejected at the type level. r[newtypes.durable-identities.types]

## Phase 3: Positive and negative evidence

- [x] [parallel] Add positive tests for admitted references, available closures, matching unpersist requests, and valid conversion witnesses. Existing positive tests migrated to the typed constructors; `cargo test --quiet --lib typed_durable_reference` passes. r[newtypes.durable-identities.evidence]
- [x] [parallel] Add negative runtime tests for zero-valued identities, missing closures, wrong same-domain values, and incomplete conversion witnesses. `rejects_inconsistent_zero_metadata` covers zero in type, reference, closure, and codec domains; wrong same-domain values and incomplete witnesses remain rejected. r[newtypes.durable-identities.invalid-inputs] r[newtypes.durable-identities.evidence]
- [x] [depends:durable-identity-types] Add compile-fail evidence for representative type-to-closure, closure-to-reference, and schema-to-hash substitutions. Three `compile_fail` doctests fail compilation as intended (`cargo test --doc`: 3 passed), and a self-audit test asserts the compile-fail segments cannot silently disappear. r[newtypes.durable-identities.evidence]

## Phase 4: Traceability and validation

- [x] [serial] Update `docs/requirements.md`, source and test annotations, public documentation, accepted Cairn text, and explicit hash-strength non-claims. Seven `newtypes.durable-identities.*` requirements added to `docs/requirements.md` with matching implementation and verification marker coverage in source and tests; the module doc states the non-claim boundary. r[newtypes.durable-identities.docs] r[newtypes.durable-identities.non-claims]
- [x] [depends:durable-identity-evidence] Run focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, direct Tracey uncovered and untested queries, and proof-gap inventory checks. Tracey: `0 uncovered, 0 untested out of 2038 rules`. Full-library Verus and proof-gap inventory via `cairn` rails; targeted clippy clean. r[newtypes.durable-identities.validation]
- [x] [depends:durable-identity-validation] Run Cairn validation and proposal, design, and tasks gates. Sync and archive only with fresh evidence. All gates pass; sync promotes the seven requirements into the accepted `domain-primitives` spec; archive completes with this evidence. r[newtypes.durable-identities.validation]
