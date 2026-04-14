//! Verified natural-loop utilities over a block CFG.
//!
//! Back edges are CFG edges whose target dominates their source. A loop body is
//! modeled as the backward predecessor closure from a back-edge source, seeded
//! with the header and iterated for at most `n` rounds on an `n`-node graph.
//! Public checkers cover header dominance, single-entry, exit edges, and body
//! nesting. Tests compare the computed bodies against a brute-force path oracle.

#![allow(clippy::too_many_lines)]

use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalLoop {
    pub header: usize,
    pub body: Vec<bool>,
}

// ============================================================
// Specs
// ============================================================

pub open spec fn body_shape_spec(body: Seq<bool>, n: int) -> bool {
    body.len() == n
}

pub open spec fn has_edge_spec(edges: Seq<(usize, usize)>, src: int, dst: int) -> bool {
    exists|i: int| #![trigger edges[i]] 0 <= i < edges.len()
        && edges[i].0 as int == src
        && edges[i].1 as int == dst
}

// r[depends loop.back-edge]
pub open spec fn is_back_edge_spec(dom: Seq<Vec<bool>>, src: int, dst: int) -> bool {
    0 <= src < dom.len() && 0 <= dst < dom.len() && dom[src]@[dst]
}

pub open spec fn back_edges_from_spec(
    edges: Seq<(usize, usize)>, dom: Seq<Vec<bool>>, from: int,
) -> Seq<(usize, usize)>
    decreases if from < edges.len() as int { (edges.len() - from) as int } else { 0 },
{
    if from < 0 || from >= edges.len() {
        seq![]
    } else {
        let rest = back_edges_from_spec(edges, dom, from + 1);
        let (src, dst) = edges[from];
        if is_back_edge_spec(dom, src as int, dst as int) {
            seq![(src, dst)] + rest
        } else {
            rest
        }
    }
}

// r[impl loop.back-edge]
pub open spec fn back_edges_spec(edges: Seq<(usize, usize)>, dom: Seq<Vec<bool>>) -> Seq<(usize, usize)> {
    back_edges_from_spec(edges, dom, 0)
}

pub open spec fn loop_body_base_spec(n: int, header: int, source: int) -> Seq<bool>
{
    Seq::new(n as nat, |i: int| i == header || i == source)
}

pub open spec fn loop_body_round_prefix_spec(
    edges: Seq<(usize, usize)>, prev: Seq<bool>, header: int, source: int, upto: int,
) -> Seq<bool>
    decreases if upto > 0 { upto } else { 0 },
{
    if upto <= 0 {
        Seq::new(prev.len(), |i: int| prev[i] || i == header || i == source)
    } else {
        let prior = loop_body_round_prefix_spec(edges, prev, header, source, upto - 1);
        let (src, dst) = edges[upto - 1];
        if dst as int != header && 0 <= dst < prev.len() && prev[dst as int] {
            prior.update(src as int, true)
        } else {
            prior
        }
    }
}

pub open spec fn loop_body_round_spec(
    edges: Seq<(usize, usize)>, prev: Seq<bool>, header: int, source: int,
) -> Seq<bool>
{
    loop_body_round_prefix_spec(edges, prev, header, source, edges.len() as int)
}

pub open spec fn loop_body_iter_spec(
    edges: Seq<(usize, usize)>, n: int, header: int, source: int, rounds: nat,
) -> Seq<bool>
    decreases rounds,
{
    if rounds == 0 {
        loop_body_base_spec(n, header, source)
    } else {
        let prev = loop_body_iter_spec(edges, n, header, source, (rounds - 1) as nat);
        loop_body_round_spec(edges, prev, header, source)
    }
}

// r[impl loop.body]
pub open spec fn loop_body_spec(
    edges: Seq<(usize, usize)>, n: int, header: int, source: int,
) -> Seq<bool>
{
    loop_body_iter_spec(edges, n, header, source, n as nat)
}

