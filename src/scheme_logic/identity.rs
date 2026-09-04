//! Finite Scheme interpreter-adjacent proof primitives.
//!
//! Steel inspires the selected surfaces only. This component does not parse or
//! execute Scheme, load modules, allocate runtime objects, or verify Steel.
// r[impl scheme-logic.docs]
// r[impl scheme-logic.evidence.negative]
// r[impl scheme-logic.evidence.positive]
// r[impl scheme-logic.final-validation]
// r[impl scheme-logic.non-claims]

mod boundary;
mod heap;
mod linkage;
mod proofs;
mod registry;
mod syntax;
mod vm;

pub use boundary::BlamePolarity;
pub use boundary::CallableShape;
pub use boundary::ContractDescriptor;
pub use boundary::ContractFault;
pub use boundary::FailurePosition;
pub use boundary::PartyId;
pub use boundary::PredicateId;
pub use boundary::WrapperComposition;
pub use boundary::WrapperDescriptor;
pub use boundary::admit_blame;
pub use boundary::admit_wrapper;
pub use boundary::blame_polarity;
pub use boundary::blamed_party;
pub use boundary::compose_wrappers;
pub use heap::BytevectorEnvelope;
pub use heap::CollectionFault;
pub use heap::GraphFault;
pub use heap::GraphFaultSite;
pub use heap::HashShape;
pub use heap::NodeId;
pub use heap::Traversal;
pub use heap::TraversalPurpose;
pub use heap::ValueGraph;
pub use heap::ValueNode;
pub use heap::admit_bytevector;
pub use heap::admit_hash_shape;
pub use heap::admit_values;
pub use heap::compare_values;
pub use heap::proper_list_length;
pub use heap::traverse_values;
pub use heap::update_vector;
pub use linkage::ImportClosure;
pub use linkage::ImportEdge;
pub use linkage::ModuleFault;
pub use linkage::ModuleFaultSite;
pub use linkage::ModuleGraph;
pub use linkage::ModuleId;
pub use linkage::ModuleRecord;
pub use linkage::NameId;
pub use linkage::admit_modules;
pub use linkage::import_closure;
pub use proofs::bounded_span;
pub use registry::InternEntry;
pub use registry::InternKey;
pub use registry::InternKind;
pub use registry::InternResult;
pub use registry::InternValue;
pub use registry::InternerFault;
pub use registry::InternerFaultSite;
pub use registry::InternerTable;
pub use registry::admit_interner;
pub use registry::intern;
pub use registry::resolve;
pub use syntax::Attachment;
pub use syntax::Delimiter;
pub use syntax::DelimiterPair;
pub use syntax::PrefixKind;
pub use syntax::ReaderEnvelope;
pub use syntax::ReaderFault;
pub use syntax::ReaderFaultSite;
pub use syntax::Span;
pub use syntax::SpanRelation;
pub use syntax::Token;
pub use syntax::TokenKind;
pub use syntax::admit_reader;
pub use vm::CodeEnvelope;
pub use vm::CodeFault;
pub use vm::CodeFaultSite;
pub use vm::CodeSummary;
pub use vm::ConstantPool;
pub use vm::ConstantValue;
pub use vm::Instruction;
pub use vm::PoolResult;
pub use vm::add_or_get_constant;
pub use vm::admit_code;
pub use vm::admit_pool;
pub use vm::rollback_pool;

/// Evidence claim classes at this component boundary.
// r[impl scheme-logic.non-claims]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimScope {
    FinitePredicates,
    SteelCompatibility,
    SchemeCompliance,
    ParserCompleteness,
    MacroSoundness,
    GarbageCollectorCorrectness,
    JitCorrectness,
    FfiSafety,
    RuntimeModuleLoading,
    WholeInterpreterCorrectness,
}

/// Unsupported claim promotion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimFault {
    UnsupportedPromotion,
}

/// Admits only Trellis-owned finite predicate claims.
// r[impl scheme-logic.non-claims]
pub fn admit_claim(scope: ClaimScope) -> Result<(), ClaimFault> {
    if scope == ClaimScope::FinitePredicates {
        Ok(())
    } else {
        Err(ClaimFault::UnsupportedPromotion)
    }
}

#[cfg(test)]
mod tests;
