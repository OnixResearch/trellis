## ADDED Requirements

### Requirement: Composite Receipt Envelope Semantics

The system MUST provide bounded composite receipt and evidence-envelope semantics that admit a receipt only when its serialized envelope, artifact references, validation window, and optional query/table evidence are all finite and internally consistent.

r[composite-primitives.receipt-envelope]

#### Scenario: Well-formed receipt is admitted

- GIVEN a receipt with admitted schema version, bounded serialized sections, declared artifacts, and consistent validation-window metadata
- WHEN receipt-envelope admission is evaluated
- THEN the receipt is accepted and every referenced artifact lies within the declared finite envelope

#### Scenario: Dangling artifact reference is rejected

- GIVEN a receipt whose evidence row references an undeclared artifact or out-of-window result
- WHEN receipt-envelope admission is evaluated
- THEN the receipt is rejected without accepting partial evidence

#### Scenario: Query evidence composes with receipt envelope

- GIVEN a graph-query or table result witness embedded in a receipt envelope
- WHEN receipt admission succeeds
- THEN the accepted receipt implies the underlying finite query/table witness predicates
