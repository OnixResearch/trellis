//! Positive and negative typed normal-form evidence.

mod negative;
mod positive;

const ROOT: u32 = 0;
const ENABLED: u32 = 1;
const NAME: u32 = 2;
const ITEMS: u32 = 3;
const FIRST_ITEM: u32 = 4;
const SECOND_ITEM: u32 = 5;
const OPTIONAL: u32 = 6;
const VARIANT: u32 = 7;
const REFERENCE: u32 = 8;
const IMPORT: u32 = 9;
const UNKNOWN: u32 = 20;
const AUTHORITY: u32 = 7;
const OTHER_AUTHORITY: u32 = 8;
const NODE_COUNT: u32 = 10;
const FIELD_COUNT: u32 = 2;
const IMPORT_COUNT: u32 = 1;
const FIRST_DIGEST_BYTE: u8 = 1;
const SECOND_DIGEST_BYTE: u8 = 2;

fn key(raw: u32) -> super::model::NodeKey {
    super::model::NodeKey(raw)
}

fn digest(byte: u8) -> super::model::Digest {
    super::model::Digest([byte; super::model::DIGEST_BYTES])
}

fn node(raw: u32, type_tag: super::model::TypeTag, kind: super::model::Kind) -> super::model::Node {
    super::model::Node {
        key: key(raw),
        declared_type: type_tag,
        kind,
        source_label: format!("source-{raw}"),
        comment: "review note".to_owned(),
    }
}

fn import_edge() -> super::model::ImportEdge {
    super::model::ImportEdge {
        source: super::model::Origin::Remote { authority: AUTHORITY },
        target: super::model::Target::Relative { authority: AUTHORITY },
        selection: super::model::Selection::Static,
        headers: super::model::HeaderPolicy::Forward,
        declared: digest(FIRST_DIGEST_BYTE),
        observed: digest(FIRST_DIGEST_BYTE),
    }
}

fn root_fields() -> Vec<super::model::Field> {
    vec![
        super::model::Field {
            name: "name".to_owned(),
            value: key(NAME),
        },
        super::model::Field {
            name: "enabled".to_owned(),
            value: key(ENABLED),
        },
        super::model::Field {
            name: "items".to_owned(),
            value: key(ITEMS),
        },
        super::model::Field {
            name: "optional".to_owned(),
            value: key(OPTIONAL),
        },
        super::model::Field {
            name: "variant".to_owned(),
            value: key(VARIANT),
        },
        super::model::Field {
            name: "reference".to_owned(),
            value: key(REFERENCE),
        },
        super::model::Field {
            name: "import".to_owned(),
            value: key(IMPORT),
        },
    ]
}

fn valid_record() -> super::model::Record {
    super::model::Record {
        root: key(ROOT),
        nodes: vec![
            node(ROOT, super::model::TypeTag::Record, super::model::Kind::Record(root_fields())),
            node(ENABLED, super::model::TypeTag::Bool, super::model::Kind::Scalar(super::model::Scalar::Bool(true))),
            node(
                NAME,
                super::model::TypeTag::Text,
                super::model::Kind::Scalar(super::model::Scalar::Text("demo".to_owned())),
            ),
            node(
                ITEMS,
                super::model::TypeTag::List,
                super::model::Kind::List(vec![key(FIRST_ITEM), key(SECOND_ITEM)]),
            ),
            node(
                FIRST_ITEM,
                super::model::TypeTag::Natural,
                super::model::Kind::Scalar(super::model::Scalar::Natural(1)),
            ),
            node(
                SECOND_ITEM,
                super::model::TypeTag::Natural,
                super::model::Kind::Scalar(super::model::Scalar::Natural(2)),
            ),
            node(OPTIONAL, super::model::TypeTag::Optional, super::model::Kind::Optional(None)),
            node(VARIANT, super::model::TypeTag::Variant, super::model::Kind::Variant {
                tag: "left".to_owned(),
                value: Some(key(NAME)),
            }),
            node(REFERENCE, super::model::TypeTag::Reference, super::model::Kind::Reference(key(NAME))),
            node(IMPORT, super::model::TypeTag::Import, super::model::Kind::Import { edge: 0 }),
        ],
        required_fields: vec![
            super::model::RequiredField {
                record: key(ROOT),
                name: "enabled".to_owned(),
                expected: super::model::TypeTag::Bool,
            },
            super::model::RequiredField {
                record: key(ROOT),
                name: "name".to_owned(),
                expected: super::model::TypeTag::Text,
            },
        ],
        imports: vec![import_edge()],
        dependency_digests: vec![digest(FIRST_DIGEST_BYTE)],
    }
}
