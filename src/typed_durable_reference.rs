//! Finite typed durable reference admission primitives.
//!
//! This module models the structural admission side of Unison-style durable
//! typed values. Durable references carry opaque finite value/type/schema/code
//! identities as distinct Rust and Verus types; unpersist requests compare
//! exact type identity and require the declared dependency closure to be
//! available; schema/type migration is admitted only through an explicit
//! finite conversion witness. No storage I/O, codec, typechecker,
//! hash-strength, or runtime durability correctness is claimed.
//!
//! The identity wrappers provide domain separation only. They are not
//! cryptographic digests, collision-resistance evidence, persistence
//! evidence, codec correctness, type-checker correctness, dependency
//! retrieval, or migration execution.
// r[impl newtypes.durable-identities.non-claims]
// r[verify newtypes.durable-identities.non-claims]
// r[impl newtypes.durable-identities.docs]
// r[verify newtypes.durable-identities.docs]
// r[impl newtypes.durable-identities.validation]
// r[verify newtypes.durable-identities.validation]
use vstd::prelude::*;

verus! {

// r[impl newtypes.durable-identities.types]
// r[verify newtypes.durable-identities.types]
// r[impl newtypes.durable-identities.invalid-inputs]
/// Reference-hash identity domain. The public `raw` field keeps the complete
/// `u64` value domain, including zero, so malformed finite metadata remains
/// expressible and rejectable by admission predicates.
///
/// ```compile_fail
/// use verified_logic::typed_durable_reference::{ClosureHash, ReferenceHash};
/// // A ClosureHash cannot be supplied where a ReferenceHash is required.
/// fn needs_reference(_: ReferenceHash) {}
/// needs_reference(ClosureHash { raw: 1 });
/// ```
#[allow(
    tigerstyle::path_segment_repetition,
    reason = "the published type name preserves the durable-reference identity contract"
)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReferenceHash {
    pub raw: u64,
}

/// Value-hash identity domain. Same raw-`u64` domain contract as
/// [`ReferenceHash`]; no semantic privilege is implied by construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ValueHash {
    pub raw: u64,
}

/// Type-hash identity domain. Same raw-`u64` domain contract as
/// [`ReferenceHash`].
///
/// ```compile_fail
/// use verified_logic::typed_durable_reference::{ClosureHash, TypeHash};
/// // A ClosureHash cannot be supplied where a TypeHash is required.
/// fn needs_type(_: TypeHash) {}
/// needs_type(ClosureHash { raw: 1 });
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypeHash {
    pub raw: u64,
}

/// Codec-hash identity domain. Same raw-`u64` domain contract as
/// [`ReferenceHash`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CodecHash {
    pub raw: u64,
}

/// Closure-hash identity domain. Same raw-`u64` domain contract as
/// [`ReferenceHash`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClosureHash {
    pub raw: u64,
}

/// Schema-version identity domain. Same raw-`u64` domain contract as
/// [`ReferenceHash`].
///
/// ```compile_fail
/// use verified_logic::typed_durable_reference::{SchemaVersion, TypeHash};
/// // A SchemaVersion cannot be supplied where a TypeHash is required.
/// fn needs_type(_: TypeHash) {}
/// needs_type(SchemaVersion { raw: 1 });
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SchemaVersion {
    pub raw: u64,
}

// r[impl newtypes.durable-identities.types]
#[allow(
    tigerstyle::path_segment_repetition,
    reason = "the published type name preserves the durable-reference identity contract"
)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypedDurableReference {
    pub reference_hash: ReferenceHash,
    pub value_hash: ValueHash,
    pub type_hash: TypeHash,
    pub schema_version: SchemaVersion,
    pub codec_hash: CodecHash,
    pub closure_hash: ClosureHash,
}

// r[impl newtypes.durable-identities.types]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClosureCatalog {
    pub available_closures: Vec<ClosureHash>,
}

// r[impl newtypes.durable-identities.types]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnpersistRequest {
    pub expected_type_hash: TypeHash,
}

