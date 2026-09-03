use super::super::*;

const INVALID_DEPTH: u32 = 2;
const INVALID_VARIANT: u32 = 9;
const SMALL_BUDGET: u32 = 1;

fn record_class(result: Result<Facts, Rejection>) -> Option<FailureClass> {
    result.err().map(|error| error.class)
}

// r[verify mir-drop.admission]
// r[verify mir-drop.evidence]
#[test]
fn invalid_mir_place_is_rejected() {
    let mut record = super::record();
    for path in &mut record.paths {
        if path.key == super::key(super::ROOT) {
            path.place = super::place(super::UNKNOWN, None);
        }
    }
    assert_eq!(record_class(admit_record(&record)), Some(FailureClass::InvalidPath));
}

// r[verify mir-drop.admission]
// r[verify mir-drop.evidence]
#[test]
fn missing_parent_and_wrong_depth_are_rejected() {
    let mut missing_parent = super::record();
    for path in &mut missing_parent.paths {
        if path.key == super::key(super::FIRST_FIELD) {
            path.parent = Some(super::key(super::UNKNOWN));
        }
    }
    assert_eq!(record_class(admit_record(&missing_parent)), Some(FailureClass::InvalidParent));

    let mut wrong_depth = super::record();
    for path in &mut wrong_depth.paths {
        if path.key == super::key(super::FIRST_FIELD) {
            path.depth = INVALID_DEPTH;
        }
    }
    assert_eq!(record_class(admit_record(&wrong_depth)), Some(FailureClass::InvalidDepth));
}

// r[verify mir-drop.obligations]
// r[verify mir-drop.evidence]
#[test]
fn unsupported_or_duplicate_metadata_is_rejected() {
    let mut unsupported = super::record();
    for metadata in &mut unsupported.metadata {
        if metadata.path == super::key(super::ROOT) {
            metadata.shape = Shape::Unsupported;
        }
    }
    assert_eq!(record_class(admit_record(&unsupported)), Some(FailureClass::UnsupportedMetadata));

    let mut duplicate = super::record();
    duplicate.metadata.push(Metadata {
        path: super::key(super::ROOT),
        shape: Shape::Direct,
    });
    assert_eq!(record_class(admit_record(&duplicate)), Some(FailureClass::DuplicateMetadata));
}

// r[verify mir-drop.initializedness]
// r[verify mir-drop.evidence]
#[test]
fn duplicate_fact_is_rejected() {
    let state = State {
        maybe_initialized: vec![super::key(super::ROOT), super::key(super::ROOT)],
        maybe_uninitialized: Vec::new(),
        obligations: vec![super::key(super::ROOT)],
    };
    assert_eq!(
        admit_state(&super::record(), &state).err().map(|error| error.class),
        Some(FailureClass::DuplicateFact)
    );
}

// r[verify mir-drop.obligations]
// r[verify mir-drop.evidence]
#[test]
fn no_glue_path_cannot_be_an_obligation() {
    let state = State {
        maybe_initialized: vec![super::key(super::NO_GLUE)],
        maybe_uninitialized: Vec::new(),
        obligations: vec![super::key(super::NO_GLUE)],
    };
    assert_eq!(
        admit_state(&super::record(), &state).err().map(|error| error.class),
        Some(FailureClass::InvalidObligation)
    );
}

// r[verify mir-drop.classification]
// r[verify mir-drop.evidence]
#[test]
fn absent_initializedness_facts_are_rejected() {
    let state = State {
        maybe_initialized: Vec::new(),
        maybe_uninitialized: Vec::new(),
        obligations: Vec::new(),
    };
    assert_eq!(
        classify(&super::record(), &state, super::key(super::ROOT), ConditionalPolicy::OpenDescendants,)
            .err()
            .map(|error| error.class),
        Some(FailureClass::InconsistentFacts)
    );
}

// r[verify mir-drop.open-drop]
// r[verify mir-drop.evidence]
#[test]
fn open_enum_requires_one_valid_active_variant() {
    let missing = super::request(super::ENUM_ROOT, Vec::new());
    assert_eq!(
        expand(&super::record(), &super::open_enum_state(), &missing).err().map(|error| error.class),
        Some(FailureClass::MissingActiveVariant)
    );

    let invalid = super::request(super::ENUM_ROOT, vec![ActiveVariant {
        path: super::key(super::ENUM_ROOT),
        variant: INVALID_VARIANT,
    }]);
    assert_eq!(
        expand(&super::record(), &super::open_enum_state(), &invalid).err().map(|error| error.class),
        Some(FailureClass::InvalidActiveVariant)
    );
}

// r[verify mir-drop.open-drop]
#[test]
fn duplicate_active_variant_fact_is_rejected() {
    let active = ActiveVariant {
        path: super::key(super::ENUM_ROOT),
        variant: super::FIRST_VARIANT,
    };
    let request = super::request(super::ENUM_ROOT, vec![active, active]);
    assert_eq!(
        expand(&super::record(), &super::open_enum_state(), &request).err().map(|error| error.class),
        Some(FailureClass::InvalidActiveVariant)
    );
}

// r[verify mir-drop.open-drop]
// r[verify mir-drop.evidence]
#[test]
fn expansion_budget_exhaustion_is_rejected() {
    let mut request = super::request(super::ROOT, Vec::new());
    request.budget = SMALL_BUDGET;
    assert_eq!(
        expand(&super::record(), &super::open_struct_state(), &request).err().map(|error| error.class),
        Some(FailureClass::BudgetExhausted)
    );
}

// r[verify mir-drop.drop-and-replace]
// r[verify mir-drop.evidence]
#[test]
fn invalid_replacement_target_is_rejected_before_write() {
    let request = ReplacementRequest {
        expansion: super::request(super::UNKNOWN, Vec::new()),
        replacement: ValueKey(super::REPLACEMENT),
    };
    assert_eq!(
        split_replace(&super::record(), &super::open_struct_state(), &request)
            .err()
            .map(|error| error.class),
        Some(FailureClass::InvalidPath)
    );
}

// r[verify mir-drop.admission]
#[test]
fn first_failure_is_deterministic() {
    let mut record = super::record();
    for path in &mut record.paths {
        if path.key == super::key(super::ROOT) {
            path.place = super::place(super::UNKNOWN, None);
        }
    }
    let first = admit_record(&record);
    let second = admit_record(&record);
    assert_eq!(first, second);
    assert_eq!(record_class(first), Some(FailureClass::InvalidPath));
}
