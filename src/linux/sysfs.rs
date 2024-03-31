//! Functions for interacting with the Linux kernel sysfs.

#[cfg(test)]
use std::cell::OnceCell;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::{fs, io};

/// Read from a kernel attribute.
pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = get_sysfs_path(path);
    fs::read(path)
}

/// Read from a kernel attribute into a [`String`].
pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    let path = get_sysfs_path(path);
    fs::read_to_string(path)
}

/// Write to a kernel attribute.
pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let path = get_sysfs_path(path);
    fs::write(path, contents)
}

fn get_sysfs_path(attribute_path: impl AsRef<Path>) -> PathBuf {
    #[cfg(not(test))]
    Path::new(SYSFS_ROOT_DIR).join(path);

    #[cfg(test)]
    SYSFS_ROOT_DIR.with(|cell| {
        cell.get()
            .expect("`SYSFS_ROOT_DIR` should be set by test")
            .join(attribute_path)
    })
}

#[cfg(not(test))]
const SYSFS_ROOT_DIR: &str = "/sys";
#[cfg(test)]
thread_local!(static SYSFS_ROOT_DIR: OnceCell<PathBuf> = OnceCell::new());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::temporary_directory::TemporaryDirectory;

    const CONTENTS: &str = "bar";
    const PATH: &str = "foo";

    mod read {
        use super::*;

        #[test]
        fn it_should_read_from_an_attribute() {
            let sysfs_dir = mock_sysfs_dir();
            let absolute_path = sysfs_dir.path().join(PATH);
            fs::write(absolute_path, CONTENTS)
                .expect("parent directory should exist and be writable");
            assert!(read(PATH)
                .is_ok_and(|attribute_contents| attribute_contents == CONTENTS.as_bytes()));
        }
    }

    mod read_to_string {
        use super::*;

        #[test]
        fn it_should_read_from_an_attribute() {
            let sysfs_dir = mock_sysfs_dir();
            let absolute_path = sysfs_dir.path().join(PATH);
            fs::write(absolute_path, CONTENTS)
                .expect("parent directory should exist and be writable");
            assert!(
                read_to_string(PATH).is_ok_and(|attribute_contents| attribute_contents == CONTENTS)
            );
        }
    }

    mod write {
        use super::*;

        #[test]
        fn it_should_write_to_an_attribute() {
            let sysfs_dir = mock_sysfs_dir();
            write(PATH, CONTENTS).expect("attribute should be writable");
            let absolute_path = sysfs_dir.path().join(PATH);
            assert!(fs::read_to_string(absolute_path)
                .is_ok_and(|attribute_contents| attribute_contents == CONTENTS));
        }
    }

    fn mock_sysfs_dir() -> TemporaryDirectory {
        let temp_dir = TemporaryDirectory::new().expect("should succeed");

        SYSFS_ROOT_DIR.with(|cell| {
            cell.set(temp_dir.path().into())
                .expect("cell should be empty");
        });

        temp_dir
    }
}
