use super::super::*;

const MISSING_BLOCK: u32 = 9;
const MISSING_ORDINAL: u32 = 3;
const SECOND_ENTRY: u32 = 1;

fn exact_trace(entries: Vec<Entry>) -> Trace {
    super::immutable_trace(entries, Order::Declared, ReachabilityPolicy::AllBlocks)
}

// r[verify mir-visitor.locations]
// r[verify mir-visitor.evidence]
#[test]
fn invalid_block_location_is_rejected() {
    let body = super::return_body();
    let location = Location::Block { block: MISSING_BLOCK };
    assert_eq!(admit_location(&body, &location), Err(FailureClass::InvalidLocation));
}

// r[verify mir-visitor.locations]
// r[verify mir-visitor.evidence]
#[test]
fn invalid_nested_ordinal_is_rejected() {
    let body = super::branch_body(super::FIRST_BITS);
    let location = Location::Place {
        owner: Owner::Statement {
            block: super::ENTRY_BLOCK,
            statement: 0,
        },
        ordinal: MISSING_ORDINAL,
    };
    assert_eq!(admit_location(&body, &location), Err(FailureClass::InvalidLocation));
}

// r[verify mir-visitor.coverage]
// r[verify mir-visitor.admission]
// r[verify mir-visitor.evidence]
#[test]
fn duplicate_requested_location_is_rejected() {
    let body = super::return_body();
    let location = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let requested = vec![location, location];
    let trace = exact_trace(vec![super::entry(location)]);
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Err(Rejection {
            class: FailureClass::DuplicateRequest,
            entry_index: Some(SECOND_ENTRY),
            location: Some(location),
        })
    );
}

// r[verify mir-visitor.coverage]
// r[verify mir-visitor.evidence]
#[test]
fn duplicate_visit_is_rejected() {
    let body = super::return_body();
    let location = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let requested = vec![location];
    let trace = exact_trace(vec![super::entry(location), super::entry(location)]);
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Err(Rejection {
            class: FailureClass::DuplicateVisit,
            entry_index: Some(SECOND_ENTRY),
            location: Some(location),
        })
    );
}

// r[verify mir-visitor.coverage]
// r[verify mir-visitor.evidence]
#[test]
fn missing_visit_is_rejected() {
    let body = super::branch_body(super::FIRST_BITS);
    let entry = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let missing = Location::Block {
        block: super::LEFT_BLOCK,
    };
    let requested = vec![entry, missing];
    let trace = exact_trace(vec![super::entry(entry)]);
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Err(Rejection {
            class: FailureClass::MissingVisit,
            entry_index: None,
            location: Some(missing),
        })
    );
}

// r[verify mir-visitor.coverage]
// r[verify mir-visitor.evidence]
#[test]
fn visit_outside_requested_set_is_rejected() {
    let body = super::return_body();
    let requested_location = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let outside = Location::Local {
        local: super::RETURN_LOCAL,
    };
    let trace = exact_trace(vec![super::entry(outside)]);
    assert_eq!(
        admit_trace(&body, None, &[requested_location], &trace),
        Err(Rejection {
            class: FailureClass::OutsideRequest,
            entry_index: Some(super::ENTRY_BLOCK),
            location: Some(outside),
        })
    );
}

// r[verify mir-visitor.traversal-order]
// r[verify mir-visitor.evidence]
#[test]
fn unreachable_location_violates_reachable_only_policy() {
    let body = super::branch_body(super::FIRST_BITS);
    let unreachable = Location::Block {
        block: super::UNREACHABLE_BLOCK,
    };
    let trace =
        super::immutable_trace(vec![super::entry(unreachable)], Order::Declared, ReachabilityPolicy::ReachableOnly);
    assert_eq!(
        admit_trace(&body, None, &[unreachable], &trace),
        Err(Rejection {
            class: FailureClass::ReachabilityMismatch,
            entry_index: Some(super::ENTRY_BLOCK),
            location: Some(unreachable),
        })
    );
}

// r[verify mir-visitor.admission]
// r[verify mir-visitor.evidence]
#[test]
fn ordered_mode_rejects_all_blocks_policy() {
    let body = super::return_body();
    let location = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let trace = super::immutable_trace(vec![super::entry(location)], Order::Preorder, ReachabilityPolicy::AllBlocks);
    assert_eq!(
        admit_trace(&body, None, &[location], &trace),
        Err(Rejection {
            class: FailureClass::InvalidMode,
            entry_index: None,
            location: None,
        })
    );
}

// r[verify mir-visitor.traversal-order]
// r[verify mir-visitor.evidence]
#[test]
fn wrong_preorder_is_rejected() {
    let body = super::branch_body(super::FIRST_BITS);
    let order = [
        super::ENTRY_BLOCK,
        super::RIGHT_BLOCK,
        super::LEFT_BLOCK,
        super::EXIT_BLOCK,
    ];
    let requested: Vec<Location> = order.iter().map(|block| Location::Block { block: *block }).collect();
    let trace = super::immutable_trace(
        order.iter().map(|block| super::entry(Location::Block { block: *block })).collect(),
        Order::Preorder,
        ReachabilityPolicy::ReachableOnly,
    );
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Err(Rejection {
            class: FailureClass::OrderMismatch,
            entry_index: None,
            location: None,
        })
    );
}

// r[verify mir-visitor.mut-shape]
// r[verify mir-visitor.evidence]
#[test]
fn unsupported_structural_mutation_is_rejected() {
    let body = super::return_body();
    assert_eq!(
        admit_mutation(&body, &body, MutationClass::UnsupportedStructural),
        Err(Rejection {
            class: FailureClass::MutationMismatch,
            entry_index: None,
            location: None,
        })
    );
}

// r[verify mir-visitor.mut-shape]
// r[verify mir-visitor.evidence]
#[test]
fn value_only_mutation_cannot_change_body_shape() {
    let before = super::branch_body(super::FIRST_BITS);
    let after = super::return_body();
    assert_eq!(
        admit_mutation(&before, &after, MutationClass::ValuesOnly),
        Err(Rejection {
            class: FailureClass::MutationMismatch,
            entry_index: None,
            location: None,
        })
    );
}

// r[verify mir-visitor.mut-shape]
// r[verify mir-visitor.admission]
#[test]
fn immutable_trace_rejects_after_body() {
    let body = super::return_body();
    let location = Location::Block {
        block: super::ENTRY_BLOCK,
    };
    let trace = exact_trace(vec![super::entry(location)]);
    assert_eq!(
        admit_trace(&body, Some(&body), &[location], &trace),
        Err(Rejection {
            class: FailureClass::MutationMismatch,
            entry_index: None,
            location: None,
        })
    );
}

// r[verify mir-visitor.admission]
#[test]
fn repeated_malformed_trace_has_the_same_first_failure() {
    let body = super::return_body();
    let invalid = Location::Block { block: MISSING_BLOCK };
    let trace = Trace {
        entries: vec![super::entry(invalid)],
        entry: super::ENTRY_BLOCK,
        coverage: CoverageMode::ExactlyOnce,
        order: Order::Preorder,
        reachability: ReachabilityPolicy::AllBlocks,
        mutation: MutationClass::UnsupportedStructural,
    };
    let first = admit_trace(&body, None, &[invalid], &trace);
    let second = admit_trace(&body, None, &[invalid], &trace);
    assert_eq!(first, second);
    assert_eq!(
        first,
        Err(Rejection {
            class: FailureClass::InvalidMode,
            entry_index: None,
            location: None,
        })
    );
}
