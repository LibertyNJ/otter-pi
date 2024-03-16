//! Conversions that are only supported on Unix-like operating systems.

use std::ffi::{CString, NulError, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

/// Converts a [`CString`] into a [`PathBuf`].
pub fn c_string_to_path_buf(c_string: CString) -> PathBuf {
    PathBuf::from(OsString::from_vec(c_string.into_bytes()))
}

/// Converts a [`PathBuf`] into a [`CString`].
///
/// # Errors
///
/// This function will return an error if `path_buf` contains a nul byte.
pub fn path_buf_to_c_string(path_buf: PathBuf) -> Result<CString, NulError> {
    CString::new(path_buf.into_os_string().into_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod c_string_to_path_buf {
        use super::*;

        #[test]
        fn it_should_return_a_path_buf_representation_of_c_string() {
            let c_string = CString::new("/foo").expect("should not contain any nul bytes");
            assert!(c_string_to_path_buf(c_string) == PathBuf::from("/foo"));
        }
    }

    mod path_buf_to_c_string {
        use super::*;

        #[test]
        fn it_should_return_ok_when_path_buf_does_not_contain_a_null_byte() {
            let path_buf = PathBuf::from("/foo");
            assert!(path_buf_to_c_string(path_buf).is_ok());
        }

        #[test]
        fn it_should_return_err_when_path_buf_contains_a_null_byte() {
            let path_buf = PathBuf::from("/f\0o");
            assert!(path_buf_to_c_string(path_buf).is_err());
        }

        #[test]
        fn it_should_return_a_c_string_representation_of_path_buf() {
            let path_buf = PathBuf::from("/foo");
            assert!(path_buf_to_c_string(path_buf).is_ok_and(|c_string| c_string
                == CString::new("/foo").expect("should not contain any nul bytes")));
        }
    }
}
