## ADDED Requirements

### Requirement: Spreadsheet Address Envelope

r[domain-primitives.spreadsheet.address]
The system MUST provide finite spreadsheet sheet-bound and cell-address envelopes that record row bounds, column bounds, cell coordinates, and deterministic address-admission results. A checked cell address MUST be admitted only when both coordinates are inside the declared finite sheet bounds, and rejected address admission MUST leave the prior admitted sheet metadata unchanged.

#### Scenario: Cell address must fit sheet bounds

- GIVEN finite spreadsheet sheet metadata and a candidate cell coordinate
- WHEN the coordinate row or column is outside the declared sheet bounds
- THEN the address primitive returns explicit out-of-bounds rejection metadata without constructing an admitted cell reference

#### Scenario: In-bounds cell address is admitted deterministically

- GIVEN finite spreadsheet sheet metadata and a candidate cell coordinate inside the declared row and column bounds
- WHEN address admission is evaluated repeatedly
- THEN the same admitted cell-address metadata is returned each time

### Requirement: Spreadsheet Range Envelope

r[domain-primitives.spreadsheet.range]
The system MUST provide finite rectangular-range envelopes that normalize two admitted endpoint addresses into a canonical rectangle, expose deterministic range membership, and expand only to admitted cell coordinates inside the declared sheet bounds.

#### Scenario: Reversed range endpoints normalize to the same rectangle

- GIVEN two admitted cell addresses in the same finite sheet
- WHEN a rectangular range is admitted using the endpoints in either order
- THEN both admissions produce the same canonical rectangle membership set

#### Scenario: Range rejects invalid endpoints

- GIVEN finite spreadsheet sheet metadata and a range candidate with an out-of-bounds endpoint
- WHEN range admission is evaluated
- THEN the range primitive rejects the range before any membership or expansion result is exposed

### Requirement: Spreadsheet Formula Envelope

r[domain-primitives.spreadsheet.formula]
The system MUST provide a finite spreadsheet formula and value model covering blank values, exact numeric values, booleans, text, explicit errors, constants, cell references, rectangular ranges, arithmetic operations, and basic finite aggregates over admitted ranges. Formula admission MUST reject syntactically or structurally invalid formulas before dependency extraction or evaluation.

#### Scenario: Formula references admitted cells and ranges only

- GIVEN finite sheet metadata and a formula containing cell and range references
- WHEN formula admission is evaluated
- THEN admission succeeds only if every referenced address and range is admitted for the sheet

#### Scenario: Invalid formula shape is rejected before evaluation

- GIVEN a candidate formula with malformed structure or an unsupported operation envelope
- WHEN formula admission is evaluated
- THEN the formula primitive returns explicit rejection metadata without attempting dependency extraction or evaluation

### Requirement: Spreadsheet Dependency Extraction

r[domain-primitives.spreadsheet.dependency]
The system MUST provide deterministic dependency-extraction predicates that return exactly the finite set of precedent cell coordinates named by an admitted formula, including all cells contained in admitted referenced ranges. Dependency extraction MUST be pure over formula structure and declared sheet bounds, and MUST NOT depend on current formatted values, cached results, external workbook files, clocks, or volatile runtime state.

#### Scenario: Range dependency expands to contained cells

- GIVEN an admitted formula that references an admitted rectangular range
- WHEN dependency extraction runs
- THEN every cell coordinate in the canonical range is included in the precedent set and no out-of-range coordinate is included

#### Scenario: Constants have no cell precedents

- GIVEN an admitted constant-only formula
- WHEN dependency extraction runs
- THEN the precedent set is empty

### Requirement: Spreadsheet Cycle Rejection

r[domain-primitives.spreadsheet.cycle]
The system MUST reject spreadsheet formula sets whose dependency graph contains a cycle before formula evaluation begins. An admitted formula set MUST have an acyclic dependency graph over the finite formula-cell coordinates and their extracted precedents.

