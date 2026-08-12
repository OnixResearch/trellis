## Phase 1: Calendar Fundamentals

- [x] `calendar` module: `is_leap_year` spec/exec/proof — Gregorian rule with century exceptions
- [x] `calendar` module: `days_in_month` spec/exec/proof — 12-month lookup with leap-year February
- [x] `calendar` module: `is_valid_date` spec/exec/proof — month in [1,12], day in [1, days_in_month]
- [x] `calendar` module: `day_of_week` spec/exec/proof — Sakamoto's algorithm, result in [0,6]
- [x] `calendar` module: proptest — leap year known values, days-in-month bounds, round-trip validation
- [x] Tracey annotations for all calendar requirements

## Phase 2: Cron Field Matching

- [x] `cron_field` module: `match_field` spec/exec/proof — range check + step divisibility
- [x] `cron_field` module: `is_valid_field` spec/exec/proof — bounds and step validation
- [x] `cron_field` module: wildcard-matches-all proof — step=1 full range matches every value
- [x] `cron_field` module: proptest — field matching against known cron patterns
- [x] Tracey annotations for all cron field requirements

## Phase 3: Schedule Matching

- [x] `cron_match` module: `matches_schedule` spec/exec/proof — five-field AND composition
- [x] `cron_match` module: `is_valid_schedule` spec/exec/proof — all fields within cron bounds
- [x] `cron_match` module: day-of-week consistency proof — weekday derived from date, not caller
- [x] `cron_match` module: proptest — schedule matching against known cron expressions
- [x] Tracey annotations for all schedule matching requirements

## Phase 4: Next Occurrence

- [x] `cron_next` module: `next_occurrence` spec/exec/proof — forward search with field skipping
- [x] `cron_next` module: result-after-input proof — returned time strictly after input
- [x] `cron_next` module: result-matches-schedule proof — returned time satisfies matches_schedule
- [x] `cron_next` module: bounded search proof — terminates within max_years
- [x] `cron_next` module: leap year awareness proof — uses days_in_month for month rollover
- [x] `cron_next` module: proptest — next occurrence for known schedules, unsatisfiable schedules
- [x] Tracey annotations for all next occurrence requirements
- [x] Wire all four modules into src/lib.rs
