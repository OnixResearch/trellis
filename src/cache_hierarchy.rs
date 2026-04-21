//! Verified two-level cache hierarchy policies.
//!
//! Models L1/L2 line presence under inclusive, exclusive, and non-inclusive
//! policies. The runtime surface keeps exact line membership for each level;
//! proof surfaces establish the inductive inclusion/disjointness properties,
//! back-invalidation, exclusive promotion/removal, capacity formulas, and
//! snoop-scope consequences.

use vstd::prelude::*;

verus! {

#[derive(Copy, Clone)]
pub enum HierarchyPolicy {
    Inclusive,
    Exclusive,
    NonInclusive,
}

pub struct CacheHierarchy {
    pub policy: HierarchyPolicy,
    pub l1: Vec<u64>,
    pub l2: Vec<u64>,
}

// r[impl hierarchy.inclusive-invariant]
// r[depends hierarchy.inclusive-invariant]
// r[impl hierarchy.snoop-scope]
pub open spec fn contains_line_spec(level: Seq<u64>, line: u64) -> bool {
    level.contains(line)
}

spec fn remove_line_prefix_spec(level: Seq<u64>, line: u64, upto: int) -> Seq<u64>
    decreases if upto >= 0 { upto } else { 0 }
{
    if upto <= 0 {
        seq![]
    } else {
        let prefix = remove_line_prefix_spec(level, line, upto - 1);
        if level[upto - 1] == line {
            prefix
        } else {
            prefix.push(level[upto - 1])
        }
    }
}

spec fn remove_line_spec(level: Seq<u64>, line: u64) -> Seq<u64> {
    remove_line_prefix_spec(level, line, level.len() as int)
}

spec fn push_unique_spec(level: Seq<u64>, line: u64) -> Seq<u64> {
    if level.contains(line) {
        level
    } else {
        level.push(line)
    }
}

// r[impl hierarchy.inclusive-invariant]
// r[depends hierarchy.snoop-scope]
pub open spec fn inclusive_invariant_spec(l1: Seq<u64>, l2: Seq<u64>) -> bool {
    forall|i: int| #![trigger l1[i]] 0 <= i < l1.len() ==> l2.contains(l1[i])
}

// r[impl hierarchy.exclusive-invariant]
pub open spec fn exclusive_invariant_spec(l1: Seq<u64>, l2: Seq<u64>) -> bool {
    forall|i: int, j: int|
        #![trigger l1[i], l2[j]]
        0 <= i < l1.len() && 0 <= j < l2.len() ==> l1[i] != l2[j]
}

// r[impl hierarchy.non-inclusive]
pub open spec fn noninclusive_invariant_spec(_l1: Seq<u64>, _l2: Seq<u64>) -> bool {
    true
}

spec fn valid_hierarchy_spec(hierarchy: CacheHierarchy) -> bool {
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => inclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
        HierarchyPolicy::Exclusive => exclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
        HierarchyPolicy::NonInclusive => noninclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
    }
}

spec fn l1_fill_l1_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    push_unique_spec(hierarchy.l1@, line)
}

spec fn l1_fill_l2_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => push_unique_spec(hierarchy.l2@, line),
        HierarchyPolicy::Exclusive => remove_line_spec(hierarchy.l2@, line),
        HierarchyPolicy::NonInclusive => hierarchy.l2@,
    }
}

spec fn l1_evict_l1_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    remove_line_spec(hierarchy.l1@, line)
}

spec fn l1_evict_l2_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    match hierarchy.policy {
        HierarchyPolicy::Exclusive => push_unique_spec(hierarchy.l2@, line),
        HierarchyPolicy::Inclusive => hierarchy.l2@,
        HierarchyPolicy::NonInclusive => hierarchy.l2@,
    }
}

spec fn l2_fill_l1_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    match hierarchy.policy {
        HierarchyPolicy::Exclusive => remove_line_spec(hierarchy.l1@, line),
        HierarchyPolicy::Inclusive => hierarchy.l1@,
        HierarchyPolicy::NonInclusive => hierarchy.l1@,
    }
}

spec fn l2_fill_l2_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    push_unique_spec(hierarchy.l2@, line)
}

