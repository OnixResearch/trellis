## ADDED Requirements

### Requirement: Immutable Definition Binding Refactor

The system MUST define finite refactoring-session metadata where moving a name from an old definition hash to a new definition hash never mutates the old definition identity and always records both old and new bindings explicitly.

r[domain-primitives.semantic-refactoring.immutable-binding]

#### Scenario: Rename does not mutate old definition

- GIVEN a name binding initially pointing to old definition hash `H1`
- WHEN the name is moved to new definition hash `H2`
- THEN the old definition `H1` remains present and unchanged in the finite codebase model

#### Scenario: Unknown replacement is rejected

- GIVEN a refactoring step moving a name to a definition hash absent from the finite codebase model
- WHEN refactoring admission is evaluated
- THEN the step is rejected with missing-new-definition metadata

### Requirement: Compatible Replacement Propagation

The system MUST define compatible replacement propagation where dependents of an old definition may be moved to a new definition only when declared type/interface compatibility metadata admits the replacement, and propagation preserves dependent validity in the finite model.

r[domain-primitives.semantic-refactoring.compatible-propagation]

#### Scenario: Compatible replacement propagates to dependent

- GIVEN a dependent definition referencing old hash `H1` and a compatible replacement hash `H2`
- WHEN propagation admission is evaluated
- THEN the dependent may be admitted with the reference updated to `H2`

#### Scenario: Incompatible replacement cannot propagate

- GIVEN a dependent definition referencing old hash `H1` and a replacement hash `H2` whose type/interface metadata is incompatible
- WHEN propagation admission is evaluated
- THEN the dependent is not rewritten and is recorded as unresolved

### Requirement: Refactoring Session Validity

The system MUST define finite refactoring session validity where every step either preserves admitted bindings/dependents or records unresolved dependents explicitly, so the codebase model remains queryable and no name points to an absent definition.

r[domain-primitives.semantic-refactoring.session-validity]

#### Scenario: Session with unresolved dependent remains explicit

- GIVEN an incompatible replacement with at least one dependent that cannot be propagated
- WHEN session validity is evaluated
- THEN the session is admitted only if that dependent appears in the unresolved-dependent set

#### Scenario: Dangling name rejects session

- GIVEN a refactoring session whose resulting name table points to an absent definition hash
- WHEN session validity is evaluated
- THEN the session is rejected with dangling-name metadata

#### Scenario: Fully propagated session has no unresolved dependents

- GIVEN every dependent of an old definition is compatible with the replacement
- WHEN session validity is evaluated after propagation
- THEN the unresolved-dependent set is empty
