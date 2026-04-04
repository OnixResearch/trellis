## Context

verified-logic handles time as scalar u64 values — deadlines, durations,
expiration timestamps. The `interval` module does half-open range math.
The `job_timeout` module computes deadlines from start + duration. None
decompose time into calendar fields.

Related existing modules: `deadline_queue` (EDF ordering on u64
deadlines), `token_timestamp` (expiration with skew tolerance),
`job_timeout` (start + duration → deadline), `interval` (half-open
range contains/overlap/split).

The gap: no leap year logic, no days-in-month, no day-of-week, no cron
field matching, no next-occurrence search.

## Goals / Non-Goals

**Goals:**
- Verified leap year predicate matching the Gregorian calendar rules
- Verified days-in-month with leap-year-aware February
- Verified day-of-week computation from (year, month, day)
- Verified date validation (month in range, day ≤ days-in-month)
- Verified cron field matching with (min, max, step) representation
- Verified five-field cron schedule matching
- Verified next-occurrence search with bounded year horizon
- All modules follow existing pattern: spec fn + exec fn + proof fn + tests

**Non-Goals:**
- Timezone conversion or DST handling
- Epoch ↔ calendar conversion (requires division chains; out of scope)
- Cron string parsing (`"*/5 3 * * 1"` → struct) — that's a parser, not logic
- Sub-minute precision (cron operates on minutes)
- Named months or weekdays (use integer encodings)
- Persistent schedule storage or timer management

## Decisions

### 1. Calendar values as u64, not a date struct

**Choice:** Functions take `(year: u64, month: u64, day: u64)` as
separate parameters. Month is 1-indexed (1=January, 12=December).
Day-of-week returns 0=Sunday through 6=Saturday.

**Rationale:** Matches the library convention — every module uses flat
u64 parameters. Verus reasons about integer arithmetic directly. A
struct would add indirection without helping Z3. The caller composes
validation (`is_valid_date`) with computation (`day_of_week`) at the
call site.

**Alternative:** A `Date` struct with invariants. Rejected — every other
module in the library uses flat integers, and Verus handles struct
invariants awkwardly compared to preconditions.

### 2. Leap year uses the full Gregorian rule

**Choice:** `is_leap_year(y)` returns true when `(y % 4 == 0 && y % 100 != 0) || y % 400 == 0`.

**Rationale:** The standard rule. Year 2000 is leap, 1900 is not, 2024
is leap. Verus handles modular arithmetic with `%` directly. No reason
to simplify — the full rule is three comparisons.

**Alternative:** Julian rule (just `y % 4 == 0`). Rejected — incorrect
for the Gregorian calendar.

### 3. Day-of-week via Tomohiko Sakamoto's algorithm

**Choice:** Use Sakamoto's method, which computes day-of-week from
(year, month, day) using a small lookup table and modular arithmetic.
The table is 12 entries (one per month offset).

**Rationale:** The algorithm is compact (one array lookup, a few
additions, one mod-7), well-documented, and maps cleanly to Verus exec
code. Zeller's congruence is equivalent but uses floor division, which
is harder to reason about in Verus. Sakamoto's uses only addition and
mod.

**Alternative:** Zeller's congruence. Rejected — floor division on
negative intermediates complicates the Verus proof for January/February
adjustment. Sakamoto avoids negative values entirely.

**Implementation:** The month offset table `[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4]`
is encoded as a match expression (Verus doesn't support array indexing
with the same ease as spec-level Seq). The year is adjusted by -1 for
January and February before the main computation.

### 4. Cron fields as (min, max, step) triples

**Choice:** A cron field is three u64 values: `min`, `max`, `step`.
A value `v` matches when `min <= v <= max` and `(v - min) % step == 0`.
Wildcard `*` becomes `(field_min, field_max, 1)`. Step `*/N` becomes
`(field_min, field_max, N)`. Range `A-B` becomes `(A, B, 1)`. Range
with step `A-B/N` becomes `(A, B, N)`.

