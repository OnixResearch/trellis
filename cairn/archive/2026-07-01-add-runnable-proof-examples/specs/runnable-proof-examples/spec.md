## ADDED Requirements

### Requirement: Runnable Proof Examples

The repository MUST provide runnable proof examples that demonstrate Trellis's `spec fn` / `exec fn` / `ensures` style with command-backed positive and negative evidence.

r[runnable-proof-examples.basic]

#### Scenario: Positive example is command-backed

- GIVEN a runnable proof example is documented
- WHEN a user follows the documented positive command
- THEN the command verifies the example's stated spec/exec/proof relationship without introducing trusted bodies, `assume(...)` soundness oracles, or untracked proof-gap markers.

#### Scenario: Negative example is isolated and expected

- GIVEN an intentional failing proof example or fixture is documented
- WHEN a user follows the documented negative command
- THEN the command reports the expected failure shape and the repository treats that failure as a successful negative check rather than part of the normal passing verification path.

#### Scenario: Example has requirement coverage

- GIVEN a runnable proof example is added
- WHEN Tracey coverage runs
- THEN every introduced requirement ID has `r[impl]` and `r[verify]` coverage or an explicit bounded exemption.

### Requirement: Example Command Inventory

The repository SHOULD document exact commands for positive examples, negative examples, and any opt-in example checks that are not part of the default fast path.

r[runnable-proof-examples.commands]

#### Scenario: Commands are reproducible

- GIVEN a user opens the runnable proof examples documentation
- WHEN they inspect an example entry
- THEN the entry names the source file, the positive verification command, any runtime test command, and the negative expected-failure command when a negative fixture exists.

#### Scenario: Opt-in checks are labeled

- GIVEN an example command is intentionally excluded from the default verification path because it is slow or expected to fail
- WHEN the documentation renders the command
- THEN it labels the command as opt-in and explains the expected outcome.
