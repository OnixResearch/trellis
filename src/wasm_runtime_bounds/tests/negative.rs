use super::*;

const DIFFERENT_ID: u64 = 99;
const TOO_LARGE_REQUEST: u32 = PAGE_CAPACITY + 1;
const TWO_STEP_BUDGET: u32 = 2;
const OVERFLOW_REQUEST: u32 = 2;
const OVERFLOW_CHUNK_LENGTH: u64 = 2;
const MISSING_OBSERVATION: u32 = 90;
const GAP_OFFSET: u64 = BYTE_FIRST_LENGTH + 1;
const OVERLAP_OFFSET: u64 = BYTE_FIRST_LENGTH - 1;
const TRAILING_TOTAL: u64 = BYTE_TOTAL + 1;
const TOO_MANY_CHUNKS: usize = MAX_CHUNKS as usize + 1;

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.model]
#[test]
fn malformed_transition_models_are_rejected() {
    let mut duplicate = linear_model();
    duplicate.transitions.push(duplicate.transitions[0]);
    assert_eq!(
        admit_model(&duplicate),
        Err(ModelFaultSite {
            class: ModelFault::DuplicateTransitionKey,
            index: Some(STATE_THREE),
        }),
    );

    let mut source_outside = linear_model();
    source_outside.transitions[0].from = state(STATE_COUNT);
    assert_eq!(
        admit_model(&source_outside),
        Err(ModelFaultSite {
            class: ModelFault::SourceOutsideModel,
            index: Some(0),
        }),
    );

    let mut target_outside = linear_model();
    target_outside.transitions[0].effect = StepEffect::Continue(state(STATE_COUNT));
    assert_eq!(
        admit_model(&target_outside),
        Err(ModelFaultSite {
            class: ModelFault::TargetOutsideModel,
            index: Some(0),
        }),
    );
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.fuel]
#[test]
fn insufficient_fuel_returns_the_original_value() {
    let available = Fuel {
        remaining: PREFIX_BUDGET,
    };
    let fault = consume_fuel(available, SUFFIX_BUDGET).expect_err("insufficient fuel must reject");

    assert_eq!(fault.available, available);
    assert_eq!(fault.requested, SUFFIX_BUDGET);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.model]
#[test]
fn missing_observation_or_transition_rejects_execution() {
    let start = running_cursor(INITIAL_FUEL);
    assert_eq!(
        run_bounded(&linear_model(), start, &[observation(OBSERVATION_FIRST)], TWO_STEP_BUDGET,),
        Err(RunFault::ObservationExhausted { at_step: PREFIX_BUDGET }),
    );
    assert_eq!(start, running_cursor(INITIAL_FUEL));

    assert_eq!(
        run_bounded(&linear_model(), start, &[observation(MISSING_OBSERVATION)], PREFIX_BUDGET,),
        Err(RunFault::MissingTransition { at_step: 0 }),
    );
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.segmentation]
#[test]
fn segment_comparison_rejects_model_observation_and_budget_drift() {
    let model = linear_model();
    let mut changed_model = linear_model();
    changed_model.transitions[0].effect = StepEffect::Trap(TrapCode(TRAP_CODE));
    let input = observations();
    let mut changed_input = observations();
    changed_input[0] = observation(MISSING_OBSERVATION);

    assert_eq!(
        compare_segments(SegmentRequest {
            model: &model,
            resume_model: &changed_model,
            start: running_cursor(INITIAL_FUEL),
            observations: &input,
            resume_observations: &input,
            prefix_budget: PREFIX_BUDGET,
            suffix_budget: SUFFIX_BUDGET,
        }),
        Err(SegmentationFault::ModelDrift),
    );
    assert_eq!(
        compare_segments(SegmentRequest {
            model: &model,
            resume_model: &model,
            start: running_cursor(INITIAL_FUEL),
            observations: &input,
            resume_observations: &changed_input,
            prefix_budget: PREFIX_BUDGET,
            suffix_budget: SUFFIX_BUDGET,
        }),
        Err(SegmentationFault::ObservationDrift),
    );
    assert_eq!(
        compare_segments(SegmentRequest {
            model: &model,
            resume_model: &model,
            start: running_cursor(INITIAL_FUEL),
            observations: &input,
            resume_observations: &input,
            prefix_budget: MAX_STEPS,
            suffix_budget: PREFIX_BUDGET,
        }),
        Err(SegmentationFault::CombinedBudgetOverflow),
    );
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.continuation]
#[test]
fn unsupported_boundary_cannot_be_captured() {
    let result = RunResult {
        cursor: running_cursor(INITIAL_FUEL),
        consumed_steps: 0,
        class: RunClass::Finished,
    };
    assert_eq!(capture_continuation(result, bindings()), Err(CaptureFault::UnsupportedBoundary),);

    let forged = Continuation {
        cursor: running_cursor(INITIAL_FUEL),
        reason: ContinuationReason::HostPause,
        bindings: bindings(),
    };
    assert_eq!(admit_resume(&forged, bindings()), Err(ResumeFault::InconsistentBoundary),);
}

