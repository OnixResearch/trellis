use super::super::*;

// r[verify mir-dataflow.domain]
// r[verify mir-dataflow.lattice-laws]
// r[verify mir-dataflow.evidence]
#[test]
fn finite_chain_domain_is_admitted() {
    assert_eq!(admit_domain(&super::domain()), Ok(()));
    assert_eq!(
        join_state(&super::domain(), super::state(super::MIDDLE), super::state(super::TOP)),
        Ok(super::state(super::TOP))
    );
}

// r[verify mir-dataflow.admission]
// r[verify mir-dataflow.convergence]
// r[verify mir-dataflow.docs]
// r[verify mir-dataflow.final-validation]
#[test]
fn complete_analysis_is_admitted_with_derived_budget() {
    assert_eq!(
        admit_analysis(&super::body(), &super::analysis(Direction::Forward)),
        Ok(AdmissionFacts {
            block_count: super::BLOCK_COUNT,
            required_budget: super::REQUIRED_BUDGET,
            effect_count: super::EFFECT_COUNT,
            edge_effect_count: super::EDGE_EFFECT_COUNT,
        })
    );
}

// r[verify mir-dataflow.direction]
// r[verify mir-dataflow.convergence]
// r[verify mir-dataflow.evidence]
#[test]
fn forward_analysis_reaches_stable_block_states() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward));
    assert_eq!(
        outcome.map(|results| (results.block_entries, results.block_exits)),
        Ok((
            vec![
                super::state(super::BOTTOM),
                super::state(super::TOP),
                super::state(super::TOP),
            ],
            vec![
                super::state(super::TOP),
                super::state(super::TOP),
                super::state(super::TOP),
            ],
        ))
    );
}

// r[verify mir-dataflow.direction]
// r[verify mir-dataflow.effects]
#[test]
fn backward_direction_reverses_location_order_only() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Backward));
    assert_eq!(
        outcome.map(|results| results.block_exits),
        Ok(vec![
            super::state(super::MIDDLE),
            super::state(super::BOTTOM),
            super::state(super::BOTTOM),
        ])
    );
}

// r[verify mir-dataflow.effects]
// r[verify mir-dataflow.results]
// r[verify mir-dataflow.evidence]
#[test]
fn before_effect_is_applied_immediately_before_primary() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward)).and_then(|results| {
        inspect_state(&results, Point::Before {
            location: super::terminator_location(),
        })
        .and_then(|before| {
            inspect_state(&results, Point::Primary {
                location: super::terminator_location(),
            })
            .map(|primary| (before, primary))
        })
    });
    assert_eq!(outcome, Ok((super::state(super::MIDDLE), super::state(super::TOP),)));
}

// r[verify mir-dataflow.per-edge]
// r[verify mir-dataflow.results]
#[test]
fn edge_state_is_available_for_admitted_successor() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward)).and_then(|results| {
        inspect_state(&results, Point::Edge {
            source: super::MIDDLE_BLOCK,
            target: super::EXIT_BLOCK,
        })
    });
    assert_eq!(outcome, Ok(super::state(super::TOP)));
}

// r[verify mir-dataflow.results]
// r[verify mir-dataflow.evidence]
#[test]
fn result_visitor_preserves_requested_location_order() {
    let requested = [super::terminator_location(), super::statement_location()];
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward))
        .and_then(|results| visit_locations(&results, &requested));
    assert_eq!(
        outcome.map(|states| { states.into_iter().map(|state| state.location).collect::<Vec<_>>() }),
        Ok(requested.to_vec())
    );
}

// r[verify mir-dataflow.non-claims]
#[test]
fn fixed_results_export_finite_states_not_analysis_semantics() {
    let outcome = run_to_fixed_point(&super::body(), &super::analysis(Direction::Forward));
    assert!(matches!(outcome, Ok(Results { .. })));
}
