use super::super::*;

// r[verify backend-trace.handles]
// r[verify backend-trace.block-mapping]
// r[verify backend-trace.lowering-trace]
// r[verify backend-trace.handle-validity]
// r[verify backend-trace.terminators]
// r[verify backend-trace.ssa-like-locals]
// r[verify backend-trace.admission]
// r[verify backend-trace.evidence]
// r[verify backend-trace.docs]
// r[verify backend-trace.final-validation]
#[test]
fn complete_builder_record_is_admitted() {
    assert_eq!(
        admit_record(&super::body(), &super::valid_record()),
        Ok(Facts {
            block_count: super::BLOCK_COUNT,
            operation_count: super::OPERATION_COUNT,
            value_count: super::VALUE_COUNT,
            local_count: super::LOCAL_COUNT,
        })
    );
}

// r[verify backend-trace.block-mapping]
// r[verify backend-trace.terminators]
// r[verify backend-trace.evidence]
#[test]
fn declared_call_expansion_and_unwind_block_are_admitted() {
    let record = super::valid_record();
    assert!(record.mappings.iter().any(|mapping| mapping.expansions == vec![BlockKey(super::EXPANDED_ENTRY)]));
    assert_eq!(admit_record(&super::body(), &record).map(|facts| facts.operation_count), Ok(super::OPERATION_COUNT));
}

// r[verify backend-trace.lowering-trace]
// r[verify backend-trace.handle-validity]
#[test]
fn operand_place_rvalue_and_statement_entries_compose() {
    let record = super::valid_record();
    assert_eq!(u32::try_from(record.entries.len()), Ok(super::OPERATION_COUNT));
    assert_eq!(admit_record(&super::body(), &record).map(|facts| facts.value_count), Ok(super::VALUE_COUNT));
}

// r[verify backend-trace.ssa-like-locals]
#[test]
fn direct_and_memory_backed_locals_are_distinct() {
    let record = super::valid_record();
    assert!(record.locals.iter().any(|class| class.storage == LocalStorage::DirectValue));
    assert!(record.locals.iter().any(|class| class.storage == LocalStorage::MemoryBacked));
    assert_eq!(admit_record(&super::body(), &record).map(|facts| facts.local_count), Ok(super::LOCAL_COUNT));
}

// r[verify backend-trace.non-claims]
#[test]
fn admitted_facts_export_counts_not_backend_semantics() {
    assert_eq!(
        admit_record(&super::body(), &super::valid_record()).map(|facts| facts.block_count),
        Ok(super::BLOCK_COUNT)
    );
}
