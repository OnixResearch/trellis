use super::super::*;

const FULL_VISIT_COUNT: u32 = 4;
const REACHABLE_BLOCK_COUNT: u32 = 4;
const SECOND_ORDINAL: u32 = 1;

fn block_locations(order: &[u32]) -> Vec<Location> {
    order.iter().map(|block| Location::Block { block: *block }).collect()
}

fn block_entries(order: &[u32]) -> Vec<Entry> {
    order.iter().map(|block| super::entry(Location::Block { block: *block })).collect()
}

// r[verify mir-visitor.trace]
// r[verify mir-visitor.coverage]
// r[verify mir-visitor.docs]
// r[verify mir-visitor.final-validation]
#[test]
fn immutable_full_body_trace_is_admitted_exactly_once() {
    let body = super::return_body();
    let requested = vec![
        Location::Body,
        Location::Local {
            local: super::RETURN_LOCAL,
        },
        Location::Block {
            block: super::ENTRY_BLOCK,
        },
        Location::Terminator {
            block: super::ENTRY_BLOCK,
        },
    ];
    let trace = super::immutable_trace(
        requested.iter().copied().map(super::entry).collect(),
        Order::Declared,
        ReachabilityPolicy::AllBlocks,
    );
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Ok(TraceFacts {
            visited_count: FULL_VISIT_COUNT,
            requested_count: FULL_VISIT_COUNT,
            block_order: vec![super::ENTRY_BLOCK],
            shape: Shape {
                locals: 1,
                blocks: 1,
                statements: 0,
                terminators: 1,
                successors: 0,
            },
        })
    );
}

// r[verify mir-visitor.locations]
// r[verify mir-visitor.evidence]
#[test]
fn place_operand_and_rvalue_locations_are_admitted() {
    let body = super::branch_body(super::FIRST_BITS);
    let statement_owner = Owner::Statement {
        block: super::ENTRY_BLOCK,
        statement: 0,
    };
    let locations = [
        Location::Statement {
            block: super::ENTRY_BLOCK,
            statement: 0,
        },
        Location::Rvalue {
            block: super::ENTRY_BLOCK,
            statement: 0,
        },
        Location::Place {
            owner: statement_owner,
            ordinal: 0,
        },
        Location::Place {
            owner: statement_owner,
            ordinal: SECOND_ORDINAL,
        },
        Location::Operand {
            owner: statement_owner,
            ordinal: 0,
        },
        Location::Operand {
            owner: statement_owner,
            ordinal: SECOND_ORDINAL,
        },
        Location::Operand {
            owner: Owner::Terminator {
                block: super::ENTRY_BLOCK,
            },
            ordinal: 0,
        },
        Location::Successor {
            block: super::ENTRY_BLOCK,
            ordinal: SECOND_ORDINAL,
        },
    ];
    for location in locations {
        assert_eq!(admit_location(&body, &location), Ok(()));
    }
}

// r[verify mir-visitor.traversal-order]
// r[verify mir-visitor.evidence]
#[test]
fn preorder_is_deterministic_and_excludes_unreachable_blocks() {
    let body = super::branch_body(super::FIRST_BITS);
    assert_eq!(
        preorder_blocks(&body, super::ENTRY_BLOCK),
        Ok(vec![
            super::ENTRY_BLOCK,
            super::LEFT_BLOCK,
            super::EXIT_BLOCK,
            super::RIGHT_BLOCK,
        ])
    );
}

// r[verify mir-visitor.traversal-order]
// r[verify mir-visitor.evidence]
#[test]
fn reverse_postorder_is_deterministic_and_complete() {
    let body = super::branch_body(super::FIRST_BITS);
    assert_eq!(
        reverse_postorder_blocks(&body, super::ENTRY_BLOCK),
        Ok(vec![
            super::ENTRY_BLOCK,
            super::RIGHT_BLOCK,
            super::LEFT_BLOCK,
            super::EXIT_BLOCK,
        ])
    );
}

// r[verify mir-visitor.coverage]
// r[verify mir-visitor.traversal-order]
// r[verify mir-visitor.evidence]
#[test]
fn preorder_trace_covers_each_reachable_block_once() {
    let body = super::branch_body(super::FIRST_BITS);
    let order = vec![
        super::ENTRY_BLOCK,
        super::LEFT_BLOCK,
        super::EXIT_BLOCK,
        super::RIGHT_BLOCK,
    ];
    let requested = block_locations(&order);
    let trace = super::immutable_trace(block_entries(&order), Order::Preorder, ReachabilityPolicy::ReachableOnly);
    assert_eq!(
        admit_trace(&body, None, &requested, &trace),
        Ok(TraceFacts {
            visited_count: REACHABLE_BLOCK_COUNT,
            requested_count: REACHABLE_BLOCK_COUNT,
            block_order: order,
            shape: super::branch_shape(),
        })
    );
}

// r[verify mir-visitor.mut-shape]
// r[verify mir-visitor.evidence]
#[test]
fn value_only_mutation_preserves_declared_shape() {
    let before = super::branch_body(super::FIRST_BITS);
    let after = super::branch_body(super::SECOND_BITS);
    assert_eq!(admit_mutation(&before, &after, MutationClass::ValuesOnly), Ok(super::branch_shape()));

    let order = vec![
        super::ENTRY_BLOCK,
        super::LEFT_BLOCK,
        super::EXIT_BLOCK,
        super::RIGHT_BLOCK,
    ];
    let requested = block_locations(&order);
    let trace = Trace {
        entries: block_entries(&order),
        entry: super::ENTRY_BLOCK,
        coverage: CoverageMode::ExactlyOnce,
        order: Order::Preorder,
        reachability: ReachabilityPolicy::ReachableOnly,
        mutation: MutationClass::ValuesOnly,
    };
    assert_eq!(
        admit_trace(&before, Some(&after), &requested, &trace),
        Ok(TraceFacts {
            visited_count: REACHABLE_BLOCK_COUNT,
            requested_count: REACHABLE_BLOCK_COUNT,
            block_order: order,
            shape: super::branch_shape(),
        })
    );
}

// r[verify mir-visitor.non-claims]
#[test]
fn traversal_exports_locations_and_shape_not_pass_semantics() {
    let body = super::branch_body(super::FIRST_BITS);
    assert_eq!(body_shape(&body), Ok(super::branch_shape()));
}
