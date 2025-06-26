use super::Args;

use crate::{ExamConfig, ExamTester};
use std::path::PathBuf;

/// Access
impl Args {
    /// Returns the base directory to use.
    /// This is the stored directory if it is absolute,
    /// otherwise it is the stored directory relative to the current directory.
    pub fn base_dir(&self) -> PathBuf {
        let base_dir = &self.directory;
        if base_dir.is_relative() {
            PathBuf::default().join(base_dir)
        } else {
            base_dir.clone()
        }
    }

    /// Returns an exam config based on the arguments.
    pub fn exam_config(&self) -> ExamConfig {
        ExamConfig::default()
            .with_base_dir(self.base_dir())
            .with_test_timeout(std::time::Duration::from_secs(self.timeout))
    }

    /// Returns an exam tester based on the arguments.
    pub fn exam_tester(&self) -> ExamTester {
        ExamTester::new(self.exam_config(), self.verbose(), self.dry_run())
    }

    /// Returns whether the verbose mode option is set.
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    /// Returns whether the dry run mode option is set.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}
