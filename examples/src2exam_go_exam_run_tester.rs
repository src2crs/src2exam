// Note: This must be run from the project root directory to get the directories right.

use src2exam::exam_tester::exam::{ExamInfo, ExamTester};

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    let testdata_dir = base_dir.join("testdata");
    let exam_dir = testdata_dir.join("go-exam");

    let mut exam_info = ExamInfo::new_de();
    exam_info.set_base_dir(exam_dir);
    let exam_tester = ExamTester::new(exam_info);

    // Copying student submissions and tests to the grading directory.
    // Note the ordering of the operations:
    // As no files are overwritten, copying the submissions first
    // copies the students' tests into the grading directory
    // and adds only the extra tests from the tasks directory.
    // Otherwise, the original tests would be prioritized.
    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
