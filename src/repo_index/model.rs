use std::fmt;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

pub const BLAKE3_DIGEST_HEX_LENGTH: usize = 64;
pub const CURRENT_SCHEMA_VERSION: u64 = 4;
pub const DEFAULT_DB_PATH: &str = "data/repo-index.redb";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoPathError {
    Empty,
    Absolute,
    NonCanonicalSegment,
}

impl fmt::Display for RepoPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("repo path must not be empty"),
            Self::Absolute => f.write_str("repo path must be relative"),
            Self::NonCanonicalSegment => f.write_str("repo path must not contain empty, `.` or `..` segments"),
        }
    }
}

impl std::error::Error for RepoPathError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct RepoPath(String);

impl RepoPath {
    // r[impl repo-index.query]
    pub fn new(value: impl Into<String>) -> Result<Self, RepoPathError> {
        let normalized = value.into().replace('\\', "/");
        if normalized.is_empty() {
            return Err(RepoPathError::Empty);
        }
        if normalized.starts_with('/') || has_windows_drive_prefix(&normalized) {
            return Err(RepoPathError::Absolute);
        }
        if normalized.split('/').any(|segment| segment.is_empty() || matches!(segment, "." | "..")) {
            return Err(RepoPathError::NonCanonicalSegment);
        }
        Ok(Self(normalized))
    }

    // r[impl repo-index.query]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for RepoPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for RepoPath {
    type Error = RepoPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for RepoPath {
    type Error = RepoPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for RepoPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blake3DigestHexError {
    InvalidLength,
    InvalidCharacter,
}

impl fmt::Display for Blake3DigestHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "BLAKE3 digest must contain {BLAKE3_DIGEST_HEX_LENGTH} hex characters"),
            Self::InvalidCharacter => f.write_str("BLAKE3 digest must contain only hexadecimal characters"),
        }
    }
}

impl std::error::Error for Blake3DigestHexError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Blake3DigestHex(String);

impl Blake3DigestHex {
    // r[impl repo-index.query]
    pub fn new(value: impl Into<String>) -> Result<Self, Blake3DigestHexError> {
        let value = value.into();
        if value.len() != BLAKE3_DIGEST_HEX_LENGTH {
            return Err(Blake3DigestHexError::InvalidLength);
        }
        if !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(Blake3DigestHexError::InvalidCharacter);
        }
        Ok(Self(value.to_ascii_lowercase()))
    }

    // r[impl repo-index.content-digest]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let digest_hex = blake3::hash(bytes).to_hex().to_string();
        debug_assert_eq!(digest_hex.len(), BLAKE3_DIGEST_HEX_LENGTH);
        Self(digest_hex)
    }

    // r[impl repo-index.query]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Blake3DigestHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Blake3DigestHex {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for Blake3DigestHex {
    type Error = Blake3DigestHexError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Blake3DigestHex {
    type Error = Blake3DigestHexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Blake3DigestHex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementIdError {
    Empty,
    ContainsWhitespace,
    ContainsBracket,
}

impl fmt::Display for RequirementIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("requirement ID must not be empty"),
            Self::ContainsWhitespace => f.write_str("requirement ID must be one token"),
            Self::ContainsBracket => f.write_str("requirement ID must not contain brackets"),
        }
    }
}

impl std::error::Error for RequirementIdError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct RequirementId(String);

impl RequirementId {
    // r[impl repo-index.cross-reference]
    // r[impl repo-index.query]
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementIdError> {
        let value = value.into();
        if value.is_empty() {
            return Err(RequirementIdError::Empty);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(RequirementIdError::ContainsWhitespace);
        }
        if value.contains(['[', ']']) {
            return Err(RequirementIdError::ContainsBracket);
        }
        Ok(Self(value))
    }

