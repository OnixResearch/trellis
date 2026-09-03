//! Pure admission and transition logic for finite MIR pipeline traces.

/// Maximum trace length accepted by the bounded admission helper.
pub const MAX_TRACE_STEPS: usize = 128;

/// Returns the finite total-order rank for one MIR phase.
// r[impl mir-pass.phase-order]
#[must_use]
pub const fn phase_rank(phase: super::model::Phase) -> u8 {
    phase as u8
}

const fn optimization_rank(level: super::model::OptimizationLevel) -> u8 {
    level as u8
}

const fn validation_rank(kind: super::model::ValidationKind) -> u8 {
    kind as u8
}

const fn artifact_state_tag(state: super::model::ArtifactState) -> u8 {
    state as u8
}

fn run_pass(
    state: super::model::State,
    settings: super::model::Settings,
    descriptor: super::model::Descriptor,
) -> Result<super::model::State, super::model::FailureClass> {
    if state.artifact != super::model::ArtifactState::Available {
        return Err(super::model::FailureClass::StealOrder);
    }
    if descriptor != super::registration::registered_descriptor(descriptor.kind) {
        return Err(super::model::FailureClass::PassCompatibility);
    }
    if !super::proofs::pass_phase_compatible(
        phase_rank(state.phase),
        phase_rank(descriptor.input),
        phase_rank(descriptor.output),
    ) {
        return Err(super::model::FailureClass::PassCompatibility);
    }
    if !super::proofs::optimization_admits(
        optimization_rank(settings.optimization),
        optimization_rank(descriptor.minimum_optimization),
    ) {
        return Err(super::model::FailureClass::OptimizationGate);
    }
    if !super::proofs::validation_admits(
        validation_rank(state.validation),
        validation_rank(descriptor.required_validation),
    ) {
        return Err(super::model::FailureClass::MissingValidation);
    }

    let is_same_phase = descriptor.input == descriptor.output;
    let validation = if is_same_phase && descriptor.structure == super::model::StructureEffect::Preserved {
        state.validation
    } else {
        super::model::ValidationKind::Missing
    };
    Ok(super::model::State {
        phase: descriptor.output,
        artifact: super::model::ArtifactState::Available,
        validation,
        borrow_read_recorded: false,
    })
}

/// Applies one event without performing external effects.
// r[impl mir-pass.admission]
// r[impl mir-pass.steal-order]
pub fn apply_step(
    state: super::model::State,
    settings: super::model::Settings,
    step: super::model::Step,
) -> Result<super::model::State, super::model::FailureClass> {
    match step {
        super::model::Step::Validate(kind) => {
            if !super::proofs::artifact_readable(artifact_state_tag(state.artifact)) {
                return Err(super::model::FailureClass::StealOrder);
            }
            if kind == super::model::ValidationKind::Missing {
                return Err(super::model::FailureClass::MissingValidation);
            }
            Ok(super::model::State {
                validation: kind,
                ..state
            })
        }
        super::model::Step::RunPass(descriptor) => run_pass(state, settings, descriptor),
        super::model::Step::Read => {
            if !super::proofs::artifact_readable(artifact_state_tag(state.artifact)) {
                return Err(super::model::FailureClass::StealOrder);
            }
            Ok(super::model::State {
                borrow_read_recorded: state.artifact == super::model::ArtifactState::Borrowed
                    || state.borrow_read_recorded,
                ..state
            })
        }
        super::model::Step::Borrow => {
            if state.artifact != super::model::ArtifactState::Available {
                return Err(super::model::FailureClass::StealOrder);
            }
            Ok(super::model::State {
                artifact: super::model::ArtifactState::Borrowed,
                borrow_read_recorded: false,
                ..state
            })
        }
        super::model::Step::Steal(policy) => {
            if state.artifact == super::model::ArtifactState::Stolen {
                return Err(super::model::FailureClass::StealOrder);
            }
            if policy == super::model::StealPolicy::PriorRead
                && (state.artifact != super::model::ArtifactState::Borrowed || !state.borrow_read_recorded)
            {
                return Err(super::model::FailureClass::StealOrder);
            }
            Ok(super::model::State {
                artifact: super::model::ArtifactState::Stolen,
                borrow_read_recorded: false,
                ..state
            })
        }
        super::model::Step::Advance(target) => {
            if state.artifact == super::model::ArtifactState::Borrowed {
                return Err(super::model::FailureClass::StealOrder);
            }
            if !super::proofs::phase_step_allowed(phase_rank(state.phase), phase_rank(target)) {
                return Err(super::model::FailureClass::PhaseOrder);
            }
            Ok(super::model::State {
                phase: target,
                artifact: super::model::ArtifactState::Available,
                validation: super::model::ValidationKind::Missing,
                borrow_read_recorded: false,
            })
        }
    }
}

/// Admits a bounded trace and reports its deterministic first failure.
// r[impl mir-pass.admission]
// r[impl mir-pass.failure-metadata]
pub fn admit_trace(
    settings: super::model::Settings,
    steps: &[super::model::Step],
) -> Result<super::model::State, super::model::Rejection> {
    if steps.len() > MAX_TRACE_STEPS {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::TraceLimit,
            step_index: MAX_TRACE_STEPS,
        });
    }

    let mut state = super::model::State::initial();
    for (step_index, step) in steps.iter().copied().enumerate() {
        state = apply_step(state, settings, step).map_err(|class| super::model::Rejection { class, step_index })?;
    }
    Ok(state)
}
