//! Conversions that are only supported on Unix-like operating systems.

use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

/// Converts a [`CString`] into a [`PathBuf`].
#[must_use]
pub fn c_string_to_path_buf(c_string: CString) -> PathBuf {
    PathBuf::from(OsString::from_vec(c_string.into_bytes()))
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
}
