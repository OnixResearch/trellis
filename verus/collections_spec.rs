use vstd::prelude::*;

verus! {

/// A sequence is sorted in ascending order.
pub open spec fn is_sorted(s: Seq<u64>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

// --- sorted_insert ---

// r[depends collections.sorted-insert]
/// Spec: insert value at the correct position in a sorted sequence.
/// Returns a new sequence that is sorted and contains all original
/// elements plus the new value.
pub open spec fn sorted_insert_spec(s: Seq<u64>, value: u64) -> Seq<u64>
    decreases s.len()
{
    // Find first index where s[i] >= value, insert there.
    // Defined recursively for proof clarity.
    if s.len() == 0 {
        seq![value]
    } else if value <= s[0] {
        seq![value] + s
    } else {
        seq![s[0]] + sorted_insert_spec(s.subrange(1, s.len() as int), value)
    }
}

// r[verify collections.sorted-insert]
/// Proof: inserting into a sorted sequence produces a sorted sequence.
proof fn sorted_insert_preserves_sorted(s: Seq<u64>, value: u64)
    requires is_sorted(s)
    ensures is_sorted(sorted_insert_spec(s, value))
    decreases s.len()
{
    if s.len() == 0 {
        // Base case: seq![value] is trivially sorted
        assert(is_sorted(seq![value]));
    } else if value <= s[0] {
        // Prepend case: value <= s[0] <= s[i] for all i
        let result = seq![value] + s;
        assert forall|i: int, j: int| 0 <= i < j < result.len()
            implies result[i] <= result[j]
        by {
            if i == 0 {
                if j == 1 {
                    // value <= s[0]
                } else {
                    // value <= s[0] <= s[j-1]
                    assert(s[0] <= s[j - 1]);
                }
            } else {
                // both in s
                assert(s[i - 1] <= s[j - 1]);
            }
        }
    } else {
        // Recursive case: s[0] < value, recurse on tail
        let tail = s.subrange(1, s.len() as int);

        // Tail is sorted
        assert forall|i: int, j: int| 0 <= i < j < tail.len()
            implies tail[i] <= tail[j]
        by {
            assert(s[i + 1] <= s[j + 1]);
        }

        sorted_insert_preserves_sorted(tail, value);

        let inserted_tail = sorted_insert_spec(tail, value);
        let result = seq![s[0]] + inserted_tail;

        // Show result is sorted: s[0] <= inserted_tail[0], and
        // inserted_tail is sorted.
        assert forall|i: int, j: int| 0 <= i < j < result.len()
            implies result[i] <= result[j]
        by {
            if i == 0 {
                // result[0] == s[0], result[j] == inserted_tail[j-1]
                // Need s[0] <= inserted_tail[j-1]
                // inserted_tail is sorted, so inserted_tail[0] <= inserted_tail[j-1]
                // and s[0] <= inserted_tail[0] because either:
                //   - inserted_tail[0] == value (and s[0] < value)
                //   - inserted_tail[0] == tail[0] == s[1] (and s[0] <= s[1])
                if tail.len() == 0 {
                    // inserted_tail == seq![value], s[0] < value
                } else if value <= tail[0] {
                    // inserted_tail[0] == value, s[0] < value
                } else {
                    // inserted_tail[0] == tail[0] == s[1], s[0] <= s[1]
                    assert(s[0] <= s[1]);
                }
                if j > 1 {
                    assert(inserted_tail[0] <= inserted_tail[j - 1]);
                }
            } else {
                // Both in inserted_tail
                assert(inserted_tail[i - 1] <= inserted_tail[j - 1]);
            }
        }
    }
}

// r[verify collections.sorted-insert]
/// Proof: the length grows by exactly one.
proof fn sorted_insert_length(s: Seq<u64>, value: u64)
    ensures sorted_insert_spec(s, value).len() == s.len() + 1
    decreases s.len()
{
    if s.len() == 0 {
    } else if value <= s[0] {
    } else {
        sorted_insert_length(s.subrange(1, s.len() as int), value);
    }
}

// --- binary_search ---

// r[depends collections.binary-search]
/// Spec: binary search returns Some(i) where s[i] == target if present,
/// None if target is not in the sequence.
pub open spec fn binary_search_spec(s: Seq<u64>, target: u64) -> Option<int> {
    if exists|i: int| 0 <= i < s.len() && s[i] == target {
        Some(choose|i: int| 0 <= i < s.len() && s[i] == target)
    } else {
        None
    }
}

// r[verify collections.binary-search]
/// Proof: if binary_search returns Some(i), then s[i] == target and
/// i is in bounds.
proof fn binary_search_found_valid(s: Seq<u64>, target: u64)
    ensures
        binary_search_spec(s, target).is_some() ==> {
            let i = binary_search_spec(s, target).unwrap();
            &&& 0 <= i < s.len()
            &&& s[i] == target
        }
{
}

// r[verify collections.binary-search]
/// Proof: if target is not in the sequence, binary_search returns None.
proof fn binary_search_none_means_absent(s: Seq<u64>, target: u64)
    ensures
        binary_search_spec(s, target).is_none() ==>
            forall|i: int| 0 <= i < s.len() ==> s[i] != target
{
}

} // verus!
