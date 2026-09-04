//! Finite contract, wrapper, and blame metadata.

const ARITY_LIMIT: u32 = 1_024;

/// Opaque flat-predicate identity. Predicates are never executed here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PredicateId(pub u32);

/// Opaque contract-party identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PartyId(pub u32);

/// Supported finite contract descriptors.
// r[impl scheme-logic.contract-blame]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractDescriptor {
    Flat { predicate: PredicateId },
    Function { domain_arity: u32, codomain_arity: u32 },
}

/// Wrapped callable shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallableShape {
    pub parameter_count: u32,
    pub result_count: u32,
}

/// One finite wrapper and its caller/provider parties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrapperDescriptor {
    pub contract: ContractDescriptor,
    pub callable: CallableShape,
    pub caller: PartyId,
    pub provider: PartyId,
}

/// Contract position where a failure occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailurePosition {
    Domain,
    Codomain,
}

/// Stable blame polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlamePolarity {
    Caller,
    Provider,
}

/// Admitted two-wrapper composition metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrapperComposition {
    pub inner: WrapperDescriptor,
    pub outer: WrapperDescriptor,
}

/// Stable contract and blame failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractFault {
    ArityTooLarge,
    ArityMismatch,
    PartyCollision,
    IncompatibleComposition,
    WrongBlameParty,
}

fn contract_matches(contract: ContractDescriptor, callable: CallableShape) -> Result<(), ContractFault> {
    match contract {
        ContractDescriptor::Flat { .. } => Ok(()),
        ContractDescriptor::Function {
            domain_arity,
            codomain_arity,
        } => {
            if domain_arity > ARITY_LIMIT || codomain_arity > ARITY_LIMIT {
                return Err(ContractFault::ArityTooLarge);
            }
            if domain_arity != callable.parameter_count || codomain_arity != callable.result_count {
                return Err(ContractFault::ArityMismatch);
            }
            Ok(())
        }
    }
}

/// Admits finite arity and distinct blame parties.
// r[impl scheme-logic.contract-blame]
pub fn admit_wrapper(wrapper: WrapperDescriptor) -> Result<(), ContractFault> {
    if wrapper.callable.parameter_count > ARITY_LIMIT || wrapper.callable.result_count > ARITY_LIMIT {
        return Err(ContractFault::ArityTooLarge);
    }
    if wrapper.caller == wrapper.provider {
        return Err(ContractFault::PartyCollision);
    }
    contract_matches(wrapper.contract, wrapper.callable)
}

/// Returns the fixed domain/codomain blame polarity.
// r[impl scheme-logic.contract-blame]
#[must_use]
pub const fn blame_polarity(position: FailurePosition) -> BlamePolarity {
    match position {
        FailurePosition::Domain => BlamePolarity::Caller,
        FailurePosition::Codomain => BlamePolarity::Provider,
    }
}

/// Returns the party selected by the fixed blame polarity.
// r[impl scheme-logic.contract-blame]
#[must_use]
pub const fn blamed_party(wrapper: WrapperDescriptor, position: FailurePosition) -> PartyId {
    match position {
        FailurePosition::Domain => wrapper.caller,
        FailurePosition::Codomain => wrapper.provider,
    }
}

/// Checks supplied blame metadata against the wrapper position.
// r[impl scheme-logic.contract-blame]
pub fn admit_blame(
    wrapper: WrapperDescriptor,
    position: FailurePosition,
    attributed_party: PartyId,
) -> Result<BlamePolarity, ContractFault> {
    admit_wrapper(wrapper)?;
    if blamed_party(wrapper, position) != attributed_party {
        return Err(ContractFault::WrongBlameParty);
    }
    Ok(blame_polarity(position))
}

/// Composes wrappers only across one matching callable and party boundary.
// r[impl scheme-logic.contract-blame]
pub fn compose_wrappers(
    inner: WrapperDescriptor,
    outer: WrapperDescriptor,
) -> Result<WrapperComposition, ContractFault> {
    admit_wrapper(inner)?;
    admit_wrapper(outer)?;
    if inner.callable != outer.callable || inner.provider != outer.caller {
        return Err(ContractFault::IncompatibleComposition);
    }
    Ok(WrapperComposition { inner, outer })
}
