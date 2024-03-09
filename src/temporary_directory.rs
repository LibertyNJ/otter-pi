use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Default for TemporaryDirectory {
    fn default() -> Self {
        let mut path = env::temp_dir();
        path.push("tHiS dIr3cT0rY 1S uN1Ik3Ly T0 3x15t");
        Self { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_path_that_begins_with_the_system_temporary_directory() {
        let temp_dir = TemporaryDirectory::new();
        assert!(temp_dir.get_path().starts_with(env::temp_dir()));
    }

    #[test]
    fn it_should_return_a_path_that_does_not_exist_after_it_goes_out_of_scope() {
        let path = TemporaryDirectory::new().get_path().to_owned();
        assert!(path.try_exists().is_ok_and(|exists| !exists));
    }
}
