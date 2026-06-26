## Phase 1: Scope and Traceability

- [x] Align proposal, design, and delta spec on single-token validation scope; keep proof-chain resolution and CID lookup explicitly out of scope
- [x] Add concrete `r[ucan-token.*]` requirement IDs to the change spec and `docs/requirements.md`, including the canonical-bytes assumption boundary
- [x] Reconcile change artifacts with the implemented `src/ucan_token.rs` surface (`UcanPayload`, `UcanToken`, `ValidationError`, validation helpers)
- [x] Reconcile the payload-field story for `prf`, `nnc`, and `fct` against the currently implemented surface and the final change scope

## Phase 2: Sub-Checks and Contracts

- [x] Implement or confirm `check_temporal_validity(exp, nbf, now, tolerance)` against the current UCAN temporal contract
- [x] Implement or confirm `check_issuer_audience(iss, aud)` — iss != aud
- [x] Implement or confirm `check_required_fields(payload)` — iss, aud, att non-empty
- [x] Implement or confirm `check_attenuation_chain(parent_att, child_att)` as a separate parent/child helper, not proof-chain resolution
- [x] Define the signature boundary explicitly: `validate_envelope` consumes a supplied `sig_valid` result and does not construct canonical payload bytes
- [x] Tie `r[ucan-token.canonical-bytes-assumed]` to explicit proof/test/archive evidence so Tracey coverage is actionable

## Phase 3: Validation Pipeline

- [x] Implement or confirm `validate_envelope(payload, sig_valid, now, tolerance) -> Result<_, ValidationError>` for single-token validation only
- [x] Prove soundness: `validate_envelope ok ==> sig_valid && temporal_valid && iss != aud && required fields present`
- [x] Prove completeness: `sig_valid && temporal_valid && iss != aud && required fields present ==> validate_envelope ok`
- [x] Prove self-delegation rejection: `iss == aud ==> Err`
- [x] Prove attenuation monotonicity for the separate parent/child capability check
- [x] Reconcile the originally proposed `UcanEnvelope` wrapper with the landed payload-centric API surface
- [x] Reconcile `ValidationError` against the current failure modes and confirm each negative-path test maps to an expected variant family

## Phase 4: Tests and Validation Evidence

- [x] Run targeted baseline checks before changes for the UCAN token surface and record results
- [x] Add or confirm positive tests: valid envelope succeeds under valid signature/time/identity/fields
- [x] Add or confirm negative tests: expired, not-yet-valid, self-delegation, missing fields, invalid signature
- [x] Add or confirm attenuation tests for narrowing vs widening parent/child capability sets
- [x] Add `r[impl]` and `r[verify]` Tracey tags to all public token-surface items, including explicit treatment of the canonical-bytes and required-fields requirements
- [x] Run fresh validation (`cargo test`, `cargo clippy`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `tracey query uncovered`, `tracey query untested`) and save evidence

## Phase 5: Sync and Archive

- [x] Sync the landed UCAN token spec into canonical repo docs/spec state as needed
- [x] Archive `openspec/changes/ucan-token-envelope` with fresh evidence once validation is green
