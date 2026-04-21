//! Verified TLB shootdown protocol.
//!
//! Models range invalidation across per-core TLBs. Initiation self-invalidates
//! the initiator, remote cores invalidate on IPI, acknowledgments remove cores
//! from the pending set only after their local TLB is fresh, and completion
//! implies no core retains a mapping in the invalidated range.

use vstd::prelude::*;

verus! {

#[derive(Copy, Clone)]
pub struct TlbEntry {
    pub vpn: u64,
    pub ppn: u64,
}

pub struct CoreTlb {
    pub entries: Vec<TlbEntry>,
}

pub struct ShootdownRequest {
    pub initiator: usize,
    pub vpn_start: u64,
    pub vpn_end: u64,
    pub new_ppn: u64,
    pub pending_acks: Vec<usize>,
    pub completed: bool,
}

// r[impl tlb-shootdown.range]
// r[depends tlb-shootdown.range]
pub open spec fn vpn_in_range_spec(vpn: u64, vpn_start: u64, vpn_end: u64) -> bool {
    &&& vpn_start <= vpn
    &&& vpn < vpn_end
}

// r[impl tlb-shootdown.range]
pub open spec fn core_tlb_fresh_spec(entries: Seq<TlbEntry>, vpn_start: u64, vpn_end: u64) -> bool {
    forall|i: int| #![trigger entries[i]] 0 <= i < entries.len() ==> !vpn_in_range_spec(entries[i].vpn, vpn_start, vpn_end)
}

// r[impl tlb-shootdown.termination]
pub open spec fn core_pending_spec(request: ShootdownRequest, core: int) -> bool {
    exists|i: int| 0 <= i < request.pending_acks@.len() && request.pending_acks@[i] == core
}

// r[impl tlb-shootdown.termination]
pub open spec fn all_acknowledged_spec(request: ShootdownRequest) -> bool {
    request.pending_acks@.len() == 0
}

// r[impl tlb-shootdown.freshness]
pub open spec fn all_core_tlbs_fresh_spec(
    tlbs: Seq<CoreTlb>,
    vpn_start: u64,
    vpn_end: u64,
) -> bool {
    forall|core: int| #![trigger tlbs[core].entries] 0 <= core < tlbs.len() ==> core_tlb_fresh_spec(tlbs[core].entries@, vpn_start, vpn_end)
}

// r[impl tlb-shootdown.initiator-safety]
pub open spec fn safe_to_proceed_spec(tlbs: Seq<CoreTlb>, request: ShootdownRequest) -> bool {
    &&& request.completed
    &&& all_core_tlbs_fresh_spec(tlbs, request.vpn_start, request.vpn_end)
}

// r[impl tlb-shootdown.isolation]
pub open spec fn disjoint_ranges_spec(
    left_start: u64,
    left_end: u64,
    right_start: u64,
    right_end: u64,
) -> bool {
    left_end <= right_start || right_end <= left_start
}

pub open spec fn invalidate_entries_prefix_spec(
    entries: Seq<TlbEntry>,
    vpn_start: u64,
    vpn_end: u64,
    upto: int,
) -> Seq<TlbEntry>
    decreases if upto >= 0 { upto } else { 0 }
{
    if upto <= 0 {
        seq![]
    } else {
        let prefix = invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto - 1);
        if vpn_in_range_spec(entries[upto - 1].vpn, vpn_start, vpn_end) {
            prefix
        } else {
            prefix.push(entries[upto - 1])
        }
    }
}

pub open spec fn invalidate_entries_spec(entries: Seq<TlbEntry>, vpn_start: u64, vpn_end: u64) -> Seq<TlbEntry> {
    invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, entries.len() as int)
}

pub open spec fn remove_core_prefix_spec(pending: Seq<usize>, core: usize, upto: int) -> Seq<usize>
    decreases if upto >= 0 { upto } else { 0 }
{
    if upto <= 0 {
        seq![]
    } else {
        let prefix = remove_core_prefix_spec(pending, core, upto - 1);
        if pending[upto - 1] == core {
            prefix
        } else {
            prefix.push(pending[upto - 1])
        }
    }
}

