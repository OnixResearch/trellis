## Context

The domain-primitives umbrella now tracks many reusable finite primitive
families. Data validation is a cross-cutting family that should be separated
from configuration/feature flags, parsing/format handling, access-control
policy, and business logic so future proofs can reason about validation shape
without inheriting external-validator or policy-engine claims.

## Goals / Non-Goals

**Goals:**

- specify exact finite schema-predicate and field-presence primitives
- specify enum/range and cross-field predicate envelopes over admitted finite
  inputs
- specify deterministic error accumulation and normalization decisions
- implement validation admission/rejection over exact finite metadata

**Non-Goals:**

- proving external schema-language conformance or validator-library correctness
- proving business-rule completeness, security, privacy, or UI behavior
- claiming automatic repair or lossy coercion correctness

## Decisions

### 1. Model schemas as finite predicate envelopes

**Choice:** A data-validation schema is represented by finite predicate metadata
that names admitted fields, value envelopes, and predicate IDs rather than by an
arbitrary external schema language.

**Rationale:** Finite predicate envelopes are Verus-friendly and compose with
existing typed-value, parsing/format, and text/string families.

**Alternative:** Model a full external schema language first. Rejected because
language conformance and validator semantics are too broad for this reusable
primitive slice.

### 2. Keep invalid inputs explicit and non-mutating

**Choice:** Required/optional field checks, enum/range constraints, and
cross-field predicates return explicit pass/fail decisions with reason metadata
and do not mutate candidate records.

**Rationale:** Explicit decisions make rejection behavior testable and avoid
hiding validation failures behind runtime exceptions or external stores.

### 3. Treat normalization as a decision, not a correctness claim

**Choice:** Normalization primitives record whether an admitted deterministic
normalization was applied, rejected, or left unchanged, and expose input/output
shape metadata.

**Rationale:** This captures useful finite bookkeeping while avoiding broad
claims about lossy coercion, locale-aware normalization, or semantic repair.

## Risks / Trade-offs

**Overlap with parsing/format and configuration** → This child owns validation
predicates and decisions; parsing/format owns token/AST/codec shape, and
configuration/feature-flag owns config-key/default/rollout semantics.

**Business-rule drift** → Requirements explicitly avoid business-rule
completeness. Domain-specific policy rules must live in later focused changes.

**Over-broad schema language scope** → The first implementation tasks must use
finite predicate envelopes and leave external schema conformance out of scope.

## Validation Plan

- `openspec validate add-data-validation-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested
  checks, strict OpenSpec validation, and final repo validation evidence
