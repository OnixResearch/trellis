//! Verified delegation chain validation for capability authorization.
//!
//! Models the credential chain walk from aspen's `is_credential_chain_valid`:
//! each level attenuates capabilities, each parent holds Delegate,
//! and depth is bounded by `MAX_DELEGATION_DEPTH`.

#[cfg(verus_keep_ghost)]
use crate::access_mode::*;
#[cfg(verus_keep_ghost)]
use crate::containment::*;
#[cfg(verus_keep_ghost)]
use crate::subsumption::*;

use vstd::prelude::*;

verus! {

/// Maximum delegation depth (matches aspen's constant).
pub open spec fn MAX_DELEGATION_DEPTH() -> u64 { 8 }

/// The Delegate variant tag (must match subsumption::V_DELEGATE).
pub open spec fn V_DELEGATE() -> u8 { 7 }

// ============================================================
// Chain level representation
// ============================================================

/// A single level in the delegation chain.
/// Each level has a depth and a set of capabilities.
/// We model capabilities as (variant, scope, mode) tuples.
/// `has_delegate` indicates whether the level holds the Delegate capability.
pub struct ChainLevel {
    pub depth: u64,
    pub has_delegate: bool,
    /// Capabilities at this level — for containment checking we track
    /// representative (variant, mode) pairs. Scope is checked separately.
    pub variant: u8,
    pub mode: u8,
}

// ============================================================
// Depth validation
// ============================================================

// r[depends delegation.depth-bounded]
/// Spec: is the depth within bounds?
pub open spec fn depth_valid_spec(depth: u64) -> bool {
    depth <= MAX_DELEGATION_DEPTH()
}

// r[impl delegation.depth-bounded]
/// Returns true if the depth is within the maximum.
pub fn depth_valid(depth: u64) -> (result: bool)
    ensures result == depth_valid_spec(depth),
{
    depth <= 8
}

// r[depends delegation.root-depth-zero]
/// Spec: is this a valid root level?
pub open spec fn is_root_spec(depth: u64) -> bool {
    depth == 0
}

// r[impl delegation.root-depth-zero]
/// Returns true if this is a root level.
pub fn is_root(depth: u64) -> (result: bool)
    ensures result == is_root_spec(depth),
{
    depth == 0
}

// r[depends delegation.depth-increments]
/// Spec: does the child depth increment from parent by exactly 1?
pub open spec fn depth_increments_spec(parent_depth: u64, child_depth: u64) -> bool {
    child_depth == parent_depth + 1
}

// r[impl delegation.depth-increments]
/// Returns true if child depth is parent depth + 1.
pub fn depth_increments(parent_depth: u64, child_depth: u64) -> (result: bool)
    requires parent_depth < u64::MAX
    ensures result == depth_increments_spec(parent_depth, child_depth),
{
    child_depth == parent_depth + 1
}

// ============================================================
// Chain level validation
// ============================================================

// r[depends delegation.delegate-gate]
/// Spec: is the parent allowed to delegate?
pub open spec fn can_delegate_spec(has_delegate: bool) -> bool {
    has_delegate
}

// r[impl delegation.delegate-gate]
/// Returns true if the parent holds the Delegate capability.
pub fn can_delegate(has_delegate: bool) -> (result: bool)
    ensures result == can_delegate_spec(has_delegate),
{
    has_delegate
}

// r[depends delegation.chain-attenuation]
/// Spec: is a single parent→child transition valid?
///
/// Requirements:
/// 1. Parent has Delegate capability.
/// 2. Child depth == parent depth + 1.
/// 3. Child depth within bounds.
/// 4. Parent variant subsumes child variant (attenuation on variant axis).
/// 5. Child mode ≤ parent mode (attenuation on mode axis).
pub open spec fn level_valid_spec(
    parent_has_delegate: bool,
    parent_depth: u64,
    parent_variant: u8,
    parent_mode: u8,
    child_depth: u64,
    child_variant: u8,
    child_mode: u8,
) -> bool {
    can_delegate_spec(parent_has_delegate)
    && depth_increments_spec(parent_depth, child_depth)
    && depth_valid_spec(child_depth)
    && subsumes_spec(parent_variant, child_variant)
    && mode_le_spec(child_mode, parent_mode)
}

// r[impl delegation.chain-attenuation]
/// Returns true if the parent→child transition is valid.
pub fn level_valid(
    parent_has_delegate: bool,
    parent_depth: u64,
    parent_variant: u8,
    parent_mode: u8,
    child_depth: u64,
    child_variant: u8,
    child_mode: u8,
) -> (result: bool)
    requires parent_depth < u64::MAX
    ensures result == level_valid_spec(
        parent_has_delegate, parent_depth, parent_variant, parent_mode,
        child_depth, child_variant, child_mode,
    ),
{
    can_delegate(parent_has_delegate)
    && depth_increments(parent_depth, child_depth)
    && depth_valid(child_depth)
    && crate::subsumption::subsumes(parent_variant, child_variant)
    && crate::access_mode::mode_le(child_mode, parent_mode)
}

// r[depends delegation.chain-valid]
/// Spec: is the full chain valid? Recursive check from level `from` to the end.
/// `chain[0]` is the root. Each consecutive pair must pass `level_valid`.
pub open spec fn chain_valid_spec(chain: Seq<ChainLevel>, from: int) -> bool
    decreases chain.len() - from,
{
    if from < 0 || from >= chain.len() - 1 {
        true
    } else {
        let parent = chain[from];
        let child = chain[from + 1];
        level_valid_spec(
            parent.has_delegate, parent.depth, parent.variant, parent.mode,
            child.depth, child.variant, child.mode,
        )
        && chain_valid_spec(chain, from + 1)
    }
}

// r[depends delegation.chain-valid]
/// Spec: full chain validation including root check and depth bounds.
pub open spec fn chain_fully_valid_spec(chain: Seq<ChainLevel>) -> bool {
    chain.len() > 0
    && is_root_spec(chain[0].depth)
    && depth_valid_spec(chain[chain.len() - 1 as int].depth)
    && chain_valid_spec(chain, 0)
}

// r[impl delegation.chain-valid]
/// Validates a delegation chain.
/// @verified-logic:no-ensures
#[verifier::external_body]
pub fn chain_valid(chain: &[ChainLevel]) -> (result: bool)
    ensures result == chain_fully_valid_spec(chain@),
{
    if chain.is_empty() {
        return false;
    }
    if chain[0].depth != 0 {
        return false;
    }
    let last = chain.len() - 1;
    if chain[last].depth > 8 {
        return false;
    }
    let mut i: usize = 0;
    while i < chain.len() - 1 {
        let parent = &chain[i];
        let child = &chain[i + 1];
        if !parent.has_delegate {
            return false;
        }
        if child.depth != parent.depth + 1 {
            return false;
        }
        if child.depth > 8 {
            return false;
        }
        if !crate::subsumption::subsumes(parent.variant, child.variant) {
            return false;
        }
        if !crate::access_mode::mode_le(child.mode, parent.mode) {
            return false;
        }
        i += 1;
    }
    true
}

// ============================================================
// Proofs
// ============================================================

// r[verify delegation.root-depth-zero]
/// Root has depth zero.
proof fn root_is_zero()
    ensures is_root_spec(0)
{}

// r[verify delegation.depth-bounded]
/// Depth 0 through 8 are valid.
proof fn depth_bounds()
    ensures
        depth_valid_spec(0),
        depth_valid_spec(8),
        !depth_valid_spec(9),
{}

// r[verify delegation.depth-increments]
/// Depth increments by exactly one.
proof fn depth_inc_correct(parent: u64, child: u64)
    requires depth_increments_spec(parent, child)
    ensures child == parent + 1
{}

// r[verify delegation.delegate-gate]
/// Without Delegate, delegation is denied.
proof fn no_delegate_denies(
    pd: u64, pv: u8, pm: u8,
    cd: u64, cv: u8, cm: u8,
)
    ensures !level_valid_spec(false, pd, pv, pm, cd, cv, cm)
{}

// r[verify delegation.chain-attenuation]
/// Escalation on variant axis is rejected.
proof fn variant_escalation_rejected(
    pd: u64, pv: u8, pm: u8,
    cd: u64, cv: u8, cm: u8,
)
    requires !subsumes_spec(pv, cv)
    ensures !level_valid_spec(true, pd, pv, pm, cd, cv, cm)
{}

// r[verify delegation.chain-attenuation]
/// Escalation on mode axis is rejected.
proof fn mode_escalation_rejected(
    pd: u64, pv: u8, pm: u8,
    cd: u64, cv: u8, cm: u8,
)
    requires !mode_le_spec(cm, pm)
    ensures !level_valid_spec(true, pd, pv, pm, cd, cv, cm)
{}

// r[verify delegation.chain-valid]
/// Single-element chain (root only) is valid.
proof fn single_root_valid()
    ensures chain_fully_valid_spec(seq![
        ChainLevel { depth: 0, has_delegate: false, variant: 0, mode: 0 },
    ])
{
    let chain = seq![
        ChainLevel { depth: 0, has_delegate: false, variant: 0, mode: 0 },
    ];
    // chain.len() == 1, from=0 >= len-1=0, so chain_valid_spec is true
    assert(chain_valid_spec(chain, 0));
}

// r[verify delegation.chain-valid]
/// Two-level chain with proper attenuation is valid.
proof fn two_level_valid()
    ensures chain_fully_valid_spec(seq![
        ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 1, has_delegate: false, variant: 0, mode: 0 },
    ])
{
    let chain = seq![
        ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 1, has_delegate: false, variant: 0, mode: 0 },
    ];
    assert(chain.len() == 2);
    assert(is_root_spec(chain[0].depth));
    assert(depth_valid_spec(chain[1].depth));
    // Unfold chain_valid_spec(chain, 0): from=0, len=2, so check level 0→1
    assert(level_valid_spec(
        chain[0].has_delegate, chain[0].depth, chain[0].variant, chain[0].mode,
        chain[1].depth, chain[1].variant, chain[1].mode,
    ));
    // chain_valid_spec(chain, 1): from=1, len=2, from >= len-1, so true
    assert(chain_valid_spec(chain, 1));
    assert(chain_valid_spec(chain, 0));
}

