use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

mod code_lang;
mod constructors;
mod conversions;
mod defaults;
mod dir_getters;

pub use code_lang::CodeLang;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExamConfig {
    base_dir: PathBuf,
    tasks_dirname: String,
    submissions_dirname: String,
    grading_dirname: String,
    code_language: CodeLang,
    test_timeout: Duration,
}

impl ExamConfig {
    /// Returns the student names for the exam.
    /// Each subdirectory name in the submissions directory
    /// is expected to be a student's name.
    /// The names are sorted in lexicographical order.
    pub fn student_names(&self) -> Result<Vec<String>, String> {
        let mut names = crate::filesystem::subdir_names(&self.submissions_dir())?;
        names.sort();
        Ok(names)
    }

    /// Returns the tasks for the exam.
    /// Each subdirectory in the tasks directory is expected to be a task.
    /// The task name is the subdirectory name.
    /// The task names are sorted in lexicographical order.
    pub fn task_names(&self) -> Result<Vec<String>, String> {
        let mut tasks = crate::filesystem::subdir_names(&self.tasks_dir())?;
        tasks.sort();
        Ok(tasks)
    }

    /// Returns the test timeout for the exam.
    pub fn test_timeout(&self) -> Duration {
        self.test_timeout
    }

    /// Returns a string summarizing the ExamInfo's directories.
    fn directory_summary(&self) -> String {
        format!(
            "Base directory: {:?}\nSubmissions directory: {:?}\nTasks directory: {:?}\nGrading directory: {:?}\nTest timeout: {:?}",
            self.base_dir(),
            self.submissions_dir(),
            self.tasks_dir(),
            self.grading_dir(),
            self.test_timeout(),
        )
    }

    /// Returns a string summarizing the ExamInfo's student and task names.
    fn property_summary(&self) -> String {
        let student_names = self.student_names().unwrap_or_default();
        let task_names = self.task_names().unwrap_or_default();

        format!(
            "Student names: {:?}\nTask names: {:?}",
            student_names, task_names
        )
    }

    /// Returns a string containing a summary of the ExamInfo.
    /// TODO: Pretty print the output.
    pub fn summary(&self) -> String {
        format!(
            "Summary:\n{}\n{}",
            self.directory_summary(),
            self.property_summary(),
        )
    }
}

impl Default for ExamConfig {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::from("."),
            tasks_dirname: Self::default_tasks_dirname(),
            submissions_dirname: Self::default_submissions_dirname(),
            grading_dirname: Self::default_grading_dirname(),
            test_timeout: Self::default_timeout(),
            code_language: Self::default_code_language(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_exam_config() {
        let exam_config = ExamConfig::default();
        assert_eq!(exam_config.base_dir, PathBuf::from("."));
        assert_eq!(exam_config.tasks_dirname, "tasks");
        assert_eq!(exam_config.submissions_dirname, "submissions");
        assert_eq!(exam_config.grading_dirname, "grading");
        assert_eq!(exam_config.test_timeout, Duration::from_secs(30));
        assert_eq!(exam_config.code_language, CodeLang::default());
    }

    #[test]
    fn exam_config_serialization() {
        let exam_config = ExamConfig::default()
            .with_base_dir("basedir")
            .with_tasks_subdir("tasksdir")
            .with_submissions_subdir("submissionsdir")
            .with_grading_subdir("gradingdir")
            .with_coding_language("Go")
            .with_test_timeout(Duration::from_secs(15));
        let serialized = toml::to_string(&exam_config).unwrap();

        let expected_toml = [
            r#"base_dir = "basedir""#,
            r#"tasks_dirname = "tasksdir""#,
            r#"submissions_dirname = "submissionsdir""#,
            r#"grading_dirname = "gradingdir""#,
            r#"code_language = "Go""#,
            r#""#,
            r#"[test_timeout]"#,
            r#"secs = 15"#,
            r#"nanos = 0"#,
        ]
        .join("\n");
        assert_eq!(serialized.trim(), expected_toml.trim());
    }

    #[test]
    fn exam_config_deserialization() {
        let toml_str = [
            r#"base_dir = "basedir""#,
            r#"tasks_dirname = "tasksdir""#,
            r#"submissions_dirname = "submissionsdir""#,
            r#"grading_dirname = "gradingdir""#,
            r#"code_language = "Go""#,
            r#""#,
            r#"[test_timeout]"#,
            r#"secs = 15"#,
            r#"nanos = 0"#,
        ]
        .join("\n");

        let exam_config: ExamConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(exam_config.base_dir, PathBuf::from("basedir"));
        assert_eq!(exam_config.tasks_dirname, "tasksdir");
        assert_eq!(exam_config.submissions_dirname, "submissionsdir");
        assert_eq!(exam_config.grading_dirname, "gradingdir");
        assert_eq!(exam_config.code_language, CodeLang::Go);
        assert_eq!(exam_config.test_timeout, Duration::from_secs(15));
    }
}
