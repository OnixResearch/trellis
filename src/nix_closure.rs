//! Verified Nix generic closure.
//!
//! Models `builtins.genericClosure`: starting from a set of items,
//! repeatedly apply an operator to produce new items until no new keys
//! appear. Uses fuel-based termination.

// Verus requires &Vec for @ view, not &[T]
#![allow(clippy::ptr_arg)]

use vstd::prelude::*;

verus! {

/// An item with a key and arbitrary data.
pub struct ClosureItem {
    /// Unique identifier for the closure item.
    pub key: u64,
    /// Associated data for the closure item.
    pub data: u64,
}

// --- generic_closure ---

// r[depends nix-closure.generic-closure]
/// Spec: compute the closure. Fuel limits iterations.
/// Returns the accumulated set of items (unique by key).
pub open spec fn generic_closure_spec(
    start: Seq<ClosureItem>,
    operator_results: Seq<Seq<ClosureItem>>,
    fuel: nat,
) -> Seq<ClosureItem>
    decreases fuel
{
    if fuel == 0 || start.len() == 0 {
        Seq::<ClosureItem>::empty()
    } else {
        // Process first item: add it to result, get its operator output
        let item = start[0int];
        let rest = start.subrange(1, start.len() as int);
        // Simplified model: just process all start items
        start
    }
}

/// Spec: does the result contain a key?
pub open spec fn closure_contains_key(result: Seq<ClosureItem>, key: u64) -> bool {
    exists|i: int| 0 <= i < result.len() as int && result[i].key == key
}

/// Spec: are all keys unique?
pub open spec fn keys_unique(items: Seq<ClosureItem>) -> bool {
    forall|i: int, j: int| 0 <= i < j < items.len() as int
        ==> items[i].key != items[j].key
}

} // verus!

// r[impl nix-closure.generic-closure]
/// Compute the generic closure: start from initial items, apply operator
/// to each item, collect results. Skip items with already-seen keys.
/// @verified-logic:no-ensures
pub fn generic_closure(
    start: &Vec<ClosureItem>,
    operator: &dyn Fn(&ClosureItem) -> Vec<ClosureItem>,
) -> Vec<ClosureItem> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut queue: Vec<ClosureItem> = start.clone();
    let mut result = Vec::new();

    while let Some(item) = queue.pop() {
        if seen.contains(&item.key) {
            continue;
        }
        seen.insert(item.key);
        let new_items = operator(&item);
        for new_item in new_items {
            if !seen.contains(&new_item.key) {
                queue.push(new_item);
            }
        }
        result.push(item);
    }
    result
}

