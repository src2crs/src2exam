pub mod common;
use common::TestCases::{GoExamDe, GoExamEn};

use src2exam::cli::Args;

#[test]
fn exam_config_dirs_en() {
    let exam_dir = GoExamEn.dir();

    let args = Args::new(
        exam_dir.clone(),
        "tasks",
        "submissions",
        "grading",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    assert_eq!(exam_config.base_dir(), exam_dir.canonicalize().unwrap());
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("tasks"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("grading"));
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("submissions"));
}

#[test]
fn exam_config_dirs_de() {
    let exam_dir = GoExamDe.dir();

    let args = Args::new(
        exam_dir.clone(),
        "aufgaben",
        "abgaben",
        "bewertung",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    assert_eq!(exam_config.base_dir(), exam_dir.canonicalize().unwrap());
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("aufgaben"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("bewertung"));
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("abgaben"));
}

#[test]
fn student_names_testdata_go_exam_en() {
    let exam_dir = GoExamEn.dir();
    let args = Args::new(
        exam_dir.clone(),
        "tasks",
        "submissions",
        "grading",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    let student_names = exam_config.student_names().unwrap();

    assert_eq!(student_names.len(), 3);
    assert!(student_names.contains(&"student_1".to_string()));
    assert!(student_names.contains(&"student_2".to_string()));
    assert!(student_names.contains(&"student_3".to_string()));
}

#[test]
fn student_names_testdata_go_exam_de() {
    let exam_dir = GoExamDe.dir();
    let args = Args::new(
        exam_dir.clone(),
        "aufgaben",
        "abgaben",
        "bewertung",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    let student_names = exam_config.student_names().unwrap();

    assert_eq!(student_names.len(), 3);
    assert!(student_names.contains(&"student_1".to_string()));
    assert!(student_names.contains(&"student_2".to_string()));
    assert!(student_names.contains(&"student_3".to_string()));
}

/// Tests the student names with the go exam,
/// but specifies a custom submissions directory.
/// Expected outcome: An Error value is returned by student_names().
#[test]
fn student_names_testdata_go_exam_non_existent_submissions_dir() {
    let exam_dir = GoExamDe.dir();
    let args = Args::new(exam_dir.clone(), "---", "---", "---", 30u64, false, false);
    let exam_config = args
        .exam_config()
        .with_submissions_subdir("non_existent_dir");

    let student_names = exam_config.student_names();

    assert!(student_names.is_err());
}

#[test]
fn task_names_testdata_go_exam_en() {
    let exam_dir = GoExamEn.dir();
    let args = Args::new(
        exam_dir.clone(),
        "tasks",
        "submissions",
        "grading",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    let task_names = exam_config.task_names().unwrap();

    assert_eq!(task_names.len(), 3);
    assert!(task_names.contains(&"task_1".to_string()));
    assert!(task_names.contains(&"task_2".to_string()));
    assert!(task_names.contains(&"task_3".to_string()));
}

#[test]
fn task_names_testdata_go_exam_de() {
    let exam_dir = GoExamDe.dir();
    let args = Args::new(
        exam_dir.clone(),
        "aufgaben",
        "abgaben",
        "bewertung",
        30u64,
        false,
        false,
    );
    let exam_config = args.exam_config();

    let task_names = exam_config.task_names().unwrap();

    assert_eq!(task_names.len(), 3);
    assert!(task_names.contains(&"aufgabe_1".to_string()));
    assert!(task_names.contains(&"aufgabe_2".to_string()));
    assert!(task_names.contains(&"aufgabe_3".to_string()));
}

#[test]
fn task_names_testdata_go_exam_non_existent_tasks_dir() {
    let exam_dir = GoExamDe.dir();
    let args = Args::new(exam_dir.clone(), "---", "---", "---", 30u64, false, false);
    let exam_config = args.exam_config().with_tasks_subdir("non_existent_dir");
    let task_names = exam_config.task_names();

    assert!(task_names.is_err());
}
