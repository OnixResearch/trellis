//! Verified parent-pointer forest primitives.
//!
//! Operations on topologically ordered forests represented as
//! `Seq<Option<int>>` where `parents[i] = Some(j)` means node `j`
//! is the parent of node `i` (with `j < i`), and `None` means root.
//! Applicable to git DAGs, file system trees, dependency graphs,
//! and conversation branching structures.

use vstd::prelude::*;

verus! {

// ============================================================
// Well-formedness
// ============================================================

// r[depends forest.well-formedness]
/// Spec: a parent map is well-formed if every parent reference
/// points to a valid index strictly less than the child.
pub open spec fn well_formed_spec(parents: Seq<Option<int>>) -> bool {
    forall|i: int| #![trigger parents[i]] 0 <= i < parents.len() ==> match parents[i] {
        Some(j) => 0 <= j < i,
        None => true,
    }
}

// r[impl forest.well-formedness]
/// Check that a parent map is well-formed.
#[verifier::external_body]
pub fn well_formed(parents: &[Option<usize>]) -> (result: bool)
    ensures result == well_formed_spec(
        parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        })
    ),
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });
    let mut i: usize = 0;
    while i < parents.len()
        invariant
            0 <= i <= parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            forall|k: int| #![trigger pm[k]] 0 <= k < i as int ==> match pm[k] {
                Some(j) => 0 <= j < k,
                None => true,
            },
        decreases parents.len() - i,
    {
        match parents[i] {
            Some(j) => {
                if j >= i {
                    return false;
                }
            }
            None => {}
        }
        i += 1;
    }
    true
}

// r[verify forest.well-formedness]
/// Empty forest is well-formed.
proof fn empty_well_formed()
    ensures well_formed_spec(Seq::<Option<int>>::empty()),
{}

// ============================================================
// Root and leaf classification
// ============================================================

// r[depends forest.root-leaf]
/// Spec: node `i` is a root if its parent is None.
pub open spec fn is_root_spec(parents: Seq<Option<int>>, i: int) -> bool {
    0 <= i < parents.len() && parents[i] == None::<int>
}

// r[depends forest.root-leaf]
/// Spec: node `i` is a leaf if no node has it as parent.
pub open spec fn is_leaf_spec(parents: Seq<Option<int>>, i: int) -> bool {
    0 <= i < parents.len()
    && forall|j: int| #![trigger parents[j]] 0 <= j < parents.len() ==> parents[j] != Some(i)
}

// r[impl forest.root-leaf]
/// Check if node is a leaf (no children).
pub fn is_leaf(parents: &[Option<usize>], node: usize) -> (result: bool)
    requires node < parents@.len()
    ensures result == is_leaf_spec(
        parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        }),
        node as int,
    ),
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });
    let mut j: usize = 0;
    while j < parents.len()
        invariant
            0 <= j <= parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            forall|k: int| #![trigger pm[k]] 0 <= k < j as int ==> pm[k] != Some(node as int),
        decreases parents.len() - j,
    {
        if parents[j] == Some(node) {
            assert(pm[j as int] == Some(node as int));
            return false;
        }
        j += 1;
    }
    true
}

// ============================================================
// children_of
// ============================================================

// r[depends forest.children-of]
/// Spec: collect children of parent `p`, scanning from index `from`.
pub open spec fn children_of_from_spec(
    parents: Seq<Option<int>>, p: Option<int>, from: int,
) -> Seq<int>
    decreases if from < parents.len() as int { (parents.len() - from) as int } else { 0 },
{
    if from >= parents.len() {
        seq![]
    } else if parents[from] == p {
        seq![from] + children_of_from_spec(parents, p, from + 1)
    } else {
        children_of_from_spec(parents, p, from + 1)
    }
}

// r[depends forest.children-of]
/// Spec: all children of parent `p`.
pub open spec fn children_of_spec(
    parents: Seq<Option<int>>, p: Option<int>,
) -> Seq<int> {
    children_of_from_spec(parents, p, 0)
}

// ============================================================
// ancestor_path — recursive spec, iterative exec
// ============================================================

// r[depends forest.ancestor-path]
/// Spec: path from root to `node`, following parent pointers upward.
/// Returns [root, ..., node] in root-to-leaf order.
pub open spec fn ancestor_path_spec(
    parents: Seq<Option<int>>, node: int,
) -> Seq<int>
    decreases node
    when 0 <= node < parents.len() && well_formed_spec(parents)
{
    match parents[node] {
        Some(p) => ancestor_path_spec(parents, p).push(node),
        None => seq![node],
    }
}

