//! Shared identities, commands, positions, and supported-scope vocabulary.

/// One crash-fault replica identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReplicaId(pub u32);

/// One canonical command identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandId(pub u64);

/// One acceleration or base view identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewId(pub u64);

/// One original-path proposer identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProposerId(pub u32);

/// Fault models presented to boundary admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultModel {
    Crash,
    Byzantine,
}

/// Membership modes presented to boundary admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MembershipMode {
    Static,
    Dynamic,
}

/// Command shapes presented to boundary admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandMode {
    Single,
    Interactive,
    CrossGroup,
}

/// Declared capability scope for one theorem application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapabilityScope {
    pub fault_model: FaultModel,
    pub membership: MembershipMode,
    pub command_mode: CommandMode,
}

/// Stable unsupported-scope classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeFault {
    ByzantineFaults,
    DynamicMembership,
    InteractiveTransaction,
    CrossGroupAtomicity,
}

/// Admits only the initial static crash-fault, single-command slice.
// r[impl consensus.fast-path.boundary]
pub const fn admit_scope(scope: CapabilityScope) -> Result<(), ScopeFault> {
    if matches!(scope.fault_model, FaultModel::Byzantine) {
        return Err(ScopeFault::ByzantineFaults);
    }
    if matches!(scope.membership, MembershipMode::Dynamic) {
        return Err(ScopeFault::DynamicMembership);
    }
    match scope.command_mode {
        CommandMode::Single => Ok(()),
        CommandMode::Interactive => Err(ScopeFault::InteractiveTransaction),
        CommandMode::CrossGroup => Err(ScopeFault::CrossGroupAtomicity),
    }
}

/// Bound key/value commands used by the concrete conflict refinement.
// r[impl consensus.fast-path.conflict]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyValue {
    Read { key: u64 },
    Write { key: u64, value: u64 },
    Delete { key: u64 },
    Unsupported { effect: u64 },
}

impl KeyValue {
    /// Returns the single bound key when the command has known key scope.
    #[must_use]
    pub const fn key(self) -> Option<u64> {
        match self {
            Self::Read { key } | Self::Write { key, .. } | Self::Delete { key } => Some(key),
            Self::Unsupported { .. } => None,
        }
    }

    /// Returns whether the known command mutates the bound store.
    #[must_use]
    pub const fn is_write(self) -> bool {
        matches!(self, Self::Write { .. } | Self::Delete { .. })
    }
}

/// Bound client-facing response values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response {
    Value(Option<u64>),
    Ack,
    Rejected,
}

/// Receive, proposal, log, and execution positions.
// r[impl consensus.fast-path.base-order]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Positions {
    pub receive: u32,
    pub proposal: u32,
    pub log: u32,
    pub execution: u32,
}

/// Declared base-engine ordering capability.
// r[impl consensus.fast-path.base-order]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseCapability {
    pub receive_to_propose_ordered: bool,
    pub propose_to_log_ordered: bool,
    pub log_to_execute_ordered: bool,
}

impl BaseCapability {
    /// Returns whether all ordering prerequisites hold.
    #[must_use]
    pub const fn transparent(self) -> bool {
        self.receive_to_propose_ordered && self.propose_to_log_ordered && self.log_to_execute_ordered
    }
}