**Rationale:** Covers all standard cron field patterns with a single
representation. The matching predicate is one range check and one mod
operation — straightforward for Z3. No need for a set/bitvector
representation since cron fields are small ranges with regular steps.

**Alternative:** Bitvector (one bit per valid value). Works for minutes
(0-59 fits in u64) but day-of-month and month are awkward, and it
obscures the step structure that makes proofs clean. Rejected.

### 5. Schedule as five independent field triples

**Choice:** A schedule is 15 u64 values: `(min, max, step)` for each of
minute, hour, day-of-month, month, day-of-week. `matches_schedule`
checks all five fields. Day-of-week matching also validates against
`day_of_week(year, month, day)`.

**Rationale:** Flat parameters, no struct. Keeps the function signature
explicit about what's being matched. The five fields are independent
(standard cron semantics: a time matches when ALL fields match).

Cron's day-of-month vs day-of-week interaction is OR in some
implementations (Vixie cron) and AND in others (POSIX). We use AND
semantics — both must match. This is the simpler model to verify, and
callers can compose OR externally.

**Alternative:** OR semantics for day fields. Rejected — harder to
verify (the match predicate becomes conditional on which fields are
wildcards), and AND is the POSIX-specified behavior.

### 6. Next occurrence as bounded forward search

**Choice:** `next_occurrence` scans forward from the current timestamp
minute-by-minute, advancing through minute → hour → day → month → year
when a field doesn't match. Stops when all five fields match or when
the year exceeds `current_year + max_years`.

The search skips non-matching months entirely (advance to first valid
month), then non-matching days within a month (advance to first valid
day), then non-matching hours, then non-matching minutes. This avoids
scanning all 525,600 minutes in a year.

**Rationale:** Forward search is the standard algorithm for cron next
occurrence. The field-skip optimization makes it practical (most
schedules match within the first few iterations). The year bound
guarantees termination — Verus needs a decreasing measure, and
`(max_year - current_year) * 12 * 31 * 24 * 60 - elapsed_minutes`
serves as the variant.

**Alternative:** Backtracking search (try each field from year down,
backtrack on mismatch). More complex control flow, harder to specify a
decreasing measure. Rejected.

### 7. Phased implementation

**Choice:**
1. `calendar` — leap year, days-in-month, date validation, day-of-week
2. `cron_field` — single field matching with (min, max, step)
3. `cron_match` — five-field schedule matching (depends on calendar + cron_field)
4. `cron_next` — next occurrence search (depends on cron_match + calendar)

**Rationale:** Each phase builds on the previous. `calendar` is
standalone. `cron_field` is standalone. `cron_match` composes both.
`cron_next` uses `cron_match` to test candidates and `calendar` to
advance dates correctly (days-in-month for month rollover).

## Risks / Trade-offs

**[Year range limited to u64]** → Year 0 through u64::MAX. Practically
nobody schedules cron jobs in year 0 or year 18 quintillion. The u64
representation is consistent with the rest of the library. Preconditions
can require year ≥ 1970 if callers want Unix-era bounds.

**[No timezone or DST]** → Deliberate. Cron fires in a fixed timezone;
the conversion from UTC to local time is the caller's responsibility.
Modeling DST requires a timezone database, which is I/O and data, not
pure logic.

**[AND semantics for day fields]** → Some users expect Vixie cron's OR
behavior (if both day-of-month and day-of-week are specified, fire when
EITHER matches). We document AND semantics. Callers who want OR can
call `matches_schedule` twice with one field wildcarded and combine the
results.

**[Sakamoto's algorithm precondition: year ≥ 1]** → The algorithm
adjusts year by -1 for January/February, so year 0 would underflow.
We add a precondition `year >= 1`, which is a reasonable constraint.

**[Bounded search may return None]** → If no matching time exists within
the year horizon (e.g., schedule asks for February 30), `next_occurrence`
returns a sentinel or success flag. This is correct behavior — an
unsatisfiable schedule should not loop forever.
