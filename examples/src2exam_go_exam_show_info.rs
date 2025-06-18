// Note: This must be run from the project root directory to get the directories right.

use src2exam::ExamConfig;

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    let testdata_dir = base_dir.join("testdata");
    let exam_dir = testdata_dir.join("go-exam");

    let exam_config = ExamConfig::new_de().with_base_dir(exam_dir);

    println!("ExamConfig for the example Go exam:");
    println!("{}", exam_config.summary());
    println!();
}
