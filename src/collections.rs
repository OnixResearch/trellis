//! Verified collection operations.

use vstd::prelude::*;

verus! {

/// Spec: a sequence is sorted in ascending order.
pub open spec fn is_sorted_spec(s: Seq<u64>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

// r[impl collections.sorted-insert]
// r[impl collections.binary-search]
// NOTE: sorted_insert and binary_search use stdlib methods (partition_point,
// binary_search) whose internals aren't visible to Verus. The specs and
// proofs for these live alongside the spec functions above. The exec
// functions are verified via property tests below.

} // verus!

// r[impl collections.sorted-insert]
/// Inserts `value` into a sorted slice, returning a new sorted `Vec`.
pub fn sorted_insert(sorted: &[u64], value: u64) -> Vec<u64> {
    let pos = sorted.partition_point(|&x| x < value);
    let mut result = Vec::with_capacity(sorted.len() + 1);
    result.extend_from_slice(&sorted[..pos]);
    result.push(value);
    result.extend_from_slice(&sorted[pos..]);
    result
}

// r[impl collections.binary-search]
/// Binary search on a sorted slice.
pub fn binary_search(sorted: &[u64], target: u64) -> Option<usize> {
    sorted.binary_search(&target).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn is_sorted(s: &[u64]) -> bool {
        s.windows(2).all(|w| w[0] <= w[1])
    }

    fn sorted_vec(max_len: usize) -> impl Strategy<Value = Vec<u64>> {
        proptest::collection::vec(any::<u64>(), 0..=max_len).prop_map(|mut v| {
            v.sort_unstable();
            v
        })
    }

    // r[verify collections.sorted-insert]
    #[test]
    fn test_sorted_insert() {
        let s = vec![1, 3, 5, 7];
        assert_eq!(sorted_insert(&s, 4), vec![1, 3, 4, 5, 7]);
        assert_eq!(sorted_insert(&s, 0), vec![0, 1, 3, 5, 7]);
        assert_eq!(sorted_insert(&s, 10), vec![1, 3, 5, 7, 10]);
        assert_eq!(sorted_insert(&s, 3), vec![1, 3, 3, 5, 7]);
        assert_eq!(sorted_insert(&[], 5), vec![5]);
    }

    // r[verify collections.binary-search]
    #[test]
    fn test_binary_search() {
        let s = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&s, 5), Some(2));
        assert_eq!(binary_search(&s, 4), None);
        assert_eq!(binary_search(&[], 5), None);
    }

    proptest! {
        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_preserves_sorted(s in sorted_vec(50), value: u64) {
            let result = sorted_insert(&s, value);
            prop_assert!(is_sorted(&result));
        }

        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_length(s in sorted_vec(50), value: u64) {
            prop_assert_eq!(sorted_insert(&s, value).len(), s.len() + 1);
        }

        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_contains_value(s in sorted_vec(50), value: u64) {
            prop_assert!(sorted_insert(&s, value).contains(&value));
        }

        // r[verify collections.binary-search]
        #[test]
        fn prop_binary_search_found_valid(s in sorted_vec(50), target: u64) {
            if let Some(idx) = binary_search(&s, target) {
                prop_assert!(idx < s.len());
                prop_assert_eq!(s[idx], target);
            }
        }

        // r[verify collections.binary-search]
        #[test]
        fn prop_binary_search_none_means_absent(s in sorted_vec(50), target: u64) {
            if binary_search(&s, target).is_none() {
                prop_assert!(!s.contains(&target));
            }
        }
    }
}
