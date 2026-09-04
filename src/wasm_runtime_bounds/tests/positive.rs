use super::*;

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.model]
#[test]
fn valid_model_runs_to_finished_with_exact_fuel() {
    let model = linear_model();
    let result = run_bounded(&model, running_cursor(INITIAL_FUEL), &observations(), PREFIX_BUDGET + SUFFIX_BUDGET)
        .expect("valid finite model should run");

    assert_eq!(result.class, RunClass::Finished);
    assert_eq!(result.cursor.state, state(STATE_THREE));
    assert_eq!(result.cursor.fuel.remaining, INITIAL_FUEL - PREFIX_BUDGET - SUFFIX_BUDGET);
    assert_eq!(result.cursor.next_observation, PREFIX_BUDGET + SUFFIX_BUDGET);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.fuel]
#[test]
fn zero_budget_is_identity_out_of_fuel_boundary() {
    let start = running_cursor(INITIAL_FUEL);
    let result = run_bounded(&linear_model(), start, &observations(), 0).expect("zero budget is admitted");

    assert_eq!(result.class, RunClass::OutOfFuel);
    assert_eq!(result.cursor, start);
    assert_eq!(result.consumed_steps, 0);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.fuel]
#[test]
fn terminal_state_absorbs_later_budget() {
    let start = Cursor {
        state: state(STATE_THREE),
        status: MachineStatus::Finished,
        fuel: Fuel {
            remaining: INITIAL_FUEL,
        },
        next_observation: 0,
    };
    let result =
        run_bounded(&linear_model(), start, &observations(), SUFFIX_BUDGET).expect("terminal state should be absorbed");

    assert_eq!(result.class, RunClass::Finished);
    assert_eq!(result.cursor, start);
    assert_eq!(result.consumed_steps, 0);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.segmentation]
#[test]
fn explicit_host_event_pauses_after_one_step() {
    let result =
        run_bounded(&pause_model(), running_cursor(INITIAL_FUEL), &[observation(OBSERVATION_FIRST)], PREFIX_BUDGET)
            .expect("explicit pause transition should run");

    assert_eq!(result.class, RunClass::HostPaused);
    assert_eq!(result.cursor.status, MachineStatus::HostPaused);
    assert_eq!(result.cursor.state, state(STATE_ONE));
    assert_eq!(result.consumed_steps, PREFIX_BUDGET);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.model]
#[test]
fn explicit_trap_stops_after_one_step() {
    let result =
        run_bounded(&trap_model(), running_cursor(INITIAL_FUEL), &[observation(OBSERVATION_FIRST)], PREFIX_BUDGET)
            .expect("explicit trap transition should run");

    assert_eq!(result.class, RunClass::Trapped(TrapCode(TRAP_CODE)));
    assert_eq!(result.cursor.status, MachineStatus::Trapped(TrapCode(TRAP_CODE)));
    assert_eq!(result.consumed_steps, PREFIX_BUDGET);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.segmentation]
#[test]
fn nonterminal_segments_match_uninterrupted_execution() {
    let model = linear_model();
    let input = observations();
    let result = compare_segments(SegmentRequest {
        model: &model,
        resume_model: &model,
        start: running_cursor(INITIAL_FUEL),
        observations: &input,
        resume_observations: &input,
        prefix_budget: PREFIX_BUDGET,
        suffix_budget: PREFIX_BUDGET,
    })
    .expect("matching deterministic segments should compose");

    assert_eq!(result.class, RunClass::OutOfFuel);
    assert_eq!(result.cursor.state, state(STATE_TWO));
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.segmentation]
#[test]
fn terminal_prefix_boundary_matches_uninterrupted_execution() {
    let model = linear_model();
    let input = observations();
    let result = compare_segments(SegmentRequest {
        model: &model,
        resume_model: &model,
        start: running_cursor(INITIAL_FUEL),
        observations: &input,
        resume_observations: &input,
        prefix_budget: PREFIX_BUDGET + SUFFIX_BUDGET,
        suffix_budget: SUFFIX_BUDGET,
    })
    .expect("terminal prefix should absorb the suffix");

    assert_eq!(result.class, RunClass::Finished);
    assert_eq!(result.consumed_steps, PREFIX_BUDGET + SUFFIX_BUDGET);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.continuation]