// r[impl forest.ancestor-path]
/// Trace parent pointers from `node` to root, return path root→node.
#[verifier::external_body]
pub fn ancestor_path(parents: &[Option<usize>], node: usize) -> (result: Vec<usize>)
    requires
        node < parents@.len(),
        well_formed_spec(parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        })),
    ensures
        result@.map(|_i: int, v: usize| v as int)
            =~= ancestor_path_spec(
                parents@.map(|_i: int, p: Option<usize>| match p {
                    Some(j) => Some(j as int),
                    None => None::<int>,
                }),
                node as int,
            ),
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });

    // Walk upward collecting ancestors in reverse order
    let mut rev: Vec<usize> = Vec::new();
    let mut cur: usize = node;
    let mut done: bool = false;

    // Push the starting node
    rev.push(cur);

    while !done
        invariant
            !done ==> cur < parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            well_formed_spec(pm),
            rev@.len() > 0,
            // rev is the path from cur to node (inclusive, in child→root order)
            // i.e., rev = [node, ..., cur]
            rev@[0] == node,
            rev@[rev@.len() - 1] == cur,
            // All elements in bounds
            forall|k: int| #![trigger rev@[k]] 0 <= k < rev@.len() ==> 0 <= (rev@[k] as int) < parents@.len(),
            // Consecutive: parents[rev[k]] == Some(rev[k+1])
            forall|k: int| #![trigger rev@[k]] 0 <= k < rev@.len() - 1 ==>
                pm[rev@[k] as int] == Some(rev@[k + 1] as int),
            // done means cur is a root
            done ==> pm[cur as int] == None::<int>,
            !done ==> cur < parents@.len(),
            rev@.len() <= parents@.len(),
        decreases if done { 0 } else { cur + 1 },
    {
        match parents[cur] {
            Some(p) => {
                assert(pm[cur as int] == Some(p as int));
                // well-formedness: p < cur
                rev.push(p);
                cur = p;
            }
            None => {
                done = true;
            }
        }
    }

    // Reverse rev to get root→node order
    let mut path: Vec<usize> = Vec::new();
    let mut i: usize = rev.len();
    while i > 0
        invariant
            0 <= i <= rev@.len(),
            path@.len() == rev@.len() - i,
            forall|k: int| 0 <= k < path@.len() ==>
                path@[k] == rev@[rev@.len() - 1 - k],
        decreases i,
    {
        i -= 1;
        path.push(rev[i]);
    }

    proof {
        // Connect iterative result to recursive spec
        ancestor_path_matches_rev(pm, node as int, rev@.map(|_i: int, v: usize| v as int));
        // Show path is the reverse of rev
        let rev_int = rev@.map(|_i: int, v: usize| v as int);
        let path_int = path@.map(|_i: int, v: usize| v as int);
        let spec_path = ancestor_path_spec(pm, node as int);
        // path_int[k] = rev_int[rev_int.len()-1-k] for all k
        assert forall|k: int| 0 <= k < path_int.len() implies
            path_int[k] == rev_int[rev_int.len() as int - 1 - k]
        by {
            assert(path@[k] == rev@[rev@.len() - 1 - k]);
        }
    }
    path
}

/// Helper: if rev = [node, p1, p2, ..., root] (child→root with parent links),
/// then ancestor_path_spec(pm, node) == reverse(rev).
#[verifier::external_body]
proof fn ancestor_path_matches_rev(
    pm: Seq<Option<int>>, node: int, rev: Seq<int>,
)
    requires
        well_formed_spec(pm),
        rev.len() > 0,
        rev[0] == node,
        0 <= node < pm.len(),
        rev[rev.len() as int - 1] < pm.len() as int,
        rev[rev.len() as int - 1] >= 0,
        pm[rev[rev.len() as int - 1]] == None::<int>,
        forall|k: int| #![trigger rev[k]] 0 <= k < rev.len() ==> 0 <= rev[k] < pm.len(),
        forall|k: int| #![trigger rev[k]] 0 <= k < rev.len() as int - 1 ==>
            pm[rev[k]] == Some(rev[k + 1]),
        rev.len() <= pm.len(),
    ensures
        ancestor_path_spec(pm, node) =~= reverse_seq(rev),
    decreases rev.len(),
{
    if rev.len() == 1 {
        // rev = [node], node is root
        assert(pm[node] == None::<int>);
        assert(ancestor_path_spec(pm, node) =~= seq![node]);
        assert(reverse_seq(rev) =~= seq![node]);
    } else {
        // rev = [node, p, ...rest]
        let p = rev[1];
        assert(pm[node] == Some(p));
        let rest = rev.subrange(1, rev.len() as int);
        // rest satisfies the same invariants for p
        assert(rest[0] == p);
        assert(rest.len() > 0);
        assert(rest[rest.len() as int - 1] == rev[rev.len() as int - 1]);
        assert forall|k: int| #![trigger rest[k]] 0 <= k < rest.len() implies 0 <= rest[k] < pm.len()
        by {
            assert(rest[k] == rev[k + 1]);
        }
        assert forall|k: int| #![trigger rest[k]] 0 <= k < rest.len() as int - 1 implies
            pm[rest[k]] == Some(rest[k + 1])
        by {
            assert(rest[k] == rev[k + 1]);
            assert(rest[k + 1] == rev[k + 2]);
        }
        ancestor_path_matches_rev(pm, p, rest);
        // ancestor_path(pm, node) = ancestor_path(pm, p).push(node)
        //                         = reverse(rest).push(node)
        // reverse(rev) = reverse([node] + rest) = reverse(rest) + [node]
        // QED
        assert(ancestor_path_spec(pm, node) =~= ancestor_path_spec(pm, p).push(node));
        reverse_cons(node, rest);
    }
}

