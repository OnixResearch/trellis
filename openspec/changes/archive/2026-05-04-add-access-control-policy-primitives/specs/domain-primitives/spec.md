# Access-Control Policy Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite access-control policy
primitives. These primitives MUST use exact finite principals, resources,
actions, policy rules, grant/denial decisions, conflict-resolution envelopes,
bounded delegation metadata, and deterministic authorization admission results.
They MUST avoid cryptographic authentication, key-ownership, whole-system
security, information-flow security, side-channel resistance, external policy
engine correctness, distributed authorization consensus, or runtime enforcement
claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Finite Principal Envelope [r[domain-primitives.access-control-policy.principal]]
The system MUST define finite principal admission metadata for policy decision
models.

#### Scenario: Principal is declared before rule use [r[domain-primitives.access-control-policy.principal.scenario.declared-before-rule-use]]

- GIVEN finite principal metadata and a policy rule
- WHEN the rule references a principal
- THEN the principal is admitted by the declared principal envelope

### Requirement: Finite Resource Envelope [r[domain-primitives.access-control-policy.resource]]
The system MUST define finite resource admission metadata for policy decision
models.

#### Scenario: Resource is declared before authorization [r[domain-primitives.access-control-policy.resource.scenario.declared-before-authorization]]

- GIVEN finite resource metadata and an authorization request
- WHEN the request references a resource
- THEN the resource is admitted by the declared resource envelope

### Requirement: Finite Action Envelope [r[domain-primitives.access-control-policy.action]]
The system MUST define finite action admission metadata for policy decision
models.

#### Scenario: Action is declared before authorization [r[domain-primitives.access-control-policy.action.scenario.declared-before-authorization]]

- GIVEN finite action metadata and an authorization request
- WHEN the request references an action
- THEN the action is admitted by the declared action envelope

### Requirement: Policy Rule Envelope [r[domain-primitives.access-control-policy.rule]]
The system MUST define policy-rule envelopes over finite principal, resource, and
action scopes with exact effect metadata.

#### Scenario: Rule scope uses admitted values [r[domain-primitives.access-control-policy.rule.scenario.scope-uses-admitted-values]]

- GIVEN a policy-rule envelope
- WHEN the rule scope is evaluated
- THEN every principal, resource, and action referenced by the rule is admitted by its declared envelope

### Requirement: Grant and Denial Decision Metadata [r[domain-primitives.access-control-policy.grant-denial]]
The system MUST define deterministic grant/denial decision metadata for admitted
authorization requests.

#### Scenario: Decision records matched rule metadata [r[domain-primitives.access-control-policy.grant-denial.scenario.records-matched-rule-metadata]]

- GIVEN an admitted authorization request and finite policy rules
- WHEN a grant or denial decision is produced
- THEN the decision metadata identifies the matched admitted rule or the deterministic no-match result

### Requirement: Conflict Resolution Envelope [r[domain-primitives.access-control-policy.conflict]]
The system MUST define deterministic conflict-resolution envelopes for overlapping
finite policy rules.

#### Scenario: Overlapping rules resolve deterministically [r[domain-primitives.access-control-policy.conflict.scenario.overlapping-rules-resolve-deterministically]]

- GIVEN overlapping admitted policy rules for the same principal, resource, and action
- WHEN conflict resolution is evaluated
- THEN the result follows declared priority or tie-break metadata deterministically

### Requirement: Bounded Delegation Envelope [r[domain-primitives.access-control-policy.delegation]]
The system MUST define bounded delegation metadata over finite principals,
resources, actions, scopes, and chain-depth envelopes.

#### Scenario: Delegation states scope and bound [r[domain-primitives.access-control-policy.delegation.scenario.states-scope-and-bound]]

- GIVEN delegation metadata between admitted principals
- WHEN the delegation is used in an authorization model
- THEN it states the admitted resource/action scope and finite depth or chain bound

### Requirement: Authorization Admission Envelope [r[domain-primitives.access-control-policy.authorization]]
The system MUST define deterministic authorization admission results for policy
metadata, including invalid principal, resource, action, rule, conflict, and
delegation shapes.

#### Scenario: Rejects undeclared principal deterministically [r[domain-primitives.access-control-policy.authorization.scenario.rejects-undeclared-principal-deterministically]]

- GIVEN authorization metadata with a request principal outside the declared principal envelope
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming cryptographic authentication or whole-system security