// r[verify delegation.chain-valid]
/// Chain with wrong root depth is invalid.
proof fn bad_root_invalid()
    ensures !chain_fully_valid_spec(seq![
        ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 2, has_delegate: false, variant: 0, mode: 0 },
    ])
{
    let chain = seq![
        ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 2, has_delegate: false, variant: 0, mode: 0 },
    ];
    assert(!is_root_spec(chain[0].depth));
}

// r[verify delegation.depth-bounded]
/// Chain exceeding max depth is invalid.
proof fn depth_exceeded_invalid()
    ensures !chain_fully_valid_spec(seq![
        ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 2, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 3, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 4, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 5, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 6, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 7, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 8, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 9, has_delegate: false, variant: 0, mode: 0 },
    ])
{
    // Last element has depth 9, which exceeds MAX_DELEGATION_DEPTH (8).
    let chain = seq![
        ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 2, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 3, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 4, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 5, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 6, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 7, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 8, has_delegate: true, variant: 4, mode: 3 },
        ChainLevel { depth: 9, has_delegate: false, variant: 0, mode: 0 },
    ];
    assert(!depth_valid_spec(chain[chain.len() - 1 as int].depth));
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn test_root_depth() {
        assert!(is_root(0));
        assert!(!is_root(1));
    }

    #[test]
    fn test_depth_valid() {
        for d in 0..=8 {
            assert!(depth_valid(d));
        }
        assert!(!depth_valid(9));
        assert!(!depth_valid(u64::MAX));
    }

    #[test]
    fn test_depth_increments() {
        assert!(depth_increments(0, 1));
        assert!(depth_increments(7, 8));
        assert!(!depth_increments(0, 2));
        assert!(!depth_increments(1, 1));
    }

    #[test]
    fn test_single_root_chain() {
        let chain = [ChainLevel { depth: 0, has_delegate: false, variant: 0, mode: 0 }];
        assert!(chain_valid(&chain));
    }

    #[test]
    fn test_two_level_chain() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 1, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(chain_valid(&chain));
    }

    #[test]
    fn test_no_delegate_fails() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: false, variant: 4, mode: 3 },
            ChainLevel { depth: 1, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_bad_root_fails() {
        let chain = [
            ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 2, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_depth_skip_fails() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 2, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_variant_escalation_fails() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 0, mode: 3 }, // Read
            ChainLevel { depth: 1, has_delegate: false, variant: 4, mode: 3 }, // Full — escalation!
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_mode_escalation_fails() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 0 }, // Read mode
            ChainLevel { depth: 1, has_delegate: false, variant: 0, mode: 3 }, // Full mode — escalation!
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_max_depth_chain() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 2, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 3, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 4, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 5, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 6, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 7, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 8, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(chain_valid(&chain));
    }

    #[test]
    fn test_exceeds_max_depth() {
        let chain = [
            ChainLevel { depth: 0, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 1, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 2, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 3, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 4, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 5, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 6, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 7, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 8, has_delegate: true, variant: 4, mode: 3 },
            ChainLevel { depth: 9, has_delegate: false, variant: 0, mode: 0 },
        ];
        assert!(!chain_valid(&chain));
    }

    #[test]
    fn test_empty_chain_invalid() {
        let chain: [ChainLevel; 0] = [];
        assert!(!chain_valid(&chain));
    }

    proptest! {
        // r[verify delegation.depth-bounded]
        #[test]
        fn prop_depth_valid_range(d in 0u64..=8) {
            prop_assert!(depth_valid(d));
        }

        // r[verify delegation.depth-bounded]
        #[test]
        fn prop_depth_invalid_range(d in 9u64..=100) {
            prop_assert!(!depth_valid(d));
        }

        // r[verify delegation.depth-increments]
        #[test]
        fn prop_depth_increments(parent in 0u64..=100) {
            prop_assert!(depth_increments(parent, parent + 1));
            if parent > 0 {
                prop_assert!(!depth_increments(parent, parent));
            }
        }

        // r[verify delegation.chain-attenuation]
        #[test]
        fn prop_attenuation_valid(
            pv in 0u8..=3,
            pm in 0u8..=3,
        ) {
            // Full parent delegates to leaf child at same mode — valid
            prop_assert!(level_valid(true, 0, 4, pm, 1, pv, pm));
        }

        // r[verify delegation.delegate-gate]
        #[test]
        fn prop_no_delegate_denied(
            pd in 0u64..=7,
            pv in 0u8..=7,
            pm in 0u8..=3,
            cv in 0u8..=7,
            cm in 0u8..=3,
        ) {
            prop_assert!(!level_valid(false, pd, pv, pm, pd + 1, cv, cm));
        }
    }
}
