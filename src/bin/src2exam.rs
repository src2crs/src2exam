use src2exam::cli::Args;
use src2exam::exam_tester::exam::ExamTester;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let exam_info = args.exam_info();

    println!("Using base directory: {:?}", exam_info.base_dir());

    let exam_tester = ExamTester::new(exam_info);

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
