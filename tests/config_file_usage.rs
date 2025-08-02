pub mod common;
use common::TestCases::{GoExamDe, GoExamEn};

use clap::Parser;
use src2exam::{cli::Args, exam_config::CodeLang};

#[test]
fn go_exam_en_dirs() {
    let lang = GoExamEn;

    let args = Args::parse_from([
        "src2exam",
        "--base-directory",
        &lang.dir_str(),
        "--config-path",
        "config.toml",
        "--verbose",
        "--dry-run",
    ]);

    let exam_tester = args.exam_tester();
    let exam_config = exam_tester.exam_config();

    let exam_dir = lang.dir().canonicalize().unwrap();
    assert_eq!(exam_config.base_dir(), exam_dir);
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("tasks"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("grading"));
    assert_eq!(exam_config.coding_language(), CodeLang::Go);
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("submissions"));
    assert_eq!(
        exam_config.test_timeout(),
        std::time::Duration::from_secs(15)
    );
    assert_eq!(exam_tester.verbose(), true);
    assert_eq!(exam_tester.dry_run(), true);
}

#[test]
fn go_exam_de_dirs() {
    let lang = GoExamDe;

    let args = Args::parse_from([
        "src2exam",
        "--base-directory",
        &lang.dir_str(),
        "--config-path",
        "config.toml",
        "--verbose",
        "--dry-run",
    ]);

    let exam_tester = args.exam_tester();
    let exam_config = exam_tester.exam_config();

    let exam_dir = lang.dir().canonicalize().unwrap();
    assert_eq!(exam_config.base_dir(), exam_dir);
    assert_eq!(exam_config.tasks_dir(), exam_dir.join("aufgaben"));
    assert_eq!(exam_config.grading_dir(), exam_dir.join("bewertung"));
    assert_eq!(exam_config.coding_language(), CodeLang::Go);
    assert_eq!(exam_config.submissions_dir(), exam_dir.join("abgaben"));
    assert_eq!(
        exam_config.test_timeout(),
        std::time::Duration::from_secs(15)
    );
    assert_eq!(exam_tester.verbose(), true);
    assert_eq!(exam_tester.dry_run(), true);
}
