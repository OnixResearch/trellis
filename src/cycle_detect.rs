//! Verified cycle detection for directed graphs.
//!
//! Detects cycles via Kahn's algorithm: repeatedly remove zero-in-degree
//! nodes. If all nodes are removed, the graph is acyclic. If some remain,
//! those nodes form cycles. Specs define cycles via fuel-bounded path
//! existence.

#![allow(clippy::too_many_lines)]

use vstd::prelude::*;

verus! {

// ============================================================
// Path and cycle specs
// ============================================================

// r[depends cycle-detect.has-edge]
/// Spec: edge (u,v) exists in the edge list.
pub open spec fn has_edge_spec(edges: Seq<(int, int)>, u: int, v: int) -> bool {
    exists|i: int| #![trigger edges[i]] 0 <= i < edges.len() && edges[i] == (u, v)
}

// r[depends cycle-detect.has-path]
/// Spec: path exists from `src` to `dst` using at most `fuel` edges.
pub open spec fn has_path_fuel(edges: Seq<(int, int)>, src: int, dst: int, fuel: nat) -> bool
    decreases fuel,
{
    if src == dst {
        true
    } else if fuel == 0 {
        false
    } else {
        exists|mid: int| has_edge_spec(edges, src, mid)
            && has_path_fuel(edges, mid, dst, (fuel - 1) as nat)
    }
}

/// Spec: path exists with sufficient fuel.
pub open spec fn has_path_spec(edges: Seq<(int, int)>, n: int, src: int, dst: int) -> bool {
    has_path_fuel(edges, src, dst, n as nat)
}

// r[depends cycle-detect.has-cycle]
/// Spec: the graph has a cycle (some node can reach itself via >= 1 edge).
pub open spec fn has_cycle_spec(edges: Seq<(int, int)>, n: int) -> bool {
    exists|v: int| #![trigger has_nontrivial_cycle_at(edges, n, v)]
        0 <= v < n && has_nontrivial_cycle_at(edges, n, v)
}

/// Spec: node `v` is on a cycle (reachable from itself via >= 1 edge).
pub open spec fn has_nontrivial_cycle_at(edges: Seq<(int, int)>, n: int, v: int) -> bool {
    exists|mid: int| has_edge_spec(edges, v, mid)
        && has_path_fuel(edges, mid, v, n as nat)
}

// ============================================================
// Cycle detection (Kahn's method)
// ============================================================

// r[impl cycle-detect.has-cycle]
// @verified-logic:no-ensures Kahn-cycle correctness is still runtime-tested but not linked to has_cycle_spec
/// Detect whether a directed graph has a cycle.
///
/// Uses Kahn's algorithm: count in-degrees, iteratively remove
/// zero-in-degree nodes. If all n nodes are removed, no cycle.
pub fn has_cycle(edges: &[(usize, usize)], n: usize) -> (result: bool)
    requires
        n as int + edges@.len() as int <= usize::MAX as int,
        forall|i: int| #![trigger edges@[i]] 0 <= i < edges@.len() ==> {
            let (u, v) = edges@[i];
            (u as int) < n as int && (v as int) < n as int
        },
{
    // Compute in-degrees
    let mut in_deg: Vec<u64> = Vec::new();
    let mut k: usize = 0;
    while k < n
        invariant 0 <= k <= n, in_deg@.len() == k as int,
            forall|j: int| 0 <= j < k as int ==> in_deg@[j] == 0u64,
        decreases n - k,
    {
        in_deg.push(0);
        k += 1;
    }
    let mut ei: usize = 0;
    while ei < edges.len()
        invariant
            0 <= ei <= edges@.len(),
            in_deg@.len() == n as int,
            forall|i: int| #![trigger edges@[i]] 0 <= i < edges@.len() ==> {
                let (u, v) = edges@[i];
                (u as int) < n as int && (v as int) < n as int
            },
            forall|v: int| #![trigger in_deg@[v]] 0 <= v < n as int ==>
                in_deg@[v] as int <= edges@.len() as int,
        decreases edges.len() - ei,
    {
        let (_, v) = edges[ei];
        proof {
            let e = edges@[ei as int];
            assert((e.1 as int) < n as int);
        }
        if in_deg[v] < edges.len() as u64 {
            in_deg.set(v, in_deg[v] + 1);
        }
        ei += 1;
    }

    // Initialize queue with zero-in-degree nodes
    let mut queue: Vec<usize> = Vec::new();
    let mut qi_init: usize = 0;
    while qi_init < n
        invariant
            0 <= qi_init <= n,
            in_deg@.len() == n as int,
            queue@.len() <= qi_init,
            forall|s: int| #![trigger queue@[s]]
                0 <= s < queue@.len() ==> (queue@[s] as int) < n as int,
        decreases n - qi_init,
    {
        if in_deg[qi_init] == 0 {
            queue.push(qi_init);
        }
        qi_init += 1;
    }

    // Process queue: remove zero-in-degree nodes
    let mut processed: usize = 0;
    let mut qi: usize = 0;
    while qi < queue.len()
        invariant
            0 <= qi <= queue@.len(),
            in_deg@.len() == n as int,
            processed == qi,
            queue@.len() <= n,
            forall|i: int| #![trigger edges@[i]] 0 <= i < edges@.len() ==> {
                let (u, v) = edges@[i];
                (u as int) < n as int && (v as int) < n as int
            },
            forall|s: int| #![trigger queue@[s]]
                0 <= s < queue@.len() ==> (queue@[s] as int) < n as int,
        decreases n - qi,
    {
        let u = queue[qi];
        processed += 1;
        qi += 1;

        let mut ej: usize = 0;
        while ej < edges.len()
            invariant
                0 <= ej <= edges@.len(),
                in_deg@.len() == n as int,
                queue@.len() <= n,
                qi as int <= queue@.len() as int,
                forall|i: int| #![trigger edges@[i]] 0 <= i < edges@.len() ==> {
                    let (u, v) = edges@[i];
                    (u as int) < n as int && (v as int) < n as int
                },
                forall|s: int| #![trigger queue@[s]]
                    0 <= s < queue@.len() ==> (queue@[s] as int) < n as int,
            decreases edges.len() - ej,
        {
            let (src, dst) = edges[ej];
            proof {
                let e = edges@[ej as int];
                assert((e.0 as int) < n as int);
                assert((e.1 as int) < n as int);
            }
            if src == u {
                if in_deg[dst] > 0 {
                    in_deg.set(dst, in_deg[dst] - 1);
                    if in_deg[dst] == 0 {
                        if queue.len() < n {
                            queue.push(dst);
                        }
                    }
                }
            }
            ej += 1;
        }
    }

    processed < n
}

