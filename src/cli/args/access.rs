use super::{Args, Result};

use crate::{
    cli::{args::lang::Language, CliError},
    ExamConfig, ExamTester,
};
use std::path::PathBuf;

/// Access
impl Args {
    /// Returns the base directory to use.
    /// This is the stored directory if it is absolute,
    /// otherwise it is the stored directory relative to the current directory.
    pub fn base_dir(&self) -> Result<PathBuf> {
        let base_dir = &self.directory;
        let dir = if base_dir.is_relative() {
            let dir = std::env::current_dir()
                .map_err(|_| CliError::from("Failed to get current directory"))?;
            dir.join(base_dir)
        } else {
            base_dir.clone()
        };
        Ok(dir)
    }

    /// Returns an exam config based on the arguments.
    pub fn exam_config(&self) -> Result<ExamConfig> {
        let exam_config = ExamConfig::from(self.language())
            .with_base_dir(self.base_dir()?)
            .with_test_timeout(std::time::Duration::from_secs(self.timeout));
        Ok(exam_config)
    }

    /// Returns an exam tester based on the arguments.
    pub fn exam_tester(&self) -> Result<ExamTester> {
        Ok(ExamTester::new(
            self.exam_config()?,
            self.verbose(),
            self.dry_run(),
        ))
    }

    /// Returns whether the verbose mode option is set.
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    /// Returns whether the dry run mode option is set.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Returns the language to use for the exam.
    pub fn language(&self) -> Language {
        self.language.clone()
    }
}