fn out_of_fuel_continuation() -> Continuation {
    let boundary = run_bounded(&linear_model(), running_cursor(INITIAL_FUEL), &observations(), PREFIX_BUDGET)
        .expect("prefix should run");
    capture_continuation(boundary, bindings()).expect("out-of-fuel result should capture")
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.continuation]
#[test]
fn every_stale_continuation_binding_rejects_without_mutation() {
    let continuation = out_of_fuel_continuation();
    let original = continuation;

    let mut current = bindings();
    current.generation = GenerationId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::GenerationMismatch));

    let mut current = bindings();
    current.artifact = ArtifactId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::ArtifactMismatch));

    let mut current = bindings();
    current.profile = ProfileId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::ProfileMismatch));

    let mut current = bindings();
    current.input = InputId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::InputMismatch));

    let mut current = bindings();
    current.context = ContextId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::ContextMismatch));

    let mut current = bindings();
    current.observations = ObservationSetId(DIFFERENT_ID);
    assert_eq!(admit_resume(&continuation, current), Err(ResumeFault::ObservationMismatch));
    assert_eq!(continuation, original);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.allocation]
#[test]
fn allocation_failures_leave_the_original_state_unchanged() {
    let initial = allocator(AllocationPhase::Initializing);
    assert_eq!(allocate_pages(initial, TOO_LARGE_REQUEST), Err(AllocationFault::OutOfMemory),);
    assert_eq!(initial, allocator(AllocationPhase::Initializing));

    let running = allocator(AllocationPhase::Running);
    assert_eq!(allocate_pages(running, PAGE_REQUEST), Err(AllocationFault::PhaseClosed),);
    assert_eq!(running, allocator(AllocationPhase::Running));

    let full = PageAllocator {
        phase: AllocationPhase::Initializing,
        capacity_pages: MAX_ALLOCATED_PAGES,
        allocated_pages: MAX_ALLOCATED_PAGES,
    };
    assert_eq!(allocate_pages(full, OVERFLOW_REQUEST), Err(AllocationFault::CountOverflow),);
    assert_eq!(full.allocated_pages, MAX_ALLOCATED_PAGES);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.growth-policy]
#[test]
fn admitted_growth_failure_preserves_memory() {
    let mut memory = crate::wasm_primitives::new_memory(INITIAL_MEMORY_PAGES).expect("one page should be valid");
    let old_pages = memory.pages;
    let old_bytes = memory.bytes.clone();

    assert_eq!(apply_growth(&mut memory, u32::MAX, GrowthPolicy::Allow), GrowthOutcome::Rejected,);
    assert_eq!(memory.pages, old_pages);
    assert_eq!(memory.bytes, old_bytes);

    let mut malformed = crate::wasm_primitives::LinearMemory {
        bytes: Vec::new(),
        pages: INITIAL_MEMORY_PAGES,
    };
    assert_eq!(apply_growth(&mut malformed, GROWTH_PAGE_COUNT, GrowthPolicy::Allow), GrowthOutcome::InvalidMemory,);
    assert_eq!(malformed.pages, INITIAL_MEMORY_PAGES);
    assert!(malformed.bytes.is_empty());
}

