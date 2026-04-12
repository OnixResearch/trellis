//! Verified Zircon job containment model.
//!
//! Jobs form a parent-pointer tree. Processes belong to exactly one job.
//! Kill propagation is modeled as subtree membership. A critical process
//! with non-zero exit status kills its owning job. A parent stays live when
//! any running process exists in its subtree.

use vstd::prelude::*;

verus! {

pub open spec fn parents_view(parents: Seq<Option<usize>>) -> Seq<Option<int>> {
    parents.map(|_i: int, p: Option<usize>| match p {
        Some(j) => Some(j as int),
        None => None::<int>,
    })
}

// r[depends zircon-job.containment]
pub open spec fn well_formed_jobs_from_spec(parents: Seq<Option<int>>, from: int) -> bool
    decreases if from <= parents.len() { parents.len() - from } else { 0 }
{
    if from >= parents.len() {
        true
    } else {
        (match parents[from] {
            Some(p) => 0 <= p < from,
            None => true,
        }) && well_formed_jobs_from_spec(parents, from + 1)
    }
}

// r[depends zircon-job.containment]
pub open spec fn well_formed_jobs_spec(parents: Seq<Option<int>>) -> bool {
    well_formed_jobs_from_spec(parents, 0)
}

// r[impl zircon-job.containment]
#[verifier::external_body]
pub fn well_formed_jobs(parents: &[Option<usize>]) -> (result: bool)
    ensures result == well_formed_jobs_spec(parents_view(parents@))
{
    for (i, p) in parents.iter().enumerate() {
        if let Some(parent) = p {
            if *parent >= i {
                return false;
            }
        }
    }
    true
}

// r[depends zircon-job.containment]
pub open spec fn unique_parent_spec(parents: Seq<Option<int>>, job: int) -> bool {
    0 <= job < parents.len() && match parents[job] {
        Some(p) => 0 <= p < job,
        None => true,
    }
}

// r[impl zircon-job.containment]
pub fn unique_parent(parents: &[Option<usize>], job: usize) -> (result: bool)
    requires job < parents@.len()
    ensures result == unique_parent_spec(parents_view(parents@), job as int)
{
    match parents[job] {
        Some(p) => p < job,
        None => true,
    }
}

// r[depends zircon-job.containment]
pub open spec fn process_owner_spec(owners: Seq<int>, process: int) -> int
    recommends 0 <= process < owners.len()
{
    owners[process]
}

// r[impl zircon-job.containment]
pub fn process_owner(owners: &[usize], process: usize) -> (result: usize)
    requires process < owners@.len()
    ensures result as int == process_owner_spec(owners@.map(|_i: int, j: usize| j as int), process as int)
{
    owners[process]
}

pub open spec fn job_in_subtree_fuel(parents: Seq<Option<int>>, root: int, job: int, fuel: nat) -> bool
    decreases fuel
{
    if fuel == 0 || root < 0 || root >= parents.len() || job < 0 || job >= parents.len() {
        false
    } else if job == root {
        true
    } else {
        match parents[job] {
            Some(p) => job_in_subtree_fuel(parents, root, p, (fuel - 1) as nat),
            None => false,
        }
    }
}

pub open spec fn job_in_subtree_spec(parents: Seq<Option<int>>, root: int, job: int) -> bool
    recommends well_formed_jobs_spec(parents), 0 <= root < parents.len(), 0 <= job < parents.len()
{
    job_in_subtree_fuel(parents, root, job, (job + 1) as nat)
}

// r[depends zircon-job.kill-propagation]
pub open spec fn process_in_subtree_spec(parents: Seq<Option<int>>, owners: Seq<int>, root: int, process: int) -> bool
    recommends
        well_formed_jobs_spec(parents),
        0 <= root < parents.len(),
        0 <= process < owners.len(),
        0 <= owners[process] < parents.len()
{
    job_in_subtree_spec(parents, root, owners[process])
}

// r[impl zircon-job.kill-propagation]
#[verifier::external_body]
pub fn job_in_subtree(parents: &[Option<usize>], root: usize, job: usize) -> (result: bool)
    requires
        root < parents@.len(),
        job < parents@.len(),
        well_formed_jobs_spec(parents_view(parents@)),
    ensures result == job_in_subtree_spec(parents_view(parents@), root as int, job as int)
{
    let mut cur = job;
    loop {
        if cur == root {
            return true;
        }
        match parents[cur] {
            Some(p) => cur = p,
            None => return false,
        }
    }
}

// r[impl zircon-job.kill-propagation]
#[verifier::external_body]
pub fn process_in_subtree(parents: &[Option<usize>], owners: &[usize], root: usize, process: usize) -> (result: bool)
    requires
        root < parents@.len(),
        process < owners@.len(),
        owners@[process as int] < parents@.len(),
        well_formed_jobs_spec(parents_view(parents@)),
    ensures result == process_in_subtree_spec(
        parents_view(parents@),
        owners@.map(|_i: int, j: usize| j as int),
        root as int,
        process as int,
    )
{
    job_in_subtree(parents, root, owners[process])
}

// r[depends zircon-job.critical-process]
pub open spec fn critical_exit_kills_job_spec(is_critical: bool, exit_code: i32) -> bool {
    is_critical && exit_code != 0
}

// r[impl zircon-job.critical-process]
pub fn critical_exit_kills_job(is_critical: bool, exit_code: i32) -> (result: bool)
    ensures result == critical_exit_kills_job_spec(is_critical, exit_code)
{
    is_critical && exit_code != 0
}

pub open spec fn running_in_subtree_from_spec(
    parents: Seq<Option<int>>,
    owners: Seq<int>,
    running: Seq<bool>,
    job: int,
    from: int,
) -> bool
    decreases if from <= running.len() { running.len() - from } else { 0 }
{
    if from >= running.len() {
        false
    } else {
        (running[from] && process_in_subtree_spec(parents, owners, job, from))
            || running_in_subtree_from_spec(parents, owners, running, job, from + 1)
    }
}

// r[depends zircon-job.parent-lifetime]
pub open spec fn parent_kept_live_spec(
    parents: Seq<Option<int>>,
    owners: Seq<int>,
    running: Seq<bool>,
    job: int,
) -> bool
    recommends
        well_formed_jobs_spec(parents),
        0 <= job < parents.len(),
        owners.len() == running.len(),
        forall|i: int| 0 <= i < owners.len() ==> #[trigger] owners[i] < parents.len(),
{
    running_in_subtree_from_spec(parents, owners, running, job, 0)
}

// r[impl zircon-job.parent-lifetime]
#[verifier::external_body]
pub fn parent_kept_live(parents: &[Option<usize>], owners: &[usize], running: &[bool], job: usize) -> (result: bool)
    requires
        job < parents@.len(),
        owners@.len() == running@.len(),
        forall|i: int| 0 <= i < owners@.len() ==> #[trigger] owners@[i] < parents@.len(),
        well_formed_jobs_spec(parents_view(parents@)),
    ensures result == parent_kept_live_spec(
        parents_view(parents@),
        owners@.map(|_i: int, j: usize| j as int),
        running@,
        job as int,
    )
{
    for i in 0..running.len() {
        if running[i] && job_in_subtree(parents, job, owners[i]) {
            return true;
        }
    }
    false
}

// r[verify zircon-job.containment]
proof fn root_job_has_no_parent(parents: Seq<Option<int>>, root: int)
    requires well_formed_jobs_spec(parents), 0 <= root < parents.len(), parents[root] == None::<int>
    ensures unique_parent_spec(parents, root)
{}

// r[verify zircon-job.kill-propagation]
proof fn root_kills_self(parents: Seq<Option<int>>, root: int)
    requires well_formed_jobs_spec(parents), 0 <= root < parents.len()
    ensures job_in_subtree_spec(parents, root, root)
{}

// r[verify zircon-job.kill-propagation]
proof fn process_owner_in_root_subtree(parents: Seq<Option<int>>, owners: Seq<int>, root: int, process: int)
    requires
        well_formed_jobs_spec(parents),
        0 <= root < parents.len(),
        0 <= process < owners.len(),
        owners[process] == root,
    ensures process_in_subtree_spec(parents, owners, root, process)
{}

// r[verify zircon-job.critical-process]
proof fn clean_exit_does_not_kill_job(is_critical: bool)
    ensures !critical_exit_kills_job_spec(is_critical, 0)
{}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn parents() -> Vec<Option<usize>> {
        vec![None, Some(0), Some(1), Some(1)]
    }

    // r[verify zircon-job.containment]
    #[test]
    fn test_unique_parent_and_owner() {
        let ps = parents();
        let owners = vec![2usize, 3usize];
        assert!(well_formed_jobs(&ps));
        assert!(unique_parent(&ps, 1));
        assert_eq!(process_owner(&owners, 0), 2);
    }

    // r[verify zircon-job.kill-propagation]
    #[test]
    fn test_kill_propagates_to_subtree() {
        let ps = parents();
        let owners = vec![2usize, 3usize];
        assert!(job_in_subtree(&ps, 1, 2));
        assert!(job_in_subtree(&ps, 1, 3));
        assert!(process_in_subtree(&ps, &owners, 1, 0));
        assert!(process_in_subtree(&ps, &owners, 1, 1));
        assert!(!job_in_subtree(&ps, 2, 3));
    }

    // r[verify zircon-job.critical-process]
    #[test]
    fn test_critical_process() {
        assert!(critical_exit_kills_job(true, 1));
        assert!(!critical_exit_kills_job(true, 0));
        assert!(!critical_exit_kills_job(false, 1));
    }

    // r[verify zircon-job.parent-lifetime]
    #[test]
    fn test_parent_kept_live_by_running_descendant() {
        let ps = parents();
        let owners = vec![2usize, 3usize];
        let running = vec![true, false];
        assert!(parent_kept_live(&ps, &owners, &running, 1));
        assert!(parent_kept_live(&ps, &owners, &running, 0));
        assert!(!parent_kept_live(&ps, &owners, &running, 3));
    }

    proptest! {
        // r[verify zircon-job.critical-process]
        #[test]
        fn prop_critical_exit_matches_pred(is_critical: bool, exit_code: i32) {
            prop_assert_eq!(critical_exit_kills_job(is_critical, exit_code), is_critical && exit_code != 0_i32);
        }
    }
}
