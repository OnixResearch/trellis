use super::super::*;

// r[verify mir-drop.obligations]
// r[verify mir-drop.admission]
// r[verify mir-drop.evidence]
// r[verify mir-drop.docs]
// r[verify mir-drop.final-validation]
#[test]
fn path_and_type_metadata_are_admitted() {
    assert_eq!(
        admit_record(&super::record()),
        Ok(Facts {
            path_count: super::PATH_COUNT,
            metadata_count: super::PATH_COUNT,
        })
    );
}

// r[verify mir-drop.obligations]
// r[verify mir-drop.initializedness]
// r[verify mir-drop.evidence]
#[test]
fn initialize_establishes_and_move_releases_subtree_obligations() {
    let empty = State {
        maybe_initialized: Vec::new(),
        maybe_uninitialized: Vec::new(),
        obligations: Vec::new(),
    };
    let initialized = apply_action(&super::record(), &empty, Action::Initialize(super::key(super::ROOT)));
    assert_eq!(
        initialized,
        Ok(State {
            maybe_initialized: vec![
                super::key(super::ROOT),
                super::key(super::FIRST_FIELD),
                super::key(super::SECOND_FIELD),
            ],
            maybe_uninitialized: Vec::new(),
            obligations: vec![
                super::key(super::ROOT),
                super::key(super::FIRST_FIELD),
                super::key(super::SECOND_FIELD),
            ],
        })
    );
    let moved = initialized
        .and_then(|state| apply_action(&super::record(), &state, Action::Move(super::key(super::FIRST_FIELD))));
    assert!(matches!(
        moved,
        Ok(State {
            maybe_initialized,
            maybe_uninitialized,
            obligations,
        }) if !maybe_initialized.contains(&super::key(super::FIRST_FIELD))
            && maybe_uninitialized.contains(&super::key(super::FIRST_FIELD))
            && !obligations.contains(&super::key(super::FIRST_FIELD))
    ));
}

// r[verify mir-drop.classification]
// r[verify mir-drop.evidence]
#[test]
fn finite_facts_select_all_four_classes() {
    let record = super::record();
    let open = super::open_struct_state();
    assert_eq!(
        classify(&record, &open, super::key(super::FIRST_FIELD), ConditionalPolicy::OpenDescendants,),
        Ok(Classification::Static)
    );
    assert_eq!(
        classify(&record, &open, super::key(super::SECOND_FIELD), ConditionalPolicy::OpenDescendants,),
        Ok(Classification::Dead)
    );
    assert_eq!(
        classify(&record, &open, super::key(super::ROOT), ConditionalPolicy::OpenDescendants,),
        Ok(Classification::Open)
    );
    assert_eq!(
        classify(
            &record,
            &State {
                maybe_initialized: vec![super::key(super::FIRST_FIELD)],
                maybe_uninitialized: vec![super::key(super::FIRST_FIELD)],
                obligations: vec![super::key(super::FIRST_FIELD)],
            },
            super::key(super::FIRST_FIELD),
            ConditionalPolicy::WholeTarget,
        ),
        Ok(Classification::Conditional)
    );
}

// r[verify mir-drop.drop-flags]
// r[verify mir-drop.evidence]
#[test]
fn conditional_target_emits_guarded_metadata() {
    let state = State {
        maybe_initialized: vec![super::key(super::FIRST_FIELD)],
        maybe_uninitialized: vec![super::key(super::FIRST_FIELD)],
        obligations: vec![super::key(super::FIRST_FIELD)],
    };
    let mut request = super::request(super::FIRST_FIELD, Vec::new());
    request.policy = ConditionalPolicy::WholeTarget;
    assert_eq!(
        expand(&super::record(), &state, &request),
        Ok(vec![Step::Guarded {
            path: super::key(super::FIRST_FIELD),
            flag: FlagKey(super::FIRST_FIELD),
        }])
    );
}

// r[verify mir-drop.open-drop]
// r[verify mir-drop.evidence]
#[test]
fn open_struct_emits_only_initialized_child() {
    assert_eq!(
        expand(&super::record(), &super::open_struct_state(), &super::request(super::ROOT, Vec::new()),),
        Ok(vec![Step::Call {
            path: super::key(super::FIRST_FIELD),
        }])
    );
}

// r[verify mir-drop.open-drop]
// r[verify mir-drop.evidence]
#[test]
fn open_enum_uses_only_active_variant_children() {
    let active = vec![ActiveVariant {
        path: super::key(super::ENUM_ROOT),
        variant: super::FIRST_VARIANT,
    }];
    assert_eq!(
        expand(&super::record(), &super::open_enum_state(), &super::request(super::ENUM_ROOT, active),),
        Ok(vec![Step::Call {
            path: super::key(super::FIRST_VARIANT_FIELD),
        }])
    );
}

// r[verify mir-drop.drop-and-replace]
// r[verify mir-drop.evidence]
#[test]
fn old_target_steps_precede_replacement_write() {
    let request = ReplacementRequest {
        expansion: super::request(super::ROOT, Vec::new()),
        replacement: ValueKey(super::REPLACEMENT),
    };
    assert_eq!(
        split_replace(&super::record(), &super::open_struct_state(), &request),
        Ok(vec![
            Step::Call {
                path: super::key(super::FIRST_FIELD),
            },
            Step::Write {
                path: super::key(super::ROOT),
                value: ValueKey(super::REPLACEMENT),
            },
        ])
    );
}

// r[verify mir-drop.non-claims]
#[test]
fn no_glue_path_emits_no_destructor_metadata() {
    let state = State {
        maybe_initialized: vec![super::key(super::NO_GLUE)],
        maybe_uninitialized: Vec::new(),
        obligations: Vec::new(),
    };
    assert_eq!(expand(&super::record(), &state, &super::request(super::NO_GLUE, Vec::new()),), Ok(Vec::new()));
}
