use super::super::*;

fn class(result: Result<Facts, Rejection>) -> Option<FailureClass> {
    result.err().map(|error| error.class)
}

fn import_class(record: &Record) -> Option<FailureClass> {
    admit_imports(record).err().map(|error| error.class)
}

// r[verify domain-primitives.typed-config.type-admission]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn value_type_mismatch_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if node.key == super::key(super::ENABLED) {
            node.declared_type = TypeTag::Text;
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::TypeMismatch));
}

// r[verify domain-primitives.typed-config.type-admission]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn missing_required_field_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if let Kind::Record(fields) = &mut node.kind {
            fields.retain(|field| field.name != "enabled");
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::MissingField));
}

// r[verify domain-primitives.typed-config.explicit-failure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn unsupported_form_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if node.key == super::key(super::OPTIONAL) {
            node.kind = Kind::Unsupported;
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::UnsupportedForm));
}

// r[verify domain-primitives.typed-config.expression-envelope]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn dangling_child_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if node.key == super::key(super::ITEMS) {
            node.kind = Kind::List(vec![super::key(super::UNKNOWN)]);
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::DanglingNode));
}

// r[verify domain-primitives.typed-config.expression-envelope]
#[test]
fn cyclic_reference_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if node.key == super::key(super::REFERENCE) {
            node.kind = Kind::Reference(super::key(super::REFERENCE));
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::Cycle));
}

// r[verify domain-primitives.typed-config.explicit-failure]
#[test]
fn missing_root_is_rejected() {
    let mut record = super::valid_record();
    record.root = super::key(super::UNKNOWN);
    assert_eq!(class(admit_record(&record)), Some(FailureClass::MissingRoot));
}

// r[verify domain-primitives.typed-config.explicit-failure]
#[test]
fn duplicate_node_is_rejected() {
    let mut record = super::valid_record();
    record.nodes.push(super::node(super::ROOT, TypeTag::Natural, Kind::Scalar(Scalar::Natural(1))));
    assert_eq!(class(admit_record(&record)), Some(FailureClass::DuplicateNode));
}

// r[verify domain-primitives.typed-config.type-admission]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn optional_absence_cannot_satisfy_required_text() {
    let mut record = super::valid_record();
    record.required_fields.push(RequiredField {
        record: super::key(super::ROOT),
        name: "optional".to_owned(),
        expected: TypeTag::Text,
    });
    assert_eq!(class(admit_record(&record)), Some(FailureClass::TypeMismatch));
}

// r[verify domain-primitives.typed-config.normal-form]
#[test]
fn duplicate_record_field_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if let Kind::Record(fields) = &mut node.kind {
            fields.push(Field {
                name: "name".to_owned(),
                value: super::key(super::NAME),
            });
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::DuplicateField));
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn computed_import_is_rejected() {
    let mut record = super::valid_record();
    let mut edge = super::import_edge();
    edge.target = Target::Computed;
    record.imports = vec![edge];
    assert_eq!(import_class(&record), Some(FailureClass::ComputedImport));
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn conditional_import_is_rejected() {
    let mut record = super::valid_record();
    let mut edge = super::import_edge();
    edge.selection = Selection::Conditional;
    record.imports = vec![edge];
    assert_eq!(import_class(&record), Some(FailureClass::ConditionalImport));
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn remote_import_cannot_access_local_or_environment() {
    for target in [Target::LocalFile, Target::Home, Target::Environment] {
        let mut record = super::valid_record();
        let mut edge = super::import_edge();
        edge.target = target;
        edge.headers = HeaderPolicy::None;
        record.imports = vec![edge];
        assert_eq!(import_class(&record), Some(FailureClass::RemoteAttenuation));
    }
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn cross_origin_headers_are_rejected() {
    let mut record = super::valid_record();
    let mut edge = super::import_edge();
    edge.target = Target::Relative {
        authority: super::OTHER_AUTHORITY,
    };
    record.imports = vec![edge];
    assert_eq!(import_class(&record), Some(FailureClass::CrossOriginHeaders));
}

// r[verify domain-primitives.typed-config.import-closure]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn frozen_digest_mismatch_is_rejected() {
    let mut record = super::valid_record();
    let mut edge = super::import_edge();
    edge.observed = super::digest(super::SECOND_DIGEST_BYTE);
    record.imports = vec![edge];
    assert_eq!(import_class(&record), Some(FailureClass::DigestMismatch));
}

// r[verify domain-primitives.typed-config.expression-envelope]
#[test]
fn invalid_import_index_is_rejected() {
    let mut record = super::valid_record();
    for node in &mut record.nodes {
        if node.key == super::key(super::IMPORT) {
            node.kind = Kind::Import { edge: super::UNKNOWN };
        }
    }
    assert_eq!(class(admit_record(&record)), Some(FailureClass::InvalidImportIndex));
}

// r[verify domain-primitives.typed-config.semantic-refactoring]
// r[verify domain-primitives.typed-config.negative]
#[test]
fn digest_change_without_compatibility_is_rejected() {
    let before = select_digest_input(&super::valid_record());
    let mut changed = super::valid_record();
    changed.dependency_digests = vec![super::digest(super::SECOND_DIGEST_BYTE)];
    let after = select_digest_input(&changed);
    let outcome = before.and_then(|left| {
        after.and_then(|right| {
            admit_refactor(&Refactor {
                before: left,
                after: right,
                compatibility: Compatibility::None,
            })
        })
    });
    assert_eq!(outcome.err().map(|error| error.class), Some(FailureClass::DigestChangingRefactor));
}

// r[verify domain-primitives.typed-config.explicit-failure]
#[test]
fn first_failure_is_deterministic() {
    let mut record = super::valid_record();
    record.root = super::key(super::UNKNOWN);
    let first = admit_record(&record);
    let second = admit_record(&record);
    assert_eq!(first, second);
    assert_eq!(class(first), Some(FailureClass::MissingRoot));
}
