use verified_logic::arithmetic::abs_diff;
use verified_logic::bounds::clamp;
use verified_logic::quorum::is_quorum;
use verified_logic::quorum::majority;

const LEFT_VALUE: u64 = 10;
const RIGHT_VALUE: u64 = 3;
const CLAMP_INPUT: u64 = 15;
const CLAMP_MIN: u64 = 0;
const CLAMP_MAX: u64 = 10;
const CLUSTER_SIZE: u64 = 5;
const QUORUM_COUNT: u64 = 3;
const NON_QUORUM_COUNT: u64 = 2;

fn main() {
    let distance = abs_diff(LEFT_VALUE, RIGHT_VALUE);
    let bounded = clamp(CLAMP_INPUT, CLAMP_MIN, CLAMP_MAX);
    let quorum = majority(CLUSTER_SIZE);

    assert_eq!(distance, 7);
    assert_eq!(bounded, CLAMP_MAX);
    assert_eq!(quorum, QUORUM_COUNT);
    assert!(is_quorum(QUORUM_COUNT, CLUSTER_SIZE));
    assert!(!is_quorum(NON_QUORUM_COUNT, CLUSTER_SIZE));

    println!("core basics: distance={distance}, bounded={bounded}, quorum={quorum}");
}
