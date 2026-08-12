# Time Primitives Specification

## Purpose

Defines correctness properties for calendar arithmetic and cron schedule
matching: leap year detection, days-in-month, day-of-week computation,
date validation, cron field matching, full schedule matching, and
next-occurrence search.

## Requirements

### Calendar

### Requirement: Leap Year
r[time.leap-year]

`is_leap_year` MUST return true when `(year % 4 == 0 && year % 100 != 0) || year % 400 == 0`,
and false otherwise. This is the Gregorian calendar rule.

#### Scenario: Divisible by 4 but not 100

- GIVEN year = 2024
- WHEN checking is_leap_year
- THEN MUST return true

#### Scenario: Divisible by 100 but not 400

- GIVEN year = 1900
- WHEN checking is_leap_year
- THEN MUST return false

#### Scenario: Divisible by 400

- GIVEN year = 2000
- WHEN checking is_leap_year
- THEN MUST return true

#### Scenario: Not divisible by 4

- GIVEN year = 2023
- WHEN checking is_leap_year
- THEN MUST return false

### Requirement: Days in Month
r[time.days-in-month]

`days_in_month` MUST return the correct number of days for each month
(1-indexed). January=31, February=28 or 29 (leap), March=31, April=30,
May=31, June=30, July=31, August=31, September=30, October=31,
November=30, December=31.

#### Scenario: February in leap year

- GIVEN year = 2024, month = 2
- WHEN computing days_in_month
- THEN MUST return 29

#### Scenario: February in non-leap year

- GIVEN year = 2023, month = 2
- WHEN computing days_in_month
- THEN MUST return 28

#### Scenario: 31-day month

- GIVEN month = 7 (July)
- WHEN computing days_in_month
- THEN MUST return 31

#### Scenario: 30-day month

- GIVEN month = 4 (April)
- WHEN computing days_in_month
- THEN MUST return 30

#### Scenario: Result always in [28, 31]

- GIVEN any valid month (1-12) and any year
- WHEN computing days_in_month
- THEN result MUST be >= 28 and <= 31

### Requirement: Date Validation
r[time.date-validation]

`is_valid_date` MUST return true when month is in [1, 12] and day is in
[1, days_in_month(year, month)]. MUST return false otherwise.

#### Scenario: Valid date

- GIVEN year = 2026, month = 4, day = 4
- WHEN checking is_valid_date
- THEN MUST return true

#### Scenario: Day zero

- GIVEN year = 2026, month = 1, day = 0
- WHEN checking is_valid_date
- THEN MUST return false

#### Scenario: Day exceeds month

- GIVEN year = 2023, month = 2, day = 29
- WHEN checking is_valid_date
- THEN MUST return false

#### Scenario: Feb 29 in leap year

- GIVEN year = 2024, month = 2, day = 29
- WHEN checking is_valid_date
- THEN MUST return true

#### Scenario: Invalid month

- GIVEN year = 2026, month = 13, day = 1
- WHEN checking is_valid_date
- THEN MUST return false

#### Scenario: Month zero

- GIVEN year = 2026, month = 0, day = 1
- WHEN checking is_valid_date
- THEN MUST return false

### Requirement: Day of Week
r[time.day-of-week]

`day_of_week` MUST return the day of week for a valid date as a value
in [0, 6] where 0=Sunday, 1=Monday, ..., 6=Saturday. Requires year >= 1.

#### Scenario: Known Wednesday

- GIVEN year = 2026, month = 4, day = 1 (Wednesday)
- WHEN computing day_of_week
- THEN MUST return 3

#### Scenario: Known Saturday

- GIVEN year = 2026, month = 4, day = 4 (Saturday)
- WHEN computing day_of_week
- THEN MUST return 6

#### Scenario: Leap day

- GIVEN year = 2024, month = 2, day = 29 (Thursday)
- WHEN computing day_of_week
- THEN MUST return 4

#### Scenario: Result bounded

- GIVEN any valid date with year >= 1
- WHEN computing day_of_week
- THEN result MUST be in [0, 6]

### Cron Field Matching

### Requirement: Field Match
r[time.field-match]