// r[impl newtypes.durable-identities.types]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ConversionWitness {
    pub old_reference_hash: ReferenceHash,
    pub old_type_hash: TypeHash,
    pub old_schema_version: SchemaVersion,
    pub new_reference_hash: ReferenceHash,
    pub new_type_hash: TypeHash,
    pub new_schema_version: SchemaVersion,
    pub converter_closure_hash: ClosureHash,
}

// r[depends domain-primitives.typed-durable-reference.envelope]
// r[impl newtypes.durable-identities.preservation]
pub open spec fn finite_reference_metadata_spec(reference: TypedDurableReference) -> bool {
    reference.reference_hash.raw > 0
        && reference.value_hash.raw > 0
        && reference.type_hash.raw > 0
        && reference.codec_hash.raw > 0
        && reference.closure_hash.raw > 0
}

// r[depends domain-primitives.typed-durable-reference.envelope]
// r[depends domain-primitives.typed-durable-reference.unpersist-type-match]
// r[impl newtypes.durable-identities.preservation]
pub open spec fn closure_available_spec(available: Seq<ClosureHash>, closure_hash: ClosureHash) -> bool {
    exists|i: int| 0 <= i < available.len() && available[i].raw == closure_hash.raw
}

// r[depends domain-primitives.typed-durable-reference.envelope]
pub open spec fn typed_reference_admitted_spec(reference: TypedDurableReference, catalog: ClosureCatalog) -> bool {
    finite_reference_metadata_spec(reference)
        && closure_available_spec(catalog.available_closures@, reference.closure_hash)
}

// r[depends domain-primitives.typed-durable-reference.envelope]
pub open spec fn missing_reference_closure_spec(
    reference: TypedDurableReference,
    catalog: ClosureCatalog,
) -> Option<ClosureHash> {
    if closure_available_spec(catalog.available_closures@, reference.closure_hash) {
        Option::None
    } else {
        Option::Some(reference.closure_hash)
    }
}

// r[depends domain-primitives.typed-durable-reference.unpersist-type-match]
pub open spec fn unpersist_type_matches_spec(reference: TypedDurableReference, request: UnpersistRequest) -> bool {
    reference.type_hash.raw == request.expected_type_hash.raw
}

// r[depends domain-primitives.typed-durable-reference.unpersist-type-match]
pub open spec fn unpersist_admitted_spec(
    reference: TypedDurableReference,
    request: UnpersistRequest,
    catalog: ClosureCatalog,
) -> bool {
    typed_reference_admitted_spec(reference, catalog)
        && unpersist_type_matches_spec(reference, request)
}

// r[depends domain-primitives.typed-durable-reference.migration-boundary]
pub open spec fn conversion_witness_matches_spec(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
    witness: ConversionWitness,
) -> bool {
    witness.old_reference_hash.raw == old_reference.reference_hash.raw
        && witness.old_type_hash.raw == old_reference.type_hash.raw
        && witness.old_schema_version.raw == old_reference.schema_version.raw
        && witness.new_reference_hash.raw == new_reference.reference_hash.raw
        && witness.new_type_hash.raw == new_reference.type_hash.raw
        && witness.new_schema_version.raw == new_reference.schema_version.raw
        && witness.converter_closure_hash.raw > 0
}

// r[depends domain-primitives.typed-durable-reference.migration-boundary]
pub open spec fn migration_admitted_spec(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
    witness: ConversionWitness,
    catalog: ClosureCatalog,
) -> bool {
    typed_reference_admitted_spec(old_reference, catalog)
        && typed_reference_admitted_spec(new_reference, catalog)
        && conversion_witness_matches_spec(old_reference, new_reference, witness)
        && closure_available_spec(catalog.available_closures@, witness.converter_closure_hash)
}

// r[depends domain-primitives.typed-durable-reference.migration-boundary]
pub open spec fn implicit_migration_admitted_spec(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
) -> bool {
    old_reference.type_hash.raw == new_reference.type_hash.raw
        && old_reference.schema_version.raw == new_reference.schema_version.raw
}

