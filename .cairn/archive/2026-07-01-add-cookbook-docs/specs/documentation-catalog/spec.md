## ADDED Requirements

### Requirement: Cookbook Recipes

The repository MUST provide cookbook documentation that describes common user and contributor workflows with clear success criteria, validation commands, and failure-mode guidance.

r[documentation-catalog.cookbook-recipes]

#### Scenario: Recipe states observable success

- GIVEN a cookbook recipe for using or adding Trellis logic
- WHEN a reader follows the recipe
- THEN the recipe states the expected output, validation command, or file change that demonstrates completion

#### Scenario: Recipe includes a negative path

- GIVEN a recipe that introduces tests, requirements, examples, or validation checks
- WHEN the recipe describes its workflow
- THEN it includes at least one representative failure mode or rejection case

#### Scenario: Checked snippet remains current

- GIVEN a cookbook snippet is marked executable or normative
- WHEN cookbook validation runs
- THEN the snippet's referenced files and commands are checked or the validation fails with a diagnostic
