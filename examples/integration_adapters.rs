use verified_logic::integration_adapters::ConsensusReadAdapterInput;
use verified_logic::integration_adapters::IntegrationAdapterDomain;
use verified_logic::integration_adapters::IntegrationAdapterRejection;
use verified_logic::integration_adapters::NixClosurePolicyAdapterInput;
use verified_logic::integration_adapters::StorageWalRecoveryAdapterInput;
use verified_logic::integration_adapters::UcanAuthorizationAdapterInput;
use verified_logic::integration_adapters::consensus_read_adapter;
use verified_logic::integration_adapters::nix_closure_policy_adapter;
use verified_logic::integration_adapters::storage_wal_recovery_adapter;
use verified_logic::integration_adapters::ucan_authorization_adapter;
use verified_logic::nickel_priority::MergePriority;
use verified_logic::nickel_record_merge::ExecField;
use verified_logic::nix_closure::ClosureItem;
use verified_logic::ucan_token::UcanCapability;
use verified_logic::ucan_token::UcanPayload;

fn closure_item(key: u64) -> ClosureItem {
    ClosureItem { key, data: key * 10 }
}

fn exec_field(key: &[u8], value: u64) -> ExecField {
    ExecField {
        key: key.to_vec(),
        value,
        priority: MergePriority::Neutral,
    }
}

fn main() {
    let consensus = consensus_read_adapter(ConsensusReadAdapterInput {
        read_log_index: 4,
        last_applied_index: 4,
    });
    assert!(consensus.accepted);
    assert_eq!(consensus.domain, IntegrationAdapterDomain::ConsensusRead);

    let lagging_consensus = consensus_read_adapter(ConsensusReadAdapterInput {
        read_log_index: 5,
        last_applied_index: 4,
    });
    assert_eq!(lagging_consensus.rejection, Some(IntegrationAdapterRejection::ApplyCatchupRequired));

    let payload = UcanPayload {
        iss: b"did:key:issuer".to_vec(),
        aud: b"did:key:audience".to_vec(),
        att: vec![UcanCapability {
            resource: b"storage://bucket".to_vec(),
            ability: b"crud/read".to_vec(),
        }],
        prf: Vec::new(),
        exp: 100,
        nbf: 10,
    };
    let ucan = ucan_authorization_adapter(UcanAuthorizationAdapterInput {
        payload: &payload,
        signature_valid: true,
        now: 50,
        tolerance: 0,
    });
    assert!(ucan.accepted);

    let unsigned_ucan = ucan_authorization_adapter(UcanAuthorizationAdapterInput {
        payload: &payload,
        signature_valid: false,
        now: 50,
        tolerance: 0,
    });
    assert_eq!(unsigned_ucan.rejection, Some(IntegrationAdapterRejection::InvalidUcanEnvelope));

    let base_attrs = vec![(b"dep-a".to_vec(), 1)];
    let overlay_attrs = vec![(b"dep-b".to_vec(), 2)];
    let required = vec![1, 2, 3];
    let batches = vec![vec![1, 2], vec![3]];
    let raw_name = b"pkg-name".to_vec();
    let replace_from = vec![b"pkg".to_vec()];
    let replace_to = vec![b"drv".to_vec()];
    let version = b"2.0".to_vec();
    let minimum_version = b"1.0".to_vec();
    let start = vec![closure_item(1)];
    let operator = vec![(1, vec![closure_item(2)]), (2, vec![closure_item(3)])];
    let left_record = vec![exec_field(b"a", 10)];
    let right_record = vec![exec_field(b"b", 20)];
    let nix = nix_closure_policy_adapter(NixClosurePolicyAdapterInput {
        base_attrs: &base_attrs,
        overlay_attrs: &overlay_attrs,
        required_closure_keys: &required,
        dependency_batches: &batches,
        raw_name: &raw_name,
        substring_start: 0,
        substring_len: raw_name.len(),
        replace_from: &replace_from,
        replace_to: &replace_to,
        version: &version,
        minimum_version: &minimum_version,
        start: &start,
        operator: &operator,
        left_record: &left_record,
        right_record: &right_record,
    });
    assert!(nix.accepted);

    let storage = storage_wal_recovery_adapter(StorageWalRecoveryAdapterInput {
        checkpoint_lsn: 3,
        wal_end: 8,
        append_count: 2,
    });
    assert!(storage.decision.accepted);
    assert_eq!(storage.replay_count, 5);
    assert!(storage.needs_recovery);

    let overflowing_storage = storage_wal_recovery_adapter(StorageWalRecoveryAdapterInput {
        checkpoint_lsn: u64::MAX,
        wal_end: u64::MAX,
        append_count: 1,
    });
    assert_eq!(overflowing_storage.decision.rejection, Some(IntegrationAdapterRejection::WalAppendWouldOverflow));

    println!("integration adapters: accepted finite domains and rejected malformed inputs");
}
