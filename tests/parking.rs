use verified_logic::parking_lot_core_model::ParkKey;
use verified_logic::parking_lot_core_model::Queue;
use verified_logic::parking_lot_core_model::Table;
use verified_logic::parking_lot_core_model::ThreadId;
use verified_logic::parking_lot_core_model::Waiter;
use verified_logic::parking_lot_core_model::park_waiter;
use verified_logic::parking_lot_core_model::requeue_after_wake;
use verified_logic::parking_lot_core_model::requeue_preserves_waiter_count;
use verified_logic::parking_lot_core_model::table_valid;
use verified_logic::parking_lot_core_model::unpark_one;
use verified_logic::parking_lot_core_model::waiter_count;

const PRIMARY_KEY_RAW: u64 = 7;
const DUPLICATE_KEY_RAW: u64 = 8;
const DESTINATION_KEY_RAW: u64 = 9;
const ABSENT_KEY_RAW: u64 = 99;
const SECOND_THREAD_RAW: u64 = 2;
const NEW_THREAD_RAW: u64 = 10;
const PRIMARY_KEY: ParkKey = ParkKey::new(PRIMARY_KEY_RAW);
const DUPLICATE_KEY: ParkKey = ParkKey::new(DUPLICATE_KEY_RAW);
const DESTINATION_KEY: ParkKey = ParkKey::new(DESTINATION_KEY_RAW);
const ABSENT_KEY: ParkKey = ParkKey::new(ABSENT_KEY_RAW);
const FIRST_THREAD: ThreadId = ThreadId::new(1);
const SECOND_THREAD: ThreadId = ThreadId::new(SECOND_THREAD_RAW);
const NEW_THREAD: ThreadId = ThreadId::new(NEW_THREAD_RAW);

fn table() -> Table {
    Table {
        queues: vec![Queue {
            key: PRIMARY_KEY,
            waiters: vec![
                Waiter {
                    thread_id: FIRST_THREAD,
                    parked_key: PRIMARY_KEY,
                },
                Waiter {
                    thread_id: SECOND_THREAD,
                    parked_key: PRIMARY_KEY,
                },
            ],
        }],
    }
}

// r[verify newtypes.parking-lot-identities]
// r[verify newtypes.parking-lot-docs]
// r[verify newtypes.parking-lot-validation]
#[test]
fn identity_construction_round_trips_raw_values() {
    assert_eq!(PRIMARY_KEY.raw(), PRIMARY_KEY_RAW);
    assert_eq!(FIRST_THREAD.raw(), 1);
}

// r[verify newtypes.parking-lot-evidence]
#[test]
fn distinct_waiters_are_valid() {
    assert!(table_valid(&table()));
}

// r[verify newtypes.parking-lot-evidence]
// r[verify newtypes.parking-lot-non-claims]
#[test]
fn duplicate_waiter_is_rejected() {
    let mut invalid = table();
    invalid.queues.push(Queue {
        key: DUPLICATE_KEY,
        waiters: vec![Waiter {
            thread_id: FIRST_THREAD,
            parked_key: DUPLICATE_KEY,
        }],
    });
    assert!(!table_valid(&invalid));
}

// r[verify newtypes.parking-lot-evidence]
#[test]
fn mismatched_parked_key_is_rejected() {
    let invalid = Table {
        queues: vec![Queue {
            key: PRIMARY_KEY,
            waiters: vec![Waiter {
                thread_id: FIRST_THREAD,
                parked_key: DUPLICATE_KEY,
            }],
        }],
    };
    assert!(!table_valid(&invalid));
}

// r[verify newtypes.parking-lot-evidence]
// r[verify newtypes.parking-lot-api]
// r[verify newtypes.parking-lot-semantics]
#[test]
fn park_unpark_and_requeue_preserve_finite_results() {
    let empty = Table { queues: Vec::new() };
    let parked = park_waiter(&empty, PRIMARY_KEY, NEW_THREAD);
    assert!(table_valid(&parked));
    assert_eq!(waiter_count(&parked, NEW_THREAD), 1);

    let (none, unchanged) = unpark_one(&parked, ABSENT_KEY);
    assert_eq!(none, None);
    assert_eq!(unchanged, parked);

    let result = requeue_after_wake(&table(), PRIMARY_KEY, DESTINATION_KEY);
    assert_eq!(result.woken.map(|waiter| waiter.thread_id), Some(FIRST_THREAD));
    assert_eq!(result.moved, 1);
    assert!(table_valid(&result.table));
    assert!(requeue_preserves_waiter_count(&table(), &result));
}
