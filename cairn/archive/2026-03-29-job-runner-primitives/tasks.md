## Phase 1: Job State Machine + Slot Allocation

- [x] Implement src/job_state.rs: state constants (PENDING=0 through CANCELLED=5), is_valid_state spec+exec ✅
- [x] Implement is_valid_transition spec+exec with all 7 permitted edges ✅
- [x] Implement is_terminal spec+exec (Completed, Failed, Cancelled) ✅
- [x] Implement can_become_ready spec+exec: all dep_states == COMPLETED ✅
- [x] Prove: terminal states have no outgoing transitions ✅
- [x] Prove: Cancelled reachable from every non-terminal state ✅
- [x] Prove: transition exhaustiveness — every (from, to) pair classified ✅
- [x] Unit tests + proptest for job_state ✅ (11 tests)
- [x] Implement src/job_slot.rs: try_acquire_slot spec+exec, release_slot spec+exec ✅
- [x] Implement has_available_slot spec+exec ✅
- [x] Prove: capacity invariant 0 <= in_use <= capacity after acquire ✅
- [x] Prove: release never underflows (saturating at 0) ✅
- [x] Prove: acquire fails iff in_use >= capacity ✅
- [x] Unit tests + proptest for job_slot ✅ (9 tests)
- [x] Verus + tracey clean ✅ (21 + 13 = 34 verified)

## Phase 2: Priority Ordering + Idempotency

- [x] Implement src/job_priority.rs: is_higher_priority spec+exec (class, seq) lexicographic ✅
- [x] Prove transitivity: A > B ∧ B > C → A > C ✅
- [x] Prove antisymmetry: A > B → ¬(B > A) ✅
- [x] Prove totality: A ≠ B → A > B ∨ B > A ✅
- [x] Implement select_highest_priority spec+exec over a sequence of pairs ✅
- [x] Prove: result is a valid index and no other element has higher priority ✅
- [x] Unit tests + proptest for job_priority ✅ (8 tests)
- [x] Implement src/job_idempotency.rs: is_consumed spec+exec (linear membership scan) ✅
- [x] Implement try_execute spec+exec: returns (proceed, needs_append) ✅
- [x] Prove: fresh token → proceed is true ✅
- [x] Prove: consumed token → proceed is false, set unchanged ✅
- [x] Prove: monotonicity — consumed set size never decreases ✅
- [x] Unit tests + proptest for job_idempotency ✅ (7 tests)
- [x] Verus + tracey clean ✅ (11 + 10 = 21 verified)

## Phase 3: Cancellation Propagation

- [x] Implement src/job_cancel.rs: should_cancel_job spec+exec — checks if all deps are cancelled/failed (no surviving path) ✅
- [x] Implement has_surviving_dep spec+exec — linear scan for completed, non-cancelled dep ✅
- [x] Implement count_new_cancellations spec — count new cancellations per pass ✅
- [x] Prove: already-terminal jobs unaffected (Completed/Failed not re-cancelled) ✅
- [x] Prove: root jobs (no deps) never propagated ✅
- [x] Prove: count bounded by total job count ✅
- [x] Prove: count non-negative ✅
- [x] Unit tests + proptest for job_cancel ✅ (8 tests)
- [x] Verus + tracey clean ✅ (18 verified)

## Phase 4: Cross-Module Properties + Integration

- [x] job_state::can_become_ready links to job_dag::all_deps_satisfied (readiness_implies_dag_satisfied proof) ✅
- [x] job_slot::try_acquire → Running requires a slot (running_needs_slot proof) ✅
- [x] job_cancel uses job_state::is_terminal to skip already-terminal jobs (cancel_terminal_matches_state proof) ✅
- [x] job_priority::select_highest feeds into job_state Ready→Running scheduling (higher_priority_runs_first proof) ✅
- [x] Property tests for all cross-module relationships ✅ (5 proptests)
- [x] Add all modules to src/lib.rs ✅
- [x] Verus + tracey clean ✅ (4102 total crate verified, 0 errors)
