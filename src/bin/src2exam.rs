use src2exam::cli::Args;
use src2exam::exam_tester::exam::ExamTester;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let exam_tester = ExamTester::from(&args);

    if exam_tester.verbose() {
        exam_tester.print_info();
    }

    if exam_tester.dry_run() {
        return;
    }

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
