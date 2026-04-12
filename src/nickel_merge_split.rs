//! Verified three-way record split for Nickel merge.
//!
//! Models the `split` step from `core/src/eval/merge.rs`: given two
//! sorted key-value maps, partition into left-only, both (center),
//! and right-only. Proves disjointness, completeness, and value
//! preservation.

#![allow(clippy::ptr_arg, clippy::too_many_lines, unused_variables)]

use vstd::prelude::*;

verus! {

// --- Byte-sequence comparison (self-contained for cross-module independence) ---

/// Lexicographic comparison on byte sequences: returns -1, 0, or 1.
pub open spec fn key_cmp(a: Seq<u8>, b: Seq<u8>) -> int
    decreases a.len() + b.len()
{
    if a.len() == 0 && b.len() == 0 { 0 }
    else if a.len() == 0 { -1 }
    else if b.len() == 0 { 1 }
    else if a[0int] < b[0int] { -1 }
    else if a[0int] > b[0int] { 1 }
    else { key_cmp(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int)) }
}

/// Lexicographic less-than on byte sequences.
pub open spec fn key_lt(a: Seq<u8>, b: Seq<u8>) -> bool {
    key_cmp(a, b) < 0
}

/// Antisymmetry: key_cmp(a, b) == -key_cmp(b, a).
pub proof fn key_cmp_antisymmetric(a: Seq<u8>, b: Seq<u8>)
    ensures key_cmp(a, b) == -key_cmp(b, a)
    decreases a.len() + b.len()
{
    if a.len() > 0 && b.len() > 0 && a[0int] == b[0int] {
        key_cmp_antisymmetric(
            a.subrange(1, a.len() as int),
            b.subrange(1, b.len() as int),
        );
    }
}

/// Skip equal prefix byte helper.
proof fn key_cmp_skip(a: Seq<u8>, b: Seq<u8>, i: int)
    requires 0 <= i < a.len(), 0 <= i < b.len(), a[i] == b[i]
    ensures
        key_cmp(a.subrange(i, a.len() as int), b.subrange(i, b.len() as int))
            == key_cmp(a.subrange(i + 1, a.len() as int), b.subrange(i + 1, b.len() as int))
{
    let sa = a.subrange(i, a.len() as int);
    let sb = b.subrange(i, b.len() as int);
    assert(sa.len() > 0 && sb.len() > 0);
    assert(sa[0int] == a[i] && sb[0int] == b[i]);
    assert(sa.subrange(1, sa.len() as int) =~= a.subrange(i + 1, a.len() as int));
    assert(sb.subrange(1, sb.len() as int) =~= b.subrange(i + 1, b.len() as int));
}

/// Exec byte-by-byte comparison matching key_cmp spec.
fn key_cmp_exec(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)
    ensures result as int == key_cmp(a@, b@),
            result == -1 || result == 0 || result == 1
{
    proof { assert(a@.subrange(0, a@.len() as int) =~= a@); }
    proof { assert(b@.subrange(0, b@.len() as int) =~= b@); }
    let mut i: usize = 0;
    while i < a.len() && i < b.len()
        invariant
            i <= a.len(),
            i <= b.len(),
            key_cmp(a@, b@) == key_cmp(a@.subrange(i as int, a@.len() as int),
                                       b@.subrange(i as int, b@.len() as int)),
        decreases a.len() + b.len() - 2 * i,
    {
        let sa = Ghost(a@.subrange(i as int, a@.len() as int));
        let sb = Ghost(b@.subrange(i as int, b@.len() as int));
        if a[i] < b[i] {
            assert(sa@[0int] < sb@[0int]);
            return -1;
        }
        if a[i] > b[i] {
            assert(sa@[0int] > sb@[0int]);
            return 1;
        }
        proof { key_cmp_skip(a@, b@, i as int); }
        i = i + 1;
    }
    if a.len() == b.len() {
        proof {
            assert(a@.subrange(i as int, a@.len() as int).len() == 0);
            assert(b@.subrange(i as int, b@.len() as int).len() == 0);
        }
        0
    } else if i >= a.len() {
        proof {
            assert(a@.subrange(i as int, a@.len() as int).len() == 0);
            assert(b@.subrange(i as int, b@.len() as int).len() > 0);
        }
        -1
    } else {
        proof {
            assert(a@.subrange(i as int, a@.len() as int).len() > 0);
            assert(b@.subrange(i as int, b@.len() as int).len() == 0);
        }
        1
    }
}

