//! Finite transition facts and fail-closed model admission.

const MODEL_ITEM_LIMIT: u32 = 4_096;

/// Maximum number of states in one admitted model.
pub const MAX_STATES: u32 = MODEL_ITEM_LIMIT;
/// Maximum number of transition facts in one admitted model.
pub const MAX_TRANSITIONS: u32 = MODEL_ITEM_LIMIT;
/// Maximum number of steps in one bounded execution request.
pub const MAX_STEPS: u32 = MODEL_ITEM_LIMIT;
/// Maximum number of supplied host observations.
pub const MAX_OBSERVATIONS: u32 = MODEL_ITEM_LIMIT;

/// Finite runtime-state identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(pub u32);

/// Finite host-observation identity supplied by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Observation(pub u32);

/// Finite trap identity supplied by the model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrapCode(pub u32);

/// One deterministic transition result.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StepEffect {
    Continue(StateId),
    Finish(StateId),
    Trap(TrapCode),
    Pause(StateId),
}

/// One transition keyed by current state and host observation.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Transition {
    pub from: StateId,
    pub observation: Observation,
    pub effect: StepEffect,
}

/// Raw finite transition model.
// r[impl wasm-runtime-bounds.model]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionModel {
    pub state_count: u32,
    pub transitions: Vec<Transition>,
}

/// Evidence claim classes at this component boundary.
// r[impl wasm-runtime-bounds.non-claims]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimScope {
    AbstractLaws,
    ConcreteInterpreter,
    UnsafeAllocator,
    HostScheduling,
    SerializationOrMigration,
    Sandbox,
    Certification,
    AdapterCorrectness,
}

/// Rejection for a claim that exceeds the abstract proof boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimFault {
    UnsupportedPromotion,
}

/// Stable model-admission failure classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelFault {
    EmptyStateSpace,
    TooManyStates,
    TooManyTransitions,
    SourceOutsideModel,
    TargetOutsideModel,
    DuplicateTransitionKey,
}

/// Deterministic model-admission diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelFaultSite {
    pub class: ModelFault,
    pub index: Option<u32>,
}

fn transition_count(model: &TransitionModel) -> Option<u32> {
    u32::try_from(model.transitions.len()).ok()
}

fn target(effect: StepEffect) -> Option<StateId> {
    match effect {
        StepEffect::Continue(state) | StepEffect::Finish(state) | StepEffect::Pause(state) => Some(state),
        StepEffect::Trap(_) => None,
    }
}

fn site(class: ModelFault, index: Option<u32>) -> Result<(), ModelFaultSite> {
    Err(ModelFaultSite { class, index })
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

/// Admits only finite, in-range transition facts with unique keys.
// r[impl wasm-runtime-bounds.model]
pub fn admit_model(model: &TransitionModel) -> Result<(), ModelFaultSite> {
    if model.state_count == 0 {
        return site(ModelFault::EmptyStateSpace, None);
    }
    if model.state_count > MAX_STATES {
        return site(ModelFault::TooManyStates, None);
    }
    if transition_count(model).is_none_or(|count| count > MAX_TRANSITIONS) {
        return site(ModelFault::TooManyTransitions, None);
    }

    for (index, transition) in model.transitions.iter().enumerate() {
        if transition.from.0 >= model.state_count {
            return site(ModelFault::SourceOutsideModel, position(index));
        }
        if target(transition.effect).is_some_and(|state| state.0 >= model.state_count) {
            return site(ModelFault::TargetOutsideModel, position(index));
        }
        let is_duplicate = model.transitions[..index]
            .iter()
            .any(|prior| prior.from == transition.from && prior.observation == transition.observation);
        if is_duplicate {
            return site(ModelFault::DuplicateTransitionKey, position(index));
        }
    }
    Ok(())
}

/// Admits only claims over the finite abstract laws in this component.
// r[impl wasm-runtime-bounds.non-claims]
pub fn admit_claim(scope: ClaimScope) -> Result<(), ClaimFault> {
    if scope == ClaimScope::AbstractLaws {
        Ok(())
    } else {
        Err(ClaimFault::UnsupportedPromotion)
    }
}

pub(crate) fn find_transition(model: &TransitionModel, state: StateId, observation: Observation) -> Option<Transition> {
    model
        .transitions
        .iter()
        .copied()
        .find(|transition| transition.from == state && transition.observation == observation)
}
