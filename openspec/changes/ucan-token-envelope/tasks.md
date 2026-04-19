## Phase 1: Payload Structure

- [ ] Define `UcanPayload` struct with iss, aud, att, prf, exp, nbf, nnc, fct fields
- [ ] Define `UcanToken` (payload + signature) and `UcanEnvelope` (validated)
- [ ] Define `ValidationError` enum with per-check variants
- [ ] Add `r[ucan-token.*]` requirements to `docs/requirements.md`

## Phase 2: Sub-Checks

- [ ] Implement `check_temporal_validity(exp, nbf, now, tolerance)` delegating to existing `token_timestamp`
- [ ] Implement `check_issuer_audience(iss, aud)` — iss != aud
- [ ] Implement `check_attenuation_chain(parent_att, child_att)` delegating to existing `containment`
- [ ] Implement `check_required_fields(payload)` — iss, aud, att non-empty

## Phase 3: Validation Pipeline

- [ ] Implement `validate_envelope(token, now, tolerance) -> Result<UcanEnvelope, ValidationError>`
- [ ] Prove soundness: `validate_envelope ok ==> all sub-checks pass`
- [ ] Prove completeness: `all sub-checks pass ==> validate_envelope ok`
- [ ] Prove self-delegation rejection: `iss == aud ==> Err`
- [ ] Prove attenuation monotonicity composes with containment transitivity

## Phase 4: Tests and Validation

- [ ] Property tests: valid envelope with random capabilities
- [ ] Property tests: each failure mode triggered independently
- [ ] Property tests: attenuation with random capability narrowing
- [ ] Add `r[impl]` and `r[verify]` Tracey tags
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
