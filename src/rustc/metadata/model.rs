//! Finite crate-artifact and dependency-request values.

use vstd::prelude::*;

verus! {

pub const MAX_CANDIDATES: usize = 64;
pub const NO_CANDIDATE_ID: u64 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryKind {
    Rlib,
    Dylib,
    Rmeta,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capability {
    Absent,
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsageMode {
    Check,
    Document,
    Pipeline,
    Link,
    Codegen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Artifact {
    pub candidate_id: u64,
    pub crate_name_id: u64,
    pub compiler_version_id: u64,
    pub strict_version_hash: u64,
    pub stable_crate_id: u64,
    pub kind: LibraryKind,
    pub metadata: Capability,
    pub linkable_code: Capability,
    pub encoded_mir: Capability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub crate_name_id: u64,
    pub compiler_version_id: u64,
    pub strict_version_hash: u64,
    pub stable_crate_id: u64,
    pub usage: UsageMode,
    pub encoded_mir: Capability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    Accepted,
    MissingCandidateId,
    MissingCrateName,
    MissingCompilerVersion,
    MissingStrictVersionHash,
    MissingStableCrateId,
    MissingMetadata,
    KindCapabilityMismatch,
    IncompleteRequest,
    CrateNameMismatch,
    CompilerVersionMismatch,
    StrictVersionHashMismatch,
    StableCrateIdMismatch,
    NotLinkable,
    EncodedMirMissing,
    CandidateLimitExceeded,
    NoCompatibleCandidate,
    AmbiguousCandidates,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decision {
    pub reason: Reason,
    pub candidate_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub reason: Reason,
    pub selected_candidate_id: u64,
    pub conflicting_candidate_id: u64,
}

} // verus!
