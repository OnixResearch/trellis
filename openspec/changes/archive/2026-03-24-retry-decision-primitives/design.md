## Context

The retry module sits between error detection and backoff computation.
After an operation fails, the caller needs three answers: should I retry?
how many attempts remain? is the overall retry budget blown? These are
pure boolean/integer functions with no I/O.

## Goals / Non-Goals

**Goals:** Machine-checked proofs that retry decisions are bounded, that
non-retryable errors never produce RETRY outcomes, and that classification
is exhaustive.

**Non-Goals:** Backoff timing (covered by `backoff`), circuit state
(covered by `circuit_breaker`), jitter, async retry loops.

## Decisions

### 1. Integer-encoded outcomes

**Choice:** SUCCESS = 0, RETRY = 1, FAIL = 2 as `pub const u8`
**Rationale:** Same pattern as circuit_breaker (CLOSED/OPEN/HALF_OPEN),
batch (FlushDecision), process_state. Verus can't match on enum variants
in spec functions.
**Alternative:** Enum with integer proxy — rejected, adds indirection
without verification benefit.

### 2. Budget uses multiplication, not division

**Choice:** `failures * 100 <= total * budget_pct` instead of
`failures / total <= budget_pct / 100`
**Rationale:** Avoids division-by-zero when total == 0 and integer
truncation errors. The multiplication form is exact for all inputs
where `total * budget_pct <= u64::MAX`, which we require as a
precondition.
**Alternative:** Division with special-case zero — rejected, adds
branches and loses precision.

### 3. No composition with backoff/circuit_breaker in this module

**Choice:** Keep retry as pure decision logic. Cross-module properties
(e.g., should_retry && circuit_breaker == CLOSED → compute_backoff)
go in `properties.rs`.
**Rationale:** Matches project convention. Each module proves its own
properties; cross-module links are separate.

## Risks / Trade-offs

**[Overflow in budget check]** The multiplication `failures * 100` or
`total * budget_pct` can overflow u64. Mitigated by requiring
`total as int * budget_pct as int <= u64::MAX as int` in the spec's
recommends and the exec's requires. Callers must ensure the window
isn't astronomically large.
