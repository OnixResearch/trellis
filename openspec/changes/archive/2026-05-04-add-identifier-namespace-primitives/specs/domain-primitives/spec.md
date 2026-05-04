# Identifier Namespace Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite identifier/namespace
primitives. These primitives MUST use exact finite identifier envelopes,
namespace scopes, scoped uniqueness predicates, alias maps, canonicalization
decisions, collision/admission metadata, deterministic lookup results, and
registration admission/rejection envelopes. They MUST avoid distributed identity,
global uniqueness across systems, consensus, registry durability, authentication,
authorization, cryptographic ownership, security, privacy, UUID/ULID randomness
quality, hash collision resistance, external directory correctness, or broad
standard-conformance claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Identifier Envelope [r[domain-primitives.identifier-namespace.identifier]]
The system MUST provide finite identifier envelopes that record identifier bytes
or symbols, declared kind metadata, and deterministic ID-shape admission results.

#### Scenario: Malformed identifier rejected explicitly [r[domain-primitives.identifier-namespace.identifier.scenario.malformed-identifier-rejected-explicitly]]

- GIVEN finite identifier shape metadata
- WHEN a candidate identifier does not match the admitted shape
- THEN the identifier primitive returns an explicit malformed-ID decision without consulting an external identity service

### Requirement: Namespace Scope Envelope [r[domain-primitives.identifier-namespace.namespace]]
The system MUST provide finite namespace scope envelopes that define admitted
scope identifiers, parent/child scope metadata, and deterministic namespace
membership decisions.

#### Scenario: Unknown namespace rejected before lookup [r[domain-primitives.identifier-namespace.namespace.scenario.unknown-namespace-rejected-before-lookup]]

- GIVEN a finite admitted namespace set
- WHEN a candidate namespace is absent from that set
- THEN the namespace primitive rejects the namespace before any lookup or registration decision is made

### Requirement: Scoped Uniqueness Predicate [r[domain-primitives.identifier-namespace.scoped-uniqueness]]
The system MUST provide scoped uniqueness predicates over finite registry
snapshots and declared namespace scopes, returning explicit unique or duplicate
decisions.

#### Scenario: Duplicate only applies within declared scope [r[domain-primitives.identifier-namespace.scoped-uniqueness.scenario.duplicate-only-applies-within-declared-scope]]

- GIVEN two finite registry entries and a declared namespace scope
- WHEN the entries share an identifier inside that same scope
- THEN the scoped-uniqueness primitive returns a duplicate decision for that scope without claiming global uniqueness

### Requirement: Alias Map Envelope [r[domain-primitives.identifier-namespace.alias-map]]
The system MUST provide finite alias-map envelopes that map admitted aliases to
admitted canonical identifiers and expose explicit missing-target or cycle
metadata.

#### Scenario: Alias target must be admitted [r[domain-primitives.identifier-namespace.alias-map.scenario.alias-target-must-be-admitted]]

- GIVEN finite alias metadata and admitted canonical identifiers
- WHEN an alias points to a target outside the admitted identifier set
- THEN the alias-map primitive rejects the alias or records explicit missing-target metadata

### Requirement: Canonicalization Decision Envelope [r[domain-primitives.identifier-namespace.canonicalization]]
The system MUST provide canonicalization decision envelopes that record accepted,
rejected, unchanged, or mapped identifier forms without claiming semantic
equivalence beyond the declared finite map.

#### Scenario: Canonicalization decision preserves original identifier [r[domain-primitives.identifier-namespace.canonicalization.scenario.canonicalization-decision-preserves-original-identifier]]

- GIVEN a finite identifier and canonicalization metadata
- WHEN canonicalization maps or rejects the identifier
- THEN the canonicalization primitive records the original identifier and explicit decision metadata

### Requirement: Collision Metadata Envelope [r[domain-primitives.identifier-namespace.collision]]
The system MUST provide deterministic collision metadata for finite registry
snapshots, candidate identifiers, and declared namespace scopes without claiming
hash collision resistance or randomness quality.

#### Scenario: Collision metadata is deterministic [r[domain-primitives.identifier-namespace.collision.scenario.collision-metadata-is-deterministic]]

- GIVEN the same finite registry snapshot, candidate identifier, and namespace scope
- WHEN collision metadata is computed by the primitive
- THEN the primitive returns the same collision or no-collision decision metadata

### Requirement: Lookup Result Envelope [r[domain-primitives.identifier-namespace.lookup]]
The system MUST provide deterministic lookup result envelopes over finite
namespace scopes, admitted identifiers, aliases, and registry snapshots.

#### Scenario: Lookup distinguishes absent and rejected identifiers [r[domain-primitives.identifier-namespace.lookup.scenario.lookup-distinguishes-absent-and-rejected-identifiers]]

- GIVEN a finite registry snapshot and candidate identifier
- WHEN the identifier is malformed, outside the namespace, or absent from the registry
- THEN the lookup primitive returns distinct explicit decision metadata for the corresponding case

### Requirement: Registration Admission Envelope [r[domain-primitives.identifier-namespace.registration]]
The system MUST provide deterministic registration admission/rejection envelopes
that compose identifier-shape, namespace, uniqueness, alias, canonicalization,
collision, and lookup decisions over finite registry snapshots.

#### Scenario: Registration requires all finite namespace checks [r[domain-primitives.identifier-namespace.registration.scenario.registration-requires-all-finite-namespace-checks]]

- GIVEN finite identifier, namespace, alias, canonicalization, and registry metadata
- WHEN any identifier-shape, namespace, uniqueness, alias, collision, or lookup check rejects
- THEN the registration primitive returns an explicit rejection decision before any distributed-identity or security claim is made
