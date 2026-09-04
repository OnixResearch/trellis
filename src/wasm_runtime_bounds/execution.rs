//! Exact fuel accounting and deterministic bounded execution.

/// Remaining execution fuel.
// r[impl wasm-runtime-bounds.fuel]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fuel {
    pub remaining: u32,
}

/// Persistent runtime status at a bounded execution boundary.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineStatus {
    Running,
    Finished,
    Trapped(super::machine::TrapCode),
    HostPaused,
}

/// Complete resumable position in the finite model and observation stream.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub state: super::machine::StateId,
    pub status: MachineStatus,
    pub fuel: Fuel,
    pub next_observation: u32,
}

/// Closed result classes for one bounded run.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunClass {
    Running,
    Finished,
    Trapped(super::machine::TrapCode),
    HostPaused,
    OutOfFuel,
}

/// One normalized bounded-run result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub cursor: Cursor,
    pub consumed_steps: u32,
    pub class: RunClass,
}

/// Explicit fuel-consumption failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuelFault {
    pub available: Fuel,
    pub requested: u32,
}

/// Stable execution-admission and transition failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunFault {
    InvalidModel(super::machine::ModelFaultSite),
    StateOutsideModel,
    TooManyObservations,
    ObservationCursorOutside,
    BudgetTooLarge,
    ObservationExhausted { at_step: u32 },
    MissingTransition { at_step: u32 },
    FuelInvariant,
}

/// Segmentation comparison failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentationFault {
    ModelDrift,
    ObservationDrift,
    CombinedBudgetOverflow,
    Execution(RunFault),
    ResultMismatch,
}

/// Complete inputs for one segmented-versus-uninterrupted comparison.
#[derive(Clone, Copy, Debug)]
pub struct SegmentRequest<'a> {
    pub model: &'a super::machine::TransitionModel,
    pub resume_model: &'a super::machine::TransitionModel,
    pub start: Cursor,
    pub observations: &'a [super::machine::Observation],
    pub resume_observations: &'a [super::machine::Observation],
    pub prefix_budget: u32,
    pub suffix_budget: u32,
}

/// Consumes the exact requested fuel or returns the original fuel unchanged.
// r[impl wasm-runtime-bounds.fuel]
pub fn consume_fuel(fuel: Fuel, requested: u32) -> Result<Fuel, FuelFault> {
    match fuel.remaining.checked_sub(requested) {
        Some(remaining) => Ok(Fuel { remaining }),
        None => Err(FuelFault {
            available: fuel,
            requested,
        }),
    }
}

fn observation_count(observations: &[super::machine::Observation]) -> Result<u32, RunFault> {
    let count = u32::try_from(observations.len()).map_err(|_| RunFault::TooManyObservations)?;
    if count > super::machine::MAX_OBSERVATIONS {
        return Err(RunFault::TooManyObservations);
    }
    Ok(count)
}

fn absorbed(cursor: Cursor) -> Option<RunResult> {
    let class = match cursor.status {
        MachineStatus::Running => return None,
        MachineStatus::Finished => RunClass::Finished,
        MachineStatus::Trapped(code) => RunClass::Trapped(code),
        MachineStatus::HostPaused => RunClass::HostPaused,
    };
    Some(RunResult {
        cursor,
        consumed_steps: 0,
        class,
    })
}

fn apply_effect(cursor: &mut Cursor, effect: super::machine::StepEffect) -> RunClass {
    match effect {
        super::machine::StepEffect::Continue(state) => {
            cursor.state = state;
            cursor.status = MachineStatus::Running;
            RunClass::Running
        }
        super::machine::StepEffect::Finish(state) => {
            cursor.state = state;
            cursor.status = MachineStatus::Finished;
            RunClass::Finished
        }
        super::machine::StepEffect::Trap(code) => {
            cursor.status = MachineStatus::Trapped(code);
            RunClass::Trapped(code)
        }
        super::machine::StepEffect::Pause(state) => {
            cursor.state = state;
            cursor.status = MachineStatus::HostPaused;
            RunClass::HostPaused
        }
    }
}

