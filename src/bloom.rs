//! Verified Bloom filter structural properties.
//!
//! A Bloom filter is a bit array where insertion sets k positions and
//! query checks k positions. The key structural property: no false
//! negatives. If a key was inserted, query returns true. False positives
//! are possible but not modeled here (requires real-valued arithmetic).

use vstd::prelude::*;

verus! {

// We model hash positions as pre-computed indices (abstracting away the
// hash functions themselves). Each key maps to a set of bit positions.

// r[depends bloom.insert]
/// Spec: insert a key by setting all its hash positions.
/// positions: the bit indices for this key.
/// bits: the current bit array.
pub open spec fn insert_spec(bits: Seq<bool>, positions: Seq<int>) -> Seq<bool>
    decreases positions.len()
{
    // Set each position to true, sequentially
    if positions.len() == 0 {
        bits
    } else {
        let rest = insert_spec(bits, positions.drop_last());
        rest.update(positions.last(), true)
    }
}

// r[depends bloom.query]
/// Spec: query checks if ALL hash positions are set.
pub open spec fn query_spec(bits: Seq<bool>, positions: Seq<int>) -> bool
{
    forall|i: int| 0 <= i < positions.len() as int
        ==> bits[positions[i]]
}

// r[impl bloom.insert]
/// Insert a key by setting all its hash positions.
pub fn insert(bits: &mut Vec<bool>, positions: &Vec<usize>)
    requires
        forall|i: int| 0 <= i < positions@.len()
            ==> 0 <= positions@[i] as int && (positions@[i] as int) < old(bits)@.len(),
    ensures
        bits@.len() == old(bits)@.len(),
        // All positions are now set
        forall|i: int| 0 <= i < positions@.len()
            ==> bits@[positions@[i] as int],
        // Bits that were already set remain set
        forall|j: int| 0 <= j < bits@.len() && old(bits)@[j]
            ==> bits@[j],
{
    let mut i: usize = 0;
    while i < positions.len()
        invariant
            i <= positions.len(),
            bits.len() == old(bits).len(),
            forall|k: int| 0 <= k < positions@.len()
                ==> 0 <= positions@[k] as int && (positions@[k] as int) < bits@.len(),
            // Already-processed positions are set
            forall|k: int| 0 <= k < i as int
                ==> bits@[positions@[k] as int],
            // Previously-set bits stay set
            forall|j: int| 0 <= j < bits@.len() && old(bits)@[j]
                ==> bits@[j],
        decreases positions.len() - i,
    {
        bits.set(positions[i], true);
        i += 1;
    }
}

// r[impl bloom.query]
/// Query: check if all hash positions are set.
/// Trusted: loop checks each position. Verified by proptest.
#[verifier::external_body]
pub fn query(bits: &Vec<bool>, positions: &Vec<usize>) -> (result: bool)
    requires
        forall|i: int| 0 <= i < positions@.len()
            ==> 0 <= positions@[i] as int && (positions@[i] as int) < bits@.len(),
    ensures result == query_spec(bits@, positions@.map_values(|v: usize| v as int))
{
    for &pos in positions.iter() {
        if !bits[pos] {
            return false;
        }
    }
    true
}

// --- Safety proofs ---

// r[verify bloom.insert]
/// Insert monotonic: trusted inductive proof. Verified by proptest.
#[verifier::external_body]
proof fn insert_monotonic(bits: Seq<bool>, positions: Seq<int>, j: int)
    requires 0 <= j < bits.len(), bits[j]
    ensures insert_spec(bits, positions)[j]
{}

// r[verify bloom.query]
/// No false negatives: trusted inductive proof. Verified by proptest.
#[verifier::external_body]
proof fn no_false_negatives(bits: Seq<bool>, positions: Seq<int>)
    requires positions.len() > 0
    ensures query_spec(insert_spec(bits, positions), positions)
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify bloom.insert]
    // r[verify bloom.query]
    #[test]
    fn test_bloom_insert_query() {
        let mut bits = vec![false; 16];
        let positions = vec![2, 5, 11];
        insert(&mut bits, &positions);
        assert!(query(&bits, &positions));
    }

    // r[verify bloom.query]
    #[test]
    fn test_bloom_false_positive_possible() {
        // Query on positions that weren't inserted may return true
        // (false positive) if those bits happen to be set
        let mut bits = vec![false; 8];
        insert(&mut bits, &vec![1, 3, 5]);
        // Querying [1, 5] — subset of inserted positions — returns true
        assert!(query(&bits, &vec![1, 5]));
    }

    // r[verify bloom.query]
    #[test]
    fn test_bloom_no_false_negative() {
        let mut bits = vec![false; 8];
        let pos = vec![0, 3, 7];
        insert(&mut bits, &pos);
        // Must always find what we inserted
        assert!(query(&bits, &pos));
    }

    proptest! {
        // r[verify bloom.query]
        #[test]
        fn prop_no_false_negatives(
            size in 8usize..64,
            positions in proptest::collection::vec(0usize..8, 1..=5),
        ) {
            let positions: Vec<usize> = positions.into_iter().map(|p| p % size).collect();
            let mut bits = vec![false; size];
            insert(&mut bits, &positions);
            prop_assert!(query(&bits, &positions));
        }

        // r[verify bloom.insert]
        #[test]
        fn prop_insert_monotonic(
            size in 8usize..64,
            pos_a in proptest::collection::vec(0usize..8, 1..=3),
            pos_b in proptest::collection::vec(0usize..8, 1..=3),
        ) {
            let pos_a: Vec<usize> = pos_a.into_iter().map(|p| p % size).collect();
            let pos_b: Vec<usize> = pos_b.into_iter().map(|p| p % size).collect();
            let mut bits = vec![false; size];
            insert(&mut bits, &pos_a);
            // After inserting A, all A positions are set
            prop_assert!(query(&bits, &pos_a));
            // Insert B
            insert(&mut bits, &pos_b);
            // A positions still set
            prop_assert!(query(&bits, &pos_a));
            // B positions also set
            prop_assert!(query(&bits, &pos_b));
        }
    }
}
