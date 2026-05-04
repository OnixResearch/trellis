## Why

Configuration and feature-flag logic often mixes key lookup, typed-value
admission, default selection, overrides, and rollout decisions in ad hoc
runtime code. A focused configuration/feature-flag primitive family gives future
changes exact finite building blocks for config keys, typed values, defaults,
override precedence, feature gates, rollout buckets, validation decisions, and
deterministic config admission without claiming distributed config-store or flag
service behavior.

## What Changes

- Implement the `add-configuration-feature-flag-primitives` child OpenSpec
  under the domain-primitives umbrella with exact finite Rust/Verus modules.
- Define exact finite models for configuration keys, typed values, default
  metadata, override precedence, feature-gate predicates, rollout buckets,
  validation decisions, and deterministic config admission.
- Add Verus contracts, Tracey coverage, focused tests, and reviewable evidence for
  the finite primitive family.

## Capabilities

### New Capabilities

- `domain-primitives.configuration-feature-flag.key`: finite config-key and
  namespace admission.
- `domain-primitives.configuration-feature-flag.typed-value`: finite typed-value
  envelope admission.
- `domain-primitives.configuration-feature-flag.default`: default-value metadata
  tied to declared key/type envelopes.
- `domain-primitives.configuration-feature-flag.override-precedence`:
  deterministic override precedence metadata.
- `domain-primitives.configuration-feature-flag.gate`: finite feature-gate
  predicate envelope.
- `domain-primitives.configuration-feature-flag.rollout-bucket`: deterministic
  rollout-bucket metadata over finite subjects and bucket ranges.
- `domain-primitives.configuration-feature-flag.validation-decision`: explicit
  validation decision metadata.
- `domain-primitives.configuration-feature-flag.admission`: deterministic config
  admission/rejection envelope.

## Impact

- **Files**: `src/config_*`, `src/feature_flag_*`, `src/lib.rs`,
  `docs/requirements.md`, canonical domain-primitives spec, tests, and evidence.
- **APIs**: exact finite admission/checker APIs are added for config keys, typed
  values, defaults, override precedence, gates, rollout buckets, validation
  decisions, and composed admission.
- **Dependencies**: no new dependencies.
- **Testing**: focused tests, Verus, Tracey prefix checks, strict OpenSpec
  validation, and full repo validation.

## Out of Scope

- runtime config-store, database, filesystem, environment-variable, or secret
  manager correctness
- distributed rollout consistency, eventual consistency, service availability,
  or flag-delivery guarantees
- percentage-rollout statistical quality, randomization quality, A/B test
  validity, experimentation correctness, or business-rule completeness
- access-control, authentication, audit/security, or privacy claims
- wall-clock scheduling, hot reload, caching, performance, or observability
  backend behavior
