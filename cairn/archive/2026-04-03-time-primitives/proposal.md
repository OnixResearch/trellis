## Why

The library has `deadline_queue` (EDF ordering), `job_timeout` (deadline
computation), `token_timestamp` (expiration with tolerance), and
`interval` (half-open range containment). None of these model calendar
time — leap years, variable-length months, day-of-week, or recurring
schedules.

Any cron implementation needs these building blocks: "is February 29
valid this year?", "what day of the week is March 15?", "does minute 35
match the pattern `*/10`?", "when does `0 3 * * 1` fire next?". These
are pure arithmetic functions with tricky edge cases (century leap year
rules, month-length variation, weekday wraparound) that benefit from
machine-checked proofs.

The existing modules handle time-as-a-scalar (deadlines, durations,
expiration). This change adds time-as-a-calendar — the decomposed
year/month/day/hour/minute representation and the decision logic for
recurring schedule matching.

## What Changes

Four new modules covering calendar and cron schedule decision logic.

- **`calendar`** — Leap year detection, days-in-month lookup,
  date validation, and day-of-week computation (Zeller's congruence or
  Tomohiko Sakamoto's method). All operate on decomposed
  (year, month, day) triples as u64 values. Proofs that days-in-month
  is always in [28, 31], February has 29 days iff leap year, and
  day-of-week returns values in [0, 6].

- **`cron_field`** — Cron field matching. A cron field is modeled as a
  (min, max, step) triple: `*/15` in the minute column becomes
  (0, 59, 15). `match_field(value, min, max, step)` returns true when
  the value falls on a step boundary within [min, max]. Wildcard is
  (min_of_field, max_of_field, 1). Proofs that matching is
  deterministic and that the field bounds are respected.

- **`cron_match`** — Full cron schedule matching. A schedule is five
  fields (minute, hour, day-of-month, month, day-of-week). Given a
  decomposed timestamp, `matches_schedule` returns true when all five
  fields match simultaneously. Proofs that a valid schedule always
  has at least one matching minute within any matching hour, and that
  day-of-week and day-of-month constraints compose correctly.

- **`cron_next`** — Next occurrence computation. Given a schedule and
  a current decomposed timestamp, find the earliest future timestamp
  that matches. Bounded search with a configurable year horizon (avoids
  infinite loops for unsatisfiable schedules like Feb 30). Proofs that
  the result is strictly after the input and that it actually matches
  the schedule.

## Capabilities

### New Capabilities
- `calendar-leap-year`: Verified leap year predicate with century rules
- `calendar-days-in-month`: Verified days-in-month with leap year awareness
- `calendar-day-of-week`: Verified day-of-week from (year, month, day)
- `calendar-date-validation`: Verified date validity check
- `cron-field-match`: Verified cron field step-matching
- `cron-schedule-match`: Verified five-field cron schedule matching
- `cron-next-occurrence`: Verified next fire time with bounded search

### Modified Capabilities
- None — these are standalone modules with no changes to existing code

## Impact

- **Files**: 4 new src/ modules, src/lib.rs additions, tracey annotations
- **APIs**: ~20 new pub fns across 4 modules
- **Dependencies**: None — pure arithmetic on u64 using vstd
- **Testing**: Verus verification + proptest property tests
