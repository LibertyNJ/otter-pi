//! Abstractions to make managing temporary directories easier.

use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use super::posix;

/// A secure, uniquely-named temporary directory.
///
/// This is an RAII construct that automatically initializes and finalizes a
/// temporary directory bound by the lifetime of the object.
///
/// # Examples
///
/// ```
/// use otter_pi::unix::temporary_directory::TemporaryDirectory;
///
/// let path = {
///     let temp_dir = TemporaryDirectory::new().unwrap();
///     assert!(temp_dir.path().is_dir());
///     temp_dir.path().to_owned()
/// };
///
/// assert!(path.try_exists().is_ok_and(|exists| !exists));
/// ```
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    /// Securely creates a uniquely-named temporary directory.
    ///
    /// The path to the underlying temporary directory is based on the system’s
    /// temporary directory path composed with a random string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    ///
    /// use otter_pi::unix::temporary_directory::TemporaryDirectory;
    ///
    /// let temp_dir = TemporaryDirectory::new().unwrap();
    /// assert!(temp_dir.path().starts_with(env::temp_dir()));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create a temporary
    /// directory.
    pub fn new() -> Result<Self, Error> {
        let path = posix::create_temp_dir()?;
        Ok(Self { path })
    }

    /// Returns the path to the underlying temporary directory.
    ///
    /// Can be used to compose additional paths to interact with entities within
    /// the temporary directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    ///
    /// use otter_pi::unix::temporary_directory::TemporaryDirectory;
    ///
    /// let temp_dir = TemporaryDirectory::new().unwrap();
    /// let file_path = temp_dir.path().join("foo");
    /// assert!(fs::write(&file_path, "bar").is_ok());
    /// assert!(fs::read_to_string(&file_path).is_ok_and(|content| content == "bar"));
    /// ```
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn it_should_return_a_path_that_begins_with_the_system_temporary_directory() {
        let temp_dir = TemporaryDirectory::new().unwrap();
        assert!(temp_dir.path().starts_with(env::temp_dir()));
    }

    #[test]
    fn it_should_return_a_path_to_an_accessible_directory() {
        let temp_dir = TemporaryDirectory::new().unwrap();
        assert!(temp_dir.path().is_dir());
    }

    #[test]
    fn it_should_return_a_path_that_does_not_exist_after_going_out_of_scope() {
        let path = TemporaryDirectory::new().unwrap().path().to_owned();
        assert!(path.try_exists().is_ok_and(|exists| !exists));
    }

    #[test]
    fn it_should_return_a_path_that_does_not_exist_after_adding_content_and_going_out_of_scope() {
        let path = {
            let temp_dir = TemporaryDirectory::new().unwrap();
            let file_path = temp_dir.path().join("foo");
            fs::write(file_path, "bar").unwrap();
            temp_dir.path().to_owned()
        };

        assert!(path.try_exists().is_ok_and(|exists| !exists));
    }

    #[test]
    fn it_should_return_a_unique_path_for_each_instance() {
        let temp_dir_a = TemporaryDirectory::new().unwrap();
        let temp_dir_b = TemporaryDirectory::new().unwrap();
        assert_ne!(temp_dir_a.path(), temp_dir_b.path());
    }
}
