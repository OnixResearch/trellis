## Context

Spreadsheet logic is a good Trellis domain because the core safety claims are finite and compositional: cells live inside sheet bounds, ranges normalize to finite coordinate sets, formulas reference known cells, dependency graphs must be acyclic before evaluation, and recalculation must respect precedence. The implementation should reuse existing graph/topological-order proof patterns where possible, while keeping the spreadsheet-specific core small enough for direct tests and proofs.

## Decisions

### 1. Model finite sheets, not workbook I/O

**Choice:** Represent a sheet by explicit row and column bounds plus finite cell coordinates. A cell address is admitted only when its row and column are inside those declared bounds.

**Rationale:** Bounds are the foundation for every later formula and recalc claim. Keeping the model finite avoids external file formats, lazy loading, and unbounded grid semantics.

### 2. Normalize rectangular ranges before use

**Choice:** A range is admitted by normalizing two endpoint addresses into a canonical rectangle with inclusive start/end coordinates, then deriving finite membership and expansion predicates from that canonical shape.

**Rationale:** Formula dependency extraction and aggregate evaluation need one stable range meaning independent of how a user selected the endpoints.

### 3. Start with an exact value/formula subset

**Choice:** The first formula model includes blank, exact integer/rational-style numbers, booleans, text, explicit errors, constants, cell references, rectangular ranges, arithmetic, and basic aggregates such as sum/count/min/max over admitted finite ranges.

**Rationale:** This captures the core spreadsheet correctness edges without importing floating-point rounding, locale, date/time serials, formatting, volatile functions, or product-specific compatibility behavior.

### 4. Dependency extraction is pure formula structure

**Choice:** Dependency extraction walks formula syntax and returns finite precedent coordinates/ranges. It does not evaluate formulas, inspect formatted values, or consult external workbook state beyond declared sheet bounds.

**Rationale:** This keeps dependency extraction deterministic and testable as a functional core. Evaluation and recalculation can consume the extracted graph without duplicating reference-walking logic.

### 5. Reject cycles before evaluation

**Choice:** A spreadsheet formula set is admitted for evaluation only when its dependency graph is acyclic. A detected cycle yields an explicit cycle error/admission rejection rather than partial evaluation.

**Rationale:** Circular references make recalc semantics engine-specific unless iterative calculation policy is modeled. Rejecting cycles gives a crisp verified invariant for the first slice.

### 6. Recalculation order is topological and dirty-aware

**Choice:** Recalculation order must evaluate every selected formula cell only after its formula precedents. Dirty propagation marks a changed cell and every formula cell reachable through reverse dependencies, while leaving unrelated formula cells outside the dirty set.

**Rationale:** This directly addresses stale-value bugs. The claim is about order and coverage, not about UI scheduling, incremental caching, or parallel execution.

### 7. Errors are first-class deterministic values

**Choice:** Error results are explicit values/envelopes for bad references, division by zero, value-shape/type mismatch, and cycles. Error propagation is deterministic and does not mutate the admitted sheet shape.

**Rationale:** Spreadsheet bugs often hide in error cases. First-class errors let tests and proofs cover sad paths without panics or unspecified behavior.

## Non-Goals

- Parsing user formula strings or serializing workbook files.
- Product compatibility with Excel, Google Sheets, LibreOffice, or locale/date/time formats.
- Floating-point rounding, precision, or display formatting.
- Volatile functions such as random/time/indirect-style dynamic references.
- Iterative calculation of circular references.
- Copy/fill relative-reference rewriting.
- Dynamic-array spill behavior and spill collision handling.
- UI selection, editing, undo/redo, concurrency, or persistence.

## Risks / Trade-offs

- Exact arithmetic makes the first slice easier to verify but intentionally avoids real-world floating-point compatibility.
- Range expansion can be large in runtime implementations; public helpers should keep explicit finite bounds and avoid hidden unbounded loops.
- Reusing existing graph/topological-order helpers may require small spreadsheet-specific bridges from cell coordinates to graph node identifiers.
