use super::super::*;

const MISSING_INDEX: u32 = 4;
const SECOND_LOCAL_COUNT: u32 = 2;

fn failure(class: FailureClass, subject_index: Option<u32>, projection_index: Option<u32>) -> Failure {
    Failure {
        class,
        subject_index,
        projection_index,
    }
}

// r[verify mir-core.body-shape]
// r[verify mir-core.evidence]
#[test]
fn empty_block_table_is_rejected() {
    let body = Body {
        locals: vec![super::return_local()],
        blocks: Vec::new(),
    };
    assert_eq!(
        admit_body(&body),
        Err(Rejection {
            class: FailureClass::EmptyBlockTable,
            block_index: None,
            statement_index: None,
            subject_index: None,
            projection_index: None,
        })
    );
}

// r[verify mir-core.return-place]
// r[verify mir-core.evidence]
#[test]
fn missing_return_place_is_rejected() {
    let body = Body {
        locals: vec![super::local(LocalRole::Argument, TypeTag::U64)],
        blocks: vec![super::return_block()],
    };
    assert_eq!(
        admit_body(&body),
        Err(Rejection {
            class: FailureClass::MissingReturnPlace,
            block_index: None,
            statement_index: None,
            subject_index: Some(RETURN_PLACE),
            projection_index: None,
        })
    );
}

// r[verify mir-core.block-terminator]
// r[verify mir-core.evidence]
#[test]
fn missing_terminator_is_rejected_before_statements() {
    let body = Body {
        locals: vec![super::return_local()],
        blocks: vec![BasicBlock {
            statements: vec![Statement::StorageLive { local: MISSING_INDEX }],
            terminator: None,
        }],
    };
    assert_eq!(
        admit_body(&body),
        Err(Rejection {
            class: FailureClass::MissingTerminator,
            block_index: Some(RETURN_PLACE),
            statement_index: None,
            subject_index: None,
            projection_index: None,
        })
    );
}

// r[verify mir-core.local-reference]
// r[verify mir-core.evidence]
#[test]
fn invalid_statement_local_has_stable_location() {
    let body = Body {
        locals: vec![super::return_local()],
        blocks: vec![BasicBlock {
            statements: vec![Statement::StorageDead { local: MISSING_INDEX }],
            terminator: Some(Terminator::Return),
        }],
    };
    assert_eq!(
        admit_body(&body),
        Err(Rejection {
            class: FailureClass::InvalidLocal,
            block_index: Some(RETURN_PLACE),
            statement_index: Some(RETURN_PLACE),
            subject_index: Some(MISSING_INDEX),
            projection_index: None,
        })
    );
}

// r[verify mir-core.local-reference]
// r[verify mir-core.place-projection]
// r[verify mir-core.evidence]
#[test]
fn dangling_base_and_projection_index_are_rejected() {
    let dangling_base = Place {
        local: MISSING_INDEX,
        projections: Vec::new(),
    };
    assert_eq!(
        admit_place(&dangling_base, SECOND_LOCAL_COUNT),
        Err(failure(FailureClass::InvalidLocal, Some(MISSING_INDEX), None,))
    );

    let dangling_index = Place {
        local: RETURN_PLACE,
        projections: vec![Projection::Index { local: MISSING_INDEX }],
    };
    assert_eq!(
        admit_place(&dangling_index, SECOND_LOCAL_COUNT),
        Err(failure(FailureClass::InvalidLocal, Some(MISSING_INDEX), Some(RETURN_PLACE),))
    );
}

// r[verify mir-core.place-projection]
// r[verify mir-core.evidence]
#[test]
fn unsupported_projection_is_rejected() {
    let place = Place {
        local: RETURN_PLACE,
        projections: vec![Projection::Unsupported],
    };
    assert_eq!(
        admit_place(&place, SECOND_LOCAL_COUNT),
        Err(failure(FailureClass::UnsupportedProjection, None, Some(RETURN_PLACE),))
    );
}

// r[verify mir-core.rvalue-nonnesting]
// r[verify mir-core.evidence]
#[test]
fn nested_rvalue_is_rejected() {
    assert_eq!(
        admit_rvalue(&Rvalue::Nested, SECOND_LOCAL_COUNT),
        Err(failure(FailureClass::NestedRvalue, None, None))
    );
}

// r[verify mir-core.successors]
// r[verify mir-core.evidence]
#[test]
fn out_of_range_successor_is_rejected() {
    let terminator = Terminator::Goto { target: MISSING_INDEX };
    assert_eq!(
        admit_terminator(&terminator, SECOND_LOCAL_COUNT, SECOND_LOCAL_COUNT),
        Err(failure(FailureClass::OutOfRangeSuccessor, Some(MISSING_INDEX), None,))
    );
}

// r[verify mir-core.failure-metadata]
// r[verify mir-core.evidence]
#[test]
fn repeated_admission_reports_the_same_first_failure() {
    let body = Body {
        locals: vec![super::return_local()],
        blocks: vec![BasicBlock {
            statements: vec![Statement::StorageLive { local: MISSING_INDEX }],
            terminator: Some(Terminator::Goto { target: MISSING_INDEX }),
        }],
    };
    let first = admit_body(&body);
    let second = admit_body(&body);
    assert_eq!(first, second);
    assert_eq!(
        first,
        Err(Rejection {
            class: FailureClass::InvalidLocal,
            block_index: Some(RETURN_PLACE),
            statement_index: Some(RETURN_PLACE),
            subject_index: Some(MISSING_INDEX),
            projection_index: None,
        })
    );
}

// r[verify mir-core.downstream-adapter]
// r[verify mir-core.failure-metadata]
#[test]
fn adapter_fails_before_exposing_invalid_edges() {
    let body = Body {
        locals: vec![super::return_local()],
        blocks: vec![BasicBlock {
            statements: Vec::new(),
            terminator: Some(Terminator::Goto { target: MISSING_INDEX }),
        }],
    };
    assert_eq!(
        extract_adapter_facts(&body),
        Err(Rejection {
            class: FailureClass::OutOfRangeSuccessor,
            block_index: Some(RETURN_PLACE),
            statement_index: None,
            subject_index: Some(MISSING_INDEX),
            projection_index: None,
        })
    );
}
