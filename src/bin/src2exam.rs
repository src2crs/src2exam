use src2exam::cli::Args;
use src2exam::exam_tester::exam::{ExamInfo, ExamTester};

use clap::Parser;

fn main() {
    let args = Args::parse();
    let base_dir = args.base_dir();

    println!("Using base directory: {:?}", base_dir);

    let exam_info = ExamInfo::new(base_dir);
    let exam_tester = ExamTester::new(exam_info);

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
