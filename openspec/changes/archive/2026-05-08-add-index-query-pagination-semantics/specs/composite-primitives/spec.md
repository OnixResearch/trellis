## ADDED Requirements

### Requirement: Index Query Pagination Semantics

The system MUST provide bounded index/query pagination semantics where each accepted result page is scan-equivalent for its window, cursor progress is monotone, and repeated pages cannot introduce duplicates outside the finite model.

r[composite-primitives.index-query-pagination]

#### Scenario: First page matches bounded scan

- GIVEN an admitted query model and a page limit
- WHEN the first page is accepted from an index-backed result
- THEN every row in the page is accepted by the corresponding finite scan predicate

#### Scenario: Continuation cursor advances

- GIVEN an accepted page with a continuation cursor
- WHEN the next page is evaluated from that cursor
- THEN the cursor position advances monotonically and previously emitted rows are not re-emitted

#### Scenario: Invalid cursor is rejected

- GIVEN a cursor outside the finite result envelope or inconsistent with the query model
- WHEN pagination admission is evaluated
- THEN the page is rejected