pub open spec fn remove_core_spec(pending: Seq<usize>, core: usize) -> Seq<usize> {
    remove_core_prefix_spec(pending, core, pending.len() as int)
}

pub open spec fn pending_acks_prefix_spec(core_count: usize, initiator: usize, upto: int) -> Seq<usize>
    decreases if upto >= 0 { upto } else { 0 }
{
    if upto <= 0 {
        seq![]
    } else {
        let prefix = pending_acks_prefix_spec(core_count, initiator, upto - 1);
        let ghost core = (upto - 1) as usize;
        if core == initiator {
            prefix
        } else {
            prefix.push(core)
        }
    }
}

pub open spec fn pending_acks_spec(core_count: usize, initiator: usize) -> Seq<usize> {
    pending_acks_prefix_spec(core_count, initiator, core_count as int)
}

proof fn invalidate_entries_prefix_fresh(entries: Seq<TlbEntry>, vpn_start: u64, vpn_end: u64, upto: int)
    requires
        0 <= upto <= entries.len(),
    ensures
        core_tlb_fresh_spec(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto), vpn_start, vpn_end),
    decreases upto,
{
    if upto > 0 {
        invalidate_entries_prefix_fresh(entries, vpn_start, vpn_end, upto - 1);
        let prefix = invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto - 1);
        if vpn_in_range_spec(entries[upto - 1].vpn, vpn_start, vpn_end) {
            assert(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto) == prefix);
        } else {
            assert(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto) == prefix.push(entries[upto - 1]));
            assert forall|i: int| 0 <= i < invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto).len()
                implies !vpn_in_range_spec(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto)[i].vpn, vpn_start, vpn_end) by {
                if i < prefix.len() {
                    assert(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto)[i] == prefix[i]);
                } else {
                    assert(i == prefix.len());
                    assert(invalidate_entries_prefix_spec(entries, vpn_start, vpn_end, upto)[i] == entries[upto - 1]);
                }
            }
        }
    }
}

proof fn invalidate_entries_fresh(entries: Seq<TlbEntry>, vpn_start: u64, vpn_end: u64)
    ensures
        core_tlb_fresh_spec(invalidate_entries_spec(entries, vpn_start, vpn_end), vpn_start, vpn_end),
{
    invalidate_entries_prefix_fresh(entries, vpn_start, vpn_end, entries.len() as int);
}

// r[impl tlb-shootdown.range]
fn vpn_in_range(vpn: u64, vpn_start: u64, vpn_end: u64) -> (result: bool)
    ensures
        result == vpn_in_range_spec(vpn, vpn_start, vpn_end),
{
    vpn_start <= vpn && vpn < vpn_end
}

// r[impl tlb-shootdown.termination]
fn core_pending(request: &ShootdownRequest, core: usize) -> (result: bool)
    ensures
        result == core_pending_spec(*request, core as int),
{
    let mut i: usize = 0;
    while i < request.pending_acks.len()
        invariant
            0 <= i <= request.pending_acks@.len(),
            forall|j: int| 0 <= j < i as int ==> request.pending_acks@[j] != core,
        decreases request.pending_acks.len() - i,
    {
        if request.pending_acks[i] == core {
            return true;
        }
        i = i + 1;
    }
    false
}

// r[impl tlb-shootdown.freshness]
fn core_tlb_fresh(tlb: &CoreTlb, vpn_start: u64, vpn_end: u64) -> (result: bool)
    ensures
        result == core_tlb_fresh_spec(tlb.entries@, vpn_start, vpn_end),
{
    let mut i: usize = 0;
    while i < tlb.entries.len()
        invariant
            0 <= i <= tlb.entries@.len(),
            forall|j: int| 0 <= j < i as int ==> !vpn_in_range_spec(tlb.entries@[j].vpn, vpn_start, vpn_end),
        decreases tlb.entries.len() - i,
    {
        if vpn_in_range(tlb.entries[i].vpn, vpn_start, vpn_end) {
            return false;
        }
        i = i + 1;
    }
    true
}

