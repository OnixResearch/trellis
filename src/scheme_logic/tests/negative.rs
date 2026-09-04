use super::*;

const CROSSING_SOURCE_LENGTH: u32 = 4;
const TOKEN_FOURTH: u32 = 3;
const SPAN_FOURTH_END: u32 = 4;
const OUTSIDE_INDEX: u32 = 99;
const ZERO_BUDGET: u32 = 0;

fn reader_fault(envelope: &ReaderEnvelope) -> ReaderFault {
    admit_reader(envelope).expect_err("malformed reader must reject").class
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.reader-token-span]
#[test]
fn invalid_and_overlapping_token_spans_reject() {
    let mut invalid = balanced_reader();
    invalid.tokens[TOKEN_DATUM as usize].span.end = INVALID_BYTE.into();
    assert_eq!(reader_fault(&invalid), ReaderFault::InvalidSpan);

    let mut overlap = balanced_reader();
    overlap.tokens[TOKEN_DATUM as usize].span.start = TOKEN_OPEN;
    assert_eq!(reader_fault(&overlap), ReaderFault::TokenOrder);

    let mut child_outside = balanced_reader();
    child_outside.child_spans[TOKEN_OPEN as usize].child.end = SPAN_FOURTH_END;
    assert_eq!(reader_fault(&child_outside), ReaderFault::ChildOutsideParent);
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.reader-datum-structure]
#[test]
fn unmatched_mismatched_and_crossing_delimiters_reject() {
    let mut unmatched = balanced_reader();
    unmatched.delimiter_pairs.clear();
    assert_eq!(reader_fault(&unmatched), ReaderFault::UnmatchedDelimiter);

    let mut mismatched = balanced_reader();
    mismatched.tokens[TOKEN_CLOSE as usize].kind = TokenKind::Close(Delimiter::Bracket);
    assert_eq!(reader_fault(&mismatched), ReaderFault::DelimiterKind);

    let crossing = ReaderEnvelope {
        source_length: CROSSING_SOURCE_LENGTH,
        tokens: vec![
            Token {
                kind: TokenKind::Open(Delimiter::Parenthesis),
                span: span(TOKEN_OPEN, SPAN_OPEN_END),
            },
            Token {
                kind: TokenKind::Open(Delimiter::Bracket),
                span: span(TOKEN_DATUM, SPAN_DATUM_END),
            },
            Token {
                kind: TokenKind::Close(Delimiter::Parenthesis),
                span: span(TOKEN_CLOSE, SPAN_CLOSE_END),
            },
            Token {
                kind: TokenKind::Close(Delimiter::Bracket),
                span: span(TOKEN_FOURTH, SPAN_FOURTH_END),
            },
        ],
        delimiter_pairs: vec![
            DelimiterPair {
                open_index: TOKEN_OPEN,
                close_index: TOKEN_CLOSE,
            },
            DelimiterPair {
                open_index: TOKEN_DATUM,
                close_index: TOKEN_FOURTH,
            },
        ],
        attachments: Vec::new(),
        child_spans: Vec::new(),
    };
    assert_eq!(reader_fault(&crossing), ReaderFault::CrossingDelimiter);
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.reader-datum-structure]
#[test]
fn missing_or_nonadjacent_attachment_rejects() {
    let mut envelope = balanced_reader();
    envelope.tokens[TOKEN_DATUM as usize].kind = TokenKind::Prefix(PrefixKind::DatumComment);
    assert_eq!(reader_fault(&envelope), ReaderFault::MissingAttachment);

    envelope.attachments.push(Attachment {
        prefix_index: TOKEN_DATUM,
        datum_index: TOKEN_CLOSE,
    });
    assert_eq!(reader_fault(&envelope), ReaderFault::AttachmentKind);
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.interning]
#[test]
fn malformed_interner_tables_and_dangling_keys_reject() {
    let value = intern_value(InternKind::Symbol, SYMBOL_VALUE);
    let duplicate = InternerTable {
        next_key: PARAMETER_COUNT,
        entries: vec![
            InternEntry {
                key: InternKey(TOKEN_OPEN),
                value: value.clone(),
            },
            InternEntry {
                key: InternKey(TOKEN_OPEN),
                value: intern_value(InternKind::Literal, LITERAL_VALUE),
            },
        ],
    };
    assert_eq!(
        admit_interner(&duplicate).expect_err("duplicate key must reject").class,
        InternerFault::DuplicateKey,
    );

    let duplicate_value = InternerTable {
        next_key: PARAMETER_COUNT,
        entries: vec![
            InternEntry {
                key: InternKey(TOKEN_OPEN),
                value: value.clone(),
            },
            InternEntry {
                key: InternKey(TOKEN_DATUM),
                value: value.clone(),
            },
        ],
    };
    assert_eq!(
        admit_interner(&duplicate_value).expect_err("duplicate value must reject").class,
        InternerFault::DuplicateValue,
    );

    let inserted = intern(&empty_interner(), value).expect("symbol should intern");
    assert_eq!(
        resolve(&inserted.table, InternKey(OUTSIDE_INDEX)).expect_err("missing key must reject").class,
        InternerFault::MissingKey,
    );

    let dangling = InternerTable {
        next_key: PARAMETER_COUNT,
        entries: inserted.table.entries,
    };
    assert_eq!(
        admit_interner(&dangling).expect_err("missing dense association must reject").class,
        InternerFault::NextKeyMismatch,
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.module-graph]
#[test]
fn duplicate_export_and_unprovided_import_reject() {
    let mut duplicate = module_graph();
    duplicate.modules[MODULE_PROVIDER as usize].exports.push(NameId(PROVIDED_NAME));
    assert_eq!(
        admit_modules(&duplicate).expect_err("duplicate export must reject").class,
        ModuleFault::DuplicateExport,
    );

    let mut unprovided = module_graph();
    unprovided.imports[TOKEN_OPEN as usize].name = NameId(MISSING_NAME);
    assert_eq!(
        admit_modules(&unprovided).expect_err("unprovided import must reject").class,
        ModuleFault::UnprovidedImport,
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.module-graph]
#[test]
fn cyclic_module_closure_rejects_under_budget() {
    let mut graph = module_graph();
    graph.modules[MODULE_CLIENT as usize].exports.push(NameId(PROVIDED_NAME));
    graph.imports.push(ImportEdge {
        from: ModuleId(MODULE_PROVIDER),
        target: ModuleId(MODULE_CLIENT),
        name: NameId(PROVIDED_NAME),
    });
    assert_eq!(
        import_closure(&graph, ModuleId(MODULE_CLIENT), RESULT_COUNT)
            .expect_err("under-budget cycle must reject")
            .class,
        ModuleFault::BudgetExhausted,
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.contract-blame]
#[test]
fn arity_blame_and_composition_mismatches_reject() {
    let mut arity = wrapper(PARTY_CALLER, PARTY_PROVIDER);
    arity.callable.parameter_count = RESULT_COUNT;
    assert_eq!(admit_wrapper(arity), Err(ContractFault::ArityMismatch));

    let descriptor = wrapper(PARTY_CALLER, PARTY_PROVIDER);
    assert_eq!(
        admit_blame(descriptor, FailurePosition::Domain, PartyId(PARTY_PROVIDER),),
        Err(ContractFault::WrongBlameParty),
    );

    let inner = wrapper(PARTY_CALLER, PARTY_MIDDLE);
    let outer = wrapper(PARTY_CALLER, PARTY_PROVIDER);
    assert_eq!(compose_wrappers(inner, outer), Err(ContractFault::IncompatibleComposition),);
}

fn code_fault(envelope: &CodeEnvelope) -> CodeFault {
    admit_code(envelope).expect_err("unsafe bytecode must reject").class
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.bytecode-safety]
#[test]
fn bytecode_operand_bounds_reject() {
    let mut constant = safe_code();
    constant.instructions[TOKEN_OPEN as usize] = Instruction::PushConstant(OUTSIDE_INDEX);
    assert_eq!(code_fault(&constant), CodeFault::ConstantIndexOutsidePool);

    let mut local = safe_code();
    local.instructions[TOKEN_DATUM as usize] = Instruction::LoadLocal(OUTSIDE_INDEX);
    assert_eq!(code_fault(&local), CodeFault::LocalIndexOutsideFrame);

    let mut capture = safe_code();
    capture.instructions[TOKEN_DATUM as usize] = Instruction::LoadCapture(OUTSIDE_INDEX);
    assert_eq!(code_fault(&capture), CodeFault::CaptureIndexOutsideClosure);

    let mut jump = safe_code();
    jump.instructions[TOKEN_DATUM as usize] = Instruction::Jump(OUTSIDE_INDEX);
    assert_eq!(code_fault(&jump), CodeFault::JumpTargetOutsideCode);
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.bytecode-safety]
#[test]
fn stack_underflow_and_rollback_bounds_reject() {
    let underflow = CodeEnvelope {
        pool: ConstantPool { values: Vec::new() },
        instructions: vec![Instruction::Pop],
        local_count: 0,
        capture_count: 0,
        initial_stack_height: 0,
        expected_final_stack_height: 0,
    };
    assert_eq!(code_fault(&underflow), CodeFault::StackUnderflow);

    let mut call_arity = safe_code();
    call_arity.instructions[TOKEN_CLOSE as usize] = Instruction::Call {
        argument_count: u32::MAX,
    };
    assert_eq!(code_fault(&call_arity), CodeFault::CallArityTooLarge);

    let mut final_height = safe_code();
    final_height.expected_final_stack_height = INITIAL_STACK;
    assert_eq!(code_fault(&final_height), CodeFault::FinalStackMismatch);

    let pool = ConstantPool {
        values: vec![ConstantValue::Integer(CONSTANT_VALUE)],
    };
    assert_eq!(
        rollback_pool(&pool, PARAMETER_COUNT).expect_err("rollback outside pool must reject").class,
        CodeFault::RollbackOutsidePool,
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.value-graph]
#[test]
fn dangling_edges_and_under_budget_cycles_reject() {
    let dangling = ValueGraph {
        nodes: vec![ValueNode::Pair {
            car: NodeId(OUTSIDE_INDEX),
            cdr: NodeId(NODE_EMPTY),
        }],
    };
    assert_eq!(admit_values(&dangling).expect_err("dangling edge must reject").class, GraphFault::DanglingEdge,);
    assert_eq!(
        traverse_values(&cyclic_graph(), NodeId(NODE_EMPTY), TraversalPurpose::Display, ZERO_BUDGET,)
            .expect_err("zero budget cannot expand cycle")
            .class,
        GraphFault::BudgetExhausted,
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.collections-bytevectors]
#[test]
fn improper_cyclic_and_out_of_range_collections_reject() {
    let graph = proper_list_graph();
    assert_eq!(
        proper_list_length(&graph, NodeId(NODE_SCALAR), TRAVERSAL_BUDGET),
        Err(CollectionFault::ImproperList),
    );
    assert_eq!(
        proper_list_length(&cyclic_graph(), NodeId(NODE_EMPTY), TRAVERSAL_BUDGET),
        Err(CollectionFault::CyclicList),
    );
    let invalid = BytevectorEnvelope {
        declared_length: RESULT_COUNT,
        elements: vec![INVALID_BYTE],
    };
    assert_eq!(admit_bytevector(&invalid), Err(CollectionFault::ByteOutsideDomain),);
    assert_eq!(
        update_vector(&[NodeId(NODE_EMPTY)], OUTSIDE_INDEX, NodeId(NODE_PAIR)),
        Err(CollectionFault::IndexOutsideVector),
    );
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.collections-bytevectors]
#[test]
fn duplicate_hash_and_set_entries_reject() {
    let graph = proper_list_graph();
    let duplicate_key = HashShape {
        entries: vec![
            (NodeId(NODE_EMPTY), NodeId(NODE_SCALAR)),
            (NodeId(NODE_EMPTY), NodeId(NODE_PAIR)),
        ],
        set_members: Vec::new(),
    };
    assert_eq!(admit_hash_shape(&graph, &duplicate_key), Err(CollectionFault::DuplicateHashKey),);

    let duplicate_member = HashShape {
        entries: Vec::new(),
        set_members: vec![NodeId(NODE_EMPTY), NodeId(NODE_EMPTY)],
    };
    assert_eq!(admit_hash_shape(&graph, &duplicate_member), Err(CollectionFault::DuplicateSetMember),);
}

// r[verify scheme-logic.evidence.negative]
// r[verify scheme-logic.non-claims]
#[test]
fn steel_and_runtime_claim_promotions_reject() {
    let unsupported = [
        ClaimScope::SteelCompatibility,
        ClaimScope::SchemeCompliance,
        ClaimScope::ParserCompleteness,
        ClaimScope::MacroSoundness,
        ClaimScope::GarbageCollectorCorrectness,
        ClaimScope::JitCorrectness,
        ClaimScope::FfiSafety,
        ClaimScope::RuntimeModuleLoading,
        ClaimScope::WholeInterpreterCorrectness,
    ];
    for scope in unsupported {
        assert_eq!(admit_claim(scope), Err(ClaimFault::UnsupportedPromotion));
    }
}
