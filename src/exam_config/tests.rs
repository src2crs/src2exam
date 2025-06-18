use super::*;

fn exam_dir() -> PathBuf {
    let base_dir = std::env::current_dir().unwrap();
    let testdata_dir = base_dir.join("testdata");
    let exam_dir = testdata_dir.join("go-exam");

    exam_dir
}

// TODO: Is this an integration test?
#[test]
fn student_names_testdata_go_exam() {
    let exam_config = ExamConfig::new_de().with_base_dir(exam_dir());
    let student_names = exam_config.student_names().unwrap();

    assert_eq!(student_names.len(), 3);
    assert!(student_names.contains(&"student_1".to_string()));
    assert!(student_names.contains(&"student_2".to_string()));
    assert!(student_names.contains(&"student_3".to_string()));
}

/// Tests the student names with the go exam,
/// but specifies a custom submissions directory.
/// Expected outcome: An Error value is returned by student_names().
/// TODO: Is this an integration test?
#[test]
fn student_names_testdata_go_exam_non_existent_submissions_dir() {
    let exam_config = ExamConfig::new_en()
        .with_base_dir(exam_dir())
        .with_submissions_subdir("non_existent_dir");
    let student_names = exam_config.student_names();

    assert!(student_names.is_err());
}

// TODO: Is this an integration test?
#[test]
fn task_names_testdata_go_exam() {
    let exam_config = ExamConfig::new_de().with_base_dir(exam_dir());
    let task_names = exam_config.task_names().unwrap();

    assert_eq!(task_names.len(), 3);
    assert!(task_names.contains(&"task_1".to_string()));
    assert!(task_names.contains(&"task_2".to_string()));
    assert!(task_names.contains(&"task_3".to_string()));
}

// Is this an integration test?
#[test]
fn task_names_testdata_go_exam_non_existent_tasks_dir() {
    let exam_config = ExamConfig::new_de()
        .with_base_dir(exam_dir())
        .with_tasks_subdir("non_existent_dir");
    let task_names = exam_config.task_names();

    assert!(task_names.is_err());
}
