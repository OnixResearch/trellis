use proptest::prelude::*;

use super::super::*;

const MAX_PROPERTY_FAULTS: u32 = 64;
const ROTATION_BOUND: u32 = 1_024;
const VALUE_BOUND: u64 = 10_000;

proptest! {
    // r[verify consensus.fast-path.quorum]
    // r[verify consensus.fast-path.fixtures]
    #[test]
    fn formulas_hold_for_bounded_fault_budgets(faults in 1u32..=MAX_PROPERTY_FAULTS) {
        let cluster = cluster_size(faults).expect("bounded fault budget");
        let quorum = accelerated_quorum(faults).expect("bounded fault budget");
        let support = recovery_support(faults).expect("bounded fault budget");
        prop_assert!(quorum <= cluster);
        prop_assert_eq!(accelerated_failure_tolerance(faults), Some(faults / 2));
        prop_assert_eq!(intersection_lower_bound(faults), Some(support));
    }

    // r[verify consensus.fast-path.quorum]
    // r[verify consensus.fast-path.certificate]
    // r[verify consensus.fast-path.fixtures]
    #[test]
    fn finite_membership_intersection_meets_support(
        faults in 1u32..=MAX_PROPERTY_FAULTS,
        certificate_start in 0u32..ROTATION_BOUND,
        recovery_start in 0u32..ROTATION_BOUND,
    ) {
        let cluster = cluster_size(faults).expect("bounded fault budget");
        let quorum = accelerated_quorum(faults).expect("bounded fault budget");
        let majority_count = majority(faults).expect("bounded fault budget");
        let certificate_ids = (0..quorum)
            .map(|offset| (certificate_start + offset) % cluster)
            .collect::<Vec<_>>();
        let recovery_ids = (0..majority_count)
            .map(|offset| super::replica((recovery_start + offset) % cluster))
            .collect::<Vec<_>>();
        let certificate = super::certificate(
            &certificate_ids,
            1,
            super::CURRENT_VIEW,
            faults,
        );
        prop_assert_eq!(admit_certificate(&certificate), Ok(()));
        let overlap = recovery_visibility(&certificate, &recovery_ids)
            .expect("valid recovery majority");
        prop_assert!(overlap >= recovery_support(faults).expect("bounded fault budget"));
    }

    // r[verify consensus.fast-path.certificate]
    // r[verify consensus.fast-path.fixtures]
    #[test]
    fn one_fewer_acknowledgement_never_admits(
        faults in 1u32..=MAX_PROPERTY_FAULTS,
        start in 0u32..ROTATION_BOUND,
    ) {
        let cluster = cluster_size(faults).expect("bounded fault budget");
        let short_count = accelerated_quorum(faults).expect("bounded fault budget") - 1;
        let replica_ids = (0..short_count)
            .map(|offset| (start + offset) % cluster)
            .collect::<Vec<_>>();
        let certificate = super::certificate(
            &replica_ids,
            1,
            super::CURRENT_VIEW,
            faults,
        );
        prop_assert_eq!(
            super::negative::fault_class(admit_certificate(&certificate)),
            Some(CertificateFault::WrongQuorumSize),
        );
    }

    // r[verify consensus.fast-path.conflict]
    // r[verify consensus.fast-path.fixtures]
    #[test]
    fn disjoint_writes_preserve_state_and_responses(
        left_key in 0u64..VALUE_BOUND,
        right_key in 0u64..VALUE_BOUND,
        left_value in 0u64..VALUE_BOUND,
        right_value in 0u64..VALUE_BOUND,
    ) {
        prop_assume!(left_key != right_key);
        let left = KeyValue::Write { key: left_key, value: left_value };
        let right = KeyValue::Write { key: right_key, value: right_value };
        prop_assert!(!conflicts(left, right));
        prop_assert!(commute_under_contract(&Store::default(), left, right));
    }

    // r[verify consensus.fast-path.dual-path]
    // r[verify consensus.fast-path.fixtures]
    #[test]
    fn matching_completion_is_duplicate_while_retained(
        sequence in 0u64..VALUE_BOUND,
        observed_at in 0u64..VALUE_BOUND,
        extra_retention in 0u64..VALUE_BOUND,
    ) {
        let retain_until = observed_at.saturating_add(extra_retention);
        let record = DedupRecord {
            session: 1,
            sequence,
            command: super::command_id(1),
            response: Some(Response::Ack),
            retain_until,
        };
        let completion = OriginalCompletion {
            session: 1,
            sequence,
            command: super::command_id(1),
            observed_at,
        };
        prop_assert_eq!(converge(&record, &completion), Ok(Response::Ack));
    }
}