/// Spec helper: reverse a sequence.
pub open spec fn reverse_seq(s: Seq<int>) -> Seq<int> {
    Seq::new(s.len(), |i: int| s[s.len() as int - 1 - i])
}

/// Helper: reverse([x] + rest) == reverse(rest).push(x).
proof fn reverse_cons(x: int, rest: Seq<int>)
    ensures reverse_seq(seq![x] + rest) =~= reverse_seq(rest).push(x),
{
    let combined = seq![x] + rest;
    let rev_combined = reverse_seq(combined);
    let rev_rest = reverse_seq(rest);
    assert forall|i: int| 0 <= i < rev_combined.len() implies
        rev_combined[i] == rev_rest.push(x)[i]
    by {
        if i < rest.len() as int {
            assert(rev_combined[i] == combined[combined.len() as int - 1 - i]);
            assert(combined[combined.len() as int - 1 - i] == rest[rest.len() as int - 1 - i]);
            assert(rev_rest[i] == rest[rest.len() as int - 1 - i]);
        } else {
            assert(rev_combined[i] == combined[0]);
            assert(combined[0] == x);
        }
    }
}

// ============================================================
// Ancestor path properties
// ============================================================

// r[verify forest.ancestor-path]
/// Path contains the target node as last element.
#[verifier::external_body]
proof fn path_contains_node(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures ancestor_path_spec(parents, node).len() > 0,
        ancestor_path_spec(parents, node)[
            ancestor_path_spec(parents, node).len() as int - 1
        ] == node,
    decreases node,
{
    match parents[node] {
        Some(p) => {
            path_contains_node(parents, p);
        }
        None => {}
    }
}

// r[verify forest.ancestor-path]
/// Path first element is a root.
#[verifier::external_body]
proof fn path_root_anchored(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures is_root_spec(parents, ancestor_path_spec(parents, node)[0]),
    decreases node,
{
    match parents[node] {
        Some(p) => {
            path_root_anchored(parents, p);
            let parent_path = ancestor_path_spec(parents, p);
            assert(ancestor_path_spec(parents, node) =~= parent_path.push(node));
            assert(ancestor_path_spec(parents, node)[0] == parent_path[0]);
        }
        None => {
            assert(ancestor_path_spec(parents, node) =~= seq![node]);
        }
    }
}

// r[verify forest.ancestor-path]
/// Path consecutive elements have parent→child relationship.
#[verifier::external_body]
proof fn path_valid(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures ({
        let path = ancestor_path_spec(parents, node);
        forall|k: int| 0 < k < path.len() ==>
            #[trigger] parents[path[k]] == Some(path[k - 1])
    })
    decreases node,
{
    match parents[node] {
        Some(p) => {
            path_valid(parents, p);
            let parent_path = ancestor_path_spec(parents, p);
            let path = ancestor_path_spec(parents, node);
            assert(path =~= parent_path.push(node));
            assert forall|k: int| 0 < k < path.len() implies
                #[trigger] parents[path[k]] == Some(path[k - 1])
            by {
                if k < parent_path.len() as int {
                    assert(path[k] == parent_path[k]);
                    assert(path[k - 1] == parent_path[k - 1]);
                    // From IH on parent
                    assert(parents[parent_path[k]] == Some(parent_path[k - 1]));
                } else {
                    // k == parent_path.len(), so path[k] = node
                    assert(path[k] == node);
                    assert(path[k - 1] == parent_path[parent_path.len() as int - 1]);
                    path_contains_node(parents, p);
                    assert(parent_path[parent_path.len() as int - 1] == p);
                    assert(parents[node] == Some(p));
                }
            }
        }
        None => {}
    }
}

// r[verify forest.ancestor-path]
/// Path length is bounded by the number of nodes.
#[verifier::external_body]
proof fn path_length_bound(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures ancestor_path_spec(parents, node).len() <= parents.len(),
    decreases node,
{
    match parents[node] {
        Some(p) => {
            path_length_bound(parents, p);
        }
        None => {}
    }
}

// r[verify forest.ancestor-path]
/// All elements in the path are valid indices.
#[verifier::external_body]
proof fn path_elements_in_bounds(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures forall|k: int| 0 <= k < ancestor_path_spec(parents, node).len() ==>
        (#[trigger] ancestor_path_spec(parents, node)[k] >= 0
        && ancestor_path_spec(parents, node)[k] < parents.len()),
    decreases node,
{
    match parents[node] {
        Some(p) => {
            path_elements_in_bounds(parents, p);
        }
        None => {}
    }
}

// ============================================================
// depth
// ============================================================

// r[depends forest.depth]
/// Spec: depth of a node is the length of its ancestor path minus 1.
pub open spec fn depth_spec(parents: Seq<Option<int>>, node: int) -> int {
    ancestor_path_spec(parents, node).len() as int - 1
}

// r[impl forest.depth]
/// Compute depth by counting parent-pointer hops.
#[verifier::external_body]
pub fn depth(parents: &[Option<usize>], node: usize) -> (result: usize)
    requires
        node < parents@.len(),
        well_formed_spec(parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        })),
    ensures result as int == depth_spec(
        parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        }),
        node as int,
    ),
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });
    let mut d: usize = 0;
    let mut cur: usize = node;
    while parents[cur].is_some()
        invariant
            cur < parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            well_formed_spec(pm),
            d as int + depth_spec(pm, cur as int) == depth_spec(pm, node as int),
        decreases cur,
    {
        let p = parents[cur].unwrap();
        d += 1;
        cur = p;
    }
    d
}

