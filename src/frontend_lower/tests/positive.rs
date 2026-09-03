use super::super::*;

const SOURCE_COUNT: u32 = 2;
const MAPPING_COUNT: u32 = 2;
const GENERATED_COUNT: u32 = 1;
const USED_COUNT: u32 = 3;
const DEFINITION_COUNT: u32 = 1;

// r[verify hir-id.owner-scope]
// r[verify hir-id.lowering-map]
// r[verify hir-id.fresh-generated]
// r[verify hir-id.uniqueness]
// r[verify hir-id.admission]
// r[verify hir-id.def-correspondence]
// r[verify hir-id.evidence]
// r[verify hir-id.docs]
// r[verify hir-id.final-validation]
#[test]
fn complete_owner_local_record_is_admitted() {
    assert_eq!(
        admit_record(&super::valid_record()),
        Ok(Facts {
            source_count: SOURCE_COUNT,
            mapping_count: MAPPING_COUNT,
            generated_count: GENERATED_COUNT,
            used_count: USED_COUNT,
            definition_count: DEFINITION_COUNT,
        })
    );
}

// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn explicitly_discarded_created_id_need_not_be_used() {
    let record = super::valid_record();
    assert!(record.created_ids.iter().any(|created| created.policy == UsePolicy::ExplicitDiscard));
    assert_eq!(admit_record(&record).map(|facts| facts.used_count), Ok(USED_COUNT));
}

// r[verify hir-id.def-correspondence]
// r[verify hir-id.evidence]
#[test]
fn definition_metadata_is_optional_without_a_requirement() {
    let mut record = super::valid_record();
    record.definition_requirements = Vec::new();
    record.definitions = Vec::new();
    assert_eq!(admit_record(&record).map(|facts| facts.definition_count), Ok(0));
}

// r[verify hir-id.non-claims]
#[test]
fn admitted_facts_export_counts_not_frontend_semantics() {
    let facts = admit_record(&super::valid_record());
    assert_eq!(facts.map(|value| value.generated_count), Ok(GENERATED_COUNT));
}
