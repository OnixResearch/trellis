# Upstream Verus Corpus Specification Delta

## ADDED Requirements

### Requirement: Trellis consumes the central Octet formatter profile [r[trellis.upstream_verus_corpus.octet_toolchain_consumption]]

Trellis MUST consume its Verus formatter and profile artifacts from an exact published Octet revision and MUST NOT reconstruct those packages through a local release fallback. The formatter rail MUST fail when required Octet outputs are absent or when the selected profile, compatibility row, or formatter identity differs from the reviewed values.

This dependency MUST NOT silently replace Trellis's independent proof-verifier pin. A verifier upgrade MUST remain subject to Trellis's local corpus and proof validation.

#### Scenario: Central formatter profile is admitted

- GIVEN the pinned Octet input exposes the reviewed formatter and profile packages
- WHEN the Trellis formatter ratchet runs
- THEN it MUST validate the central profile and Trellis compatibility row
- AND it MUST execute the formatter from the Octet package

#### Scenario: Profile drift fails closed

- GIVEN an absent Octet package or a profile with a mismatched identity, formatter version, or Trellis compatibility status
- WHEN toolchain admission runs
- THEN the check MUST fail before formatting evidence is accepted
- AND it MUST NOT reconstruct a local formatter fallback

#### Scenario: Verifier authority remains local

- GIVEN Octet publishes a newer production verifier than the current Trellis pin
- WHEN the formatter dependency is consumed
- THEN Trellis MUST retain its reviewed local verifier until a separate corpus-backed upgrade succeeds
- AND formatter or profile identity MUST NOT be reported as proof evidence
