# UCAN Token Envelope Specification

## Purpose

Defines the UCAN token structure, field validation rules, and the
full validation pipeline that integrates signature verification,
temporal checks, and attenuation chain walking. Builds on the
existing capability-auth, ucan-signature, and ucan-did modules.

## Definitions

- **UCAN payload**: The structured data: iss (DID), aud (DID),
  att (capabilities), prf (proof CIDs), exp (expiration), nbf
  (not-before), nnc (nonce), fct (facts).
- **UCAN token**: A payload plus its signature bytes.
- **Validation**: The full check: signature verification, temporal
  validity, field constraints, and attenuation chain correctness.
- **Attenuation chain**: Each child token's att MUST be contained
  by its parent's att.

## Requirements

### Temporal Validity

#### Requirement: Temporal Checks Compose in Envelope

The envelope's temporal validity MUST be exactly the conjunction
of not-expired (considering tolerance) and not-before-satisfied.

`temporal_valid(exp, nbf, now, tol) == !is_expired(exp, now, tol) && nbf_satisfied(nbf, now, tol)`

##### Scenario: Valid window

- GIVEN exp = now + 3600, nbf = now - 60, tolerance = 30
- WHEN checking temporal validity at now
- THEN valid

##### Scenario: Expired

- GIVEN exp = now - 100, tolerance = 30
- WHEN checking temporal validity at now
- THEN invalid (expired beyond tolerance)

##### Scenario: Not yet valid

- GIVEN nbf = now + 100, tolerance = 30
- WHEN checking temporal validity at now
- THEN invalid (nbf not yet reached even with tolerance)

### Identity Constraints

#### Requirement: Self-Delegation Rejected

A UCAN token where issuer equals audience MUST be rejected.

`iss == aud ==> validate_envelope fails`

##### Scenario: Same DID for issuer and audience

- GIVEN iss = "did:key:z..." and aud = same DID
- WHEN validating
- THEN validation fails

### Attenuation

#### Requirement: Attenuation Is Monotone

Each child token's capabilities MUST be contained by its parent's
capabilities. No capability in the child may exceed any axis of
the parent.

`∀ child_cap in child.att: ∃ parent_cap in parent.att: contains(parent_cap, child_cap)`

##### Scenario: Child narrows scope

- GIVEN parent att = [{scope: "/data/", mode: Full}], child att = [{scope: "/data/sub/", mode: Read}]
- WHEN checking attenuation
- THEN valid (child is narrower on both scope and mode)

##### Scenario: Child widens scope rejected

- GIVEN parent att = [{scope: "/data/sub/", mode: Read}], child att = [{scope: "/data/", mode: Read}]
- WHEN checking attenuation
- THEN rejected (child scope is wider)

### Validation Pipeline

#### Requirement: Validation Is Sound

If validate_envelope succeeds, then signature is valid, temporal
constraints hold, identity constraints hold, and attenuation is
monotone.

##### Scenario: Valid token passes all sub-checks

- GIVEN a token that passes validate_envelope
- THEN verify(iss_pk, payload_bytes, sig) == true
- AND temporal_valid(exp, nbf, now, tol) == true
- AND iss != aud
- AND attenuation chain is monotone

#### Requirement: Validation Is Complete

If all sub-checks pass individually, validate_envelope MUST succeed.

##### Scenario: All checks green means valid

- GIVEN signature valid, temporal valid, iss != aud, attenuation monotone
- WHEN calling validate_envelope
- THEN succeeds

### Required Fields

#### Requirement: Issuer Is Always Present

A validated UCAN MUST have a non-empty issuer DID.

#### Requirement: Audience Is Always Present

A validated UCAN MUST have a non-empty audience DID.

#### Requirement: Attenuations Are Always Present

A validated UCAN MUST have at least one capability in att.
