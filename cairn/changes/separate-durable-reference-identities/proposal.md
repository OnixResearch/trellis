## Why

The typed durable reference model represents reference, value, type, codec, closure, and schema identities as `u64`. These values occur together in reference envelopes, unpersist requests, closure catalogs, and conversion witnesses. Exchanging two domains compiles and can produce a structurally plausible but incorrect witness.

The model already treats these fields as requirement-level domains and uses equality or finite admission rather than heavy arithmetic. This change introduces concrete Verus-visible identity newtypes while preserving malformed raw values for admission rejection.

## What Changes

- Add distinct `ReferenceHash`, `ValueHash`, `TypeHash`, `CodecHash`, `ClosureHash`, and `SchemaVersion` structures.
- Migrate typed references, closure catalogs, unpersist requests, conversion witnesses, specifications, executable helpers, and tests.
- Keep raw zero values representable so existing finite-metadata rejection remains meaningful.
- Preserve all envelope, closure-availability, type-match, and migration-boundary results.
- Add compile-fail evidence for cross-domain identity substitution and runtime evidence for malformed raw values.

## Impact

- **Files**: `src/typed_durable_reference.rs`, focused tests or compile-fail fixtures, `docs/requirements.md`, and the accepted `domain-primitives` Cairn spec.
- **Testing**: focused tests, compile-fail evidence, full Cargo tests, full-library Verus, Clippy, Tracey, and Cairn validation.
- **Compatibility**: callers must construct each declared identity type explicitly.
- **Claims**: the wrappers provide domain separation only. They do not add cryptographic strength, persistence, decoding, storage, or migration-execution guarantees.