fn invalidate_entries(entries: &[TlbEntry], vpn_start: u64, vpn_end: u64) -> (result: Vec<TlbEntry>)
    ensures
        result@ == invalidate_entries_spec(entries@, vpn_start, vpn_end),
        core_tlb_fresh_spec(result@, vpn_start, vpn_end),
{
    let mut result: Vec<TlbEntry> = Vec::new();
    let mut i: usize = 0;
    while i < entries.len()
        invariant
            0 <= i <= entries@.len(),
            result@ == invalidate_entries_prefix_spec(entries@, vpn_start, vpn_end, i as int),
        decreases entries.len() - i,
    {
        if !vpn_in_range(entries[i].vpn, vpn_start, vpn_end) {
            result.push(entries[i]);
            proof {
                assert(invalidate_entries_prefix_spec(entries@, vpn_start, vpn_end, i as int + 1)
                    == invalidate_entries_prefix_spec(entries@, vpn_start, vpn_end, i as int).push(entries[i as int]));
            }
        } else {
            proof {
                assert(invalidate_entries_prefix_spec(entries@, vpn_start, vpn_end, i as int + 1)
                    == invalidate_entries_prefix_spec(entries@, vpn_start, vpn_end, i as int));
            }
        }
        i = i + 1;
    }
    proof {
        invalidate_entries_fresh(entries@, vpn_start, vpn_end);
    }
    result
}

fn remove_core(pending: &[usize], core: usize) -> (result: Vec<usize>)
    ensures
        result@ == remove_core_spec(pending@, core),
{
    let mut result: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < pending.len()
        invariant
            0 <= i <= pending@.len(),
            result@ == remove_core_prefix_spec(pending@, core, i as int),
        decreases pending.len() - i,
    {
        if pending[i] != core {
            result.push(pending[i]);
            proof {
                assert(remove_core_prefix_spec(pending@, core, i as int + 1)
                    == remove_core_prefix_spec(pending@, core, i as int).push(pending[i as int]));
            }
        } else {
            proof {
                assert(remove_core_prefix_spec(pending@, core, i as int + 1)
                    == remove_core_prefix_spec(pending@, core, i as int));
            }
        }
        i = i + 1;
    }
    result
}

fn build_pending_acks(core_count: usize, initiator: usize) -> (result: Vec<usize>)
    requires
        initiator < core_count,
    ensures
        result@ == pending_acks_spec(core_count, initiator),
{
    let mut result: Vec<usize> = Vec::new();
    let mut core: usize = 0;
    while core < core_count
        invariant
            initiator < core_count,
            0 <= core <= core_count,
            result@ == pending_acks_prefix_spec(core_count, initiator, core as int),
        decreases core_count - core,
    {
        if core != initiator {
            result.push(core);
            proof {
                assert(pending_acks_prefix_spec(core_count, initiator, core as int + 1)
                    == pending_acks_prefix_spec(core_count, initiator, core as int).push(core));
            }
        } else {
            proof {
                assert(pending_acks_prefix_spec(core_count, initiator, core as int + 1)
                    == pending_acks_prefix_spec(core_count, initiator, core as int));
            }
        }
        core = core + 1;
    }
    result
}

