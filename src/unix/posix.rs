//! Features that are dependent on system conformance to POSIX standards.

use std::ffi::{c_char, CString, NulError};
use std::path::PathBuf;
use std::{env, io};

use super::convert;

extern "C" {
    /// Securely creates a directory with a unique name derived from `template`.
    ///
    /// `template` must be a string representing the desired path ending with at
    /// least six trailing "X" characters. Six or more of the trailing "X"
    /// characters will be modified in place to create a unique name for the
    /// temporary directory. The directory is created with read, write, and execute
    /// permissions for the user only.
    ///
    /// Returns a pointer to `template` on success, or a null pointer on failure and
    /// sets `errno` to indicate the error.
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
}

/// Securely creates a uniquely-named temporary directory.
///
/// The path to the underlying temporary directory is based on the system’s
/// temporary directory path composed with a random string.
///
/// # Errors
///
/// This function will return an error if it fails to create a temporary
/// directory.
pub fn create_temp_dir() -> Result<PathBuf, io::Error> {
    let template = get_temp_dir_template()?.into_raw();
    let result = unsafe { mkdtemp(template) };
    let error = io::Error::last_os_error();
    let path = unsafe { CString::from_raw(template) };

    if result.is_null() {
        Err(error)
    } else {
        Ok(convert::c_string_to_path_buf(path))
    }
}

/// Returns a template for use with `mkdtemp`.
///
/// # Errors
///
/// This function will return an error if the system’s temporary directory path
/// contains a nul byte.
fn get_temp_dir_template() -> Result<CString, NulError> {
    let mut template = env::temp_dir();
    template.push("XXXXXX");
    convert::path_buf_to_c_string(template)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_temp_dir {
        use std::fs;

        use super::*;

        #[test]
        fn it_should_return_a_path_that_begins_with_the_system_temporary_directory() {
            let temp_dir = create_temp_dir().expect("`create_temp_dir()` should succeed");
            assert!(temp_dir.starts_with(env::temp_dir()));
            let _ = fs::remove_dir_all(temp_dir);
        }

        #[test]
        fn it_should_return_a_path_to_an_accessible_directory() {
            let temp_dir = create_temp_dir().expect("`create_temp_dir()` should succeed");
            assert!(temp_dir.is_dir());
            let _ = fs::remove_dir_all(temp_dir);
        }

        #[test]
        fn it_should_return_a_unique_path_for_each_call() {
            let temp_dir_a = create_temp_dir().expect("`create_temp_dir()` should succeed");
            let temp_dir_b = create_temp_dir().expect("`create_temp_dir()` should succeed");
            assert_ne!(temp_dir_a, temp_dir_b);

            for temp_dir in &[temp_dir_a, temp_dir_b] {
                let _ = fs::remove_dir_all(temp_dir);
            }
        }
    }
}
