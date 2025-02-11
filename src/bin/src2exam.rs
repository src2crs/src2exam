use src2exam::cli::Args;
use src2exam::exam_tester::exam::{ExamInfo, ExamTester};

use clap::Parser;

fn main() {
    let args = Args::parse();
    let exam_info = ExamInfo::from(&args);

    println!("Using base directory: {:?}", exam_info.base_dir());

    let exam_tester = ExamTester::new(exam_info);

    if args.verbose() {
        println!("ExamInfo Summary:");
        exam_tester.print_info();
    }

    if args.dry_run() {
        return;
    }

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