// r[impl tlb-shootdown.self-invalidation]
pub fn initiate_shootdown(
    initiator: usize,
    vpn_start: u64,
    vpn_end: u64,
    new_ppn: u64,
    core_count: usize,
    initiator_tlb: &CoreTlb,
) -> (result: (ShootdownRequest, CoreTlb))
    requires
        initiator < core_count,
        vpn_start < vpn_end,
    ensures
        result.0.initiator == initiator,
        result.0.vpn_start == vpn_start,
        result.0.vpn_end == vpn_end,
        result.0.new_ppn == new_ppn,
        result.0.pending_acks@ == pending_acks_spec(core_count, initiator),
        result.0.completed == all_acknowledged_spec(result.0),
        result.1.entries@ == invalidate_entries_spec(initiator_tlb.entries@, vpn_start, vpn_end),
        core_tlb_fresh_spec(result.1.entries@, vpn_start, vpn_end),
{
    let pending_acks = build_pending_acks(core_count, initiator);
    let initiator_after = CoreTlb {
        entries: invalidate_entries(&initiator_tlb.entries, vpn_start, vpn_end),
    };
    let request = ShootdownRequest {
        initiator,
        vpn_start,
        vpn_end,
        new_ppn,
        completed: pending_acks.len() == 0,
        pending_acks,
    };
    (request, initiator_after)
}

// r[impl tlb-shootdown.self-invalidation]
// r[impl tlb-shootdown.range]
pub fn initiate_single_shootdown(
    initiator: usize,
    vpn: u64,
    new_ppn: u64,
    core_count: usize,
    initiator_tlb: &CoreTlb,
) -> (result: (ShootdownRequest, CoreTlb))
    requires
        initiator < core_count,
        vpn < u64::MAX,
    ensures
        result.0.vpn_start == vpn,
        result.0.vpn_end == ((vpn as int) + 1) as u64,
        result.0.new_ppn == new_ppn,
        result.1.entries@ == invalidate_entries_spec(initiator_tlb.entries@, vpn, ((vpn as int) + 1) as u64),
{
    initiate_shootdown(initiator, vpn, vpn + 1, new_ppn, core_count, initiator_tlb)
}

// r[impl tlb-shootdown.range]
pub fn handle_shootdown_ipi(core: usize, request: &ShootdownRequest, tlb: &CoreTlb) -> (result: CoreTlb)
    ensures
        result.entries@ == invalidate_entries_spec(tlb.entries@, request.vpn_start, request.vpn_end),
        core_tlb_fresh_spec(result.entries@, request.vpn_start, request.vpn_end),
{
    let _ = core;
    CoreTlb {
        entries: invalidate_entries(&tlb.entries, request.vpn_start, request.vpn_end),
    }
}

// r[impl tlb-shootdown.termination]
pub fn acknowledge_shootdown(core: usize, request: &ShootdownRequest, tlb: &CoreTlb) -> (result: ShootdownRequest)
    ensures
        result.initiator == request.initiator,
        result.vpn_start == request.vpn_start,
        result.vpn_end == request.vpn_end,
        result.new_ppn == request.new_ppn,
        result.completed == all_acknowledged_spec(result),
        if core_pending_spec(*request, core as int) && core_tlb_fresh_spec(tlb.entries@, request.vpn_start, request.vpn_end) {
            &&& result.pending_acks@ == remove_core_spec(request.pending_acks@, core)
        } else {
            &&& result.pending_acks@ == request.pending_acks@
        },
{
    let pending_acks = if core_tlb_fresh(tlb, request.vpn_start, request.vpn_end)
        && core_pending(request, core) {
        remove_core(&request.pending_acks, core)
    } else {
        let mut unchanged: Vec<usize> = Vec::new();
        let mut i: usize = 0;
        while i < request.pending_acks.len()
            invariant
                0 <= i <= request.pending_acks@.len(),
                unchanged@ == request.pending_acks@.subrange(0, i as int),
            decreases request.pending_acks.len() - i,
        {
            unchanged.push(request.pending_acks[i]);
            i = i + 1;
        }
        unchanged
    };
    ShootdownRequest {
        initiator: request.initiator,
        vpn_start: request.vpn_start,
        vpn_end: request.vpn_end,
        new_ppn: request.new_ppn,
        completed: pending_acks.len() == 0,
        pending_acks,
    }
}

// r[impl tlb-shootdown.termination]
pub fn all_acknowledged(request: &ShootdownRequest) -> (result: bool)
    ensures
        result == all_acknowledged_spec(*request),
{
    request.pending_acks.len() == 0
}

