use super::super::*;

const FIRST: u32 = 0;
const SECOND: u32 = 1;
const THIRD: u32 = 2;
const FOURTH: u32 = 3;
const FIFTH: u32 = 4;

fn rejection(class: FailureClass, table: Table, index: u32, related: Option<u32>) -> Rejection {
    Rejection {
        class,
        table,
        index: Some(index),
        related,
    }
}

// r[verify hir-id.owner-scope]
// r[verify hir-id.evidence]
#[test]
fn wrong_owner_mapping_is_rejected() {
    let mut record = super::valid_record();
    record.mappings = vec![
        Mapping {
            source: super::node(super::SOURCE_ITEM),
            target: super::other_scoped(super::RETURN_LOCAL),
        },
        super::mapping(super::SOURCE_VALUE, super::VALUE_LOCAL),
    ];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::WrongOwner, Table::Mapping, FIRST, Some(super::OTHER_OWNER),))
    );
}

// r[verify hir-id.uniqueness]
// r[verify hir-id.evidence]
#[test]
fn duplicate_hir_mapping_target_is_rejected() {
    let mut record = super::valid_record();
    record.mappings = vec![
        super::mapping(super::SOURCE_ITEM, super::RETURN_LOCAL),
        super::mapping(super::SOURCE_VALUE, super::RETURN_LOCAL),
    ];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateHirId, Table::Mapping, SECOND, Some(FIRST),))
    );
}

// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn unused_required_created_id_is_rejected() {
    let mut record = super::valid_record();
    record.used_ids = vec![super::scoped(super::RETURN_LOCAL), super::scoped(super::VALUE_LOCAL)];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::UnusedCreatedId, Table::Created, THIRD, Some(super::GENERATED_LOCAL),))
    );
}

// r[verify hir-id.lowering-map]
// r[verify hir-id.evidence]
#[test]
fn unlowered_source_is_rejected() {
    let mut record = super::valid_record();
    record.mappings = vec![super::mapping(super::SOURCE_ITEM, super::RETURN_LOCAL)];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::UnloweredSource, Table::Source, SECOND, Some(super::SOURCE_VALUE),))
    );
}

// r[verify hir-id.fresh-generated]
// r[verify hir-id.evidence]
#[test]
fn generated_id_cannot_alias_source_target() {
    let mut record = super::valid_record();
    record.generated_nodes = vec![GeneratedNode {
        generation: super::GENERATED_PROVENANCE,
        target: super::scoped(super::RETURN_LOCAL),
    }];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::GeneratedAlias, Table::Generated, FIRST, Some(super::RETURN_LOCAL),))
    );
}

// r[verify hir-id.fresh-generated]
// r[verify hir-id.evidence]
#[test]
fn duplicate_generation_provenance_is_rejected() {
    let mut record = super::valid_record();
    record.generated_nodes.push(GeneratedNode {
        generation: super::GENERATED_PROVENANCE,
        target: super::scoped(super::DISCARDED_LOCAL),
    });
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateGeneration, Table::Generated, SECOND, Some(FIRST),))
    );
}

// r[verify hir-id.fresh-generated]
// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn stale_generated_id_without_creation_is_rejected() {
    let mut record = super::valid_record();
    record.generated_nodes = vec![GeneratedNode {
        generation: super::OTHER_GENERATION,
        target: super::scoped(super::UNKNOWN_LOCAL),
    }];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::MissingCreatedId, Table::Generated, FIRST, Some(super::UNKNOWN_LOCAL),))
    );
}

// r[verify hir-id.def-correspondence]
// r[verify hir-id.evidence]
#[test]
fn missing_definition_correspondence_is_rejected() {
    let mut record = super::valid_record();
    record.definitions = Vec::new();
    assert_eq!(
        admit_record(&record),
        Err(rejection(
            FailureClass::MissingDefinition,
            Table::DefinitionRequirement,
            FIRST,
            Some(super::SOURCE_ITEM),
        ))
    );
}

// r[verify hir-id.def-correspondence]
// r[verify hir-id.evidence]
#[test]
fn unknown_definition_source_is_rejected() {
    let mut record = super::valid_record();
    record.definition_requirements = vec![super::node(super::UNKNOWN_SOURCE)];
    record.definitions = Vec::new();
    assert_eq!(
        admit_record(&record),
        Err(rejection(
            FailureClass::UnknownDefinitionSource,
            Table::DefinitionRequirement,
            FIRST,
            Some(super::UNKNOWN_SOURCE),
        ))
    );
}

// r[verify hir-id.lowering-map]
// r[verify hir-id.evidence]
#[test]
fn duplicate_mapping_source_is_rejected() {
    let mut record = super::valid_record();
    record.mappings = vec![
        super::mapping(super::SOURCE_ITEM, super::RETURN_LOCAL),
        super::mapping(super::SOURCE_ITEM, super::VALUE_LOCAL),
    ];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateSource, Table::Mapping, SECOND, Some(FIRST),))
    );
}

// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn unknown_used_id_is_rejected() {
    let mut record = super::valid_record();
    record.used_ids.push(super::scoped(super::UNKNOWN_LOCAL));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::UnknownUsedId, Table::Used, FOURTH, Some(super::UNKNOWN_LOCAL),))
    );
}

// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn explicitly_discarded_id_cannot_be_used() {
    let mut record = super::valid_record();
    record.used_ids.push(super::scoped(super::DISCARDED_LOCAL));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DiscardedIdUsed, Table::Created, FOURTH, Some(super::DISCARDED_LOCAL),))
    );
}

// r[verify hir-id.uniqueness]
// r[verify hir-id.evidence]
#[test]
fn duplicate_created_id_is_rejected() {
    let mut record = super::valid_record();
    record.created_ids.push(super::created(super::RETURN_LOCAL, UsePolicy::Required));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateCreatedId, Table::Created, FIFTH, Some(FIRST),))
    );
}

// r[verify hir-id.admission]
// r[verify hir-id.evidence]
#[test]
fn duplicate_used_id_is_rejected_deterministically() {
    let mut record = super::valid_record();
    record.used_ids.push(super::scoped(super::RETURN_LOCAL));
    let expected = Err(rejection(FailureClass::DuplicateUsedId, Table::Used, FOURTH, Some(FIRST)));
    assert_eq!(admit_record(&record), expected);
    assert_eq!(admit_record(&record), expected);
}
