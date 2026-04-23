## Context

The capability-auth modules verify individual authorization factors but
do not model the token that carries them. The token envelope is the
integration point where signature, temporal, identity, and attenuation
checks compose into a single validation decision.

## Goals / Non-Goals

**Goals:** A `UcanPayload` struct with field validation. A `validate_envelope`
function that is sound (success implies all single-token sub-checks pass)
and complete (all single-token sub-checks passing implies success).
Integration with existing UCAN temporal and containment-style modules.

**Non-Goals:** Serialization format (DAG-CBOR or JWT). Network transport.
Token storage or caching. Proof chain resolution (CID lookup). Walking
`prf` to fetch parent tokens. Proving that canonical payload bytes were
constructed from the payload struct.

## Decisions

### 1. Payload Is a Flat Struct

**Choice:** `UcanPayload` currently has fields for iss, aud, att, prf,
exp, and nbf. No nested token or embedded proof resolution.

**Rationale:** Validation operates on a single token at a time. Proof
chain walking is a separate concern that consumes validation results,
not part of single-token validation. The draft fields `nnc` and `fct`
are deferred because the implemented token surface does not validate
or store them yet. This keeps the proof surface small and artifact scope honest.

### 2. Validation Returns a Rich Error

**Choice:** `validate_envelope` returns `Result<_, ValidationError>`
where `ValidationError` is an enum with variants for each single-token
failure mode.

**Rationale:** Downstream code needs to know which check failed for
error reporting. The proof only needs the boolean view (Ok vs Err),
but the exec surface carries the reason.

### 3. Attenuation Check Is Separate From Envelope Validation

**Choice:** `check_attenuation_chain(parent_att, child_att) -> bool`
takes the parent's capabilities directly, not a parent token, and is
kept separate from `validate_envelope`.

**Rationale:** Proof chain resolution (looking up parent by CID) is
out of scope. The attenuation check is pure logic over two capability
lists. The caller is responsible for resolving the parent and invoking
this helper when chaining tokens.

### 4. Signature Verification Is an Input Contract Here

**Choice:** The envelope validator consumes a `sig_valid` input rather
than re-deriving the issuer key or canonical payload bytes internally.

**Rationale:** Existing signature and DID modules cover those lower-level
primitives, but this change does not solve canonical payload-byte
construction. Modeling signature success as an input keeps the token
module aligned with the implemented trust boundary instead of inventing
a proof it cannot justify yet.

### 5. Reuses Existing Modules

**Choice:** Temporal checks delegate to UCAN temporal logic, attenuation
logic reuses containment-style reasoning, and the signature layer is
referred to through the `sig_valid` contract.

**Rationale:** Those modules are already verified or scoped separately.
The token envelope proves that their composition is sound, not
re-proves their internals.

## Risks / Trade-offs

**[Canonical-bytes trust boundary]** → Without canonical serialization,
we cannot prove that the bytes signed match the payload struct. This is
deferred to a future change. For now, the token module makes that boundary
explicit: the caller must supply a sound `sig_valid` result for the payload
under validation, and archive evidence must mention this assumption.
