// Note: This must be run from the project root directory to get the directories right.

use clap::Parser;
use src2exam::cli::Args;

mod common;
use common::go_exam_dir_str;

fn main() {
    let args = Args::parse_from(["src2exam", "--directory", &go_exam_dir_str()]);

    let exam_config = args.exam_config();

    println!("ExamConfig for the example Go exam:");
    println!("{}", exam_config.summary());
    println!();
}
