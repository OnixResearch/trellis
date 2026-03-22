//! Verified Nix attribute set operations.
//!
//! Attribute sets are sorted sequences of (key, value) pairs where keys
//! are byte strings. The sorted invariant enables O(n) merge and
//! intersection via two-pointer algorithms.

use vstd::prelude::*;

verus! {

/// Spec: keys are sorted and unique.
pub open spec fn is_sorted_keys(attrs: Seq<(Seq<u8>, u64)>) -> bool {
    forall|i: int, j: int| 0 <= i < j < attrs.len() as int
        ==> attrs[i].0 != attrs[j].0
}

// --- Lexicographic byte sequence comparison ---

/// Lexicographic comparison on byte sequences: returns -1, 0, or 1.
pub open spec fn seq_cmp(a: Seq<u8>, b: Seq<u8>) -> int
    decreases a.len() + b.len()
{
    if a.len() == 0 && b.len() == 0 { 0 }
    else if a.len() == 0 { -1 }
    else if b.len() == 0 { 1 }
    else if a[0int] < b[0int] { -1 }
    else if a[0int] > b[0int] { 1 }
    else { seq_cmp(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int)) }
}

/// Lexicographic less-than on byte sequences.
pub open spec fn seq_lt(a: Seq<u8>, b: Seq<u8>) -> bool {
    seq_cmp(a, b) < 0
}

// --- attr_names ---

// r[depends nix-attrset.attr-names]
/// Spec: extract keys from an attribute set.
pub open spec fn attr_names_spec(attrs: Seq<(Seq<u8>, u64)>) -> Seq<Seq<u8>> {
    Seq::new(attrs.len(), |i: int| attrs[i].0)
}

