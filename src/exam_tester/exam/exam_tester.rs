use crate::exam_tester::exam::ExamInfo;
use crate::exam_tester::process::GoRunner;

pub struct ExamTester {
    exam_info: ExamInfo,
    verbose: bool,
    dry_run: bool,
}

impl ExamTester {
    pub fn new(exam_info: ExamInfo, verbose: bool, dry_run: bool) -> Self {
        Self {
            exam_info,
            verbose: verbose,
            dry_run: dry_run,
        }
    }

    /// Copies the submissions into the grading directory.
    pub fn copy_submissions(&self) {
        let submissions_dir = self.exam_info.submissions_dir();
        let grading_dir = self.exam_info.grading_dir();

        println!(
            "Copying submissions from {:?} to {:?}",
            submissions_dir, grading_dir
        );

        if !self.dry_run() {
            crate::filesystem::copy_subdirs(&submissions_dir, &grading_dir);
        }
    }

    /// Copies tests from the tasks directory to the grading directory.
    /// More precisely, copies all files ending in `_test.go` from
    /// the tasks to all corresponding submissions in the grading directory.
    ///
    /// The following conditions apply:
    /// * Will not overwrite any files
    /// * Will create new directories for tasks under known student directories
    ///   (based on the student names reported by the exam info).
    /// * Will not create or copy any other directories or files.
    pub fn copy_tests(&self) {
        let tasks_dir = self.exam_info.tasks_dir();
        let grading_dir = self.exam_info.grading_dir();

        let task_names = self.exam_info.task_names().unwrap();
        let student_names = self.exam_info.student_names().unwrap();

        for task_name in &task_names {
            let task_dir = tasks_dir.join(task_name);
            let test_files = crate::filesystem::files_with_suffix(&task_dir, "_test.go").unwrap();

            println!("Copying tests for task: {}", task_name);

            for student_name in &student_names {
                let student_dir = grading_dir.join(student_name);
                let student_task_dir = student_dir.join(task_name);

                println!("  Student: {}", student_name);

                if !self.dry_run() {
                    crate::filesystem::copy_files(&test_files, &task_dir, &student_task_dir);
                }
            }
        }
    }

    /// Runs the tests for all students and tasks in the grading directory.
    ///
    /// The tests and submissions are expected to be in the correct directories.
    /// I.e. no checks are made for the existence of the files
    /// and no files are copied or moved.
    ///
    /// A message is printed for each test run.
    /// A short summary of the result is appended to the corresponding source file.
    /// The filename is assumed to be the task name with the `.go` extension.
    pub fn run_tests(&self) {
        let grading_dir = self.exam_info.grading_dir();
        let student_names = self.exam_info.student_names().unwrap();
        let task_names = self.exam_info.task_names().unwrap();

        for student_name in &student_names {
            println!("Running tests for student: {}", student_name);
            for task_name in &task_names {
                print!("  {}: ", task_name);
                let student_task_dir = grading_dir.join(student_name).join(task_name);

                if self.dry_run() {
                    print!("(dry run)");
                } else {
                    let runner = GoRunner::new(&student_task_dir, self.exam_info.test_timeout());
                    let test_result = runner.run_tests();

                    let result_message = test_result.to_string_de();
                    let grading_message = format!("// BEWERTUNG: \n// TESTS: {}", result_message);

                    // Print result message and append result to source file.
                    print!("{}", result_message);
                    let source_file = student_task_dir.join(format!("{}.go", task_name));
                    crate::filesystem::append_to_file(&source_file, &grading_message);
                }
                println!();
            }
        }
    }

    /// Returns whether verbose mode is enabled.
    /// Verbose mode is automatically enabled in dry run mode.
    pub fn verbose(&self) -> bool {
        self.dry_run() || self.verbose
    }

    /// Returns whether dry run mode is enabled.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Prints info about what is happening.
    pub fn print_info(&self) {
        println!("Running in directory: {:?}", self.exam_info.base_dir());
        if self.verbose() {
            println!("{}", self.exam_info.summary());
        }

        if self.dry_run() {
            println!("Dry run mode enabled.");
        }
    }
}
