//! Validated relative locators for repository tooling capabilities.
//!
//! Only repository tooling binaries include this module. Verified-logic
//! modules, spec and exec functions, proof functions, and pure model code
//! operate on in-memory values, proof facts, content identities, or these
//! validated relative locators. They never open ambient filesystem paths.

use std::fmt;
use std::io;

use cap_std::fs::Dir;

/// Why a tooling path cannot be used as a capability-relative locator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeLocatorError {
    /// The path is empty.
    Empty,
    /// The path is absolute or carries a Windows drive prefix.
    Absolute,
    /// The path contains an empty, `.`, or `..` segment.
    NonCanonicalSegment,
}

impl std::error::Error for RelativeLocatorError {}

impl fmt::Display for RelativeLocatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("locator is empty"),
            Self::Absolute => formatter.write_str("locator is absolute"),
            Self::NonCanonicalSegment => formatter.write_str("locator has a non-canonical segment"),
        }
    }
}

/// A validated relative path for tooling filesystem operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelativeLocator(String);

impl RelativeLocator {
    /// Validate a relative locator.
    ///
    /// # Errors
    ///
    /// Returns an error when the value is empty, absolute, or contains an
    /// empty, `.`, or `..` segment.
    // r[impl trellis.repo_index.cap_std_boundary.conversion]
    pub fn new(value: impl Into<String>) -> Result<Self, RelativeLocatorError> {
        let normalized = value.into().replace('\\', "/");
        if normalized.is_empty() {
            return Err(RelativeLocatorError::Empty);
        }
        if normalized.starts_with('/') || has_windows_drive_prefix(&normalized) {
            return Err(RelativeLocatorError::Absolute);
        }
        if normalized.split('/').any(|segment| segment.is_empty() || matches!(segment, "." | "..")) {
            return Err(RelativeLocatorError::NonCanonicalSegment);
        }
        Ok(Self(normalized))
    }

    #[must_use]
    // r[impl trellis.repo_index.cap_std_boundary.conversion]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RelativeLocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

fn has_windows_drive_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

/// List sorted entry names under a directory handle.
///
/// # Errors
///
/// Returns an error when the locator is missing or escapes the root.
pub(crate) fn read_directory(dir: &Dir, relative: &str) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in dir.read_dir(relative)? {
        let entry = entry?;
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    names.sort();
    Ok(names)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_locator_parses_and_keeps_its_text() {
        let locator = RelativeLocator::new("docs/requirements.md").expect("valid locator");
        assert_eq!(locator.as_str(), "docs/requirements.md");
        assert_eq!(locator.to_string(), "docs/requirements.md");
    }

    #[test]
    fn negative_locators_never_parse() {
        assert_eq!(RelativeLocator::new(""), Err(RelativeLocatorError::Empty));
        assert_eq!(RelativeLocator::new("/etc/passwd"), Err(RelativeLocatorError::Absolute));
        assert_eq!(RelativeLocator::new("C:/windows"), Err(RelativeLocatorError::Absolute));
        assert_eq!(RelativeLocator::new("../outside"), Err(RelativeLocatorError::NonCanonicalSegment));
        assert_eq!(RelativeLocator::new("a/../../b"), Err(RelativeLocatorError::NonCanonicalSegment));
        assert_eq!(RelativeLocator::new("a//b"), Err(RelativeLocatorError::NonCanonicalSegment));
        assert_eq!(RelativeLocator::new("./a"), Err(RelativeLocatorError::NonCanonicalSegment));
    }
}
