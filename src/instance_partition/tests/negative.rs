use super::super::*;

const FIRST: u32 = 0;
const SECOND: u32 = 1;
const THIRD: u32 = 2;
const FOURTH: u32 = 3;
const FIFTH: u32 = 4;

fn rejection(class: FailureClass, table: Table, index: u32, subject: Option<u32>) -> Rejection {
    Rejection {
        class,
        table,
        index: Some(index),
        subject,
    }
}

// r[verify mono.item-envelope]
// r[verify mono.evidence]
#[test]
fn generic_instance_without_arguments_is_rejected() {
    let mut record = super::valid_record();
    for item in &mut record.items {
        if item.key == super::key(super::GENERIC_ITEM) {
            item.type_arguments = Vec::new();
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::MalformedInstance, Table::Item, SECOND, Some(super::GENERIC_ITEM),))
    );
}

// r[verify mono.item-envelope]
// r[verify mono.evidence]
#[test]
fn static_with_type_arguments_is_rejected() {
    let mut record = super::valid_record();
    for item in &mut record.items {
        if item.key == super::key(super::STATIC_ITEM) {
            item.type_arguments = vec![super::TYPE_ARGUMENT];
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::MalformedInstance, Table::Item, THIRD, Some(super::STATIC_ITEM),))
    );
}

// r[verify mono.collection]
// r[verify mono.evidence]
#[test]
fn missing_reachable_item_is_rejected() {
    let mut record = super::valid_record();
    record.collected.retain(|key| *key != super::key(super::SHIM_ITEM));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::MissingReachable, Table::Collected, FOURTH, Some(super::SHIM_ITEM),))
    );
}

// r[verify mono.collection]
// r[verify mono.evidence]
#[test]
fn dangling_reference_target_is_rejected() {
    let mut record = super::valid_record();
    record.references = vec![Reference {
        source: super::key(super::ROOT_ITEM),
        target: super::key(super::UNKNOWN_ITEM),
    }];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DanglingReference, Table::Reference, FIRST, Some(super::UNKNOWN_ITEM),))
    );
}

// r[verify mono.dedup]
// r[verify mono.evidence]
#[test]
fn duplicate_collected_identity_is_rejected() {
    let mut record = super::valid_record();
    record.collected.push(super::key(super::ROOT_ITEM));
    let expected = Err(rejection(FailureClass::DuplicateCollected, Table::Collected, FIFTH, Some(super::ROOT_ITEM)));
    assert_eq!(admit_record(&record), expected);
    assert_eq!(admit_record(&record), expected);
}

// r[verify mono.collection]
// r[verify mono.evidence]
#[test]
fn extra_collected_identity_is_rejected() {
    let mut record = super::valid_record();
    record.collected.push(super::key(super::UNKNOWN_ITEM));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::ExtraCollected, Table::Collected, FIFTH, Some(super::UNKNOWN_ITEM),))
    );
}

// r[verify mono.cgu-partition]
// r[verify mono.evidence]
#[test]
fn partition_overlap_is_rejected() {
    let mut record = super::valid_record();
    for unit in &mut record.units {
        if unit.key == UnitKey(super::VOLATILE_UNIT) {
            unit.items.push(super::key(super::ROOT_ITEM));
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::PartitionOverlap, Table::Unit, SECOND, Some(super::ROOT_ITEM),))
    );
}

// r[verify mono.cgu-partition]
// r[verify mono.evidence]
#[test]
fn partition_omission_is_rejected() {
    let mut record = super::valid_record();
    for unit in &mut record.units {
        unit.items.retain(|key| *key != super::key(super::SHIM_ITEM));
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::PartitionOmission, Table::Collected, FOURTH, Some(super::SHIM_ITEM),))
    );
}

// r[verify mono.cgu-classification]
// r[verify mono.evidence]
#[test]
fn generic_item_cannot_claim_stable_class() {
    let mut record = super::valid_record();
    for item in &mut record.items {
        if item.key == super::key(super::GENERIC_ITEM) {
            item.stability = Stability::Stable;
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::ClassificationMismatch, Table::Item, SECOND, Some(super::GENERIC_ITEM),))
    );
}

// r[verify mono.admission]
// r[verify mono.evidence]
#[test]
fn duplicate_item_identity_is_rejected() {
    let mut record = super::valid_record();
    record
        .items
        .push(super::item(super::ROOT_ITEM, ItemKind::Function, ConcreteShape::Plain, Stability::Stable));
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateItem, Table::Item, FIFTH, Some(super::ROOT_ITEM),))
    );
}

// r[verify mono.collection]
// r[verify mono.evidence]
#[test]
fn missing_root_is_rejected() {
    let mut record = super::valid_record();
    record.roots = vec![super::key(super::UNKNOWN_ITEM)];
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::MissingRoot, Table::Root, FIRST, Some(super::UNKNOWN_ITEM),))
    );
}

// r[verify mono.cgu-partition]
// r[verify mono.evidence]
#[test]
fn unknown_unit_item_is_rejected() {
    let mut record = super::valid_record();
    for unit in &mut record.units {
        if unit.key == UnitKey(super::STABLE_UNIT) {
            unit.items.push(super::key(super::UNKNOWN_ITEM));
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::UnknownUnitItem, Table::Unit, FIRST, Some(super::UNKNOWN_ITEM),))
    );
}

// r[verify mono.cgu-classification]
// r[verify mono.cgu-partition]
#[test]
fn unit_class_must_match_each_item() {
    let mut record = super::valid_record();
    for unit in &mut record.units {
        if unit.key == UnitKey(super::STABLE_UNIT) {
            unit.stability = Stability::Volatile;
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::UnitClassMismatch, Table::Unit, FIRST, Some(super::ROOT_ITEM),))
    );
}

// r[verify mono.cgu-partition]
#[test]
fn duplicate_unit_identity_is_rejected() {
    let mut record = super::valid_record();
    for unit in &mut record.units {
        if unit.key == UnitKey(super::VOLATILE_UNIT) {
            unit.key = UnitKey(super::STABLE_UNIT);
        }
    }
    assert_eq!(
        admit_record(&record),
        Err(rejection(FailureClass::DuplicateUnit, Table::Unit, SECOND, Some(super::STABLE_UNIT),))
    );
}