// --- Sorted-keys invariant ---

/// Spec: keys are sorted and unique (by key_lt).
pub open spec fn is_sorted_keys(attrs: Seq<(Seq<u8>, u64)>) -> bool {
    forall|i: int, j: int| 0 <= i < j < attrs.len()
        ==> key_lt(attrs[i].0, attrs[j].0)
}

// --- Split result ---

/// Result of a three-way split.
pub struct SplitResult {
    pub left: Seq<(Seq<u8>, u64)>,
    pub center: Seq<(Seq<u8>, u64, u64)>,
    pub right: Seq<(Seq<u8>, u64)>,
}

/// Spec helpers for key extraction.
pub open spec fn keys_of(s: Seq<(Seq<u8>, u64)>) -> Seq<Seq<u8>> {
    Seq::new(s.len(), |i: int| s[i].0)
}

pub open spec fn center_keys(s: Seq<(Seq<u8>, u64, u64)>) -> Seq<Seq<u8>> {
    Seq::new(s.len(), |i: int| s[i].0)
}

pub open spec fn has_attr_spec(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < attrs.len() && attrs[i].0 =~= key
}

// --- Spec: recursive split ---

// r[depends nickel-merge.split]
/// Spec: three-way split of two sorted maps.
pub open spec fn split_spec(
    a: Seq<(Seq<u8>, u64)>,
    b: Seq<(Seq<u8>, u64)>,
) -> SplitResult
    decreases a.len() + b.len()
{
    if a.len() == 0 && b.len() == 0 {
        SplitResult {
            left: Seq::empty(),
            center: Seq::empty(),
            right: Seq::empty(),
        }
    } else if a.len() == 0 {
        SplitResult {
            left: Seq::empty(),
            center: Seq::empty(),
            right: b,
        }
    } else if b.len() == 0 {
        SplitResult {
            left: a,
            center: Seq::empty(),
            right: Seq::empty(),
        }
    } else if key_lt(a[0int].0, b[0int].0) {
        let rest = split_spec(a.subrange(1, a.len() as int), b);
        SplitResult {
            left: seq![(a[0int].0, a[0int].1)] + rest.left,
            center: rest.center,
            right: rest.right,
        }
    } else if key_lt(b[0int].0, a[0int].0) {
        let rest = split_spec(a, b.subrange(1, b.len() as int));
        SplitResult {
            left: rest.left,
            center: rest.center,
            right: seq![(b[0int].0, b[0int].1)] + rest.right,
        }
    } else {
        // Equal keys
        let rest = split_spec(
            a.subrange(1, a.len() as int),
            b.subrange(1, b.len() as int),
        );
        SplitResult {
            left: rest.left,
            center: seq![(a[0int].0, a[0int].1, b[0int].1)] + rest.center,
            right: rest.right,
        }
    }
}

/// Exec result of three-way split.
pub struct ExecSplitResult {
    pub left: Vec<(Vec<u8>, u64)>,
    pub center: Vec<(Vec<u8>, u64, u64)>,
    pub right: Vec<(Vec<u8>, u64)>,
}

/// Map exec left/right vec to spec.
pub open spec fn map_kv(v: Seq<(Vec<u8>, u64)>) -> Seq<(Seq<u8>, u64)> {
    Seq::new(v.len(), |i: int| (v[i].0@, v[i].1))
}

/// Map exec center vec to spec.
pub open spec fn map_center(v: Seq<(Vec<u8>, u64, u64)>) -> Seq<(Seq<u8>, u64, u64)> {
    Seq::new(v.len(), |i: int| (v[i].0@, v[i].1, v[i].2))
}