fn fault_class(schedule: &ChunkSchedule) -> ScheduleFault {
    admit_schedule(schedule).expect_err("invalid schedule must reject").class
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.streaming]
#[test]
fn invalid_chunk_shapes_fail_closed() {
    let mut schedule = exact_schedule();
    schedule.chunks[STATE_ONE as usize].length = 0;
    assert_eq!(fault_class(&schedule), ScheduleFault::ZeroLength);

    let mut schedule = exact_schedule();
    schedule.chunks[STATE_ONE as usize].offset = GAP_OFFSET;
    assert_eq!(fault_class(&schedule), ScheduleFault::OffsetMismatch);

    let mut schedule = exact_schedule();
    schedule.chunks[STATE_ONE as usize].offset = OVERLAP_OFFSET;
    assert_eq!(fault_class(&schedule), ScheduleFault::OffsetMismatch);

    let mut schedule = exact_schedule();
    schedule.total_bytes = TRAILING_TOTAL;
    assert_eq!(fault_class(&schedule), ScheduleFault::FinalLengthMismatch);

    let missing = ChunkSchedule {
        total_bytes: BYTE_TOTAL,
        chunks: Vec::new(),
    };
    assert_eq!(fault_class(&missing), ScheduleFault::MissingChunks);

    let ambiguous_empty = ChunkSchedule {
        total_bytes: 0,
        chunks: vec![Chunk {
            offset: 0,
            length: PREFIX_BUDGET.into(),
        }],
    };
    assert_eq!(fault_class(&ambiguous_empty), ScheduleFault::UnexpectedEmptyInputChunk,);
}

// r[verify wasm-runtime-bounds.evidence]
// r[verify wasm-runtime-bounds.streaming]
#[test]
fn chunk_overflow_and_trailing_ranges_reject() {
    let overflow = ChunkSchedule {
        total_bytes: MAX_STREAM_BYTES,
        chunks: vec![
            Chunk {
                offset: 0,
                length: MAX_STREAM_BYTES,
            },
            Chunk {
                offset: MAX_STREAM_BYTES,
                length: OVERFLOW_CHUNK_LENGTH,
            },
        ],
    };
    assert_eq!(fault_class(&overflow), ScheduleFault::RangeOverflow);

    let source_too_large = ChunkSchedule {
        total_bytes: u64::MAX,
        chunks: Vec::new(),
    };
    assert_eq!(fault_class(&source_too_large), ScheduleFault::SourceTooLarge,);

    let too_many = ChunkSchedule {
        total_bytes: BYTE_TOTAL,
        chunks: vec![
            Chunk {
                offset: 0,
                length: PREFIX_BUDGET.into(),
            };
            TOO_MANY_CHUNKS
        ],
    };
    assert_eq!(fault_class(&too_many), ScheduleFault::TooManyChunks);

    let beyond = ChunkSchedule {
        total_bytes: BYTE_TOTAL,
        chunks: vec![Chunk {
            offset: 0,
            length: TRAILING_TOTAL,
        }],
    };
    assert_eq!(fault_class(&beyond), ScheduleFault::BeyondDeclaredEnd);
}

// r[verify wasm-runtime-bounds.non-claims]
// r[verify wasm-runtime-bounds.evidence]
#[test]
fn concrete_runtime_claim_promotions_reject() {
    let unsupported = [
        ClaimScope::ConcreteInterpreter,
        ClaimScope::UnsafeAllocator,
        ClaimScope::HostScheduling,
        ClaimScope::SerializationOrMigration,
        ClaimScope::Sandbox,
        ClaimScope::Certification,
        ClaimScope::AdapterCorrectness,
    ];
    for scope in unsupported {
        assert_eq!(admit_claim(scope), Err(ClaimFault::UnsupportedPromotion));
    }
}
