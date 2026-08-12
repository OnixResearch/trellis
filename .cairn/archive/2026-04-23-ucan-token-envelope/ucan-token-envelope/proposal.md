## Why

A UCAN token is more than a bag of capabilities — it has a defined
envelope structure with fields that must be validated together:
issuer (iss), audience (aud), attenuations (att), proofs (prf),
expiration (exp), not-before (nbf), nonce (nnc), and facts (fct).
The existing capability-auth modules verify individual authorization
factors (scope, mode, subsumption) and timestamp checks, but there
is no model that ties these into the actual token structure where
fields reference each other and constraints compose.

The token validation pipeline is the real integration point: check
that required fields are present, verify the signature result supplied
for this token payload, validate the temporal window, and reject
self-delegation. Attenuation remains a separate pure check over a
parent capability list and a child capability list; proof-chain
resolution and CID lookup stay outside this change. Getting this
pipeline wrong means either accepting invalid tokens (security hole)
or rejecting valid ones (broken delegation). The existing chain-
validation proofs assume a valid token sequence; this change models
what "valid single-token validation" means.

## What Changes

A new module `src/ucan_token.rs` providing:

**Data model:**
- `UcanPayload` — the currently implemented payload fields for this change: iss, aud, att, prf, exp, nbf
- `UcanToken` — payload + signature bytes

This change does not currently expose a stable `UcanEnvelope` API in the
landed surface; validation returns `Result<_, ValidationError>` over the
payload-centric interface instead.

The original draft mentioned `nnc` and `fct`, but the landed scope does not
validate or store them yet. They are deferred rather than partially modeled.

**Validation:**
- `validate_envelope(payload, sig_valid, now, tolerance) -> Result<_, ValidationError>` — the single-token pipeline
- `check_temporal_validity(exp, nbf, now, tolerance) -> bool`
- `check_issuer_audience(iss, aud) -> bool` — issuer != audience
- `check_required_fields(payload) -> bool`
- `check_attenuation_chain(parent_att, child_att) -> bool` — separate parent/child capability check, not proof resolution

**Verified properties:**
- Temporal validity: expiration and not-before checks compose correctly inside the envelope
- Issuer-audience distinctness: self-delegation is rejected
- Attenuation monotonicity: a child capability list can be checked against a parent capability list via containment
- Validation completeness: if all single-token sub-checks pass, validate_envelope succeeds
- Validation soundness: if validate_envelope succeeds, all single-token sub-checks hold
- Field presence: required fields (iss, aud, att) are never empty after validation

This module integrates the existing temporal and containment-style UCAN logic into a single-token validation decision. Signature verification is modeled at this layer through a `sig_valid` input that must come from the signature module; canonical payload-byte construction remains a documented trust boundary outside this change.

## Capabilities

### New Capabilities
- `ucan-token.temporal-validity`: timestamp checks compose in envelope context
- `ucan-token.self-delegation-rejected`: iss == aud always fails
- `ucan-token.attenuation-monotone`: child att is contained by a supplied parent att list
- `ucan-token.validation-complete`: all single-token checks pass implies valid envelope
- `ucan-token.validation-sound`: valid envelope implies all single-token checks passed
- `ucan-token.issuer-present`: issuer is non-empty after validation
- `ucan-token.audience-present`: audience is non-empty after validation
- `ucan-token.attenuations-present`: att is non-empty after validation

## Impact

- **Files**: `src/ucan_token.rs`, `docs/requirements.md`, `openspec/changes/ucan-token-envelope/**`
- **APIs**: single-token validation helpers plus a separate attenuation check helper
- **Dependencies**: Depends on existing UCAN temporal/containment logic; signature verification result is supplied by the caller
- **Testing**: Runtime tests for valid/invalid envelope combinations and proof artifacts for pipeline soundness/completeness