#### Scenario: Direct circular reference is rejected

- GIVEN a formula cell whose admitted formula directly references the same cell
- WHEN spreadsheet formula-set admission is evaluated
- THEN admission returns explicit cycle rejection metadata and no evaluation order is exposed

#### Scenario: Indirect circular reference is rejected

- GIVEN admitted formula cells whose extracted dependencies form an indirect cycle
- WHEN spreadsheet formula-set admission is evaluated
- THEN admission returns explicit cycle rejection metadata before any formula value is evaluated

### Requirement: Spreadsheet Recalculation Order

r[domain-primitives.spreadsheet.recalc]
The system MUST provide recalculation-order predicates that admit only finite orders where each selected formula cell appears after every selected formula precedent required by its extracted dependency set. Successful recalculation-order admission MUST refine the acyclic dependency graph and MUST NOT introduce formula cells outside the declared finite sheet metadata.

#### Scenario: Topological recalculation order is admitted

- GIVEN an acyclic admitted spreadsheet dependency graph and a candidate order containing selected formula cells
- WHEN every formula cell appears after its selected precedents
- THEN recalculation-order admission succeeds

#### Scenario: Precedent-after-dependent order is rejected

- GIVEN an admitted spreadsheet dependency graph and a candidate order that places a formula cell before one of its selected formula precedents
- WHEN recalculation-order admission is evaluated
- THEN the order is rejected and no evaluation result is exposed for that order

### Requirement: Spreadsheet Dirty Propagation

r[domain-primitives.spreadsheet.dirty-propagation]
The system MUST provide dirty-cell propagation predicates that include each changed cell and every formula cell reachable from the changed cells through reverse spreadsheet dependencies, while excluding formula cells that are not reachable dependents under the admitted finite dependency graph.

#### Scenario: Dependent formulas become dirty

- GIVEN an admitted acyclic spreadsheet dependency graph and a changed input cell
- WHEN dirty propagation runs
- THEN every formula cell that transitively depends on the changed cell is included in the dirty set

#### Scenario: Unrelated formulas stay clean

- GIVEN an admitted acyclic spreadsheet dependency graph, a changed cell, and a formula cell that is not reachable through reverse dependencies from that changed cell
- WHEN dirty propagation runs
- THEN the unrelated formula cell is not included in the dirty set

### Requirement: Spreadsheet Error Envelope

r[domain-primitives.spreadsheet.error]
The system MUST provide explicit deterministic spreadsheet error envelopes for bad references, division by zero, value-shape mismatches, and cycle rejection. Error-producing checks MUST return the same error metadata for the same admitted inputs, MUST NOT panic, and MUST NOT mutate admitted sheet, formula, dependency, or recalculation metadata.

#### Scenario: Division by zero returns explicit error

- GIVEN an admitted arithmetic formula whose denominator evaluates to exact zero
- WHEN formula evaluation is checked
- THEN the result is the explicit division-by-zero error envelope

#### Scenario: Bad reference returns explicit error

- GIVEN a formula reference that no longer resolves to an admitted cell under the declared sheet metadata
- WHEN formula admission or evaluation is checked
- THEN the result is the explicit bad-reference rejection or error envelope

### Requirement: Spreadsheet Admission Composition

r[domain-primitives.spreadsheet.admission]
The system MUST provide a deterministic spreadsheet-admission composition that requires address admission, range admission, formula admission, dependency extraction, cycle rejection, recalculation-order admission, dirty-propagation admission, and error-envelope determinism before any spreadsheet evaluation claim is exposed.

#### Scenario: Admission requires every spreadsheet check

- GIVEN finite spreadsheet metadata and candidate formula/recalculation inputs
- WHEN any address, range, formula, dependency, cycle, recalculation-order, dirty-propagation, or error-envelope check rejects
- THEN the spreadsheet-admission primitive returns an explicit rejection before claiming any evaluated spreadsheet value
