use super::OPTIMIZED;
use super::UNOPTIMIZED;
use crate::mir_pass_pipeline::FailureClass;
use crate::mir_pass_pipeline::Kind;
use crate::mir_pass_pipeline::MAX_TRACE_STEPS;
use crate::mir_pass_pipeline::OptimizationLevel;
use crate::mir_pass_pipeline::Phase;
use crate::mir_pass_pipeline::Settings;
use crate::mir_pass_pipeline::StealPolicy;
use crate::mir_pass_pipeline::Step;
use crate::mir_pass_pipeline::ValidationKind;
use crate::mir_pass_pipeline::admit_trace;
use crate::mir_pass_pipeline::registered_descriptor;

const LEVEL_ONE: Settings = Settings {
    optimization: OptimizationLevel::O1,
};
const OPTIMIZATION_FAILURE_STEP: usize = 4;

// r[verify mir-pass.evidence]
// r[verify mir-pass.phase-order]
#[test]
fn phase_reversal_reports_phase_order() {
    let steps = [Step::Advance(Phase::ConstPrepared), Step::Advance(Phase::Built)];
    let error = admit_trace(UNOPTIMIZED, &steps).unwrap_err();
    assert_eq!(error.class, FailureClass::PhaseOrder);
    assert_eq!(error.step_index, 1);
}

// r[verify mir-pass.evidence]
#[test]
fn skipped_phase_reports_phase_order() {
    let error = admit_trace(UNOPTIMIZED, &[Step::Advance(Phase::Promoted)]).unwrap_err();
    assert_eq!(error.class, FailureClass::PhaseOrder);
    assert_eq!(error.step_index, 0);
}

// r[verify mir-pass.evidence]
#[test]
fn altered_registration_is_rejected() {
    let mut descriptor = registered_descriptor(Kind::SimplifyCfg);
    descriptor.output = Phase::Optimized;
    let error = admit_trace(UNOPTIMIZED, &[Step::RunPass(descriptor)]).unwrap_err();
    assert_eq!(error.class, FailureClass::PassCompatibility);
}

// r[verify mir-pass.pass-compatibility]
#[test]
fn registered_pass_at_wrong_phase_is_rejected() {
    let descriptor = registered_descriptor(Kind::PromoteTemps);
    let error = admit_trace(UNOPTIMIZED, &[Step::RunPass(descriptor)]).unwrap_err();
    assert_eq!(error.class, FailureClass::PassCompatibility);
}

// r[verify mir-pass.evidence]
#[test]
fn missing_validation_is_rejected() {
    let descriptor = registered_descriptor(Kind::PrepareConst);
    let error = admit_trace(UNOPTIMIZED, &[Step::RunPass(descriptor)]).unwrap_err();
    assert_eq!(error.class, FailureClass::MissingValidation);
}

// r[verify mir-pass.optimization-gating]
#[test]
fn disabled_optimization_pass_is_rejected() {
    let steps = [
        Step::Advance(Phase::ConstPrepared),
        Step::Advance(Phase::Promoted),
        Step::Advance(Phase::DropElaboratedConstChecked),
        Step::Validate(ValidationKind::Dialect),
        Step::RunPass(registered_descriptor(Kind::InstCombine)),
    ];
    let error = admit_trace(LEVEL_ONE, &steps).unwrap_err();
    assert_eq!(error.class, FailureClass::OptimizationGate);
    assert_eq!(error.step_index, OPTIMIZATION_FAILURE_STEP);
}

// r[verify mir-pass.artifact-state]
// r[verify mir-pass.steal-order]
#[test]
fn read_after_steal_is_rejected() {
    let steps = [Step::Steal(StealPolicy::Unconditional), Step::Read];
    let error = admit_trace(OPTIMIZED, &steps).unwrap_err();
    assert_eq!(error.class, FailureClass::StealOrder);
    assert_eq!(error.step_index, 1);
}

// r[verify mir-pass.evidence]
#[test]
fn double_steal_is_rejected() {
    let steps = [
        Step::Steal(StealPolicy::Unconditional),
        Step::Steal(StealPolicy::Unconditional),
    ];
    assert_eq!(admit_trace(UNOPTIMIZED, &steps).unwrap_err().class, FailureClass::StealOrder);
}

// r[verify mir-pass.evidence]
#[test]
fn required_read_must_precede_steal() {
    let steps = [Step::Borrow, Step::Steal(StealPolicy::PriorRead)];
    assert_eq!(admit_trace(UNOPTIMIZED, &steps).unwrap_err().class, FailureClass::StealOrder);
}

// r[verify mir-pass.failure-metadata]
#[test]
fn repeated_admission_reports_same_first_failure() {
    let steps = [
        Step::Advance(Phase::Promoted),
        Step::Steal(StealPolicy::Unconditional),
        Step::Read,
    ];
    let first = admit_trace(UNOPTIMIZED, &steps).unwrap_err();
    let second = admit_trace(UNOPTIMIZED, &steps).unwrap_err();
    assert_eq!(first, second);
    assert_eq!(first.class, FailureClass::PhaseOrder);
    assert_eq!(first.step_index, 0);
}

#[test]
fn excessive_trace_is_rejected_at_bound() {
    let steps = vec![Step::Read; MAX_TRACE_STEPS + 1];
    let error = admit_trace(UNOPTIMIZED, &steps).unwrap_err();
    assert_eq!(error.class, FailureClass::TraceLimit);
    assert_eq!(error.step_index, MAX_TRACE_STEPS);
}
