## Why

A UCAN token is more than a bag of capabilities — it has a defined
envelope structure with fields that must be validated together:
issuer (iss), audience (aud), attenuations (att), proofs (prf),
expiration (exp), not-before (nbf), nonce (nnc), and facts (fct).
The existing capability-auth modules verify individual authorization
factors (scope, mode, subsumption) and timestamp checks, but there
is no model that ties these into the actual token structure where
fields reference each other and constraints compose.

The token validation pipeline is the real integration point: parse
the envelope, verify the signature over canonical bytes, check
temporal validity, walk the proof chain, and confirm attenuation
at each hop. Getting this pipeline wrong means either accepting
invalid tokens (security hole) or rejecting valid ones (broken
delegation). The existing chain-validation proofs assume a valid
token sequence; this change models what "valid token" means.

## What Changes

A new module `src/ucan_token.rs` providing:

**Data model:**
- `UcanPayload` — the UCAN payload fields: iss (Did), aud (Did), att (Vec<Capability>),
  prf (Vec<Cid>), exp (Option<u64>), nbf (Option<u64>), nnc (Option<Nonce>), fct (Vec<Fact>)
- `UcanToken` — payload + signature bytes
- `UcanEnvelope` — parsed and validated token ready for authorization

**Validation:**
- `validate_envelope(token, now, tolerance) -> Result<UcanEnvelope, ValidationError>` — the full pipeline
- `check_temporal_validity(exp, nbf, now, tolerance) -> bool`
- `check_issuer_audience(iss, aud) -> bool` — issuer != audience
- `check_attenuation_chain(parent_att, child_att) -> bool` — each child att contained by some parent att

**Verified properties:**
- Temporal composability: token-timestamp properties (already proven) compose correctly inside the envelope
- Issuer-audience distinctness: self-delegation is rejected
- Attenuation monotonicity: child token never exceeds parent capabilities
- Validation completeness: if all sub-checks pass, validate_envelope succeeds
- Validation soundness: if validate_envelope succeeds, all sub-checks hold
- Field presence: required fields (iss, aud, att) are never empty after validation

This module integrates the existing `token_timestamp`, `scope`, `subsumption`,
`containment`, and `authorize` modules into a single token-level decision.

## Capabilities

### New Capabilities
- `ucan-token.temporal-composability`: timestamp checks compose in envelope context
- `ucan-token.self-delegation-rejected`: iss == aud always fails
- `ucan-token.attenuation-monotone`: child att ⊆ parent att at each hop
- `ucan-token.validation-complete`: all checks pass implies valid envelope
- `ucan-token.validation-sound`: valid envelope implies all checks passed
- `ucan-token.required-fields`: iss, aud, att always present after validation

## Impact

- **Files**: `src/ucan_token.rs` (new)
- **APIs**: ~8 pub fns (validate_envelope, temporal check, attenuation check, field accessors)
- **Dependencies**: Depends on existing capability-auth modules (scope, containment, token_timestamp)
- **Testing**: Property tests for valid/invalid envelope combinations; Verus proofs for pipeline soundness/completeness
