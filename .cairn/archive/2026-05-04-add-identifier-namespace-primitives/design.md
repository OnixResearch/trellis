## Context

The domain-primitives umbrella now tracks finite validation, configuration,
policy, and storage/index families. Identifier/namespace handling is a reusable
cross-cutting primitive family that should be separated from access-control,
configuration, storage indexes, and text/string parsing so future proofs can
reason about ID admission and scoped lookup without inheriting distributed
identity or security claims.

## Goals / Non-Goals

**Goals:**

- specify exact finite identifier and namespace-scope envelopes
- specify scoped uniqueness and alias-map predicates over finite registries
- specify canonicalization and collision decisions as explicit metadata
- specify deterministic lookup and registration admission/rejection with exact finite implementation

**Non-Goals:**

- proving global uniqueness, distributed registry behavior, authentication, or
  cryptographic ownership
- proving external standard conformance for UUID, URI, DNS, filesystem paths, or
  database uniqueness
- claiming hash/randomness quality, security, privacy, performance, or lifecycle
  policy completeness

## Decisions

### 1. Model IDs and namespaces as finite envelopes

**Choice:** Identifier primitives use finite ID-shape metadata and namespace
scope envelopes rather than external naming services or unbounded global
registries.

**Rationale:** Finite envelopes are Verus-friendly and compose with storage,
configuration, policy, and text/string primitives.

**Alternative:** Start with UUID/URI/DNS conformance. Rejected because external
syntax and global semantics are too broad for this reusable primitive slice.

### 2. Keep uniqueness scoped and explicit

**Choice:** Uniqueness predicates are only over a declared finite namespace scope
or registry snapshot and produce explicit duplicate/collision decisions.

**Rationale:** This is useful for local reasoning without claiming distributed
identity, consensus, or durable registry enforcement.

### 3. Treat canonicalization and aliases as decisions

**Choice:** Alias and canonicalization primitives record accepted, rejected,
unchanged, or mapped decisions with target metadata.

**Rationale:** This captures finite bookkeeping while avoiding broad claims about
semantic equivalence, security, or lifecycle policy completeness.

## Risks / Trade-offs

**Overlap with text/string and filesystem paths** → This child owns finite ID and
namespace semantics. Text/string owns byte/string spans and normalization shape;
filesystem/URI/path-specific behavior must live in later focused changes.

**Security drift** → Requirements explicitly avoid authentication,
authorization, cryptographic ownership, and anti-spoofing claims.

**Global uniqueness overclaim** → The model is scoped to finite registries and
must not claim distributed or universal uniqueness.

## Validation Plan

- `openspec validate add-identifier-namespace-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about future implementation tasks
- implementation adds `docs/requirements.md`, `r[impl]`,
  `r[depends]`, `r[verify]`, tests, Verus proof evidence, and Tracey evidence
