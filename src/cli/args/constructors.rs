use crate::ExamConfig;

use super::Args;

use std::path::PathBuf;

/// Constructors
impl Args {
    /// Creates a new Args instance from the given values.
    pub fn new<B, T, S, G, U, C>(
        base_directory: B,
        tasks_dirname: T,
        submissions_dirname: S,
        grading_dirname: G,
        timeout: U,
        verbose: bool,
        dry_run: bool,
        config_path: C,
        create_config: bool,
    ) -> Self
    where
        B: Into<PathBuf>,
        T: Into<String>,
        S: Into<String>,
        G: Into<String>,
        U: Into<u64>,
        C: Into<Option<PathBuf>>,
    {
        Self {
            base_directory: base_directory.into(),
            tasks_dirname: tasks_dirname.into(),
            submissions_dirname: submissions_dirname.into(),
            grading_dirname: grading_dirname.into(),
            timeout: timeout.into(),
            verbose,
            config_path: config_path.into(),
            dry_run,
            create_config,
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new(
            ExamConfig::default_base_dirname(),
            ExamConfig::default_tasks_dirname(),
            ExamConfig::default_submissions_dirname(),
            ExamConfig::default_grading_dirname(),
            ExamConfig::default_timeout_secs(),
            false,
            false,
            None,
            false,
        )
    }
}
