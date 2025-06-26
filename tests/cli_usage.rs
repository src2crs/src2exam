pub mod common;
use common::TestCases::{GoExamDe, GoExamEn};

use clap::Parser;
use src2exam::cli::Args;

#[test]
fn go_exam_de_dirs() {
    let args = Args::parse_from([
        "src2exam",
        "--directory",
        &GoExamDe.dir_str(),
        "--language",
        "de",
        "--timeout",
        "15",
        "--verbose",
        "--dry-run",
    ]);

    let exam_tester = args.exam_tester().unwrap();
    let exam_config = exam_tester.exam_config();

    let exam_dir = GoExamDe.dir().canonicalize().unwrap();
    assert_eq!(exam_config.base_dir(), exam_dir);
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("aufgaben"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("bewertung"));
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("abgaben"));
    assert_eq!(
        exam_config.test_timeout(),
        std::time::Duration::from_secs(15)
    );
    assert_eq!(exam_tester.verbose(), true);
    assert_eq!(exam_tester.dry_run(), true);
}

#[test]
fn go_exam_en_dirs() {
    let args = Args::parse_from([
        "src2exam",
        "--directory",
        &GoExamEn.dir_str(),
        "--language",
        "en",
        "--timeout",
        "15",
        "--verbose",
        "--dry-run",
    ]);

    let exam_tester = args.exam_tester().unwrap();
    let exam_config = exam_tester.exam_config();

    let exam_dir = GoExamEn.dir().canonicalize().unwrap();
    assert_eq!(exam_config.base_dir(), exam_dir);
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("tasks"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("grading"));
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("submissions"));
    assert_eq!(
        exam_config.test_timeout(),
        std::time::Duration::from_secs(15)
    );
    assert_eq!(exam_tester.verbose(), true);
    assert_eq!(exam_tester.dry_run(), true);
}