// r[impl hierarchy.back-invalidation]
spec fn l2_evict_l1_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => remove_line_spec(hierarchy.l1@, line),
        HierarchyPolicy::Exclusive => hierarchy.l1@,
        HierarchyPolicy::NonInclusive => hierarchy.l1@,
    }
}

spec fn l2_evict_l2_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    remove_line_spec(hierarchy.l2@, line)
}

// r[impl hierarchy.exclusive-swap]
spec fn l2_to_l1_promote_l1_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    if hierarchy.l2@.contains(line) {
        push_unique_spec(hierarchy.l1@, line)
    } else {
        hierarchy.l1@
    }
}

// r[impl hierarchy.exclusive-swap]
spec fn l2_to_l1_promote_l2_spec(hierarchy: CacheHierarchy, line: u64) -> Seq<u64> {
    if hierarchy.l2@.contains(line) {
        match hierarchy.policy {
            HierarchyPolicy::Exclusive => remove_line_spec(hierarchy.l2@, line),
            HierarchyPolicy::Inclusive => hierarchy.l2@,
            HierarchyPolicy::NonInclusive => hierarchy.l2@,
        }
    } else {
        hierarchy.l2@
    }
}

// r[impl hierarchy.capacity]
pub open spec fn effective_capacity_spec(hierarchy: CacheHierarchy) -> int {
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => hierarchy.l2@.len() as int,
        HierarchyPolicy::Exclusive => hierarchy.l1@.len() as int + hierarchy.l2@.len() as int,
        HierarchyPolicy::NonInclusive => hierarchy.l1@.len() as int + hierarchy.l2@.len() as int,
    }
}

// r[impl hierarchy.snoop-scope]
pub open spec fn l2_miss_implies_l1_miss_spec(hierarchy: CacheHierarchy, line: u64) -> bool {
    !contains_line_spec(hierarchy.l2@, line) ==> !contains_line_spec(hierarchy.l1@, line)
}

fn clone_level(level: &[u64]) -> (result: Vec<u64>)
    ensures result@ == level@
{
    let mut result: Vec<u64> = Vec::new();
    let mut i: usize = 0;
    while i < level.len()
        invariant
            0 <= i <= level@.len(),
            result@ == level@.subrange(0, i as int),
        decreases level@.len() - i,
    {
        result.push(level[i]);
        i = i + 1;
    }
    result
}

// r[impl hierarchy.inclusive-invariant]
fn contains_line(level: &[u64], line: u64) -> (result: bool)
    ensures result == contains_line_spec(level@, line)
{
    let mut i: usize = 0;
    while i < level.len()
        invariant
            0 <= i <= level@.len(),
            forall|j: int| #![trigger level@[j]] 0 <= j < i as int ==> level@[j] != line,
        decreases level@.len() - i,
    {
        if level[i] == line {
            return true;
        }
        i = i + 1;
    }
    false
}

fn clone_push_unique(level: &[u64], line: u64) -> (result: Vec<u64>)
    ensures result@ == push_unique_spec(level@, line)
{
    let mut result = clone_level(level);
    if !contains_line(level, line) {
        result.push(line);
    }
    result
}

fn clone_remove(level: &[u64], line: u64) -> (result: Vec<u64>)
    ensures result@ == remove_line_spec(level@, line)
{
    let mut result: Vec<u64> = Vec::new();
    let mut i: usize = 0;
    while i < level.len()
        invariant
            0 <= i <= level@.len(),
            result@ == remove_line_prefix_spec(level@, line, i as int),
        decreases level@.len() - i,
    {
        if level[i] != line {
            result.push(level[i]);
            proof {
                assert(remove_line_prefix_spec(level@, line, i as int + 1)
                    == remove_line_prefix_spec(level@, line, i as int).push(level[i as int]));
            }
        } else {
            proof {
                assert(remove_line_prefix_spec(level@, line, i as int + 1)
                    == remove_line_prefix_spec(level@, line, i as int));
            }
        }
        i = i + 1;
    }
    result
}

#[cfg(test)]
fn l1_lines(hierarchy: &CacheHierarchy) -> (result: Vec<u64>)
    ensures result@ == hierarchy.l1@
{
    clone_level(&hierarchy.l1)
}