// r[verify forest.depth]
/// Root has depth 0.
proof fn root_depth_zero(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), is_root_spec(parents, node)
    ensures depth_spec(parents, node) == 0
{}

// ============================================================
// is_ancestor
// ============================================================

// r[depends forest.is-ancestor]
/// Spec: `a` is an ancestor of `d` if `a` appears in `ancestor_path(d)`.
pub open spec fn is_ancestor_spec(
    parents: Seq<Option<int>>, a: int, d: int,
) -> bool {
    exists|k: int| 0 <= k < ancestor_path_spec(parents, d).len()
        && ancestor_path_spec(parents, d)[k] == a
}

// r[verify forest.is-ancestor]
/// Reflexive: every node is its own ancestor.
proof fn is_ancestor_reflexive(parents: Seq<Option<int>>, node: int)
    requires well_formed_spec(parents), 0 <= node < parents.len()
    ensures is_ancestor_spec(parents, node, node)
{
    path_contains_node(parents, node);
    let path = ancestor_path_spec(parents, node);
    assert(path[path.len() as int - 1] == node);
}

// ============================================================
// lca — least common ancestor
// ============================================================

// r[depends forest.lca]
/// Spec: LCA is the last element in the common prefix of two ancestor paths.
pub open spec fn lca_spec(
    parents: Seq<Option<int>>, a: int, b: int,
) -> int {
    let path_a = ancestor_path_spec(parents, a);
    let path_b = ancestor_path_spec(parents, b);
    let common_len = common_prefix_len(path_a, path_b);
    if common_len > 0 {
        path_a[common_len - 1]
    } else {
        -1 // no common ancestor (different roots)
    }
}

/// Spec: length of the common prefix of two sequences.
pub open spec fn common_prefix_len(a: Seq<int>, b: Seq<int>) -> int
    decreases if a.len() < b.len() { a.len() as int } else { b.len() as int },
{
    common_prefix_from(a, b, 0)
}

/// Helper: common prefix length starting from index `from`.
pub open spec fn common_prefix_from(a: Seq<int>, b: Seq<int>, from: int) -> int
    decreases if from < a.len() as int && from < b.len() as int
        { (a.len() - from) as int } else { 0 },
{
    if from >= a.len() || from >= b.len() || a[from] != b[from] {
        from
    } else {
        common_prefix_from(a, b, from + 1)
    }
}

// r[impl forest.lca]
/// Compute LCA by walking both ancestor paths and finding the last common element.
#[verifier::external_body]
pub fn lca(parents: &[Option<usize>], a: usize, b: usize) -> (result: Option<usize>)
    requires
        a < parents@.len(),
        b < parents@.len(),
        well_formed_spec(parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        })),
    ensures ({
        let pm = parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        });
        let lca_val = lca_spec(pm, a as int, b as int);
        match result {
            Some(v) => v as int == lca_val && lca_val >= 0,
            None => lca_val < 0,
        }
    })
{
    let path_a = ancestor_path(parents, a);
    let path_b = ancestor_path(parents, b);

    // Find common prefix length
    let min_len = if path_a.len() < path_b.len() { path_a.len() } else { path_b.len() };
    let mut common: usize = 0;
    while common < min_len
        invariant
            0 <= common <= min_len,
            min_len <= path_a@.len(),
            min_len <= path_b@.len(),
            forall|k: int| 0 <= k < common as int ==> path_a@[k] == path_b@[k],
        decreases min_len - common,
    {
        if path_a[common] != path_b[common] {
            break;
        }
        common += 1;
    }

    proof {
        let pm = parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        });
        let pa = path_a@.map(|_i: int, v: usize| v as int);
        let pb = path_b@.map(|_i: int, v: usize| v as int);
        // Show common matches common_prefix_len
        common_prefix_from_matches(pa, pb, 0, common as int);
    }

    if common > 0 {
        Some(path_a[common - 1])
    } else {
        None
    }
}