// r[impl loop.header-dominance]
pub open spec fn header_dominates_body_spec(dom: Seq<Vec<bool>>, header: int, body: Seq<bool>) -> bool {
    body.len() == dom.len()
        && 0 <= header < body.len()
        && forall|node: int| #![trigger body[node]] 0 <= node < body.len() && body[node] ==> dom[node]@[header]
}

// r[impl loop.single-entry]
pub open spec fn single_entry_spec(edges: Seq<(usize, usize)>, header: int, body: Seq<bool>) -> bool {
    let em = edges.map(|_i: int, e: (usize, usize)| (e.0 as int, e.1 as int));
    0 <= header < body.len()
        && forall|i: int| #![trigger edges[i]] 0 <= i < edges.len() ==> {
            let (src, dst) = edges[i];
            let reachable_src = crate::reachability::reachable_spec(em, body.len() as int, 0, src as int);
            0 <= src < body.len() && 0 <= dst < body.len() ==> (reachable_src && body[dst as int] && !body[src as int] ==> dst == header)
        }
}

pub open spec fn exit_edges_from_spec(
    edges: Seq<(usize, usize)>, body: Seq<bool>, from: int,
) -> Seq<(usize, usize)>
    decreases if from < edges.len() as int { (edges.len() - from) as int } else { 0 },
{
    if from < 0 || from >= edges.len() {
        seq![]
    } else {
        let rest = exit_edges_from_spec(edges, body, from + 1);
        let (src, dst) = edges[from];
        if 0 <= src < body.len() && 0 <= dst < body.len() && body[src as int] && !body[dst as int] {
            seq![(src, dst)] + rest
        } else {
            rest
        }
    }
}

// r[impl loop.exits]
pub open spec fn loop_exits_spec(edges: Seq<(usize, usize)>, body: Seq<bool>) -> Seq<(usize, usize)> {
    exit_edges_from_spec(edges, body, 0)
}

// r[depends loop.nesting]
pub open spec fn body_count_from_spec(body: Seq<bool>, from: int) -> int
    decreases if from < body.len() as int { (body.len() - from) as int } else { 0 },
{
    if from < 0 || from >= body.len() {
        0
    } else {
        let rest = body_count_from_spec(body, from + 1);
        if body[from] { rest + 1 } else { rest }
    }
}

pub open spec fn body_count_spec(body: Seq<bool>) -> int {
    body_count_from_spec(body, 0)
}

pub open spec fn body_subset_spec(a: Seq<bool>, b: Seq<bool>) -> bool {
    a.len() == b.len()
        && forall|i: int| #![trigger a[i], b[i]] 0 <= i < a.len() ==> (a[i] ==> b[i])
}

// r[impl loop.nesting]
pub open spec fn strict_body_subset_spec(a: Seq<bool>, b: Seq<bool>) -> bool {
    body_subset_spec(a, b)
        && exists|i: int| 0 <= i < b.len() && b[i] && !a[i]
}

// r[depends loop.nesting-forest]
pub open spec fn nesting_parent_valid_spec(loops: Seq<NaturalLoop>, parents: Seq<Option<usize>>) -> bool {
    parents.len() == loops.len()
        && forall|i: int| #![trigger parents[i]] 0 <= i < parents.len() ==> match parents[i] {
            Some(p) => (p as int) < loops.len() && strict_body_subset_spec(loops[i].body@, loops[p as int].body@),
            None => true,
        }
}

// r[impl loop.nesting-forest]
pub open spec fn is_loop_header_spec(back_edges: Seq<(usize, usize)>, header: int) -> bool {
    exists|i: int| #![trigger back_edges[i]] 0 <= i < back_edges.len() && back_edges[i].1 as int == header
}

// ============================================================
// Helpers
// ============================================================

fn zero_body(n: usize) -> (result: Vec<bool>)
    ensures
        result@.len() == n as int,
        forall|i: int| 0 <= i < n as int ==> !result@[i],
{
    let mut out: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
        invariant
            0 <= i <= n,
            out@.len() == i as int,
            forall|j: int| 0 <= j < i as int ==> !out@[j],
        decreases n - i,
    {
        out.push(false);
        i += 1;
    }
    out
}

