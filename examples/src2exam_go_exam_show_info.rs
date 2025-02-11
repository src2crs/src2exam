// Note: This must be run from the project root directory to get the directories right.

use src2exam::exam_tester::exam;

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    let testdata_dir = base_dir.join("testdata");
    let exam_dir = testdata_dir.join("go-exam");

    let mut exam_info = exam::ExamInfo::new_de();
    exam_info.set_base_dir(exam_dir);

    println!("ExamInfo for the example Go exam:");
    println!("{}", exam_info.summary());
    println!();
}
