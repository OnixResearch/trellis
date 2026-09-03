use super::OPTIMIZED;
use super::UNOPTIMIZED;
use crate::mir_pass_pipeline::ArtifactState;
use crate::mir_pass_pipeline::Kind;
use crate::mir_pass_pipeline::Phase;
use crate::mir_pass_pipeline::StealPolicy;
use crate::mir_pass_pipeline::Step;
use crate::mir_pass_pipeline::ValidationKind;
use crate::mir_pass_pipeline::admit_trace;
use crate::mir_pass_pipeline::registered_descriptor;

// r[verify mir-pass.evidence]
// r[verify mir-pass.phase-order]
#[test]
fn ordered_query_phases_are_admitted() {
    let steps = [
        Step::Advance(Phase::ConstPrepared),
        Step::Advance(Phase::Promoted),
        Step::Advance(Phase::DropElaboratedConstChecked),
        Step::Advance(Phase::Optimized),
    ];

    let final_state = admit_trace(UNOPTIMIZED, &steps).unwrap();
    assert_eq!(final_state.phase, Phase::Optimized);
    assert_eq!(final_state.artifact, ArtifactState::Available);
}

// r[verify mir-pass.evidence]
// r[verify mir-pass.pass-compatibility]
#[test]
fn registered_same_phase_mutation_is_admitted() {
    let simplify = registered_descriptor(Kind::SimplifyCfg);
    let state = admit_trace(UNOPTIMIZED, &[Step::RunPass(simplify)]).unwrap();
    assert_eq!(state.phase, Phase::Built);
}

// r[verify mir-pass.evidence]
// r[verify mir-pass.pass-compatibility]
// r[verify mir-pass.admission]
#[test]
fn validated_pipeline_reaches_optimized_mir() {
    let steps = [
        Step::Validate(ValidationKind::Structural),
        Step::RunPass(registered_descriptor(Kind::RemoveStorageMarkers)),
        Step::Validate(ValidationKind::Structural),
        Step::RunPass(registered_descriptor(Kind::PrepareConst)),
        Step::Validate(ValidationKind::Dialect),
        Step::RunPass(registered_descriptor(Kind::PromoteTemps)),
        Step::Validate(ValidationKind::Dialect),
        Step::RunPass(registered_descriptor(Kind::ElaborateDrops)),
        Step::Validate(ValidationKind::Dialect),
        Step::RunPass(registered_descriptor(Kind::InstCombine)),
    ];

    let state = admit_trace(OPTIMIZED, &steps).unwrap();
    assert_eq!(state.phase, Phase::Optimized);
}

// r[verify mir-pass.evidence]
// r[verify mir-pass.optimization-gating]
#[test]
fn enabled_level_two_admits_optimization_pass() {
    let steps = [
        Step::Advance(Phase::ConstPrepared),
        Step::Advance(Phase::Promoted),
        Step::Advance(Phase::DropElaboratedConstChecked),
        Step::Validate(ValidationKind::Dialect),
        Step::RunPass(registered_descriptor(Kind::InstCombine)),
    ];

    assert_eq!(admit_trace(OPTIMIZED, &steps).unwrap().phase, Phase::Optimized);
}

// r[verify mir-pass.evidence]
// r[verify mir-pass.steal-order]
#[test]
fn dependent_read_before_steal_is_admitted() {
    let steps = [
        Step::Borrow,
        Step::Read,
        Step::Steal(StealPolicy::PriorRead),
        Step::Advance(Phase::ConstPrepared),
    ];

    let state = admit_trace(UNOPTIMIZED, &steps).unwrap();
    assert_eq!(state.phase, Phase::ConstPrepared);
    assert_eq!(state.artifact, ArtifactState::Available);
}

// r[verify mir-pass.artifact-state]
// r[verify mir-pass.non-claims]
#[test]
fn available_read_preserves_artifact_state() {
    let state = admit_trace(UNOPTIMIZED, &[Step::Read]).unwrap();
    assert_eq!(state.artifact, ArtifactState::Available);
}