// r[impl domain-primitives.typed-durable-reference.envelope]
#[allow(
    tigerstyle::path_segment_repetition,
    reason = "the requirement-linked API keeps its published durable-reference name"
)]
pub fn finite_reference_metadata(reference: TypedDurableReference) -> (result: bool)
    ensures result == finite_reference_metadata_spec(reference)
{
    reference.reference_hash.raw > 0
        && reference.value_hash.raw > 0
        && reference.type_hash.raw > 0
        && reference.codec_hash.raw > 0
        && reference.closure_hash.raw > 0
}

// r[impl domain-primitives.typed-durable-reference.envelope]
// r[impl domain-primitives.typed-durable-reference.unpersist-type-match]
#[allow(
    tigerstyle::borrowed_argument_types,
    reason = "the owned Vec view stays aligned with the existing Verus Seq contract"
)]
pub fn closure_available(available: &Vec<ClosureHash>, closure_hash: ClosureHash) -> (result: bool)
    ensures result == closure_available_spec(available@, closure_hash)
{
    let mut i: usize = 0;
    while i < available.len()
        invariant
            0 <= i <= available.len(),
            forall|k: int| 0 <= k < i ==> available@[k].raw != closure_hash.raw,
        decreases available.len() - i,
    {
        if available[i].raw == closure_hash.raw {
            assert(closure_available_spec(available@, closure_hash));
            return true;
        }
        i = i + 1;
    }
    assert forall|k: int| 0 <= k < available@.len() implies available@[k].raw != closure_hash.raw by {
        assert(0 <= k < i);
    }
    assert(!closure_available_spec(available@, closure_hash));
    false
}

// r[impl domain-primitives.typed-durable-reference.envelope]
pub fn typed_reference_admitted(reference: TypedDurableReference, catalog: &ClosureCatalog) -> (result: bool)
    ensures result == typed_reference_admitted_spec(reference, *catalog)
{
    finite_reference_metadata(reference) && closure_available(&catalog.available_closures, reference.closure_hash)
}

// r[impl domain-primitives.typed-durable-reference.envelope]
#[allow(
    tigerstyle::path_segment_repetition,
    reason = "the requirement-linked API keeps its published durable-reference name"
)]
pub fn missing_reference_closure(
    reference: TypedDurableReference,
    catalog: &ClosureCatalog,
) -> (result: Option<ClosureHash>)
    ensures result == missing_reference_closure_spec(reference, *catalog)
{
    if closure_available(&catalog.available_closures, reference.closure_hash) {
        None
    } else {
        Some(reference.closure_hash)
    }
}

// r[impl domain-primitives.typed-durable-reference.unpersist-type-match]
pub fn unpersist_type_matches(reference: TypedDurableReference, request: UnpersistRequest) -> (result: bool)
    ensures result == unpersist_type_matches_spec(reference, request)
{
    reference.type_hash.raw == request.expected_type_hash.raw
}

// r[impl domain-primitives.typed-durable-reference.unpersist-type-match]
pub fn unpersist_admitted(
    reference: TypedDurableReference,
    request: UnpersistRequest,
    catalog: &ClosureCatalog,
) -> (result: bool)
    ensures result == unpersist_admitted_spec(reference, request, *catalog)
{
    typed_reference_admitted(reference, catalog) && unpersist_type_matches(reference, request)
}

// r[impl domain-primitives.typed-durable-reference.migration-boundary]
pub fn conversion_witness_matches(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
    witness: ConversionWitness,
) -> (result: bool)
    ensures result == conversion_witness_matches_spec(old_reference, new_reference, witness)
{
    witness.old_reference_hash.raw == old_reference.reference_hash.raw
        && witness.old_type_hash.raw == old_reference.type_hash.raw
        && witness.old_schema_version.raw == old_reference.schema_version.raw
        && witness.new_reference_hash.raw == new_reference.reference_hash.raw
        && witness.new_type_hash.raw == new_reference.type_hash.raw
        && witness.new_schema_version.raw == new_reference.schema_version.raw
        && witness.converter_closure_hash.raw > 0
}

