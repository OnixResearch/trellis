## Context

rustc crate dependencies can be loaded from `rlib`, `dylib`, or `rmeta` artifacts containing rustc-specific metadata. The metadata includes compiler version, strict version hash, stable crate ID, exported item information, optional encoded MIR, and dependency data. Build pipelining may use `rmeta` before full codegen finishes.

Trellis should model finite identity and admission facts over declared metadata, not byte-level archive formats.

## Decisions

### 1. Model library kinds and metadata presence

**Choice:** Library-kind metadata distinguishes `rlib`, `dylib`, and `rmeta`, and admission checks which kinds carry linkable code versus metadata-only content.

**Rationale:** This captures the key behavioral boundary for cargo check/doc pipelining and link-time use.

### 2. Use rustc-mandated identity fields

**Choice:** The primitive models SVH and stable crate ID fields as declared rustc identity values and checks equality/compatibility. It does not substitute BLAKE3 for these interop fields.

**Rationale:** Hash choice is mandated by rustc metadata compatibility. Trellis should preserve the external protocol boundary.

### 3. Keep hash security out of scope

**Choice:** Equality of declared identity fields is modeled; collision resistance of rustc hashes is not proven.

**Rationale:** The primitive is an identity/admission model, not a cryptographic proof.

### 4. Treat encoded MIR as optional metadata

**Choice:** Metadata may include encoded MIR when required for codegen, but `rmeta` used for fast check/doc surfaces may omit codegen-relevant MIR according to declared capability metadata.

**Rationale:** Optionality matters for correct consumer claims.

### 5. Model dependency lookup deterministically

**Choice:** Dependency loading consumes a finite candidate set and accepts exactly one compatible metadata candidate or rejects with deterministic ambiguity/mismatch metadata.

**Rationale:** This makes negative evidence for wrong versions and ambiguous dependencies clear.

### 6. Bound selection and keep effects outside the core

**Choice:** Candidate selection accepts at most `MAX_CANDIDATES` declared artifacts. A pure bounded scan returns the first unique candidate or the first conflicting pair. Separate source files own model, admission, resolver, proof, and test responsibilities.

**Rationale:** The explicit bound prevents unbounded search. The core performs no filesystem, archive, linker, or registry I/O.

## Risks / Trade-offs

- Real archive scanning has many platform details. This primitive intentionally models declared metadata only.
- Exact rustc hash construction is not proven. Later adapters may verify input selection if needed, but this change checks compatibility fields.