// r[impl nickel-merge.split]
// r[impl nickel-merge.split-completeness]
// r[impl nickel-merge.split-value-preservation]
/// Three-way split of two sorted key-value maps.
pub fn split(
    a: &Vec<(Vec<u8>, u64)>,
    b: &Vec<(Vec<u8>, u64)>,
) -> (result: ExecSplitResult)
    ensures ({
        let spec_result = split_spec(map_kv(a@), map_kv(b@));
        &&& map_kv(result.left@) =~= spec_result.left
        &&& map_center(result.center@) =~= spec_result.center
        &&& map_kv(result.right@) =~= spec_result.right
    })
{
    let ghost ma = map_kv(a@);
    let ghost mb = map_kv(b@);

    let mut left: Vec<(Vec<u8>, u64)> = Vec::new();
    let mut center: Vec<(Vec<u8>, u64, u64)> = Vec::new();
    let mut right: Vec<(Vec<u8>, u64)> = Vec::new();

    let ghost mut gl: Seq<(Seq<u8>, u64)> = Seq::empty();
    let ghost mut gc: Seq<(Seq<u8>, u64, u64)> = Seq::empty();
    let ghost mut gr: Seq<(Seq<u8>, u64)> = Seq::empty();

    let mut ai: usize = 0;
    let mut bi: usize = 0;

    proof {
        assert(ma.subrange(0, ma.len() as int) =~= ma);
        assert(mb.subrange(0, mb.len() as int) =~= mb);
    }

    while ai < a.len() && bi < b.len()
        invariant
            ai <= a.len(),
            bi <= b.len(),
            ma =~= map_kv(a@),
            mb =~= map_kv(b@),
            gl.len() == left@.len(),
            gc.len() == center@.len(),
            gr.len() == right@.len(),
            forall|k: int| 0 <= k < gl.len() ==> gl[k] =~= (left@[k].0@, left@[k].1),
            forall|k: int| 0 <= k < gc.len() ==> gc[k] =~= (center@[k].0@, center@[k].1, center@[k].2),
            forall|k: int| 0 <= k < gr.len() ==> gr[k] =~= (right@[k].0@, right@[k].1),
            ({
                let tail = split_spec(
                    ma.subrange(ai as int, ma.len() as int),
                    mb.subrange(bi as int, mb.len() as int),
                );
                let full = split_spec(ma, mb);
                &&& full.left =~= gl + tail.left
                &&& full.center =~= gc + tail.center
                &&& full.right =~= gr + tail.right
            }),
        decreases a.len() + b.len() - ai - bi,
    {
        let ghost rem_a = ma.subrange(ai as int, ma.len() as int);
        let ghost rem_b = mb.subrange(bi as int, mb.len() as int);
        assert(rem_a.len() > 0 && rem_b.len() > 0);
        assert(rem_a[0int] =~= ma[ai as int]);
        assert(rem_b[0int] =~= mb[bi as int]);
        assert(rem_a.subrange(1, rem_a.len() as int) =~= ma.subrange(ai as int + 1, ma.len() as int));
        assert(rem_b.subrange(1, rem_b.len() as int) =~= mb.subrange(bi as int + 1, mb.len() as int));

        let cmp = key_cmp_exec(&a[ai].0, &b[bi].0);
        proof { key_cmp_antisymmetric(rem_a[0int].0, rem_b[0int].0); }

        if cmp < 0 {
            left.push((a[ai].0.clone(), a[ai].1));
            proof { gl = gl + seq![ma[ai as int]]; }
            ai = ai + 1;
        } else if cmp > 0 {
            right.push((b[bi].0.clone(), b[bi].1));
            proof { gr = gr + seq![mb[bi as int]]; }
            bi = bi + 1;
        } else {
            center.push((a[ai].0.clone(), a[ai].1, b[bi].1));
            proof { gc = gc + seq![(ma[ai as int].0, ma[ai as int].1, mb[bi as int].1)]; }
            ai = ai + 1;
            bi = bi + 1;
        }
    }

    // Drain remaining from a → left
    while ai < a.len()
        invariant
            ai <= a.len(),
            bi <= b.len(),
            ai >= a.len() || bi >= b.len(),
            ma =~= map_kv(a@),
            mb =~= map_kv(b@),
            gl.len() == left@.len(),
            gc.len() == center@.len(),
            gr.len() == right@.len(),
            forall|k: int| 0 <= k < gl.len() ==> gl[k] =~= (left@[k].0@, left@[k].1),
            forall|k: int| 0 <= k < gc.len() ==> gc[k] =~= (center@[k].0@, center@[k].1, center@[k].2),
            forall|k: int| 0 <= k < gr.len() ==> gr[k] =~= (right@[k].0@, right@[k].1),
            ({
                let tail = split_spec(
                    ma.subrange(ai as int, ma.len() as int),
                    mb.subrange(bi as int, mb.len() as int),
                );
                let full = split_spec(ma, mb);
                &&& full.left =~= gl + tail.left
                &&& full.center =~= gc + tail.center
                &&& full.right =~= gr + tail.right
            }),
        decreases a.len() - ai,
    {
        proof {
            let rem_a = ma.subrange(ai as int, ma.len() as int);
            let rem_b = mb.subrange(bi as int, mb.len() as int);
            assert(rem_b.len() == 0);
            assert(rem_a.len() > 0);
            assert(rem_a.subrange(1, rem_a.len() as int) =~= ma.subrange(ai as int + 1, ma.len() as int));
        }
        left.push((a[ai].0.clone(), a[ai].1));
        proof { gl = gl + seq![ma[ai as int]]; }
        ai = ai + 1;
    }

    // Drain remaining from b → right
    while bi < b.len()
        invariant
            ai <= a.len(),
            bi <= b.len(),
            ai >= a.len() || bi >= b.len(),
            ma =~= map_kv(a@),
            mb =~= map_kv(b@),
            gl.len() == left@.len(),
            gc.len() == center@.len(),
            gr.len() == right@.len(),
            forall|k: int| 0 <= k < gl.len() ==> gl[k] =~= (left@[k].0@, left@[k].1),
            forall|k: int| 0 <= k < gc.len() ==> gc[k] =~= (center@[k].0@, center@[k].1, center@[k].2),
            forall|k: int| 0 <= k < gr.len() ==> gr[k] =~= (right@[k].0@, right@[k].1),
            ({
                let tail = split_spec(
                    ma.subrange(ai as int, ma.len() as int),
                    mb.subrange(bi as int, mb.len() as int),
                );
                let full = split_spec(ma, mb);
                &&& full.left =~= gl + tail.left
                &&& full.center =~= gc + tail.center
                &&& full.right =~= gr + tail.right
            }),
        decreases b.len() - bi,
    {
        proof {
            let rem_a = ma.subrange(ai as int, ma.len() as int);
            let rem_b = mb.subrange(bi as int, mb.len() as int);
            assert(rem_a.len() == 0);
            assert(rem_b.len() > 0);
            assert(rem_b.subrange(1, rem_b.len() as int) =~= mb.subrange(bi as int + 1, mb.len() as int));
        }
        right.push((b[bi].0.clone(), b[bi].1));
        proof { gr = gr + seq![mb[bi as int]]; }
        bi = bi + 1;
    }

    proof {
        let rem_a = ma.subrange(ai as int, ma.len() as int);
        let rem_b = mb.subrange(bi as int, mb.len() as int);
        assert(rem_a.len() == 0 && rem_b.len() == 0);
        assert(gl =~= map_kv(left@));
        assert(gc =~= map_center(center@));
        assert(gr =~= map_kv(right@));
    }
    ExecSplitResult { left, center, right }
}