/// Executes at most `budget` deterministic transitions.
// r[impl wasm-runtime-bounds.fuel]
// r[impl wasm-runtime-bounds.segmentation]
pub fn run_bounded(
    model: &super::machine::TransitionModel,
    start: Cursor,
    observations: &[super::machine::Observation],
    budget: u32,
) -> Result<RunResult, RunFault> {
    super::machine::admit_model(model).map_err(RunFault::InvalidModel)?;
    if start.state.0 >= model.state_count {
        return Err(RunFault::StateOutsideModel);
    }
    if budget > super::machine::MAX_STEPS {
        return Err(RunFault::BudgetTooLarge);
    }
    let observation_len = observation_count(observations)?;
    if start.next_observation > observation_len {
        return Err(RunFault::ObservationCursorOutside);
    }
    if let Some(result) = absorbed(start) {
        return Ok(result);
    }
    if budget == 0 {
        return Ok(RunResult {
            cursor: start,
            consumed_steps: 0,
            class: RunClass::OutOfFuel,
        });
    }

    let mut cursor = start;
    let mut consumed_steps = 0;
    while consumed_steps < budget {
        if cursor.fuel.remaining == 0 {
            return Ok(RunResult {
                cursor,
                consumed_steps,
                class: RunClass::OutOfFuel,
            });
        }
        if cursor.next_observation >= observation_len {
            return Err(RunFault::ObservationExhausted {
                at_step: consumed_steps,
            });
        }
        let observation_index =
            usize::try_from(cursor.next_observation).map_err(|_| RunFault::ObservationCursorOutside)?;
        let observation = observations[observation_index];
        let transition =
            super::machine::find_transition(model, cursor.state, observation).ok_or(RunFault::MissingTransition {
                at_step: consumed_steps,
            })?;
        cursor.fuel = consume_fuel(cursor.fuel, 1).map_err(|_| RunFault::FuelInvariant)?;
        cursor.next_observation += 1;
        consumed_steps += 1;
        let class = apply_effect(&mut cursor, transition.effect);
        if class != RunClass::Running {
            return Ok(RunResult {
                cursor,
                consumed_steps,
                class,
            });
        }
    }
    Ok(RunResult {
        cursor,
        consumed_steps,
        class: RunClass::OutOfFuel,
    })
}

fn compose_result(first: RunResult, second: RunResult) -> Option<RunResult> {
    Some(RunResult {
        cursor: second.cursor,
        consumed_steps: first.consumed_steps.checked_add(second.consumed_steps)?,
        class: second.class,
    })
}

/// Checks segmented execution against one uninterrupted run.
// r[impl wasm-runtime-bounds.segmentation]
pub fn compare_segments(request: SegmentRequest<'_>) -> Result<RunResult, SegmentationFault> {
    if request.model != request.resume_model {
        return Err(SegmentationFault::ModelDrift);
    }
    if request.observations != request.resume_observations {
        return Err(SegmentationFault::ObservationDrift);
    }
    let combined_budget = request
        .prefix_budget
        .checked_add(request.suffix_budget)
        .filter(|combined| *combined <= super::machine::MAX_STEPS)
        .ok_or(SegmentationFault::CombinedBudgetOverflow)?;
    let uninterrupted = run_bounded(request.model, request.start, request.observations, combined_budget)
        .map_err(SegmentationFault::Execution)?;
    let prefix = run_bounded(request.model, request.start, request.observations, request.prefix_budget)
        .map_err(SegmentationFault::Execution)?;
    let segmented = if prefix.class == RunClass::OutOfFuel {
        let suffix = run_bounded(request.model, prefix.cursor, request.observations, request.suffix_budget)
            .map_err(SegmentationFault::Execution)?;
        compose_result(prefix, suffix).ok_or(SegmentationFault::CombinedBudgetOverflow)?
    } else {
        prefix
    };
    if segmented != uninterrupted {
        return Err(SegmentationFault::ResultMismatch);
    }
    Ok(uninterrupted)
}
