## Context

verified-logic has six job runner modules: `job_state` (lifecycle FSM),
`job_slot` (concurrency slots), `job_priority` (scheduling order),
`job_idempotency` (duplicate prevention), `job_cancel` (cascade
propagation), and `job_dag` (dependency tracking). These handle the
scheduling side — deciding when to run a job.

Related existing modules: `retry` (generic should_retry), `expiration`
(TTL checks), `backoff` (delay computation), `circuit_breaker`
(failure tracking). None of these model job-specific execution
concerns: retry with error classification, deadline extension, or
multi-criteria result validation.

## Goals / Non-Goals

**Goals:**
- Verified retry decision with error classification and cooldown
- Verified timeout detection with deadline extension and grace periods
- Verified result validation with exit code + hash + size criteria
- All modules follow existing pattern: spec fn + exec fn + proof fn + tests
- Cross-module link: retry→job_state (Failed→retry→Ready),
  timeout→job_state (timeout→Failed), result→job_state (validation→state)

**Non-Goals:**
- Actual job execution (process spawning, I/O)
- Backoff delay computation — covered by existing `backoff` module
- Generic retry logic — covered by existing `retry` module
- Time measurement — we take timestamps as inputs
- Hash computation — we take pre-computed hashes as inputs

## Decisions

### 1. Error codes as u64 with range-based classification

**Choice:** Error codes are plain u64 values. Classification uses
fixed ranges: 0-99 permanent, 100-199 transient (retryable), 200+
unknown (non-retryable).

**Rationale:** Range-based classification is common in HTTP status
codes (4xx vs 5xx), gRPC codes, and job runner APIs. Using fixed
ranges avoids needing an error code registry or lookup table. Z3
handles range comparisons instantly.

**Alternative:** Bitmap flags or enum. Rejected — adds complexity
for no verification benefit, and Verus handles integer ranges better
than bitfields for this use case.

### 2. Cooldown as timestamp comparison, not duration tracking

**Choice:** `is_cooldown_elapsed(now, last_attempt_time, cooldown_ms)`
compares `now >= last_attempt_time + cooldown_ms` with saturating
addition. The caller provides timestamps.

**Rationale:** Same pattern as `expiration::is_expired` and
`token_timestamp::is_expired`. Keeping time as external input makes
the logic pure and testable. Saturating addition handles overflow
the same way as the expiration module.

### 3. Deadline extension bounded by max_extension parameter

**Choice:** `extend_deadline(deadline, grace, max_extension)` returns
`min(deadline + grace, deadline + max_extension)` with saturation.
The caller controls how much extension is permitted.

**Rationale:** Unbounded extensions defeat the purpose of timeouts.
The max_extension parameter lets the caller enforce policy (e.g.,
"at most 2x the original timeout"). The min() cap is simple for Z3
to reason about.

### 4. Result validation returns first-failure index, not bool

**Choice:** `validate_result(...)` returns u8: 0=pass, 1=bad exit,
2=bad hash, 3=bad size. Returns the FIRST failing criterion.

**Rationale:** Matches the `evaluate_conditions` pattern from the
CAS module — the caller needs to know which criterion failed for
error reporting. First-failure ordering means the result is
deterministic even when multiple criteria fail.

**Alternative:** Return a tuple of three bools. Rejected — first-failure
gives a single deterministic answer and is easier to prove properties
about (e.g., 0 iff all pass).

### 5. u8 status constants for validation results

**Choice:** `pub const VALID: u8 = 0; pub const BAD_EXIT: u8 = 1;`
etc. Same encoding pattern as all other state modules.

**Rationale:** Consistent with job_state, process_state, circuit_breaker.

## Risks / Trade-offs

**[Fixed error code ranges are arbitrary]** → True, but the logic
is parameterized — callers can map their error codes to the 0-99 /
100-199 / 200+ ranges. The proof that retryable errors are in [100,200)
and non-retryable are outside that range holds regardless of semantic
meaning.

**[No retry backoff]** → By design. The existing `backoff` module
computes delays. This module decides *whether* to retry, not *when*.
The caller composes: `if should_retry(...) { delay = compute_backoff(...) }`.

**[Hash comparison is u64 equality]** → We take pre-hashed values.
The hash function itself is out of scope (same as `chain` module
taking hash_spec as uninterpreted). If the caller needs cryptographic
hash verification, they use `chain::compute_entry_hash` or similar.
