## Phase 1: Payload Structure

- [x] Define `UcanPayload` struct with iss, aud, att, prf, exp, nbf, nnc, fct fields
- [x] Define `UcanToken` (payload + signature) and `UcanEnvelope` (validated)
- [x] Define `ValidationError` enum with per-check variants
- [x] Add `r[ucan-token.*]` requirements to `docs/requirements.md`

## Phase 2: Sub-Checks

- [x] Implement `check_temporal_validity(exp, nbf, now, tolerance)` delegating to existing `token_timestamp`
- [x] Implement `check_issuer_audience(iss, aud)` — iss != aud
- [x] Implement `check_attenuation_chain(parent_att, child_att)` delegating to existing `containment`
- [x] Implement `check_required_fields(payload)` — iss, aud, att non-empty

## Phase 3: Validation Pipeline

- [x] Implement `validate_envelope(token, now, tolerance) -> Result<UcanEnvelope, ValidationError>`
- [x] Prove soundness: `validate_envelope ok ==> all sub-checks pass`
- [x] Prove completeness: `all sub-checks pass ==> validate_envelope ok`
- [x] Prove self-delegation rejection: `iss == aud ==> Err`
- [x] Prove attenuation monotonicity composes with containment transitivity

## Phase 4: Tests and Validation

- [x] Property tests: valid envelope with random capabilities
- [x] Property tests: each failure mode triggered independently
- [x] Property tests: attenuation with random capability narrowing
- [x] Add `r[impl]` and `r[verify]` Tracey tags
- [x] Run `./scripts/verify.sh` green — 9 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
