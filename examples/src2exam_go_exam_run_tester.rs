// Note: This must be run from the project root directory to get the directories right.

use src2exam::cli::Args;

mod common;
use common::exam_dir;

fn main() {
    let args = Args::new(exam_dir(), 30u64, "de", false, false);
    let exam_tester = args.exam_tester().unwrap();

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