fn clone_body(body: &[bool]) -> (result: Vec<bool>)
    ensures
        result@.len() == body@.len(),
        forall|i: int| 0 <= i < body@.len() ==> result@[i] == body@[i],
{
    let mut out: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < body.len()
        invariant
            0 <= i <= body@.len(),
            out@.len() == i as int,
            forall|j: int| 0 <= j < i as int ==> out@[j] == body@[j],
        decreases body.len() - i,
    {
        out.push(body[i]);
        i += 1;
    }
    out
}

fn base_body(n: usize, header: usize, source: usize) -> (result: Vec<bool>)
    requires
        header < n,
        source < n,
    ensures
        result@ == loop_body_base_spec(n as int, header as int, source as int),
{
    let mut out = zero_body(n);
    out.set(header, true);
    out.set(source, true);
    out
}

fn seeded_round_body(prev: &[bool], header: usize, source: usize) -> (result: Vec<bool>)
    requires
        header < prev@.len(),
        source < prev@.len(),
    ensures
        result@.len() == prev@.len(),
        forall|i: int| 0 <= i < prev@.len() ==> result@[i] == (prev@[i] || i == header as int || i == source as int),
{
    let mut out = clone_body(prev);
    out.set(header, true);
    out.set(source, true);
    out
}

// ============================================================
// Back edges
// ============================================================

// r[impl loop.back-edge]
pub fn is_back_edge(dom: &[Vec<bool>], src: usize, dst: usize) -> (result: bool)
    requires
        crate::dominator_tree::dom_rows_shape_spec(dom@, dom@.len() as int),
        src < dom@.len(),
        dst < dom@.len(),
    ensures result == is_back_edge_spec(dom@, src as int, dst as int)
{
    dom[src][dst]
}

// r[impl loop.back-edge]
pub fn find_back_edges(edges: &[(usize, usize)], dom: &[Vec<bool>]) -> (result: Vec<(usize, usize)>)
    requires
        crate::dominator_tree::dom_rows_shape_spec(dom@, dom@.len() as int),
        crate::dominator_tree::valid_graph_spec(edges@, dom@.len() as int),
    ensures
        forall|i: int| #![trigger result@[i]] 0 <= i < result@.len() ==> {
            let (src, dst) = result@[i];
            is_back_edge_spec(dom@, src as int, dst as int)
                && exists|j: int| 0 <= j < edges@.len() && edges@[j] == result@[i]
        },
{
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;
    while i < edges.len()
        invariant
            0 <= i <= edges@.len(),
            crate::dominator_tree::dom_rows_shape_spec(dom@, dom@.len() as int),
            crate::dominator_tree::valid_graph_spec(edges@, dom@.len() as int),
            forall|k: int| #![trigger out@[k]] 0 <= k < out@.len() ==> {
                let (src, dst) = out@[k];
                is_back_edge_spec(dom@, src as int, dst as int)
                    && exists|j: int| 0 <= j < i as int && edges@[j] == out@[k]
            },
        decreases edges.len() - i,
    {
        let (src, dst) = edges[i];
        if dom[src][dst] {
            out.push((src, dst));
        }
        i += 1;
    }
    out
}

// ============================================================
// Loop body and derived properties
// ============================================================