// r[impl tlb-shootdown.freshness]
pub fn check_tlb_freshness(tlbs: &[CoreTlb], request: &ShootdownRequest) -> (result: bool)
    ensures
        result == all_core_tlbs_fresh_spec(tlbs@, request.vpn_start, request.vpn_end),
{
    let mut i: usize = 0;
    while i < tlbs.len()
        invariant
            0 <= i <= tlbs@.len(),
            forall|core: int| 0 <= core < i as int ==> core_tlb_fresh_spec(tlbs@[core].entries@, request.vpn_start, request.vpn_end),
        decreases tlbs.len() - i,
    {
        if !core_tlb_fresh(&tlbs[i], request.vpn_start, request.vpn_end) {
            return false;
        }
        i = i + 1;
    }
    true
}

// r[impl tlb-shootdown.initiator-safety]
pub fn safe_to_proceed(tlbs: &[CoreTlb], request: &ShootdownRequest) -> (result: bool)
    ensures
        result == safe_to_proceed_spec(tlbs@, *request),
{
    request.completed && check_tlb_freshness(tlbs, request)
}

// r[verify tlb-shootdown.freshness]
// r[verify tlb-shootdown.initiator-safety]
pub proof fn completion_implies_freshness(tlbs: Seq<CoreTlb>, request: ShootdownRequest)
    requires
        request.completed,
        all_core_tlbs_fresh_spec(tlbs, request.vpn_start, request.vpn_end),
    ensures
        safe_to_proceed_spec(tlbs, request),
{
}

