use super::super::*;

const FIRST_COMMAND: u64 = 1;
const FIRST_KEY: u64 = 5;
const SECOND_KEY: u64 = 40;

// r[verify consensus.fast-path.quorum]
// r[verify consensus.fast-path.fixtures]
#[test]
fn three_and_five_replica_profiles_match_formulas() {
    assert_eq!(cluster_size(1), Some(3));
    assert_eq!(majority(1), Some(2));
    assert_eq!(accelerated_quorum(1), Some(3));
    assert_eq!(accelerated_failure_tolerance(1), Some(0));

    assert_eq!(cluster_size(2), Some(5));
    assert_eq!(majority(2), Some(3));
    assert_eq!(accelerated_quorum(2), Some(4));
    assert_eq!(accelerated_failure_tolerance(2), Some(1));
    assert_eq!(recovery_support(2), Some(2));
    assert_eq!(intersection_lower_bound(2), Some(2));
}

// r[verify consensus.fast-path.certificate]
// r[verify consensus.fast-path.fixtures]
#[test]
fn same_view_certificate_exposes_structural_facts() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    assert_eq!(admit_certificate(&certificate), Ok(()));
    assert_eq!(certified_command(&certificate), Ok(super::command_id(FIRST_COMMAND)));
    assert_eq!(replica_set(&certificate).map(|set| set.len()), Ok(4));
}

// r[verify consensus.fast-path.quorum]
// r[verify consensus.fast-path.fixtures]
#[test]
fn recovery_majority_observes_support_threshold() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    let replies = vec![super::replica(0), super::replica(1), super::replica(4)];
    assert_eq!(recovery_visibility(&certificate, &replies), Ok(2));
}

// r[verify consensus.fast-path.base-order]
// r[verify consensus.fast-path.fixtures]
#[test]
fn prerequisites_compose_through_log_order() {
    let capability = super::transparent_capability();
    let first = super::positions(0);
    let second = super::positions(1);
    assert!(base_admits(capability));
    assert!(receive_before(&first, &second));
    assert!(propose_before(&first, &second));
    assert!(log_before(&first, &second));
    assert!(execute_before(&first, &second));
    assert_eq!(composed_execution_sequence(capability, &first, &second), Ok(true));
    assert!(receive_time_ack_valid(capability));
    assert!(proposal_bound_ack_valid(capability));
}

// r[verify consensus.fast-path.conflict]
// r[verify consensus.fast-path.fixtures]
#[test]
fn disjoint_writes_commute_state_and_responses() {
    let store = Store::default();
    let left = KeyValue::Write {
        key: FIRST_KEY,
        value: 10,
    };
    let right = KeyValue::Write {
        key: SECOND_KEY,
        value: 20,
    };
    assert!(!conflicts(left, right));
    assert!(commute_under_contract(&store, left, right));
    assert!(refinement_holds(&store, left, right));
    assert_eq!(adjacent_swap_preserves(true, &store, left, right), Ok(true));
}

// r[verify consensus.fast-path.recovery]
// r[verify consensus.fast-path.fixtures]
#[test]
fn accepted_set_carries_forward_before_marker() {
    let command = super::command_id(FIRST_COMMAND);
    let mut first = RecoverySnapshot::new(super::view(3), super::view(4)).expect("view advances");
    first.begin(vec![command]).expect("possible set is valid");
    first.accept(vec![command]).expect("accepted set is a subset");
    let mut next = first.carry_forward(super::view(5)).expect("later coordinator view");
    assert_eq!(next.accepted_set, vec![command]);
    assert!(next.admit_normal().is_err());
    next.commit_marker(super::BASE_LOG_LENGTH).expect("marker commits");
    next.finish().expect("normal admission opens");
    assert!(recovered_precedes_new_view(&next, command));
    assert_eq!(admit_recovery_order(&next, command, 0, 1, true), Ok(()));
}

// r[verify consensus.fast-path.recovery]
#[test]
fn empty_recovery_commits_explicit_boundary() {
    let mut snapshot = RecoverySnapshot::new(super::view(3), super::view(4)).expect("view advances");
    snapshot.begin(Vec::new()).expect("empty possible set is valid");
    snapshot.accept(Vec::new()).expect("empty accepted set is valid");
    snapshot.commit_marker(super::BASE_LOG_LENGTH).expect("empty marker commits");
    assert_eq!(snapshot.marker_log_index, Some(crate::noop_commit::noop_commit_index(super::BASE_LOG_LENGTH)));
    snapshot.finish().expect("normal admission opens");
    assert!(snapshot.marker_present);
}