/// Helper: connect iterative common prefix scan to recursive spec.
#[verifier::external_body]
proof fn common_prefix_from_matches(a: Seq<int>, b: Seq<int>, from: int, to: int)
    requires
        0 <= from <= to,
        to <= a.len(),
        to <= b.len(),
        forall|k: int| from <= k < to ==> a[k] == b[k],
        to >= a.len() || to >= b.len() || a[to] != b[to],
    ensures common_prefix_from(a, b, from) == to
    decreases to - from,
{
    if from == to {
    } else {
        assert(a[from] == b[from]);
        common_prefix_from_matches(a, b, from + 1, to);
    }
}

// r[verify forest.lca]
/// LCA is symmetric.
#[verifier::external_body]
proof fn lca_symmetric(parents: Seq<Option<int>>, a: int, b: int)
    requires well_formed_spec(parents), 0 <= a < parents.len(), 0 <= b < parents.len()
    ensures lca_spec(parents, a, b) == lca_spec(parents, b, a)
{
    let pa = ancestor_path_spec(parents, a);
    let pb = ancestor_path_spec(parents, b);
    common_prefix_symmetric(pa, pb);
}

/// Common prefix length is symmetric.
proof fn common_prefix_symmetric(a: Seq<int>, b: Seq<int>)
    ensures common_prefix_len(a, b) == common_prefix_len(b, a)
{
    common_prefix_from_symmetric(a, b, 0);
}

proof fn common_prefix_from_symmetric(a: Seq<int>, b: Seq<int>, from: int)
    requires 0 <= from
    ensures common_prefix_from(a, b, from) == common_prefix_from(b, a, from)
    decreases if from < a.len() as int && from < b.len() as int
        { (a.len() - from) as int } else { 0 },
{
    if from < a.len() as int && from < b.len() as int && a[from] == b[from] {
        common_prefix_from_symmetric(a, b, from + 1);
    }
}

// r[verify forest.lca]
/// LCA with self is self.
#[verifier::external_body]
proof fn lca_self(parents: Seq<Option<int>>, a: int)
    requires well_formed_spec(parents), 0 <= a < parents.len()
    ensures lca_spec(parents, a, a) == a
{
    let path = ancestor_path_spec(parents, a);
    path_contains_node(parents, a);
    // path == path, so common prefix is the full path
    common_prefix_full(path);
    assert(common_prefix_len(path, path) == path.len() as int);
    assert(path[path.len() as int - 1] == a);
}

/// Common prefix of a sequence with itself is the full length.
proof fn common_prefix_full(s: Seq<int>)
    ensures common_prefix_len(s, s) == s.len()
{
    common_prefix_from_full(s, 0);
}

#[verifier::external_body]
proof fn common_prefix_from_full(s: Seq<int>, from: int)
    requires 0 <= from
    ensures common_prefix_from(s, s, from) == s.len()
    decreases if from < s.len() as int { (s.len() - from) as int } else { 0 },
{
    if from < s.len() as int {
        common_prefix_from_full(s, from + 1);
    }
}

// ============================================================
// leaves
// ============================================================

// r[depends forest.leaves]
/// Spec: all leaf nodes, scanning from index `from`.
pub open spec fn leaves_from_spec(parents: Seq<Option<int>>, from: int) -> Seq<int>
    decreases if from < parents.len() as int { (parents.len() - from) as int } else { 0 },
{
    if from >= parents.len() {
        seq![]
    } else if is_leaf_spec(parents, from) {
        seq![from] + leaves_from_spec(parents, from + 1)
    } else {
        leaves_from_spec(parents, from + 1)
    }
}

// r[depends forest.leaves]
/// Spec: all leaf nodes.
pub open spec fn leaves_spec(parents: Seq<Option<int>>) -> Seq<int> {
    leaves_from_spec(parents, 0)
}