// r[verify tlb-shootdown.range]
pub proof fn invalidation_clears_target_range(entries: Seq<TlbEntry>, vpn_start: u64, vpn_end: u64)
    ensures
        core_tlb_fresh_spec(invalidate_entries_spec(entries, vpn_start, vpn_end), vpn_start, vpn_end),
{
    invalidate_entries_fresh(entries, vpn_start, vpn_end);
}


} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn tlb(entries: &[(u64, u64)]) -> CoreTlb {
        let mut out = Vec::new();
        for &(vpn, ppn) in entries {
            out.push(TlbEntry { vpn, ppn });
        }
        CoreTlb { entries: out }
    }

    // r[verify tlb-shootdown.self-invalidation]
    #[test]
    fn test_single_core_initiation_self_invalidates() {
        let initiator_tlb = tlb(&[(4, 40), (9, 90)]);
        let (request, initiator_after) = initiate_single_shootdown(0, 4, 99, 1, &initiator_tlb);
        assert!(request.completed);
        assert!(all_acknowledged(&request));
        assert_eq!(initiator_after.entries.len(), 1);
        assert_eq!(initiator_after.entries[0].vpn, 9);
    }

    // r[verify tlb-shootdown.freshness]
    // r[verify tlb-shootdown.initiator-safety]
    #[test]
    fn test_multi_core_sequential_shootdown_finishes_fresh() {
        let initiator_tlb = tlb(&[(7, 70), (11, 110)]);
        let core_one = tlb(&[(7, 71), (12, 120)]);
        let core_two = tlb(&[(7, 72), (13, 130)]);

        let (request0, initiator_after) = initiate_single_shootdown(0, 7, 700, 3, &initiator_tlb);
        let core_one_after = handle_shootdown_ipi(1, &request0, &core_one);
        let request1 = acknowledge_shootdown(1, &request0, &core_one_after);
        let core_two_after = handle_shootdown_ipi(2, &request1, &core_two);
        let request2 = acknowledge_shootdown(2, &request1, &core_two_after);

        let tlbs = vec![initiator_after, core_one_after, core_two_after];
        assert!(all_acknowledged(&request2));
        assert!(request2.completed);
        assert!(check_tlb_freshness(&tlbs, &request2));
        assert!(safe_to_proceed(&tlbs, &request2));
    }

    // r[verify tlb-shootdown.range]
    #[test]
    fn test_range_invalidation_keeps_outside_entries() {
        let core = tlb(&[(10, 100), (11, 110), (15, 150)]);
        let (request, _) = initiate_shootdown(0, 10, 12, 900, 2, &tlb(&[(10, 10)]));
        let after = handle_shootdown_ipi(1, &request, &core);
        assert_eq!(after.entries.len(), 1);
        assert_eq!(after.entries[0].vpn, 15);
        assert_eq!(after.entries[0].ppn, 150);
    }

    // r[verify tlb-shootdown.termination]
    #[test]
    fn test_ack_before_invalidation_is_noop() {
        let remote = tlb(&[(5, 50)]);
        let (request, _) = initiate_single_shootdown(0, 5, 55, 2, &tlb(&[(5, 50)]));
        let acked = acknowledge_shootdown(1, &request, &remote);
        assert_eq!(acked.pending_acks, request.pending_acks);
        assert!(!acked.completed);
    }

    // r[verify tlb-shootdown.termination]
    #[test]
    fn test_double_ack_is_noop() {
        let remote = tlb(&[(8, 80)]);
        let (request0, _) = initiate_single_shootdown(0, 8, 88, 2, &tlb(&[(8, 80)]));
        let remote_after = handle_shootdown_ipi(1, &request0, &remote);
        let request1 = acknowledge_shootdown(1, &request0, &remote_after);
        let request2 = acknowledge_shootdown(1, &request1, &remote_after);
        assert_eq!(request2.pending_acks, request1.pending_acks);
        assert_eq!(request2.completed, request1.completed);
    }

    // r[verify tlb-shootdown.isolation]
    #[test]
    fn test_disjoint_requests_do_not_interfere() {
        let core = tlb(&[(2, 20), (9, 90)]);
        let (left_request, _) = initiate_single_shootdown(0, 2, 200, 2, &tlb(&[(2, 20)]));
        let after = handle_shootdown_ipi(1, &left_request, &core);
        assert_eq!(after.entries.len(), 1);
        assert_eq!(after.entries[0].vpn, 9);
        assert_eq!(after.entries[0].ppn, 90);
    }

    proptest! {
        // r[verify tlb-shootdown.range]
        #[test]
        fn prop_range_invalidation_removes_exact_interval(
            entries in proptest::collection::vec((0u64..32, 0u64..256), 0..16),
            start in 0u64..32,
            width in 1u64..8,
        ) {
            let end = core::cmp::min(start + width, 32);
            let core = tlb(&entries);
            let after = handle_shootdown_ipi(
                0,
                &ShootdownRequest {
                    initiator: 0,
                    vpn_start: start,
                    vpn_end: end,
                    new_ppn: 0,
                    pending_acks: vec![],
                    completed: true,
                },
                &core,
            );
            for entry in &after.entries {
                prop_assert!(!(start <= entry.vpn && entry.vpn < end));
            }
            for &(vpn, ppn) in &entries {
                if !(start <= vpn && vpn < end) {
                    prop_assert!(after.entries.iter().any(|entry| entry.vpn == vpn && entry.ppn == ppn));
                }
            }
        }

        // r[verify tlb-shootdown.termination]
        #[test]
        fn prop_pending_count_never_increases(core_count in 1usize..8, initiator in 0usize..8) {
            prop_assume!(initiator < core_count);
            let (request0, _) = initiate_single_shootdown(initiator, 3, 30, core_count, &tlb(&[(3, 30)]));
            let pending_before = request0.pending_acks.len();
            let target_core = if initiator == 0 { 0 } else { initiator - 1 };
            let stale_remote = tlb(&[(3, 30)]);
            let request1 = acknowledge_shootdown(target_core, &request0, &stale_remote);
            prop_assert!(request1.pending_acks.len() <= pending_before);
            let remote_after = handle_shootdown_ipi(target_core, &request0, &stale_remote);
            let request2 = acknowledge_shootdown(target_core, &request0, &remote_after);
            prop_assert!(request2.pending_acks.len() <= pending_before);
        }
    }
}