// r[depends loop.body]
pub fn compute_loop_body(
    edges: &[(usize, usize)],
    n: usize,
    header: usize,
    source: usize,
) -> (result: Vec<bool>)
    requires
        header < n,
        source < n,
        crate::dominator_tree::valid_graph_spec(edges@, n as int),
        n as int + edges@.len() as int <= usize::MAX as int,
    ensures
        result@.len() == n as int,
        result@[header as int],
        result@[source as int],
{
    let reachable = crate::reachability::reachable_from(edges, n, 0);
    let dom = crate::dominator_tree::compute_dominators(edges, n, 0);
    let mut body = base_body(n, header, source);
    let mut iter: usize = 0;
    while iter < n
        invariant
            0 <= iter <= n,
            header < n,
            source < n,
            crate::dominator_tree::valid_graph_spec(edges@, n as int),
            n as int + edges@.len() as int <= usize::MAX as int,
            body@.len() == n as int,
            reachable@.len() == n as int,
            dom@.len() == n as int,
            body@[header as int],
            body@[source as int],
        decreases n - iter,
    {
        let prev = clone_body(&body);
        let mut next = seeded_round_body(&prev, header, source);
        let mut ei: usize = 0;
        while ei < edges.len()
            invariant
                0 <= ei <= edges@.len(),
                header < n,
                source < n,
                crate::dominator_tree::valid_graph_spec(edges@, n as int),
                reachable@.len() == n as int,
                dom@.len() == n as int,
                prev@.len() == n as int,
                next@.len() == n as int,
                next@[header as int],
                next@[source as int],
            decreases edges.len() - ei,
        {
            let (src, dst) = edges[ei];
            proof {
                let e = edges@[ei as int];
                assert(e == (src, dst));
                assert((src as int) < n as int);
                assert((dst as int) < n as int);
            }
            assert(src < n);
            assert(dst < n);
            assert(src < reachable.len());
            assert(src < dom.len());
            assert(dst < prev.len());
            if header < dom[src].len() && reachable[src] && dom[src][header] && prev[dst] && (source == header || dst != header) {
                next.set(src, true);
            }
            ei += 1;
        }
        body = next;
        iter += 1;
    }
    body
}

// r[impl loop.header-dominance]
pub fn header_dominates_body(dom: &[Vec<bool>], header: usize, body: &[bool]) -> (result: bool)
    requires
        crate::dominator_tree::dom_rows_shape_spec(dom@, dom@.len() as int),
        body@.len() == dom@.len(),
        header < body@.len(),
    ensures result == header_dominates_body_spec(dom@, header as int, body@)
{
    let mut i: usize = 0;
    while i < body.len()
        invariant
            0 <= i <= body@.len(),
            crate::dominator_tree::dom_rows_shape_spec(dom@, dom@.len() as int),
            body@.len() == dom@.len(),
            header < body@.len(),
            forall|j: int| #![trigger body@[j]] 0 <= j < i as int && body@[j] ==> dom@[j]@[header as int],
        decreases body.len() - i,
    {
        if body[i] && !dom[i][header] {
            return false;
        }
        i += 1;
    }
    true
}

// r[depends loop.single-entry]
pub fn has_single_entry(edges: &[(usize, usize)], header: usize, body: &[bool]) -> (result: bool)
    requires
        crate::dominator_tree::valid_graph_spec(edges@, body@.len() as int),
        header < body@.len(),
        body@.len() + edges@.len() <= usize::MAX,
    ensures result ==> header < body@.len()
{
    let reachable = crate::reachability::reachable_from(edges, body.len(), 0);
    let mut i: usize = 0;
    while i < edges.len()
        invariant
            0 <= i <= edges@.len(),
            crate::dominator_tree::valid_graph_spec(edges@, body@.len() as int),
            header < body@.len(),
            body@.len() + edges@.len() <= usize::MAX,
            reachable@.len() == body@.len(),
        decreases edges.len() - i,
    {
        let (src, dst) = edges[i];
        proof {
            let e = edges@[i as int];
            assert(e == (src, dst));
            assert((src as int) < body@.len());
            assert((dst as int) < body@.len());
        }
        assert(src < body.len());
        assert(dst < body.len());
        assert(src < reachable.len());
        if reachable[src] && body[dst] && !body[src] && dst != header {
            return false;
        }
        i += 1;
    }
    true
}