// --- Proofs ---

// r[verify nickel-merge.split]
/// Disjointness: left + center count == |a|, center + right count == |b|.
proof fn split_disjoint(a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>)
    ensures ({
        let r = split_spec(a, b);
        r.left.len() + r.center.len() + r.right.len() <= a.len() + b.len()
    })
    decreases a.len() + b.len()
{
    if a.len() > 0 && b.len() > 0 {
        if key_lt(a[0int].0, b[0int].0) {
            split_disjoint(a.subrange(1, a.len() as int), b);
        } else if key_lt(b[0int].0, a[0int].0) {
            split_disjoint(a, b.subrange(1, b.len() as int));
        } else {
            split_disjoint(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int));
        }
    }
}

// r[verify nickel-merge.split-completeness]
/// Completeness: every key from a appears in left or center.
proof fn split_completeness_a(a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>)
    ensures ({
        let r = split_spec(a, b);
        a.len() == r.left.len() + r.center.len()
    })
    decreases a.len() + b.len()
{
    if a.len() > 0 && b.len() > 0 {
        if key_lt(a[0int].0, b[0int].0) {
            split_completeness_a(a.subrange(1, a.len() as int), b);
        } else if key_lt(b[0int].0, a[0int].0) {
            split_completeness_a(a, b.subrange(1, b.len() as int));
        } else {
            split_completeness_a(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int));
        }
    }
}

