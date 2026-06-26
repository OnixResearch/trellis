# TLB Shootdown Specification

## Purpose

Defines requirements for a verified TLB shootdown protocol that removes
stale cached translations from targeted cores before the protocol reports
completion. The model focuses on stale-entry invalidation, not full page
walk semantics.

## Assumptions

- A single-page request targets one virtual page at a time.
- A core is **responsive** when it handles the request at most once and,
  if targeted, eventually acknowledges after invalidating its stale entry.
- The model proves absence of stale cached translations. Subsequent page
  walks and installation of the new mapping are outside this module.
- Range shootdown is a lifting layer over the single-page core.

## Requirements

### Requirement: Post-Shootdown Freshness [r[tlb-shootdown.freshness]]

After a single-page shootdown completes, every targeted core MUST have the
stale cached translation removed for the target page.

#### Scenario: All targeted cores invalidated

- GIVEN a single-page shootdown for page `V`
- AND every targeted core has acknowledged the request
- WHEN any targeted core's cached translation for `V` is inspected
- THEN the cached translation for `V` is absent

#### Scenario: Completion implies no stale cache hit

- GIVEN a completed single-page shootdown for page `V`
- WHEN a targeted core later accesses `V`
- THEN any use of the old cached translation is impossible because the
  stale entry is absent

### Requirement: Protocol Termination Under Responsiveness [r[tlb-shootdown.termination]]

Under the responsiveness assumption, acknowledgments MUST make monotone
progress toward completion and completion MUST hold exactly when all
required acknowledgments have been recorded.

#### Scenario: Pending count decreases on first acknowledgment

- GIVEN a shootdown with a targeted core `C` not yet acknowledged
- WHEN `C` acknowledges
- THEN the pending-acknowledgment count decreases by one

#### Scenario: Pending count never increases

- GIVEN a shootdown in progress
- WHEN any core handles an acknowledgment step
- THEN the pending-acknowledgment count does not increase

#### Scenario: Completion matches all acknowledgments

- GIVEN a shootdown request
- WHEN completion is queried
- THEN completion is true if and only if all required acknowledgments are present

### Requirement: Initiator Completion Guard [r[tlb-shootdown.initiator-safety]]

The module MUST expose completion only after every required acknowledgment
has been recorded. Callers MAY use completion as the guard for proceeding
with effects outside this module.

#### Scenario: Incomplete request cannot proceed

- GIVEN a shootdown request with at least one pending acknowledgment
- WHEN completion is queried
- THEN the request is not complete

#### Scenario: Completed request may proceed

- GIVEN a shootdown request with all required acknowledgments recorded
- WHEN completion is queried
- THEN the request is complete

### Requirement: Self-Invalidation [r[tlb-shootdown.self-invalidation]]

The initiating core MUST invalidate its own cached translation for the
single target page before waiting on other cores.

#### Scenario: Initiator invalidated at start

- GIVEN initiator core `C` starts a single-page shootdown for page `V`
- WHEN initiation returns the request and updated page-local TLB state
- THEN core `C`'s cached translation for `V` is absent
- AND `C` is already marked acknowledged in the request state

### Requirement: Disjoint-Page Isolation [r[tlb-shootdown.isolation]]

Handling a shootdown for one target page MUST NOT modify cached
translations for any different page.

#### Scenario: Other page preserved

- GIVEN a page-indexed TLB matrix and a shootdown targeting page `V1`
- WHEN one protocol step updates page `V1`
- THEN every page `V2 != V1` remains unchanged

#### Scenario: Disjoint requests are structurally independent

- GIVEN two single-page shootdowns targeting distinct pages `V1` and `V2`
- WHEN one request performs an invalidation step
- THEN the other target page's cached translations are unchanged

### Requirement: Range Invalidation Coverage [r[tlb-shootdown.range]]

Range invalidation MUST apply the single-page invalidation rule to every
page in `[start, end)` and preserve pages outside the range.

#### Scenario: Every page in range invalidated

- GIVEN a page-indexed TLB matrix and range `[start, end)`
- WHEN the range invalidation helper is applied for a core or for all cores
- THEN every page `V` with `start <= V < end` is updated exactly as the
  single-page invalidation rule specifies

#### Scenario: Pages outside range preserved

- GIVEN a page-indexed TLB matrix and range `[start, end)`
- WHEN range invalidation is applied
- THEN every page `V` outside `[start, end)` is unchanged

#### Scenario: Empty and singleton ranges

- GIVEN a range with `start == end`
- WHEN range invalidation is applied
- THEN the result is unchanged

- GIVEN a range with `end == start + 1`
- WHEN range invalidation is applied
- THEN the result is equivalent to the single-page invalidation rule for `start`
