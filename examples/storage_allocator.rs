use verified_logic::buddy::buddy_of;
use verified_logic::buddy::can_coalesce;
use verified_logic::buddy::is_aligned_to_order;
use verified_logic::buddy::parent_block;
use verified_logic::wal_recovery::can_append;
use verified_logic::wal_recovery::needs_recovery;
use verified_logic::wal_recovery::replay_count;
use verified_logic::wal_recovery::replay_range;

const BLOCK_ADDR: u64 = 64;
const BUDDY_ORDER: u64 = 4;
const EXPECTED_BUDDY: u64 = 80;
const EXPECTED_PARENT: u64 = 64;
const CHECKPOINT_LSN: u64 = 10;
const WAL_END: u64 = 14;
const EXPECTED_REPLAY_COUNT: u64 = 4;
const APPEND_COUNT: u64 = 2;

fn main() {
    let buddy = buddy_of(BLOCK_ADDR, BUDDY_ORDER);
    let parent = parent_block(BLOCK_ADDR, BUDDY_ORDER);

    assert_eq!(buddy, EXPECTED_BUDDY);
    assert_eq!(parent, EXPECTED_PARENT);
    assert!(is_aligned_to_order(BLOCK_ADDR, BUDDY_ORDER));
    assert!(can_coalesce(BLOCK_ADDR, buddy, BUDDY_ORDER));

    let replay = replay_range(CHECKPOINT_LSN, WAL_END);
    assert_eq!(replay, (CHECKPOINT_LSN, WAL_END));
    assert_eq!(replay_count(CHECKPOINT_LSN, WAL_END), EXPECTED_REPLAY_COUNT);
    assert!(needs_recovery(CHECKPOINT_LSN, WAL_END));
    assert!(can_append(WAL_END, APPEND_COUNT));

    println!("storage/allocator: buddy={buddy}, replay_count={EXPECTED_REPLAY_COUNT}");
}