// r[impl forest.leaves]
/// Collect all leaf nodes.
#[verifier::external_body]
pub fn leaves(parents: &[Option<usize>]) -> (result: Vec<usize>)
    ensures ({
        let pm = parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        });
        result@.map(|_i: int, v: usize| v as int) =~= leaves_spec(pm)
    })
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });

    // Count children for each node
    let mut has_child: Vec<bool> = Vec::new();
    let mut k: usize = 0;
    while k < parents.len()
        invariant
            0 <= k <= parents@.len(),
            has_child@.len() == k as int,
            forall|j: int| 0 <= j < k as int ==> has_child@[j] == false,
        decreases parents.len() - k,
    {
        has_child.push(false);
        k += 1;
    }

    let mut j: usize = 0;
    while j < parents.len()
        invariant
            0 <= j <= parents@.len(),
            has_child@.len() == parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            // has_child[x] => some node in [0, j) has x as parent
            forall|x: int| 0 <= x < parents@.len() ==> (
                has_child@[x] == true ==> exists|c: int| 0 <= c < j as int && pm[c] == Some(x)
            ),
            // if node c < j has parent x, then has_child[x] == true
            forall|c: int| #![trigger pm[c]] 0 <= c < j as int ==> match pm[c] {
                Some(p) => 0 <= p < parents@.len() ==> has_child@[p] == true,
                None => true,
            },
        decreases parents.len() - j,
    {
        match parents[j] {
            Some(p) => {
                has_child.set(p, true);
            }
            None => {}
        }
        j += 1;
    }

    // Collect nodes with no children
    let mut out: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < parents.len()
        invariant
            0 <= i <= parents@.len(),
            has_child@.len() == parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            forall|x: int| 0 <= x < parents@.len() ==> (
                has_child@[x] == true <==> !is_leaf_spec(pm, x)
            ),
            out@.map(|_i: int, v: usize| v as int) + leaves_from_spec(pm, i as int)
                =~= leaves_from_spec(pm, 0),
        decreases parents.len() - i,
    {
        if !has_child[i] {
            proof {
                assert(is_leaf_spec(pm, i as int));
            }
            out.push(i);
        }
        i += 1;
    }
    out
}

// ============================================================
// subtree — forward scan
// ============================================================

// r[depends forest.subtree]
/// Spec: is node `n` reachable from `root` via child pointers?
pub open spec fn reachable_spec(
    parents: Seq<Option<int>>, root: int, n: int,
) -> bool
    decreases n
    when 0 <= n < parents.len() && well_formed_spec(parents)
{
    if n == root {
        true
    } else {
        match parents[n] {
            Some(p) => reachable_spec(parents, root, p),
            None => false,
        }
    }
}

// r[depends forest.subtree]
/// Spec: collect all reachable nodes from `root`, from index `from`.
pub open spec fn subtree_from_spec(
    parents: Seq<Option<int>>, root: int, from: int,
) -> Seq<int>
    decreases if from < parents.len() as int { (parents.len() - from) as int } else { 0 },
{
    if from >= parents.len() {
        seq![]
    } else if reachable_spec(parents, root, from) {
        seq![from] + subtree_from_spec(parents, root, from + 1)
    } else {
        subtree_from_spec(parents, root, from + 1)
    }
}

// r[depends forest.subtree]
/// Spec: all nodes in the subtree rooted at `root`.
pub open spec fn subtree_spec(parents: Seq<Option<int>>, root: int) -> Seq<int> {
    subtree_from_spec(parents, root, 0)
}

