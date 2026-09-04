//! Base ordering and bounded two-operation history composition.

/// Returns whether command A is received before command B.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn receive_before(left: &super::vocabulary::Positions, right: &super::vocabulary::Positions) -> bool {
    left.receive < right.receive
}

/// Returns whether command A is proposed before command B.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn propose_before(left: &super::vocabulary::Positions, right: &super::vocabulary::Positions) -> bool {
    left.proposal < right.proposal
}

/// Returns whether command A is logged before command B.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn log_before(left: &super::vocabulary::Positions, right: &super::vocabulary::Positions) -> bool {
    left.log < right.log
}

/// Returns whether command A executes before command B.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn execute_before(left: &super::vocabulary::Positions, right: &super::vocabulary::Positions) -> bool {
    left.execution < right.execution
}

/// Returns whether all base prerequisites hold.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn base_admits(capability: super::vocabulary::BaseCapability) -> bool {
    capability.transparent()
}

/// Checks whether receive-time acknowledgement is allowed.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn receive_time_ack_valid(capability: super::vocabulary::BaseCapability) -> bool {
    capability.receive_to_propose_ordered
}

/// Checks whether proposal-bound acknowledgement is allowed.
// r[impl consensus.fast-path.base-order]
#[must_use]
pub const fn proposal_bound_ack_valid(capability: super::vocabulary::BaseCapability) -> bool {
    capability.propose_to_log_ordered && capability.log_to_execute_ordered
}

/// Composes receive, proposal, log, and execution order.
// r[impl consensus.fast-path.base-order]
pub fn composed_execution_sequence(
    capability: super::vocabulary::BaseCapability,
    left: &super::vocabulary::Positions,
    right: &super::vocabulary::Positions,
) -> Result<bool, &'static str> {
    if !capability.receive_to_propose_ordered {
        return Err("receive-to-propose ordering is required");
    }
    if !capability.propose_to_log_ordered {
        return Err("propose-to-log ordering is required");
    }
    if !capability.log_to_execute_ordered {
        return Err("log-to-execute ordering is required");
    }
    if !receive_before(left, right) {
        return Err("left command must be received first");
    }
    if !propose_before(left, right) {
        return Err("receive order must be preserved in proposal order");
    }
    if !log_before(left, right) {
        return Err("proposal order must be preserved in log order");
    }
    if !execute_before(left, right) {
        return Err("log order must be preserved in execution order");
    }
    Ok(true)
}

/// One shared history operation plus its base-engine positions.
// r[impl consensus.fast-path.linearization]
#[derive(Clone, Copy, Debug)]
pub struct Invocation {
    pub operation: crate::consistency_history::HistoryOp,
    pub positions: super::vocabulary::Positions,
}

/// Stable or recovered-view ordering evidence.
pub enum ViewEvidence<'a> {
    Stable {
        acceleration: super::vocabulary::ViewId,
        base: super::vocabulary::ViewId,
    },
    Recovered {
        snapshot: &'a super::barrier::RecoverySnapshot,
        command: super::vocabulary::CommandId,
    },
}

/// Inputs to the bounded cross-module theorem checker.
pub struct CompositionInput<'a> {
    pub certificate: &'a super::membership::Certificate,
    pub capability: super::vocabulary::BaseCapability,
    pub state: &'a super::agreement::Store,
    pub first_command: super::vocabulary::KeyValue,
    pub second_command: super::vocabulary::KeyValue,
    pub first: &'a Invocation,
    pub second: &'a Invocation,
    pub view: ViewEvidence<'a>,
    pub record: &'a super::agreement::DedupRecord,
    pub completion: &'a super::agreement::OriginalCompletion,
}

/// Stable composition rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionFault {
    InvalidCertificate,
    InvalidHistoryOperation,
    InvalidObjectWitness,
    MissingRealtimeOrder,
    CommandHistoryMismatch,
    CommandsDoNotConflict,
    UnsoundConflictRefinement,
    IncompatibleBaseOrder,
    InvalidViewOrder,
    IdentityMismatch,
    InvalidDeduplication,
}

fn history_matches(invocation: &Invocation, command: super::vocabulary::KeyValue) -> bool {
    command.key().is_some_and(|key| invocation.operation.object == key)
}

/// Checks all premises of the bounded conflicting-operation theorem.
// r[impl consensus.fast-path.linearization]
pub fn admit_composition(input: &CompositionInput<'_>) -> Result<(), CompositionFault> {
    super::membership::admit_certificate(input.certificate).map_err(|_| CompositionFault::InvalidCertificate)?;
    if !crate::consistency_history::is_complete_op(input.first.operation)
        || !crate::consistency_history::is_complete_op(input.second.operation)
    {
        return Err(CompositionFault::InvalidHistoryOperation);
    }
    let bounded_history = [input.first.operation, input.second.operation];
    if !crate::consistency_object::linearizability(&bounded_history) {
        return Err(CompositionFault::InvalidObjectWitness);
    }
    if !crate::consistency_relations::realtime_order(input.first.operation, input.second.operation) {
        return Err(CompositionFault::MissingRealtimeOrder);
    }
    if !history_matches(input.first, input.first_command) || !history_matches(input.second, input.second_command) {
        return Err(CompositionFault::CommandHistoryMismatch);
    }
    if !super::agreement::conflicts(input.first_command, input.second_command) {
        return Err(CompositionFault::CommandsDoNotConflict);
    }
    if !super::agreement::refinement_holds(input.state, input.first_command, input.second_command) {
        return Err(CompositionFault::UnsoundConflictRefinement);
    }
    super::agreement::converge(input.record, input.completion).map_err(|_| CompositionFault::InvalidDeduplication)?;
    let certified =
        super::membership::certified_command(input.certificate).map_err(|_| CompositionFault::InvalidCertificate)?;
    if certified != input.record.command
        || certified.0 != input.first.operation.op_id
        || certified != input.completion.command
    {
        return Err(CompositionFault::IdentityMismatch);
    }
    let is_view_admitted = match input.view {
        ViewEvidence::Stable { acceleration, base } => {
            acceleration == base && base == input.certificate.bound_base_view
        }
        ViewEvidence::Recovered { snapshot, command } => {
            super::barrier::recovered_precedes_new_view(snapshot, command) && command == certified
        }
    };
    if !is_view_admitted {
        return Err(CompositionFault::InvalidViewOrder);
    }
    composed_execution_sequence(input.capability, &input.first.positions, &input.second.positions)
        .map_err(|_| CompositionFault::IncompatibleBaseOrder)?;
    Ok(())
}

/// Checks the full concrete contract before an adjacent swap.
// r[impl consensus.fast-path.linearization]
pub fn adjacent_swap_preserves(
    adjacent: bool,
    state: &super::agreement::Store,
    left: super::vocabulary::KeyValue,
    right: super::vocabulary::KeyValue,
) -> Result<bool, CompositionFault> {
    if !adjacent {
        return Err(CompositionFault::IncompatibleBaseOrder);
    }
    if super::agreement::conflicts(left, right) {
        return Err(CompositionFault::CommandsDoNotConflict);
    }
    if !super::agreement::commute_under_contract(state, left, right) {
        return Err(CompositionFault::UnsoundConflictRefinement);
    }
    Ok(true)
}
