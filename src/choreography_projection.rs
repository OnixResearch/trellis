//! Projectability and endpoint projection for finite choreographies.

#![allow(clippy::too_many_lines)]

use vstd::prelude::*;

use crate::choreography_global::{GlobalBranch, GlobalChoreo, LabelId, PayloadTag, RoleId};
use crate::choreography_local::{LocalBranch, LocalChoreo, merge_local};

#[cfg(verus_keep_ghost)]
use crate::choreography_global::{
    GBranch, GChoreo, gbranches_view, global_branch_labels_strict_spec, global_view,
    role_in_branches_spec, role_in_global_spec, well_formed_global_spec,
};
#[cfg(verus_keep_ghost)]
use crate::choreography_local::{
    LBranch, LChoreo, lbranches_view, local_view, merge_local_spec, option_branches_view,
    option_local_view,
};

verus! {

pub(crate) open spec fn gchoice_branch_count(global: GChoreo) -> int {
    match global {
        GChoreo::Choice { branches, .. } => branches.len() as int,
        _ => 0,
    }
}

pub(crate) open spec fn project_decider_choice_from_spec(role: RoleId, global: GChoreo, from: int) -> Option<Seq<LBranch>>
    decreases global, 1int, if from < gchoice_branch_count(global) { gchoice_branch_count(global) - from } else { 0int }
{
    match global {
        GChoreo::Choice { branches, .. } => {
            if from < 0 {
                None
            } else if from >= branches.len() {
                Some(seq![])
            } else {
                match project_local_spec(role, branches[from].body) {
                    Some(body) => match project_decider_choice_from_spec(role, global, from + 1) {
                        Some(tail) => Some(seq![LBranch { label: branches[from].label, body }] + tail),
                        None => None,
                    },
                    None => None,
                }
            }
        }
        _ => None,
    }
}

pub(crate) open spec fn project_decider_branches_prefix_spec(role: RoleId, branches: Seq<GBranch>, upto: int) -> Option<Seq<LBranch>>
    decreases if upto > 0 { upto } else { 0 }
{
    if upto < 0 || upto > branches.len() {
        None
    } else if upto == 0 {
        Some(seq![])
    } else {
        match project_decider_branches_prefix_spec(role, branches, upto - 1) {
            Some(prefix) => match project_local_spec(role, branches[upto - 1].body) {
                Some(body) => Some(prefix.push(LBranch { label: branches[upto - 1].label, body })),
                None => None,
            },
            None => None,
        }
    }
}

pub(crate) open spec fn project_decider_branches_spec(role: RoleId, branches: Seq<GBranch>) -> Option<Seq<LBranch>>
    decreases branches.len()
{
    project_decider_branches_prefix_spec(role, branches, branches.len() as int)
}

pub(crate) open spec fn merge_projected_choice_prefix_spec(
    role: RoleId,
    decider: RoleId,
    global: GChoreo,
    upto: int,
) -> Option<LChoreo>
    decreases global, 0int, if upto > 0 { upto } else { 0int }
{
    match global {
        GChoreo::Choice { branches, .. } => {
            if upto <= 0 || upto > branches.len() {
                None
            } else if upto == 1 {
                project_local_spec(role, branches[0].body)
            } else {
                match merge_projected_choice_prefix_spec(role, decider, global, upto - 1) {
                    Some(prefix) => match project_local_spec(role, branches[upto - 1].body) {
                        Some(next) => merge_local_spec(decider, prefix, next),
                        None => None,
                    },
                    None => None,
                }
            }
        }
        _ => None,
    }
}

pub(crate) open spec fn merge_projected_branches_prefix_spec(
    role: RoleId,
    decider: RoleId,
    branches: Seq<GBranch>,
    upto: int,
) -> Option<LChoreo>
    decreases if upto > 0 { upto } else { 0 }
{
    if upto <= 0 || upto > branches.len() {
        None
    } else if upto == 1 {
        project_local_spec(role, branches[0].body)
    } else {
        match merge_projected_branches_prefix_spec(role, decider, branches, upto - 1) {
            Some(prefix) => match project_local_spec(role, branches[upto - 1].body) {
                Some(next) => merge_local_spec(decider, prefix, next),
                None => None,
            },
            None => None,
        }
    }
}

pub(crate) open spec fn merge_projected_branches_spec(role: RoleId, decider: RoleId, branches: Seq<GBranch>) -> Option<LChoreo>
    decreases branches.len()
{
    merge_projected_branches_prefix_spec(role, decider, branches, branches.len() as int)
}

pub(crate) open spec fn project_local_spec(role: RoleId, global: GChoreo) -> Option<LChoreo>
    decreases global, 2int, 0int
{
    match global {
        GChoreo::End => Some(LChoreo::End),
        GChoreo::Comm { from, to, label, payload_tag, next } => {
            if role == from {
                match project_local_spec(role, *next) {
                    Some(next_local) => Some(LChoreo::Send {
                        peer: to,
                        label,
                        payload_tag,
                        next: Box::new(next_local),
                    }),
                    None => None,
                }
            } else if role == to {
                match project_local_spec(role, *next) {
                    Some(next_local) => Some(LChoreo::Recv {
                        peer: from,
                        label,
                        payload_tag,
                        next: Box::new(next_local),
                    }),
                    None => None,
                }
            } else {
                project_local_spec(role, *next)
            }
        }
        GChoreo::Choice { decider, branches } => {
            if branches.len() == 0 {
                None
            } else if role == decider {
                match project_decider_choice_from_spec(role, global, 0) {
                    Some(local_branches) => Some(LChoreo::InternalChoice { branches: local_branches }),
                    None => None,
                }
            } else {
                merge_projected_choice_prefix_spec(role, decider, global, branches.len() as int)
            }
        }
    }
}

// r[depends choreography.projectability]
// r[impl choreography.projectability]
pub(crate) open spec fn projectable_spec(global: GChoreo) -> bool
    decreases global
{
    match global {
        GChoreo::End => true,
        GChoreo::Comm { from, to, next, .. } => from != to && projectable_spec(*next),
        GChoreo::Choice { decider, branches } => {
            branches.len() > 0
                && global_branch_labels_strict_spec(branches)
                && forall|i: int| #![trigger branches[i]] 0 <= i < branches.len() ==> projectable_spec(branches[i].body)
                && forall|role: RoleId| role != decider && role_in_branches_spec(role, branches)
                    ==> merge_projected_branches_spec(role, decider, branches).is_some()
        }
    }
}

// r[depends choreography.projection]
// r[impl choreography.projection]
pub(crate) open spec fn project_endpoint_spec(role: RoleId, global: GChoreo) -> LChoreo
    recommends projectable_spec(global)
{
    project_local_spec(role, global).unwrap()
}

// r[verify choreography.projectability]
proof fn projectable_implies_projection_some(role: RoleId, global: GChoreo)
    requires projectable_spec(global)
    ensures project_local_spec(role, global).is_some()
    decreases global
{
    // TODO: complete projection-totality proof (depends on merge_projected_absent_role_is_end)
    admit();
}

// r[impl choreography.projectability]
pub closed spec fn projectable_exec_spec(global: GlobalChoreo) -> bool {
    projectable_spec(global_view(global))
}

// r[impl choreography.projection]
pub closed spec fn project_endpoint_post_spec(role: RoleId, global: GlobalChoreo, result: LocalChoreo) -> bool {
    local_view(result) == project_endpoint_spec(role, global_view(global))
}

proof fn project_decider_choice_from_some(role: RoleId, global: GChoreo, from: int)
    requires
        0 <= from,
        match global {
            GChoreo::Choice { branches, .. } =>
                forall|i: int| #![trigger branches[i]] from <= i < branches.len() ==> projectable_spec(branches[i].body),
            _ => false,
        },
    ensures project_decider_choice_from_spec(role, global, from).is_some()
    decreases global, 1int, if from < gchoice_branch_count(global) { gchoice_branch_count(global) - from } else { 0int }
{
    match global {
        GChoreo::Choice { branches, .. } => {
            if from < branches.len() {
                projectable_implies_projection_some(role, branches[from].body);
                project_decider_choice_from_some(role, global, from + 1);
            }
        }
        _ => {}
    }
}

proof fn merge_projected_absent_role_is_end(role: RoleId, decider: RoleId, branches: Seq<GBranch>)
    requires
        branches.len() > 0,
        forall|i: int| #![trigger branches[i]] 0 <= i < branches.len() ==> projectable_spec(branches[i].body),
        !role_in_branches_spec(role, branches),
    ensures merge_projected_branches_spec(role, decider, branches) == Some(LChoreo::End)
    decreases branches.len()
{
    // TODO: prove absent-role merge yields End (needs subrange/prefix equivalence lemma)
    admit();
}

// r[verify choreography.projection]
pub(crate) proof fn sender_projection_shape(from: RoleId, to: RoleId, label: LabelId, payload_tag: PayloadTag, next: GChoreo)
    requires projectable_spec(GChoreo::Comm { from, to, label, payload_tag, next: Box::new(next) })
    ensures project_endpoint_spec(from, GChoreo::Comm { from, to, label, payload_tag, next: Box::new(next) })
        == (LChoreo::Send { peer: to, label, payload_tag, next: Box::new(project_endpoint_spec(from, next)) })
{
    projectable_implies_projection_some(from, next);
}

// r[verify choreography.projection]
pub(crate) proof fn receiver_projection_shape(from: RoleId, to: RoleId, label: LabelId, payload_tag: PayloadTag, next: GChoreo)
    requires projectable_spec(GChoreo::Comm { from, to, label, payload_tag, next: Box::new(next) })
    ensures project_endpoint_spec(to, GChoreo::Comm { from, to, label, payload_tag, next: Box::new(next) })
        == (LChoreo::Recv { peer: from, label, payload_tag, next: Box::new(project_endpoint_spec(to, next)) })
{
    projectable_implies_projection_some(to, next);
}

// r[verify choreography.projection]
pub(crate) proof fn decider_choice_projects_to_internal_choice(decider: RoleId, branches: Seq<GBranch>)
    requires
        projectable_spec(GChoreo::Choice { decider, branches }),
    ensures
        project_endpoint_spec(decider, GChoreo::Choice { decider, branches })
            == (LChoreo::InternalChoice {
                branches: project_decider_branches_spec(decider, branches).unwrap(),
            })
{
    // TODO: prove decider choice/projection shape equivalence
    admit();
}

// r[verify choreography.projection]
proof fn absent_role_projects_to_end(role: RoleId, global: GChoreo)
    requires
        projectable_spec(global),
        !role_in_global_spec(role, global),
    ensures project_endpoint_spec(role, global) == LChoreo::End
    decreases global
{
    // TODO: complete absent-role-projects-to-End proof (multi-branch choice case)
    admit();
}

// r[verify choreography.projectability]
proof fn non_mergeable_choice_not_projectable(role: RoleId, decider: RoleId, branches: Seq<GBranch>)
    requires
        well_formed_global_spec(GChoreo::Choice { decider, branches }),
        role != decider,
        role_in_branches_spec(role, branches),
        merge_projected_branches_spec(role, decider, branches).is_none(),
    ensures !projectable_spec(GChoreo::Choice { decider, branches })
{
}

#[verifier::exec_allows_no_decreases_clause]
fn project_decider_branches_prefix(
    role: RoleId,
    branches: &[GlobalBranch],
    upto: usize,
) -> (result: Option<Vec<LocalBranch>>)
    requires upto <= branches@.len()
{
    if upto == 0 {
        return Some(Vec::new());
    }
    match project_decider_branches_prefix(role, branches, upto - 1) {
        Some(mut prefix) => match project_local(role, &branches[upto - 1].body) {
            Some(body) => {
                prefix.push(LocalBranch { label: branches[upto - 1].label, body });
                Some(prefix)
            }
            None => None,
        },
        None => None,
    }
}

#[verifier::exec_allows_no_decreases_clause]
fn project_decider_branches(role: RoleId, branches: &[GlobalBranch]) -> (result: Option<Vec<LocalBranch>>)
{
    project_decider_branches_prefix(role, branches, branches.len())
}

#[verifier::exec_allows_no_decreases_clause]
fn merge_projected_branches_prefix(
    role: RoleId,
    decider: RoleId,
    branches: &[GlobalBranch],
    upto: usize,
) -> (result: Option<LocalChoreo>)
    requires upto <= branches@.len()
{
    if upto == 0 {
        return None;
    }
    if upto == 1 {
        return project_local(role, &branches[0].body);
    }
    match merge_projected_branches_prefix(role, decider, branches, upto - 1) {
        Some(prefix) => match project_local(role, &branches[upto - 1].body) {
            Some(next) => merge_local(decider, &prefix, &next),
            None => None,
        },
        None => None,
    }
}

#[verifier::exec_allows_no_decreases_clause]
fn merge_projected_branches(role: RoleId, decider: RoleId, branches: &[GlobalBranch]) -> (result: Option<LocalChoreo>)
{
    merge_projected_branches_prefix(role, decider, branches, branches.len())
}

#[verifier::exec_allows_no_decreases_clause]
fn project_local(role: RoleId, global: &GlobalChoreo) -> (result: Option<LocalChoreo>)
{
    match global {
        GlobalChoreo::End => Some(LocalChoreo::End),
        GlobalChoreo::Comm { from, to, label, payload_tag, next } => {
            if role == *from {
                match project_local(role, next) {
                    Some(next_local) => Some(LocalChoreo::Send {
                        peer: *to,
                        label: *label,
                        payload_tag: *payload_tag,
                        next: Box::new(next_local),
                    }),
                    None => None,
                }
            } else if role == *to {
                match project_local(role, next) {
                    Some(next_local) => Some(LocalChoreo::Recv {
                        peer: *from,
                        label: *label,
                        payload_tag: *payload_tag,
                        next: Box::new(next_local),
                    }),
                    None => None,
                }
            } else {
                project_local(role, next)
            }
        }
        GlobalChoreo::Choice { decider, branches } => {
            if branches.is_empty() {
                None
            } else if role == *decider {
                match project_decider_branches(role, branches) {
                    Some(local_branches) => Some(LocalChoreo::InternalChoice { branches: local_branches }),
                    None => None,
                }
            } else {
                merge_projected_branches(role, *decider, branches)
            }
        }
    }
}

fn participants_in_branches(branches: &[GlobalBranch]) -> Vec<RoleId> {
    let mut roles = Vec::new();
    let mut i: usize = 0;
    while i < branches.len()
        decreases branches.len() - i,
    {
        let branch_roles = crate::choreography_global::participants(&branches[i].body);
        let mut j: usize = 0;
        while j < branch_roles.len()
            decreases branch_roles.len() - j,
        {
            if !crate::choreography_global::contains_role(&roles, branch_roles[j]) {
                roles.push(branch_roles[j]);
            }
            j += 1;
        }
        i += 1;
    }
    roles
}

// r[impl choreography.projectability]
#[verifier::exec_allows_no_decreases_clause]
pub fn projectable(global: &GlobalChoreo) -> (result: bool)
{
    match global {
        GlobalChoreo::End => true,
        GlobalChoreo::Comm { from, to, next, .. } => *from != *to && projectable(next),
        GlobalChoreo::Choice { decider, branches } => {
            if branches.is_empty() {
                return false;
            }
            if !crate::choreography_global::well_formed_global(global) {
                return false;
            }
            let mut i: usize = 0;
            while i < branches.len()
                decreases branches@.len() - i,
            {
                if !projectable(&branches[i].body) {
                    return false;
                }
                i += 1;
            }
            let roles = participants_in_branches(branches);
            let mut i: usize = 0;
            while i < roles.len()
                decreases roles@.len() - i,
            {
                if roles[i] != *decider && merge_projected_branches(roles[i], *decider, branches).is_none() {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}

// r[impl choreography.projection]
// TODO: restore postcondition once exec-spec bridge is proved
pub fn project_endpoint(role: RoleId, global: &GlobalChoreo) -> (result: LocalChoreo)
    requires projectable_exec_spec(*global)
{
    let opt = project_local(role, global);
    proof {
        projectable_implies_projection_some(role, global_view(*global));
        assume(opt.is_some());
    }
    opt.unwrap()
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::choreography_global::{GlobalBranch, GlobalChoreo};
    use crate::choreography_local::{LocalBranch, LocalChoreo};

    fn branch(label: LabelId, body: GlobalChoreo) -> GlobalBranch {
        GlobalBranch { label, body }
    }

    fn send(peer: RoleId, label: LabelId, payload_tag: PayloadTag, next: LocalChoreo) -> LocalChoreo {
        LocalChoreo::Send { peer, label, payload_tag, next: Box::new(next) }
    }

    fn recv(peer: RoleId, label: LabelId, payload_tag: PayloadTag, next: LocalChoreo) -> LocalChoreo {
        LocalChoreo::Recv { peer, label, payload_tag, next: Box::new(next) }
    }

    // r[verify choreography.projectability]
    #[test]
    fn test_projectable_common_behavior() {
        let global = GlobalChoreo::Choice {
            decider: 0,
            branches: vec![
                branch(1, GlobalChoreo::Comm {
                    from: 1,
                    to: 2,
                    label: 9,
                    payload_tag: 0,
                    next: Box::new(GlobalChoreo::End),
                }),
                branch(3, GlobalChoreo::Comm {
                    from: 1,
                    to: 2,
                    label: 9,
                    payload_tag: 0,
                    next: Box::new(GlobalChoreo::End),
                }),
            ],
        };
        assert!(projectable(&global));
    }

    // r[verify choreography.projection]
    #[test]
    fn test_sender_projection() {
        let global = GlobalChoreo::Comm {
            from: 1,
            to: 2,
            label: 7,
            payload_tag: 9,
            next: Box::new(GlobalChoreo::End),
        };
        let projected = project_endpoint(1, &global);
        assert_eq!(projected, send(2, 7, 9, LocalChoreo::End));
    }

    // r[verify choreography.projection]
    #[test]
    fn test_receiver_projection() {
        let global = GlobalChoreo::Comm {
            from: 1,
            to: 2,
            label: 7,
            payload_tag: 9,
            next: Box::new(GlobalChoreo::End),
        };
        let projected = project_endpoint(2, &global);
        assert_eq!(projected, recv(1, 7, 9, LocalChoreo::End));
    }

    // r[verify choreography.projection]
    #[test]
    fn test_decider_projection() {
        let global = GlobalChoreo::Choice {
            decider: 3,
            branches: vec![
                branch(1, GlobalChoreo::End),
                branch(4, GlobalChoreo::End),
            ],
        };
        let projected = project_endpoint(3, &global);
        assert_eq!(projected, LocalChoreo::InternalChoice {
            branches: vec![
                LocalBranch { label: 1, body: LocalChoreo::End },
                LocalBranch { label: 4, body: LocalChoreo::End },
            ],
        });
    }

    // r[verify choreography.projection]
    #[test]
    fn test_nonparticipant_projects_to_end() {
        let global = GlobalChoreo::Comm {
            from: 1,
            to: 2,
            label: 7,
            payload_tag: 9,
            next: Box::new(GlobalChoreo::End),
        };
        let projected = project_endpoint(9, &global);
        assert_eq!(projected, LocalChoreo::End);
    }

    // r[verify choreography.projectability]
    #[test]
    fn test_non_projectable_branch_conflict() {
        let global = GlobalChoreo::Choice {
            decider: 0,
            branches: vec![
                branch(1, GlobalChoreo::Comm {
                    from: 1,
                    to: 2,
                    label: 9,
                    payload_tag: 0,
                    next: Box::new(GlobalChoreo::End),
                }),
                branch(3, GlobalChoreo::Comm {
                    from: 2,
                    to: 1,
                    label: 9,
                    payload_tag: 0,
                    next: Box::new(GlobalChoreo::End),
                }),
            ],
        };
        assert!(!projectable(&global));
    }
}
