//! Capability roots for repository-index tooling.
//!
//! The repository-index binary opens an operator-declared repository root and
//! index database root once, then resolves validated relative locators against
//! those handles. Traversal, absolute paths, and symlink escapes fail closed.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use cap_std::ambient_authority;
use cap_std::fs::Dir;

use crate::locator::RelativeLocator;
use crate::locator::read_directory;

/// The operator-declared repository root for repository-index tooling.
pub struct RepositoryRoot {
    dir: Dir,
}

impl RepositoryRoot {
    /// Open an operator-declared repository root as a capability handle.
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

    /// Read a repository file through the capability handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator is missing or escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn read(&self, relative: &RelativeLocator) -> io::Result<Vec<u8>> {
        self.dir.read(relative.as_str())
    }

    /// Report whether a repository locator is a directory.
    #[must_use]
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn is_dir(&self, relative: &RelativeLocator) -> bool {
        self.dir.metadata(relative.as_str()).is_ok_and(|metadata| metadata.is_dir())
    }

    /// Report whether a repository locator is a file.
    #[must_use]
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn is_file(&self, relative: &RelativeLocator) -> bool {
        self.dir.metadata(relative.as_str()).is_ok_and(|metadata| metadata.is_file())
    }

    /// List sorted entry names under a repository directory.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator is missing or escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn read_dir(&self, relative: &RelativeLocator) -> io::Result<Vec<String>> {
        read_directory(&self.dir, relative.as_str())
    }
}

/// The operator-declared root that owns the repository-index database.
pub struct IndexDatabaseRoot {
    root: PathBuf,
    dir: Dir,
}

impl IndexDatabaseRoot {
    /// Open an operator-declared index root as a capability handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the root cannot be opened as a directory.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn open(root: &Path) -> io::Result<Self> {
        Ok(Self {
            root: root.to_path_buf(),
            dir: Dir::open_ambient_dir(root, ambient_authority())?,
        })
    }

    /// Resolve the database path for a validated relative locator.
    #[must_use]
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn database_path(&self, relative: &RelativeLocator) -> PathBuf {
        self.root.join(relative.as_str())
    }

    /// Create the parent directory of a database locator through the handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn create_parent_dir(&self, relative: &RelativeLocator) -> io::Result<()> {
        let Some(parent) = Path::new(relative.as_str()).parent() else {
            return Ok(());
        };
        if parent.as_os_str().is_empty() {
            return Ok(());
        }
        self.dir.create_dir_all(parent)
    }

    /// Remove a database file through the capability handle.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator escapes the root.
    // r[impl trellis.repo_index.cap_std_boundary.root_wrappers]
    pub fn remove_file(&self, relative: &RelativeLocator) -> io::Result<()> {
        self.dir.remove_file(relative.as_str())
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
    fn positive_repository_read_stays_under_the_root() -> io::Result<()> {
        let base = tempdir()?;
        fs::create_dir_all(base.path().join("src"))?;
        fs::write(base.path().join("src/lib.rs"), b"content")?;
        let root = RepositoryRoot::open(base.path())?;
        let file = locator("src/lib.rs");
        assert_eq!(root.read(&file)?, b"content");
        assert!(root.is_file(&file));
        assert!(!root.is_dir(&file));
        assert!(root.is_dir(&locator("src")));
        assert_eq!(root.read_dir(&locator("src"))?, vec!["lib.rs".to_owned()]);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn negative_symlink_escape_fails_closed() -> io::Result<()> {
        let base = tempdir()?;
        let outside = tempdir()?;
        fs::write(outside.path().join("secret.txt"), b"secret")?;
        std::os::unix::fs::symlink(outside.path(), base.path().join("link"))?;
        let root = RepositoryRoot::open(base.path())?;
        assert!(root.read(&locator("link/secret.txt")).is_err(), "symlink escape must fail");
        Ok(())
    }

    #[test]
    fn negative_missing_roots_fail_closed() {
        let base = tempdir().expect("temp dir");
        let missing = base.path().join("missing");
        assert!(RepositoryRoot::open(&missing).is_err());
        assert!(IndexDatabaseRoot::open(&missing).is_err());
    }

    #[test]
    fn positive_index_database_root_bounds_the_database_path() -> io::Result<()> {
        let base = tempdir()?;
        let index = IndexDatabaseRoot::open(base.path())?;
        let database = locator("data/repo-index.redb");
        index.create_parent_dir(&database)?;
        assert!(base.path().join("data").is_dir());
        assert!(index.database_path(&database).starts_with(base.path()));
        fs::write(index.database_path(&database), b"db")?;
        index.remove_file(&database)?;
        assert!(!base.path().join("data/repo-index.redb").exists());
        Ok(())
    }
}