`match_field(value, min, max, step)` MUST return true when
`min <= value <= max` and `(value - min) % step == 0`. MUST return
false otherwise. Step MUST be >= 1.

#### Scenario: Wildcard (every value)

- GIVEN value = 35, min = 0, max = 59, step = 1
- WHEN checking match_field
- THEN MUST return true

#### Scenario: Step match

- GIVEN value = 30, min = 0, max = 59, step = 15
- WHEN checking match_field
- THEN MUST return true (30 - 0 = 30, 30 % 15 == 0)

#### Scenario: Step miss

- GIVEN value = 7, min = 0, max = 59, step = 15
- WHEN checking match_field
- THEN MUST return false (7 % 15 != 0)

#### Scenario: Below range

- GIVEN value = 3, min = 5, max = 10, step = 1
- WHEN checking match_field
- THEN MUST return false

#### Scenario: Above range

- GIVEN value = 15, min = 5, max = 10, step = 1
- WHEN checking match_field
- THEN MUST return false

#### Scenario: Range with step

- GIVEN value = 8, min = 2, max = 10, step = 3
- WHEN checking match_field
- THEN MUST return true (8 - 2 = 6, 6 % 3 == 0)

#### Scenario: Range with step miss

- GIVEN value = 7, min = 2, max = 10, step = 3
- WHEN checking match_field
- THEN MUST return false (7 - 2 = 5, 5 % 3 != 0)

### Requirement: Field Match Deterministic
r[time.field-match-deterministic]

For the same (value, min, max, step), `match_field` MUST always return
the same result.

#### Scenario: Repeated call

- GIVEN value = 10, min = 0, max = 59, step = 5
- WHEN calling match_field twice
- THEN both calls MUST return true

### Requirement: Wildcard Matches All in Range
r[time.wildcard-matches-all-in-range]

When step = 1 and min/max span the full field range, every value in
[min, max] MUST match.

#### Scenario: All minutes

- GIVEN min = 0, max = 59, step = 1
- WHEN checking match_field for any value in [0, 59]
- THEN MUST return true

### Requirement: Valid Field Bounds
r[time.valid-field-bounds]

`is_valid_field(min, max, step, field_min, field_max)` MUST return true
when `field_min <= min <= max <= field_max` and `step >= 1`. MUST return
false otherwise.

#### Scenario: Valid minute field

- GIVEN min = 0, max = 59, step = 15, field_min = 0, field_max = 59
- WHEN checking is_valid_field
- THEN MUST return true

#### Scenario: Step zero

- GIVEN min = 0, max = 59, step = 0, field_min = 0, field_max = 59
- WHEN checking is_valid_field
- THEN MUST return false

#### Scenario: Min exceeds max

- GIVEN min = 10, max = 5, step = 1, field_min = 0, field_max = 59
- WHEN checking is_valid_field
- THEN MUST return false

#### Scenario: Exceeds field bounds

- GIVEN min = 0, max = 60, step = 1, field_min = 0, field_max = 59
- WHEN checking is_valid_field
- THEN MUST return false

### Cron Schedule Matching

### Requirement: All Fields Must Match
r[time.all-fields-must-match]

`matches_schedule` MUST return true only when all five fields (minute,
hour, day-of-month, month, day-of-week) match the given timestamp.
If any single field does not match, the result MUST be false.

#### Scenario: All fields match

- GIVEN schedule: minute=0, hour=3, dom=*, month=*, dow=1 (Monday)
- AND timestamp: 2026-04-06 03:00 (Monday)
- WHEN checking matches_schedule
- THEN MUST return true

#### Scenario: Minute mismatch

- GIVEN schedule: minute=0, hour=3, dom=*, month=*, dow=*
- AND timestamp: 2026-04-06 03:15
- WHEN checking matches_schedule
- THEN MUST return false

#### Scenario: Day-of-week mismatch

- GIVEN schedule: minute=0, hour=3, dom=*, month=*, dow=1 (Monday)
- AND timestamp: 2026-04-04 03:00 (Saturday)
- WHEN checking matches_schedule
- THEN MUST return false

### Requirement: Day-of-Week Consistency
r[time.day-of-week-consistency]

