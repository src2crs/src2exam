use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug)]
pub struct ExamInfo {
    base_dir: PathBuf,
    custom_tasks_dir: Option<PathBuf>,
    custom_submissions_dir: Option<PathBuf>,
    custom_grading_dir: Option<PathBuf>,
    test_timeout: Option<Duration>,
}

impl ExamInfo {
    /// Creates a new `ExamInfo` instance with the given base directory.
    pub fn new<T: Into<PathBuf>>(base_dir: T) -> Self {
        Self {
            base_dir: base_dir.into(),
            ..Default::default()
        }
    }

    /// Returns the default base directory to use for the exam.
    pub fn base_dir_default() -> PathBuf {
        std::env::current_dir().unwrap()
    }

    /// Returns the base directory of the exam.
    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    /// Returns the default path to the submissions directory.
    pub fn submissions_dir_default(&self) -> PathBuf {
        self.base_dir.join("abgaben")
    }

    /// Returns the path to the submissions directory.
    /// Uses the default submissions directory if no custom directory is set.
    pub fn submissions_dir(&self) -> PathBuf {
        self.custom_submissions_dir
            .clone()
            .unwrap_or_else(|| self.submissions_dir_default())
    }

    /// Sets the custom submissions directory.
    pub fn set_submissions_dir<T: Into<PathBuf>>(&mut self, dir: T) {
        self.custom_submissions_dir = Some(dir.into());
    }

    /// Returns the default path to the tasks directory.
    pub fn tasks_dir_default(&self) -> PathBuf {
        self.base_dir.join("aufgaben")
    }

    /// Returns the path to the tasks directory.
    pub fn tasks_dir(&self) -> PathBuf {
        self.custom_tasks_dir
            .clone()
            .unwrap_or_else(|| self.tasks_dir_default())
    }

    /// Sets the custom tasks directory.
    pub fn set_tasks_dir<T: Into<PathBuf>>(&mut self, dir: T) {
        self.custom_tasks_dir = Some(dir.into());
    }

    /// Returns the default path to the grading directory.
    pub fn grading_dir_default(&self) -> PathBuf {
        self.base_dir.join("bewertung")
    }

    /// Returns the path to the grading directory.
    /// Uses the default grading directory if no custom directory is set.
    ///
    /// The grading directory is where copies of the submissions
    /// will be stored together with tests and other grading related files.
    /// Files in this directory are supposed to be modified when grading the exam.
    /// From these files, the final reports will be generated.
    pub fn grading_dir(&self) -> PathBuf {
        self.custom_grading_dir
            .clone()
            .unwrap_or_else(|| self.grading_dir_default())
    }

    /// Sets the custom grading directory.
    pub fn set_grading_dir<T: Into<PathBuf>>(&mut self, dir: T) {
        self.custom_grading_dir = Some(dir.into());
    }

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

    /// Returns the default test timeout for the exam.
    pub fn test_timeout_default() -> Duration {
        Duration::from_secs(30)
    }

    /// Returns the test timeout for the exam.
    pub fn test_timeout(&self) -> Duration {
        self.test_timeout
            .unwrap_or_else(|| Self::test_timeout_default())
    }

    /// Sets the test timeout for the exam.
    /// The timeout is specified in seconds.
    pub fn set_test_timeout(&mut self, timeout: u64) {
        self.test_timeout = Some(Duration::from_secs(timeout));
    }
}

impl Default for ExamInfo {
    // A default ExamInfo instance is created with the current directory as the base directory.
    fn default() -> Self {
        Self {
            base_dir: Self::base_dir_default(),
            custom_tasks_dir: None,
            custom_submissions_dir: None,
            custom_grading_dir: None,
            test_timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_examinfo_dirs() {
        let exam_info = ExamInfo::new("exam_basedir");

        let base_dir = PathBuf::from("exam_basedir");
        let submissions_dir = PathBuf::from("exam_basedir/abgaben");
        let tasks_dir = PathBuf::from("exam_basedir/aufgaben");
        let grading_dir = PathBuf::from("exam_basedir/bewertung");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }

    #[test]
    fn default_examinfo_dirs() {
        let exam_info = ExamInfo::default();

        let base_dir = std::env::current_dir().unwrap();
        let submissions_dir = base_dir.join("abgaben");
        let tasks_dir = base_dir.join("aufgaben");
        let grading_dir = base_dir.join("bewertung");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }

    #[test]
    fn custom_dirs() {
        let mut exam_info = ExamInfo::new("exam_basedir");

        let custom_submissions_dir = PathBuf::from("custom_submissions");
        let custom_tasks_dir = PathBuf::from("custom_tasks");
        let custom_grading_dir = PathBuf::from("custom_grading");

        exam_info.set_submissions_dir(&custom_submissions_dir);
        exam_info.set_tasks_dir(&custom_tasks_dir);
        exam_info.set_grading_dir(&custom_grading_dir);

        assert_eq!(exam_info.submissions_dir(), custom_submissions_dir);
        assert_eq!(exam_info.tasks_dir(), custom_tasks_dir);
        assert_eq!(exam_info.grading_dir(), custom_grading_dir);
    }

    #[test]
    fn student_names_testdata_go_exam() {
        let base_dir = std::env::current_dir().unwrap();
        let testdata_dir = base_dir.join("testdata");
        let exam_dir = testdata_dir.join("go-exam");

        let exam_info = ExamInfo::new(exam_dir);
        let student_names = exam_info.student_names().unwrap();

        assert_eq!(student_names.len(), 3);
        assert!(student_names.contains(&"student_1".to_string()));
        assert!(student_names.contains(&"student_2".to_string()));
        assert!(student_names.contains(&"student_3".to_string()));
    }

    /// Tests the student names with the go exam,
    /// but specifies a custom submissions directory.
    /// Expected outcome: An Error value is returned by student_names().
    #[test]
    fn student_names_testdata_go_exam_modified_submissions() {
        let base_dir = std::env::current_dir().unwrap();
        let testdata_dir = base_dir.join("testdata");
        let exam_dir = testdata_dir.join("go-exam");

        let mut exam_info = ExamInfo::new(exam_dir);
        exam_info.set_submissions_dir("non_existent_dir");
        let student_names = exam_info.student_names();

        assert_eq!(
            student_names,
            Err("No such file or directory (os error 2)".to_string())
        );
    }

    #[test]
    fn task_names_testdata_go_exam() {
        let base_dir = std::env::current_dir().unwrap();
        let testdata_dir = base_dir.join("testdata");
        let exam_dir = testdata_dir.join("go-exam");

        let exam_info = ExamInfo::new(exam_dir);
        let task_names = exam_info.task_names().unwrap();

        assert_eq!(task_names.len(), 3);
        assert!(task_names.contains(&"task_1".to_string()));
        assert!(task_names.contains(&"task_2".to_string()));
        assert!(task_names.contains(&"task_3".to_string()));
    }

    #[test]
    fn task_names_testdata_go_exam_modified_tasks() {
        let base_dir = std::env::current_dir().unwrap();
        let testdata_dir = base_dir.join("testdata");
        let exam_dir = testdata_dir.join("go-exam");

        let mut exam_info = ExamInfo::new(exam_dir);
        exam_info.set_tasks_dir("non_existent_dir");
        let task_names = exam_info.task_names();

        assert_eq!(
            task_names,
            Err("No such file or directory (os error 2)".to_string())
        );
    }
}
