use verified_logic::raft_consensus_core::RAFT_OPENRAFT_APPLY_CATCHUP_REQUIRED;
use verified_logic::raft_consensus_core::RAFT_OPENRAFT_OK;
use verified_logic::raft_consensus_core::RaftReadLinearizerMetadata;
use verified_logic::raft_consensus_core::raft_openraft_read_linearizer_admission_result;
use verified_logic::raft_consensus_core::raft_read_linearizer_ready;

const READ_INDEX: u64 = 8;
const CAUGHT_UP_APPLIED_INDEX: u64 = 8;
const STALE_APPLIED_INDEX: u64 = 7;

fn main() {
    let ready = RaftReadLinearizerMetadata {
        read_log_index: READ_INDEX,
        last_applied_index: CAUGHT_UP_APPLIED_INDEX,
    };
    let stale = RaftReadLinearizerMetadata {
        read_log_index: READ_INDEX,
        last_applied_index: STALE_APPLIED_INDEX,
    };

    assert!(raft_read_linearizer_ready(&ready));
    assert!(!raft_read_linearizer_ready(&stale));

    let accepted = raft_openraft_read_linearizer_admission_result(&ready);
    assert!(accepted.admitted);
    assert_eq!(accepted.failing_check, RAFT_OPENRAFT_OK);

    let rejected = raft_openraft_read_linearizer_admission_result(&stale);
    assert!(!rejected.admitted);
    assert_eq!(rejected.failing_check, RAFT_OPENRAFT_APPLY_CATCHUP_REQUIRED);

    println!("consensus admission: read index {READ_INDEX} waits for local apply catch-up");
}
