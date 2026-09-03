use super::super::*;

// r[verify domain-primitives.typed-config.expression-envelope]
// r[verify domain-primitives.typed-config.type-admission]
// r[verify domain-primitives.typed-config.explicit-failure]
// r[verify domain-primitives.typed-config.positive]
#[test]
fn complete_typed_record_is_admitted() {
    assert_eq!(
        admit_record(&super::valid_record()),
        Ok(Facts {
            node_count: super::NODE_COUNT,
            field_count: super::FIELD_COUNT,
            import_count: super::IMPORT_COUNT,
        })
    );
}

// r[verify domain-primitives.typed-config.normal-form]
// r[verify domain-primitives.typed-config.positive]
#[test]
fn record_fields_normalize_to_name_order() {
    let labels = normalize(&super::valid_record()).map(|normal| {
        normal
            .entries
            .into_iter()
            .find(|entry| entry.key == super::key(super::ROOT))
            .map(|entry| entry.children.into_iter().map(|child| child.label).collect::<Vec<_>>())
    });
    assert_eq!(
        labels,
        Ok(Some(vec![
            "enabled".to_owned(),
            "import".to_owned(),
            "items".to_owned(),
            "name".to_owned(),
            "optional".to_owned(),
            "reference".to_owned(),
            "variant".to_owned(),
        ]))
    );
}

// r[verify domain-primitives.typed-config.normal-form]
// r[verify domain-primitives.typed-config.semantic-digest]
#[test]
fn source_order_labels_and_comments_do_not_change_digest_input() {
    let before = super::valid_record();
    let mut after = before.clone();
    for node in &mut after.nodes {
        node.source_label = "renamed".to_owned();
        node.comment = "different review text".to_owned();
        if let Kind::Record(fields) = &mut node.kind {
            fields.reverse();
        }
    }
    assert_eq!(select_digest_input(&before), select_digest_input(&after));
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.positive]
#[test]
fn matching_frozen_same_origin_import_is_admitted() {
    assert_eq!(admit_imports(&super::valid_record()), Ok(super::IMPORT_COUNT));
}

// r[verify domain-primitives.typed-config.semantic-digest]
#[test]
fn dependency_digest_drift_changes_selected_input() {
    let before = super::valid_record();
    let mut after = before.clone();
    after.dependency_digests = vec![super::digest(super::SECOND_DIGEST_BYTE)];
    assert_ne!(select_digest_input(&before), select_digest_input(&after));
}

// r[verify domain-primitives.typed-config.semantic-refactoring]
// r[verify domain-primitives.typed-config.positive]
#[test]
fn equal_digest_input_admits_source_refactor() {
    let input = select_digest_input(&super::valid_record());
    let request = input.map(|selected| Refactor {
        before: selected.clone(),
        after: selected,
        compatibility: Compatibility::None,
    });
    assert_eq!(request.and_then(|value| admit_refactor(&value)), Ok(()));
}

// r[verify domain-primitives.typed-config.semantic-refactoring]
#[test]
fn explicit_compatibility_admits_digest_change() {
    let before = select_digest_input(&super::valid_record());
    let mut changed = super::valid_record();
    changed.dependency_digests = vec![super::digest(super::SECOND_DIGEST_BYTE)];
    let after = select_digest_input(&changed);
    let request = before.and_then(|left| {
        after.map(|right| Refactor {
            before: left,
            after: right,
            compatibility: Compatibility::Compatible,
        })
    });
    assert_eq!(request.and_then(|value| admit_refactor(&value)), Ok(()));
}

// r[verify domain-primitives.typed-config.docs]
// r[verify domain-primitives.typed-config.final-validation]
// r[verify domain-primitives.typed-config.non-claims]
#[test]
fn selected_input_contains_metadata_not_a_hash_claim() {
    let selected = select_digest_input(&super::valid_record());
    assert!(matches!(selected, Ok(DigestInput { .. })));
}