// r[impl domain-primitives.typed-durable-reference.migration-boundary]
pub fn migration_admitted(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
    witness: ConversionWitness,
    catalog: &ClosureCatalog,
) -> (result: bool)
    ensures result == migration_admitted_spec(old_reference, new_reference, witness, *catalog)
{
    typed_reference_admitted(old_reference, catalog)
        && typed_reference_admitted(new_reference, catalog)
        && conversion_witness_matches(old_reference, new_reference, witness)
        && closure_available(&catalog.available_closures, witness.converter_closure_hash)
}

// r[impl domain-primitives.typed-durable-reference.migration-boundary]
pub fn implicit_migration_admitted(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
) -> (result: bool)
    ensures result == implicit_migration_admitted_spec(old_reference, new_reference)
{
    old_reference.type_hash.raw == new_reference.type_hash.raw
        && old_reference.schema_version.raw == new_reference.schema_version.raw
}

// r[verify domain-primitives.typed-durable-reference.unpersist-type-match]
// r[verify newtypes.durable-identities.preservation]
pub proof fn unpersist_requires_exact_type(
    reference: TypedDurableReference,
    request: UnpersistRequest,
    catalog: ClosureCatalog,
)
    requires unpersist_admitted_spec(reference, request, catalog)
    ensures reference.type_hash.raw == request.expected_type_hash.raw
{
}

// r[verify domain-primitives.typed-durable-reference.envelope]
pub proof fn admitted_reference_has_available_closure(
    reference: TypedDurableReference,
    catalog: ClosureCatalog,
)
    requires typed_reference_admitted_spec(reference, catalog)
    ensures closure_available_spec(catalog.available_closures@, reference.closure_hash)
{
}

