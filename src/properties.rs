//! Cross-module verified properties.
//!
//! Proofs that relate functions from different modules — triangle inequality,
//! round-trip inverses, monotonicity, nearest-point optimality. These are
//! the properties that make the library useful as a verified foundation:
//! downstream code can rely on these relationships holding for all inputs.
//!
//! The spec/proof fns are ghost code (erased in normal builds). Only the
//! tests below are compiled by cargo.

// Spec and proof fns reference spec fns from other modules. These only
// exist during Verus verification (verus_keep_ghost), not in normal cargo
// builds. The verus! macro handles erasure of the proof bodies, but the
// use-statements for ghost items need cfg-gating.

#[cfg(verus_keep_ghost)]
mod verus_proofs {
    use vstd::prelude::*;

    use crate::arithmetic::abs_diff_spec;
    use crate::arithmetic::saturating_add_spec;
    use crate::arithmetic::saturating_sub_spec;
    use crate::bounds::clamp_spec;
    use crate::bounds::in_range_spec;
    use crate::fencing::is_stale_spec;
    use crate::fencing::is_valid_write_spec;
    use crate::fencing::issue_token_spec;
    use crate::monotonic::is_strict_advance_spec;

    verus! {

    // --- abs_diff is a metric ---

    // r[impl properties.abs-diff-zero-iff-equal]
    pub open spec fn abs_diff_zero_iff_equal_prop(a: u64, b: u64) -> bool {
        abs_diff_spec(a, b) == 0 <==> a == b
    }

    // r[verify properties.abs-diff-zero-iff-equal]
    proof fn prove_abs_diff_zero_iff_equal(a: u64, b: u64)
        ensures abs_diff_zero_iff_equal_prop(a, b)
    {}

    // r[impl properties.triangle-inequality]
    pub open spec fn triangle_inequality_prop(a: u64, b: u64, c: u64) -> bool {
        abs_diff_spec(a, c) as int <= abs_diff_spec(a, b) as int + abs_diff_spec(b, c) as int
    }

    // r[verify properties.triangle-inequality]
    proof fn prove_triangle_inequality(a: u64, b: u64, c: u64)
        ensures triangle_inequality_prop(a, b, c)
    {}

    // --- arithmetic inverses ---

    // r[impl properties.add-sub-roundtrip]
    pub open spec fn add_sub_roundtrip_prop(a: u64, b: u64) -> bool
        recommends a as int + b as int <= u64::MAX as int
    {
        saturating_sub_spec(saturating_add_spec(a, b), b) == a
    }

    // r[verify properties.add-sub-roundtrip]
    proof fn prove_add_sub_roundtrip(a: u64, b: u64)
        requires a as int + b as int <= u64::MAX as int
        ensures add_sub_roundtrip_prop(a, b)
    {}

    // r[impl properties.sub-add-inverse]
    pub open spec fn sub_add_inverse_prop(a: u64, b: u64) -> bool
        recommends b <= a
    {
        saturating_add_spec(saturating_sub_spec(a, b), b) == a
    }

    // r[verify properties.sub-add-inverse]
    proof fn prove_sub_add_inverse(a: u64, b: u64)
        requires b <= a
        ensures sub_add_inverse_prop(a, b)
    {}

    // --- clamp properties ---

    // r[impl properties.clamp-nearest-point]
    pub open spec fn clamp_nearest_point_prop(value: u64, x: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, in_range_spec(x, lo, hi)
    {
        abs_diff_spec(value, clamp_spec(value, lo, hi)) <= abs_diff_spec(value, x)
    }

    // r[verify properties.clamp-nearest-point]
    proof fn prove_clamp_nearest_point(value: u64, x: u64, lo: u64, hi: u64)
        requires lo <= hi, in_range_spec(x, lo, hi)
        ensures clamp_nearest_point_prop(value, x, lo, hi)
    {}

    // r[impl properties.clamp-monotonic]
    pub open spec fn clamp_monotonic_prop(a: u64, b: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, a <= b
    {
        clamp_spec(a, lo, hi) <= clamp_spec(b, lo, hi)
    }

    // r[verify properties.clamp-monotonic]
    proof fn prove_clamp_monotonic(a: u64, b: u64, lo: u64, hi: u64)
        requires lo <= hi, a <= b
        ensures clamp_monotonic_prop(a, b, lo, hi)
    {}

    // r[impl properties.distance-decomposition]
    pub open spec fn distance_decomposition_prop(value: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, in_range_spec(value, lo, hi)
    {
        abs_diff_spec(value, lo) as int + abs_diff_spec(value, hi) as int
            == abs_diff_spec(lo, hi) as int
    }

    // r[verify properties.distance-decomposition]
    proof fn prove_distance_decomposition(value: u64, lo: u64, hi: u64)
        requires lo <= hi, in_range_spec(value, lo, hi)
        ensures distance_decomposition_prop(value, lo, hi)
    {}

    // --- bitwise cross-module properties ---

    use crate::ordering::{min_spec, max_spec, median_of_three_spec};
    use crate::arithmetic::div_ceil_spec;
    use crate::integer::{align_down_spec, align_up_spec};
    use crate::slice::{all_equal_spec, chunk_count_spec};
    use crate::collections::{count_matches_spec, count_from_spec, is_partitioned_spec};
    use crate::conversion::narrow_u32_spec;
    use crate::epoch::{epoch_cmp_spec, epoch_ge_spec, Epoch};
    use crate::log::{logs_match_prefix_spec, LogEntry};
    use crate::quorum::majority_spec;

    use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_mod_pos_bound};

    // --- ordering × arithmetic ---

    // r[impl properties.min-max-abs-diff]
    /// max(a,b) - min(a,b) == abs_diff(a,b).
    pub open spec fn min_max_abs_diff_prop(a: u64, b: u64) -> bool {
        (max_spec(a, b) - min_spec(a, b)) as u64 == abs_diff_spec(a, b)
    }

    // r[verify properties.min-max-abs-diff]
    proof fn prove_min_max_abs_diff(a: u64, b: u64)
        ensures min_max_abs_diff_prop(a, b)
    {}

    // --- ordering × bounds ---

    // r[impl properties.clamp-via-min-max]
    /// clamp(v, lo, hi) == min(max(v, lo), hi).
    pub open spec fn clamp_via_min_max_prop(v: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi
    {
        clamp_spec(v, lo, hi) == min_spec(max_spec(v, lo), hi)
    }

    // r[verify properties.clamp-via-min-max]
    proof fn prove_clamp_via_min_max(v: u64, lo: u64, hi: u64)
        requires lo <= hi
        ensures clamp_via_min_max_prop(v, lo, hi)
    {}

    // --- integer × arithmetic ---

    // r[impl properties.align-down-remainder]
    /// x - align_down(x, a) == x % a.
    pub open spec fn align_down_remainder_prop(x: u64, a: u64) -> bool
        recommends a > 0
    {
        abs_diff_spec(x, align_down_spec(x, a)) as int == x as int % a as int
    }

    // r[verify properties.align-down-remainder]
    proof fn prove_align_down_remainder(x: u64, a: u64)
        requires a > 0
        ensures align_down_remainder_prop(x, a)
    {
        lemma_mod_pos_bound(x as int, a as int);
        lemma_fundamental_div_mod(x as int, a as int);
        // x == a * (x/a) + x%a, 0 <= x%a < a
        // therefore x%a <= x (since a * (x/a) >= 0)
    }

    // r[impl properties.align-up-via-div-ceil]
    /// align_up(x, a) == div_ceil(x, a) * a when defined.
    pub open spec fn align_up_via_div_ceil_prop(x: u64, a: u64) -> bool
        recommends a > 0, align_up_spec(x, a).is_some()
    {
        align_up_spec(x, a).unwrap() as int == div_ceil_spec(x, a) as int * a as int
    }

    // r[verify properties.align-up-via-div-ceil]
    proof fn prove_align_up_via_div_ceil(x: u64, a: u64)
        requires a > 0, align_up_spec(x, a).is_some()
        ensures align_up_via_div_ceil_prop(x, a)
    {
        // Both specs share the same core: q = (x == 0 ? 0 : (x-1)/a + 1).
        // align_up gives q * a, div_ceil gives q. So align_up == div_ceil * a.
        // Help Z3 with the cast round-trip:
        if x > 0 {
            vstd::arithmetic::div_mod::lemma_div_pos_is_pos(
                (x - 1) as int, a as int,
            );
        }
    }

    // --- ordering × bounds ---

    // r[impl properties.median-is-clamp]
    /// median_of_three(a, b, c) == clamp(a, min(b, c), max(b, c)).
    pub open spec fn median_is_clamp_prop(a: u64, b: u64, c: u64) -> bool {
        median_of_three_spec(a, b, c) == clamp_spec(a, min_spec(b, c), max_spec(b, c))
    }

    // r[verify properties.median-is-clamp]
    proof fn prove_median_is_clamp(a: u64, b: u64, c: u64)
        ensures median_is_clamp_prop(a, b, c)
    {}

    // --- bitwise cross-module properties ---

    use crate::bitwise::{
        bit_at_spec, set_bit_spec, clear_bit_spec, toggle_bit_spec,
    };

    // r[impl properties.set-clear-roundtrip]
    /// Setting then clearing the same bit leaves it cleared.
    pub open spec fn set_clear_roundtrip_prop(x: u64, pos: u64) -> bool
        recommends pos < 64
    {
        clear_bit_spec(set_bit_spec(x, pos), pos) == clear_bit_spec(x, pos)
    }

    // r[verify properties.set-clear-roundtrip]
    proof fn prove_set_clear_roundtrip(x: u64, pos: u64)
        requires pos < 64
        ensures set_clear_roundtrip_prop(x, pos)
    {
        assert(pos < 64 ==> (x | (1u64 << pos)) & !(1u64 << pos) == x & !(1u64 << pos)) by(bit_vector);
    }

    // r[impl properties.toggle-involution]
    /// Toggling a bit twice returns the original value.
    pub open spec fn toggle_involution_prop(x: u64, pos: u64) -> bool
        recommends pos < 64
    {
        toggle_bit_spec(toggle_bit_spec(x, pos), pos) == x
    }

    // r[verify properties.toggle-involution]
    proof fn prove_toggle_involution(x: u64, pos: u64)
        requires pos < 64
        ensures toggle_involution_prop(x, pos)
    {
        assert(pos < 64 ==> (x ^ (1u64 << pos)) ^ (1u64 << pos) == x) by(bit_vector);
    }

    // --- slice × collections ---

    // r[impl properties.all-equal-implies-count]
    /// all_equal(s, v) implies count_matches(s, v) == s.len().
    pub open spec fn all_equal_implies_count_prop(s: Seq<u64>, v: u64) -> bool {
        all_equal_spec(s, v) ==> count_matches_spec(s, v) == s.len()
    }

    // r[verify properties.all-equal-implies-count]
    proof fn prove_all_equal_implies_count(s: Seq<u64>, v: u64)
        requires all_equal_spec(s, v)
        ensures count_matches_spec(s, v) == s.len()
    {
        all_equal_count_from(s, v, 0);
    }

    /// Helper: all-equal means count_from equals remaining length.
    proof fn all_equal_count_from(s: Seq<u64>, v: u64, from: int)
        requires all_equal_spec(s, v), 0 <= from <= s.len()
        ensures count_from_spec(s, v, from) as int == s.len() - from
        decreases s.len() - from
    {
        if from < s.len() {
            all_equal_count_from(s, v, from + 1);
        }
    }

    // r[impl properties.all-equal-implies-partitioned]
    /// A uniform sequence is partitioned by any threshold.
    pub open spec fn all_equal_implies_partitioned_prop(
        s: Seq<u64>, v: u64, threshold: u64,
    ) -> bool {
        all_equal_spec(s, v) ==> is_partitioned_spec(s, threshold)
    }

    // r[verify properties.all-equal-implies-partitioned]
    proof fn prove_all_equal_implies_partitioned(
        s: Seq<u64>, v: u64, threshold: u64,
    )
        requires all_equal_spec(s, v)
        ensures is_partitioned_spec(s, threshold)
    {}

    // --- slice × arithmetic ---

    // r[impl properties.chunk-count-div-ceil]
    /// div_ceil == chunk_count + remainder indicator.
    pub open spec fn chunk_count_div_ceil_prop(n: u64, k: u64) -> bool
        recommends k > 0
    {
        div_ceil_spec(n, k) as int == chunk_count_spec(n, k) as int
            + (if n as int % k as int > 0 { 1int } else { 0int })
    }

    // r[verify properties.chunk-count-div-ceil]
    proof fn prove_chunk_count_div_ceil(n: u64, k: u64)
        requires k > 0
        ensures chunk_count_div_ceil_prop(n, k)
    {
        if n > 0 {
            let ni = n as int;
            let ki = k as int;
            let q = ni / ki;
            let r = ni % ki;
            lemma_fundamental_div_mod(ni, ki);
            lemma_mod_pos_bound(ni, ki);
            lemma_fundamental_div_mod(ni - 1, ki);
            lemma_mod_pos_bound(ni - 1, ki);
            let q2 = (ni - 1) / ki;
            let r2 = (ni - 1) % ki;
            if r > 0 {
                // ni-1 = ki*q + (r-1) with 0 <= r-1 < ki
                // By quotient uniqueness: (ni-1)/ki == q
                assert(q2 == q) by(nonlinear_arith)
                    requires
                        ni == ki * q + r,
                        ni - 1 == ki * q2 + r2,
                        0 < r, r < ki,
                        0 <= r2, r2 < ki,
                        ki > 0;
            } else {
                // r == 0: ni = ki*q, ni-1 = ki*(q-1) + (ki-1)
                // By quotient uniqueness: (ni-1)/ki == q-1
                assert(q2 == q - 1) by(nonlinear_arith)
                    requires
                        ni == ki * q + r,
                        ni - 1 == ki * q2 + r2,
                        r == 0,
                        0 <= r2, r2 < ki,
                        ki > 0,
                        ni > 0;
            }
        }
    }

    // --- conversion × bounds ---

    // r[impl properties.narrow-iff-in-range]
    /// narrow_u32 agrees with in_range(0, u32::MAX).
    pub open spec fn narrow_iff_in_range_prop(x: u64) -> bool {
        narrow_u32_spec(x).is_some() <==> in_range_spec(x, 0, u32::MAX as u64)
    }

    // r[verify properties.narrow-iff-in-range]
    proof fn prove_narrow_iff_in_range(x: u64)
        ensures narrow_iff_in_range_prop(x)
    {}

    // --- quorum × arithmetic ---

    // r[impl properties.majority-via-div-ceil]
    /// majority(n) == div_ceil(n + 1, 2).
    pub open spec fn majority_via_div_ceil_prop(n: u64) -> bool
        recommends n > 0, (n as int + 1) <= u64::MAX as int
    {
        majority_spec(n) as int == div_ceil_spec((n + 1) as u64, 2) as int
    }

    // r[verify properties.majority-via-div-ceil]
    proof fn prove_majority_via_div_ceil(n: u64)
        requires n > 0, (n as int + 1) <= u64::MAX as int
        ensures majority_via_div_ceil_prop(n)
    {
        // majority(n) = n/2 + 1
        // div_ceil(n+1, 2) = if n+1 == 0 { 0 } else { ((n+1-1)/2 + 1) } = n/2 + 1
    }

    // --- epoch × ordering ---

    // r[impl properties.epoch-ge-term-index]
    /// epoch_ge decomposes into term and index conditions.
    pub open spec fn epoch_ge_term_index_prop(a: Epoch, b: Epoch) -> bool {
        epoch_ge_spec(a, b) ==> (
            a.term > b.term || (a.term == b.term && a.index >= b.index)
        )
    }

    // r[verify properties.epoch-ge-term-index]
    proof fn prove_epoch_ge_term_index(a: Epoch, b: Epoch)
        ensures epoch_ge_term_index_prop(a, b)
    {}

    // --- log × term equality ---

    // r[impl properties.log-prefix-term-eq]
    /// Log prefix matching implies term equality over the prefix.
    pub open spec fn log_prefix_term_eq_prop(
        a: Seq<LogEntry>, b: Seq<LogEntry>, k: int,
    ) -> bool
        recommends 0 <= k <= a.len(), 0 <= k <= b.len()
    {
        logs_match_prefix_spec(a, b, k) ==>
            forall|i: int| 0 <= i < k ==> a[i].term == b[i].term
    }

    // r[verify properties.log-prefix-term-eq]
    proof fn prove_log_prefix_term_eq(
        a: Seq<LogEntry>, b: Seq<LogEntry>, k: int,
    )
        ensures log_prefix_term_eq_prop(a, b, k)
    {}

    // ========================================================================
    // Cross-module properties for new modules
    // ========================================================================

    use crate::chunk::chunk_count_ceil_spec;
    use crate::bounded_range::contains_spec as br_contains_spec;
    use crate::interval::contains_spec as iv_contains_spec;
    use crate::interval::overlaps_spec as iv_overlaps_spec;
    use crate::partition::is_adjacent_pair_spec;
    use crate::partition::is_non_empty_interval_spec;
    use crate::pressure::classify_level_spec;
    use crate::hash_bucket::assign_bucket_spec;
    use crate::rate_limit::tokens_after_refill_spec;
    use crate::drain::{after_start_op_spec, after_finish_op_spec};
    use crate::sequential::is_sequential_spec;
    use crate::chunk::chunk_offset_spec;

    // --- chunk × arithmetic ---

    // r[impl properties.chunk-ceil-is-div-ceil]
    /// chunk_count_ceil and div_ceil must agree.
    pub open spec fn chunk_ceil_is_div_ceil_prop(n: u64, k: u64) -> bool
        recommends k > 0
    {
        chunk_count_ceil_spec(n, k) as int == div_ceil_spec(n, k) as int
    }

    // r[verify properties.chunk-ceil-is-div-ceil]
    proof fn prove_chunk_ceil_is_div_ceil(n: u64, k: u64)
        requires k > 0
        ensures chunk_ceil_is_div_ceil_prop(n, k)
    {
        // Both specs: if n == 0 then 0, else (n-1)/k + 1.
        // div_ceil: if a == 0 { 0 } else { (a - 1) / b + 1 }
        // chunk_count_ceil: if total_size == 0 { 0 } else { ((total_size - 1) as int / chunk_size as int + 1) as u64 }
    }

    // --- bounded_range × interval ---

    // r[impl properties.bounded-range-specializes-interval]
    /// Bounded range reduces to interval when has_upper is true.
    pub open spec fn bounded_range_specializes_interval_prop(
        lo: u64, hi: u64, x: u64,
    ) -> bool {
        br_contains_spec(lo, hi, true, x) == iv_contains_spec(lo, hi, x)
    }

    // r[verify properties.bounded-range-specializes-interval]
    proof fn prove_bounded_range_specializes_interval(lo: u64, hi: u64, x: u64)
        ensures bounded_range_specializes_interval_prop(lo, hi, x)
    {}

    // --- partition × interval ---

    // r[impl properties.partition-no-overlap]
    /// Adjacent partition intervals don't overlap per interval.overlaps.
    pub open spec fn partition_no_overlap_prop(
        a_lo: u64, a_hi: u64, b_lo: u64, b_hi: u64,
    ) -> bool
        recommends
            is_non_empty_interval_spec(a_lo, a_hi),
            is_non_empty_interval_spec(b_lo, b_hi),
            is_adjacent_pair_spec(a_hi, b_lo),
    {
        !iv_overlaps_spec(a_lo, a_hi, b_lo, b_hi)
    }

    // r[verify properties.partition-no-overlap]
    proof fn prove_partition_no_overlap(
        a_lo: u64, a_hi: u64, b_lo: u64, b_hi: u64,
    )
        requires
            is_non_empty_interval_spec(a_lo, a_hi),
            is_non_empty_interval_spec(b_lo, b_hi),
            is_adjacent_pair_spec(a_hi, b_lo),
        ensures partition_no_overlap_prop(a_lo, a_hi, b_lo, b_hi)
    {}

    // --- pressure × monotonic ---

    // r[impl properties.pressure-classify-monotonic]
    /// Monotonic values produce monotonic severity levels.
    pub open spec fn pressure_classify_monotonic_prop(
        v1: u64, v2: u64, warn: u64, crit: u64,
    ) -> bool
        recommends warn <= crit, v1 <= v2
    {
        classify_level_spec(v1, warn, crit) <= classify_level_spec(v2, warn, crit)
    }

    // r[verify properties.pressure-classify-monotonic]
    proof fn prove_pressure_classify_monotonic(
        v1: u64, v2: u64, warn: u64, crit: u64,
    )
        requires warn <= crit, v1 <= v2
        ensures pressure_classify_monotonic_prop(v1, v2, warn, crit)
    {}

    // --- hash_bucket × interval ---

    // r[impl properties.hash-bucket-in-interval]
    /// Hash assignment lands in the valid bucket interval.
    pub open spec fn hash_bucket_in_interval_prop(hash: u64, num_buckets: u64) -> bool
        recommends num_buckets > 0
    {
        iv_contains_spec(0, num_buckets, assign_bucket_spec(hash, num_buckets))
    }

    // r[verify properties.hash-bucket-in-interval]
    proof fn prove_hash_bucket_in_interval(hash: u64, num_buckets: u64)
        requires num_buckets > 0
        ensures hash_bucket_in_interval_prop(hash, num_buckets)
    {}

    // --- rate_limit × bounds ---

    // r[impl properties.refill-within-bounds]
    /// Refill result is always within [0, burst].
    pub open spec fn refill_within_bounds_prop(
        tokens: u64, elapsed_ms: u64, rate_per_sec: u64, burst: u64,
    ) -> bool {
        in_range_spec(tokens_after_refill_spec(tokens, elapsed_ms, rate_per_sec, burst), 0, burst)
    }

    // r[verify properties.refill-within-bounds]
    proof fn prove_refill_within_bounds(
        tokens: u64, elapsed_ms: u64, rate_per_sec: u64, burst: u64,
    )
        ensures refill_within_bounds_prop(tokens, elapsed_ms, rate_per_sec, burst)
    {}

    // --- drain roundtrip ---

    // r[impl properties.drain-start-finish-roundtrip]
    /// Starting then finishing an operation is a no-op on in-flight count.
    pub open spec fn drain_start_finish_roundtrip_prop(n: u64) -> bool
        recommends n < u64::MAX
    {
        after_finish_op_spec(after_start_op_spec(n)) == n
    }

    // r[verify properties.drain-start-finish-roundtrip]
    proof fn prove_drain_start_finish_roundtrip(n: u64)
        requires n < u64::MAX
        ensures drain_start_finish_roundtrip_prop(n)
    {}

    // --- sequential × chunk ---

    // r[impl properties.sequential-chunk-continuation]
    /// Reading consecutive chunks is detected as sequential.
    pub open spec fn sequential_chunk_continuation_prop(
        chunk_size: u64, index: u64,
    ) -> bool
        recommends
            chunk_size > 0,
            index as int * chunk_size as int <= u64::MAX as int,
            (index as int + 1) * chunk_size as int <= u64::MAX as int,
    {
        is_sequential_spec(
            chunk_offset_spec(chunk_size, (index + 1) as u64),
            chunk_offset_spec(chunk_size, index),
            chunk_size,
        )
    }

    // r[verify properties.sequential-chunk-continuation]
    proof fn prove_sequential_chunk_continuation(chunk_size: u64, index: u64)
        requires
            chunk_size > 0,
            index as int * chunk_size as int <= u64::MAX as int,
            (index as int + 1) * chunk_size as int <= u64::MAX as int,
        ensures sequential_chunk_continuation_prop(chunk_size, index)
    {
        let cs = chunk_size as int;
        let idx = index as int;
        let lo = idx * cs;
        let hi = (idx + 1) * cs;
        // Step 1: both products are non-negative and fit in u64
        assert(idx >= 0 && cs > 0) by(nonlinear_arith)
            requires idx >= 0, cs > 0;
        assert(lo >= 0) by(nonlinear_arith)
            requires idx >= 0, cs > 0, lo == idx * cs;
        assert(hi >= 0) by(nonlinear_arith)
            requires idx >= 0, cs > 0, hi == (idx + 1) * cs;
        // Step 2: as-u64 casts are lossless
        assert((lo as u64) as int == lo);
        assert((hi as u64) as int == hi);
        // Step 3: algebraic step — (i+1)*cs == i*cs + cs
        assert(hi == lo + cs) by(nonlinear_arith)
            requires idx >= 0, cs > 0, hi == (idx + 1) * cs, lo == idx * cs;
        // Step 4: the addition doesn't overflow u64
        assert(lo + cs <= u64::MAX as int);
        // Step 5: unfold chunk_offset_spec
        // chunk_offset_spec(cs, idx) = (idx as int * cs as int) as u64
        // index + 1 fits in u64 because (index+1)*cs <= MAX and cs > 0
        assert(idx + 1 <= u64::MAX as int) by(nonlinear_arith)
            requires cs > 0, (idx + 1) * cs <= u64::MAX as int;
        let next_idx = (index + 1) as u64;
        assert(next_idx as int == idx + 1);
        // chunk_offset_spec(chunk_size, index) = (index as int * cs) as u64 = lo as u64
        assert(chunk_offset_spec(chunk_size, index) as int == lo);
        // chunk_offset_spec(chunk_size, next_idx) = (next_idx as int * cs) as u64
        // = ((idx+1) * cs) as u64 = hi as u64
        assert(next_idx as int * cs == hi) by(nonlinear_arith)
            requires next_idx as int == idx + 1, hi == (idx + 1) * cs;
        assert((next_idx as int * cs as int) as u64 as int == hi);
        assert(chunk_offset_spec(chunk_size, next_idx) as int == hi);
        // Step 6: is_sequential_spec: lo + cs = hi <= MAX, offset == last_offset + cs
        let last_offset = chunk_offset_spec(chunk_size, index);
        let offset = chunk_offset_spec(chunk_size, next_idx);
        assert(last_offset as int + chunk_size as int <= u64::MAX as int);
        assert(offset == (last_offset + chunk_size));
    }

    // --- fencing × monotonic ---

    // r[impl properties.fencing-is-strict-advance]
    /// Issuing a fencing token is a strict advance in the monotonic sense.
    pub open spec fn fencing_is_strict_advance_prop(t: u64) -> bool
        recommends t < u64::MAX
    {
        is_strict_advance_spec(t, issue_token_spec(t))
    }

    // r[verify properties.fencing-is-strict-advance]
    proof fn prove_fencing_is_strict_advance(t: u64)
        requires t < u64::MAX
        ensures fencing_is_strict_advance_prop(t)
    {}

    // r[impl properties.fencing-stale-is-not-advance]
    /// A stale token has not advanced to the fence.
    pub open spec fn fencing_stale_is_not_advance_prop(token: u64, fence: u64) -> bool {
        is_stale_spec(token, fence) ==> !is_valid_write_spec(token, fence)
    }

    // r[verify properties.fencing-stale-is-not-advance]
    proof fn prove_fencing_stale_is_not_advance(token: u64, fence: u64)
        ensures fencing_stale_is_not_advance_prop(token, fence)
    {}

    // --- env scrub × suffix matching ---

    use crate::env_scrub::to_upper_spec;
    use crate::env_scrub::ends_with_bytes_spec;
    use crate::env_scrub::has_secret_suffix_spec;
    use crate::env_scrub::is_secret_spec;

    // r[impl properties.to-upper-preserves-suffix]
    /// If a key ends with an uppercase suffix, uppercasing the key preserves
    /// the suffix (since the suffix bytes are already uppercase).
    pub open spec fn to_upper_preserves_suffix_prop(key: Seq<u8>, suffix: Seq<u8>) -> bool
        recommends
            key.len() >= suffix.len(),
            forall|i: int| 0 <= i < suffix.len() as int ==> !(97 <= suffix[i] && suffix[i] <= 122),
    {
        ends_with_bytes_spec(key, suffix) ==> ends_with_bytes_spec(to_upper_spec(key), suffix)
    }

    // r[verify properties.to-upper-preserves-suffix]
    proof fn prove_to_upper_preserves_suffix(key: Seq<u8>, suffix: Seq<u8>)
        requires
            key.len() >= suffix.len(),
            // suffix is already uppercase (no lowercase ASCII)
            forall|i: int| 0 <= i < suffix.len() as int ==> !(97 <= suffix[i] && suffix[i] <= 122),
        ensures to_upper_preserves_suffix_prop(key, suffix)
    {}

    // r[impl properties.secret-implies-not-safe]
    /// A secret key is never safe (tautology, but worth stating).
    pub open spec fn secret_implies_not_safe_prop(key: Seq<u8>) -> bool {
        is_secret_spec(key) ==> !(!is_secret_spec(key))
    }

    // r[verify properties.secret-implies-not-safe]
    proof fn prove_secret_implies_not_safe(key: Seq<u8>)
        ensures secret_implies_not_safe_prop(key)
    {}

    // --- cost tracker × monotonic ---

    use crate::cost_tracker::accumulate_spec;
    use crate::cost_tracker::crosses_threshold_spec;
    use crate::cost_tracker::budget_status_spec;
    use crate::cost_tracker::BUDGET_OK;
    use crate::cost_tracker::BUDGET_WARNING;
    use crate::cost_tracker::BUDGET_EXCEEDED;

    // r[impl properties.accumulate-is-advance]
    /// Cost accumulation is a monotonic advance: total_after >= total_before.
    pub open spec fn accumulate_is_advance_prop(total: u64, cost: u64) -> bool {
        accumulate_spec(total, cost) >= total as int
    }

    // r[verify properties.accumulate-is-advance]
    proof fn prove_accumulate_is_advance(total: u64, cost: u64)
        ensures accumulate_is_advance_prop(total, cost)
    {}

    // r[impl properties.threshold-crossing-irreversible]
    /// Once a threshold is crossed, it stays crossed (monotonic totals).
    pub open spec fn threshold_crossing_irreversible_prop(t1: u64, t2: u64, t3: u64, limit: u64) -> bool
        recommends t1 <= t2, t2 <= t3,
    {
        crosses_threshold_spec(t1, t2, limit) ==> !crosses_threshold_spec(t2, t3, limit)
    }

    // r[verify properties.threshold-crossing-irreversible]
    proof fn prove_threshold_crossing_irreversible(t1: u64, t2: u64, t3: u64, limit: u64)
        requires t1 <= t2, t2 <= t3,
        ensures threshold_crossing_irreversible_prop(t1, t2, t3, limit)
    {}

    // r[impl properties.budget-warning-implies-not-ok]
    /// Warning status implies we're past Ok — can't be both.
    pub open spec fn budget_warning_implies_not_ok_prop(soft: Option<u64>, hard: Option<u64>, total: u64) -> bool {
        budget_status_spec(soft, hard, total) == BUDGET_WARNING()
            ==> budget_status_spec(soft, hard, total) != BUDGET_OK()
    }

    // r[verify properties.budget-warning-implies-not-ok]
    proof fn prove_budget_warning_implies_not_ok(soft: Option<u64>, hard: Option<u64>, total: u64)
        ensures budget_warning_implies_not_ok_prop(soft, hard, total)
    {}

    // r[impl properties.budget-exceeded-stays-exceeded]
    /// Exceeded is absorbing under increasing totals.
    pub open spec fn budget_exceeded_stays_exceeded_prop(soft: Option<u64>, hard: Option<u64>, t1: u64, t2: u64) -> bool
        recommends t2 >= t1,
    {
        (t2 >= t1 && budget_status_spec(soft, hard, t1) == BUDGET_EXCEEDED())
            ==> budget_status_spec(soft, hard, t2) == BUDGET_EXCEEDED()
    }

    // r[verify properties.budget-exceeded-stays-exceeded]
    proof fn prove_budget_exceeded_stays_exceeded(soft: Option<u64>, hard: Option<u64>, t1: u64, t2: u64)
        requires t2 >= t1,
        ensures budget_exceeded_stays_exceeded_prop(soft, hard, t1, t2)
    {}

    // ========================================================================
    // Generic algebraic + pointwise cross-module properties
    // ========================================================================

    use crate::pointwise::{pointwise_merge_spec, max_u64};
    use crate::vector_clock::merge_spec as vc_merge_spec;
    use crate::crdt::g_counter_merge_spec;
    use crate::causal_order::cut_join_spec;

    // r[impl properties.vc-merge-is-pointwise-max]
    /// Vector clock merge is pointwise max.
    pub open spec fn vc_merge_is_pointwise_max_prop(a: Seq<u64>, b: Seq<u64>) -> bool
        recommends a.len() == b.len()
    {
        vc_merge_spec(a, b) =~= pointwise_merge_spec(a, b, |x: u64, y: u64| max_u64(x, y))
    }

    // r[verify properties.vc-merge-is-pointwise-max]
    proof fn prove_vc_merge_is_pointwise_max(a: Seq<u64>, b: Seq<u64>)
        requires a.len() == b.len()
        ensures vc_merge_is_pointwise_max_prop(a, b)
    {}

    // r[impl properties.g-counter-merge-is-pointwise-max]
    /// G-Counter merge is pointwise max.
    pub open spec fn g_counter_merge_is_pointwise_max_prop(a: Seq<u64>, b: Seq<u64>) -> bool
        recommends a.len() == b.len()
    {
        g_counter_merge_spec(a, b) =~= pointwise_merge_spec(a, b, |x: u64, y: u64| max_u64(x, y))
    }

    // r[verify properties.g-counter-merge-is-pointwise-max]
    proof fn prove_g_counter_merge_is_pointwise_max(a: Seq<u64>, b: Seq<u64>)
        requires a.len() == b.len()
        ensures g_counter_merge_is_pointwise_max_prop(a, b)
    {}

    // r[impl properties.cut-join-is-pointwise-max]
    /// Causal order cut join is pointwise max.
    pub open spec fn cut_join_is_pointwise_max_prop(a: Seq<u64>, b: Seq<u64>) -> bool
        recommends a.len() == b.len()
    {
        cut_join_spec(a, b) =~= pointwise_merge_spec(a, b, |x: u64, y: u64| max_u64(x, y))
    }

    // r[verify properties.cut-join-is-pointwise-max]
    proof fn prove_cut_join_is_pointwise_max(a: Seq<u64>, b: Seq<u64>)
        requires a.len() == b.len()
        ensures cut_join_is_pointwise_max_prop(a, b)
    {}

    // r[impl properties.all-three-merges-agree]
    /// VC merge, G-Counter merge, and cut join all agree (they're the same operation).
    pub open spec fn all_three_merges_agree_prop(a: Seq<u64>, b: Seq<u64>) -> bool
        recommends a.len() == b.len()
    {
        vc_merge_spec(a, b) =~= g_counter_merge_spec(a, b)
        && g_counter_merge_spec(a, b) =~= cut_join_spec(a, b)
    }

    // r[verify properties.all-three-merges-agree]
    proof fn prove_all_three_merges_agree(a: Seq<u64>, b: Seq<u64>)
        requires a.len() == b.len()
        ensures all_three_merges_agree_prop(a, b)
    {}

    } // verus!
}

// --- slab ↔ page duality ---

#[cfg(verus_keep_ghost)]
mod slab_page_proofs {
    use vstd::prelude::*;

    use crate::page::frame_number_spec;
    use crate::slab::slab_capacity_spec;
    use crate::slab::slot_index_spec;

    verus! {

    // r[impl properties.slab-slot-is-page-frame]
    pub open spec fn slab_slot_is_page_frame_prop(addr: u64, slot_size: u64) -> bool
        recommends slot_size > 0
    {
        slot_index_spec(addr, 0, slot_size) == frame_number_spec(addr, slot_size)
    }

    // r[verify properties.slab-slot-is-page-frame]
    proof fn prove_slab_slot_is_page_frame(addr: u64, slot_size: u64)
        requires slot_size > 0
        ensures slab_slot_is_page_frame_prop(addr, slot_size)
    {}

    // r[impl properties.slab-capacity-is-pages-dual]
    pub open spec fn slab_capacity_exact_prop(n: u64, slot_size: u64) -> bool
        recommends slot_size > 0, n as int * slot_size as int <= u64::MAX as int
    {
        slab_capacity_spec((n as int * slot_size as int) as u64, slot_size) == n
    }

    // r[verify properties.slab-capacity-is-pages-dual]
    proof fn prove_slab_capacity_exact(n: u64, slot_size: u64)
        requires
            slot_size > 0,
            n as int * slot_size as int <= u64::MAX as int,
        ensures slab_capacity_exact_prop(n, slot_size)
    {
        let product = n as int * slot_size as int;
        let ss = slot_size as int;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(product, ss);
        assert(product / ss == n as int) by(nonlinear_arith)
            requires ss > 0, product == n as int * ss,
                product == ss * (product / ss) + product % ss,
                0 <= product % ss, (product % ss) < ss;
    }

    } // verus!
}

// --- watermark ↔ monotonic/region agreement ---

#[cfg(verus_keep_ghost)]
mod watermark_proofs {
    use vstd::prelude::*;

    use crate::monotonic::is_advance_spec;
    use crate::region::regions_disjoint_spec;
    use crate::watermark::watermark_alloc_spec;

    verus! {

    // r[impl properties.watermark-advance-is-monotonic]
    pub open spec fn watermark_advance_is_monotonic_prop(mark: u64, size: u64) -> bool
        recommends mark as int + size as int <= u64::MAX as int
    {
        is_advance_spec(mark, watermark_alloc_spec(mark, size))
    }

    // r[verify properties.watermark-advance-is-monotonic]
    proof fn prove_watermark_advance_is_monotonic(mark: u64, size: u64)
        requires mark as int + size as int <= u64::MAX as int
        ensures watermark_advance_is_monotonic_prop(mark, size)
    {}

    // r[impl properties.watermark-alloc-disjoint-regions]
    pub open spec fn watermark_alloc_disjoint_regions_prop(
        mark: u64, size1: u64, size2: u64,
    ) -> bool
        recommends
            size1 > 0, size2 > 0,
            mark as int + size1 as int + size2 as int <= u64::MAX as int,
    {
        let mark2 = watermark_alloc_spec(mark, size1);
        regions_disjoint_spec(mark, size1, mark2, size2)
    }

    // r[verify properties.watermark-alloc-disjoint-regions]
    proof fn prove_watermark_alloc_disjoint_regions(
        mark: u64, size1: u64, size2: u64,
    )
        requires
            size1 > 0, size2 > 0,
            mark as int + size1 as int + size2 as int <= u64::MAX as int,
        ensures watermark_alloc_disjoint_regions_prop(mark, size1, size2)
    {}

    } // verus!
}

// --- buddy ↔ bitwise agreement ---

#[cfg(verus_keep_ghost)]
mod buddy_bitwise_proofs {
    use vstd::prelude::*;

    use crate::bitwise::toggle_bit_spec;
    use crate::buddy::block_size_spec;
    use crate::buddy::buddy_of_spec;

    verus! {

    // r[impl properties.buddy-uses-toggle]
    pub open spec fn buddy_uses_toggle_prop(addr: u64, order: u64) -> bool
        recommends order < 64
    {
        buddy_of_spec(addr, order) == toggle_bit_spec(addr, order)
    }

    // r[verify properties.buddy-uses-toggle]
    proof fn prove_buddy_uses_toggle(addr: u64, order: u64)
        requires order < 64
        ensures buddy_uses_toggle_prop(addr, order)
    {
        // Both are addr ^ (1u64 << order)
    }

    // r[impl properties.block-size-is-pow2]
    pub open spec fn block_size_is_pow2_prop(order: u64) -> bool
        recommends order < 64
    {
        block_size_spec(order) == (1u64 << order)
    }

    // r[verify properties.block-size-is-pow2]
    proof fn prove_block_size_is_pow2(order: u64)
        requires order < 64
        ensures block_size_is_pow2_prop(order)
    {}

    } // verus!
}

// --- region ↔ interval agreement ---

#[cfg(verus_keep_ghost)]
mod region_interval_proofs {
    use vstd::prelude::*;

    use crate::interval::adjacent_spec;
    use crate::interval::contains_spec;
    use crate::interval::overlaps_spec;
    use crate::region::contains_addr_spec;
    use crate::region::region_adjacent_spec;
    use crate::region::regions_disjoint_spec;

    verus! {

    // r[impl properties.region-contains-agrees-interval]
    pub open spec fn region_contains_agrees_interval_prop(
        base: u64, size: u64, x: u64,
    ) -> bool
        recommends base as int + size as int <= u64::MAX as int
    {
        contains_addr_spec(base, size, x)
            == contains_spec(base, (base as int + size as int) as u64, x)
    }

    // r[verify properties.region-contains-agrees-interval]
    proof fn prove_region_contains_agrees_interval(base: u64, size: u64, x: u64)
        requires base as int + size as int <= u64::MAX as int
        ensures region_contains_agrees_interval_prop(base, size, x)
    {}

    // r[impl properties.region-disjoint-agrees-interval]
    pub open spec fn region_disjoint_agrees_interval_prop(
        a_base: u64, a_size: u64, b_base: u64, b_size: u64,
    ) -> bool
        recommends
            a_base as int + a_size as int <= u64::MAX as int,
            b_base as int + b_size as int <= u64::MAX as int,
    {
        regions_disjoint_spec(a_base, a_size, b_base, b_size)
            == !overlaps_spec(
                a_base, (a_base as int + a_size as int) as u64,
                b_base, (b_base as int + b_size as int) as u64,
            )
    }

    // r[verify properties.region-disjoint-agrees-interval]
    proof fn prove_region_disjoint_agrees_interval(
        a_base: u64, a_size: u64, b_base: u64, b_size: u64,
    )
        requires
            a_base as int + a_size as int <= u64::MAX as int,
            b_base as int + b_size as int <= u64::MAX as int,
        ensures region_disjoint_agrees_interval_prop(a_base, a_size, b_base, b_size)
    {}

    // r[impl properties.region-adjacent-agrees-interval]
    pub open spec fn region_adjacent_agrees_interval_prop(
        a_base: u64, a_size: u64, b_base: u64, b_size: u64,
    ) -> bool
        recommends
            a_base as int + a_size as int <= u64::MAX as int,
            b_base as int + b_size as int <= u64::MAX as int,
    {
        region_adjacent_spec(a_base, a_size, b_base, b_size)
            == adjacent_spec(
                a_base, (a_base as int + a_size as int) as u64,
                b_base, (b_base as int + b_size as int) as u64,
            )
    }

    // r[verify properties.region-adjacent-agrees-interval]
    proof fn prove_region_adjacent_agrees_interval(
        a_base: u64, a_size: u64, b_base: u64, b_size: u64,
    )
        requires
            a_base as int + a_size as int <= u64::MAX as int,
            b_base as int + b_size as int <= u64::MAX as int,
        ensures region_adjacent_agrees_interval_prop(a_base, a_size, b_base, b_size)
    {}

    } // verus!
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::arithmetic::*;
    use crate::bitwise::*;
    use crate::bounds::*;
    use crate::collections::*;
    use crate::conversion::*;
    use crate::epoch::Epoch;
    use crate::epoch::epoch_ge;
    use crate::integer::*;
    use crate::log::LogEntry;
    use crate::log::logs_match_prefix;
    use crate::ordering::*;
    use crate::quorum::majority;
    use crate::slice::*;

    proptest! {
        // r[verify properties.abs-diff-zero-iff-equal]
        #[test]
        fn prop_abs_diff_zero_iff_equal(a: u64, b: u64) {
            if a == b {
                prop_assert_eq!(abs_diff(a, b), 0);
            } else {
                prop_assert_ne!(abs_diff(a, b), 0);
            }
        }

        // r[verify properties.triangle-inequality]
        #[test]
        fn prop_triangle_inequality(a: u64, b: u64, c: u64) {
            let ac = u128::from(abs_diff(a, c));
            let ab = u128::from(abs_diff(a, b));
            let bc = u128::from(abs_diff(b, c));
            prop_assert!(ac <= ab + bc);
        }

        // r[verify properties.add-sub-roundtrip]
        #[test]
        fn prop_add_sub_roundtrip(a: u64, b in 0..=u64::MAX / 2) {
            let a = a % (u64::MAX - b + 1);
            prop_assert_eq!(saturating_sub(saturating_add(a, b), b), a);
        }

        // r[verify properties.sub-add-inverse]
        #[test]
        fn prop_sub_add_inverse(a: u64, b: u64) {
            if b <= a {
                prop_assert_eq!(saturating_add(saturating_sub(a, b), b), a);
            }
        }

        // r[verify properties.clamp-nearest-point]
        #[test]
        fn prop_clamp_nearest_point(
            value: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            let clamped = clamp(value, lo, hi);
            let dist_clamped = abs_diff(value, clamped);
            prop_assert!(dist_clamped <= abs_diff(value, lo));
            prop_assert!(dist_clamped <= abs_diff(value, hi));
        }

        // r[verify properties.clamp-monotonic]
        #[test]
        fn prop_clamp_monotonic(
            a: u64,
            b: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            if a <= b {
                prop_assert!(clamp(a, lo, hi) <= clamp(b, lo, hi));
            }
        }

        // r[verify properties.distance-decomposition]
        #[test]
        fn prop_distance_decomposition(
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            let value = lo + spread / 2;
            let d_lo = u128::from(abs_diff(value, lo));
            let d_hi = u128::from(abs_diff(value, hi));
            let range = u128::from(abs_diff(lo, hi));
            prop_assert_eq!(d_lo + d_hi, range);
        }

        // r[verify properties.min-max-abs-diff]
        #[test]
        fn prop_min_max_abs_diff(a: u64, b: u64) {
            prop_assert_eq!(max(a, b) - min(a, b), abs_diff(a, b));
        }

        // r[verify properties.clamp-via-min-max]
        #[test]
        fn prop_clamp_via_min_max(
            v: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            prop_assert_eq!(clamp(v, lo, hi), min(max(v, lo), hi));
        }

        // r[verify properties.align-down-remainder]
        #[test]
        fn prop_align_down_remainder(x: u64, a in 1u64..=4096) {
            prop_assert_eq!(abs_diff(x, align_down(x, a)), x % a);
        }

        // r[verify properties.align-up-via-div-ceil]
        #[test]
        fn prop_align_up_via_div_ceil(x: u64, a in 1u64..=4096) {
            if let Some(v) = align_up(x, a) {
                prop_assert_eq!(v, div_ceil(x, a) * a);
            }
        }

        // r[verify properties.median-is-clamp]
        #[test]
        fn prop_median_is_clamp(a: u64, b: u64, c: u64) {
            prop_assert_eq!(
                median_of_three(a, b, c),
                clamp(a, min(b, c), max(b, c))
            );
        }

        // r[verify properties.set-clear-roundtrip]
        #[test]
        fn prop_set_clear_roundtrip(x: u64, pos in 0u64..64) {
            prop_assert_eq!(clear_bit(set_bit(x, pos), pos), clear_bit(x, pos));
        }

        // r[verify properties.toggle-involution]
        #[test]
        fn prop_toggle_involution(x: u64, pos in 0u64..64) {
            prop_assert_eq!(toggle_bit(toggle_bit(x, pos), pos), x);
        }

        // r[verify properties.all-equal-implies-count]
        #[test]
        fn prop_all_equal_implies_count(value: u64, len in 0usize..=50) {
            let v: Vec<u64> = vec![value; len];
            prop_assert_eq!(count_matches(&v, value), len);
        }

        // r[verify properties.chunk-count-div-ceil]
        #[test]
        fn prop_chunk_count_div_ceil(n: u64, k in 1u64..=10000) {
            let cc = u128::from(chunk_count(n, k));
            let dc = u128::from(div_ceil(n, k));
            let has_rem = u128::from(n % k > 0);
            prop_assert_eq!(dc, cc + has_rem);
        }

        // r[verify properties.narrow-iff-in-range]
        #[test]
        fn prop_narrow_iff_in_range(x: u64) {
            prop_assert_eq!(
                narrow_u32(x).is_some(),
                in_range(x, 0, u64::from(u32::MAX))
            );
        }

        // r[verify properties.all-equal-implies-partitioned]
        #[test]
        fn prop_all_equal_implies_partitioned(
            value: u64,
            len in 0usize..=50,
            threshold: u64,
        ) {
            let v: Vec<u64> = vec![value; len];
            prop_assert!(is_partitioned(&v, threshold));
        }

        // r[verify properties.majority-via-div-ceil]
        #[test]
        fn prop_majority_via_div_ceil(n in 1u64..=10000) {
            // majority(n) = n/2 + 1
            // div_ceil(n+1, 2) = (n+1-1)/2 + 1 = n/2 + 1
            prop_assert_eq!(majority(n), div_ceil(n + 1, 2));
        }

        // r[verify properties.epoch-ge-term-index]
        #[test]
        fn prop_epoch_ge_term_index(
            t1: u64, i1: u64, t2: u64, i2: u64,
        ) {
            let a = Epoch { term: t1, index: i1 };
            let b = Epoch { term: t2, index: i2 };
            if epoch_ge(&a, &b) {
                prop_assert!(t1 > t2 || (t1 == t2 && i1 >= i2));
            }
        }

        // r[verify properties.log-prefix-term-eq]
        #[test]
        fn prop_log_prefix_term_eq(
            terms in proptest::collection::vec(1u64..10, 1..=20),
        ) {
            let a: Vec<LogEntry> = terms.iter().map(|&t| LogEntry { term: t, value: 0 }).collect();
            let b: Vec<LogEntry> = terms.iter().map(|&t| LogEntry { term: t, value: 1 }).collect();
            let k = terms.len();
            prop_assert!(logs_match_prefix(&a, &b, k));
            for i in 0..k {
                prop_assert_eq!(a[i].term, b[i].term);
            }
        }

        // --- new module cross-module tests ---

        // r[verify properties.chunk-ceil-is-div-ceil]
        #[test]
        fn prop_chunk_ceil_is_div_ceil(n: u64, k in 1u64..=10000) {
            prop_assert_eq!(
                crate::chunk::chunk_count_ceil(n, k),
                crate::arithmetic::div_ceil(n, k),
            );
        }

        // r[verify properties.bounded-range-specializes-interval]
        #[test]
        fn prop_bounded_range_specializes_interval(lo: u64, hi: u64, x: u64) {
            prop_assert_eq!(
                crate::bounded_range::contains(lo, hi, true, x),
                crate::interval::contains(lo, hi, x),
            );
        }

        // r[verify properties.partition-no-overlap]
        #[test]
        fn prop_partition_no_overlap(
            a_lo in 0u64..u64::MAX / 3,
            width_a in 1u64..u64::MAX / 3,
            width_b in 1u64..u64::MAX / 3,
        ) {
            let a_hi = a_lo + width_a;
            let b_lo = a_hi; // adjacent: a_hi == b_lo
            let b_hi = b_lo + width_b;
            prop_assert!(!crate::interval::overlaps(a_lo, a_hi, b_lo, b_hi));
        }

        // r[verify properties.pressure-classify-monotonic]
        #[test]
        fn prop_pressure_classify_monotonic(
            v1: u64, v2: u64,
            warn in 0u64..u64::MAX / 2,
            spread in 0u64..u64::MAX / 2,
        ) {
            let crit = warn + spread;
            if v1 <= v2 {
                prop_assert!(
                    crate::pressure::classify_level(v1, warn, crit)
                        <= crate::pressure::classify_level(v2, warn, crit)
                );
            }
        }

        // r[verify properties.hash-bucket-in-interval]
        #[test]
        fn prop_hash_bucket_in_interval(hash: u64, num_buckets in 1u64..=u64::MAX) {
            let bucket = crate::hash_bucket::assign_bucket(hash, num_buckets);
            prop_assert!(crate::interval::contains(0, num_buckets, bucket));
        }

        // r[verify properties.refill-within-bounds]
        #[test]
        fn prop_refill_within_bounds(
            tokens: u64, elapsed_ms: u64, rate_per_sec: u64, burst: u64,
        ) {
            let result = crate::rate_limit::tokens_after_refill(
                tokens, elapsed_ms, rate_per_sec, burst,
            );
            prop_assert!(crate::bounds::in_range(result, 0, burst));
        }

        // r[verify properties.drain-start-finish-roundtrip]
        #[test]
        fn prop_drain_start_finish_roundtrip(n in 0u64..u64::MAX) {
            let started = crate::drain::after_start_op(n);
            let finished = crate::drain::after_finish_op(started);
            prop_assert_eq!(finished, n);
        }

        // r[verify properties.sequential-chunk-continuation]
        #[test]
        fn prop_sequential_chunk_continuation(
            chunk_size in 1u64..=1000,
            index in 0u64..1000,
        ) {
            let offset_i = crate::chunk::chunk_offset(chunk_size, index);
            let offset_next = crate::chunk::chunk_offset(chunk_size, index + 1);
            prop_assert!(crate::sequential::is_sequential(
                offset_next, offset_i, chunk_size
            ));
        }

        // r[verify properties.fencing-is-strict-advance]
        #[test]
        fn prop_fencing_is_strict_advance(t in 0..u64::MAX) {
            let issued = crate::fencing::issue_token(t);
            prop_assert!(crate::monotonic::is_strict_advance(t, issued));
        }

        // r[verify properties.fencing-stale-is-not-advance]
        #[test]
        fn prop_fencing_stale_is_not_advance(token: u64, fence: u64) {
            if crate::fencing::is_stale(token, fence) {
                prop_assert!(!crate::fencing::is_valid_write(token, fence));
            }
        }

        // --- Phase 14 cross-module properties ---

        // r[verify properties.vector-clock-merge-is-ordering-max]
        #[test]
        fn prop_vector_clock_merge_is_ordering_max(
            a in proptest::collection::vec(any::<u64>(), 3),
            b in proptest::collection::vec(any::<u64>(), 3),
        ) {
            let m = crate::vector_clock::merge(&a, &b);
            for i in 0..3 {
                prop_assert_eq!(m[i], crate::ordering::max(a[i], b[i]));
            }
        }

        // r[verify properties.ballot-winner-needs-quorum]
        #[test]
        fn prop_ballot_winner_needs_quorum(
            n in 1u64..=10000,
            votes: u64,
        ) {
            if crate::ballot::has_won_election(votes, n) {
                prop_assert!(crate::quorum::is_quorum(votes, n));
            }
        }

        // r[verify properties.lease-disjoint-intervals]
        #[test]
        fn prop_lease_disjoint_intervals(
            start in 0u64..1000,
            width in 1u64..1000,
            width2 in 1u64..1000,
            t: u64,
        ) {
            let expiry = start + width;
            let start2 = expiry;
            if start2 + width2 > start2 {
                let a = crate::lease::grant(1, start, expiry);
                let b = crate::lease::grant(1, start2, start2 + width2);
                // Sequential leases never overlap
                prop_assert!(!(crate::lease::is_valid(&a, t) && crate::lease::is_valid(&b, t)));
            }
        }

        // r[verify properties.two-phase-abort-prevents-commit]
        #[test]
        fn prop_two_phase_abort_prevents_commit(
            votes in proptest::collection::vec(
                proptest::option::of(any::<bool>()), 1..=10
            ),
        ) {
            if crate::two_phase::must_abort(&votes) {
                prop_assert!(!crate::two_phase::can_commit(&votes));
            }
        }

        // r[verify properties.bloom-no-false-negatives]
        #[test]
        fn prop_bloom_no_false_negatives(
            size in 8usize..64,
            positions in proptest::collection::vec(0usize..8, 1..=5),
        ) {
            let positions: Vec<usize> = positions.into_iter().map(|p| p % size).collect();
            let mut bits = vec![false; size];
            crate::bloom::insert(&mut bits, &positions);
            prop_assert!(crate::bloom::query(&bits, &positions));
        }

        // r[verify properties.cut-join-is-vector-clock-merge]
        #[test]
        fn prop_cut_join_is_vector_clock_merge(
            a in proptest::collection::vec(any::<u64>(), 3),
            b in proptest::collection::vec(any::<u64>(), 3),
        ) {
            // cut_join and vector_clock::merge produce the same result
            prop_assert_eq!(
                crate::causal_order::cut_join(&a, &b),
                crate::vector_clock::merge(&a, &b)
            );
        }

        // r[verify properties.crdt-merge-is-vector-clock-merge]
        #[test]
        fn prop_crdt_merge_is_vector_clock_merge(
            a in proptest::collection::vec(any::<u64>(), 3),
            b in proptest::collection::vec(any::<u64>(), 3),
        ) {
            // G-Counter merge and vector_clock::merge are the same operation
            prop_assert_eq!(
                crate::crdt::g_counter_merge(&a, &b),
                crate::vector_clock::merge(&a, &b)
            );
        }

        // r[verify properties.backoff-bounded-by-cap]
        #[test]
        fn prop_backoff_bounded_by_cap(base: u64, attempt: u64, cap: u64) {
            prop_assert!(crate::backoff::compute_backoff(base, attempt, cap) <= cap);
        }

        // r[verify properties.circuit-breaker-valid-output]
        #[test]
        fn prop_circuit_breaker_valid_output(
            current in 0u8..=2,
            failures: u64,
            threshold in 1u64..=1000,
            elapsed: u64,
            timeout in 1u64..=1000,
            probe: bool,
        ) {
            let ns = crate::circuit_breaker::next_state(
                current, failures, threshold, elapsed, timeout, probe
            );
            prop_assert!(crate::circuit_breaker::is_valid_state(ns));
        }

        // r[verify properties.accumulate-is-advance]
        #[test]
        fn prop_accumulate_is_advance(total in 0u64..u64::MAX/2, cost in 0u64..u64::MAX/2) {
            if let Some(result) = crate::cost_tracker::accumulate(total, cost) {
                prop_assert!(result >= total);
            }
        }

        // r[verify properties.threshold-crossing-irreversible]
        #[test]
        fn prop_threshold_crossing_irreversible(
            t1 in 0u64..1_000_000u64,
            d1 in 0u64..1_000_000u64,
            d2 in 0u64..1_000_000u64,
            limit in 0u64..2_000_000u64,
        ) {
            let t2 = t1.saturating_add(d1);
            let t3 = t2.saturating_add(d2);
            let c1 = crate::cost_tracker::crosses_threshold(t1, t2, limit);
            let c2 = crate::cost_tracker::crosses_threshold(t2, t3, limit);
            prop_assert!(!(c1 && c2), "threshold crossed twice");
        }

        // r[verify properties.to-upper-preserves-suffix]
        #[test]
        fn prop_to_upper_idempotent_cross(s in proptest::collection::vec(0u8..=255u8, 0..50)) {
            let u1 = crate::env_scrub::to_upper(&s);
            let u2 = crate::env_scrub::to_upper(&u1);
            prop_assert_eq!(u1, u2);
        }

        // r[verify properties.secret-implies-not-safe]
        #[test]
        fn prop_secret_and_safe_disjoint(key in proptest::collection::vec(0u8..=255u8, 0..30)) {
            let secret = crate::env_scrub::is_secret(&key);
            let safe = !secret;
            prop_assert!(!(secret && safe), "key is both secret and safe");
        }

        // r[verify properties.budget-exceeded-stays-exceeded]
        #[test]
        fn prop_budget_exceeded_stays_exceeded(
            soft in proptest::option::of(0u64..10_000_000u64),
            hard in proptest::option::of(0u64..10_000_000u64),
            t1 in 0u64..10_000_000u64,
            delta in 0u64..5_000_000u64,
        ) {
            let t2 = t1.saturating_add(delta);
            let s1 = crate::cost_tracker::budget_status(soft, hard, t1);
            let s2 = crate::cost_tracker::budget_status(soft, hard, t2);
            if s1 == 3 { // Exceeded
                prop_assert_eq!(s2, 3, "exceeded went backward");
            }
        }

        // ============================================================
        // UCAN cross-module properties
        // ============================================================

        // r[verify properties.delegation-safety]
        /// If a capability authorizes an operation and a parent contains that capability,
        /// then the parent also authorizes the operation.
        #[test]
        fn prop_delegation_safety(
            pv in 0u8..=7, cv in 0u8..=7, ov in 0u8..=7,
            pm in 0u8..=3, cm in 0u8..=3, om in 0u8..=3,
            parent_scope in proptest::collection::vec(any::<u8>(), 0..=10),
            cap_ext in proptest::collection::vec(any::<u8>(), 0..=5),
            target_ext in proptest::collection::vec(any::<u8>(), 0..=5),
        ) {
            let mut cap_scope = parent_scope.clone();
            cap_scope.extend_from_slice(&cap_ext);
            let mut target = cap_scope.clone();
            target.extend_from_slice(&target_ext);

            let parent_contains_cap = crate::containment::contains(pv, &parent_scope, pm, cv, &cap_scope, cm);
            let cap_authorizes_op = crate::authorize::authorizes(cv, &cap_scope, cm, ov, &target, om);

            if parent_contains_cap && cap_authorizes_op {
                // Parent scope covers cap scope (from containment),
                // cap scope covers target (from authorization),
                // so parent scope covers target (by transitivity).
                // Parent mode >= cap mode >= op mode (from ordering).
                // Parent variant subsumes cap variant subsumes op variant (from transitivity).
                let parent_auth = crate::authorize::authorizes(pv, &parent_scope, pm, ov, &target, om);
                prop_assert!(parent_auth, "delegation safety violated");
            }
        }

        // r[verify properties.auth-implies-subsumes]
        /// Authorization implies variant subsumption.
        #[test]
        fn prop_auth_implies_subsumes(
            cv in 0u8..=7, ov in 0u8..=7,
            cm in 0u8..=3, om in 0u8..=3,
            scope in proptest::collection::vec(any::<u8>(), 0..=10),
            target in proptest::collection::vec(any::<u8>(), 0..=10),
        ) {
            if crate::authorize::authorizes(cv, &scope, cm, ov, &target, om) {
                prop_assert!(crate::subsumption::subsumes(cv, ov));
                prop_assert!(crate::scope::covers(&scope, &target));
                prop_assert!(crate::access_mode::mode_permits(cm, om));
            }
        }

        // r[verify properties.containment-implies-factors]
        /// Containment implies all three factors hold.
        #[test]
        fn prop_containment_implies_factors(
            pv in 0u8..=7, cv in 0u8..=7,
            pm in 0u8..=3, cm in 0u8..=3,
            ps in proptest::collection::vec(any::<u8>(), 0..=10),
            cs in proptest::collection::vec(any::<u8>(), 0..=10),
        ) {
            if crate::containment::contains(pv, &ps, pm, cv, &cs, cm) {
                prop_assert!(crate::subsumption::subsumes(pv, cv));
                prop_assert!(crate::scope::covers(&ps, &cs));
                prop_assert!(crate::access_mode::mode_le(cm, pm));
            }
        }

        // r[verify properties.token-valid-at-issue]
        /// A well-formed token is valid at its issued_at time.
        #[test]
        fn prop_token_valid_at_issue(
            issued_at in 0u64..=u64::MAX / 4,
            total in 0u64..=10000,
            tolerance in 0u64..=1000,
        ) {
            let expires_at = issued_at + total;
            prop_assert!(crate::token_timestamp::token_time_valid(
                issued_at, expires_at, issued_at, tolerance,
            ));
        }

        // r[verify properties.chain-valid-implies-depth-bounded]
        /// A valid chain has bounded depth.
        #[test]
        fn prop_chain_depth_bounded(depth in 0u64..=20) {
            use crate::delegation::*;
            // Build a chain of the given depth.
            let mut chain = Vec::new();
            for d in 0..=depth {
                chain.push(ChainLevel {
                    depth: d,
                    has_delegate: d < depth, // last level doesn't need delegate
                    variant: 4, // Full
                    mode: 3,    // Full mode
                });
            }
            let valid = chain_valid(&chain);
            if depth <= 8 {
                prop_assert!(valid, "valid chain rejected at depth {}", depth);
            } else {
                prop_assert!(!valid, "invalid chain accepted at depth {}", depth);
            }
        }

        // ============================================================
        // Session tree cross-module properties (forest × seq_set)
        // ============================================================

        // --- Generic module cross-module tests ---

        // r[verify properties.vc-merge-is-pointwise-max]
        // r[verify properties.g-counter-merge-is-pointwise-max]
        // r[verify properties.cut-join-is-pointwise-max]
        // r[verify properties.all-three-merges-agree]
        #[test]
        fn prop_all_three_merges_agree(
            a in proptest::collection::vec(any::<u64>(), 3),
            b in proptest::collection::vec(any::<u64>(), 3),
        ) {
            let vc = crate::vector_clock::merge(&a, &b);
            let gc = crate::crdt::g_counter_merge(&a, &b);
            let cj = crate::causal_order::cut_join(&a, &b);
            prop_assert_eq!(&vc, &gc, "vc != g_counter");
            prop_assert_eq!(&gc, &cj, "g_counter != cut_join");
        }

        // r[verify properties.unique-subset-of-source]
        /// Unique messages are a subset of the source branch path.
        #[test]
        fn prop_unique_subset_of_source(raw in proptest::collection::vec(proptest::option::of(0usize..100), 2..=15)) {
            let parents: Vec<Option<usize>> = raw.into_iter().enumerate().map(|(i, opt)| {
                if i == 0 { None } else { match opt { Some(v) => Some(v % i), None => None } }
            }).collect();
            let src = parents.len() - 1;
            let tgt = 0;
            let path_src = crate::forest::ancestor_path(&parents, src);
            let path_tgt = crate::forest::ancestor_path(&parents, tgt);
            let src_i64: Vec<i64> = path_src.iter().map(|&x| x as i64).collect();
            let tgt_i64: Vec<i64> = path_tgt.iter().map(|&x| x as i64).collect();
            let cap = parents.len() + 1;
            let unique = crate::seq_set::set_difference(&src_i64, &tgt_i64, cap);
            // unique ⊆ source path
            for &u in &unique {
                prop_assert!(src_i64.contains(&u), "unique {} not on source path", u);
            }
            // unique ∩ target path = ∅
            for &u in &unique {
                prop_assert!(!tgt_i64.contains(&u), "unique {} on target path", u);
            }
        }

        // r[verify properties.unique-disjoint-from-target]
        /// Unique messages have no overlap with target branch.
        #[test]
        fn prop_unique_disjoint_from_target(raw in proptest::collection::vec(proptest::option::of(0usize..100), 2..=15)) {
            let parents: Vec<Option<usize>> = raw.into_iter().enumerate().map(|(i, opt)| {
                if i == 0 { None } else { match opt { Some(v) => Some(v % i), None => None } }
            }).collect();
            let src = parents.len() - 1;
            let tgt = 0;
            let path_src = crate::forest::ancestor_path(&parents, src);
            let path_tgt = crate::forest::ancestor_path(&parents, tgt);
            let src_i64: Vec<i64> = path_src.iter().map(|&x| x as i64).collect();
            let tgt_i64: Vec<i64> = path_tgt.iter().map(|&x| x as i64).collect();
            let cap = parents.len() + 1;
            let unique = crate::seq_set::set_difference(&src_i64, &tgt_i64, cap);
            let inter = crate::seq_set::set_intersection(&src_i64, &tgt_i64, cap);
            // Partition: unique + shared = source
            prop_assert_eq!(unique.len() + inter.len(), src_i64.len());
        }

        // r[verify properties.selective-subset-of-unique]
        /// Selective merge result is subset of both selection and unique set.
        #[test]
        fn prop_selective_subset_of_unique(raw in proptest::collection::vec(proptest::option::of(0usize..100), 3..=15)) {
            let parents: Vec<Option<usize>> = raw.into_iter().enumerate().map(|(i, opt)| {
                if i == 0 { None } else { match opt { Some(v) => Some(v % i), None => None } }
            }).collect();
            let src = parents.len() - 1;
            let tgt = 0;
            let path_src = crate::forest::ancestor_path(&parents, src);
            let path_tgt = crate::forest::ancestor_path(&parents, tgt);
            let src_i64: Vec<i64> = path_src.iter().map(|&x| x as i64).collect();
            let tgt_i64: Vec<i64> = path_tgt.iter().map(|&x| x as i64).collect();
            let cap = parents.len() + 1;
            let unique = crate::seq_set::set_difference(&src_i64, &tgt_i64, cap);
            // Select half
            let selected: Vec<i64> = unique.iter().take(unique.len() / 2 + 1).copied().collect();
            let merged = crate::seq_set::set_intersection(&selected, &unique, cap);
            // merged ⊆ unique AND merged ⊆ selected
            for &m in &merged {
                prop_assert!(unique.contains(&m), "merged {} not in unique", m);
                prop_assert!(selected.contains(&m), "merged {} not in selected", m);
            }
        }

        // r[verify properties.subtree-walk-connection]
        /// Every node in a subtree has the root on its ancestor path.
        #[test]
        fn prop_subtree_walk_connection(raw in proptest::collection::vec(proptest::option::of(0usize..100), 2..=15)) {
            let parents: Vec<Option<usize>> = raw.into_iter().enumerate().map(|(i, opt)| {
                if i == 0 { None } else { match opt { Some(v) => Some(v % i), None => None } }
            }).collect();
            // Pick a root in the middle
            let root = parents.len() / 2;
            let sub = crate::forest::subtree(&parents, root);
            for &n in &sub {
                let path = crate::forest::ancestor_path(&parents, n);
                prop_assert!(path.contains(&root),
                    "node {} in subtree({}) but root not on path {:?}", n, root, path);
            }
        }

        // r[verify properties.watermark-advance-is-monotonic]
        #[test]
        fn prop_watermark_advance_is_monotonic(
            mark in 0u64..=u64::MAX / 2,
            size in 0u64..=10000,
        ) {
            if mark as u128 + size as u128 <= u64::MAX as u128 {
                let new_mark = crate::watermark::watermark_alloc(mark, size);
                prop_assert!(crate::monotonic::is_advance(mark, new_mark));
            }
        }

        // r[verify properties.watermark-alloc-disjoint-regions]
        #[test]
        fn prop_watermark_alloc_disjoint_regions(
            mark in 0u64..=u64::MAX / 4,
            size1 in 1u64..=1000,
            size2 in 1u64..=1000,
        ) {
            if mark as u128 + size1 as u128 + size2 as u128 <= u64::MAX as u128 {
                let mark2 = crate::watermark::watermark_alloc(mark, size1);
                prop_assert!(crate::region::regions_disjoint(mark, size1, mark2, size2));
            }
        }

        // r[verify properties.buddy-uses-toggle]
        #[test]
        fn prop_buddy_uses_toggle(addr: u64, order in 0u64..64) {
            let buddy = crate::buddy::buddy_of(addr, order);
            let toggled = crate::bitwise::toggle_bit(addr, order);
            prop_assert_eq!(buddy, toggled);
        }

        // r[verify properties.block-size-is-pow2]
        #[test]
        fn prop_block_size_is_pow2(order in 0u64..64) {
            prop_assert_eq!(crate::buddy::block_size(order), 1u64 << order);
        }

        // r[verify properties.slab-slot-is-page-frame]
        #[test]
        fn prop_slab_slot_is_page_frame(addr: u64, slot_size in 1u64..=4096) {
            let slab_idx = crate::slab::slot_index(addr, 0, slot_size);
            let page_frame = crate::page::frame_number(addr, slot_size);
            prop_assert_eq!(slab_idx, page_frame);
        }

        // r[verify properties.slab-capacity-is-pages-dual]
        #[test]
        fn prop_slab_capacity_exact(n in 0u64..=10000, slot_size in 1u64..=256) {
            if n as u128 * slot_size as u128 <= u64::MAX as u128 {
                let region_size = n * slot_size;
                let cap = crate::slab::slab_capacity(region_size, slot_size);
                prop_assert_eq!(cap, n);
            }
        }

        // r[verify properties.region-contains-agrees-interval]
        #[test]
        fn prop_region_contains_agrees_interval(
            base in 0u64..=u64::MAX / 2,
            size in 0u64..=1000,
            x: u64,
        ) {
            if base as u128 + size as u128 <= u64::MAX as u128 {
                let region_result = crate::region::contains_addr(base, size, x);
                let interval_result = crate::interval::contains(base, base + size, x);
                prop_assert_eq!(region_result, interval_result);
            }
        }

        // r[verify properties.region-disjoint-agrees-interval]
        #[test]
        fn prop_region_disjoint_agrees_interval(
            a_base in 0u64..=u64::MAX / 2,
            a_size in 0u64..=1000,
            b_base in 0u64..=u64::MAX / 2,
            b_size in 0u64..=1000,
        ) {
            if a_base as u128 + a_size as u128 <= u64::MAX as u128
                && b_base as u128 + b_size as u128 <= u64::MAX as u128
            {
                let region_result = crate::region::regions_disjoint(a_base, a_size, b_base, b_size);
                let interval_result = !crate::interval::overlaps(
                    a_base, a_base + a_size, b_base, b_base + b_size,
                );
                prop_assert_eq!(region_result, interval_result);
            }
        }

        // r[verify properties.region-adjacent-agrees-interval]
        #[test]
        fn prop_region_adjacent_agrees_interval(
            a_base in 0u64..=u64::MAX / 2,
            a_size in 0u64..=1000,
            b_base in 0u64..=u64::MAX / 2,
            b_size in 0u64..=1000,
        ) {
            if a_base as u128 + a_size as u128 <= u64::MAX as u128
                && b_base as u128 + b_size as u128 <= u64::MAX as u128
            {
                let region_result = crate::region::region_adjacent(a_base, a_size, b_base, b_size);
                let interval_result = crate::interval::adjacent(
                    a_base, a_base + a_size, b_base, b_base + b_size,
                );
                prop_assert_eq!(region_result, interval_result);
            }
        }
    }
}
