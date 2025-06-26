// Note: This must be run from the project root directory to get the directories right.

use src2exam::cli::Args;

mod common;
use common::exam_dir;

fn main() {
    let args = Args::new(exam_dir(), 30u64, "de", false, false);
    let exam_config = args.exam_config().unwrap();

    println!("ExamConfig for the example Go exam:");
    println!("{}", exam_config.summary());
    println!();
}