When matching, the day-of-week field MUST be compared against the
computed day-of-week for the given (year, month, day), not against a
caller-provided weekday value. This ensures the weekday constraint is
always consistent with the calendar.

#### Scenario: Weekday derived from date

- GIVEN schedule with dow = 6 (Saturday)
- AND timestamp: year=2026, month=4, day=4
- WHEN matching, day_of_week(2026, 4, 4) = 6
- THEN MUST match the dow field

### Requirement: Valid Schedule
r[time.valid-schedule]

`is_valid_schedule` MUST return true when all five fields have valid
bounds: minute in [0, 59], hour in [0, 23], day-of-month in [1, 31],
month in [1, 12], day-of-week in [0, 6], and all steps >= 1.

#### Scenario: Standard cron

- GIVEN minute=(0,59,15), hour=(3,3,1), dom=(1,31,1), month=(1,12,1), dow=(0,6,1)
- WHEN checking is_valid_schedule
- THEN MUST return true

#### Scenario: Invalid hour range

- GIVEN hour=(0,24,1)
- WHEN checking is_valid_schedule
- THEN MUST return false

### Next Occurrence

### Requirement: Result After Input
r[time.result-after-input]

`next_occurrence` MUST return a timestamp strictly after the input
timestamp. The result minute, hour, day, month, or year MUST differ
from the input such that the result represents a later point in time.

#### Scenario: Next minute

- GIVEN schedule: every minute (*), timestamp: 2026-04-04 10:30
- WHEN computing next_occurrence
- THEN result MUST be 2026-04-04 10:31

#### Scenario: Hour rollover

- GIVEN schedule: minute=0, timestamp: 2026-04-04 10:30
- WHEN computing next_occurrence
- THEN result MUST be 2026-04-04 11:00

### Requirement: Result Matches Schedule
r[time.result-matches-schedule]

The timestamp returned by `next_occurrence` MUST satisfy
`matches_schedule` for the given schedule. If the function returns a
result (not the "no match" sentinel), that result is a valid firing time.

#### Scenario: Weekly schedule

- GIVEN schedule: minute=0, hour=9, dom=*, month=*, dow=1 (every Monday 9:00)
- AND timestamp: 2026-04-04 12:00 (Saturday)
- WHEN computing next_occurrence
- THEN result MUST be 2026-04-06 09:00 (Monday)
- AND matches_schedule(result, schedule) MUST be true

### Requirement: Bounded Search
r[time.bounded-search]

`next_occurrence` MUST accept a `max_years` parameter. If no matching
timestamp exists within `[now, now + max_years years]`, the function
MUST return a failure indicator (false/sentinel). The search MUST NOT
examine timestamps beyond the year bound.

#### Scenario: Satisfiable within bound

- GIVEN schedule: minute=0, hour=0, dom=29, month=2, dow=* (Feb 29 midnight)
- AND timestamp: 2025-01-01, max_years = 4
- WHEN computing next_occurrence
- THEN MUST find 2028-02-29 00:00 (next leap year Feb 29)

#### Scenario: Unsatisfiable

- GIVEN schedule: minute=0, hour=0, dom=31, month=2, dow=* (Feb 31, impossible)
- AND any timestamp, max_years = 10
- WHEN computing next_occurrence
- THEN MUST return failure (no February has 31 days)

### Requirement: Month Rollover
r[time.month-rollover]

When advancing past the last day of a month, the search MUST correctly
roll over to the first valid day of the next month. When advancing past
December, the year MUST increment.

#### Scenario: End of month

- GIVEN schedule: dom=31, timestamp at April 30
- WHEN no more days in April match
- THEN search advances to May 31

#### Scenario: End of year

- GIVEN schedule: month=1, timestamp in December
- WHEN no more days in December match
- THEN search advances to January of next year

### Requirement: Leap Year Awareness
r[time.leap-year-awareness]

The next-occurrence search MUST use `days_in_month` (which accounts for
leap years) when determining how many days exist in the current month.
It MUST NOT assume all Februaries have 28 days.

#### Scenario: Crossing into leap February

- GIVEN schedule: dom=29, month=2, timestamp: 2027-01-01
- WHEN searching, 2027 is not a leap year
- THEN MUST skip 2027 February and find 2028-02-29
