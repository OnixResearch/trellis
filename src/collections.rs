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

        // insert at start
        let r = sorted_insert(&s, 0);
        assert_eq!(r, vec![0, 1, 3, 5, 7]);
        assert!(is_sorted(&r));

        // insert at end
        let r = sorted_insert(&s, 10);
        assert_eq!(r, vec![1, 3, 5, 7, 10]);
        assert!(is_sorted(&r));

        // insert duplicate
        let r = sorted_insert(&s, 3);
        assert_eq!(r, vec![1, 3, 3, 5, 7]);
        assert!(is_sorted(&r));

        // empty input
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
}
