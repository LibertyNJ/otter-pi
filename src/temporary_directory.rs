use std::path::{Path, PathBuf};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_path_that_does_not_exist_after_it_goes_out_of_scope() {
        let path = TemporaryDirectory::new().get_path().to_owned();
        assert!(path.try_exists().is_ok_and(|exists| !exists));
    }
}
