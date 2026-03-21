//! Verified collection operations.

// r[impl collections.sorted-insert]
/// Inserts `value` into a sorted slice, returning a new sorted `Vec`
/// containing all original elements plus `value`.
///
/// The input slice must be sorted in ascending order.
pub fn sorted_insert(sorted: &[u64], value: u64) -> Vec<u64> {
    let pos = sorted.partition_point(|&x| x < value);
    let mut result = Vec::with_capacity(sorted.len() + 1);
    result.extend_from_slice(&sorted[..pos]);
    result.push(value);
    result.extend_from_slice(&sorted[pos..]);
    result
}

// r[impl collections.binary-search]
/// Binary search on a sorted slice. Returns `Some(index)` if `target`
/// is found, `None` otherwise.
///
/// The input slice must be sorted in ascending order.
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

    // r[verify collections.sorted-insert]
    #[test]
    fn test_sorted_insert() {
        let s = vec![1, 3, 5, 7];
        let r = sorted_insert(&s, 4);
        assert_eq!(r, vec![1, 3, 4, 5, 7]);
        assert!(is_sorted(&r));

        let r = sorted_insert(&s, 0);
        assert_eq!(r, vec![0, 1, 3, 5, 7]);
        assert!(is_sorted(&r));

        let r = sorted_insert(&s, 10);
        assert_eq!(r, vec![1, 3, 5, 7, 10]);
        assert!(is_sorted(&r));

        let r = sorted_insert(&s, 3);
        assert_eq!(r, vec![1, 3, 3, 5, 7]);
        assert!(is_sorted(&r));

        let r = sorted_insert(&[], 5);
        assert_eq!(r, vec![5]);
    }

    // r[verify collections.binary-search]
    #[test]
    fn test_binary_search() {
        let s = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&s, 5), Some(2));
        assert_eq!(binary_search(&s, 1), Some(0));
        assert_eq!(binary_search(&s, 9), Some(4));
        assert_eq!(binary_search(&s, 4), None);
        assert_eq!(binary_search(&s, 0), None);
        assert_eq!(binary_search(&[], 5), None);
    }

    // Strategy: generate a sorted Vec<u64> by sorting random values.
    fn sorted_vec(max_len: usize) -> impl Strategy<Value = Vec<u64>> {
        proptest::collection::vec(any::<u64>(), 0..=max_len).prop_map(|mut v| {
            v.sort_unstable();
            v
        })
    }

    proptest! {
        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_preserves_sorted(
            s in sorted_vec(50),
            value: u64,
        ) {
            let result = sorted_insert(&s, value);
            prop_assert!(is_sorted(&result),
                "result not sorted: {:?}", result);
        }

        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_length(
            s in sorted_vec(50),
            value: u64,
        ) {
            let result = sorted_insert(&s, value);
            prop_assert_eq!(result.len(), s.len() + 1);
        }

        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_contains_value(
            s in sorted_vec(50),
            value: u64,
        ) {
            let result = sorted_insert(&s, value);
            prop_assert!(result.contains(&value));
        }

        // r[verify collections.sorted-insert]
        #[test]
        fn prop_sorted_insert_contains_all_original(
            s in sorted_vec(50),
            value: u64,
        ) {
            let result = sorted_insert(&s, value);
            // Every original element appears at least as many times.
            for &x in &s {
                let orig_count = s.iter().filter(|&&v| v == x).count();
                let result_count = result.iter().filter(|&&v| v == x).count();
                prop_assert!(result_count >= orig_count,
                    "element {} lost: {} in original, {} in result", x, orig_count, result_count);
            }
        }

        // r[verify collections.binary-search]
        #[test]
        fn prop_binary_search_found_valid(
            s in sorted_vec(50),
            target: u64,
        ) {
            if let Some(idx) = binary_search(&s, target) {
                prop_assert!(idx < s.len());
                prop_assert_eq!(s[idx], target);
            }
        }

        // r[verify collections.binary-search]
        #[test]
        fn prop_binary_search_none_means_absent(
            s in sorted_vec(50),
            target: u64,
        ) {
            if binary_search(&s, target).is_none() {
                prop_assert!(!s.contains(&target));
            }
        }

        // r[verify collections.binary-search]
        #[test]
        fn prop_binary_search_finds_existing(s in sorted_vec(50)) {
            // If the vec is non-empty, searching for an existing element
            // must return Some.
            if let Some(&val) = s.first() {
                prop_assert!(binary_search(&s, val).is_some());
            }
        }
    }
}
