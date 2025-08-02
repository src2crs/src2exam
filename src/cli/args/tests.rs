use super::*;
use crate::ExamConfig;
use std::time::Duration;

#[test]
fn values_for_default_struct() {
    let args = Args::default();

    assert_eq!(args.exam_config(), ExamConfig::default());

    assert_eq!(args.base_directory, ExamConfig::default_base_dir());
    assert_eq!(args.timeout, ExamConfig::default_timeout_secs());
    assert!(!args.verbose);
    assert!(!args.dry_run);

    assert_eq!(args.base_dir(), ExamConfig::default_base_dir());
}

#[test]
fn values_for_custom_struct() {
    let args = Args::new(
        "base_dir",
        "taskdir",
        "submissiondir",
        "gradingdir",
        "Go",
        5u64,
        true,
        false,
        None,
        false,
    );

    assert_eq!(args.base_directory, PathBuf::from("base_dir"));
    assert_eq!(args.tasks_dirname, "taskdir");
    assert_eq!(args.submissions_dirname, "submissiondir");
    assert_eq!(args.grading_dirname, "gradingdir");
    assert_eq!(args.timeout, 5);
    assert!(args.verbose);
    assert!(!args.dry_run);

    assert_eq!(args.base_dir(), PathBuf::from("base_dir"));
}

#[test]
fn exam_config_for_default_struct_() {
    let args = Args::default();
    let exam_config = args.exam_config();

    let expected_base_dir = ExamConfig::default_base_dir();
    let expected_tasks_dir = expected_base_dir.join(ExamConfig::default_tasks_dirname());
    let expected_grading_dir = expected_base_dir.join(ExamConfig::default_grading_dirname());
    let expected_submissions_dir =
        expected_base_dir.join(ExamConfig::default_submissions_dirname());
    let expected_timeout = Duration::from_secs(ExamConfig::default_timeout_secs());

    assert_eq!(exam_config.base_dir(), expected_base_dir);
    assert_eq!(exam_config.tasks_dir(), expected_tasks_dir);
    assert_eq!(exam_config.grading_dir(), expected_grading_dir);
    assert_eq!(exam_config.submissions_dir(), expected_submissions_dir);
    assert_eq!(exam_config.test_timeout(), expected_timeout);
}

#[test]
fn exam_config_for_custom_struct() {
    // TODO: Also test subdirectory names.

    let args = Args::new(
        "custom_base_dir",
        "taskdir",
        "submissiondir",
        "gradingdir",
        "Go",
        60u64,
        true,
        true,
        None,
        false,
    );

    let expected_base_dir = PathBuf::from("custom_base_dir");
    let expected_tasks_dir = expected_base_dir.join("taskdir");
    let expected_grading_dir = expected_base_dir.join("gradingdir");
    let expected_submissions_dir = expected_base_dir.join("submissiondir");
    let expected_timeout = Duration::from_secs(60);

    let exam_config = args.exam_config();
    assert_eq!(exam_config.base_dir(), expected_base_dir);
    assert_eq!(exam_config.tasks_dir(), expected_tasks_dir);
    assert_eq!(exam_config.grading_dir(), expected_grading_dir);
    assert_eq!(exam_config.submissions_dir(), expected_submissions_dir);
    assert_eq!(exam_config.test_timeout(), expected_timeout);
}
