## Coverage Index

| Requirement ID | Future module |
|---|---|
| `domain-primitives.access-control-policy.principal` | `src/policy_principal.rs` |
| `domain-primitives.access-control-policy.resource` | `src/policy_resource.rs` |
| `domain-primitives.access-control-policy.action` | `src/policy_action.rs` |
| `domain-primitives.access-control-policy.rule` | `src/policy_rule.rs` |
| `domain-primitives.access-control-policy.grant-denial` | `src/policy_grant_denial.rs` |
| `domain-primitives.access-control-policy.conflict` | `src/policy_conflict.rs` |
| `domain-primitives.access-control-policy.delegation` | `src/policy_delegation.rs` |
| `domain-primitives.access-control-policy.authorization` | `src/policy_authorization.rs` |

## Phase 1: Spec foundation

- [x] Author spec-only access-control policy primitive OpenSpec and umbrella taxonomy update. `[covers=domain-primitives.access-control-policy.principal,domain-primitives.access-control-policy.resource,domain-primitives.access-control-policy.action,domain-primitives.access-control-policy.rule,domain-primitives.access-control-policy.grant-denial,domain-primitives.access-control-policy.conflict,domain-primitives.access-control-policy.delegation,domain-primitives.access-control-policy.authorization]`

## Phase 2: Future implementation slices

- [x] Implement finite principal admission metadata. `[covers=domain-primitives.access-control-policy.principal]`
- [x] Implement finite resource admission metadata. `[covers=domain-primitives.access-control-policy.resource]`
- [x] Implement finite action admission metadata. `[covers=domain-primitives.access-control-policy.action]`
- [x] Implement scoped policy-rule envelopes. `[covers=domain-primitives.access-control-policy.rule]`
- [x] Implement deterministic grant/denial decision metadata. `[covers=domain-primitives.access-control-policy.grant-denial]`
- [x] Implement conflict-resolution envelopes for overlapping rules. `[covers=domain-primitives.access-control-policy.conflict]`
- [x] Implement bounded delegation envelopes. `[covers=domain-primitives.access-control-policy.delegation]`
- [x] Implement deterministic authorization admission/rejection envelopes. `[covers=domain-primitives.access-control-policy.authorization]`

## Phase 3: Future verification and archive

- [x] Add `docs/requirements.md` anchors and Tracey `r[impl]`/`r[depends]`/`r[verify]` coverage for implemented primitives.
- [x] Run focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested queries, and final repo validation.
- [x] Sync delta specs and archive after all implementation/evidence tasks are complete.
