//! Verified collection operations.
//!
//! `binary_search` and `sorted_insert` are hand-implemented with verified
//! loop invariants. Verus checks that:
//! - `binary_search` returns a valid index when found, and proves absence when not
//! - `sorted_insert` produces a sorted result of length n+1 containing the value

use vstd::prelude::*;

verus! {

/// Spec: a sequence is sorted in ascending order.
pub open spec fn is_sorted_spec(s: Seq<u64>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

// --- binary_search ---

// r[impl collections.binary-search]
/// Binary search on a sorted slice.
///
/// Returns `Some(index)` where `slice[index] == target`, or `None` if
/// absent. The loop invariant tracks that elements below `lo` and at/above
/// `hi` cannot be the target, narrowing the window each iteration.
pub fn binary_search(slice: &[u64], target: u64) -> (result: Option<usize>)
    requires is_sorted_spec(slice@)
    ensures
        result.is_some() ==> {
            let i = result.unwrap() as int;
            &&& 0 <= i < slice@.len()
            &&& slice@[i] == target
        },
        result.is_none() ==> forall|i: int| 0 <= i < slice@.len() ==> slice@[i] != target,
{
    let mut lo: usize = 0;
    let mut hi: usize = slice.len();

    while lo < hi
        invariant
            0 <= lo <= hi <= slice@.len(),
            is_sorted_spec(slice@),
            forall|i: int| 0 <= i < lo as int ==> slice@[i] != target,
            forall|i: int| hi as int <= i < slice@.len() ==> slice@[i] != target,
        decreases hi - lo,
    {
        let mid = lo + (hi - lo) / 2;

        if slice[mid] == target {
            return Some(mid);
        } else if slice[mid] < target {
            assert forall|i: int| 0 <= i <= mid as int implies slice@[i] != target
            by {
                if i >= lo as int {
                    assert(slice@[i] <= slice@[mid as int]);
                }
            }
            lo = mid + 1;
        } else {
            assert forall|i: int| mid as int <= i < slice@.len() implies slice@[i] != target
            by {
                if i < hi as int {
                    assert(slice@[mid as int] <= slice@[i]);
                }
            }
            hi = mid;
        }
    }

    None
}

// --- sorted_insert ---

/// Finds the insertion point for `value` in a sorted slice.
/// All elements before the returned index are < value;
/// all elements at or after are >= value.
fn find_insert_pos(slice: &[u64], value: u64) -> (pos: usize)
    requires is_sorted_spec(slice@)
    ensures
        0 <= pos <= slice@.len(),
        forall|i: int| 0 <= i < pos as int ==> slice@[i] < value,
        forall|i: int| pos as int <= i < slice@.len() ==> slice@[i] >= value,
{
    let mut lo: usize = 0;
    let mut hi: usize = slice.len();

    while lo < hi
        invariant
            0 <= lo <= hi <= slice@.len(),
            is_sorted_spec(slice@),
            forall|i: int| 0 <= i < lo as int ==> slice@[i] < value,
            forall|i: int| hi as int <= i < slice@.len() ==> slice@[i] >= value,
        decreases hi - lo,
    {
        let mid = lo + (hi - lo) / 2;

        if slice[mid] < value {
            assert forall|i: int| 0 <= i <= mid as int implies slice@[i] < value
            by {
                if i >= lo as int {
                    assert(slice@[i] <= slice@[mid as int]);
                }
            }
            lo = mid + 1;
        } else {
            assert forall|i: int| mid as int <= i < slice@.len() implies slice@[i] >= value
            by {
                if i < hi as int {
                    assert(slice@[mid as int] <= slice@[i]);
                }
            }
            hi = mid;
        }
    }

    lo
}

// r[impl collections.sorted-insert]
/// Inserts `value` into a sorted slice, returning a new sorted `Vec`.
///
/// Uses a verified insertion point search, then builds the result by
/// copying elements before the position, inserting the value, and
/// copying elements after. The postcondition proves:
/// - result length is input length + 1
/// - result is sorted
/// - result contains the inserted value
pub fn sorted_insert(slice: &[u64], value: u64) -> (result: Vec<u64>)
    requires
        is_sorted_spec(slice@),
        slice@.len() < usize::MAX,
    ensures
        result@.len() == slice@.len() + 1,
        is_sorted_spec(result@),
        exists|i: int| 0 <= i < result@.len() && result@[i] == value,
{
    let pos = find_insert_pos(slice, value);
    let mut result = Vec::with_capacity(slice.len() + 1);

    let mut i: usize = 0;
    while i < pos
        invariant
            0 <= i <= pos,
            pos <= slice@.len(),
            result@.len() == i as int,
            forall|j: int| 0 <= j < i as int ==> result@[j] == slice@[j],
        decreases pos - i,
    {
        result.push(slice[i]);
        i = i + 1;
    }

    result.push(value);

    let mut i: usize = pos;
    while i < slice.len()
        invariant
            pos <= i <= slice@.len(),
            pos <= slice@.len(),
            result@.len() == i as int + 1,
            forall|j: int| 0 <= j < pos as int ==> result@[j] == slice@[j],
            result@[pos as int] == value,
            forall|j: int| pos as int <= j < i as int ==> result@[j + 1] == slice@[j],
        decreases slice@.len() - i as int,
    {
        result.push(slice[i]);
        i = i + 1;
    }

    // Prove the result is sorted by case analysis on position relative to
    // the insertion point.
    assert forall|i: int, j: int| 0 <= i < j < result@.len()
        implies result@[i] <= result@[j]
    by {
        let p = pos as int;
        if i < p && j < p {
            assert(result@[i] == slice@[i]);
            assert(result@[j] == slice@[j]);
        } else if i < p && j == p {
            assert(result@[i] == slice@[i]);
            assert(slice@[i] < value);
        } else if i < p && j > p {
            assert(result@[i] == slice@[i]);
            assert(result@[j] == slice@[j - 1]);
        } else if i == p && j > p {
            assert(result@[j] == slice@[j - 1]);
            assert(slice@[j - 1] >= value);
        } else if i > p && j > p {
            assert(result@[i] == slice@[i - 1]);
            assert(result@[j] == slice@[j - 1]);
        }
    }

    assert(result@[pos as int] == value);

    result
}

// --- merge_sorted ---

// r[impl collections.merge-sorted]
/// Merges two sorted slices into a single sorted `Vec`.
///
/// Two-pointer merge: pick the smaller head, advance that pointer.
/// The invariant tracks that:
/// - result is sorted
/// - every element in result <= the next element from both a and b
pub fn merge_sorted(a: &[u64], b: &[u64]) -> (result: Vec<u64>)
    requires
        is_sorted_spec(a@),
        is_sorted_spec(b@),
        a@.len() + b@.len() < usize::MAX,
    ensures
        result@.len() == a@.len() + b@.len(),
        is_sorted_spec(result@),
{
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < a.len() || j < b.len()
        invariant
            0 <= i <= a@.len(),
            0 <= j <= b@.len(),
            result@.len() == i as int + j as int,
            is_sorted_spec(a@),
            is_sorted_spec(b@),
            is_sorted_spec(result@),
            forall|k: int| 0 <= k < result@.len() ==> (
                (i < a@.len() ==> result@[k] <= a@[i as int]) &&
                (j < b@.len() ==> result@[k] <= b@[j as int])
            ),
            a@.len() + b@.len() < usize::MAX,
        decreases a@.len() - i as int + b@.len() - j as int,
    {
        if i < a.len() && (j >= b.len() || a[i] <= b[j]) {
            result.push(a[i]);
            i = i + 1;
        } else {
            result.push(b[j]);
            j = j + 1;
        }
    }

    result
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

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

        // r[verify collections.merge-sorted]
        #[test]
        fn prop_merge_sorted_length(
            a in sorted_vec(30),
            b in sorted_vec(30),
        ) {
            let result = merge_sorted(&a, &b);
            prop_assert_eq!(result.len(), a.len() + b.len());
        }

        // r[verify collections.merge-sorted]
        #[test]
        fn prop_merge_sorted_is_sorted(
            a in sorted_vec(30),
            b in sorted_vec(30),
        ) {
            let result = merge_sorted(&a, &b);
            prop_assert!(is_sorted(&result));
        }

        // r[verify collections.merge-sorted]
        #[test]
        fn prop_merge_sorted_contains_all(
            a in sorted_vec(30),
            b in sorted_vec(30),
        ) {
            let result = merge_sorted(&a, &b);
            // Every element from a and b appears in result
            for &x in &a {
                prop_assert!(result.contains(&x));
            }
            for &x in &b {
                prop_assert!(result.contains(&x));
            }
        }
    }

    // r[verify collections.merge-sorted]
    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
        assert_eq!(merge_sorted(&[], &[4, 5, 6]), vec![4, 5, 6]);
        assert_eq!(merge_sorted(&[1, 1], &[1, 1]), vec![1, 1, 1, 1]);
        let empty: &[u64] = &[];
        assert_eq!(merge_sorted(empty, empty), vec![]);
    }
}
