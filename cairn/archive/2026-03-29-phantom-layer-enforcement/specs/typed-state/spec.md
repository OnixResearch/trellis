# Typed State Specification

## Purpose

Provides compile-time enforcement of verification pipelines via phantom
type parameters. Data wrapped as `Stamped<Verified, T>` can only be
produced by passing through a verification predicate. This guarantee
holds even in downstream crates that don't use Verus.

## Requirements

### Requirement: Stamp Construction

The system MUST provide `stamp(value: T) -> Stamped<Unverified, T>` that
wraps raw data in the unverified state. The inner value MUST be preserved.

#### Scenario: Value preservation

- GIVEN any value `v`
- WHEN `stamp(v)` is called
- THEN `result.inner() == v`

### Requirement: Verified Transition

The system MUST provide `verify_with(stamped, predicate)` that transitions
from `Stamped<Unverified, T>` to `Stamped<Verified, T>` if and only if
the predicate returns true.

#### Scenario: Predicate passes

- GIVEN a `Stamped<Unverified, T>` value where predicate(inner) is true
- WHEN `verify_with` is called
- THEN `Some(Stamped<Verified, T>)` is returned

#### Scenario: Predicate fails

- GIVEN a `Stamped<Unverified, T>` value where predicate(inner) is false
- WHEN `verify_with` is called
- THEN `None` is returned

### Requirement: Unwrap Verified

The system MUST provide `unwrap_verified(stamped: Stamped<Verified, T>) -> T`
that extracts the inner value. This function MUST only accept verified data.

#### Scenario: Roundtrip

- GIVEN a value `v` where predicate(v) is true
- WHEN `stamp(v)` then `verify_with(predicate)` then `unwrap_verified` is called
- THEN the result equals `v`

### Requirement: Predicate Preservation

If `verify_with` returns `Some`, the predicate MUST have returned true
for the inner value at the time of verification.

#### Scenario: Implication

- GIVEN `verify_with(stamped, pred)` returns `Some(verified)`
- THEN `pred(verified.inner())` was true at verification time

### Requirement: Inner Access

The system MUST provide `inner(stamped) -> &T` that gives read access
to the wrapped value in any state. Inspection before verification
MUST be possible.

#### Scenario: Unverified access

- GIVEN a `Stamped<Unverified, T>` value
- WHEN `inner()` is called
- THEN the wrapped value is returned

#### Scenario: Verified access

- GIVEN a `Stamped<Verified, T>` value
- WHEN `inner()` is called
- THEN the wrapped value is returned

### Requirement: No Forge

It MUST be impossible to construct `Stamped<Verified, T>` without calling
`verify_with`. This is enforced structurally by private fields — no
runtime check, no Verus proof needed, just compilation prevention.

#### Scenario: Direct construction rejected

- GIVEN an attempt to construct `Stamped<Verified, T>` directly
  (e.g., `Stamped { value: v, _state: Verified }`)
- THEN compilation fails with a field visibility error
