//! Positive, negative, and property evidence.

use super::*;

mod negative;
mod positive;
mod property;

const FAULT_BUDGET: u32 = 2;
const ACTIVE_PROPOSER: u32 = 0;
const CURRENT_VIEW: u64 = 9;
const BASE_LOG_LENGTH: u64 = 0;

fn replica(raw: u32) -> ReplicaId {
    ReplicaId(raw)
}

fn command_id(raw: u64) -> CommandId {
    CommandId(raw)
}

fn view(raw: u64) -> ViewId {
    ViewId(raw)
}

fn ack(replica_raw: u32, command_raw: u64, view_raw: u64) -> Acknowledgement {
    Acknowledgement {
        replica: replica(replica_raw),
        command: command_id(command_raw),
        acceleration_view: view(view_raw),
        base_view: view(view_raw),
        promised_proposers: vec![ProposerId(ACTIVE_PROPOSER)],
    }
}

fn certificate(replicas: &[u32], command_raw: u64, view_raw: u64, fault_budget: u32) -> Certificate {
    Certificate {
        acknowledgements: replicas.iter().map(|replica_raw| ack(*replica_raw, command_raw, view_raw)).collect(),
        active_proposers: vec![ProposerId(ACTIVE_PROPOSER)],
        bound_base_view: view(view_raw),
        fault_budget,
    }
}

fn five_node_certificate(command_raw: u64) -> Certificate {
    certificate(&[0, 1, 2, 3], command_raw, CURRENT_VIEW, FAULT_BUDGET)
}

fn history_operation(
    operation_id: u64,
    object: u64,
    invoke: u64,
    respond: u64,
) -> crate::consistency_history::HistoryOp {
    crate::consistency_history::HistoryOp {
        op_id: operation_id,
        kind: crate::consistency_history::OP_WRITE,
        object,
        txn: operation_id,
        session: operation_id,
        invoke,
        respond,
        observed: 0,
        value: 0,
    }
}

fn positions(offset: u32) -> Positions {
    Positions {
        receive: offset,
        proposal: offset,
        log: offset,
        execution: offset,
    }
}

fn transparent_capability() -> BaseCapability {
    BaseCapability {
        receive_to_propose_ordered: true,
        propose_to_log_ordered: true,
        log_to_execute_ordered: true,
    }
}