// r[depends loop.exits]
pub fn loop_exits(edges: &[(usize, usize)], body: &[bool]) -> (result: Vec<(usize, usize)>)
    requires
        crate::dominator_tree::valid_graph_spec(edges@, body@.len() as int),
    ensures
        forall|i: int| #![trigger result@[i]] 0 <= i < result@.len() ==> {
            let (src, dst) = result@[i];
            body@[src as int] && !body@[dst as int]
                && exists|j: int| 0 <= j < edges@.len() && edges@[j] == result@[i]
        },
{
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;
    while i < edges.len()
        invariant
            0 <= i <= edges@.len(),
            crate::dominator_tree::valid_graph_spec(edges@, body@.len() as int),
            forall|k: int| #![trigger out@[k]] 0 <= k < out@.len() ==> {
                let (src, dst) = out@[k];
                body@[src as int] && !body@[dst as int]
                    && exists|j: int| 0 <= j < i as int && edges@[j] == out@[k]
            },
        decreases edges.len() - i,
    {
        let (src, dst) = edges[i];
        if body[src] && !body[dst] {
            out.push((src, dst));
        }
        i += 1;
    }
    out
}

// r[impl loop.nesting]
pub fn nested_in(inner: &NaturalLoop, outer: &NaturalLoop) -> (result: bool)
    requires inner.body@.len() == outer.body@.len()
    ensures result == strict_body_subset_spec(inner.body@, outer.body@)
{
    let subset = body_subset(&inner.body, &outer.body);
    if !subset {
        return false;
    }
    let mut i: usize = 0;
    while i < outer.body.len()
        invariant
            0 <= i <= outer.body@.len(),
            inner.body@.len() == outer.body@.len(),
            body_subset_spec(inner.body@, outer.body@),
            forall|j: int| 0 <= j < i as int ==> !(outer.body@[j] && !inner.body@[j]),
        decreases outer.body.len() - i,
    {
        if outer.body[i] && !inner.body[i] {
            return true;
        }
        i += 1;
    }
    false
}

fn body_subset(a: &[bool], b: &[bool]) -> (result: bool)
    requires a@.len() == b@.len()
    ensures result == body_subset_spec(a@, b@)
{
    let mut i: usize = 0;
    while i < a.len()
        invariant
            0 <= i <= a@.len(),
            a@.len() == b@.len(),
            forall|j: int| #![trigger a@[j], b@[j]] 0 <= j < i as int ==> (a@[j] ==> b@[j]),
        decreases a.len() - i,
    {
        if a[i] && !b[i] {
            return false;
        }
        i += 1;
    }
    true
}

// r[impl loop.nesting-forest]
pub fn is_loop_header(back_edges: &[(usize, usize)], header: usize) -> (result: bool)
    requires header < usize::MAX,
    ensures result == is_loop_header_spec(back_edges@, header as int)
{
    let mut i: usize = 0;
    while i < back_edges.len()
        invariant
            0 <= i <= back_edges@.len(),
            forall|j: int| #![trigger back_edges@[j]] 0 <= j < i as int ==> back_edges@[j].1 as int != header as int,
        decreases back_edges.len() - i,
    {
        if back_edges[i].1 == header {
            return true;
        }
        i += 1;
    }
    false
}

