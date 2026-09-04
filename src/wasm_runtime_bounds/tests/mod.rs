//! Positive, negative, and property evidence.

use super::*;

mod negative;
mod positive;
mod property;

const STATE_COUNT: u32 = 4;
const STATE_ZERO: u32 = 0;
const STATE_ONE: u32 = 1;
const STATE_TWO: u32 = 2;
const STATE_THREE: u32 = 3;
const OBSERVATION_FIRST: u32 = 10;
const OBSERVATION_SECOND: u32 = 11;
const OBSERVATION_THIRD: u32 = 12;
const TRAP_CODE: u32 = 40;
const GENERATION_ID: u64 = 20;
const ARTIFACT_ID: u64 = 21;
const PROFILE_ID: u64 = 22;
const INPUT_ID: u64 = 23;
const CONTEXT_ID: u64 = 24;
const OBSERVATION_SET_ID: u64 = 25;
const INITIAL_FUEL: u32 = 8;
const PREFIX_BUDGET: u32 = 1;
const SUFFIX_BUDGET: u32 = 2;
const PAGE_CAPACITY: u32 = 8;
const PAGE_REQUEST: u32 = 3;
const INITIAL_MEMORY_PAGES: usize = 1;
const INITIAL_MEMORY_PAGE_COUNT: u32 = 1;
const GROWTH_PAGE_COUNT: u32 = 1;
const EXPECTED_GROWN_MEMORY_PAGES: usize = 2;
const BYTE_TOTAL: u64 = 9;
const BYTE_FIRST_LENGTH: u64 = 4;
const BYTE_SECOND_LENGTH: u64 = 3;
const BYTE_THIRD_LENGTH: u64 = 2;

fn state(raw: u32) -> StateId {
    StateId(raw)
}

fn observation(raw: u32) -> Observation {
    Observation(raw)
}

fn running_cursor(fuel: u32) -> Cursor {
    Cursor {
        state: state(STATE_ZERO),
        status: MachineStatus::Running,
        fuel: Fuel { remaining: fuel },
        next_observation: 0,
    }
}

fn linear_model() -> TransitionModel {
    TransitionModel {
        state_count: STATE_COUNT,
        transitions: vec![
            Transition {
                from: state(STATE_ZERO),
                observation: observation(OBSERVATION_FIRST),
                effect: StepEffect::Continue(state(STATE_ONE)),
            },
            Transition {
                from: state(STATE_ONE),
                observation: observation(OBSERVATION_SECOND),
                effect: StepEffect::Continue(state(STATE_TWO)),
            },
            Transition {
                from: state(STATE_TWO),
                observation: observation(OBSERVATION_THIRD),
                effect: StepEffect::Finish(state(STATE_THREE)),
            },
        ],
    }
}

fn pause_model() -> TransitionModel {
    TransitionModel {
        state_count: STATE_COUNT,
        transitions: vec![Transition {
            from: state(STATE_ZERO),
            observation: observation(OBSERVATION_FIRST),
            effect: StepEffect::Pause(state(STATE_ONE)),
        }],
    }
}

fn trap_model() -> TransitionModel {
    TransitionModel {
        state_count: STATE_COUNT,
        transitions: vec![Transition {
            from: state(STATE_ZERO),
            observation: observation(OBSERVATION_FIRST),
            effect: StepEffect::Trap(TrapCode(TRAP_CODE)),
        }],
    }
}

fn observations() -> Vec<Observation> {
    vec![
        observation(OBSERVATION_FIRST),
        observation(OBSERVATION_SECOND),
        observation(OBSERVATION_THIRD),
    ]
}

fn bindings() -> ResumeBindings {
    ResumeBindings {
        generation: GenerationId(GENERATION_ID),
        artifact: ArtifactId(ARTIFACT_ID),
        profile: ProfileId(PROFILE_ID),
        input: InputId(INPUT_ID),
        context: ContextId(CONTEXT_ID),
        observations: ObservationSetId(OBSERVATION_SET_ID),
    }
}

fn allocator(phase: AllocationPhase) -> PageAllocator {
    PageAllocator {
        phase,
        capacity_pages: PAGE_CAPACITY,
        allocated_pages: 0,
    }
}

fn exact_schedule() -> ChunkSchedule {
    ChunkSchedule {
        total_bytes: BYTE_TOTAL,
        chunks: vec![
            Chunk {
                offset: 0,
                length: BYTE_FIRST_LENGTH,
            },
            Chunk {
                offset: BYTE_FIRST_LENGTH,
                length: BYTE_SECOND_LENGTH,
            },
            Chunk {
                offset: BYTE_FIRST_LENGTH + BYTE_SECOND_LENGTH,
                length: BYTE_THIRD_LENGTH,
            },
        ],
    }
}
