use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct ExamConfig {
    base_dir: PathBuf,
    tasks_dirname: String,
    submissions_dirname: String,
    grading_dirname: String,
    test_timeout: Duration,
}

mod constructors;
mod dir_getters;

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
            tasks_dirname: "".into(),
            submissions_dirname: "".into(),
            grading_dirname: "".into(),
            test_timeout: Duration::from_secs(5),
        }
    }
}
