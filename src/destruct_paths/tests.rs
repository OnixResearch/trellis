//! Positive and negative finite destruction-decision evidence.

mod negative;
mod positive;

const ROOT: u32 = 0;
const FIRST_FIELD: u32 = 1;
const SECOND_FIELD: u32 = 2;
const ENUM_ROOT: u32 = 3;
const FIRST_VARIANT_FIELD: u32 = 4;
const SECOND_VARIANT_FIELD: u32 = 5;
const NO_GLUE: u32 = 6;
const UNKNOWN: u32 = 9;
const PATH_COUNT: u32 = 7;
const FIRST_VARIANT: u32 = 0;
const SECOND_VARIANT: u32 = 1;
const REPLACEMENT: u32 = 20;
const EXPANSION_BUDGET: u32 = 8;

fn key(raw: u32) -> super::model::PathKey {
    super::model::PathKey(raw)
}

fn place(local: u32, field: Option<u32>) -> crate::mir_core::Place {
    let projections = field.map_or_else(Vec::new, |field| vec![crate::mir_core::Projection::Field { field }]);
    crate::mir_core::Place { local, projections }
}

fn body() -> crate::mir_core::Body {
    crate::mir_core::Body {
        locals: vec![
            crate::mir_core::LocalDecl {
                role: crate::mir_core::LocalRole::Return,
                type_tag: crate::mir_core::TypeTag::Aggregate,
            },
            crate::mir_core::LocalDecl {
                role: crate::mir_core::LocalRole::Temporary,
                type_tag: crate::mir_core::TypeTag::Aggregate,
            },
        ],
        blocks: vec![crate::mir_core::BasicBlock {
            statements: Vec::new(),
            terminator: Some(crate::mir_core::Terminator::Return),
        }],
    }
}

fn path(raw: u32, local: u32, field: Option<u32>, parent: Option<u32>, depth: u32) -> super::model::Path {
    super::model::Path {
        key: key(raw),
        place: place(local, field),
        parent: parent.map(key),
        depth,
    }
}

fn record() -> super::model::Record {
    super::model::Record {
        body: body(),
        paths: vec![
            path(ROOT, 0, None, None, 0),
            path(FIRST_FIELD, 1, Some(0), Some(ROOT), 1),
            path(SECOND_FIELD, 1, Some(1), Some(ROOT), 1),
            path(ENUM_ROOT, 0, None, None, 0),
            path(FIRST_VARIANT_FIELD, 1, Some(0), Some(ENUM_ROOT), 1),
            path(SECOND_VARIANT_FIELD, 1, Some(1), Some(ENUM_ROOT), 1),
            path(NO_GLUE, 1, None, None, 0),
        ],
        metadata: vec![
            super::model::Metadata {
                path: key(ROOT),
                shape: super::model::Shape::Fields(vec![key(FIRST_FIELD), key(SECOND_FIELD)]),
            },
            super::model::Metadata {
                path: key(FIRST_FIELD),
                shape: super::model::Shape::Direct,
            },
            super::model::Metadata {
                path: key(SECOND_FIELD),
                shape: super::model::Shape::Direct,
            },
            super::model::Metadata {
                path: key(ENUM_ROOT),
                shape: super::model::Shape::Enum(vec![
                    super::model::VariantFields {
                        variant: FIRST_VARIANT,
                        children: vec![key(FIRST_VARIANT_FIELD)],
                    },
                    super::model::VariantFields {
                        variant: SECOND_VARIANT,
                        children: vec![key(SECOND_VARIANT_FIELD)],
                    },
                ]),
            },
            super::model::Metadata {
                path: key(FIRST_VARIANT_FIELD),
                shape: super::model::Shape::Direct,
            },
            super::model::Metadata {
                path: key(SECOND_VARIANT_FIELD),
                shape: super::model::Shape::Direct,
            },
            super::model::Metadata {
                path: key(NO_GLUE),
                shape: super::model::Shape::NoGlue,
            },
        ],
    }
}

fn open_struct_state() -> super::model::State {
    super::model::State {
        maybe_initialized: vec![key(ROOT), key(FIRST_FIELD)],
        maybe_uninitialized: vec![key(ROOT), key(SECOND_FIELD)],
        obligations: vec![key(ROOT), key(FIRST_FIELD)],
    }
}

fn open_enum_state() -> super::model::State {
    super::model::State {
        maybe_initialized: vec![key(ENUM_ROOT), key(FIRST_VARIANT_FIELD)],
        maybe_uninitialized: vec![key(ENUM_ROOT), key(SECOND_VARIANT_FIELD)],
        obligations: vec![key(ENUM_ROOT), key(FIRST_VARIANT_FIELD)],
    }
}

fn request(target: u32, active_variants: Vec<super::model::ActiveVariant>) -> super::model::ExpansionRequest {
    super::model::ExpansionRequest {
        target: key(target),
        policy: super::model::ConditionalPolicy::OpenDescendants,
        active_variants,
        budget: EXPANSION_BUDGET,
    }
}