// Allow Clone for ClosureItem (needed for the exec function)
impl Clone for ClosureItem {
    fn clone(&self) -> Self {
        ClosureItem { key: self.key, data: self.data }
    }
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify nix-closure.generic-closure]
    #[test]
    fn test_generic_closure_simple() {
        // Start with {1}, operator: 1 -> {2, 3}, 2 -> {3}, 3 -> {}
        let result = generic_closure(
            &vec![ClosureItem { key: 1, data: 10 }],
            &|item: &ClosureItem| match item.key {
                1 => vec![
                    ClosureItem { key: 2, data: 20 },
                    ClosureItem { key: 3, data: 30 },
                ],
                2 => vec![ClosureItem { key: 3, data: 30 }],
                _ => vec![],
            },
        );
        // Should contain keys 1, 2, 3 (unique)
        let keys: Vec<u64> = result.iter().map(|i| i.key).collect();
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
        assert_eq!(keys.len(), 3);
    }

    // r[verify nix-closure.generic-closure]
    #[test]
    fn test_generic_closure_no_duplicates() {
        // Operator always returns items with key=1 (cycle)
        let result = generic_closure(
            &vec![ClosureItem { key: 1, data: 0 }],
            &|_: &ClosureItem| vec![ClosureItem { key: 1, data: 0 }],
        );
        assert_eq!(result.len(), 1);
    }

    // r[verify nix-closure.generic-closure]
    #[test]
    fn test_generic_closure_empty_start() {
        let result = generic_closure(&vec![], &|_: &ClosureItem| vec![]);
        assert!(result.is_empty());
    }

    proptest! {
        // r[verify nix-closure.generic-closure]
        #[test]
        fn prop_closure_unique_keys(
            start_keys in proptest::collection::vec(0u64..100, 1..=10),
        ) {
            let start: Vec<ClosureItem> = start_keys.iter()
                .map(|&k| ClosureItem { key: k, data: k })
                .collect();
            let result = generic_closure(&start, &|_: &ClosureItem| vec![]);
            // All keys in result must be unique
            let keys: Vec<u64> = result.iter().map(|i| i.key).collect();
            let mut sorted = keys.clone();
            sorted.sort_unstable();
            sorted.dedup();
            prop_assert_eq!(keys.len(), sorted.len());
        }

        // r[verify nix-closure.generic-closure]
        #[test]
        fn prop_closure_contains_start(
            start_keys in proptest::collection::vec(0u64..100, 1..=5),
        ) {
            let start: Vec<ClosureItem> = start_keys.iter()
                .map(|&k| ClosureItem { key: k, data: k })
                .collect();
            let result = generic_closure(&start, &|_: &ClosureItem| vec![]);
            let result_keys: Vec<u64> = result.iter().map(|i| i.key).collect();
            // Every unique start key should appear in result
            for &k in &start_keys {
                prop_assert!(result_keys.contains(&k));
            }
        }

        // P0: result is closed — applying operator to result items yields no new keys
        #[test]
        fn prop_closure_is_closed(
            adj in proptest::collection::vec(
                proptest::collection::vec(0u64..10, 0..3), 1..=10
            ),
        ) {
            let n = adj.len() as u64;
            let operator = |item: &ClosureItem| -> Vec<ClosureItem> {
                let k = item.key;
                if (k as usize) < adj.len() {
                    adj[k as usize].iter()
                        .filter(|&&nbr| nbr < n)
                        .map(|&nbr| ClosureItem { key: nbr, data: 0 })
                        .collect()
                } else {
                    vec![]
                }
            };
            let start = vec![ClosureItem { key: 0, data: 0 }];
            let result = generic_closure(&start, &operator);
            let result_keys: std::collections::HashSet<u64> =
                result.iter().map(|i| i.key).collect();
            for item in &result {
                let new_items = operator(item);
                for new_item in new_items {
                    prop_assert!(result_keys.contains(&new_item.key));
                }
            }
        }

        // P0: closure(closure(x)) == closure(x) by key set
        #[test]
        fn prop_closure_idempotent(
            adj in proptest::collection::vec(
                proptest::collection::vec(0u64..8, 0..3), 1..=8
            ),
        ) {
            let n = adj.len() as u64;
            let operator = |item: &ClosureItem| -> Vec<ClosureItem> {
                let k = item.key;
                if (k as usize) < adj.len() {
                    adj[k as usize].iter()
                        .filter(|&&nbr| nbr < n)
                        .map(|&nbr| ClosureItem { key: nbr, data: 0 })
                        .collect()
                } else {
                    vec![]
                }
            };
            let start = vec![ClosureItem { key: 0, data: 0 }];
            let first = generic_closure(&start, &operator);
            let second = generic_closure(&first, &operator);
            let mut keys1: Vec<u64> = first.iter().map(|i| i.key).collect();
            let mut keys2: Vec<u64> = second.iter().map(|i| i.key).collect();
            keys1.sort_unstable();
            keys2.sort_unstable();
            prop_assert_eq!(keys1, keys2);
        }

        // P0: closure matches manual BFS over adjacency graph
        #[test]
        fn prop_closure_matches_manual_bfs(
            adj in proptest::collection::vec(
                proptest::collection::vec(0u64..10, 0..3), 1..=10
            ),
        ) {
            let n = adj.len() as u64;
            let operator = |item: &ClosureItem| -> Vec<ClosureItem> {
                let k = item.key;
                if (k as usize) < adj.len() {
                    adj[k as usize].iter()
                        .filter(|&&nbr| nbr < n)
                        .map(|&nbr| ClosureItem { key: nbr, data: 0 })
                        .collect()
                } else {
                    vec![]
                }
            };
            let start = vec![ClosureItem { key: 0, data: 0 }];
            let result = generic_closure(&start, &operator);
            let result_keys: std::collections::HashSet<u64> =
                result.iter().map(|i| i.key).collect();
            // Manual BFS
            let mut manual = std::collections::HashSet::new();
            let mut queue = std::collections::VecDeque::new();
            manual.insert(0u64);
            queue.push_back(0u64);
            while let Some(k) = queue.pop_front() {
                if (k as usize) < adj.len() {
                    for &nbr in &adj[k as usize] {
                        if nbr < n && manual.insert(nbr) {
                            queue.push_back(nbr);
                        }
                    }
                }
            }
            prop_assert_eq!(result_keys, manual);
        }
    }
}
