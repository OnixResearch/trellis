## Why

Identifier and namespace handling often mixes raw IDs, scoped uniqueness,
canonical forms, aliases, collision checks, lookup behavior, and registration
metadata in ad hoc code. A focused identifier/namespace primitive family gives
future changes exact finite building blocks for IDs, namespaces, scoped
uniqueness, alias maps, canonicalization decisions, collision/admission metadata,
and deterministic lookup/registration envelopes without claiming distributed
identity, authentication, or security behavior.

## What Changes

- Add a implemented `add-identifier-namespace-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for identifier envelopes, namespace scopes, scoped
  uniqueness predicates, alias maps, canonicalization decisions,
  collision/admission metadata, deterministic lookup, and deterministic
  registration admission.
- Include implementation, Verus proofs, Tracey coverage, and evidence for the finite envelope primitives.

## Capabilities

### New Capabilities

- `domain-primitives.identifier-namespace.identifier`: finite identifier
  envelopes and ID-shape admission.
- `domain-primitives.identifier-namespace.namespace`: finite namespace scope
  metadata.
- `domain-primitives.identifier-namespace.scoped-uniqueness`: scoped uniqueness
  predicates over finite registries.
- `domain-primitives.identifier-namespace.alias-map`: finite alias mapping and
  target admission metadata.
- `domain-primitives.identifier-namespace.canonicalization`: explicit
  canonicalization decision envelopes.
- `domain-primitives.identifier-namespace.collision`: deterministic collision
  metadata.
- `domain-primitives.identifier-namespace.lookup`: deterministic lookup result
  envelopes.
- `domain-primitives.identifier-namespace.registration`: deterministic
  registration admission/rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts and implementation add `src/id_*`, `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: exact finite runtime admission APIs are added.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation plus focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- distributed identity, global uniqueness across systems, consensus, registry
  durability, service availability, or external directory correctness
- authentication, authorization, cryptographic ownership, privacy, security, or
  anti-spoofing claims
- UUID/ULID/randomness quality, hash collision resistance, DNS/URI standard
  conformance, filesystem path semantics, or database uniqueness enforcement
- performance, caching, migration, or lifecycle policy completeness
