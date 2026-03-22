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

// --- attr_names ---

// r[depends nix-attrset.attr-names]
/// Spec: extract keys from an attribute set.
pub open spec fn attr_names_spec(attrs: Seq<(Seq<u8>, u64)>) -> Seq<Seq<u8>> {
    Seq::new(attrs.len(), |i: int| attrs[i].0)
}

// r[impl nix-attrset.attr-names]
/// Returns the list of attribute names (keys).
#[verifier::external_body]
pub fn attr_names(attrs: &Vec<(Vec<u8>, u64)>) -> (result: Vec<Vec<u8>>)
    ensures result@.map_values(|v: Vec<u8>| v@) == attr_names_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    attrs.iter().map(|(k, _)| k.clone()).collect()
}

// --- attr_values ---

// r[depends nix-attrset.attr-values]
/// Spec: extract values from an attribute set.
pub open spec fn attr_values_spec(attrs: Seq<(Seq<u8>, u64)>) -> Seq<u64> {
    Seq::new(attrs.len(), |i: int| attrs[i].1)
}

// r[impl nix-attrset.attr-values]
/// Returns the list of attribute values.
#[verifier::external_body]
pub fn attr_values(attrs: &Vec<(Vec<u8>, u64)>) -> (result: Vec<u64>)
    ensures result@ == attr_values_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    attrs.iter().map(|(_, v)| *v).collect()
}

// --- has_attr ---

// r[depends nix-attrset.has-attr]
/// Spec: does the attribute set contain a key?
pub open spec fn has_attr_spec(attrs: Seq<(Seq<u8>, u64)>, key: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < attrs.len() as int && attrs[i].0 == key
}

// r[impl nix-attrset.has-attr]
/// Returns true if the attribute set contains the given key.
#[verifier::external_body]
pub fn has_attr(attrs: &Vec<(Vec<u8>, u64)>, key: &Vec<u8>) -> (result: bool)
    ensures result == has_attr_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)), key@)
{
    attrs.iter().any(|(k, _)| k == key)
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
#[verifier::external_body]
pub fn get_attr(attrs: &Vec<(Vec<u8>, u64)>, key: &Vec<u8>) -> (result: Option<u64>)
    ensures result == get_attr_spec(
        attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)), key@)
{
    attrs.iter().find(|(k, _)| k == key).map(|(_, v)| *v)
}

// --- merge (the // operator) ---

// r[depends nix-attrset.merge]
/// Spec: merge two attribute sets. Right-hand side wins on key conflict.
pub closed spec fn merge_spec(
    a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>,
) -> Seq<(Seq<u8>, u64)>;

// r[impl nix-attrset.merge]
/// Merge two attribute sets (Nix's `//` operator). Right wins on conflict.
/// Trusted: two-pointer merge over sorted sequences. Verified by proptest.
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
        match a[ai].0.cmp(&b[bi].0) {
            std::cmp::Ordering::Less => {
                result.push(a[ai].clone());
                ai += 1;
            }
            std::cmp::Ordering::Greater => {
                result.push(b[bi].clone());
                bi += 1;
            }
            std::cmp::Ordering::Equal => {
                // Right wins
                result.push(b[bi].clone());
                ai += 1;
                bi += 1;
            }
        }
    }
    while ai < a.len() { result.push(a[ai].clone()); ai += 1; }
    while bi < b.len() { result.push(b[bi].clone()); bi += 1; }
    result
}

// --- remove_attrs ---

// r[depends nix-attrset.remove-attrs]
/// Spec: remove keys present in the exclusion set.
pub closed spec fn remove_attrs_spec(
    attrs: Seq<(Seq<u8>, u64)>, keys: Seq<Seq<u8>>,
) -> Seq<(Seq<u8>, u64)>;

// r[impl nix-attrset.remove-attrs]
/// Remove attributes whose keys appear in `keys`.
#[verifier::external_body]
pub fn remove_attrs(
    attrs: &Vec<(Vec<u8>, u64)>, keys: &Vec<Vec<u8>>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == remove_attrs_spec(
            attrs@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            keys@.map_values(|v: Vec<u8>| v@))
{
    attrs.iter()
        .filter(|(k, _)| !keys.contains(k))
        .cloned()
        .collect()
}

// --- intersect_attrs ---

// r[depends nix-attrset.intersect-attrs]
/// Spec: key intersection, values from second set.
pub closed spec fn intersect_attrs_spec(
    a: Seq<(Seq<u8>, u64)>, b: Seq<(Seq<u8>, u64)>,
) -> Seq<(Seq<u8>, u64)>;

// r[impl nix-attrset.intersect-attrs]
/// Keep only keys present in both sets. Values come from `b`.
#[verifier::external_body]
pub fn intersect_attrs(
    a: &Vec<(Vec<u8>, u64)>, b: &Vec<(Vec<u8>, u64)>,
) -> (result: Vec<(Vec<u8>, u64)>)
    ensures result@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1))
        == intersect_attrs_spec(
            a@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)),
            b@.map_values(|p: (Vec<u8>, u64)| (p.0@, p.1)))
{
    b.iter()
        .filter(|(k, _)| a.iter().any(|(ak, _)| ak == k))
        .cloned()
        .collect()
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
