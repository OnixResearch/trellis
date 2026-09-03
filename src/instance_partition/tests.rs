//! Positive and negative concrete-item collection evidence.

mod negative;
mod positive;

const ROOT_ITEM: u32 = 1;
const GENERIC_ITEM: u32 = 2;
const STATIC_ITEM: u32 = 3;
const SHIM_ITEM: u32 = 4;
const UNKNOWN_ITEM: u32 = 9;
const TYPE_ARGUMENT: u64 = 10;
const STABLE_UNIT: u32 = 100;
const VOLATILE_UNIT: u32 = 101;
const ITEM_COUNT: u32 = 4;
const ITEM_COUNT_USIZE: usize = 4;
const ROOT_COUNT: u32 = 2;
const COLLECTED_COUNT: u32 = 4;
const UNIT_COUNT: u32 = 2;

fn key(raw: u32) -> super::model::ItemKey {
    super::model::ItemKey(raw)
}

fn item(
    raw: u32,
    kind: super::model::ItemKind,
    shape: super::model::ConcreteShape,
    stability: super::model::Stability,
) -> super::model::Item {
    let type_arguments = if shape == super::model::ConcreteShape::Generic {
        vec![TYPE_ARGUMENT]
    } else {
        Vec::new()
    };
    let shim_source = if kind == super::model::ItemKind::Shim {
        Some(key(GENERIC_ITEM))
    } else {
        None
    };
    super::model::Item {
        key: key(raw),
        kind,
        shape,
        type_arguments,
        const_arguments: Vec::new(),
        shim_source,
        origin: super::model::Origin::Local,
        stability,
    }
}

fn valid_record() -> super::model::Record {
    super::model::Record {
        items: vec![
            item(
                ROOT_ITEM,
                super::model::ItemKind::Function,
                super::model::ConcreteShape::Plain,
                super::model::Stability::Stable,
            ),
            item(
                GENERIC_ITEM,
                super::model::ItemKind::Function,
                super::model::ConcreteShape::Generic,
                super::model::Stability::Volatile,
            ),
            item(
                STATIC_ITEM,
                super::model::ItemKind::Static,
                super::model::ConcreteShape::Static,
                super::model::Stability::Stable,
            ),
            item(
                SHIM_ITEM,
                super::model::ItemKind::Shim,
                super::model::ConcreteShape::Shim,
                super::model::Stability::Volatile,
            ),
        ],
        roots: vec![key(ROOT_ITEM), key(ROOT_ITEM)],
        references: vec![
            super::model::Reference {
                source: key(ROOT_ITEM),
                target: key(GENERIC_ITEM),
            },
            super::model::Reference {
                source: key(ROOT_ITEM),
                target: key(STATIC_ITEM),
            },
            super::model::Reference {
                source: key(GENERIC_ITEM),
                target: key(SHIM_ITEM),
            },
            super::model::Reference {
                source: key(ROOT_ITEM),
                target: key(SHIM_ITEM),
            },
        ],
        collected: vec![key(ROOT_ITEM), key(GENERIC_ITEM), key(STATIC_ITEM), key(SHIM_ITEM)],
        units: vec![
            super::model::Unit {
                key: super::model::UnitKey(STABLE_UNIT),
                stability: super::model::Stability::Stable,
                items: vec![key(ROOT_ITEM), key(STATIC_ITEM)],
            },
            super::model::Unit {
                key: super::model::UnitKey(VOLATILE_UNIT),
                stability: super::model::Stability::Volatile,
                items: vec![key(GENERIC_ITEM), key(SHIM_ITEM)],
            },
        ],
    }
}