    // r[impl repo-index.query]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RequirementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for RequirementId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for RequirementId {
    type Error = RequirementIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for RequirementId {
    type Error = RequirementIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for RequirementId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExtractorKind {
    Rust,
    RequirementsDoc,
    OpenSpecMarkdown,
    Markdown,
}

impl ExtractorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::RequirementsDoc => "requirements-doc",
            Self::OpenSpecMarkdown => "openspec-markdown",
            Self::Markdown => "markdown",
        }
    }

    // r[impl repo-index.cross-reference]
    pub fn from_path(path: &RepoPath) -> Self {
        if path.as_str().starts_with("src/") && has_extension(path.as_str(), "rs") {
            Self::Rust
        } else if path.as_str() == "docs/requirements.md" {
            Self::RequirementsDoc
        } else if path.as_str().starts_with("openspec/") && has_extension(path.as_str(), "md") {
            Self::OpenSpecMarkdown
        } else {
            Self::Markdown
        }
    }

    // r[impl repo-index.incremental-refresh]
    pub fn extraction_key(self, digest_hex: &Blake3DigestHex) -> String {
        format!("{}:{digest_hex}", self.as_str())
    }
}

impl fmt::Display for ExtractorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolKind {
    Module,
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Const,
    Static,
    Function,
    ProofFunction,
    SpecFunction,
}

