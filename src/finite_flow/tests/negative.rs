use super::super::*;

fn admission_class(result: Result<AdmissionFacts, Rejection>) -> Option<FailureClass> {
    result.err().map(|error| error.class)
}

// r[verify mir-dataflow.domain]
// r[verify mir-dataflow.evidence]
#[test]
fn missing_bottom_top_and_height_are_rejected() {
    let mut missing_bottom = super::domain();
    missing_bottom.bottom = None;
    assert_eq!(admit_domain(&missing_bottom).err().map(|error| error.class), Some(FailureClass::MissingBottom));

    let mut missing_top = super::domain();
    missing_top.top = None;
    assert_eq!(admit_domain(&missing_top).err().map(|error| error.class), Some(FailureClass::MissingTop));

    let mut missing_height = super::domain();
    missing_height.height = None;
    assert_eq!(admit_domain(&missing_height).err().map(|error| error.class), Some(FailureClass::MissingHeight));
}

// r[verify mir-dataflow.lattice-laws]
// r[verify mir-dataflow.evidence]
#[test]
fn malformed_or_non_idempotent_join_is_rejected() {
    let mut malformed = super::domain();
    malformed.join = Vec::new();
    assert_eq!(admit_domain(&malformed).err().map(|error| error.class), Some(FailureClass::InvalidJoinTable));

    let mut unlawful = super::domain();
    unlawful.join = vec![super::state(super::BOTTOM); super::JOIN_TABLE_LEN];
    assert_eq!(admit_domain(&unlawful).err().map(|error| error.class), Some(FailureClass::LatticeLaw));
}

// r[verify mir-dataflow.admission]
// r[verify mir-dataflow.evidence]
#[test]
fn non_monotone_transfer_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    for effect in &mut analysis.effects {
        if effect.location == super::statement_location() {
            effect.primary = super::transfer(&[super::TOP, super::BOTTOM, super::TOP]);
        }
    }
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::NonMonotoneTransfer));
}

// r[verify mir-dataflow.effects]
// r[verify mir-dataflow.evidence]
#[test]
fn transfer_output_outside_domain_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    for effect in &mut analysis.effects {
        if effect.location == super::statement_location() {
            effect.primary = super::transfer(&[super::BOTTOM, super::MIDDLE, super::INVALID_INDEX]);
        }
    }
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::InvalidTransfer));
}

// r[verify mir-dataflow.per-edge]
// r[verify mir-dataflow.evidence]
#[test]
fn edge_effect_must_name_an_admitted_successor() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.edge_effects = vec![EdgeEffect {
        source: super::ENTRY_BLOCK,
        target: super::EXIT_BLOCK,
        transfer: super::identity_transfer(),
    }];
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::InvalidEdgeEffect));
}

// r[verify mir-dataflow.per-edge]
#[test]
fn duplicate_edge_effect_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.edge_effects.push(EdgeEffect {
        source: super::MIDDLE_BLOCK,
        target: super::EXIT_BLOCK,
        transfer: super::identity_transfer(),
    });
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::DuplicateEdgeEffect));
}

// r[verify mir-dataflow.effects]
// r[verify mir-dataflow.evidence]
#[test]
fn effect_location_must_be_statement_or_terminator() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.effects = vec![LocationEffect {
        location: crate::mir_visitor_traversal::Location::Rvalue {
            block: super::ENTRY_BLOCK,
            statement: 0,
        },
        before: super::identity_transfer(),
        primary: super::identity_transfer(),
    }];
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::InvalidLocation));
}

// r[verify mir-dataflow.admission]
#[test]
fn duplicate_location_effect_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.effects.push(LocationEffect {
        location: super::statement_location(),
        before: super::identity_transfer(),
        primary: super::identity_transfer(),
    });
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::DuplicateEffect));
}

// r[verify mir-dataflow.convergence]
// r[verify mir-dataflow.evidence]
#[test]
fn insufficient_budget_is_rejected_before_iteration() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.convergence_budget = 0;
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::BudgetTooSmall));
}

// r[verify mir-dataflow.convergence]
#[test]
fn excessive_budget_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.convergence_budget = super::TOO_LARGE_BUDGET;
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::BudgetTooLarge));
}

// r[verify mir-dataflow.admission]
#[test]
fn invalid_boundary_is_rejected() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.boundary_block = super::INVALID_INDEX;
    assert_eq!(admission_class(admit_analysis(&super::body(), &analysis)), Some(FailureClass::InvalidBoundary));
}

// r[verify mir-dataflow.results]
// r[verify mir-dataflow.evidence]
#[test]
fn invalid_cursor_point_is_rejected() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward)).and_then(|results| {
        inspect_state(&results, Point::BlockEntry {
            block: super::INVALID_INDEX,
        })
    });
    assert_eq!(outcome.err().map(|error| error.class), Some(FailureClass::InvalidResultPoint));
}

// r[verify mir-dataflow.results]
#[test]
fn duplicate_result_request_is_rejected() {
    let requested = [super::statement_location(), super::statement_location()];
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward))
        .and_then(|results| visit_locations(&results, &requested));
    assert_eq!(outcome.err().map(|error| error.class), Some(FailureClass::DuplicateRequest));
}

// r[verify mir-dataflow.admission]
#[test]
fn first_failure_is_deterministic() {
    let mut analysis = super::analysis(Direction::Forward);
    analysis.boundary_block = super::INVALID_INDEX;
    analysis.convergence_budget = 0;
    let first = admit_analysis(&super::body(), &analysis);
    let second = admit_analysis(&super::body(), &analysis);
    assert_eq!(first, second);
    assert_eq!(admission_class(first), Some(FailureClass::InvalidBoundary));
}
