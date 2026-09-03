use super::super::*;

const SECOND_BLOCK: u32 = 1;
const THIRD_BLOCK: u32 = 2;
const FINAL_BLOCK: u32 = 3;
const BLOCK_COUNT: u32 = 4;
const SECOND_LOCAL: u32 = 1;
const LOCAL_COUNT: u32 = 2;
const FIELD_INDEX: u32 = 2;
const TEST_BITS: u64 = 7;

// r[verify mir-core.body-shape]
// r[verify mir-core.return-place]
// r[verify mir-core.block-terminator]
// r[verify mir-core.docs]
// r[verify mir-core.final-validation]
#[test]
fn single_block_return_body_is_admitted() {
    let body = super::return_body();
    assert_eq!(admit_body(&body), Ok(()));
}

// r[verify mir-core.local-reference]
// r[verify mir-core.place-projection]
// r[verify mir-core.rvalue-nonnesting]
// r[verify mir-core.evidence]
#[test]
fn places_operands_rvalues_and_statements_are_admitted() {
    let local_count = LOCAL_COUNT;
    let projected = Place {
        local: SECOND_LOCAL,
        projections: vec![
            Projection::Dereference,
            Projection::Field { field: FIELD_INDEX },
            Projection::Index { local: SECOND_LOCAL },
        ],
    };
    assert_eq!(admit_place(&projected, local_count), Ok(()));

    let moved = Operand::Move(projected.clone());
    assert_eq!(admit_operand(&moved, local_count), Ok(()));

    let value = Rvalue::Binary {
        operator: BinaryOperator::Add,
        left: moved,
        right: super::constant(TEST_BITS),
    };
    assert_eq!(admit_rvalue(&value, local_count), Ok(()));

    let statement = Statement::Assign {
        destination: super::place(RETURN_PLACE),
        value,
    };
    assert_eq!(admit_statement(&statement, local_count), Ok(()));
    assert_eq!(admit_statement(&Statement::StorageLive { local: SECOND_LOCAL }, local_count,), Ok(()));
}

// r[verify mir-core.successors]
// r[verify mir-core.downstream-adapter]
// r[verify mir-core.evidence]
#[test]
fn branch_call_and_drop_edges_are_extracted_in_order() {
    let body = Body {
        locals: vec![
            super::return_local(),
            super::local(LocalRole::Temporary, TypeTag::Reference),
        ],
        blocks: vec![
            BasicBlock {
                statements: vec![Statement::Nop],
                terminator: Some(Terminator::Switch {
                    discriminant: super::constant(0),
                    targets: vec![SECOND_BLOCK, THIRD_BLOCK],
                }),
            },
            BasicBlock {
                statements: Vec::new(),
                terminator: Some(Terminator::Call {
                    function: super::constant(TEST_BITS),
                    arguments: vec![Operand::Copy(super::place(SECOND_LOCAL))],
                    destination: Some((super::place(RETURN_PLACE), FINAL_BLOCK)),
                }),
            },
            BasicBlock {
                statements: Vec::new(),
                terminator: Some(Terminator::Drop {
                    place: super::place(SECOND_LOCAL),
                    target: FINAL_BLOCK,
                }),
            },
            super::return_block(),
        ],
    };

    assert_eq!(admit_body(&body), Ok(()));
    assert_eq!(
        extract_adapter_facts(&body),
        Ok(AdapterFacts {
            block_count: BLOCK_COUNT,
            local_count: LOCAL_COUNT,
            edges: vec![
                Edge {
                    source: RETURN_PLACE,
                    target: SECOND_BLOCK,
                },
                Edge {
                    source: RETURN_PLACE,
                    target: THIRD_BLOCK,
                },
                Edge {
                    source: SECOND_BLOCK,
                    target: FINAL_BLOCK,
                },
                Edge {
                    source: THIRD_BLOCK,
                    target: FINAL_BLOCK,
                },
            ],
        })
    );
}

// r[verify mir-core.rvalue-nonnesting]
// r[verify mir-core.evidence]
#[test]
fn every_shallow_rvalue_family_is_admitted() {
    let local_count = SECOND_BLOCK;
    let place = super::place(RETURN_PLACE);
    let values = [
        Rvalue::Use(super::constant(0)),
        Rvalue::Reference(place),
        Rvalue::Aggregate(vec![super::constant(TEST_BITS)]),
    ];
    for value in values {
        assert_eq!(admit_rvalue(&value, local_count), Ok(()));
    }
}

// r[verify mir-core.successors]
// r[verify mir-core.evidence]
// r[verify mir-core.non-claims]
#[test]
fn terminal_terminators_have_no_edges() {
    let return_terminator = Terminator::Return;
    let unreachable_terminator = Terminator::Unreachable;
    assert_eq!(admit_terminator(&return_terminator, SECOND_BLOCK, SECOND_BLOCK), Ok(()));
    assert_eq!(admit_terminator(&unreachable_terminator, SECOND_BLOCK, SECOND_BLOCK), Ok(()));
}
