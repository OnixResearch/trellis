use super::*;

const EXPECTED_MODULE_COUNT: usize = 2;
const EXPECTED_MODULE_BUDGET: u32 = 2;
const EXPECTED_POOL_LENGTH: usize = 1;
const EXPECTED_LIST_LENGTH: u32 = 1;
const EXPECTED_VECTOR_LENGTH: usize = 2;

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.reader-token-span]
// r[verify scheme-logic.reader-datum-structure]
#[test]
fn balanced_reader_and_child_spans_are_admitted() {
    assert!(bounded_span(TOKEN_OPEN, SPAN_OPEN_END, SOURCE_LENGTH));
    assert_eq!(admit_reader(&balanced_reader()), Ok(()));
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.reader-datum-structure]
#[test]
fn quote_attachment_names_the_following_datum() {
    let envelope = ReaderEnvelope {
        source_length: SPAN_DATUM_END,
        tokens: vec![
            Token {
                kind: TokenKind::Prefix(PrefixKind::Quote),
                span: span(TOKEN_OPEN, SPAN_OPEN_END),
            },
            Token {
                kind: TokenKind::Atom,
                span: span(TOKEN_DATUM, SPAN_DATUM_END),
            },
        ],
        delimiter_pairs: Vec::new(),
        attachments: vec![Attachment {
            prefix_index: TOKEN_OPEN,
            datum_index: TOKEN_DATUM,
        }],
        child_spans: Vec::new(),
    };
    assert_eq!(admit_reader(&envelope), Ok(()));
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.interning]
#[test]
fn intern_then_resolve_roundtrips() {
    let value = intern_value(InternKind::Symbol, SYMBOL_VALUE);
    let result = intern(&empty_interner(), value.clone()).expect("bounded symbol should intern");
    let resolved = resolve(&result.table, result.key).expect("returned key should resolve");

    assert_eq!(resolved, value);
    assert!(result.was_inserted);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.interning]
#[test]
fn repeated_intern_keeps_existing_key_and_table() {
    let value = intern_value(InternKind::Literal, LITERAL_VALUE);
    let first = intern(&empty_interner(), value.clone()).expect("literal should intern");
    let second = intern(&first.table, value).expect("existing literal should resolve");

    assert_eq!(second.key, first.key);
    assert_eq!(second.table, first.table);
    assert!(!second.was_inserted);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.module-graph]
#[test]
fn provided_import_closure_is_cycle_safe() {
    let graph = module_graph();
    assert_eq!(admit_modules(&graph), Ok(()));
    let closure =
        import_closure(&graph, ModuleId(MODULE_CLIENT), TRAVERSAL_BUDGET).expect("provided graph should close");

    assert_eq!(closure.modules.len(), EXPECTED_MODULE_COUNT);
    assert!(closure.modules.contains(&ModuleId(MODULE_PROVIDER)));
    assert_eq!(closure.consumed_budget, EXPECTED_MODULE_BUDGET);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.contract-blame]
#[test]
fn function_arity_and_blame_polarity_are_explicit() {
    let descriptor = wrapper(PARTY_CALLER, PARTY_PROVIDER);
    assert_eq!(admit_wrapper(descriptor), Ok(()));
    assert_eq!(admit_blame(descriptor, FailurePosition::Domain, PartyId(PARTY_CALLER),), Ok(BlamePolarity::Caller),);
    assert_eq!(
        admit_blame(descriptor, FailurePosition::Codomain, PartyId(PARTY_PROVIDER),),
        Ok(BlamePolarity::Provider),
    );
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.contract-blame]
#[test]
fn matching_wrapper_boundary_composes() {
    let inner = wrapper(PARTY_CALLER, PARTY_MIDDLE);
    let outer = wrapper(PARTY_MIDDLE, PARTY_PROVIDER);
    let composition = compose_wrappers(inner, outer).expect("matching wrappers should compose");

    assert_eq!(composition.inner, inner);
    assert_eq!(composition.outer, outer);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.bytecode-safety]
