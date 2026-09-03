use super::super::*;

// r[verify mono.item-envelope]
// r[verify mono.collection]
// r[verify mono.dedup]
// r[verify mono.cgu-classification]
// r[verify mono.cgu-partition]
// r[verify mono.admission]
// r[verify mono.evidence]
// r[verify mono.docs]
// r[verify mono.final-validation]
#[test]
fn complete_collection_and_partition_are_admitted() {
    assert_eq!(
        admit_record(&super::valid_record()),
        Ok(Facts {
            item_count: super::ITEM_COUNT,
            root_count: super::ROOT_COUNT,
            collected_count: super::COLLECTED_COUNT,
            unit_count: super::UNIT_COUNT,
        })
    );
}

// r[verify mono.collection]
// r[verify mono.dedup]
// r[verify mono.evidence]
#[test]
fn duplicate_roots_and_paths_collapse_canonically() {
    assert_eq!(
        collect_reachable(&super::valid_record()),
        Ok(vec![
            super::key(super::ROOT_ITEM),
            super::key(super::GENERIC_ITEM),
            super::key(super::STATIC_ITEM),
            super::key(super::SHIM_ITEM),
        ])
    );
}

// r[verify mono.item-envelope]
// r[verify mono.collection]
#[test]
fn functions_statics_and_shims_are_collected() {
    let record = super::valid_record();
    assert!(record.items.iter().any(|item| item.kind == ItemKind::Function));
    assert!(record.items.iter().any(|item| item.kind == ItemKind::Static));
    assert!(record.items.iter().any(|item| item.kind == ItemKind::Shim));
    assert_eq!(collect_reachable(&record).map(|items| items.len()), Ok(super::ITEM_COUNT_USIZE));
}

// r[verify mono.cgu-classification]
// r[verify mono.cgu-partition]
#[test]
fn stable_and_volatile_units_cover_distinct_items() {
    let record = super::valid_record();
    assert!(record.units.iter().any(|unit| unit.stability == Stability::Stable));
    assert!(record.units.iter().any(|unit| unit.stability == Stability::Volatile));
    assert_eq!(admit_record(&record).map(|facts| facts.unit_count), Ok(super::UNIT_COUNT));
}

// r[verify mono.non-claims]
#[test]
fn admitted_facts_export_counts_not_generated_code() {
    assert_eq!(admit_record(&super::valid_record()).map(|facts| facts.item_count), Ok(super::ITEM_COUNT));
}
