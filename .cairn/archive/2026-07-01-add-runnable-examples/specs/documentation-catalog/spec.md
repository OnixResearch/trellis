## ADDED Requirements

### Requirement: Runnable Example Coverage

The repository MUST provide checked runnable examples that demonstrate representative stable public APIs without requiring users to invoke Verus directly.

r[documentation-catalog.runnable-examples]

#### Scenario: Example compiles through normal cargo

- GIVEN a documented example under `examples/`
- WHEN the example validation command runs
- THEN the example compiles with normal cargo and does not require ghost/proof execution at runtime

#### Scenario: Admission example shows rejection

- GIVEN a public API whose useful behavior includes rejecting invalid input
- WHEN a representative example covers that API
- THEN the example or its companion test demonstrates both an accepted input and a rejected input

#### Scenario: README links remain accurate

- GIVEN README or guide text references a runnable example
- WHEN documentation catalog validation runs
- THEN every referenced example path exists and is part of the example validation set