#[test]
fn constant_dedup_and_rollback_preserve_lookup() {
    let pool = ConstantPool {
        values: vec![ConstantValue::Integer(CONSTANT_VALUE)],
    };
    let existing =
        add_or_get_constant(&pool, ConstantValue::Integer(CONSTANT_VALUE)).expect("existing constant should resolve");
    assert_eq!(existing.index, TOKEN_OPEN);
    assert_eq!(existing.pool, pool);
    assert!(!existing.was_inserted);

    let rollback = rollback_pool(&pool, RESULT_COUNT).expect("whole prefix should be retained");
    assert_eq!(rollback.values.len(), EXPECTED_POOL_LENGTH);
    assert_eq!(rollback.values[TOKEN_OPEN as usize], ConstantValue::Integer(CONSTANT_VALUE));
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.bytecode-safety]
#[test]
fn bounded_bytecode_has_exact_stack_summary() {
    let summary = admit_code(&safe_code()).expect("bounded bytecode should be admitted");
    assert_eq!(summary.final_stack_height, FINAL_STACK);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.value-graph]
#[test]
fn acyclic_and_cyclic_graphs_traverse_with_visited_sets() {
    let acyclic = traverse_values(&proper_list_graph(), NodeId(NODE_PAIR), TraversalPurpose::Display, TRAVERSAL_BUDGET)
        .expect("acyclic graph should traverse");
    assert_eq!(acyclic.visited.len(), TRAVERSAL_BUDGET as usize);

    let cyclic = traverse_values(&cyclic_graph(), NodeId(NODE_EMPTY), TraversalPurpose::Display, TRAVERSAL_BUDGET)
        .expect("visited set should stop the cycle");
    assert_eq!(cyclic.visited, vec![NodeId(NODE_EMPTY)]);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.value-graph]
#[test]
fn cyclic_shape_equality_terminates() {
    assert_eq!(compare_values(&cyclic_graph(), NodeId(NODE_EMPTY), NodeId(NODE_EMPTY), TRAVERSAL_BUDGET,), Ok(true),);
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.collections-bytevectors]
#[test]
fn proper_list_and_vector_update_preserve_shape() {
    let graph = proper_list_graph();
    assert_eq!(proper_list_length(&graph, NodeId(NODE_PAIR), TRAVERSAL_BUDGET), Ok(EXPECTED_LIST_LENGTH),);
    let vector = vec![NodeId(NODE_EMPTY), NodeId(NODE_SCALAR)];
    let updated =
        update_vector(&vector, RESULT_COUNT, NodeId(NODE_PAIR)).expect("in-range immutable update should succeed");
    assert_eq!(updated.len(), EXPECTED_VECTOR_LENGTH);
    assert_eq!(updated[RESULT_COUNT as usize], NodeId(NODE_PAIR));
    assert_eq!(vector[RESULT_COUNT as usize], NodeId(NODE_SCALAR));
}

// r[verify scheme-logic.evidence.positive]
// r[verify scheme-logic.collections-bytevectors]
#[test]
fn bytevector_and_hash_shapes_are_admitted() {
    let bytes = BytevectorEnvelope {
        declared_length: SOURCE_LENGTH,
        elements: vec![0, 1, u16::from(u8::MAX)],
    };
    assert_eq!(admit_bytevector(&bytes), Ok(()));

    let graph = proper_list_graph();
    let hashes = HashShape {
        entries: vec![(NodeId(NODE_SCALAR), NodeId(NODE_PAIR))],
        set_members: vec![NodeId(NODE_EMPTY)],
    };
    assert_eq!(admit_hash_shape(&graph, &hashes), Ok(()));
}

// r[verify scheme-logic.docs]
// r[verify scheme-logic.final-validation]
// r[verify scheme-logic.non-claims]
#[test]
fn public_boundary_admits_only_finite_predicates() {
    assert_eq!(admit_claim(ClaimScope::FinitePredicates), Ok(()));
}
