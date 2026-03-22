//! Verified chain hash integrity for append-only logs.
//!
//! Models a hash chain where each entry's hash depends on its predecessor,
//! its log index, its term, and its data. Blake3 is modeled as an
//! uninterpreted function with two trusted axioms (determinism and fixed
//! output size). Proves that appending a correctly-hashed entry preserves
//! chain validity.
//!
//! Matches aspen's `compute_entry_hash` and the chain validity invariant
//! from `spec/chain_hash.rs`.

use vstd::prelude::*;

verus! {

/// A chain hash entry: the hash of a log entry in the chain.
pub struct ChainEntry {
    /// The hash value (abstract — 32 bytes in practice).
    pub hash: u64,
    /// Log index this entry covers.
    pub index: u64,
    /// Election term.
    pub term: u64,
    /// Abstract data identifier (hash of the actual data).
    pub data_id: u64,
}

// --- hash function (axiomatized) ---

/// Abstract hash function modeled as uninterpreted.
///
/// In practice this is blake3(prev_hash || index || term || data).
/// We model it as a pure function from four u64 inputs to a u64 hash.
/// The axioms below assert determinism — Verus treats it as opaque.
// r[depends chain.hash-deterministic]
// r[impl chain.hash-deterministic]
pub closed spec fn hash_spec(prev: u64, index: u64, term: u64, data_id: u64) -> u64;

// r[verify chain.hash-deterministic]
/// Axiom: the hash function is deterministic.
#[verifier::external_body]
pub proof fn hash_deterministic(
    prev1: u64, idx1: u64, term1: u64, data1: u64,
    prev2: u64, idx2: u64, term2: u64, data2: u64,
)
    requires prev1 == prev2, idx1 == idx2, term1 == term2, data1 == data2
    ensures hash_spec(prev1, idx1, term1, data1) == hash_spec(prev2, idx2, term2, data2)
{}

// --- compute_entry_hash ---

/// Genesis hash constant.
pub const GENESIS_HASH: u64 = 0u64;

// r[depends chain.compute-entry-hash]
/// Spec: compute the chain hash for an entry.
pub open spec fn compute_entry_hash_spec(
    prev_hash: u64, index: u64, term: u64, data_id: u64,
) -> u64 {
    hash_spec(prev_hash, index, term, data_id)
}

// r[impl chain.compute-entry-hash]
/// Computes the chain hash for a log entry.
///
/// Trusted external body — in production this is blake3. Verus trusts
/// the ensures clause and verifies all callers against the spec.
#[verifier::external_body]
pub fn compute_entry_hash(
    prev_hash: u64, index: u64, term: u64, data_id: u64,
) -> (result: u64)
    ensures result == compute_entry_hash_spec(prev_hash, index, term, data_id)
{
    // Runtime: deterministic mix for testing.
    // Verus ignores this body and trusts the ensures.
    let mut h = prev_hash;
    h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(index);
    h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(term);
    h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(data_id);
    h
}

// --- chain validity ---

// r[depends chain.chain-valid]
// r[impl chain.chain-valid]
/// Spec: chain is valid over entries [0, len).
///
/// Each entry's stored hash equals the hash computed from its predecessor.
pub open spec fn chain_valid_spec(entries: Seq<ChainEntry>) -> bool {
    forall|i: int| #![trigger entries[i]]
        0 <= i < entries.len() ==> {
        let prev_hash = if i == 0 { GENESIS_HASH as u64 } else { entries[i - 1].hash };
        entries[i].hash == compute_entry_hash_spec(
            prev_hash, entries[i].index, entries[i].term, entries[i].data_id,
        )
    }
}

// r[verify chain.chain-valid]
/// Empty chain is trivially valid.
proof fn empty_chain_valid()
    ensures chain_valid_spec(Seq::<ChainEntry>::empty())
{}

// --- append preserves chain ---

/// Spec: the new entry's hash is correctly computed from the chain tip.
pub open spec fn correctly_hashed(
    entries: Seq<ChainEntry>, new_entry: ChainEntry,
) -> bool {
    let prev_hash = if entries.len() == 0 {
        GENESIS_HASH as u64
    } else {
        entries[entries.len() - 1].hash
    };
    new_entry.hash == compute_entry_hash_spec(
        prev_hash, new_entry.index, new_entry.term, new_entry.data_id,
    )
}

// r[depends chain.append-preserves]
/// Spec: appending a correctly-hashed entry preserves validity.
pub open spec fn append_preserves_prop(
    entries: Seq<ChainEntry>, new_entry: ChainEntry,
) -> bool {
    (chain_valid_spec(entries) && correctly_hashed(entries, new_entry))
        ==> chain_valid_spec(entries.push(new_entry))
}

// r[impl chain.append-preserves]
// r[verify chain.append-preserves]
/// Proof: appending a correctly-hashed entry preserves chain validity.
proof fn prove_append_preserves(entries: Seq<ChainEntry>, new_entry: ChainEntry)
    requires
        chain_valid_spec(entries),
        correctly_hashed(entries, new_entry),
    ensures chain_valid_spec(entries.push(new_entry))
{
    let new_chain = entries.push(new_entry);

    assert forall|i: int| #![trigger new_chain[i]] 0 <= i < new_chain.len()
    implies {
        let prev_hash = if i == 0 { GENESIS_HASH as u64 } else { new_chain[i - 1].hash };
        new_chain[i].hash == compute_entry_hash_spec(
            prev_hash, new_chain[i].index, new_chain[i].term, new_chain[i].data_id,
        )
    } by {
        if i < entries.len() as int {
            // Existing entry: unchanged, validity preserved from pre-chain
            assert(new_chain[i] == entries[i]);
            if i > 0 {
                assert(new_chain[i - 1] == entries[i - 1]);
            }
        } else {
            // New entry: validity by construction
            assert(i == entries.len() as int);
            assert(new_chain[i] == new_entry);
            if i > 0 {
                assert(new_chain[i - 1] == entries[i - 1]);
            }
        }
    }
}

// r[verify chain.chain-valid]
/// Prefix of a valid chain is valid.
proof fn prefix_valid(entries: Seq<ChainEntry>, k: int)
    requires chain_valid_spec(entries), 0 <= k <= entries.len()
    ensures chain_valid_spec(entries.subrange(0, k))
{
    let sub = entries.subrange(0, k);
    assert forall|i: int| #![trigger sub[i]] 0 <= i < sub.len()
    implies {
        let prev_hash = if i == 0 { GENESIS_HASH as u64 } else { sub[i - 1].hash };
        sub[i].hash == compute_entry_hash_spec(
            prev_hash, sub[i].index, sub[i].term, sub[i].data_id,
        )
    } by {
        assert(sub[i] == entries[i]);
        if i > 0 {
            assert(sub[i - 1] == entries[i - 1]);
        }
    }
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // Simple test hash for runtime tests (not the axiomatized one)
    fn test_hash(prev: u64, index: u64, term: u64, data_id: u64) -> u64 {
        // Deterministic mix for testing
        let mut h = prev;
        h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(index);
        h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(term);
        h = h.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(data_id);
        h
    }

    fn build_chain(entries: &[(u64, u64, u64)]) -> Vec<ChainEntry> {
        let mut chain = Vec::new();
        let mut prev_hash = GENESIS_HASH;
        for &(index, term, data_id) in entries {
            let hash = test_hash(prev_hash, index, term, data_id);
            chain.push(ChainEntry {
                hash,
                index,
                term,
                data_id,
            });
            prev_hash = hash;
        }
        chain
    }

    fn is_chain_valid(chain: &[ChainEntry]) -> bool {
        for (i, entry) in chain.iter().enumerate() {
            let prev = if i == 0 { GENESIS_HASH } else { chain[i - 1].hash };
            let expected = test_hash(prev, entry.index, entry.term, entry.data_id);
            if entry.hash != expected {
                return false;
            }
        }
        true
    }

    // r[verify chain.chain-valid]
    #[test]
    fn test_empty_chain_valid() {
        assert!(is_chain_valid(&[]));
    }

    // r[verify chain.compute-entry-hash]
    #[test]
    fn test_hash_deterministic() {
        let h1 = test_hash(0, 1, 1, 42);
        let h2 = test_hash(0, 1, 1, 42);
        assert_eq!(h1, h2);
    }

    // r[verify chain.chain-valid]
    #[test]
    fn test_chain_valid() {
        let chain = build_chain(&[(1, 1, 100), (2, 1, 200), (3, 2, 300)]);
        assert!(is_chain_valid(&chain));
    }

    // r[verify chain.chain-valid]
    #[test]
    fn test_corruption_detected() {
        let mut chain = build_chain(&[(1, 1, 100), (2, 1, 200), (3, 2, 300)]);
        // Corrupt the second entry's hash
        chain[1].hash = 999;
        assert!(!is_chain_valid(&chain));
    }

    // r[verify chain.append-preserves]
    #[test]
    fn test_append_preserves() {
        let mut chain = build_chain(&[(1, 1, 100), (2, 1, 200)]);
        assert!(is_chain_valid(&chain));

        // Append correctly
        let prev_hash = chain.last().unwrap().hash;
        let new_hash = test_hash(prev_hash, 3, 2, 300);
        chain.push(ChainEntry {
            hash: new_hash,
            index: 3,
            term: 2,
            data_id: 300,
        });
        assert!(is_chain_valid(&chain));
    }

    proptest! {
        // r[verify chain.hash-deterministic]
        #[test]
        fn prop_hash_deterministic(prev: u64, idx: u64, term: u64, data: u64) {
            prop_assert_eq!(test_hash(prev, idx, term, data), test_hash(prev, idx, term, data));
        }

        // r[verify chain.chain-valid]
        // r[verify chain.append-preserves]
        #[test]
        fn prop_build_chain_valid(
            entries in proptest::collection::vec((1u64..100, 1u64..10, 0u64..1000), 0..=20),
        ) {
            let indexed: Vec<(u64, u64, u64)> = entries
                .iter()
                .enumerate()
                .map(|(i, &(_, term, data))| ((i + 1) as u64, term, data))
                .collect();
            let chain = build_chain(&indexed);
            prop_assert!(is_chain_valid(&chain));
        }

        // r[verify chain.chain-valid]
        #[test]
        fn prop_corruption_detected(
            entries in proptest::collection::vec((1u64..10, 0u64..1000), 2..=10),
            corrupt_idx in 0usize..10,
        ) {
            let indexed: Vec<(u64, u64, u64)> = entries
                .iter()
                .enumerate()
                .map(|(i, &(term, data))| ((i + 1) as u64, term, data))
                .collect();
            let mut chain = build_chain(&indexed);
            let idx = corrupt_idx % chain.len();
            chain[idx].hash = chain[idx].hash.wrapping_add(1);
            prop_assert!(!is_chain_valid(&chain));
        }

        // r[verify chain.chain-valid]
        #[test]
        fn prop_prefix_valid(
            entries in proptest::collection::vec((1u64..10, 0u64..1000), 1..=10),
            k_raw: usize,
        ) {
            let indexed: Vec<(u64, u64, u64)> = entries
                .iter()
                .enumerate()
                .map(|(i, &(term, data))| ((i + 1) as u64, term, data))
                .collect();
            let chain = build_chain(&indexed);
            let k = k_raw % (chain.len() + 1);
            prop_assert!(is_chain_valid(&chain[..k]));
        }
    }
}
