// Note: This must be run from the project root directory to get the directories right.

use clap::Parser;
use src2exam::cli::Args;

mod common;
use common::go_exam_dir_str;

fn main() {
    let args = Args::parse_from(["src2exam", "--directory", &go_exam_dir_str()]);
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
