use verified_logic::bounded_modular_tree_fold::balanced_u24_sum;
use verified_logic::bounded_modular_tree_fold::sequential_u24_sum;
use verified_logic::u24::MAX;
use verified_logic::u24::ZERO;

const LEAF_LIMIT: u64 = 4;

fn main() {
    let leaves = [MAX, 1, ZERO, 1];
    let sequential = sequential_u24_sum(&leaves, LEAF_LIMIT).expect("declared profile must be admitted");
    let balanced = balanced_u24_sum(&leaves, LEAF_LIMIT).expect("declared profile must be admitted");

    assert_eq!(sequential.value, balanced.value);
    println!("u24 sum: {}", balanced.value);
    println!("abstract combines: {}", balanced.combine_count);
    println!("halving rounds: {}", balanced.round_count);
}
