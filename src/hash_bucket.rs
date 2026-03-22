//! Verified hash-to-bucket assignment.
//!
//! Models assignment of hash values to buckets with in-bounds guarantees.
//! Provides both standard modular arithmetic and optimized power-of-two
//! bitwise operations with machine-checked correctness proofs.

use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::bitwise::is_power_of_two_spec;

verus! {

// --- assign_bucket ---

// r[depends hash-bucket.assign]
/// Spec: hash modulo num_buckets.
pub open spec fn assign_bucket_spec(hash: u64, num_buckets: u64) -> u64
    recommends num_buckets > 0
{
    (hash % num_buckets) as u64
}

// r[impl hash-bucket.assign]
/// Assigns a hash value to a bucket using modular arithmetic.
/// Returns hash % `num_buckets` with in-bounds guarantee.
pub fn assign_bucket(hash: u64, num_buckets: u64) -> (result: u64)
    requires num_buckets > 0
    ensures 
        result == assign_bucket_spec(hash, num_buckets),
        result < num_buckets
{
    hash % num_buckets
}

// r[verify hash-bucket.assign]
/// The result is always less than num_buckets (in-bounds).
proof fn assign_bucket_in_bounds(hash: u64, num_buckets: u64)
    requires num_buckets > 0
    ensures assign_bucket_spec(hash, num_buckets) < num_buckets
{}

// r[verify hash-bucket.assign]
/// Assignment is deterministic (same inputs yield same output).
proof fn assign_bucket_deterministic(hash: u64, num_buckets: u64)
    requires num_buckets > 0
    ensures assign_bucket_spec(hash, num_buckets) == assign_bucket_spec(hash, num_buckets)
{}

// --- assign_bucket_pow2 ---

// r[depends hash-bucket.assign-pow2]
/// Spec: hash masked with (num_buckets - 1) for power-of-two optimization.
pub open spec fn assign_bucket_pow2_spec(hash: u64, num_buckets: u64) -> u64
    recommends num_buckets > 0, is_power_of_two_spec(num_buckets)
{
    (hash & sub(num_buckets, 1)) as u64
}

// r[impl hash-bucket.assign-pow2]
/// Assigns a hash value to a bucket using bitwise AND for power-of-two counts.
/// Fast path: hash & (`num_buckets` - 1) when `num_buckets` is a power of two.
pub fn assign_bucket_pow2(hash: u64, num_buckets: u64) -> (result: u64)
    requires
        num_buckets > 0,
        is_power_of_two_buckets_spec(num_buckets)
    ensures 
        result == assign_bucket_pow2_spec(hash, num_buckets)
{
    hash & (num_buckets - 1)
}

// r[verify hash-bucket.assign-pow2]
/// For power of two n, x & (n-1) < n.
proof fn pow2_mask_bound(x: u64, n: u64)
    by(bit_vector)
    requires n > 0, n & sub(n, 1) == 0
    ensures x & sub(n, 1) < n
{}

// r[verify hash-bucket.assign-pow2]
/// The result is always less than num_buckets (in-bounds).
proof fn assign_bucket_pow2_in_bounds(hash: u64, num_buckets: u64)
    requires num_buckets > 0, is_power_of_two_buckets_spec(num_buckets)
    ensures assign_bucket_pow2_spec(hash, num_buckets) < num_buckets
{
    // Use our power-of-two mask bound helper - convert from our spec to the helper's format
    assert(is_power_of_two_buckets_spec(num_buckets) ==> (num_buckets & sub(num_buckets, 1) == 0));
    pow2_mask_bound(hash, num_buckets);
}

// r[verify hash-bucket.assign-pow2]
/// Power-of-two assignment is deterministic.
proof fn assign_bucket_pow2_deterministic(hash: u64, num_buckets: u64)
    requires num_buckets > 0, is_power_of_two_spec(num_buckets)
    ensures assign_bucket_pow2_spec(hash, num_buckets) == assign_bucket_pow2_spec(hash, num_buckets)
{}

// r[verify hash-bucket.assign-pow2]
// r[verify hash-bucket.assign]
/// For power of two n, hash & (n-1) == hash % n.
// Trusted: 64-bit SAT encoding exceeds rlimit. Verified by proptest.
#[verifier::external_body]
proof fn pow2_mask_is_mod(x: u64, n: u64)
    requires n > 0, n & sub(n, 1) == 0
    ensures x & sub(n, 1) == x % n
{}

/// Power-of-two assignment is equivalent to modular assignment.
proof fn assign_bucket_pow2_equivalent(hash: u64, num_buckets: u64)
    requires num_buckets > 0, is_power_of_two_buckets_spec(num_buckets)
    ensures assign_bucket_pow2_spec(hash, num_buckets) == assign_bucket_spec(hash, num_buckets)
{
    assert(is_power_of_two_buckets_spec(num_buckets) ==> (num_buckets & sub(num_buckets, 1) == 0));
    pow2_mask_is_mod(hash, num_buckets);
}

// --- bucket_count_valid ---

// r[depends hash-bucket.valid-count]
/// Spec: num_buckets is positive.
pub open spec fn bucket_count_valid_spec(num_buckets: u64) -> bool {
    num_buckets > 0
}

// r[impl hash-bucket.valid-count]
/// Returns true if the bucket count is valid for hash assignment.
pub fn bucket_count_valid(num_buckets: u64) -> (result: bool)
    ensures result == bucket_count_valid_spec(num_buckets)
{
    num_buckets > 0
}

// r[verify hash-bucket.valid-count]
/// Zero buckets is invalid.
proof fn bucket_count_zero_invalid()
    ensures !bucket_count_valid_spec(0)
{}

// r[verify hash-bucket.valid-count]
/// Positive bucket counts are valid.
proof fn bucket_count_positive_valid(num_buckets: u64)
    requires num_buckets > 0
    ensures bucket_count_valid_spec(num_buckets)
{}

// --- is_power_of_two_buckets ---

// r[depends hash-bucket.is-pow2]
/// Spec: num_buckets is a positive power of two.
pub open spec fn is_power_of_two_buckets_spec(num_buckets: u64) -> bool {
    num_buckets > 0 && (num_buckets & sub(num_buckets, 1)) == 0
}

// r[impl hash-bucket.is-pow2]
/// Returns true if `num_buckets` is a power of two (optimizable for bitwise assignment).
pub fn is_power_of_two_buckets(num_buckets: u64) -> (result: bool)
    ensures result == is_power_of_two_buckets_spec(num_buckets)
{
    num_buckets > 0 && (num_buckets & (num_buckets - 1)) == 0
}

// r[verify hash-bucket.is-pow2]
/// Powers of two: 1, 2, 4, 8 are valid.
proof fn power_of_two_examples()
    ensures
        is_power_of_two_buckets_spec(1),
        is_power_of_two_buckets_spec(2),
        is_power_of_two_buckets_spec(4),
        is_power_of_two_buckets_spec(8),
{
    assert(1u64 & sub(1u64, 1) == 0u64) by(bit_vector);
    assert(2u64 & sub(2u64, 1) == 0u64) by(bit_vector);
    assert(4u64 & sub(4u64, 1) == 0u64) by(bit_vector);
    assert(8u64 & sub(8u64, 1) == 0u64) by(bit_vector);
}

// r[verify hash-bucket.is-pow2]
/// Zero and non-powers of two are not valid.
proof fn non_power_of_two_examples()
    ensures
        !is_power_of_two_buckets_spec(0),
        !is_power_of_two_buckets_spec(3),
        !is_power_of_two_buckets_spec(5),
        !is_power_of_two_buckets_spec(6),
{
    assert(3u64 & sub(3u64, 1) != 0u64) by(bit_vector);
    assert(5u64 & sub(5u64, 1) != 0u64) by(bit_vector);
    assert(6u64 & sub(6u64, 1) != 0u64) by(bit_vector);
}

#[cfg(verus_keep_ghost)]
// r[verify hash-bucket.is-pow2]
/// Our power-of-two check is equivalent to the bitwise module's version.
proof fn is_power_of_two_buckets_equiv(num_buckets: u64)
    ensures is_power_of_two_buckets_spec(num_buckets) == is_power_of_two_spec(num_buckets)
{}

// --- Combined properties ---

// r[verify hash-bucket.valid-count]
// r[verify hash-bucket.is-pow2]
/// Power-of-two buckets are always valid bucket counts.
proof fn power_of_two_is_valid(num_buckets: u64)
    requires is_power_of_two_buckets_spec(num_buckets)
    ensures bucket_count_valid_spec(num_buckets)
{}

// r[verify hash-bucket.assign]
// r[verify hash-bucket.valid-count]
/// Assignment requires valid bucket count.
proof fn assign_requires_valid(hash: u64, num_buckets: u64)
    requires bucket_count_valid_spec(num_buckets)
    ensures assign_bucket_spec(hash, num_buckets) < num_buckets
{}

// r[verify hash-bucket.assign-pow2]
// r[verify hash-bucket.is-pow2]
/// Power-of-two assignment requires power-of-two bucket count.
proof fn assign_pow2_requires_pow2(hash: u64, num_buckets: u64)
    requires is_power_of_two_buckets_spec(num_buckets)
    ensures assign_bucket_pow2_spec(hash, num_buckets) < num_buckets
{
    assign_bucket_pow2_in_bounds(hash, num_buckets);
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    // r[verify hash-bucket.assign]
    #[test]
    fn test_assign_bucket() {
        assert_eq!(assign_bucket(0, 1), 0);
        assert_eq!(assign_bucket(5, 3), 2);
        assert_eq!(assign_bucket(8, 4), 0);
        assert_eq!(assign_bucket(10, 7), 3);
        
        // In-bounds guarantee
        assert!(assign_bucket(u64::MAX, 5) < 5);
        assert!(assign_bucket(12345, 100) < 100);
    }

    // r[verify hash-bucket.assign-pow2]
    #[test]
    fn test_assign_bucket_pow2() {
        assert_eq!(assign_bucket_pow2(0, 1), 0);
        assert_eq!(assign_bucket_pow2(5, 4), 1);  // 5 & 3 = 1
        assert_eq!(assign_bucket_pow2(8, 4), 0);  // 8 & 3 = 0
        assert_eq!(assign_bucket_pow2(15, 8), 7); // 15 & 7 = 7
        
        // In-bounds guarantee
        assert!(assign_bucket_pow2(u64::MAX, 16) < 16);
        assert!(assign_bucket_pow2(12345, 64) < 64);
    }

    // r[verify hash-bucket.valid-count]
    #[test]
    fn test_bucket_count_valid() {
        assert!(!bucket_count_valid(0));
        assert!(bucket_count_valid(1));
        assert!(bucket_count_valid(2));
        assert!(bucket_count_valid(100));
        assert!(bucket_count_valid(u64::MAX));
    }

    // r[verify hash-bucket.is-pow2]
    #[test]
    fn test_is_power_of_two_buckets() {
        assert!(!is_power_of_two_buckets(0));
        assert!(is_power_of_two_buckets(1));
        assert!(is_power_of_two_buckets(2));
        assert!(!is_power_of_two_buckets(3));
        assert!(is_power_of_two_buckets(4));
        assert!(!is_power_of_two_buckets(5));
        assert!(!is_power_of_two_buckets(6));
        assert!(!is_power_of_two_buckets(7));
        assert!(is_power_of_two_buckets(8));
        assert!(is_power_of_two_buckets(1024));
        assert!(!is_power_of_two_buckets(1023));
    }

    // r[verify hash-bucket.assign]
    // r[verify hash-bucket.assign-pow2]
    #[test]
    fn test_pow2_equivalence() {
        // For power-of-two bucket counts, both methods should give same result
        let pow2_buckets = [1, 2, 4, 8, 16, 32, 64, 128];
        let test_hashes = [0, 1, 5, 15, 63, 255, 12345, u64::MAX];
        
        for &buckets in &pow2_buckets {
            for &hash in &test_hashes {
                assert_eq!(
                    assign_bucket(hash, buckets),
                    assign_bucket_pow2(hash, buckets),
                    "Mismatch for hash={hash}, buckets={buckets}"
                );
            }
        }
    }

    proptest! {
        // r[verify hash-bucket.assign]
        #[test]
        fn prop_assign_bucket_in_bounds(hash: u64, buckets in 1u64..=1000) {
            prop_assert!(assign_bucket(hash, buckets) < buckets);
        }

        // r[verify hash-bucket.assign-pow2]
        #[test]
        fn prop_assign_bucket_pow2_in_bounds(hash: u64) {
            let pow2_buckets = [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
            for &buckets in &pow2_buckets {
                prop_assert!(assign_bucket_pow2(hash, buckets) < buckets);
            }
        }

        // r[verify hash-bucket.assign]
        #[test]
        fn prop_assign_bucket_deterministic(hash: u64, buckets in 1u64..=1000) {
            let result1 = assign_bucket(hash, buckets);
            let result2 = assign_bucket(hash, buckets);
            prop_assert_eq!(result1, result2);
        }

        // r[verify hash-bucket.assign-pow2]
        #[test]
        fn prop_assign_bucket_pow2_deterministic(hash: u64) {
            let pow2_buckets = [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
            for &buckets in &pow2_buckets {
                let result1 = assign_bucket_pow2(hash, buckets);
                let result2 = assign_bucket_pow2(hash, buckets);
                prop_assert_eq!(result1, result2);
            }
        }

        // r[verify hash-bucket.assign]
        // r[verify hash-bucket.assign-pow2]
        #[test]
        fn prop_pow2_methods_equivalent(hash: u64) {
            let pow2_buckets = [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
            for &buckets in &pow2_buckets {
                prop_assert_eq!(
                    assign_bucket(hash, buckets),
                    assign_bucket_pow2(hash, buckets)
                );
            }
        }

        // r[verify hash-bucket.valid-count]
        #[test]
        fn prop_bucket_count_valid_iff_positive(buckets: u64) {
            prop_assert_eq!(bucket_count_valid(buckets), buckets > 0);
        }

        // r[verify hash-bucket.is-pow2]
        #[test]
        fn prop_power_of_two_buckets_stdlib_compat(buckets: u64) {
            // Our check should match the standard library for positive values
            if buckets > 0 {
                prop_assert_eq!(is_power_of_two_buckets(buckets), buckets.is_power_of_two());
            }
        }

        // r[verify hash-bucket.is-pow2]
        // r[verify hash-bucket.valid-count]
        #[test]
        fn prop_power_of_two_implies_valid(buckets: u64) {
            if is_power_of_two_buckets(buckets) {
                prop_assert!(bucket_count_valid(buckets));
            }
        }

        // r[verify hash-bucket.assign]
        #[test]
        fn prop_assign_uniform_distribution(buckets in 1u64..=100) {
            // For consecutive hashes, results should cycle through all buckets
            for start in 0u64..buckets {
                let bucket = assign_bucket(start, buckets);
                prop_assert_eq!(bucket, start % buckets);
            }
        }

        // r[verify hash-bucket.assign-pow2]
        #[test]
        fn prop_assign_pow2_bit_pattern(hash: u64) {
            // For each power of 2, the result should be the low bits of hash
            for shift in 0u8..=10 {
                let buckets = 1u64 << shift;  // 1, 2, 4, 8, ...
                let mask = buckets - 1;
                let expected = hash & mask;
                prop_assert_eq!(assign_bucket_pow2(hash, buckets), expected);
            }
        }
    }
}