impl SymbolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::TypeAlias => "type-alias",
            Self::Const => "const",
            Self::Static => "static",
            Self::Function => "function",
            Self::ProofFunction => "proof-function",
            Self::SpecFunction => "spec-function",
        }
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SymbolFact {
    pub name: String,
    pub kind: SymbolKind,
    pub line: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequirementReference {
    pub id: RequirementId,
    pub relation: String,
    pub line: u32,
    pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequirementDefinition {
    pub id: RequirementId,
    pub section: Option<String>,
    pub line: u32,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HeadingFact {
    pub level: u8,
    pub line: u32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExtractedFacts {
    pub symbols: Vec<SymbolFact>,
    pub requirement_refs: Vec<RequirementReference>,
    pub requirement_defs: Vec<RequirementDefinition>,
    pub headings: Vec<HeadingFact>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OpenSpecContext {
    pub change_name: Option<String>,
    pub domain: Option<String>,
    pub document_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathRow {
    pub path: RepoPath,
    pub digest_hex: Blake3DigestHex,
    pub extractor_kind: ExtractorKind,
    pub size_bytes: u64,
    pub openspec: Option<OpenSpecContext>,
}

impl PathRow {
    // r[impl repo-index.incremental-refresh]
    pub fn extraction_key(&self) -> String {
        self.extractor_kind.extraction_key(&self.digest_hex)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GenerationStats {
    pub tracked_paths: u64,
    pub unique_digests: u64,
    pub symbol_count: u64,
    pub requirement_reference_count: u64,
    pub requirement_definition_count: u64,
    pub heading_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshSummary {
    pub generation: u64,
    pub tracked_paths: u64,
    pub inserted_extractions: u64,
    pub reused_extractions: u64,
    pub added_paths: u64,
    pub changed_paths: u64,
    pub unchanged_paths: u64,
    pub deleted_paths: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathQueryResult {
    pub row: PathRow,
    pub facts: ExtractedFacts,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolHit {
    pub path: RepoPath,
    pub digest_hex: Blake3DigestHex,
    pub symbol: SymbolFact,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequirementDefinitionHit {
    pub path: RepoPath,
    pub digest_hex: Blake3DigestHex,
    pub definition: RequirementDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequirementReferenceHit {
    pub path: RepoPath,
    pub digest_hex: Blake3DigestHex,
    pub reference: RequirementReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequirementQueryResult {
    pub definitions: Vec<RequirementDefinitionHit>,
    pub references: Vec<RequirementReferenceHit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestFactsView {
    pub extractor_kind: ExtractorKind,
    pub facts: ExtractedFacts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestQueryResult {
    pub digest_hex: Blake3DigestHex,
    pub paths: Vec<PathRow>,
    pub facts_by_kind: Vec<DigestFactsView>,
}

fn has_windows_drive_prefix(path: &str) -> bool {
    matches!(path.as_bytes(), [drive, b':', ..] if drive.is_ascii_alphabetic())
}

fn has_extension(path: &str, extension: &str) -> bool {
    Path::new(path).extension().is_some_and(|value| value.eq_ignore_ascii_case(extension))
}

#[cfg(test)]
mod tests {
    use super::BLAKE3_DIGEST_HEX_LENGTH;
    use super::Blake3DigestHex;
    use super::Blake3DigestHexError;
    use super::RepoPath;
    use super::RepoPathError;
    use super::RequirementId;
    use super::RequirementIdError;

    #[test]
    fn repo_path_normalizes_platform_separators() {
        let path = RepoPath::new(r"src\repo_index\model.rs").expect("valid repo path");
        assert_eq!(path.as_str(), "src/repo_index/model.rs");
    }

    #[test]
    fn repo_path_rejects_non_relative_or_non_canonical_values() {
        assert_eq!(RepoPath::new(""), Err(RepoPathError::Empty));
        assert_eq!(RepoPath::new("/src/lib.rs"), Err(RepoPathError::Absolute));
        assert_eq!(RepoPath::new(r"C:\src\lib.rs"), Err(RepoPathError::Absolute));
        assert_eq!(RepoPath::new("src/./lib.rs"), Err(RepoPathError::NonCanonicalSegment));
        assert_eq!(RepoPath::new("src/../lib.rs"), Err(RepoPathError::NonCanonicalSegment));
        assert_eq!(RepoPath::new("src//lib.rs"), Err(RepoPathError::NonCanonicalSegment));
    }

    #[test]
    fn blake3_digest_canonicalizes_valid_hex() {
        let uppercase = "A".repeat(BLAKE3_DIGEST_HEX_LENGTH);
        let digest = Blake3DigestHex::new(uppercase).expect("valid BLAKE3 digest");
        assert_eq!(digest.as_str(), "a".repeat(BLAKE3_DIGEST_HEX_LENGTH));
    }

    #[test]
    fn blake3_digest_rejects_invalid_length_and_characters() {
        let short = "a".repeat(BLAKE3_DIGEST_HEX_LENGTH - 1);
        assert_eq!(Blake3DigestHex::new(short), Err(Blake3DigestHexError::InvalidLength));

        let invalid = format!("{}g", "a".repeat(BLAKE3_DIGEST_HEX_LENGTH - 1));
        assert_eq!(Blake3DigestHex::new(invalid), Err(Blake3DigestHexError::InvalidCharacter));
    }

    #[test]
    fn requirement_id_accepts_one_token() {
        let requirement_id = RequirementId::new("repo-index.query").expect("valid requirement ID");
        assert_eq!(requirement_id.as_str(), "repo-index.query");
    }

    #[test]
    fn requirement_id_rejects_invalid_tokens() {
        assert_eq!(RequirementId::new(""), Err(RequirementIdError::Empty));
        assert_eq!(RequirementId::new("repo index.query"), Err(RequirementIdError::ContainsWhitespace));
        assert_eq!(RequirementId::new("repo-index[query"), Err(RequirementIdError::ContainsBracket));
    }

    #[test]
    fn newtypes_keep_string_serialization_shape() {
        let path = RepoPath::new("src/lib.rs").expect("valid repo path");
        assert_eq!(serde_json::to_string(&path).expect("serialize repo path"), r#""src/lib.rs""#);

        let digest = Blake3DigestHex::new("a".repeat(BLAKE3_DIGEST_HEX_LENGTH)).expect("valid BLAKE3 digest");
        assert_eq!(
            serde_json::to_string(&digest).expect("serialize BLAKE3 digest"),
            format!(r#""{}""#, "a".repeat(BLAKE3_DIGEST_HEX_LENGTH))
        );

        let requirement_id = RequirementId::new("repo-index.query").expect("valid requirement ID");
        assert_eq!(serde_json::to_string(&requirement_id).expect("serialize requirement ID"), r#""repo-index.query""#);
    }

    #[test]
    fn invalid_serialized_values_do_not_bypass_constructors() {
        let digest = serde_json::from_str::<Blake3DigestHex>(r#""not-a-digest""#);
        assert!(digest.is_err());

        let path = serde_json::from_str::<RepoPath>(r#""../src/lib.rs""#);
        assert!(path.is_err());

        let requirement_id = serde_json::from_str::<RequirementId>(r#""repo index.query""#);
        assert!(requirement_id.is_err());

        assert!(serde_json::from_str::<RepoPath>("null").is_err());
        assert!(serde_json::from_str::<Blake3DigestHex>("null").is_err());
        assert!(serde_json::from_str::<RequirementId>("null").is_err());
    }
}
