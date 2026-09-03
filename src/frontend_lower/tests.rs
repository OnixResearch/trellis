//! Positive and negative HIR ID-lowering evidence.

mod negative;
mod positive;

const OWNER: u32 = 1;
const OTHER_OWNER: u32 = 2;
const SOURCE_ITEM: u32 = 10;
const SOURCE_VALUE: u32 = 11;
const UNKNOWN_SOURCE: u32 = 12;
const GENERATED_PROVENANCE: u32 = 100;
const OTHER_GENERATION: u32 = 101;
const DEFINITION: u32 = 50;
const RETURN_LOCAL: u32 = 0;
const VALUE_LOCAL: u32 = 1;
const GENERATED_LOCAL: u32 = 2;
const DISCARDED_LOCAL: u32 = 3;
const UNKNOWN_LOCAL: u32 = 9;

fn owner() -> super::model::OwnerKey {
    super::model::OwnerKey(OWNER)
}

fn node(raw: u32) -> super::model::SourceKey {
    super::model::SourceKey(raw)
}

fn scoped(local: u32) -> super::model::ScopedKey {
    super::model::ScopedKey { owner: owner(), local }
}

fn other_scoped(local: u32) -> super::model::ScopedKey {
    super::model::ScopedKey {
        owner: super::model::OwnerKey(OTHER_OWNER),
        local,
    }
}

fn mapping(source: u32, local: u32) -> super::model::Mapping {
    super::model::Mapping {
        source: node(source),
        target: scoped(local),
    }
}

fn created(local: u32, policy: super::model::UsePolicy) -> super::model::CreatedKey {
    super::model::CreatedKey {
        id: scoped(local),
        policy,
    }
}

fn valid_record() -> super::model::Record {
    super::model::Record {
        owner: owner(),
        source_nodes: vec![node(SOURCE_ITEM), node(SOURCE_VALUE)],
        mappings: vec![mapping(SOURCE_ITEM, RETURN_LOCAL), mapping(SOURCE_VALUE, VALUE_LOCAL)],
        created_ids: vec![
            created(RETURN_LOCAL, super::model::UsePolicy::Required),
            created(VALUE_LOCAL, super::model::UsePolicy::Required),
            created(GENERATED_LOCAL, super::model::UsePolicy::Required),
            created(DISCARDED_LOCAL, super::model::UsePolicy::ExplicitDiscard),
        ],
        used_ids: vec![scoped(RETURN_LOCAL), scoped(VALUE_LOCAL), scoped(GENERATED_LOCAL)],
        generated_nodes: vec![super::model::GeneratedNode {
            generation: GENERATED_PROVENANCE,
            target: scoped(GENERATED_LOCAL),
        }],
        definition_requirements: vec![node(SOURCE_ITEM)],
        definitions: vec![super::model::DefinitionEntry {
            source: node(SOURCE_ITEM),
            definition: super::model::DefinitionKey(DEFINITION),
        }],
    }
}
