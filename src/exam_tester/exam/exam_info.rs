use crate::exam_tester::exam::exam_info_language::ExamInfoLanguage;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug)]
pub struct ExamInfo {
    base_dir: PathBuf,
    tasks_dirname: String,
    submissions_dirname: String,
    grading_dirname: String,
    test_timeout: Duration,
}

impl ExamInfo {
    /// Creates a new `ExamInfo` instance with the given base directory.
    pub fn new(lang: ExamInfoLanguage) -> Self {
        Self {
            base_dir: lang.base_dir(),
            tasks_dirname: lang.tasks_dirname(),
            submissions_dirname: lang.submissions_dirname(),
            grading_dirname: lang.grading_dirname(),
            test_timeout: lang.test_timeout_default(),
        }
    }

    /// Creates a new `ExamInfo` instance with german defaults.
    pub fn new_de() -> Self {
        Self::new(ExamInfoLanguage::De)
    }

    /// Creates a new `ExamInfo` instance with english defaults.
    pub fn new_en() -> Self {
        Self::new(ExamInfoLanguage::En)
    }

    /// Returns the base directory of the exam.
    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    /// Sets the base directory of the exam.
    pub fn set_base_dir<T: Into<PathBuf>>(&mut self, base_dir: T) {
        self.base_dir = base_dir.into();
    }

    /// Returns the path to the submissions directory.
    /// Uses the default submissions directory if no custom directory is set.
    pub fn submissions_dir(&self) -> PathBuf {
        self.base_dir.join(&self.submissions_dirname)
    }

    /// Sets the custom submissions directory.
    pub fn set_submissions_dirname<T: Into<String>>(&mut self, dir: T) {
        self.submissions_dirname = dir.into();
    }

    /// Returns the path to the tasks directory.
    pub fn tasks_dir(&self) -> PathBuf {
        self.base_dir.join(&self.tasks_dirname)
    }

    /// Sets the custom tasks directory.
    pub fn set_tasks_dirname<T: Into<String>>(&mut self, dir: T) {
        self.tasks_dirname = dir.into();
    }

    /// Returns the path to the grading directory.
    /// Uses the default grading directory if no custom directory is set.
    ///
    /// The grading directory is where copies of the submissions
    /// will be stored together with tests and other grading related files.
    /// Files in this directory are supposed to be modified when grading the exam.
    /// From these files, the final reports will be generated.
    pub fn grading_dir(&self) -> PathBuf {
        self.base_dir.join(&self.grading_dirname)
    }

    /// Sets the custom grading directory.
    pub fn set_grading_dirname<T: Into<String>>(&mut self, dir: T) {
        self.grading_dirname = dir.into();
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
    }

    /// Sets the test timeout for the exam.
    /// The timeout is specified in seconds.
    pub fn set_test_timeout(&mut self, timeout: u64) {
        self.test_timeout = Duration::from_secs(timeout);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_examinfo_dirs_de() {
        let exam_info = ExamInfo::new_de();

        let base_dir = PathBuf::from(".");
        let submissions_dir = PathBuf::from("./abgaben");
        let tasks_dir = PathBuf::from("./aufgaben");
        let grading_dir = PathBuf::from("./bewertung");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }

    #[test]
    fn new_examinfo_dirs_en() {
        let exam_info = ExamInfo::new_en();

        let base_dir = PathBuf::from(".");
        let submissions_dir = PathBuf::from("./submissions");
        let tasks_dir = PathBuf::from("./tasks");
        let grading_dir = PathBuf::from("./grading");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }

    #[test]
    fn custom_dirs() {
        let mut exam_info = ExamInfo::new_en();
        exam_info.set_base_dir("exam_basedir");

        exam_info.set_submissions_dirname("custom_submissions");
        exam_info.set_tasks_dirname("custom_tasks");
        exam_info.set_grading_dirname("custom_grading");

        assert_eq!(
            exam_info.submissions_dir(),
            PathBuf::from("exam_basedir/custom_submissions")
        );
        assert_eq!(
            exam_info.tasks_dir(),
            PathBuf::from("exam_basedir/custom_tasks")
        );
        assert_eq!(
            exam_info.grading_dir(),
            PathBuf::from("exam_basedir/custom_grading")
        );
    }

    #[test]
    fn student_names_testdata_go_exam() {
        let base_dir = std::env::current_dir().unwrap();
        let testdata_dir = base_dir.join("testdata");
        let exam_dir = testdata_dir.join("go-exam");

        let mut exam_info = ExamInfo::new_de();
        exam_info.set_base_dir(exam_dir);
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

        let mut exam_info = ExamInfo::new_en();
        exam_info.set_base_dir(exam_dir);
        exam_info.set_submissions_dirname("non_existent_dir");
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

        let mut exam_info = ExamInfo::new_de();
        exam_info.set_base_dir(exam_dir);
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

        let mut exam_info = ExamInfo::new_de();
        exam_info.set_base_dir(exam_dir);
        exam_info.set_tasks_dirname("non_existent_dir");
        let task_names = exam_info.task_names();

        assert_eq!(
            task_names,
            Err("No such file or directory (os error 2)".to_string())
        );
    }
}
