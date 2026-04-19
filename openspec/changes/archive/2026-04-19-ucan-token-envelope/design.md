## Context

The capability-auth modules verify individual authorization factors but
do not model the token that carries them. The token envelope is the
integration point where signature, temporal, identity, and attenuation
checks compose into a single validation decision.

## Goals / Non-Goals

**Goals:** A `UcanPayload` struct with field validation. A `validate_envelope`
function that is sound (success implies all sub-checks pass) and complete
(all sub-checks passing implies success). Integration with existing
capability-auth containment and token-timestamp modules.

**Non-Goals:** Serialization format (DAG-CBOR or JWT). Network transport.
Token storage or caching. Proof chain resolution (CID lookup).

## Decisions

### 1. Payload Is a Flat Struct

**Choice:** `UcanPayload` has fields for iss, aud, att, prf, exp, nbf,
nnc, fct. No nested token or embedded proof resolution.

**Rationale:** Validation operates on a single token at a time. Proof
chain walking is a separate concern that consumes validation results,
not part of single-token validation. Keeps the proof surface small.

### 2. Validation Returns a Rich Error

**Choice:** `validate_envelope` returns `Result<UcanEnvelope, ValidationError>`
where `ValidationError` is an enum with variants for each failure mode.

**Rationale:** Downstream code needs to know which check failed for
error reporting. The proof only needs the boolean view (Ok vs Err),
but the exec surface carries the reason.

### 3. Attenuation Check Takes Parent Att as Input

**Choice:** `check_attenuation_chain(parent_att, child_att) -> bool`
takes the parent's capabilities directly, not a parent token.

**Rationale:** Proof chain resolution (looking up parent by CID) is
out of scope. The attenuation check is pure logic over two capability
lists. The caller is responsible for resolving the parent.

### 4. Reuses Existing Modules

**Choice:** Temporal checks delegate to `token_timestamp`, containment
to `containment`, scope to `scope`.

**Rationale:** Those modules are already verified. The token envelope
proves that their composition is sound, not re-proves their internals.

## Risks / Trade-offs

**[Serialization gap]** → Without canonical serialization, we cannot
prove that the bytes signed match the payload struct. This is deferred
to a future canonical-serialization change. For now, the proof assumes
the caller provides correct payload bytes.