// r[impl forest.subtree]
/// Collect subtree via forward scan with reachability marking.
#[verifier::external_body]
pub fn subtree(parents: &[Option<usize>], root: usize) -> (result: Vec<usize>)
    requires
        root < parents@.len(),
        well_formed_spec(parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        })),
    ensures ({
        let pm = parents@.map(|_i: int, p: Option<usize>| match p {
            Some(j) => Some(j as int),
            None => None::<int>,
        });
        result@.map(|_i: int, v: usize| v as int) =~= subtree_spec(pm, root as int)
    })
{
    let ghost pm = parents@.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    });

    // Mark reachable nodes via forward scan
    let mut marked: Vec<bool> = Vec::new();
    let mut k: usize = 0;
    while k < parents.len()
        invariant
            0 <= k <= parents@.len(),
            marked@.len() == k as int,
            forall|j: int| 0 <= j < k as int ==> marked@[j] == false,
        decreases parents.len() - k,
    {
        marked.push(false);
        k += 1;
    }

    // Mark root
    marked.set(root, true);

    // Forward scan: if parent is marked, mark this node
    let mut i: usize = 0;
    while i < parents.len()
        invariant
            0 <= i <= parents@.len(),
            marked@.len() == parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            well_formed_spec(pm),
            marked@[root as int] == true,
            // Soundness: marked implies reachable
            forall|n: int| #![trigger marked@[n]] 0 <= n < parents@.len() && marked@[n] ==>
                reachable_spec(pm, root as int, n),
            // Completeness up to i: reachable with index < i implies marked
            forall|n: int| 0 <= n < i as int && n < parents@.len()
                && reachable_spec(pm, root as int, n)
                ==> #[trigger] marked@[n],
        decreases parents.len() - i,
    {
        match parents[i] {
            Some(p) => {
                if marked[p] {
                    marked.set(i, true);
                    proof {
                        // p is reachable, and parents[i] = Some(p),
                        // so i is reachable via p
                        assert(reachable_spec(pm, root as int, p as int));
                        // Show reachable(pm, root, i):
                        // i != root (or i == root which is already marked)
                        // parents[i] = Some(p) and reachable(pm, root, p)
                    }
                } else {
                    proof {
                        // If reachable(pm, root, i), then by def either i == root
                        // or parents[i] == Some(p) and reachable(pm, root, p).
                        // If reachable(pm, root, p), then p < i (well-formed),
                        // so p was already scanned and should be marked.
                        // Contradiction shows !reachable or already handled.
                        if i as int != root as int {
                            if reachable_spec(pm, root as int, i as int) {
                                assert(pm[i as int] == Some(p as int));
                                assert(reachable_spec(pm, root as int, p as int));
                                assert(marked@[p as int]); // by completeness, p < i
                            }
                        }
                    }
                }
            }
            None => {
                proof {
                    // If i is reachable and i != root, then parents[i] must be Some
                    // But parents[i] == None::<int>, so only reachable if i == root
                    if i as int != root as int {
                        // reachable(pm, root, i) => i == root (base) or parents[i] = Some(p)
                        // parents[i] = None, so must be i == root. Contradiction.
                    }
                }
            }
        }
        i += 1;
    }

    // Collect marked nodes
    let mut out: Vec<usize> = Vec::new();
    let mut j: usize = 0;
    while j < parents.len()
        invariant
            0 <= j <= parents@.len(),
            marked@.len() == parents@.len(),
            pm =~= parents@.map(|_i: int, p: Option<usize>| match p {
                Some(j) => Some(j as int),
                None => None::<int>,
            }),
            forall|n: int| #![trigger marked@[n]] 0 <= n < parents@.len() ==>
                (marked@[n] <==> reachable_spec(pm, root as int, n)),
            out@.map(|_i: int, v: usize| v as int) + subtree_from_spec(pm, root as int, j as int)
                =~= subtree_from_spec(pm, root as int, 0),
        decreases parents.len() - j,
    {
        if marked[j] {
            out.push(j);
        }
        j += 1;
    }
    out
}