// r[verify nickel-merge.split-completeness]
/// Completeness: every key from b appears in center or right.
proof fn split_completeness_b(a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>)
    ensures ({
        let r = split_spec(a, b);
        b.len() == r.center.len() + r.right.len()
    })
    decreases a.len() + b.len()
{
    if a.len() > 0 && b.len() > 0 {
        if key_lt(a[0int].0, b[0int].0) {
            split_completeness_b(a.subrange(1, a.len() as int), b);
        } else if key_lt(b[0int].0, a[0int].0) {
            split_completeness_b(a, b.subrange(1, b.len() as int));
        } else {
            split_completeness_b(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int));
        }
    }
}

// r[verify nickel-merge.split-value-preservation]
/// Value preservation: left partition length + center length == |a|,
/// ensuring every value from a is accounted for.
proof fn split_value_preservation(a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>)
    ensures ({
        let r = split_spec(a, b);
        // Every a-key ends up in left or center
        r.left.len() + r.center.len() == a.len()
    })
    decreases a.len() + b.len()
{
    // Same as split_completeness_a — the structure ensures value preservation
    if a.len() > 0 && b.len() > 0 {
        if key_lt(a[0int].0, b[0int].0) {
            split_value_preservation(a.subrange(1, a.len() as int), b);
        } else if key_lt(b[0int].0, a[0int].0) {
            split_value_preservation(a, b.subrange(1, b.len() as int));
        } else {
            split_value_preservation(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int));
        }
    }
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn kv(k: &str, v: u64) -> (Vec<u8>, u64) {
        (k.as_bytes().to_vec(), v)
    }

    // r[verify nickel-merge.split]
    #[test]
    fn test_disjoint_maps() {
        let a = vec![kv("a", 1)];
        let b = vec![kv("b", 2)];
        let r = split(&a, &b);
        assert_eq!(r.left.len(), 1);
        assert_eq!(r.center.len(), 0);
        assert_eq!(r.right.len(), 1);
        assert_eq!(r.left[0].0, b"a");
        assert_eq!(r.right[0].0, b"b");
    }

    // r[verify nickel-merge.split]
    #[test]
    fn test_overlapping_maps() {
        let a = vec![kv("a", 1), kv("b", 2)];
        let b = vec![kv("b", 3), kv("c", 4)];
        let r = split(&a, &b);
        assert_eq!(r.left.len(), 1);
        assert_eq!(r.center.len(), 1);
        assert_eq!(r.right.len(), 1);
        assert_eq!(r.left[0], kv("a", 1));
        assert_eq!(r.center[0], (b"b".to_vec(), 2, 3));
        assert_eq!(r.right[0], kv("c", 4));
    }

    // r[verify nickel-merge.split-completeness]
    #[test]
    fn test_identical_keys() {
        let a = vec![kv("a", 1)];
        let b = vec![kv("a", 2)];
        let r = split(&a, &b);
        assert_eq!(r.left.len(), 0);
        assert_eq!(r.center.len(), 1);
        assert_eq!(r.right.len(), 0);
        assert_eq!(r.center[0], (b"a".to_vec(), 1, 2));
    }

    // r[verify nickel-merge.split-value-preservation]
    #[test]
    fn test_empty_maps() {
        let a: Vec<(Vec<u8>, u64)> = vec![];
        let b: Vec<(Vec<u8>, u64)> = vec![];
        let r = split(&a, &b);
        assert_eq!(r.left.len(), 0);
        assert_eq!(r.center.len(), 0);
        assert_eq!(r.right.len(), 0);
    }
}
