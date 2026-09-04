//! Exact ordered chunk-schedule admission.

/// Maximum number of chunks in one schedule.
pub const MAX_CHUNKS: u32 = 4_096;
/// Maximum declared source length in bytes.
pub const MAX_STREAM_BYTES: u64 = u64::MAX - 1;

/// One half-open source-byte span.
// r[impl wasm-runtime-bounds.streaming]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub offset: u64,
    pub length: u64,
}

/// One complete source partition request.
// r[impl wasm-runtime-bounds.streaming]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkSchedule {
    pub total_bytes: u64,
    pub chunks: Vec<Chunk>,
}

/// Stable schedule rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScheduleFault {
    SourceTooLarge,
    TooManyChunks,
    MissingChunks,
    UnexpectedEmptyInputChunk,
    ZeroLength,
    OffsetMismatch,
    RangeOverflow,
    BeyondDeclaredEnd,
    FinalLengthMismatch,
}

/// Deterministic schedule rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScheduleFaultSite {
    pub class: ScheduleFault,
    pub index: Option<u32>,
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject(class: ScheduleFault, index: Option<u32>) -> Result<(), ScheduleFaultSite> {
    Err(ScheduleFaultSite { class, index })
}

/// Admits only an exact ordered partition of the declared source bytes.
// r[impl wasm-runtime-bounds.streaming]
pub fn admit_schedule(schedule: &ChunkSchedule) -> Result<(), ScheduleFaultSite> {
    if schedule.total_bytes > MAX_STREAM_BYTES {
        return reject(ScheduleFault::SourceTooLarge, None);
    }
    let chunk_count = u32::try_from(schedule.chunks.len()).map_err(|_| ScheduleFaultSite {
        class: ScheduleFault::TooManyChunks,
        index: None,
    })?;
    if chunk_count > MAX_CHUNKS {
        return reject(ScheduleFault::TooManyChunks, None);
    }
    if schedule.total_bytes == 0 {
        return if schedule.chunks.is_empty() {
            Ok(())
        } else {
            reject(ScheduleFault::UnexpectedEmptyInputChunk, Some(0))
        };
    }
    if schedule.chunks.is_empty() {
        return reject(ScheduleFault::MissingChunks, None);
    }

    let mut next_offset = 0;
    for (index, chunk) in schedule.chunks.iter().enumerate() {
        let at = position(index);
        if chunk.length == 0 {
            return reject(ScheduleFault::ZeroLength, at);
        }
        if chunk.offset != next_offset {
            return reject(ScheduleFault::OffsetMismatch, at);
        }
        let Some(end) = chunk.offset.checked_add(chunk.length) else {
            return reject(ScheduleFault::RangeOverflow, at);
        };
        if end > schedule.total_bytes {
            return reject(ScheduleFault::BeyondDeclaredEnd, at);
        }
        next_offset = end;
    }
    if next_offset != schedule.total_bytes {
        return reject(ScheduleFault::FinalLengthMismatch, None);
    }
    Ok(())
}

/// Counts the admitted spans that contain one source-byte offset.
// r[impl wasm-runtime-bounds.streaming]
#[must_use]
pub fn coverage_count(schedule: &ChunkSchedule, byte_offset: u64) -> Option<u32> {
    admit_schedule(schedule).ok()?;
    let count = schedule
        .chunks
        .iter()
        .filter(|chunk| {
            chunk
                .offset
                .checked_add(chunk.length)
                .is_some_and(|end| chunk.offset <= byte_offset && byte_offset < end)
        })
        .count();
    u32::try_from(count).ok()
}
