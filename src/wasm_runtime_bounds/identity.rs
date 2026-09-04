//! Finite abstract WebAssembly runtime-bound proof primitives.
//!
//! This component owns deterministic transition, fuel, segmentation,
//! continuation, allocation, growth-policy, and byte-schedule laws. It does
//! not execute or verify a concrete interpreter, allocator, host, or sandbox.
// r[impl wasm-runtime-bounds.docs]
// r[impl wasm-runtime-bounds.evidence]
// r[impl wasm-runtime-bounds.final-validation]
// r[impl wasm-runtime-bounds.non-claims]

mod binding;
mod execution;
mod machine;
mod proofs;
mod resources;
mod streaming;

pub use binding::ArtifactId;
pub use binding::CaptureFault;
pub use binding::ContextId;
pub use binding::Continuation;
pub use binding::ContinuationReason;
pub use binding::GenerationId;
pub use binding::InputId;
pub use binding::ObservationSetId;
pub use binding::ProfileId;
pub use binding::ResumeBindings;
pub use binding::ResumeFault;
pub use binding::ResumeGrant;
pub use binding::admit_resume;
pub use binding::capture_continuation;
pub use execution::Cursor;
pub use execution::Fuel;
pub use execution::FuelFault;
pub use execution::MachineStatus;
pub use execution::RunClass;
pub use execution::RunFault;
pub use execution::RunResult;
pub use execution::SegmentRequest;
pub use execution::SegmentationFault;
pub use execution::compare_segments;
pub use execution::consume_fuel;
pub use execution::run_bounded;
pub use machine::ClaimFault;
pub use machine::ClaimScope;
pub use machine::MAX_OBSERVATIONS;
pub use machine::MAX_STATES;
pub use machine::MAX_STEPS;
pub use machine::MAX_TRANSITIONS;
pub use machine::ModelFault;
pub use machine::ModelFaultSite;
pub use machine::Observation;
pub use machine::StateId;
pub use machine::StepEffect;
pub use machine::Transition;
pub use machine::TransitionModel;
pub use machine::TrapCode;
pub use machine::admit_claim;
pub use machine::admit_model;
pub use resources::AllocationFault;
pub use resources::AllocationPhase;
pub use resources::GrowthOutcome;
pub use resources::GrowthPolicy;
pub use resources::MAX_ALLOCATED_PAGES;
pub use resources::PageAllocator;
pub use resources::allocate_pages;
pub use resources::apply_growth;
pub use resources::begin_execution;
pub use resources::finish_execution;
pub use streaming::Chunk;
pub use streaming::ChunkSchedule;
pub use streaming::MAX_CHUNKS;
pub use streaming::MAX_STREAM_BYTES;
pub use streaming::ScheduleFault;
pub use streaming::ScheduleFaultSite;
pub use streaming::admit_schedule;
pub use streaming::coverage_count;

#[cfg(test)]
mod tests;
