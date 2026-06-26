# UCAN Token Envelope Specification

## Purpose

Defines the UCAN token structure, field validation rules, and the
single-token validation pipeline that integrates a supplied signature
verification result with temporal checks and identity/field constraints.
Also defines a separate attenuation helper over parent/child capability
lists. Builds on the existing UCAN modules.

## Definitions

- **UCAN payload**: The structured data modeled in this change:
  iss (DID), aud (DID), att (capabilities), prf (proof CIDs),
  exp (expiration), nbf (not-before).
- **Deferred payload fields**: `nnc` and `fct` are intentionally out of
  scope for this change and are not validated by the current token module.
- **UCAN token**: A payload plus its signature bytes.
- **Validation**: The single-token check: supplied signature-valid bit,
  temporal validity, and field/identity constraints.
- **Attenuation check**: A separate pure check where each child token's
  att MUST be contained by a supplied parent att list.

## Requirements

**Temporal Validity**

### Requirement: Temporal Checks Compose in Envelope [r[ucan-token.temporal-validity]]

The envelope's temporal validity MUST be exactly the conjunction
of not-expired (considering tolerance) and not-before-satisfied.

`temporal_valid(exp, nbf, now, tol) == !is_expired(exp, now, tol) && nbf_satisfied(nbf, now, tol)`

#### Scenario: Valid window

- GIVEN exp = now + 3600, nbf = now - 60, tolerance = 30
- WHEN checking temporal validity at now
- THEN valid

#### Scenario: Expired

- GIVEN exp = now - 100, tolerance = 30
- WHEN checking temporal validity at now
- THEN invalid (expired beyond tolerance)

#### Scenario: Not yet valid

- GIVEN nbf = now + 100, tolerance = 30
- WHEN checking temporal validity at now
- THEN invalid (nbf not yet reached even with tolerance)

**Identity Constraints**

### Requirement: Self-Delegation Rejected [r[ucan-token.self-delegation-rejected]]

A UCAN token where issuer equals audience MUST be rejected.

`iss == aud ==> validate_envelope fails`

#### Scenario: Same DID for issuer and audience

- GIVEN iss = "did:key:z..." and aud = same DID
- WHEN validating
- THEN validation fails

**Attenuation**

### Requirement: Attenuation Is Monotone [r[ucan-token.attenuation-monotone]]

Attenuation MUST require each child capability to be contained by some parent
capability for a supplied parent capability list and child capability list. No
capability in the child may exceed any axis of the parent.

`∀ child_cap in child.att: ∃ parent_cap in parent.att: contains(parent_cap, child_cap)`

#### Scenario: Child narrows scope

- GIVEN parent att = [{scope: "/data/", mode: Full}], child att = [{scope: "/data/sub/", mode: Read}]
- WHEN checking attenuation
- THEN valid (child is narrower on both scope and mode)

#### Scenario: Child widens scope rejected

- GIVEN parent att = [{scope: "/data/sub/", mode: Read}], child att = [{scope: "/data/", mode: Read}]
- WHEN checking attenuation
- THEN rejected (child scope is wider)

**Validation Pipeline**

### Requirement: Validation Is Sound [r[ucan-token.validation-sound]]

Validation success MUST imply that the supplied signature-valid input is true,
temporal constraints hold, identity constraints hold, and required fields are
present.

#### Scenario: Valid token passes all sub-checks

- GIVEN a payload and signature-valid input that pass validate_envelope
- THEN sig_valid == true
- AND temporal_valid(exp, nbf, now, tol) == true
- AND iss != aud
- AND required fields are present

### Requirement: Validation Is Complete [r[ucan-token.validation-complete]]

If all single-token sub-checks pass individually, validate_envelope MUST succeed.

#### Scenario: All checks green means valid

- GIVEN sig_valid, temporal valid, iss != aud, and required fields present
- WHEN calling validate_envelope
- THEN succeeds

**Required Fields**

### Requirement: Issuer Is Always Present [r[ucan-token.issuer-present]]

A validated UCAN MUST have a non-empty issuer DID.

#### Scenario: Missing issuer rejected

- GIVEN a token payload with an empty issuer DID
- WHEN validating the token envelope
- THEN validation rejects the payload

### Requirement: Audience Is Always Present [r[ucan-token.audience-present]]

A validated UCAN MUST have a non-empty audience DID.

#### Scenario: Missing audience rejected

- GIVEN a token payload with an empty audience DID
- WHEN validating the token envelope
- THEN validation rejects the payload

### Requirement: Attenuations Are Always Present [r[ucan-token.attenuations-present]]

A validated UCAN MUST have at least one capability in att.

#### Scenario: Empty attenuation list rejected

- GIVEN a token payload with no attenuations
- WHEN validating the token envelope
- THEN validation rejects the payload

**Constraints**

### Requirement: Canonical Byte Construction Is Out Of Scope Here [r[ucan-token.canonical-bytes-assumed]]

This change MUST treat canonical payload-byte construction as an external
assumption. The token validation layer consumes a supplied signature-valid
result and does not prove that payload bytes were derived canonically from
the payload struct.

#### Scenario: Signature bit supplied externally

- GIVEN a payload struct and a caller-supplied signature-valid result
- WHEN the token envelope is validated
- THEN validation uses the supplied signature result without deriving canonical
  bytes inside this layer
