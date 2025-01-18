// Note: This must be run from the project root directory to get the directories right.

use src2exam::exam_tester::exam;

fn main() {
    let base_dir = std::env::current_dir().unwrap();
    let testdata_dir = base_dir.join("testdata");
    let exam_dir = testdata_dir.join("go-exam");

    let exam_info = exam::ExamInfo::new(&exam_dir);
    let submissions_dir = exam_info.submissions_dir();
    let tasks_dir = exam_info.tasks_dir();
    let grading_dir = exam_info.grading_dir();

    let student_names = exam_info.student_names().unwrap();
    let task_names = exam_info.task_names().unwrap();

    println!("ExamInfo for the example Go exam in {:?}:", exam_dir);
    println!("{:#?}", exam_info);
    println!();

    println!("Directories:");
    println!("  Submission directory: {:?}", submissions_dir);
    println!("  Tasks directory: {:?}", tasks_dir);
    println!("  Grading directory: {:?}", grading_dir);
    println!();

    println!("Exam properties:");
    println!("  Task names: {:?}", task_names);
    println!("  Student names: {:?}", student_names);
    println!();
}