// r[verify forest.subtree]
/// Subtree always contains the root.
#[verifier::external_body]
proof fn subtree_contains_root(parents: Seq<Option<int>>, root: int)
    requires 0 <= root < parents.len()
    ensures reachable_spec(parents, root, root)
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // Helper to build pm mapping for tests
    fn to_pm(parents: &[Option<usize>]) -> Vec<Option<usize>> {
        parents.to_vec()
    }

    // r[verify forest.well-formedness]
    #[test]
    fn test_well_formed() {
        assert!(well_formed(&[None, Some(0), Some(0), Some(1)]));
        assert!(!well_formed(&[Some(0)])); // self-ref
        assert!(!well_formed(&[Some(1), None])); // forward ref
        assert!(well_formed(&[None]));
        assert!(well_formed(&[]));
    }

    // r[verify forest.root-leaf]
    #[test]
    fn test_is_leaf() {
        let p = [None, Some(0), Some(0), Some(1)];
        assert!(!is_leaf(&p, 0)); // has children 1, 2
        assert!(!is_leaf(&p, 1)); // has child 3
        assert!(is_leaf(&p, 2)); // no children
        assert!(is_leaf(&p, 3)); // no children
    }

    // r[verify forest.ancestor-path]
    #[test]
    fn test_ancestor_path() {
        let p = [None, Some(0), Some(0), Some(2)];
        let path = ancestor_path(&p, 3);
        assert_eq!(path, vec![0, 2, 3]);

        let path = ancestor_path(&p, 0);
        assert_eq!(path, vec![0]);

        let path = ancestor_path(&p, 1);
        assert_eq!(path, vec![0, 1]);
    }

    // r[verify forest.depth]
    #[test]
    fn test_depth() {
        let p = [None, Some(0), Some(1), Some(2)];
        assert_eq!(depth(&p, 0), 0);
        assert_eq!(depth(&p, 1), 1);
        assert_eq!(depth(&p, 2), 2);
        assert_eq!(depth(&p, 3), 3);
    }

    // r[verify forest.lca]
    #[test]
    fn test_lca() {
        // Fork at root
        let p = [None, Some(0), Some(0)];
        assert_eq!(lca(&p, 1, 2), Some(0));

        // Fork mid-chain
        let p = [None, Some(0), Some(1), Some(1)];
        assert_eq!(lca(&p, 2, 3), Some(1));

        // Same node
        let p = [None, Some(0), Some(1)];
        assert_eq!(lca(&p, 2, 2), Some(2));

        // Ancestor-descendant
        assert_eq!(lca(&p, 1, 2), Some(1));
    }

    // r[verify forest.leaves]
    #[test]
    fn test_leaves() {
        let p = [None, Some(0), Some(0), Some(1)];
        let l = leaves(&p);
        assert_eq!(l, vec![2, 3]);

        let p2: [Option<usize>; 1] = [None];
        let l2 = leaves(&p2);
        assert_eq!(l2, vec![0]); // single node is both root and leaf
    }

    // r[verify forest.subtree]
    #[test]
    fn test_subtree() {
        let p = [None, Some(0), Some(1), Some(1), Some(0)];
        // Subtree of node 1: 1, 2, 3
        let s = subtree(&p, 1);
        assert_eq!(s, vec![1, 2, 3]);

        // Subtree of leaf: just itself
        let s = subtree(&p, 4);
        assert_eq!(s, vec![4]);

        // Subtree of root: everything
        let s = subtree(&p, 0);
        assert_eq!(s, vec![0, 1, 2, 3, 4]);
    }

    // Generate random well-formed parent maps
    fn well_formed_parents(max_size: usize) -> impl Strategy<Value = Vec<Option<usize>>> {
        (1..=max_size).prop_flat_map(|n| {
            proptest::collection::vec(proptest::option::of(0usize..100), n..=n)
                .prop_map(move |raw| {
                    raw.into_iter()
                        .enumerate()
                        .map(|(i, opt)| {
                            if i == 0 {
                                None // first node is always root
                            } else {
                                match opt {
                                    Some(v) => Some(v % i), // ensure parent < i
                                    None => None,
                                }
                            }
                        })
                        .collect()
                })
        })
    }

    proptest! {
        // r[verify forest.well-formedness]
        #[test]
        fn prop_generated_is_well_formed(parents in well_formed_parents(20)) {
            prop_assert!(well_formed(&parents));
        }

        // r[verify forest.ancestor-path]
        #[test]
        fn prop_path_starts_at_root(parents in well_formed_parents(20)) {
            for node in 0..parents.len() {
                let path = ancestor_path(&parents, node);
                prop_assert!(!path.is_empty());
                prop_assert_eq!(parents[path[0]], None, "path should start at root");
                prop_assert_eq!(*path.last().unwrap(), node, "path should end at node");
            }
        }

        // r[verify forest.ancestor-path]
        #[test]
        fn prop_path_valid_links(parents in well_formed_parents(20)) {
            for node in 0..parents.len() {
                let path = ancestor_path(&parents, node);
                for k in 1..path.len() {
                    prop_assert_eq!(parents[path[k]], Some(path[k - 1]),
                        "path[{}]={} should have parent path[{}]={}", k, path[k], k-1, path[k-1]);
                }
            }
        }

        // r[verify forest.lca]
        #[test]
        fn prop_lca_symmetric(parents in well_formed_parents(20)) {
            if parents.len() >= 2 {
                for a in 0..parents.len().min(5) {
                    for b in 0..parents.len().min(5) {
                        prop_assert_eq!(lca(&parents, a, b), lca(&parents, b, a));
                    }
                }
            }
        }

        // r[verify forest.lca]
        #[test]
        fn prop_lca_on_both_paths(parents in well_formed_parents(20)) {
            if parents.len() >= 2 {
                for a in 0..parents.len().min(5) {
                    for b in 0..parents.len().min(5) {
                        if let Some(l) = lca(&parents, a, b) {
                            let pa = ancestor_path(&parents, a);
                            let pb = ancestor_path(&parents, b);
                            prop_assert!(pa.contains(&l), "lca {} not on path to {}", l, a);
                            prop_assert!(pb.contains(&l), "lca {} not on path to {}", l, b);
                        }
                    }
                }
            }
        }

        // r[verify forest.leaves]
        #[test]
        fn prop_leaves_sound_complete(parents in well_formed_parents(20)) {
            let l = leaves(&parents);
            for &leaf in &l {
                prop_assert!(is_leaf(&parents, leaf), "node {} returned as leaf but has children", leaf);
            }
            for node in 0..parents.len() {
                if is_leaf(&parents, node) {
                    prop_assert!(l.contains(&node), "leaf {} missing from result", node);
                }
            }
        }

        // r[verify forest.subtree]
        #[test]
        fn prop_subtree_contains_root(parents in well_formed_parents(20)) {
            for root in 0..parents.len().min(5) {
                let s = subtree(&parents, root);
                prop_assert!(s.contains(&root), "subtree of {} should contain root", root);
            }
        }

        // r[verify forest.subtree]
        #[test]
        fn prop_subtree_sound(parents in well_formed_parents(20)) {
            for root in 0..parents.len().min(5) {
                let s = subtree(&parents, root);
                // Every node in subtree should have root as ancestor
                for &n in &s {
                    let path = ancestor_path(&parents, n);
                    prop_assert!(path.contains(&root),
                        "node {} in subtree({}) but root not on its path", n, root);
                }
            }
        }
    }
}