// r[verify domain-primitives.typed-durable-reference.migration-boundary]
pub proof fn migration_requires_explicit_witness_match(
    old_reference: TypedDurableReference,
    new_reference: TypedDurableReference,
    witness: ConversionWitness,
    catalog: ClosureCatalog,
)
    requires migration_admitted_spec(old_reference, new_reference, witness, catalog)
    ensures conversion_witness_matches_spec(old_reference, new_reference, witness)
{
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(type_hash: u64, schema_version: u64, closure_hash: u64) -> TypedDurableReference {
        TypedDurableReference {
            reference_hash: ReferenceHash { raw: 10 + type_hash + schema_version },
            value_hash: ValueHash { raw: 20 + type_hash },
            type_hash: TypeHash { raw: type_hash },
            schema_version: SchemaVersion { raw: schema_version },
            codec_hash: CodecHash { raw: 30 },
            closure_hash: ClosureHash { raw: closure_hash },
        }
    }

    fn make_catalog(hashes: &[u64]) -> ClosureCatalog {
        ClosureCatalog {
            available_closures: hashes.iter().map(|raw| ClosureHash { raw: *raw }).collect(),
        }
    }

    fn witness_for(
        old_reference: &TypedDurableReference,
        new_reference: &TypedDurableReference,
        converter_closure_hash: u64,
    ) -> ConversionWitness {
        ConversionWitness {
            old_reference_hash: old_reference.reference_hash,
            old_type_hash: old_reference.type_hash,
            old_schema_version: old_reference.schema_version,
            new_reference_hash: new_reference.reference_hash,
            new_type_hash: new_reference.type_hash,
            new_schema_version: new_reference.schema_version,
            converter_closure_hash: ClosureHash { raw: converter_closure_hash },
        }
    }

    // r[verify domain-primitives.typed-durable-reference.envelope]
    // r[verify newtypes.durable-identities.evidence]
    #[test]
    fn admits_well_formed_reference_and_reports_missing_closure() {
        let catalog = make_catalog(&[7, 9]);
        let admitted = reference(100, 1, 7);
        let missing = reference(100, 1, 8);

        assert!(typed_reference_admitted(admitted, &catalog));
        assert_eq!(missing_reference_closure(admitted, &catalog), None);
        assert!(!typed_reference_admitted(missing, &catalog));
        assert_eq!(missing_reference_closure(missing, &catalog), Some(ClosureHash { raw: 8 }));
    }

    // r[verify domain-primitives.typed-durable-reference.envelope]
    // r[verify newtypes.durable-identities.invalid-inputs]
    #[test]
    fn rejects_inconsistent_zero_metadata() {
        let catalog = make_catalog(&[7]);
        let mut zero_type = reference(100, 1, 7);
        zero_type.type_hash = TypeHash { raw: 0 };
        let mut zero_reference = reference(100, 1, 7);
        zero_reference.reference_hash = ReferenceHash { raw: 0 };
        let mut zero_closure = reference(100, 1, 7);
        zero_closure.closure_hash = ClosureHash { raw: 0 };
        let mut zero_codec = reference(100, 1, 7);
        zero_codec.codec_hash = CodecHash { raw: 0 };

        assert!(!finite_reference_metadata(zero_type));
        assert!(!typed_reference_admitted(zero_type, &catalog));
        assert!(!finite_reference_metadata(zero_reference));
        assert!(!finite_reference_metadata(zero_closure));
        assert!(!finite_reference_metadata(zero_codec));
        // Schema version participates in migration comparison but not in
        // finite-metadata admission, matching the pre-migration model.
        assert!(typed_reference_admitted(reference(100, 0, 7), &catalog));
    }

    // r[verify domain-primitives.typed-durable-reference.unpersist-type-match]
    // r[verify newtypes.durable-identities.preservation]
    #[test]
    fn unpersist_requires_type_match_and_available_closure() {
        let catalog = make_catalog(&[7]);
        let unavailable = make_catalog(&[9]);
        let durable = reference(100, 1, 7);

        assert!(unpersist_admitted(
            durable,
            UnpersistRequest { expected_type_hash: TypeHash { raw: 100 } },
            &catalog,
        ));
        assert!(!unpersist_admitted(
            durable,
            UnpersistRequest { expected_type_hash: TypeHash { raw: 101 } },
            &catalog,
        ));
        assert!(!unpersist_admitted(
            durable,
            UnpersistRequest { expected_type_hash: TypeHash { raw: 100 } },
            &unavailable,
        ));
    }

    // r[verify domain-primitives.typed-durable-reference.migration-boundary]
    // r[verify newtypes.durable-identities.preservation]
    #[test]
    fn migration_requires_explicit_matching_witness() {
        let catalog = make_catalog(&[7, 8, 99]);
        let old_reference = reference(100, 1, 7);
        let new_reference = reference(200, 2, 8);
        let witness = witness_for(&old_reference, &new_reference, 99);
        let mut wrong_witness = witness;
        wrong_witness.new_type_hash = TypeHash { raw: 201 };

        assert!(migration_admitted(old_reference, new_reference, witness, &catalog));
        assert!(!migration_admitted(old_reference, new_reference, wrong_witness, &catalog));
        assert!(!implicit_migration_admitted(old_reference, new_reference));
        assert!(implicit_migration_admitted(old_reference, reference(100, 1, 8)));
    }

    // r[impl newtypes.durable-identities.evidence]
    // r[verify newtypes.durable-identities.evidence]
    #[test]
    fn compile_fail_doctests_cover_cross_domain_substitution() {
        // The compile_fail doctests on ReferenceHash, TypeHash, and
        // SchemaVersion prove that cross-domain identity substitution fails
        // to compile; this test asserts those compile-fail segments exist in
        // the public docs so the evidence cannot silently disappear.
        let source = include_str!("typed_durable_reference.rs");
        let compile_fail_blocks = source.matches("```compile_fail").count();
        assert!(compile_fail_blocks >= 3);
        assert!(source.contains("needs_reference(ClosureHash { raw: 1 })"));
        assert!(source.contains("needs_type(ClosureHash { raw: 1 })"));
        assert!(source.contains("needs_type(SchemaVersion { raw: 1 })"));
    }
}