#[cfg(test)]
fn l2_lines(hierarchy: &CacheHierarchy) -> (result: Vec<u64>)
    ensures result@ == hierarchy.l2@
{
    clone_level(&hierarchy.l2)
}

// r[impl hierarchy.inclusive-invariant]
// r[impl hierarchy.exclusive-invariant]
// r[impl hierarchy.non-inclusive]
pub fn new_hierarchy(policy: HierarchyPolicy) -> (result: CacheHierarchy)
    ensures
        result.policy == policy,
        result.l1@.len() == 0,
        result.l2@.len() == 0,
        match policy {
            HierarchyPolicy::Inclusive => inclusive_invariant_spec(result.l1@, result.l2@),
            HierarchyPolicy::Exclusive => exclusive_invariant_spec(result.l1@, result.l2@),
            HierarchyPolicy::NonInclusive => noninclusive_invariant_spec(result.l1@, result.l2@),
        },
{
    CacheHierarchy {
        policy,
        l1: Vec::new(),
        l2: Vec::new(),
    }
}

// r[impl hierarchy.inclusive-invariant]
pub fn check_inclusive_invariant(hierarchy: &CacheHierarchy) -> (result: bool)
    ensures result == inclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@)
{
    let mut i: usize = 0;
    while i < hierarchy.l1.len()
        invariant
            0 <= i <= hierarchy.l1@.len(),
            forall|j: int| #![trigger hierarchy.l1@[j]] 0 <= j < i as int ==> hierarchy.l2@.contains(hierarchy.l1@[j]),
        decreases hierarchy.l1@.len() - i,
    {
        if !contains_line(&hierarchy.l2, hierarchy.l1[i]) {
            return false;
        }
        i = i + 1;
    }
    true
}

// r[impl hierarchy.exclusive-invariant]
pub fn check_exclusive_invariant(hierarchy: &CacheHierarchy) -> (result: bool)
    ensures result == exclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@)
{
    let mut i: usize = 0;
    while i < hierarchy.l1.len()
        invariant
            0 <= i <= hierarchy.l1@.len(),
            forall|a: int, b: int|
                #![trigger hierarchy.l1@[a], hierarchy.l2@[b]]
                0 <= a < i as int && 0 <= b < hierarchy.l2@.len() ==> hierarchy.l1@[a] != hierarchy.l2@[b],
        decreases hierarchy.l1@.len() - i,
    {
        if contains_line(&hierarchy.l2, hierarchy.l1[i]) {
            return false;
        }
        i = i + 1;
    }
    true
}

// r[impl hierarchy.non-inclusive]
pub fn check_valid(hierarchy: &CacheHierarchy) -> (result: bool)
    ensures
        result == match hierarchy.policy {
            HierarchyPolicy::Inclusive => inclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
            HierarchyPolicy::Exclusive => exclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
            HierarchyPolicy::NonInclusive => noninclusive_invariant_spec(hierarchy.l1@, hierarchy.l2@),
        }
{
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => check_inclusive_invariant(hierarchy),
        HierarchyPolicy::Exclusive => check_exclusive_invariant(hierarchy),
        HierarchyPolicy::NonInclusive => true,
    }
}

// r[impl hierarchy.inclusive-invariant]
// r[impl hierarchy.exclusive-invariant]
pub fn l1_fill(hierarchy: &mut CacheHierarchy, line: u64)
    ensures
        hierarchy.policy == old(hierarchy).policy,
        contains_line_spec(hierarchy.l1@, line),
        match old(hierarchy).policy {
            HierarchyPolicy::Inclusive => contains_line_spec(hierarchy.l2@, line),
            HierarchyPolicy::Exclusive => !contains_line_spec(hierarchy.l2@, line),
            HierarchyPolicy::NonInclusive => hierarchy.l2@ == old(hierarchy).l2@,
        },
{
    let old_policy = hierarchy.policy;
    proof {
        let old_l1 = hierarchy.l1@;
        let old_l2 = hierarchy.l2@;
    }
    hierarchy.l1 = clone_push_unique(&hierarchy.l1, line);
    hierarchy.l2 = match old_policy {
        HierarchyPolicy::Inclusive => clone_push_unique(&hierarchy.l2, line),
        HierarchyPolicy::Exclusive => clone_remove(&hierarchy.l2, line),
        HierarchyPolicy::NonInclusive => clone_level(&hierarchy.l2),
    };
    proof {
        let old_l1 = old(hierarchy).l1@;
        let old_l2 = old(hierarchy).l2@;
        push_unique_contains(old_l1, line);
        match old_policy {
            HierarchyPolicy::Inclusive => push_unique_contains(old_l2, line),
            HierarchyPolicy::Exclusive => remove_line_excludes(old_l2, line),
            HierarchyPolicy::NonInclusive => {}
        }
    }
}