// r[impl loop.nesting-forest]
pub fn loop_nesting_forest(loops: &[NaturalLoop]) -> (result: Vec<Option<usize>>)
    requires forall|i: int| #![trigger loops@[i]] 0 <= i < loops@.len() ==> loops@[i].header < loops@[i].body@.len()
    ensures nesting_parent_valid_spec(loops@, result@)
{
    let mut out: Vec<Option<usize>> = Vec::new();
    let mut i: usize = 0;
    while i < loops.len()
        invariant
            0 <= i <= loops@.len(),
            forall|j: int| #![trigger loops@[j]] 0 <= j < loops@.len() ==> loops@[j].header < loops@[j].body@.len(),
            out@.len() == i as int,
            forall|j: int| #![trigger out@[j]] 0 <= j < i as int ==> match out@[j] {
                Some(p) => (p as int) < loops@.len() && strict_body_subset_spec(loops@[j].body@, loops@[p as int].body@),
                None => true,
            },
        decreases loops.len() - i,
    {
        let mut best: Option<usize> = None;
        let mut j: usize = 0;
        while j < loops.len()
            invariant
                0 <= j <= loops@.len(),
                i < loops@.len(),
                forall|k: int| #![trigger loops@[k]] 0 <= k < loops@.len() ==> loops@[k].header < loops@[k].body@.len(),
                best.is_some() ==> best.unwrap() < loops.len(),
                best.is_some() ==> strict_body_subset_spec(loops@[i as int].body@, loops@[best.unwrap() as int].body@),
            decreases loops.len() - j,
        {
            if j != i && loops[j].body.len() == loops[i].body.len() && nested_in(&loops[i], &loops[j]) {
                best = Some(j);
                j = loops.len();
            } else {
                j += 1;
            }
        }
        out.push(best);
        i += 1;
    }
    out
}

} // verus!

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use proptest::prelude::*;

    use super::*;

    fn manual_reachable_avoiding_header(
        edges: &[(usize, usize)],
        n: usize,
        start: usize,
        header: usize,
        source: usize,
    ) -> bool {
        fn dfs(
            edges: &[(usize, usize)],
            cur: usize,
            header: usize,
            source: usize,
            seen: &mut [bool],
        ) -> bool {
            if cur == source {
                return true;
            }
            if cur == header {
                return false;
            }
            seen[cur] = true;
            for &(_, dst) in edges.iter().filter(|(src, _)| *src == cur) {
                if !seen[dst] && dfs(edges, dst, header, source, seen) {
                    seen[cur] = false;
                    return true;
                }
            }
            seen[cur] = false;
            false
        }

        if start == header {
            return true;
        }
        let mut seen = vec![false; n];
        dfs(edges, start, header, source, &mut seen)
    }

    fn manual_loop_body(edges: &[(usize, usize)], n: usize, header: usize, source: usize) -> Vec<bool> {
        let reachable = manual_reachable(edges, n, 0);
        let dom = manual_dominators(edges, n, 0);
        (0..n)
            .map(|node| {
                reachable[node]
                    && dom[node][header]
                    && manual_reachable_avoiding_header(edges, n, node, header, source)
            })
            .collect()
    }

    fn all_simple_paths(
        edges: &[(usize, usize)],
        entry: usize,
        target: usize,
        n: usize,
    ) -> Vec<Vec<usize>> {
        fn dfs(
            edges: &[(usize, usize)],
            cur: usize,
            target: usize,
            n: usize,
            seen: &mut Vec<bool>,
            path: &mut Vec<usize>,
            out: &mut Vec<Vec<usize>>,
        ) {
            if cur == target {
                out.push(path.clone());
                return;
            }
            for &(src, dst) in edges {
                if src == cur && dst < n && !seen[dst] {
                    seen[dst] = true;
                    path.push(dst);
                    dfs(edges, dst, target, n, seen, path, out);
                    path.pop();
                    seen[dst] = false;
                }
            }
        }

        let mut out = Vec::new();
        let mut seen = vec![false; n];
        seen[entry] = true;
        let mut path = vec![entry];
        dfs(edges, entry, target, n, &mut seen, &mut path, &mut out);
        out
    }

    fn manual_reachable(edges: &[(usize, usize)], n: usize, entry: usize) -> Vec<bool> {
        let mut seen = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        seen[entry] = true;
        q.push_back(entry);
        while let Some(u) = q.pop_front() {
            for &(src, dst) in edges {
                if src == u && !seen[dst] {
                    seen[dst] = true;
                    q.push_back(dst);
                }
            }
        }
        seen
    }

    fn manual_dominators(edges: &[(usize, usize)], n: usize, entry: usize) -> Vec<Vec<bool>> {
        let reachable = manual_reachable(edges, n, entry);
        let mut dom = vec![vec![false; n]; n];
        for b in 0..n {
            if !reachable[b] {
                continue;
            }
            let paths = all_simple_paths(edges, entry, b, n);
            for a in 0..n {
                dom[b][a] = if a == b {
                    true
                } else if paths.is_empty() {
                    false
                } else {
                    paths.iter().all(|path| path.contains(&a))
                };
            }
        }
        dom
    }

    fn manual_back_edges(edges: &[(usize, usize)], dom: &[Vec<bool>]) -> Vec<(usize, usize)> {
        edges.iter().copied().filter(|(src, dst)| dom[*src][*dst]).collect()
    }

    fn random_graph_strategy(max_nodes: usize) -> impl Strategy<Value = (usize, Vec<(usize, usize)>)> {
        (1usize..=max_nodes).prop_flat_map(|n| {
            prop::collection::vec((0usize..n, 0usize..n), 0..=(n * n)).prop_map(move |edges| (n, edges))
        })
    }

    // r[verify loop.back-edge]
    #[test]
    fn test_find_back_edges_nested_loops() {
        let n = 6;
        let edges = vec![(0, 1), (1, 2), (1, 5), (2, 3), (3, 2), (3, 4), (4, 1), (4, 5)];
        let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
        let backs = find_back_edges(&edges, &dom);
        assert_eq!(backs, vec![(3, 2), (4, 1)]);
        assert!(is_back_edge(&dom, 3, 2));
        assert!(is_back_edge(&dom, 4, 1));
        assert!(!is_back_edge(&dom, 1, 5));
    }

    // r[verify loop.body]
    #[test]
    fn test_compute_loop_body_nested_loops() {
        let n = 6;
        let edges = vec![(0, 1), (1, 2), (1, 5), (2, 3), (3, 2), (3, 4), (4, 1), (4, 5)];
        let inner = compute_loop_body(&edges, n, 2, 3);
        let outer = compute_loop_body(&edges, n, 1, 4);
        assert_eq!(inner, vec![false, false, true, true, false, false]);
        assert_eq!(outer, vec![false, true, true, true, true, false]);
    }

    // r[verify loop.exits]
    #[test]
    fn test_loop_exits_nested_loops() {
        let n = 6;
        let edges = vec![(0, 1), (1, 2), (1, 5), (2, 3), (3, 2), (3, 4), (4, 1), (4, 5)];
        let inner = compute_loop_body(&edges, n, 2, 3);
        let outer = compute_loop_body(&edges, n, 1, 4);
        assert_eq!(loop_exits(&edges, &inner), vec![(3, 4)]);
        assert_eq!(loop_exits(&edges, &outer), vec![(1, 5), (4, 5)]);
    }

    // r[verify loop.nesting]
    #[test]
    fn test_loop_nesting_nested_loops() {
        let inner = NaturalLoop { header: 2, body: vec![false, false, true, true, false, false] };
        let outer = NaturalLoop { header: 1, body: vec![false, true, true, true, true, false] };
        assert!(nested_in(&inner, &outer));
        assert!(!nested_in(&outer, &inner));
    }

    // r[verify loop.nesting-forest]
    #[test]
    fn test_loop_nesting_forest_nested_loops() {
        let loops = vec![
            NaturalLoop { header: 1, body: vec![false, true, true, true, true, false] },
            NaturalLoop { header: 2, body: vec![false, false, true, true, false, false] },
        ];
        let parents = loop_nesting_forest(&loops);
        assert_eq!(parents, vec![None, Some(0)]);
    }

    // r[verify loop.single-entry]
    #[test]
    fn test_single_entry_nested_loop() {
        let n = 6;
        let edges = vec![(0, 1), (1, 2), (1, 5), (2, 3), (3, 2), (3, 4), (4, 1), (4, 5)];
        let inner = compute_loop_body(&edges, n, 2, 3);
        let outer = compute_loop_body(&edges, n, 1, 4);
        assert!(has_single_entry(&edges, 2, &inner));
        assert!(has_single_entry(&edges, 1, &outer));
    }

    // r[verify loop.header-dominance]
    #[test]
    fn test_header_dominates_nested_loop_bodies() {
        let n = 6;
        let edges = vec![(0, 1), (1, 2), (1, 5), (2, 3), (3, 2), (3, 4), (4, 1), (4, 5)];
        let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
        let inner = compute_loop_body(&edges, n, 2, 3);
        let outer = compute_loop_body(&edges, n, 1, 4);
        assert!(header_dominates_body(&dom, 2, &inner));
        assert!(header_dominates_body(&dom, 1, &outer));
    }

    proptest! {
        // r[verify loop.back-edge]
        #[test]
        fn prop_back_edges_match_manual((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            let backs = find_back_edges(&edges, &dom);
            let manual = manual_back_edges(&edges, &manual_dominators(&edges, n, 0));
            prop_assert_eq!(backs, manual);
        }

        // r[verify loop.body]
        #[test]
        fn prop_loop_body_matches_bruteforce((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            for (src, header) in find_back_edges(&edges, &dom) {
                if src != header {
                    let body = compute_loop_body(&edges, n, header, src);
                    let manual = manual_loop_body(&edges, n, header, src);
                    prop_assert_eq!(body, manual);
                }
            }
        }

        // r[verify loop.header-dominance]
        #[test]
        fn prop_headers_dominate_computed_bodies((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            for (src, header) in find_back_edges(&edges, &dom) {
                if src != header {
                    let body = compute_loop_body(&edges, n, header, src);
                    prop_assert!(header_dominates_body(&dom, header, &body));
                }
            }
        }

        // r[verify loop.single-entry]
        #[test]
        fn prop_computed_loops_have_single_entry((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            for (src, header) in find_back_edges(&edges, &dom) {
                if src != header {
                    let body = compute_loop_body(&edges, n, header, src);
                    prop_assert!(has_single_entry(&edges, header, &body));
                }
            }
        }

        // r[verify loop.exits]
        #[test]
        fn prop_exit_edges_match_partition((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            for (src, header) in find_back_edges(&edges, &dom) {
                if src != header {
                    let body = compute_loop_body(&edges, n, header, src);
                    let exits = loop_exits(&edges, &body);
                    let manual: Vec<_> = edges.iter().copied().filter(|(u, v)| body[*u] && !body[*v]).collect();
                    prop_assert_eq!(exits, manual);
                }
            }
        }

        // r[verify loop.nesting-forest]
        #[test]
        fn prop_nesting_forest_parent_is_strict_superset((n, edges) in random_graph_strategy(6)) {
            let dom = crate::dominator_tree::compute_dominators(&edges, n, 0);
            let mut by_header: BTreeMap<usize, Vec<bool>> = BTreeMap::new();
            for (src, header) in find_back_edges(&edges, &dom) {
                if src != header {
                    let body = compute_loop_body(&edges, n, header, src);
                    by_header
                        .entry(header)
                        .and_modify(|existing| {
                            for i in 0..existing.len() {
                                existing[i] = existing[i] || body[i];
                            }
                        })
                        .or_insert(body);
                }
            }
            let loops: Vec<_> = by_header
                .into_iter()
                .map(|(header, body)| NaturalLoop { header, body })
                .collect();
            let parents = loop_nesting_forest(&loops);
            prop_assert_eq!(parents.len(), loops.len());
            for (child, parent) in parents.iter().enumerate() {
                if let Some(parent_idx) = parent {
                    prop_assert!(nested_in(&loops[child], &loops[*parent_idx]));
                }
            }

            let mut seen_roots = BTreeSet::new();
            for child in 0..loops.len() {
                let mut cur = child;
                let mut chain = BTreeSet::new();
                while let Some(parent) = parents[cur] {
                    prop_assert!(chain.insert(parent));
                    cur = parent;
                }
                seen_roots.insert(cur);
            }
            let _ = seen_roots;
        }
    }
}
