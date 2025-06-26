use crate::ExamConfig;

use super::Args;

use std::path::PathBuf;

/// Constructors
impl Args {
    /// Creates a new Args instance from the given values.
    pub fn new<P, T>(directory: P, timeout: T, verbose: bool, dry_run: bool) -> Self
    where
        P: Into<PathBuf>,
        T: Into<u64>,
    {
        Self {
            directory: directory.into(),
            timeout: timeout.into(),
            verbose,
            dry_run,
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new(
            ExamConfig::default_base_dirname(),
            ExamConfig::default_timeout_secs(),
            false,
            false,
        )
    }
}