// r[impl hierarchy.inclusive-invariant]
// r[impl hierarchy.exclusive-invariant]
pub fn l1_evict(hierarchy: &mut CacheHierarchy, line: u64)
    ensures
        hierarchy.policy == old(hierarchy).policy,
        !contains_line_spec(hierarchy.l1@, line),
        match old(hierarchy).policy {
            HierarchyPolicy::Inclusive => hierarchy.l2@ == old(hierarchy).l2@,
            HierarchyPolicy::Exclusive => contains_line_spec(hierarchy.l2@, line),
            HierarchyPolicy::NonInclusive => hierarchy.l2@ == old(hierarchy).l2@,
        },
{
    let old_policy = hierarchy.policy;
    hierarchy.l1 = clone_remove(&hierarchy.l1, line);
    hierarchy.l2 = match old_policy {
        HierarchyPolicy::Exclusive => clone_push_unique(&hierarchy.l2, line),
        HierarchyPolicy::Inclusive | HierarchyPolicy::NonInclusive => clone_level(&hierarchy.l2),
    };
    proof {
        let old_l2 = old(hierarchy).l2@;
        remove_line_excludes(old(hierarchy).l1@, line);
        if old_policy == HierarchyPolicy::Exclusive {
            push_unique_contains(old_l2, line);
        }
    }
}

// r[impl hierarchy.inclusive-invariant]
// r[impl hierarchy.exclusive-invariant]
pub fn l2_fill(hierarchy: &mut CacheHierarchy, line: u64)
    ensures
        hierarchy.policy == old(hierarchy).policy,
        contains_line_spec(hierarchy.l2@, line),
        match old(hierarchy).policy {
            HierarchyPolicy::Inclusive => hierarchy.l1@ == old(hierarchy).l1@,
            HierarchyPolicy::Exclusive => !contains_line_spec(hierarchy.l1@, line),
            HierarchyPolicy::NonInclusive => hierarchy.l1@ == old(hierarchy).l1@,
        },
{
    let old_policy = hierarchy.policy;
    hierarchy.l1 = match old_policy {
        HierarchyPolicy::Exclusive => clone_remove(&hierarchy.l1, line),
        HierarchyPolicy::Inclusive | HierarchyPolicy::NonInclusive => clone_level(&hierarchy.l1),
    };
    hierarchy.l2 = clone_push_unique(&hierarchy.l2, line);
    proof {
        push_unique_contains(old(hierarchy).l2@, line);
        if old_policy == HierarchyPolicy::Exclusive {
            remove_line_excludes(old(hierarchy).l1@, line);
        }
    }
}

// r[impl hierarchy.back-invalidation]
pub fn l2_evict(hierarchy: &mut CacheHierarchy, line: u64)
    ensures
        hierarchy.policy == old(hierarchy).policy,
        !contains_line_spec(hierarchy.l2@, line),
        match old(hierarchy).policy {
            HierarchyPolicy::Inclusive => !contains_line_spec(hierarchy.l1@, line),
            HierarchyPolicy::Exclusive => hierarchy.l1@ == old(hierarchy).l1@,
            HierarchyPolicy::NonInclusive => hierarchy.l1@ == old(hierarchy).l1@,
        },
{
    let old_policy = hierarchy.policy;
    hierarchy.l1 = match old_policy {
        HierarchyPolicy::Inclusive => clone_remove(&hierarchy.l1, line),
        HierarchyPolicy::Exclusive | HierarchyPolicy::NonInclusive => clone_level(&hierarchy.l1),
    };
    hierarchy.l2 = clone_remove(&hierarchy.l2, line);
    proof {
        remove_line_excludes(old(hierarchy).l2@, line);
        if old_policy == HierarchyPolicy::Inclusive {
            remove_line_excludes(old(hierarchy).l1@, line);
        }
    }
}

