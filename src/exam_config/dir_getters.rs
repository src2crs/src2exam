use super::*;

impl ExamConfig {
    /// Returns the base directory of the exam.
    pub fn base_dir(&self) -> PathBuf {
        self.base_dir.clone()
    }

    /// Returns the path to the submissions directory.
    /// Uses the default submissions directory if no custom directory is set.
    pub fn submissions_dir(&self) -> PathBuf {
        self.base_dir.join(&self.submissions_dirname)
    }

    /// Returns the path to the tasks directory.
    pub fn tasks_dir(&self) -> PathBuf {
        self.base_dir.join(&self.tasks_dirname)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_examconfig_de_default_dir_paths() {
        let exam_info = ExamConfig::new_de();

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
    fn new_examconfig_en_default_dir_paths() {
        let exam_info = ExamConfig::new_en();

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
    fn new_examconfig_en_custom_dir_paths() {
        let exam_info = ExamConfig::new_en()
            .with_base_dir("exam_basedir")
            .with_submissions_subdir("custom_submissions")
            .with_tasks_subdir("custom_tasks")
            .with_grading_subdir("custom_grading");

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
}
