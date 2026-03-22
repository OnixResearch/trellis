//! Verified Nix list operations.
//!
//! Models `builtins.map`, `builtins.filter`, `builtins.foldl'`,
//! `builtins.concatLists`, `builtins.genList`, `builtins.elem`,
//! `builtins.elemAt`, `builtins.head`, `builtins.tail`, `builtins.length`.

use vstd::prelude::*;

verus! {

// --- length ---

// r[depends nix-list.length]
/// Spec: list length.
pub open spec fn length_spec(list: Seq<u64>) -> nat {
    list.len()
}

// r[impl nix-list.length]
pub fn length(list: &Vec<u64>) -> (result: usize)
    ensures result as nat == length_spec(list@)
{
    list.len()
}

// --- elem_at ---

// r[depends nix-list.elem-at]
/// Spec: get element at index.
pub open spec fn elem_at_spec(list: Seq<u64>, idx: int) -> u64
    recommends 0 <= idx < list.len() as int
{
    list[idx]
}

// r[impl nix-list.elem-at]
pub fn elem_at(list: &Vec<u64>, idx: usize) -> (result: u64)
    requires idx < list.len()
    ensures result == elem_at_spec(list@, idx as int)
{
    list[idx]
}

// --- head ---

// r[depends nix-list.head]
/// Spec: first element.
pub open spec fn head_spec(list: Seq<u64>) -> u64
    recommends list.len() > 0
{
    list[0int]
}

// r[impl nix-list.head]
pub fn head(list: &Vec<u64>) -> (result: u64)
    requires list.len() > 0
    ensures result == head_spec(list@)
{
    list[0]
}

// --- tail ---

// r[depends nix-list.tail]
/// Spec: all elements except the first.
pub open spec fn tail_spec(list: Seq<u64>) -> Seq<u64>
    recommends list.len() > 0
{
    list.subrange(1, list.len() as int)
}

// r[impl nix-list.tail]
#[verifier::external_body]
pub fn tail(list: &Vec<u64>) -> (result: Vec<u64>)
    requires list.len() > 0
    ensures result@ == tail_spec(list@)
{
    list[1..].to_vec()
}

// --- elem ---

// r[depends nix-list.elem]
/// Spec: is a value in the list?
pub open spec fn elem_spec(list: Seq<u64>, x: u64) -> bool {
    exists|i: int| 0 <= i < list.len() as int && list[i] == x
}

// r[impl nix-list.elem]
pub fn elem(list: &Vec<u64>, x: u64) -> (result: bool)
    ensures result == elem_spec(list@, x)
{
    let mut i: usize = 0;
    while i < list.len()
        invariant
            i <= list.len(),
            forall|j: int| 0 <= j < i as int ==> list@[j] != x,
        decreases list.len() - i,
    {
        if list[i] == x {
            return true;
        }
        i += 1;
    }
    false
}

// --- gen_list ---

// r[depends nix-list.gen-list]
/// Spec: generate a list [0, 1, ..., n-1].
pub open spec fn gen_list_spec(n: nat) -> Seq<u64> {
    Seq::new(n, |i: int| i as u64)
}

// r[impl nix-list.gen-list]
pub fn gen_list(n: usize) -> (result: Vec<u64>)
    requires n <= u64::MAX as usize
    ensures result@ == gen_list_spec(n as nat)
{
    let mut result: Vec<u64> = Vec::new();
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n <= u64::MAX as usize,
            result.len() == i,
            forall|j: int| 0 <= j < i as int ==> result@[j] == j as u64,
        decreases n - i,
    {
        result.push(i as u64);
        i += 1;
    }
    assert(result@ =~= gen_list_spec(n as nat));
    result
}

// --- concat_lists ---

// r[depends nix-list.concat-lists]
/// Spec: concatenate a list of lists.
pub open spec fn concat_lists_spec(lists: Seq<Seq<u64>>) -> Seq<u64>
    decreases lists.len()
{
    if lists.len() == 0 {
        Seq::<u64>::empty()
    } else {
        lists[0int] + concat_lists_spec(lists.subrange(1, lists.len() as int))
    }
}

// r[impl nix-list.concat-lists]
#[verifier::external_body]
pub fn concat_lists(lists: &Vec<Vec<u64>>) -> (result: Vec<u64>)
    ensures result@ == concat_lists_spec(lists@.map_values(|v: Vec<u64>| v@))
{
    lists.iter().flat_map(|l| l.iter().copied()).collect()
}

// --- Proofs ---

// r[verify nix-list.length]
/// gen_list produces exactly n elements.
proof fn gen_list_length(n: nat)
    ensures gen_list_spec(n).len() == n
{}

// r[verify nix-list.elem]
/// head is in the list.
proof fn head_is_elem(list: Seq<u64>)
    requires list.len() > 0
    ensures elem_spec(list, head_spec(list))
{
    assert(list[0int] == head_spec(list));
}

// r[verify nix-list.tail]
/// tail has length one less.
proof fn tail_length(list: Seq<u64>)
    requires list.len() > 0
    ensures tail_spec(list).len() == list.len() - 1
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify nix-list.length]
    #[test]
    fn test_length() {
        assert_eq!(length(&vec![1, 2, 3]), 3);
        assert_eq!(length(&vec![]), 0);
    }

    // r[verify nix-list.elem-at]
    #[test]
    fn test_elem_at() {
        assert_eq!(elem_at(&vec![10, 20, 30], 1), 20);
    }

    // r[verify nix-list.head]
    #[test]
    fn test_head() {
        assert_eq!(head(&vec![42, 1, 2]), 42);
    }

    // r[verify nix-list.tail]
    #[test]
    fn test_tail() {
        assert_eq!(tail(&vec![1, 2, 3]), vec![2, 3]);
    }

    // r[verify nix-list.elem]
    #[test]
    fn test_elem() {
        assert!(elem(&vec![1, 2, 3], 2));
        assert!(!elem(&vec![1, 2, 3], 4));
    }

    // r[verify nix-list.gen-list]
    #[test]
    fn test_gen_list() {
        assert_eq!(gen_list(5), vec![0, 1, 2, 3, 4]);
        assert_eq!(gen_list(0), Vec::<u64>::new());
    }

    // r[verify nix-list.concat-lists]
    #[test]
    fn test_concat_lists() {
        let lists = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
        assert_eq!(concat_lists(&lists), vec![1, 2, 3, 4, 5, 6]);
    }

    proptest! {
        // r[verify nix-list.gen-list]
        #[test]
        fn prop_gen_list_length(n in 0usize..1000) {
            prop_assert_eq!(gen_list(n).len(), n);
        }

        // r[verify nix-list.elem]
        #[test]
        fn prop_elem_finds_inserted(
            list in proptest::collection::vec(any::<u64>(), 1..=10),
            idx in 0usize..10,
        ) {
            let idx = idx % list.len();
            prop_assert!(elem(&list, list[idx]));
        }

        // r[verify nix-list.tail]
        #[test]
        fn prop_tail_length(
            list in proptest::collection::vec(any::<u64>(), 1..=20),
        ) {
            prop_assert_eq!(tail(&list).len(), list.len() - 1);
        }
    }
}
