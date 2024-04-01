//! Interfaces for interacting with the Linux kernel sysfs.

#[cfg(test)]
use std::cell::OnceCell;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// Interface for reading and writing to kernel attributes using paths that are
/// relative to the sysfs root directory.
pub struct Sysfs {
    path_cache: RefCell<HashMap<PathBuf, PathBuf>>,
}

impl Sysfs {
    /// Creates a new `Sysfs` interface.
    pub fn new() -> Self {
        Self {
            path_cache: RefCell::new(HashMap::new()),
        }
    }

    /// Read from a kernel attribute.
    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path_ref = self.resolve_path(path);
        fs::read(path_ref.as_path())
    }

    /// Read from a kernel attribute into a [`String`].
    pub fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
        let path_ref = self.resolve_path(path);
        fs::read_to_string(path_ref.as_path())
    }

    /// Write to a kernel attribute.
    pub fn write(&self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
        let path_ref = self.resolve_path(path);
        fs::write(path_ref.as_path(), contents)
    }

    fn cache_path(&self, attribute_path: PathBuf, path: PathBuf) {
        self.path_cache.borrow_mut().insert(attribute_path, path);
    }

    fn get_cached_path(&self, path: &Path) -> Ref<'_, PathBuf> {
        Ref::map(self.path_cache.borrow(), |cache| {
            cache.get(path).expect("path should be cached")
        })
    }

    fn has_cached_path(&self, path: &Path) -> bool {
        self.path_cache.borrow().contains_key(path)
    }

    fn resolve_attribute_path(attribute_path: &Path) -> PathBuf {
        #[cfg(not(test))]
        let path = Path::new(Self::SYSFS_ROOT_DIR).join(attribute_path);

        #[cfg(test)]
        let path = Self::SYSFS_ROOT_DIR.with(|cell| {
            cell.get()
                .expect("`SYSFS_ROOT_DIR` should be set by test")
                .join(attribute_path)
        });

        path
    }

    fn resolve_path(&self, attribute_path: impl AsRef<Path>) -> Ref<'_, PathBuf> {
        let attribute_path = attribute_path.as_ref();

        if !self.has_cached_path(attribute_path) {
            let path = Self::resolve_attribute_path(attribute_path);
            self.cache_path(attribute_path.into(), path);
        }

        self.get_cached_path(attribute_path)
    }

    #[cfg(not(test))]
    const SYSFS_ROOT_DIR: &str = "/sys";

    #[cfg(test)]
    thread_local! {
        static SYSFS_ROOT_DIR: OnceCell<PathBuf> = const { OnceCell::new() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::temporary_directory::TemporaryDirectory;

    #[test]
    fn it_should_read_from_an_attribute() {
        let _sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        assert!(sysfs
            .read("class/pwm/pwmchip0/npwm")
            .is_ok_and(|contents| contents == NPWM.as_bytes()));
    }

    #[test]
    fn it_should_return_an_error_when_reading_from_a_device_that_does_not_exist() {
        let _sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        assert!(sysfs.read("class/pwm/pwmchip1/npwm").is_err());
    }

    #[test]
    fn it_should_read_from_an_attribute_to_a_string() {
        let _sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        assert!(sysfs
            .read_to_string("class/pwm/pwmchip0/npwm")
            .is_ok_and(|contents| contents == NPWM));
    }

    #[test]
    fn it_should_return_an_error_when_reading_to_a_string_from_a_device_that_does_not_exist() {
        let _sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        assert!(sysfs.read_to_string("class/pwm/pwmchip1/npwm").is_err());
    }

    #[test]
    fn it_should_write_to_an_attribute() {
        let sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        let channel = "0";
        sysfs
            .write("class/pwm/pwmchip0/export", channel)
            .expect("attribute should exist and be writable");
        let path = sysfs_dir.path().join("class/pwm/pwmchip0/export");
        assert!(fs::read_to_string(path).is_ok_and(|contents| contents == channel));
    }

    #[test]
    fn it_should_return_an_error_when_writing_to_an_attribute_for_a_device_that_does_not_exist() {
        let _sysfs_dir = mock_sysfs_dir();
        let sysfs = Sysfs::new();
        assert!(sysfs.write("class/pwm/pwmchip1/export", "0").is_err());
    }

    fn mock_sysfs_dir() -> TemporaryDirectory {
        let sysfs_dir = TemporaryDirectory::new().expect("should succeed");

        Sysfs::SYSFS_ROOT_DIR.with(|cell| {
            cell.set(sysfs_dir.path().into())
                .expect("`SYSFS_ROOT_DIR` should not be set");
        });

        let pwm_controller_path = sysfs_dir.path().join("class/pwm/pwmchip0");
        fs::create_dir_all(&pwm_controller_path).expect("parent directory should be writable");
        let export_path = pwm_controller_path.join("export");
        fs::write(export_path, "").expect("parent directory should exist and be writable");
        let npwm_path = pwm_controller_path.join("npwm");
        fs::write(npwm_path, NPWM).expect("parent directory should exist and be writable");
        sysfs_dir
    }

    const NPWM: &str = "1";
}
