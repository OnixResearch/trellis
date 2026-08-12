## Context

`TypedDurableReference` contains six raw `u64` fields with different meanings. `ConversionWitness` repeats old and new reference, type, schema, and closure identities. The admission rules compare domains correctly, but the Rust and Verus type systems do not distinguish them.

Unlike arithmetic-heavy index models, this module mostly uses equality, nonzero checks, and finite membership. It is a reasonable Verus newtype candidate if the migration preserves invalid-input expressiveness and does not weaken proofs.

## Decisions

### 1. Use concrete identity structures

**Choice:** Define separate Verus-visible structures for reference, value, type, codec, closure, and schema identities. Each structure contains one public raw `u64` field.

**Rationale:** Concrete structures are explicit and avoid generic tag or `PhantomData` behavior in Verus.

### 2. Keep invalid raw values representable

**Choice:** Permit every raw `u64`, including zero, in each structure. Keep finite metadata validity as an explicit predicate.

**Rationale:** Existing admission requirements must continue to model and reject malformed metadata. A validated constructor would narrow the theorem domain and weaken negative evidence.

### 3. Type collections by their contents

**Choice:** Store `ClosureHash` in closure catalogs and unresolved-closure results. Store `TypeHash` in unpersist requests. Use the exact identity type in every witness field.

**Rationale:** Wrapping only top-level envelope fields would leave raw collection and request seams where substitutions remain possible.

### 4. Preserve domain-specific equality

**Choice:** Type-match predicates compare `TypeHash`; closure predicates compare `ClosureHash`; migration predicates compare matching old and new identity domains.

**Rationale:** The proof should state the domain distinction directly instead of erasing all wrappers to raw values.

### 5. Keep arithmetic minimal and explicit

**Choice:** Use raw-field projection only for nonzero finite-metadata checks and schema comparisons that need the underlying order or equality.

**Rationale:** The wrappers provide type value only when semantic predicates retain them. Raw projection remains necessary for numeric admission.

### 6. Require proof and compile-time evidence

**Choice:** Preserve all existing Verus proofs and add compile-fail evidence for representative cross-domain substitutions. Add runtime negative tests using zero-valued wrappers.

**Rationale:** Verus evidence covers semantic preservation. Compile-fail evidence covers the structural API guarantee.

### 7. Keep hash claims unchanged

**Choice:** Retain the current abstract finite identity model and its explicit non-claims. Do not reinterpret modeled `u64` values as cryptographic digests.

**Rationale:** Type separation does not supply collision resistance or storage authenticity.

## Functional core / imperative shell split

- **Pure core**: identity envelopes, finite validity, closure membership, unpersist admission, and migration-witness matching.
- **Imperative shell**: Cargo and Verus invocation, compile-fail execution, Tracey queries, Cairn commands, and evidence capture.

## Risks / Trade-offs

- The module has many repeated field comparisons, so projection churn may be larger than the source footprint suggests.
- Public callers must update every literal and fixture.
- A wrapper with a public raw field prevents domain exchange but does not enforce nonzero validity.
- Generic helper reuse may decrease because each identity domain is concrete.

## Non-Goals

- No cryptographic hash or BLAKE3 implementation change.
- No storage, serialization, codec, type-checker, or dependency-fetch behavior.
- No automatic schema migration.
- No validated constructor that removes malformed inputs from the proof domain.
- No newtypes for unrelated identity models in other modules.
