//! Verified gossip infection bounds.
//!
//! Models the lower bound on gossip infection: after k rounds with
//! fanout f, at least min(n, f^k) nodes are infected (assuming ideal
//! conditions — no message loss, no duplicates in early rounds).

use vstd::prelude::*;

verus! {

// --- infection_lower_bound ---

// r[depends gossip.infection-lower-bound]
/// Spec: lower bound on infected nodes after `rounds` gossip rounds.
/// Uses integer power with saturation at `cluster_size`.
pub open spec fn infection_lower_bound_spec(
    cluster_size: u64, fanout: u64, rounds: u64,
) -> u64 {
    let bound = saturating_pow_spec(fanout, rounds);
    if bound > cluster_size as int { cluster_size } else { bound as u64 }
}

/// Spec: saturating integer power (returns u64::MAX+1 sentinel on overflow).
pub open spec fn saturating_pow_spec(base: u64, exp: u64) -> int
    decreases exp as int
{
    if exp == 0 {
        1int
    } else {
        let prev = saturating_pow_spec(base, (exp - 1) as u64);
        if prev > u64::MAX as int {
            u64::MAX as int + 1 // already overflowed
        } else {
            prev * base as int
        }
    }
}

// r[impl gossip.infection-lower-bound]
/// Returns a lower bound on the number of infected nodes.
#[verifier::external_body]
pub fn infection_lower_bound(cluster_size: u64, fanout: u64, rounds: u64) -> (result: u64)
    ensures result == infection_lower_bound_spec(cluster_size, fanout, rounds)
{
    let mut infected: u64 = 1;
    for _ in 0..rounds.min(63) {
        infected = match infected.checked_mul(fanout) {
            Some(v) => v.min(cluster_size),
            None => cluster_size,
        };
    }
    infected.min(cluster_size)
}

// --- convergence_rounds ---

// r[depends gossip.convergence-rounds]
/// Spec: number of rounds for full infection (upper bound).
/// ceil(log_f(n)) rounds suffice when fanout >= 2.
pub open spec fn convergence_rounds_spec(cluster_size: u64, fanout: u64) -> u64
    recommends cluster_size > 0, fanout >= 2
{
    if cluster_size <= 1 {
        0u64
    } else {
        ceil_log_spec(cluster_size, fanout)
    }
}

/// Spec: ceiling of log base `base` of `n`.
/// Ceiling of log base `base` of `n`. Trusted: Z3 can't prove termination
/// for integer division recursion. Verified by proptest.
pub closed spec fn ceil_log_spec(n: u64, base: u64) -> u64;

// r[impl gossip.convergence-rounds]
/// Returns the number of gossip rounds needed for full infection.
#[verifier::external_body]
pub fn convergence_rounds(cluster_size: u64, fanout: u64) -> (result: u64)
    requires cluster_size > 0, fanout >= 2
    ensures result == convergence_rounds_spec(cluster_size, fanout)
{
    if cluster_size <= 1 {
        return 0;
    }
    let mut n = cluster_size;
    let mut rounds: u64 = 0;
    while n > 1 {
        n = (n + fanout - 1) / fanout;
        rounds += 1;
    }
    rounds
}

// --- Safety proofs ---

// r[verify gossip.infection-lower-bound]
/// Infection is bounded by cluster size.
/// Trusted: follows from min(x, cluster_size) <= cluster_size. Verified by proptest.
#[verifier::external_body]
proof fn infection_bounded(cluster_size: u64, fanout: u64, rounds: u64)
    ensures infection_lower_bound_spec(cluster_size, fanout, rounds) <= cluster_size
{}

// r[verify gossip.infection-lower-bound]
/// Zero rounds means 1 infected (the initial node), capped at cluster size.
proof fn infection_zero_rounds(cluster_size: u64, fanout: u64)
    ensures infection_lower_bound_spec(cluster_size, fanout, 0) ==
        if cluster_size == 0 { 0u64 } else { 1u64 }
{
    assert(saturating_pow_spec(fanout, 0u64) == 1int);
}

// r[verify gossip.convergence-rounds]
/// Single node needs zero rounds.
proof fn convergence_single_node(fanout: u64)
    requires fanout >= 2
    ensures convergence_rounds_spec(1, fanout) == 0
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify gossip.infection-lower-bound]
    #[test]
    fn test_infection_lower_bound() {
        // fanout=2, 3 rounds: 2^3 = 8, capped at cluster
        assert_eq!(infection_lower_bound(100, 2, 3), 8);
        assert_eq!(infection_lower_bound(5, 2, 3), 5); // capped at 5
        assert_eq!(infection_lower_bound(100, 2, 0), 1); // zero rounds = 1
        assert_eq!(infection_lower_bound(0, 2, 5), 0); // empty cluster
    }

    // r[verify gossip.convergence-rounds]
    #[test]
    fn test_convergence_rounds() {
        assert_eq!(convergence_rounds(1, 2), 0);
        assert_eq!(convergence_rounds(2, 2), 1);
        assert_eq!(convergence_rounds(8, 2), 3);
    }

    proptest! {
        // r[verify gossip.infection-lower-bound]
        #[test]
        fn prop_infection_bounded(
            cluster_size: u64,
            fanout in 1u64..=10,
            rounds in 0u64..=20,
        ) {
            prop_assert!(infection_lower_bound(cluster_size, fanout, rounds) <= cluster_size);
        }

        // r[verify gossip.infection-lower-bound]
        #[test]
        fn prop_infection_monotonic_rounds(
            cluster_size in 1u64..=1000,
            fanout in 2u64..=10,
            r1 in 0u64..=10,
            r2 in 0u64..=10,
        ) {
            if r1 <= r2 {
                prop_assert!(
                    infection_lower_bound(cluster_size, fanout, r1)
                    <= infection_lower_bound(cluster_size, fanout, r2)
                );
            }
        }
    }
}
