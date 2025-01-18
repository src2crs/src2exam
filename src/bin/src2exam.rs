use clap::Parser;
use std::path::PathBuf;

use src2exam::exam_tester::exam::{ExamInfo, ExamTester};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = ExamInfo::base_dir_default().into_os_string())]
    directory: PathBuf,
    /// The timeout for running the tests in seconds.
    #[arg(short, long, default_value = ExamInfo::test_timeout_default().as_secs().to_string())]
    timeout: u64,
}

fn main() {
    let args = Args::parse();
    let base_dir = args.directory;
    let base_dir = if base_dir.is_relative() {
        std::env::current_dir().unwrap().join(base_dir)
    } else {
        base_dir
    };

    println!("Using base directory: {:?}", base_dir);

    let exam_info = ExamInfo::new(base_dir);
    let exam_tester = ExamTester::new(exam_info);

    exam_tester.copy_submissions();
    exam_tester.copy_tests();
    exam_tester.run_tests();
}