// ============================================================
// Properties
// ============================================================

// r[verify cycle-detect.has-path]
/// Every node has a trivial path to itself.
proof fn path_reflexive(edges: Seq<(int, int)>, src: int)
    ensures has_path_fuel(edges, src, src, 0)
{}

// r[verify cycle-detect.has-path]
/// A single edge gives a path of length 1.
proof fn edge_gives_path(edges: Seq<(int, int)>, u: int, v: int)
    requires has_edge_spec(edges, u, v)
    ensures has_path_fuel(edges, u, v, 1)
{
    assert(has_path_fuel(edges, v, v, 0));
}

// r[verify cycle-detect.has-path]
/// More fuel doesn't lose paths.
proof fn more_fuel_keeps_path(edges: Seq<(int, int)>, src: int, dst: int, fuel: nat)
    requires has_path_fuel(edges, src, dst, fuel)
    ensures has_path_fuel(edges, src, dst, fuel + 1)
    decreases fuel,
{
    if src == dst {
    } else {
        let mid = choose|mid: int| has_edge_spec(edges, src, mid)
            && has_path_fuel(edges, mid, dst, (fuel - 1) as nat);
        more_fuel_keeps_path(edges, mid, dst, (fuel - 1) as nat);
    }
}

// r[verify cycle-detect.has-cycle]
/// Self-loop is a cycle.
proof fn self_loop_is_cycle(edges: Seq<(int, int)>, n: int, v: int)
    requires 0 <= v < n, has_edge_spec(edges, v, v)
    ensures has_cycle_spec(edges, n)
{
    assert(has_path_fuel(edges, v, v, 0));
    assert(has_nontrivial_cycle_at(edges, n, v));
}