// r[verify consensus.fast-path.dual-path]
// r[verify consensus.fast-path.fixtures]
#[test]
fn retained_record_returns_authoritative_cached_response() {
    let record = DedupRecord {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        response: Some(Response::Ack),
        retain_until: 10,
    };
    let completion = OriginalCompletion {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        observed_at: 5,
    };
    assert_eq!(identity_matches(&record, &completion), Ok(()));
    assert_eq!(converge(&record, &completion), Ok(Response::Ack));
}

fn composition_input<'a>(
    certificate: &'a Certificate,
    store: &'a Store,
    first: &'a Invocation,
    second: &'a Invocation,
    record: &'a DedupRecord,
    completion: &'a OriginalCompletion,
    view: ViewEvidence<'a>,
) -> CompositionInput<'a> {
    CompositionInput {
        certificate,
        capability: super::transparent_capability(),
        state: store,
        first_command: KeyValue::Write {
            key: FIRST_KEY,
            value: 10,
        },
        second_command: KeyValue::Read { key: FIRST_KEY },
        first,
        second,
        view,
        record,
        completion,
    }
}

// r[verify consensus.fast-path.linearization]
// r[verify consensus.fast-path.fixtures]
#[test]
fn bounded_stable_view_composition_preserves_real_time_order() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    let store = Store::default();
    let first = Invocation {
        operation: super::history_operation(FIRST_COMMAND, FIRST_KEY, 0, 2),
        positions: super::positions(0),
    };
    let second = Invocation {
        operation: super::history_operation(2, FIRST_KEY, 3, 4),
        positions: super::positions(1),
    };
    let record = DedupRecord {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        response: Some(Response::Ack),
        retain_until: 10,
    };
    let completion = OriginalCompletion {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        observed_at: 5,
    };
    let input = composition_input(&certificate, &store, &first, &second, &record, &completion, ViewEvidence::Stable {
        acceleration: super::view(super::CURRENT_VIEW),
        base: super::view(super::CURRENT_VIEW),
    });
    assert_eq!(admit_composition(&input), Ok(()));
}

// r[verify consensus.fast-path.linearization]
// r[verify consensus.fast-path.recovery]
#[test]
fn bounded_recovered_view_composition_preserves_real_time_order() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    let command = super::command_id(FIRST_COMMAND);
    let mut snapshot = RecoverySnapshot::new(super::view(8), super::view(super::CURRENT_VIEW)).expect("view advances");
    snapshot.begin(vec![command]).expect("begin recovery");
    snapshot.accept(vec![command]).expect("accept recovered command");
    snapshot.commit_marker(super::BASE_LOG_LENGTH).expect("commit marker");
    snapshot.finish().expect("finish recovery");
    let store = Store::default();
    let first = Invocation {
        operation: super::history_operation(FIRST_COMMAND, FIRST_KEY, 0, 2),
        positions: super::positions(0),
    };
    let second = Invocation {
        operation: super::history_operation(2, FIRST_KEY, 3, 4),
        positions: super::positions(1),
    };
    let record = DedupRecord {
        session: 7,
        sequence: 1,
        command,
        response: Some(Response::Ack),
        retain_until: 10,
    };
    let completion = OriginalCompletion {
        session: 7,
        sequence: 1,
        command,
        observed_at: 5,
    };
    let input =
        composition_input(&certificate, &store, &first, &second, &record, &completion, ViewEvidence::Recovered {
            snapshot: &snapshot,
            command,
        });
    assert_eq!(admit_composition(&input), Ok(()));
}

// r[verify consensus.fast-path.boundary]
// r[verify consensus.fast-path.fixtures]
// r[verify consensus.fast-path.evidence]
// r[verify consensus.fast-path.non-claims]
#[test]
fn supported_scope_is_explicit_and_finite() {
    let scope = CapabilityScope {
        fault_model: FaultModel::Crash,
        membership: MembershipMode::Static,
        command_mode: CommandMode::Single,
    };
    assert_eq!(admit_scope(scope), Ok(()));
}
