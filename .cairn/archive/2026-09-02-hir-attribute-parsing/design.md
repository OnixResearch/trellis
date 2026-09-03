## Context

Inert attributes are parsed during AST lowering to HIR, while active attributes are expanded earlier. The guide describes parser traits, allowed targets, duplicate policy, safety, stability, and early/limited parsing.

Trellis can model the admission policy over finite parsed attribute metadata while staying independent of rustc token streams and diagnostics.

## Decisions

### 1. Model only inert parsed attributes

**Choice:** The primitive covers finite inert attribute metadata after tokenization, not active attribute expansion.

**Rationale:** Active expansion depends on macro execution. Inert attribute policy is finite and deterministic.

### 2. Represent parser classes explicitly

**Choice:** Parser descriptors are classified as no-args, single, combine, or generic. Admission checks whether the observed occurrences and arguments match the descriptor.

**Rationale:** The guide's parser taxonomy maps cleanly to finite checks.

### 3. Keep target, duplicate, safety, and stability policies separate

**Choice:** Attribute admission composes independent predicates for allowed target, duplicate handling, unsafe wrapper, and feature stability.

**Rationale:** Separate predicates are easier to test and prove than one monolithic parser.

### 4. Model diagnostics as reason codes

**Choice:** Parser failures return deterministic reason codes and input indices, not rustc-rendered diagnostic messages.

**Rationale:** Trellis should verify deterministic rejection, not presentation.

### 5. Require early/late agreement when both are declared

**Choice:** If limited early parsing and later HIR parsing are both modeled for an attribute, their successful parsed metadata must agree or the discrepancy must be rejected.

**Rationale:** This is useful for front-end consumers that need early parsed attributes without duplicating policy.

### 6. Use a functional core with aggregate finite inputs

**Choice:** The core consumes one typed descriptor, one aggregate occurrence, and explicit phase metadata. It uses fixed-width counts and target masks without token streams or registry I/O. Separate source files own model, registry, rule, admission, agreement, proof, and test responsibilities.

**Rationale:** Aggregate facts keep policy deterministic and bounded. The split keeps every production file below the Octet source-size limit.

## Risks / Trade-offs

- Token parsing can be deferred; the first version may consume already-tokenized argument shapes.
- Some rustc attributes combine multiple names or carry special logic. The generic parser descriptor can cover these later without claiming all attributes now.
