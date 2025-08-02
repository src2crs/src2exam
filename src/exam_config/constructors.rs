use super::*;

impl ExamConfig {
    /// Creates an `ExamConfig` instance from self with the given base directory.
    pub fn with_base_dir<P: Into<PathBuf>>(self, base_dir: P) -> Self {
        let mut result = self;
        result.base_dir = base_dir.into();
        result
    }

    /// Creates an `ExamConfig` instance from self with the given tasks subdirectory.
    /// The tasks directory is relative to the base directory.
    pub fn with_submissions_subdir<S: Into<String>>(self, submissions_subdir: S) -> Self {
        let mut result = self;
        result.submissions_dirname = submissions_subdir.into();
        result
    }

    /// Creates an `ExamConfig` instance from self with the given tasks subdirectory.
    /// The tasks directory is relative to the base directory.
    pub fn with_tasks_subdir<S: Into<String>>(self, tasks_subdir: S) -> Self {
        let mut result = self;
        result.tasks_dirname = tasks_subdir.into();
        result
    }

    /// Creates an `ExamConfig` instance from self with the given grading subdirectory.
    /// The grading directory is relative to the base directory.
    pub fn with_grading_subdir<S: Into<String>>(self, grading_subdir: S) -> Self {
        let mut result = self;
        result.grading_dirname = grading_subdir.into();
        result
    }

    /// Creates an `ExamConfig` instance from self with the given coding language.
    pub fn with_coding_language<S: Into<String>>(self, coding_lang: S) -> Self {
        let mut result = self;
        // TODO: Improve error handling.
        result.code_language =
            CodeLang::try_from(coding_lang.into()).expect("invalid coding language");
        result
    }

    /// Creates a new `ExamConfig` instance from self with the given test timeout.
    pub fn with_test_timeout(self, timeout: Duration) -> Self {
        let mut result = self;
        result.test_timeout = timeout;
        result
    }
}

#[cfg(test)]
mod tests {}
