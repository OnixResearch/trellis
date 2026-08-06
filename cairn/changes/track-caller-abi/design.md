## Context

`#[track_caller]` propagates a caller-location reference through tracked functions. Direct calls can pass hidden location metadata; indirect calls require a reify shim to avoid ABI mismatch. The attribute is valid only for certain functions and trait/default method cases carry inheritance behavior.

Trellis should model finite metadata and call-edge behavior, not runtime stack walking or panic formatting.

## Decisions

### 1. Validate attribute targets explicitly

**Choice:** Function metadata records ABI kind, closure/naked status, trait/default inheritance metadata, and track-caller annotation source. Admission rejects invalid target shapes.

**Rationale:** Attribute validity is finite and easy to test negatively.

### 2. Model caller location as abstract metadata

**Choice:** Caller locations are abstract finite identifiers with span/source metadata, not allocated `Location` values.

**Rationale:** The primitive verifies propagation rules, not allocation or string formatting.

### 3. Propagate through tracked chains

**Choice:** A tracked callee receives the caller's current location metadata if the caller is tracked; otherwise it receives metadata for the immediate call site.

**Rationale:** This captures the core ABI inversion described by the guide.

### 4. Require shims for erased tracked function pointers

**Choice:** Taking a pointer to a tracked function requires declared reify-shim metadata that adapts the hidden argument boundary.

**Rationale:** This is the soundness-critical indirect-call case.

### 5. Record const/codegen agreement as finite trace equivalence

**Choice:** The primitive compares finite const-context and codegen-context caller-location traces over the same call graph.

**Rationale:** The guide notes both contexts must agree. A finite trace model is enough for Trellis claims.

## Risks / Trade-offs

- Trait inheritance can become broad. Keep it to finite metadata for prototype/default/impl override cases.
- Real spans and `Location` allocation are outside scope; consumers needing source filenames must add adapters.