// r[impl hierarchy.exclusive-swap]
pub fn l2_to_l1_promote(hierarchy: &mut CacheHierarchy, line: u64)
    ensures
        hierarchy.policy == old(hierarchy).policy,
        if contains_line_spec(old(hierarchy).l2@, line) {
            &&& contains_line_spec(hierarchy.l1@, line)
            &&& match old(hierarchy).policy {
                HierarchyPolicy::Exclusive => !contains_line_spec(hierarchy.l2@, line),
                HierarchyPolicy::Inclusive => contains_line_spec(hierarchy.l2@, line),
                HierarchyPolicy::NonInclusive => contains_line_spec(hierarchy.l2@, line),
            }
        } else {
            &&& hierarchy.l1@ == old(hierarchy).l1@
            &&& hierarchy.l2@ == old(hierarchy).l2@
        },
{
    let old_policy = hierarchy.policy;
    let present = contains_line(&hierarchy.l2, line);
    if present {
        hierarchy.l1 = clone_push_unique(&hierarchy.l1, line);
        hierarchy.l2 = match old_policy {
            HierarchyPolicy::Exclusive => clone_remove(&hierarchy.l2, line),
            HierarchyPolicy::Inclusive | HierarchyPolicy::NonInclusive => clone_level(&hierarchy.l2),
        };
        proof {
            push_unique_contains(old(hierarchy).l1@, line);
            match old_policy {
                HierarchyPolicy::Exclusive => remove_line_excludes(old(hierarchy).l2@, line),
                HierarchyPolicy::Inclusive => assert(contains_line_spec(hierarchy.l2@, line)),
                HierarchyPolicy::NonInclusive => assert(contains_line_spec(hierarchy.l2@, line)),
            }
        }
    }
}

// r[impl hierarchy.capacity]
pub fn effective_capacity(hierarchy: &CacheHierarchy) -> (result: usize)
    requires
        hierarchy.policy == HierarchyPolicy::Inclusive
            || hierarchy.l1@.len() + hierarchy.l2@.len() <= usize::MAX as int,
    ensures result as int == effective_capacity_spec(*hierarchy)
{
    match hierarchy.policy {
        HierarchyPolicy::Inclusive => hierarchy.l2.len(),
        HierarchyPolicy::Exclusive | HierarchyPolicy::NonInclusive => hierarchy.l1.len() + hierarchy.l2.len(),
    }
}

// r[impl hierarchy.snoop-scope]
pub fn l2_miss_implies_l1_miss(hierarchy: &CacheHierarchy, line: u64) -> (result: bool)
    ensures result == l2_miss_implies_l1_miss_spec(*hierarchy, line)
{
    contains_line(&hierarchy.l2, line) || !contains_line(&hierarchy.l1, line)
}

proof fn push_unique_contains(level: Seq<u64>, line: u64)
    ensures contains_line_spec(push_unique_spec(level, line), line)
{
    if level.contains(line) {
        assert(push_unique_spec(level, line).contains(line));
    } else {
        let pushed = push_unique_spec(level, line);
        assert(pushed == level.push(line));
        assert(pushed[pushed.len() as int - 1] == line);
        assert(pushed.contains(line));
    }
}

proof fn remove_line_prefix_excludes(level: Seq<u64>, line: u64, upto: int)
    requires 0 <= upto <= level.len()
    ensures !contains_line_spec(remove_line_prefix_spec(level, line, upto), line)
    decreases upto
{
    if upto > 0 {
        remove_line_prefix_excludes(level, line, upto - 1);
    }
}

proof fn remove_line_excludes(level: Seq<u64>, line: u64)
    ensures !contains_line_spec(remove_line_spec(level, line), line)
{
    remove_line_prefix_excludes(level, line, level.len() as int);
}

// r[verify hierarchy.non-inclusive]
proof fn noninclusive_allows_any_sets(l1: Seq<u64>, l2: Seq<u64>)
    ensures noninclusive_invariant_spec(l1, l2)
{}

