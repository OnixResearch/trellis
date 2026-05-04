## Why

`verified-logic` has finite resource, constraint, and optimization primitives,
but it still lacks a focused family for access-control policy building blocks.
Future authorization, capability, workflow, resource, and orchestration models
need exact finite principal/resource/action/rule/grant/denial/conflict/delegation
primitives without claiming cryptographic strength or whole-system security.

This change implements a focused exact finite access-control policy primitive
family with Tracey coverage, tests, Verus proofs, evidence, and archive work.

## What Changes

- Add reusable finite access-control policy requirements for:
  - `policy_principal`: finite principal identity/admission metadata.
  - `policy_resource`: finite resource identity/admission metadata.
  - `policy_action`: finite action identity/admission metadata.
  - `policy_rule`: rule envelopes over principal/resource/action scopes.
  - `policy_grant_denial`: deterministic grant/denial decision metadata.
  - `policy_conflict`: conflict-resolution envelopes for overlapping rules.
  - `policy_delegation`: finite delegation metadata with explicit scope and depth
    bounds.
  - `policy_authorization`: deterministic authorization admission/rejection
    envelopes.
- Exclude cryptographic authentication, key ownership, distributed identity,
  external policy-engine correctness, whole-system security, information-flow
  security, side-channel resistance, and compliance claims unless a later focused
  OpenSpec owns those models.

## Capabilities

### New Capabilities

- `domain-primitives.access-control-policy.principal`: finite principal metadata.
- `domain-primitives.access-control-policy.resource`: finite resource metadata.
- `domain-primitives.access-control-policy.action`: finite action metadata.
- `domain-primitives.access-control-policy.rule`: scoped policy-rule envelopes.
- `domain-primitives.access-control-policy.grant-denial`: deterministic decision
  metadata.
- `domain-primitives.access-control-policy.conflict`: conflict-resolution
  metadata.
- `domain-primitives.access-control-policy.delegation`: bounded delegation
  envelopes.
- `domain-primitives.access-control-policy.authorization`: deterministic
  authorization admission envelopes.

## Impact

- **Files**: OpenSpec artifacts plus `src/policy_*`, `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: adds focused `src/policy_*` exact finite admission/checker APIs.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final repo validation before archive.

## Out of Scope

- cryptographic authentication, signature verification, or key-control claims
- whole-system security, information-flow, side-channel, compliance, or external
  policy-engine correctness claims
- distributed identity/consensus or runtime enforcement guarantees
