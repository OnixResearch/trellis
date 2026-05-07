# Storage Index Primitives Delta Specification

## ADDED Requirements

### Requirement: Page Range [r[storage.page-range]]
The system MUST define a finite page-range primitive with checked page
identifier bounds, span length metadata, and half-open page interval admission.

#### Scenario: Checked page range admits finite half-open span [r[storage.page-range.scenario.checked-page-range-admits-finite-half-open-span]]

- GIVEN finite page-count metadata, a start page, and an end page with start less than or equal to end
- WHEN a checked page-range constructor evaluates the span
- THEN it admits only page identifiers within the half-open interval and within the finite page count

#### Scenario: Invalid page range is rejected without mutation [r[storage.page-range.scenario.invalid-page-range-rejected-without-mutation]]

- GIVEN a previously valid page-range value
- WHEN a public checked operation receives overflowing, reversed, or out-of-bounds page metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: Record Slot Table [r[storage.record-slot-table]]
The system MUST define a finite record-slot table primitive with slot-count
bounds, occupied/free state metadata, and valid slot-index admission.

#### Scenario: Slot table admits only in-range occupied slots [r[storage.record-slot-table.scenario.slot-table-admits-only-in-range-occupied-slots]]

- GIVEN finite slot-count metadata and occupied/free state for each admitted slot
- WHEN a slot lookup evaluates a candidate slot index
- THEN it admits occupied-slot access only for indices within the finite slot count whose state is occupied

#### Scenario: Free slot remains explicit state [r[storage.record-slot-table.scenario.free-slot-remains-explicit-state]]

- GIVEN a finite slot table with a free slot
- WHEN the slot-state primitive evaluates that slot
- THEN it exposes the free state explicitly rather than treating the slot as an occupied record

### Requirement: Key Range [r[storage.key-range]]
The system MUST define an ordered finite key-range primitive with lower and upper
bounds, half-open membership, and overlap/disjointness metadata.

#### Scenario: Key range membership follows half-open bounds [r[storage.key-range.scenario.key-range-membership-follows-half-open-bounds]]

- GIVEN an admitted ordered key range with lower bound less than or equal to upper bound
- WHEN membership is evaluated for a candidate key
- THEN the key is admitted exactly when it is greater than or equal to the lower bound and strictly less than the upper bound

#### Scenario: Invalid key range is rejected [r[storage.key-range.scenario.invalid-key-range-rejected]]

- GIVEN unordered or non-representable key bounds
- WHEN a checked key-range operation evaluates the bounds
- THEN it rejects them without exposing inconsistent membership metadata

### Requirement: Cursor Bounds [r[storage.cursor-bounds]]
The system MUST define a cursor-bounds primitive for finite scan ranges, current
cursor position, end state, and monotone cursor movement.

#### Scenario: Cursor movement stays within admitted scan range [r[storage.cursor-bounds.scenario.cursor-movement-stays-within-admitted-scan-range]]

- GIVEN an admitted finite scan range and cursor position within or at the end of that range
- WHEN the cursor advances under the declared movement policy
- THEN the resulting cursor is either within the same range at a later position or explicitly at the end state

#### Scenario: Cursor rejects movement from invalid position [r[storage.cursor-bounds.scenario.cursor-rejects-movement-from-invalid-position]]

- GIVEN a cursor position outside the admitted scan range and not equal to the declared end state
- WHEN a checked cursor operation evaluates the position
- THEN it rejects the operation without constructing an invalid next cursor

### Requirement: Segment Map [r[storage.segment-map]]
The system MUST define a finite segment-map primitive from logical key/page
ranges to physical segment/page spans with explicit coverage and disjointness
metadata.

#### Scenario: Segment map associates logical range with finite physical span [r[storage.segment-map.scenario.segment-map-associates-logical-range-with-finite-physical-span]]

- GIVEN an admitted logical range and finite physical segment/page span
- WHEN the segment-map primitive evaluates the mapping
- THEN it exposes only physical positions within the declared segment/page bounds for that logical range

#### Scenario: Segment map rejects overlapping inconsistent spans [r[storage.segment-map.scenario.segment-map-rejects-overlapping-inconsistent-spans]]

- GIVEN segment-map metadata with overlapping logical ranges that map to inconsistent physical spans under the declared policy
- WHEN a checked segment-map constructor evaluates the metadata
- THEN it rejects the map instead of exposing ambiguous logical-to-physical lookup metadata

### Requirement: Lookup Result Admission [r[storage.lookup-result]]
The system MUST define a lookup-result primitive that admits present, absent, and
out-of-range outcomes according to finite table/range metadata and deterministic
decision rules.

#### Scenario: Present lookup result references admitted record [r[storage.lookup-result.scenario.present-lookup-result-references-admitted-record]]

- GIVEN finite table/range metadata and a lookup result marked present
- WHEN the lookup-result primitive validates the result
- THEN the referenced key, slot, page, or record metadata is within the admitted finite bounds and matches the declared lookup key/range relation

#### Scenario: Absent and out-of-range results are distinguished [r[storage.lookup-result.scenario.absent-and-out-of-range-results-are-distinguished]]

- GIVEN a lookup key and admitted key-range metadata
- WHEN the lookup-result primitive validates an unsuccessful result
- THEN it distinguishes an in-range absent result from an out-of-range rejected result according to the declared bounds

### Requirement: Compaction Envelope [r[storage.compaction-envelope]]
The system MUST define a finite compaction envelope primitive that preserves live
logical records and admitted key/slot membership while allowing physical page or
segment spans to be remapped.

#### Scenario: Compaction preserves live logical record identity [r[storage.compaction-envelope.scenario.compaction-preserves-live-logical-record-identity]]

- GIVEN pre-compaction and post-compaction finite metadata plus a declared live-record set
- WHEN the compaction envelope validates the remap
- THEN every live logical record admitted before compaction is admitted after compaction with the same logical identity and allowed physical span remapping

#### Scenario: Compaction may drop non-live records only by explicit policy [r[storage.compaction-envelope.scenario.compaction-may-drop-non-live-records-only-by-explicit-policy]]

- GIVEN a record not included in the declared live-record set
- WHEN the compaction envelope omits that record from post-compaction metadata
- THEN the omission is valid only when the declared compaction policy explicitly allows dropping non-live records

### Requirement: Versioned Slot [r[storage.versioned-slot]]
The system MUST define a versioned-slot primitive over finite version metadata,
slot state, visibility bounds, and deterministic latest-visible selection.

#### Scenario: Latest-visible selection chooses maximal admitted version [r[storage.versioned-slot.scenario.latest-visible-selection-chooses-maximal-admitted-version]]

- GIVEN a finite set of versions for a slot and an admitted read/version bound
- WHEN the versioned-slot primitive selects a visible entry
- THEN the selected entry is visible under the bound and no other admitted visible entry has a greater version under the declared ordering

#### Scenario: Versioned slot rejects ambiguous latest entries [r[storage.versioned-slot.scenario.versioned-slot-rejects-ambiguous-latest-entries]]

- GIVEN two distinct entries that are both maximal under the declared version ordering without a deterministic tie policy
- WHEN a checked versioned-slot operation evaluates latest-visible selection
- THEN it rejects the metadata instead of exposing an ambiguous latest-visible result
