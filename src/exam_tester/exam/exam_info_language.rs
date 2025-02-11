use clap::ValueEnum;
use std::path::PathBuf;
use std::time::Duration;

/// Provides default values for ExamInfo struct
/// for different languages.
/// Uses `En` as the default language.
#[derive(Debug, ValueEnum, Clone)]
pub enum ExamInfoLanguage {
    En,
    De,
}

impl ExamInfoLanguage {
    /// Returns the default path to the submissions directory.
    pub fn submissions_dirname(&self) -> String {
        String::from(match self {
            Self::En => "submissions",
            Self::De => "abgaben",
        })
    }

    /// Returns the default path to the tasks directory.
    pub fn tasks_dirname(&self) -> String {
        String::from(match self {
            Self::En => "tasks",
            Self::De => "aufgaben",
        })
    }

    /// Returns the default path to the grading directory.
    pub fn grading_dirname(&self) -> String {
        String::from(match self {
            Self::En => "grading",
            Self::De => "bewertung",
        })
    }

    /// Returns the default for the base directory.
    pub fn base_dir(&self) -> PathBuf {
        PathBuf::from(".")
    }

    /// Returns the default timeout for running the tests.
    pub fn test_timeout_default(&self) -> Duration {
        Duration::from_secs(5)
    }
}