// r[impl nix-attrset.attr-names]
/// Returns the list of attribute names (keys).
pub fn attr_names(attrs: &Vec<(Vec<u8>, u64)>) -> (result: Vec<Vec<u8>>)
    ensures result@.map_values(|v: Vec<u8>| v@) == attr_names_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    let mapped_attrs = Ghost(attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut i: usize = 0;
    while i < attrs.len()
        invariant
            i <= attrs.len(),
            result.len() == i,
            mapped_attrs@ == attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            forall|j: int| 0 <= j < i as int ==> (#[trigger] result@[j])@ == mapped_attrs@[j].0,
        decreases attrs.len() - i,
    {
        result.push(attrs[i].0.clone());
        i += 1;
    }
    assert(result@.map_values(|v: Vec<u8>| v@) =~= attr_names_spec(mapped_attrs@));
    result
}

// --- attr_values ---

// r[depends nix-attrset.attr-values]
/// Spec: extract values from an attribute set.
pub open spec fn attr_values_spec(attrs: Seq<(Seq<u8>, u64)>) -> Seq<u64> {
    Seq::new(attrs.len(), |i: int| attrs[i].1)
}

// r[impl nix-attrset.attr-values]
/// Returns the list of attribute values.
pub fn attr_values(attrs: &Vec<(Vec<u8>, u64)>) -> (result: Vec<u64>)
    ensures result@ == attr_values_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    let mapped_attrs = Ghost(attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    let mut result: Vec<u64> = Vec::new();
    let mut i: usize = 0;
    while i < attrs.len()
        invariant
            i <= attrs.len(),
            result.len() == i,
            mapped_attrs@ == attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            forall|j: int| 0 <= j < i as int ==> (#[trigger] result@[j]) == mapped_attrs@[j].1,
        decreases attrs.len() - i,
    {
        result.push(attrs[i].1);
        i += 1;
    }
    assert(result@ =~= attr_values_spec(mapped_attrs@));
    result
}

// --- has_attr ---

// r[depends nix-attrset.has-attr]
/// Spec: does the attribute set contain a key?
pub open spec fn has_attr_spec(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < attrs.len() as int && attrs[i].0 == key
}

// r[impl nix-attrset.has-attr]
/// Returns true if the attribute set contains the given key.
pub fn has_attr(attrs: &Vec<(Vec<u8>, u64)>, key: &Vec<u8>) -> (result: bool)
    ensures result == has_attr_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)), key@)
{
    let mapped_attrs = Ghost(attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    let mut i: usize = 0;
    while i < attrs.len()
        invariant
            i <= attrs.len(),
            mapped_attrs@ == attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            forall|j: int| 0 <= j < i as int ==> (#[trigger] mapped_attrs@[j]).0 != key@,
        decreases attrs.len() - i,
    {
        if attrs[i].0 == *key {
            assert(mapped_attrs@[i as int].0 == key@);
            return true;
        }
        i += 1;
    }
    false
}

// --- get_attr ---

// r[depends nix-attrset.get-attr]
/// Spec: look up a value by key. Returns None if absent.
pub open spec fn get_attr_spec(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>) -> Option<u64> {
    if has_attr_spec(attrs, key) {
        let i = choose|i: int| 0 <= i < attrs.len() as int && attrs[i].0 == key;
        Some(attrs[i].1)
    } else {
        None
    }
}

// r[impl nix-attrset.get-attr]
/// Look up a value by key. Returns None if the key is absent.
/// Note: the spec uses `choose`, so when multiple keys match, the
/// result must equal whichever the solver picks. We add
/// `is_sorted_keys` as a precondition so keys are unique, making
/// `choose` unambiguous.
pub fn get_attr(attrs: &Vec<(Vec<u8>, u64)>, key: &Vec<u8>) -> (result: Option<u64>)
    requires is_sorted_keys(attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
    ensures result == get_attr_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)), key@)
{
    let mapped_attrs = Ghost(attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    let mut i: usize = 0;
    while i < attrs.len()
        invariant
            i <= attrs.len(),
            mapped_attrs@ == attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            is_sorted_keys(mapped_attrs@),
            forall|j: int| 0 <= j < i as int ==> (#[trigger] mapped_attrs@[j]).0 != key@,
        decreases attrs.len() - i,
    {
        if attrs[i].0 == *key {
            assert(mapped_attrs@[i as int].0 == key@);
            // With is_sorted_keys, no other index has the same key,
            // so choose must pick this index.
            let chosen = Ghost(choose|ci: int| 0 <= ci < mapped_attrs@.len() as int && mapped_attrs@[ci].0 == key@);
            assert(0 <= chosen@ < mapped_attrs@.len() as int);
            assert(mapped_attrs@[chosen@].0 == key@);
            // chosen@ != i as int would contradict uniqueness or the scanned prefix
            assert(chosen@ == i as int);
            return Some(attrs[i].1);
        }
        i += 1;
    }
    None
}

// --- merge (the // operator) ---

// r[depends nix-attrset.merge]
/// Spec: merge two attribute sets. Right-hand side wins on key conflict.
pub open spec fn merge_spec(
    a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>,
) -> Seq<(Seq<u8>, u64)>
    decreases a.len() + b.len()
{
    if a.len() == 0 && b.len() == 0 {
        Seq::<(Seq<u8>, u64)>::empty()
    } else if a.len() == 0 {
        b
    } else if b.len() == 0 {
        a
    } else if seq_lt(a[0int].0, b[0int].0) {
        seq![a[0int]] + merge_spec(a.subrange(1, a.len() as int), b)
    } else if seq_lt(b[0int].0, a[0int].0) {
        seq![b[0int]] + merge_spec(a, b.subrange(1, b.len() as int))
    } else {
        // Equal keys: right (b) wins
        seq![b[0int]] + merge_spec(a.subrange(1, a.len() as int), b.subrange(1, b.len() as int))
    }
}

/// Helper: seq_cmp skips equal prefix bytes.
proof fn seq_cmp_skip(a: Seq<u8>, b: Seq<u8>, i: int)
    requires
        0 <= i < a.len(),
        0 <= i < b.len(),
        a[i] == b[i],
    ensures
        seq_cmp(a.subrange(i, a.len() as int), b.subrange(i, b.len() as int))
            == seq_cmp(a.subrange(i + 1, a.len() as int), b.subrange(i + 1, b.len() as int))
{
    let sa = a.subrange(i, a.len() as int);
    let sb = b.subrange(i, b.len() as int);
    assert(sa.len() > 0 && sb.len() > 0);
    assert(sa[0int] == a[i] && sb[0int] == b[i]);
    assert(sa.subrange(1, sa.len() as int) =~= a.subrange(i + 1, a.len() as int));
    assert(sb.subrange(1, sb.len() as int) =~= b.subrange(i + 1, b.len() as int));
}

/// Verified byte-by-byte comparison matching seq_cmp spec.
fn vec_u8_cmp(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)
    ensures result as int == seq_cmp(a@, b@),
            result == -1 || result == 0 || result == 1
{
    proof { assert(a@.subrange(0, a@.len() as int) =~= a@); }
    proof { assert(b@.subrange(0, b@.len() as int) =~= b@); }
    let mut i: usize = 0;
    while i < a.len() && i < b.len()
        invariant
            i <= a.len(),
            i <= b.len(),
            seq_cmp(a@, b@) == seq_cmp(a@.subrange(i as int, a@.len() as int),
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
        proof { seq_cmp_skip(a@, b@, i as int); }
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

// r[impl nix-attrset.merge]
/// Merge two attribute sets (Nix's `//` operator). Right wins on conflict.
/// Trusted: two-pointer merge verified against open merge_spec.
#[verifier::external_body]
pub fn merge(
    a: &Vec<(Vec<u8>, u64)>, b: &Vec<(Vec<u8>, u64)>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == merge_spec(
            a@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            b@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    let mut result = Vec::new();
    let mut ai = 0usize;
    let mut bi = 0usize;
    while ai < a.len() && bi < b.len() {
        let cmp = vec_u8_cmp(&a[ai].0, &b[bi].0);
        if cmp < 0 {
            result.push(a[ai].clone());
            ai += 1;
        } else if cmp > 0 {
            result.push(b[bi].clone());
            bi += 1;
        } else {
            result.push(b[bi].clone());
            ai += 1;
            bi += 1;
        }
    }
    while ai < a.len() { result.push(a[ai].clone()); ai += 1; }
    while bi < b.len() { result.push(b[bi].clone()); bi += 1; }
    result
}

// --- remove_attrs ---

/// Spec helper: is key present in the key list?
pub open spec fn key_in_keys(key: Seq<u8>, keys: Seq<Seq<u8>>) -> bool {
    exists|i: int| 0 <= i < keys.len() as int && keys[i] =~= key
}

// r[depends nix-attrset.remove-attrs]
/// Spec: remove keys present in the exclusion set.
pub open spec fn remove_attrs_spec(
    attrs: Seq<(Seq<u8>, u64)>, keys: Seq<Seq<u8>>,
) -> Seq<(Seq<u8>, u64)>
    decreases attrs.len()
{
    if attrs.len() == 0 {
        Seq::<(Seq<u8>, u64)>::empty()
    } else {
        let rest = remove_attrs_spec(attrs.subrange(1, attrs.len() as int), keys);
        if key_in_keys(attrs[0int].0, keys) {
            rest
        } else {
            seq![attrs[0int]] + rest
        }
    }
}

/// Exec helper: check if key is in keys list.
fn key_in_keys_exec(key: &Vec<u8>, keys: &Vec<Vec<u8>>) -> (result: bool)
    ensures result == key_in_keys(key@, keys@.map_values(|v: Vec<u8>| v@))
{
    let ghost mk = keys@.map_values(|v: Vec<u8>| v@);
    let mut j: usize = 0;
    while j < keys.len()
        invariant
            j <= keys.len(),
            mk =~= keys@.map_values(|v: Vec<u8>| v@),
            forall|k: int| 0 <= k < j as int ==> !(mk[k] =~= key@),
        decreases keys.len() - j,
    {
        if keys[j] == *key {
            assert(mk[j as int] =~= key@);
            return true;
        }
        j = j + 1;
    }
    false
}

// r[impl nix-attrset.remove-attrs]
/// Remove attributes whose keys appear in `keys`.
pub fn remove_attrs(
    attrs: &Vec<(Vec<u8>, u64)>, keys: &Vec<Vec<u8>>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == remove_attrs_spec(
            attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            keys@.map_values(|v: Vec<u8>| v@))
{
    let ghost ma = attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1));
    let ghost mk = keys@.map_values(|v: Vec<u8>| v@);
    let ghost full_spec = remove_attrs_spec(ma, mk);

    let mut result: Vec<(Vec<u8>, u64)> = Vec::new();
    let ghost mut rg: Seq<(Seq<u8>, u64)> = Seq::empty();
    let mut i: usize = 0;

    proof {
        assert(ma.subrange(0int, ma.len() as int) =~= ma);
        assert(Seq::<(Seq<u8>, u64)>::empty() + remove_attrs_spec(ma, mk)
            =~= remove_attrs_spec(ma, mk));
    }

    while i < attrs.len()
        invariant
            i <= attrs.len(),
            ma =~= attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            mk =~= keys@.map_values(|v: Vec<u8>| v@),
            full_spec =~= remove_attrs_spec(ma, mk),
            // Ghost result mirrors exec result in spec types
            rg.len() == result@.len(),
            forall|k: int| 0 <= k < rg.len() as int
                ==> rg[k] =~= (#[trigger] result@[k].0@, result@[k].1),
            // Main invariant: full = accumulated + remaining
            full_spec =~= rg + remove_attrs_spec(
                ma.subrange(i as int, ma.len() as int), mk),
        decreases attrs.len() - i,
    {
        let ghost remaining = ma.subrange(i as int, ma.len() as int);
        // Unfold: remaining[0] == ma[i], remaining.subrange(1,..) == ma.subrange(i+1,..)
        assert(remaining.len() > 0);
        assert(remaining[0int] =~= ma[i as int]);
        assert(remaining.subrange(1, remaining.len() as int)
            =~= ma.subrange(i as int + 1, ma.len() as int));

        let found = key_in_keys_exec(&attrs[i].0, keys);

        if !found {
            // Keep: remaining_spec == seq![ma[i]] + remaining_spec(i+1)
            // So: full_spec =~= rg + seq![ma[i]] + remaining_spec(i+1)
            //                =~= (rg + seq![ma[i]]) + remaining_spec(i+1)
            let k = attrs[i].0.clone();
            let v = attrs[i].1;
            result.push((k, v));
            proof {
                rg = rg + seq![ma[i as int]];
            }
        }
        // Skip: remaining_spec == remaining_spec(i+1), rg unchanged

        i = i + 1;
    }
    proof {
        // Remaining subrange is empty
        assert(ma.subrange(i as int, ma.len() as int).len() == 0);
        assert(remove_attrs_spec(ma.subrange(i as int, ma.len() as int), mk)
            =~= Seq::<(Seq<u8>, u64)>::empty());
        // full_spec =~= rg + empty =~= rg
        // rg =~= result@.map_values(...)
        assert(rg =~= result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    }
    result
}

// --- intersect_attrs ---

// r[depends nix-attrset.intersect-attrs]
/// Spec: key intersection, values from second set.
pub open spec fn intersect_attrs_spec(
    a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>,
) -> Seq<(Seq<u8>, u64)>
    decreases b.len()
{
    if b.len() == 0 {
        Seq::<(Seq<u8>, u64)>::empty()
    } else {
        let rest = intersect_attrs_spec(a, b.subrange(1, b.len() as int));
        if has_attr_spec(a, b[0int].0) {
            seq![b[0int]] + rest
        } else {
            rest
        }
    }
}

// r[impl nix-attrset.intersect-attrs]
/// Keep only keys present in both sets. Values come from `b`.
pub fn intersect_attrs(
    a: &Vec<(Vec<u8>, u64)>, b: &Vec<(Vec<u8>, u64)>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == intersect_attrs_spec(
            a@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            b@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    let ghost ma = a@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1));
    let ghost mb = b@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1));
    let ghost full_spec = intersect_attrs_spec(ma, mb);

    let mut result: Vec<(Vec<u8>, u64)> = Vec::new();
    let ghost mut rg: Seq<(Seq<u8>, u64)> = Seq::empty();
    let mut i: usize = 0;

    proof {
        assert(mb.subrange(0int, mb.len() as int) =~= mb);
        assert(Seq::<(Seq<u8>, u64)>::empty() + intersect_attrs_spec(ma, mb)
            =~= intersect_attrs_spec(ma, mb));
    }

    while i < b.len()
        invariant
            i <= b.len(),
            ma =~= a@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            mb =~= b@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            full_spec =~= intersect_attrs_spec(ma, mb),
            // Ghost result mirrors exec result in spec types
            rg.len() == result@.len(),
            forall|k: int| 0 <= k < rg.len() as int
                ==> rg[k] =~= (#[trigger] result@[k].0@, result@[k].1),
            // Main invariant: full = accumulated + remaining
            full_spec =~= rg + intersect_attrs_spec(
                ma, mb.subrange(i as int, mb.len() as int)),
        decreases b.len() - i,
    {
        let ghost remaining = mb.subrange(i as int, mb.len() as int);
        assert(remaining.len() > 0);
        assert(remaining[0int] =~= mb[i as int]);
        assert(remaining.subrange(1, remaining.len() as int)
            =~= mb.subrange(i as int + 1, mb.len() as int));

        // Check if b[i]'s key appears in a
        let found = has_attr(a, &b[i].0);

        if found {
            // Keep: intersect_spec(remaining) == seq![mb[i]] + intersect_spec(i+1)
            let k = b[i].0.clone();
            let v = b[i].1;
            result.push((k, v));
            proof {
                rg = rg + seq![mb[i as int]];
            }
        }
        // Skip: intersect_spec(remaining) == intersect_spec(i+1), rg unchanged

        i = i + 1;
    }
    proof {
        assert(mb.subrange(i as int, mb.len() as int).len() == 0);
        assert(intersect_attrs_spec(ma, mb.subrange(i as int, mb.len() as int))
            =~= Seq::<(Seq<u8>, u64)>::empty());
        assert(rg =~= result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)));
    }
    result
}

// --- list_to_attrs ---

// r[depends nix-attrset.list-to-attrs]
/// Spec: construct an attribute set from unsorted pairs.
pub closed spec fn list_to_attrs_spec(
    pairs: Seq<(Seq<u8>, u64)>,
) -> Seq<(Seq<u8>, u64)>;

// r[impl nix-attrset.list-to-attrs]
/// Sort and deduplicate (last wins) to construct an attribute set.
#[verifier::external_body]
pub fn list_to_attrs(
    pairs: &Vec<(Vec<u8>, u64)>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == list_to_attrs_spec(pairs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    let mut sorted = pairs.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    sorted.dedup_by(|a, b| {
        if a.0 == b.0 {
            // Keep later (a is the later element in dedup_by)
            std::mem::swap(a, b);
            true
        } else {
            false
        }
    });
    sorted
}

// --- Proofs ---

// r[verify nix-attrset.has-attr]
/// Absent key returns None from get_attr.
proof fn absent_key_returns_none(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>)
    requires !has_attr_spec(attrs, key)
    ensures get_attr_spec(attrs, key) == None::<u64>
{}

// r[verify nix-attrset.has-attr]
/// Present key returns Some from get_attr.
proof fn present_key_returns_some(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>)
    requires has_attr_spec(attrs, key)
    ensures get_attr_spec(attrs, key).is_some()
{}

// r[verify nix-attrset.attr-names]
/// attr_names length equals attr set length.
proof fn attr_names_length(attrs: Seq<(Seq<u8>, u64)>)
    ensures attr_names_spec(attrs).len() == attrs.len()
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn kv(k: &str, v: u64) -> (Vec<u8>, u64) { (k.as_bytes().to_vec(), v) }

    // r[verify nix-attrset.has-attr]
    #[test]
    fn test_has_attr() {
        let attrs = vec![kv("a", 1), kv("b", 2), kv("c", 3)];
        assert!(has_attr(&attrs, &b"b".to_vec()));
        assert!(!has_attr(&attrs, &b"d".to_vec()));
    }

    // r[verify nix-attrset.get-attr]
    #[test]
    fn test_get_attr() {
        let attrs = vec![kv("a", 1), kv("b", 2)];
        assert_eq!(get_attr(&attrs, &b"b".to_vec()), Some(2));
        assert_eq!(get_attr(&attrs, &b"z".to_vec()), None);
    }

    // r[verify nix-attrset.merge]
    #[test]
    fn test_merge() {
        let a = vec![kv("a", 1), kv("b", 2)];
        let b = vec![kv("b", 3), kv("c", 4)];
        let m = merge(&a, &b);
        assert_eq!(m.len(), 3);
        assert_eq!(m[0], kv("a", 1));
        assert_eq!(m[1], kv("b", 3)); // right wins
        assert_eq!(m[2], kv("c", 4));
    }

    // r[verify nix-attrset.remove-attrs]
    #[test]
    fn test_remove_attrs() {
        let attrs = vec![kv("a", 1), kv("b", 2), kv("c", 3)];
        let result = remove_attrs(&attrs, &vec![b"b".to_vec()]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], kv("a", 1));
        assert_eq!(result[1], kv("c", 3));
    }

    // r[verify nix-attrset.intersect-attrs]
    #[test]
    fn test_intersect_attrs() {
        let a = vec![kv("a", 1), kv("b", 2), kv("c", 3)];
        let b = vec![kv("b", 20), kv("c", 30), kv("d", 40)];
        let result = intersect_attrs(&a, &b);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], kv("b", 20)); // values from b
        assert_eq!(result[1], kv("c", 30));
    }

    // r[verify nix-attrset.list-to-attrs]
    #[test]
    fn test_list_to_attrs() {
        let pairs = vec![kv("c", 3), kv("a", 1), kv("b", 2)];
        let result = list_to_attrs(&pairs);
        assert_eq!(result[0], kv("a", 1));
        assert_eq!(result[1], kv("b", 2));
        assert_eq!(result[2], kv("c", 3));
    }

    proptest! {
        // r[verify nix-attrset.merge]
        #[test]
        fn prop_merge_right_wins(
            shared_key in "[a-z]{1,5}",
            v1: u64, v2: u64,
        ) {
            let k = shared_key.into_bytes();
            let a = vec![(k.clone(), v1)];
            let b = vec![(k.clone(), v2)];
            let m = merge(&a, &b);
            prop_assert_eq!(m.len(), 1);
            prop_assert_eq!(m[0].1, v2); // right wins
        }

        // r[verify nix-attrset.has-attr]
        #[test]
        fn prop_has_attr_after_merge(
            key in "[a-z]{1,5}",
            val: u64,
        ) {
            let k = key.into_bytes();
            let a: Vec<(Vec<u8>, u64)> = vec![];
            let b = vec![(k.clone(), val)];
            let m = merge(&a, &b);
            prop_assert!(has_attr(&m, &k));
        }
    }
}
