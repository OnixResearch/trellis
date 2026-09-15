//! Capability roots for proof-evidence tooling.
//!
//! The proof-evidence binary opens an operator-declared input root and
//! artifact output root once, then resolves validated relative locators against
//! those handles. Traversal, absolute paths, and symlink escapes fail closed.

use std::io;
use std::path::Path;

use cap_std::ambient_authority;
use cap_std::fs::Dir;

use crate::locator::RelativeLocator;
use crate::locator::read_directory;

/// The operator-declared input root for proof-evidence tooling.
pub struct ProofEvidenceInputRoot {
    dir: Dir,
}

impl ProofEvidenceInputRoot {
    /// Open an operator-declared proof-evidence input root.
    ///
    /// # Errors
    ///
    /// Returns an error when the root cannot be opened as a directory.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn open(root: &Path) -> io::Result<Self> {
        Ok(Self {
            dir: Dir::open_ambient_dir(root, ambient_authority())?,
        })
    }

    /// Read an input file as text through the capability handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator is missing or escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn read_to_string(&self, relative: &RelativeLocator) -> io::Result<String> {
        self.dir.read_to_string(relative.as_str())
    }

    /// List sorted entry names under an input directory.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator is missing or escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn read_dir(&self, relative: &RelativeLocator) -> io::Result<Vec<String>> {
        read_directory(&self.dir, relative.as_str())
    }

    /// Report whether an input locator is a directory.
    #[must_use]
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn is_dir(&self, relative: &RelativeLocator) -> bool {
        self.dir.metadata(relative.as_str()).is_ok_and(|metadata| metadata.is_dir())
    }
}

/// The operator-declared output root for proof-evidence artifacts.
pub struct ArtifactOutputRoot {
    dir: Dir,
}

impl ArtifactOutputRoot {
    /// Open an operator-declared artifact output root.
    ///
    /// # Errors
    ///
    /// Returns an error when the root cannot be opened as a directory.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn open(root: &Path) -> io::Result<Self> {
        Ok(Self {
            dir: Dir::open_ambient_dir(root, ambient_authority())?,
        })
    }

    /// Write artifact bytes through the capability handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn write(&self, relative: &RelativeLocator, bytes: &[u8]) -> io::Result<()> {
        self.dir.write(relative.as_str(), bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn locator(value: &str) -> RelativeLocator {
        RelativeLocator::new(value).expect("valid locator")
    }

    #[test]
    fn positive_input_read_and_output_write_stay_under_the_roots() -> io::Result<()> {
        let base = tempdir()?;
        fs::write(base.path().join("requirements.md"), b"r[example.rule]")?;
        let input = ProofEvidenceInputRoot::open(base.path())?;
        let requirements = locator("requirements.md");
        assert!(!input.is_dir(&requirements));
        assert_eq!(input.read_to_string(&requirements)?, "r[example.rule]");

        let output = ArtifactOutputRoot::open(base.path())?;
        output.write(&locator("bundle.json"), b"{}")?;
        assert_eq!(fs::read(base.path().join("bundle.json"))?, b"{}");
        Ok(())
    }

    #[test]
    fn negative_missing_roots_and_escape_locators_fail_closed() {
        let base = tempdir().expect("temp dir");
        let missing = base.path().join("missing");
        assert!(ProofEvidenceInputRoot::open(&missing).is_err());
        assert!(ArtifactOutputRoot::open(&missing).is_err());
        assert!(RelativeLocator::new("../escape.json").is_err());
        assert!(RelativeLocator::new("/absolute.json").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn negative_input_symlink_escape_fails_closed() -> io::Result<()> {
        let base = tempdir()?;
        let outside = tempdir()?;
        fs::write(outside.path().join("secret.md"), b"secret")?;
        std::os::unix::fs::symlink(outside.path(), base.path().join("link"))?;
        let input = ProofEvidenceInputRoot::open(base.path())?;
        assert!(input.read_to_string(&locator("link/secret.md")).is_err(), "symlink escape must fail");
        Ok(())
    }
}
