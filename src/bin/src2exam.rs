use src2exam::cli::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let exam_tester = args.exam_tester();

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