#[test]
fn matching_out_of_fuel_continuation_is_admitted() {
    let boundary = run_bounded(&linear_model(), running_cursor(INITIAL_FUEL), &observations(), PREFIX_BUDGET)
        .expect("prefix should run");
    let continuation = capture_continuation(boundary, bindings()).expect("out-of-fuel boundary should be capturable");
    let grant = admit_resume(&continuation, bindings()).expect("matching bindings should resume");

    assert_eq!(grant.reason, ContinuationReason::OutOfFuel);
    assert_eq!(grant.cursor, continuation.cursor);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.continuation]
#[test]
fn matching_host_pause_continuation_returns_running_cursor() {
    let boundary =
        run_bounded(&pause_model(), running_cursor(INITIAL_FUEL), &[observation(OBSERVATION_FIRST)], PREFIX_BUDGET)
            .expect("pause transition should run");
    let continuation = capture_continuation(boundary, bindings()).expect("host pause should be capturable");
    let grant = admit_resume(&continuation, bindings()).expect("matching pause bindings should resume");

    assert_eq!(grant.reason, ContinuationReason::HostPause);
    assert_eq!(grant.cursor.status, MachineStatus::Running);
    assert_eq!(grant.cursor.state, continuation.cursor.state);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.allocation]
#[test]
fn initialization_allocation_is_monotone_and_capped() {
    let initial = allocator(AllocationPhase::Initializing);
    let allocated = allocate_pages(initial, PAGE_REQUEST).expect("request within capacity should fit");
    let running = begin_execution(allocated).expect("valid initialized allocation should start");
    let finished = finish_execution(running).expect("running allocation should finish");

    assert_eq!(allocated.allocated_pages, PAGE_REQUEST);
    assert!(allocated.allocated_pages <= allocated.capacity_pages);
    assert_eq!(finished.phase, AllocationPhase::Finished);
    assert_eq!(finished.allocated_pages, PAGE_REQUEST);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.growth-policy]
#[test]
fn growth_policy_denies_or_delegates_without_changing_existing_laws() {
    let mut memory = crate::wasm_primitives::new_memory(INITIAL_MEMORY_PAGES).expect("one page should be valid");
    let original_bytes = memory.bytes.clone();
    let original_pages = memory.pages;

    assert_eq!(apply_growth(&mut memory, GROWTH_PAGE_COUNT, GrowthPolicy::Deny), GrowthOutcome::Denied,);
    assert_eq!(memory.pages, original_pages);
    assert_eq!(memory.bytes, original_bytes);

    assert_eq!(apply_growth(&mut memory, GROWTH_PAGE_COUNT, GrowthPolicy::Allow), GrowthOutcome::Applied {
        previous_pages: INITIAL_MEMORY_PAGE_COUNT,
    },);
    assert_eq!(memory.pages, EXPECTED_GROWN_MEMORY_PAGES);
    assert_eq!(&memory.bytes[..original_bytes.len()], original_bytes.as_slice());
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.streaming]
#[test]
fn empty_and_exact_chunk_schedules_are_admitted() {
    let empty = ChunkSchedule {
        total_bytes: 0,
        chunks: Vec::new(),
    };
    assert_eq!(admit_schedule(&empty), Ok(()));

    let schedule = exact_schedule();
    assert_eq!(admit_schedule(&schedule), Ok(()));
    for byte_offset in 0..BYTE_TOTAL {
        assert_eq!(coverage_count(&schedule, byte_offset), Some(1));
    }
}

// r[verify wasm-runtime-bounds.docs]
// r[verify wasm-runtime-bounds.final-validation]
// r[verify wasm-runtime-bounds.non-claims]
#[test]
fn public_boundary_admits_only_abstract_law_claims() {
    assert_eq!(admit_claim(ClaimScope::AbstractLaws), Ok(()));
}