// r[verify cycle-detect.has-path]
/// Path concatenation: if src→mid and mid→dst, then src→dst.
proof fn path_concat(edges: Seq<(int, int)>, src: int, mid: int, dst: int, f1: nat, f2: nat)
    requires
        has_path_fuel(edges, src, mid, f1),
        has_path_fuel(edges, mid, dst, f2),
    ensures has_path_fuel(edges, src, dst, (f1 + f2) as nat)
    decreases f1,
{
    if src == mid {
        fuel_monotone(edges, src, dst, f2, (f1 + f2) as nat);
    } else {
        let next = choose|next: int| has_edge_spec(edges, src, next)
            && has_path_fuel(edges, next, mid, (f1 - 1) as nat);
        path_concat(edges, next, mid, dst, (f1 - 1) as nat, f2);
    }
}

proof fn fuel_monotone(edges: Seq<(int, int)>, src: int, dst: int, f1: nat, f2: nat)
    requires f2 >= f1, has_path_fuel(edges, src, dst, f1)
    ensures has_path_fuel(edges, src, dst, f2)
    decreases f1,
{
    if src == dst {
    } else {
        let next = choose|next: int| has_edge_spec(edges, src, next)
            && has_path_fuel(edges, next, dst, (f1 - 1) as nat);
        fuel_monotone(edges, next, dst, (f1 - 1) as nat, (f2 - 1) as nat);
    }
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle_linear() {
        let edges = [(0, 1), (1, 2)];
        assert!(!has_cycle(&edges, 3));
    }

    #[test]
    fn test_cycle_triangle() {
        let edges = [(0, 1), (1, 2), (2, 0)];
        assert!(has_cycle(&edges, 3));
    }

    #[test]
    fn test_self_loop() {
        let edges = [(0, 0)];
        assert!(has_cycle(&edges, 1));
    }

    #[test]
    fn test_no_edges() {
        let edges: [(usize, usize); 0] = [];
        assert!(!has_cycle(&edges, 5));
    }

    #[test]
    fn test_diamond_no_cycle() {
        let edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
        assert!(!has_cycle(&edges, 4));
    }

    #[test]
    fn test_disconnected_with_cycle() {
        let edges = [(2, 3), (3, 2)];
        assert!(has_cycle(&edges, 4));
    }

    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        fn random_dag(max_nodes: usize) -> impl Strategy<Value = (Vec<(usize, usize)>, usize)> {
            (2..=max_nodes).prop_flat_map(|n| {
                proptest::collection::vec((0..n, 0..n), 0..=(n * 2).min(30)).prop_map(move |raw| {
                    let edges: Vec<(usize, usize)> = raw
                        .into_iter()
                        .filter(|&(u, v)| u < v)
                        .collect();
                    (edges, n)
                })
            })
        }

        fn random_graph_with_cycle(max_nodes: usize) -> impl Strategy<Value = (Vec<(usize, usize)>, usize)> {
            (3..=max_nodes).prop_flat_map(|n| {
                proptest::collection::vec((0..n, 0..n), 0..=(n * 2).min(20)).prop_map(move |mut edges| {
                    for i in 0..n {
                        edges.push((i, (i + 1) % n));
                    }
                    (edges, n)
                })
            })
        }

        proptest! {
            #[test]
            fn prop_dag_no_cycle((edges, n) in random_dag(12)) {
                prop_assert!(!has_cycle(&edges, n));
            }

            #[test]
            fn prop_forced_cycle_detected((edges, n) in random_graph_with_cycle(10)) {
                prop_assert!(has_cycle(&edges, n));
            }

            #[test]
            fn prop_no_edges_no_cycle(n in 1..20usize) {
                let edges: Vec<(usize, usize)> = vec![];
                prop_assert!(!has_cycle(&edges, n));
            }

            #[test]
            fn prop_self_loop_detected(v in 0..15usize) {
                let edges = vec![(v, v)];
                prop_assert!(has_cycle(&edges, v + 1));
            }
        }
    }
}
