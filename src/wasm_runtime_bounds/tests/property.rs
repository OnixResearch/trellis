use proptest::prelude::*;

use super::*;

const MAX_PROP_FUEL: u32 = 64;
const MAX_PROP_SEGMENT: u32 = 3;
const MAX_PROP_CHUNKS: usize = 16;
const MIN_PROP_CHUNK_LENGTH: u16 = 1;
const MAX_PROP_CHUNK_LENGTH: u16 = 128;

proptest! {
    // r[verify wasm-runtime-bounds.evidence]
    // r[verify wasm-runtime-bounds.fuel]
    #[test]
    fn prop_fuel_consumption_is_exact_or_unchanged(
        remaining in 0u32..=MAX_PROP_FUEL,
        requested in 0u32..=MAX_PROP_FUEL,
    ) {
        let fuel = Fuel { remaining };
        match consume_fuel(fuel, requested) {
            Ok(after) => {
                prop_assert!(requested <= remaining);
                prop_assert_eq!(after.remaining, remaining - requested);
            }
            Err(fault) => {
                prop_assert!(requested > remaining);
                prop_assert_eq!(fault.available, fuel);
            }
        }
    }

    // r[verify wasm-runtime-bounds.evidence]
    // r[verify wasm-runtime-bounds.segmentation]
    #[test]
    fn prop_deterministic_segments_match(
        fuel in 0u32..=MAX_PROP_FUEL,
        prefix in 0u32..=MAX_PROP_SEGMENT,
        suffix in 0u32..=MAX_PROP_SEGMENT,
    ) {
        let model = linear_model();
        let input = observations();
        let result = compare_segments(SegmentRequest {
            model: &model,
            resume_model: &model,
            start: running_cursor(fuel),
            observations: &input,
            resume_observations: &input,
            prefix_budget: prefix,
            suffix_budget: suffix,
        });
        prop_assert!(result.is_ok());
    }

    // r[verify wasm-runtime-bounds.evidence]
    // r[verify wasm-runtime-bounds.streaming]
    #[test]
    fn prop_positive_partitions_cover_each_byte_once(
        lengths in prop::collection::vec(
            MIN_PROP_CHUNK_LENGTH..=MAX_PROP_CHUNK_LENGTH,
            1..=MAX_PROP_CHUNKS,
        ),
    ) {
        let mut offset = 0u64;
        let mut chunks = Vec::with_capacity(lengths.len());
        for length in lengths {
            let length = u64::from(length);
            chunks.push(Chunk { offset, length });
            offset += length;
        }
        let schedule = ChunkSchedule {
            total_bytes: offset,
            chunks,
        };
        prop_assert_eq!(admit_schedule(&schedule), Ok(()));
        for byte_offset in 0..schedule.total_bytes {
            prop_assert_eq!(coverage_count(&schedule, byte_offset), Some(1));
        }
    }
}
