use super::*;

const CANDIDATE_ID: u64 = 11;
const OTHER_CANDIDATE_ID: u64 = 12;
const THIRD_CANDIDATE_ID: u64 = 13;
const CRATE_NAME_ID: u64 = 21;
const OTHER_CRATE_NAME_ID: u64 = 22;
const COMPILER_VERSION_ID: u64 = 31;
const OTHER_COMPILER_VERSION_ID: u64 = 32;
const STRICT_VERSION_HASH: u64 = 41;
const OTHER_STRICT_VERSION_HASH: u64 = 42;
const STABLE_CRATE_ID: u64 = 51;
const OTHER_STABLE_CRATE_ID: u64 = 52;

fn artifact(kind: LibraryKind) -> Artifact {
    let linkable_code = match kind {
        LibraryKind::Rlib | LibraryKind::Dylib => Capability::Present,
        LibraryKind::Rmeta => Capability::Absent,
    };
    Artifact {
        candidate_id: CANDIDATE_ID,
        crate_name_id: CRATE_NAME_ID,
        compiler_version_id: COMPILER_VERSION_ID,
        strict_version_hash: STRICT_VERSION_HASH,
        stable_crate_id: STABLE_CRATE_ID,
        kind,
        metadata: Capability::Present,
        linkable_code,
        encoded_mir: Capability::Present,
    }
}

fn request(usage: UsageMode) -> Request {
    Request {
        crate_name_id: CRATE_NAME_ID,
        compiler_version_id: COMPILER_VERSION_ID,
        strict_version_hash: STRICT_VERSION_HASH,
        stable_crate_id: STABLE_CRATE_ID,
        usage,
        encoded_mir: Capability::Absent,
    }
}

mod negative;
mod positive;
