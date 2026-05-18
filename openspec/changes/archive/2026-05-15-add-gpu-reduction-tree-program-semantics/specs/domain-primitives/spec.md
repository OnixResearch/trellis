## ADDED Requirements

### Requirement: GPU Reduction Tree Program Semantics

The system MUST define finite GPU reduction-tree program semantics where admitted reduction stages have positive input cardinality, bounded active lanes, monotonically non-increasing output cardinality, and a final stage that produces the declared finite number of reduction outputs without claiming operator algebra or numerical accuracy.

r[domain-primitives.gpu.reduction-tree-program]

#### Scenario: Stage reduces active cardinality

- GIVEN an admitted reduction stage with positive input count and bounded active lanes
- WHEN stage admission is evaluated
- THEN the stage output count is positive and no larger than the input count

#### Scenario: Tree reaches declared outputs

- GIVEN a finite sequence of admitted reduction stages
- WHEN tree admission is evaluated against a declared output count
- THEN the final stage cardinality equals the declared output count

#### Scenario: Invalid expanding stage is rejected

- GIVEN a reduction stage whose output count exceeds its input count
- WHEN stage admission is evaluated
- THEN the stage is rejected
