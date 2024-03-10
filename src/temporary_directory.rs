//! Abstractions to make managing temporary directories easier.

use std::ffi::{c_char, CString, OsString};
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::{env, error, fs, io};

extern "C" {
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
}

/// A secure, uniquely-named temporary directory.
///
/// This is an RAII construct that automatically initializes and finalizes a
/// temporary directory bound by the lifetime of the object.
///
/// # Examples
///
/// ```
/// use otter_pi::temporary_directory::TemporaryDirectory;
///
/// let path = {
///     let temp_dir = TemporaryDirectory::new().unwrap();
///     assert!(temp_dir.get_path().is_dir());
///     temp_dir.get_path().to_owned()
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
    /// The path to the underlying temporary directory is based on the system's
    /// temporary directory path composed with a random string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    ///
    /// use otter_pi::temporary_directory::TemporaryDirectory;
    ///
    /// let temp_dir = TemporaryDirectory::new().unwrap();
    /// assert!(temp_dir.get_path().starts_with(env::temp_dir()));
    /// ```
    /// # Errors
    ///
    /// This function will return an error if the internal path template derived
    /// from the system temporary directory path contains a nul byte, or if it fails
    /// to create the underlying temporary directory for any reason.
    #[cfg(unix)]
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut template = env::temp_dir();
        template.push("XXXXXX");
        let template = CString::new(template.into_os_string().into_vec())?;
        let template = template.into_raw();
        let ptr = unsafe { mkdtemp(template) };
        let error = io::Error::last_os_error();
        let path = unsafe { CString::from_raw(template) };

        if ptr.is_null() {
            Err(error)?
        } else {
            let path = PathBuf::from(OsString::from_vec(path.into_bytes()));
            Ok(Self { path })
        }
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
    /// use otter_pi::temporary_directory::TemporaryDirectory;
    ///
    /// let temp_dir = TemporaryDirectory::new().unwrap();
    /// let file_path = temp_dir.get_path().join("foo");
    /// assert!(fs::write(&file_path, "bar").is_ok());
    /// assert!(fs::read_to_string(&file_path).is_ok_and(|content| content == "bar"));
    /// ```
    #[must_use]
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_path_that_begins_with_the_system_temporary_directory() {
        let temp_dir = TemporaryDirectory::new().unwrap();
        assert!(temp_dir.get_path().starts_with(env::temp_dir()));
    }

    #[test]
    fn it_should_return_a_path_to_an_accessible_directory() {
        let temp_dir = TemporaryDirectory::new().unwrap();
        assert!(temp_dir.get_path().is_dir());
    }

    #[test]
    fn it_should_return_a_path_that_does_not_exist_after_it_goes_out_of_scope() {
        let path = TemporaryDirectory::new().unwrap().get_path().to_owned();
        assert!(path.try_exists().is_ok_and(|exists| !exists));
    }
}
