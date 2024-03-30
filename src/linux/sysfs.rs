//! Functions for interacting with the Linux kernel sysfs.

#[cfg(test)]
use std::cell::OnceCell;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::{fs, io};

/// Write to a kernel attribute.
pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    #[cfg(not(test))]
    let path = Path::new(SYSFS_ROOT_DIR).join(path);

    #[cfg(test)]
    let path = SYSFS_ROOT_DIR.with(|cell| {
        cell.get()
            .expect("`SYSFS_ROOT_DIR` should be set by test")
            .join(path)
    });

    fs::write(path, contents)
}

#[cfg(not(test))]
const SYSFS_ROOT_DIR: &str = "/sys";
#[cfg(test)]
thread_local!(static SYSFS_ROOT_DIR: OnceCell<PathBuf> = OnceCell::new());

#[cfg(test)]
mod tests {
    use super::*;

    mod write {
        use super::*;
        use crate::unix::temporary_directory::TemporaryDirectory;

        #[test]
        fn it_should_write_to_an_attribute() {
            let temp_dir = TemporaryDirectory::new().expect("should succeed");

            SYSFS_ROOT_DIR.with(|cell| {
                cell.set(temp_dir.path().into())
                    .expect("cell should be empty");
            });

            let path = "foo";
            let contents = "bar";
            let attribute_path = temp_dir.path().join(path);
            fs::write(&attribute_path, "").expect("parent directory should exist and be writable");
            write(path, contents).expect("attribute should exist and be writable");
            assert!(fs::read_to_string(attribute_path)
                .is_ok_and(|attribute_contents| attribute_contents == contents),);
        }
    }
}
