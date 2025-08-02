use crate::exam_config::code_lang::CodeLang;

use super::ExamConfig;
use std::path::PathBuf;

impl ExamConfig {
    /// Returns the default base directory name for the exam.
    pub fn default_base_dirname() -> String {
        ".".to_string()
    }

    /// Returns the default base directory.
    pub fn default_base_dir() -> PathBuf {
        PathBuf::from(Self::default_base_dirname())
    }

    /// Returns the default tasks directory name.
    pub fn default_tasks_dirname() -> String {
        "tasks".to_string()
    }

    /// Returns the default submissions directory name.
    pub fn default_submissions_dirname() -> String {
        "submissions".to_string()
    }

    /// Returns the default grading directory name.
    pub fn default_grading_dirname() -> String {
        "grading".to_string()
    }

    /// Returns the default code language.
    pub fn default_code_language() -> CodeLang {
        CodeLang::default()
    }

    pub fn default_code_language_string() -> String {
        Self::default_code_language().to_string()
    }

    /// Returns the default test timeout for the exam as a number of seconds.
    pub fn default_timeout_secs() -> u64 {
        30
    }

    /// Returns the default test timeout for the exam.
    pub fn default_timeout() -> std::time::Duration {
        std::time::Duration::from_secs(Self::default_timeout_secs())
    }
}