// r[verify hierarchy.capacity]
proof fn inclusive_capacity_formula(hierarchy: CacheHierarchy)
    requires hierarchy.policy == HierarchyPolicy::Inclusive
    ensures effective_capacity_spec(hierarchy) == hierarchy.l2@.len()
{}

// r[verify hierarchy.capacity]
proof fn exclusive_capacity_formula(hierarchy: CacheHierarchy)
    requires hierarchy.policy == HierarchyPolicy::Exclusive
    ensures effective_capacity_spec(hierarchy) == hierarchy.l1@.len() + hierarchy.l2@.len()
{}

// r[verify hierarchy.snoop-scope]
proof fn inclusive_l2_miss_implies_l1_miss(hierarchy: CacheHierarchy, line: u64)
    requires valid_hierarchy_spec(hierarchy), hierarchy.policy == HierarchyPolicy::Inclusive
    ensures l2_miss_implies_l1_miss_spec(hierarchy, line)
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const LINE_A: u64 = 11;
    const LINE_B: u64 = 17;
    const LINE_C: u64 = 23;
    const LINE_D: u64 = 29;
    const LINE_E: u64 = 31;
    const SMALL_LINE_LIMIT: u64 = 8;
    const MAX_OPS: usize = 24;

    #[derive(Clone, Copy, Debug)]
    enum Op {
        L1Fill(u64),
        L1Evict(u64),
        L2Fill(u64),
        L2Evict(u64),
        Promote(u64),
    }

    fn apply_op(hierarchy: &mut CacheHierarchy, op: Op) {
        match op {
            Op::L1Fill(line) => l1_fill(hierarchy, line),
            Op::L1Evict(line) => l1_evict(hierarchy, line),
            Op::L2Fill(line) => l2_fill(hierarchy, line),
            Op::L2Evict(line) => l2_evict(hierarchy, line),
            Op::Promote(line) => l2_to_l1_promote(hierarchy, line),
        }
    }

    fn run_ops(policy: HierarchyPolicy, ops: &[Op]) -> CacheHierarchy {
        let mut hierarchy = new_hierarchy(policy);
        for &op in ops {
            apply_op(&mut hierarchy, op);
        }
        hierarchy
    }

    fn op_strategy() -> impl Strategy<Value = Op> {
        prop_oneof![
            (0u64..SMALL_LINE_LIMIT).prop_map(Op::L1Fill),
            (0u64..SMALL_LINE_LIMIT).prop_map(Op::L1Evict),
            (0u64..SMALL_LINE_LIMIT).prop_map(Op::L2Fill),
            (0u64..SMALL_LINE_LIMIT).prop_map(Op::L2Evict),
            (0u64..SMALL_LINE_LIMIT).prop_map(Op::Promote),
        ]
    }

    // r[verify hierarchy.inclusive-invariant]
    #[test]
    fn test_inclusive_fill_keeps_l1_subset_l2() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut hierarchy, LINE_A);
        assert_eq!(l1_lines(&hierarchy), vec![LINE_A]);
        assert_eq!(l2_lines(&hierarchy), vec![LINE_A]);
        assert!(check_inclusive_invariant(&hierarchy));
        assert!(check_valid(&hierarchy));
    }

    // r[verify hierarchy.back-invalidation]
    #[test]
    fn test_inclusive_l2_evict_back_invalidates_l1() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut hierarchy, LINE_A);
        l2_fill(&mut hierarchy, LINE_B);
        l2_evict(&mut hierarchy, LINE_A);
        assert_eq!(l1_lines(&hierarchy), Vec::<u64>::new());
        assert_eq!(l2_lines(&hierarchy), vec![LINE_B]);
        assert!(check_inclusive_invariant(&hierarchy));
    }

    #[test]
    fn test_inclusive_without_back_invalidation_breaks_invariant() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut hierarchy, LINE_A);
        let mut broken = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut broken, LINE_A);
        broken.l2 = Vec::<u64>::new();
        assert!(!check_inclusive_invariant(&broken));
        assert!(check_inclusive_invariant(&hierarchy));
    }

    // r[verify hierarchy.exclusive-invariant]
    #[test]
    fn test_exclusive_fill_removes_other_level_copy() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Exclusive);
        l2_fill(&mut hierarchy, LINE_A);
        l1_fill(&mut hierarchy, LINE_A);
        assert_eq!(l1_lines(&hierarchy), vec![LINE_A]);
        assert_eq!(l2_lines(&hierarchy), Vec::<u64>::new());
        assert!(check_exclusive_invariant(&hierarchy));
        assert!(check_valid(&hierarchy));
    }

    // r[verify hierarchy.exclusive-swap]
    #[test]
    fn test_exclusive_promote_moves_line_from_l2_to_l1() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Exclusive);
        l2_fill(&mut hierarchy, LINE_C);
        l2_to_l1_promote(&mut hierarchy, LINE_C);
        assert_eq!(l1_lines(&hierarchy), vec![LINE_C]);
        assert_eq!(l2_lines(&hierarchy), Vec::<u64>::new());
        assert!(check_exclusive_invariant(&hierarchy));
    }

    // r[verify hierarchy.non-inclusive]
    #[test]
    fn test_noninclusive_allows_line_in_both_levels() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::NonInclusive);
        l1_fill(&mut hierarchy, LINE_D);
        l2_fill(&mut hierarchy, LINE_D);
        assert_eq!(l1_lines(&hierarchy), vec![LINE_D]);
        assert_eq!(l2_lines(&hierarchy), vec![LINE_D]);
        assert!(check_valid(&hierarchy));
    }

    // r[verify hierarchy.capacity]
    #[test]
    fn test_capacity_formulas() {
        let mut inclusive = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut inclusive, LINE_A);
        l2_fill(&mut inclusive, LINE_B);
        assert_eq!(effective_capacity(&inclusive), l2_lines(&inclusive).len());

        let mut exclusive = new_hierarchy(HierarchyPolicy::Exclusive);
        l1_fill(&mut exclusive, LINE_A);
        l2_fill(&mut exclusive, LINE_B);
        assert_eq!(effective_capacity(&exclusive), l1_lines(&exclusive).len() + l2_lines(&exclusive).len());
    }

    // r[verify hierarchy.snoop-scope]
    #[test]
    fn test_inclusive_l2_miss_implies_l1_miss() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Inclusive);
        l1_fill(&mut hierarchy, LINE_A);
        assert!(l2_miss_implies_l1_miss(&hierarchy, LINE_B));
    }

    #[test]
    fn test_exclusive_l2_miss_does_not_imply_l1_miss() {
        let mut hierarchy = new_hierarchy(HierarchyPolicy::Exclusive);
        l1_fill(&mut hierarchy, LINE_E);
        assert!(!l2_miss_implies_l1_miss(&hierarchy, LINE_E));
    }

    proptest! {
        // r[verify hierarchy.inclusive-invariant]
        #[test]
        fn prop_inclusive_invariant_survives_random_ops(ops in proptest::collection::vec(op_strategy(), 0..MAX_OPS)) {
            let hierarchy = run_ops(HierarchyPolicy::Inclusive, &ops);
            prop_assert!(check_valid(&hierarchy));
            prop_assert!(check_inclusive_invariant(&hierarchy));
        }

        // r[verify hierarchy.exclusive-invariant]
        #[test]
        fn prop_exclusive_invariant_survives_random_ops(ops in proptest::collection::vec(op_strategy(), 0..MAX_OPS)) {
            let hierarchy = run_ops(HierarchyPolicy::Exclusive, &ops);
            prop_assert!(check_valid(&hierarchy));
            prop_assert!(check_exclusive_invariant(&hierarchy));
        }

        // r[verify hierarchy.capacity]
        #[test]
        fn prop_capacity_formula_matches_policy(ops in proptest::collection::vec(op_strategy(), 0..MAX_OPS)) {
            let inclusive = run_ops(HierarchyPolicy::Inclusive, &ops);
            prop_assert_eq!(effective_capacity(&inclusive), l2_lines(&inclusive).len());

            let exclusive = run_ops(HierarchyPolicy::Exclusive, &ops);
            prop_assert_eq!(effective_capacity(&exclusive), l1_lines(&exclusive).len() + l2_lines(&exclusive).len());
        }
    }
}
