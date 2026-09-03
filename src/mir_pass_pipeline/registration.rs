//! Canonical finite MIR pass registrations.

/// Returns the reviewed descriptor for one finite pass identity.
// r[impl mir-pass.pass-compatibility]
#[must_use]
pub const fn registered_descriptor(kind: super::model::Kind) -> super::model::Descriptor {
    match kind {
        super::model::Kind::SimplifyCfg => super::model::Descriptor {
            kind,
            input: super::model::Phase::Built,
            output: super::model::Phase::Built,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::Preserved,
            required_validation: super::model::ValidationKind::Missing,
            minimum_optimization: super::model::OptimizationLevel::O0,
        },
        super::model::Kind::RemoveStorageMarkers => super::model::Descriptor {
            kind,
            input: super::model::Phase::Built,
            output: super::model::Phase::Built,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::RequiresValidation,
            required_validation: super::model::ValidationKind::Structural,
            minimum_optimization: super::model::OptimizationLevel::O0,
        },
        super::model::Kind::PrepareConst => super::model::Descriptor {
            kind,
            input: super::model::Phase::Built,
            output: super::model::Phase::ConstPrepared,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::RequiresValidation,
            required_validation: super::model::ValidationKind::Structural,
            minimum_optimization: super::model::OptimizationLevel::O0,
        },
        super::model::Kind::PromoteTemps => super::model::Descriptor {
            kind,
            input: super::model::Phase::ConstPrepared,
            output: super::model::Phase::Promoted,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::RequiresValidation,
            required_validation: super::model::ValidationKind::Dialect,
            minimum_optimization: super::model::OptimizationLevel::O0,
        },
        super::model::Kind::ElaborateDrops => super::model::Descriptor {
            kind,
            input: super::model::Phase::Promoted,
            output: super::model::Phase::DropElaboratedConstChecked,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::RequiresValidation,
            required_validation: super::model::ValidationKind::Dialect,
            minimum_optimization: super::model::OptimizationLevel::O0,
        },
        super::model::Kind::InstCombine => super::model::Descriptor {
            kind,
            input: super::model::Phase::DropElaboratedConstChecked,
            output: super::model::Phase::Optimized,
            mutation: super::model::MutationKind::InPlace,
            structure: super::model::StructureEffect::RequiresValidation,
            required_validation: super::model::ValidationKind::Dialect,
            minimum_optimization: super::model::OptimizationLevel::O2,
        },
    }
}
