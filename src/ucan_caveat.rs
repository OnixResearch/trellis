//! Verified composable attenuation constraints (caveats) for UCAN capabilities.
//!
//! Models typed caveats (time bound, rate limit) with conjunction semantics,
//! monotone containment (delegation only narrows), and sound intersection.

#![allow(clippy::ptr_arg)]

use vstd::prelude::*;

verus! {

/// A typed attenuation constraint.
pub enum Caveat {
    /// Time-window restriction.
    TimeBound { not_before: u64, not_after: u64 },
    /// Rate limit within a sliding window.
    RateLimit { max_count: u64, window_secs: u64 },
}

/// Runtime context for caveat evaluation.
pub struct CaveatContext {
    pub now: u64,
    pub count_in_window: u64,
}

// --- Satisfaction ---

/// Single caveat satisfaction.
// r[impl ucan-caveat.satisfaction-decidable]
pub open spec fn caveat_satisfied_spec(c: Caveat, ctx: CaveatContext) -> bool {
    match c {
        Caveat::TimeBound { not_before, not_after } => {
            ctx.now >= not_before && ctx.now <= not_after
        }
        Caveat::RateLimit { max_count, window_secs: _ } => {
            ctx.count_in_window <= max_count
        }
    }
}

/// Caveat set satisfaction: conjunction (all must pass).
// r[impl ucan-caveat.set-conjunction]
pub open spec fn caveat_set_satisfied_spec(caveats: Seq<Caveat>, ctx: CaveatContext) -> bool {
    forall|i: int| #![auto] 0 <= i < caveats.len() ==> caveat_satisfied_spec(caveats[i], ctx)
}

/// Check single caveat at runtime.
// r[impl ucan-caveat.satisfaction-decidable]
pub fn caveat_satisfied(c: &Caveat, ctx: &CaveatContext) -> (result: bool)
    ensures result == caveat_satisfied_spec(*c, *ctx),
{
    match c {
        Caveat::TimeBound { not_before, not_after } => {
            ctx.now >= *not_before && ctx.now <= *not_after
        }
        Caveat::RateLimit { max_count, window_secs: _ } => {
            ctx.count_in_window <= *max_count
        }
    }
}

/// Check caveat set at runtime.
// r[impl ucan-caveat.set-conjunction]
pub fn caveat_set_satisfied(caveats: &Vec<Caveat>, ctx: &CaveatContext) -> (result: bool)
    ensures result == caveat_set_satisfied_spec(caveats@, *ctx),
{
    let mut i: usize = 0;
    while i < caveats.len()
        invariant
            0 <= i <= caveats.len(),
            forall|k: int| #![auto] 0 <= k < i ==> caveat_satisfied_spec(caveats@[k], *ctx),
        decreases caveats.len() - i,
    {
        if !caveat_satisfied(&caveats[i], ctx) {
            return false;
        }
        i = i + 1;
    }
    true
}

// --- Containment ---

/// Single caveat containment: child is at least as restrictive as parent.
// r[impl ucan-caveat.containment-monotone]
pub open spec fn caveat_contains_spec(parent: Caveat, child: Caveat) -> bool {
    match (parent, child) {
        (Caveat::TimeBound { not_before: p_nb, not_after: p_na },
         Caveat::TimeBound { not_before: c_nb, not_after: c_na }) => {
            c_nb >= p_nb && c_na <= p_na
        }
        (Caveat::RateLimit { max_count: p_mc, window_secs: p_ws },
         Caveat::RateLimit { max_count: c_mc, window_secs: c_ws }) => {
            c_mc <= p_mc && c_ws == p_ws
        }
        _ => false,
    }
}

/// Caveat set containment: every parent caveat has a containing child caveat.
// r[impl ucan-caveat.set-containment-monotone]
pub open spec fn caveat_set_contains_spec(parent: Seq<Caveat>, child: Seq<Caveat>) -> bool {
    forall|i: int| 0 <= i < parent.len() ==> (#[trigger] caveat_set_contains_at(parent, child, i))
}

/// Helper: single-index containment witness.
pub open spec fn caveat_set_contains_at(parent: Seq<Caveat>, child: Seq<Caveat>, i: int) -> bool {
    exists|j: int| 0 <= j < child.len() && (#[trigger] caveat_contains_spec(parent[i], child[j]))
}

/// Caveat set intersection: concatenation of both sets.
// r[impl ucan-caveat.intersection-sound]
pub open spec fn caveat_intersect_spec(a: Seq<Caveat>, b: Seq<Caveat>) -> Seq<Caveat> {
    a + b
}

// --- Proofs ---

/// Monotonicity: containment implies satisfaction implication.
// r[verify ucan-caveat.containment-monotone]
pub proof fn caveat_contains_monotone(parent: Caveat, child: Caveat, ctx: CaveatContext)
    requires
        caveat_contains_spec(parent, child),
        caveat_satisfied_spec(child, ctx),
    ensures caveat_satisfied_spec(parent, ctx),
{
}

/// Set conjunction: set satisfied iff all individually satisfied.
// r[verify ucan-caveat.set-conjunction]
pub proof fn caveat_set_conjunction(caveats: Seq<Caveat>, ctx: CaveatContext)
    ensures
        caveat_set_satisfied_spec(caveats, ctx) ==
            (forall|i: int| #![auto] 0 <= i < caveats.len() ==> caveat_satisfied_spec(caveats[i], ctx)),
{
}

/// Set containment monotonicity: child set satisfied implies parent set satisfied.
// r[verify ucan-caveat.set-containment-monotone]
pub proof fn caveat_set_contains_monotone(parent: Seq<Caveat>, child: Seq<Caveat>, ctx: CaveatContext)
    requires
        caveat_set_contains_spec(parent, child),
        caveat_set_satisfied_spec(child, ctx),
    ensures caveat_set_satisfied_spec(parent, ctx),
{
    assert forall|i: int| #![auto] 0 <= i < parent.len()
        implies caveat_satisfied_spec(parent[i], ctx) by {
        assert(caveat_set_contains_at(parent, child, i));
        let j = choose|j: int| 0 <= j < child.len() && (#[trigger] caveat_contains_spec(parent[i], child[j]));
        caveat_contains_monotone(parent[i], child[j], ctx);
    }
}

/// Intersection soundness: satisfying intersection implies satisfying both originals.
// r[verify ucan-caveat.intersection-sound]
pub proof fn intersection_sound(a: Seq<Caveat>, b: Seq<Caveat>, ctx: CaveatContext)
    requires caveat_set_satisfied_spec(caveat_intersect_spec(a, b), ctx),
    ensures
        caveat_set_satisfied_spec(a, ctx),
        caveat_set_satisfied_spec(b, ctx),
{
    let combined = a + b;
    assert forall|i: int| #![auto] 0 <= i < a.len()
        implies caveat_satisfied_spec(a[i], ctx) by {
        assert(combined[i] == a[i]);
        assert(caveat_satisfied_spec(combined[i], ctx));
    }
    assert forall|i: int| #![auto] 0 <= i < b.len()
        implies caveat_satisfied_spec(b[i], ctx) by {
        let idx = a.len() + i;
        assert(combined[idx] == b[i]);
        assert(caveat_satisfied_spec(combined[idx], ctx));
    }
}

/// Intersection minimality: satisfying both implies satisfying intersection.
// r[impl ucan-caveat.intersection-minimal]
// r[verify ucan-caveat.intersection-minimal]
pub proof fn intersection_minimal(a: Seq<Caveat>, b: Seq<Caveat>, ctx: CaveatContext)
    requires
        caveat_set_satisfied_spec(a, ctx),
        caveat_set_satisfied_spec(b, ctx),
    ensures caveat_set_satisfied_spec(caveat_intersect_spec(a, b), ctx),
{
    let combined = a + b;
    assert forall|i: int| #![auto] 0 <= i < combined.len()
        implies caveat_satisfied_spec(combined[i], ctx) by {
        if i < a.len() as int {
            assert(combined[i] == a[i]);
        } else {
            let bi = i - a.len() as int;
            assert(combined[i] == b[bi]);
        }
    }
}

/// Delegation only adds: parent set contained means child covers everything parent covers.
// r[impl ucan-caveat.delegation-only-adds]
// r[verify ucan-caveat.delegation-only-adds]
pub proof fn delegation_only_adds(parent: Seq<Caveat>, child: Seq<Caveat>, ctx: CaveatContext)
    requires
        caveat_set_contains_spec(parent, child),
        caveat_set_satisfied_spec(child, ctx),
    ensures caveat_set_satisfied_spec(parent, ctx),
{
    caveat_set_contains_monotone(parent, child, ctx);
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify ucan-caveat.satisfaction-decidable]
    #[test]
    fn test_time_bound_satisfied() {
        let c = Caveat::TimeBound { not_before: 100, not_after: 200 };
        let ctx = CaveatContext { now: 150, count_in_window: 0 };
        assert!(caveat_satisfied(&c, &ctx));
    }

    #[test]
    fn test_time_bound_expired() {
        let c = Caveat::TimeBound { not_before: 100, not_after: 200 };
        let ctx = CaveatContext { now: 300, count_in_window: 0 };
        assert!(!caveat_satisfied(&c, &ctx));
    }

    #[test]
    fn test_time_bound_not_yet() {
        let c = Caveat::TimeBound { not_before: 100, not_after: 200 };
        let ctx = CaveatContext { now: 50, count_in_window: 0 };
        assert!(!caveat_satisfied(&c, &ctx));
    }

    #[test]
    fn test_rate_limit_ok() {
        let c = Caveat::RateLimit { max_count: 100, window_secs: 3600 };
        let ctx = CaveatContext { now: 0, count_in_window: 50 };
        assert!(caveat_satisfied(&c, &ctx));
    }

    #[test]
    fn test_rate_limit_exceeded() {
        let c = Caveat::RateLimit { max_count: 100, window_secs: 3600 };
        let ctx = CaveatContext { now: 0, count_in_window: 101 };
        assert!(!caveat_satisfied(&c, &ctx));
    }

    #[test]
    fn test_set_conjunction_one_fails() {
        let caveats = vec![
            Caveat::TimeBound { not_before: 100, not_after: 200 },
            Caveat::RateLimit { max_count: 10, window_secs: 60 },
        ];
        let ctx = CaveatContext { now: 150, count_in_window: 20 };
        assert!(!caveat_set_satisfied(&caveats, &ctx));
    }

    #[test]
    fn test_set_conjunction_all_pass() {
        let caveats = vec![
            Caveat::TimeBound { not_before: 100, not_after: 200 },
            Caveat::RateLimit { max_count: 100, window_secs: 60 },
        ];
        let ctx = CaveatContext { now: 150, count_in_window: 50 };
        assert!(caveat_set_satisfied(&caveats, &ctx));
    }
}
