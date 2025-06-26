use super::*;
use std::time::Duration;

#[test]
fn values_for_default_struct() {
    let args = Args::default();

    assert_eq!(args.directory, Args::default_path());
    assert_eq!(args.timeout, Args::default_timeout());
    assert!(!args.verbose);
    assert!(!args.dry_run);

    assert_eq!(
        args.base_dir(),
        Ok(Args::default_path().canonicalize().unwrap())
    );
}

#[test]
fn values_for_custom_struct() {
    let args = Args::new("base_dir", 5u64, true, false);

    assert_eq!(args.directory, PathBuf::from("base_dir"));
    assert_eq!(args.timeout, 5);
    assert!(args.verbose);
    assert!(!args.dry_run);

    assert_eq!(
        args.base_dir(),
        Ok(std::env::current_dir().unwrap().join("base_dir"))
    );
}

#[test]
fn exam_config_for_default_struct_() {
    let args = Args::default();
    let exam_config = args.exam_config().unwrap();

    let expected_base_dir = Args::default_path().canonicalize().unwrap();
    let expected_timeout = Duration::from_secs(Args::default_timeout());

    // TODO: Also test directories.

    assert_eq!(exam_config.base_dir(), expected_base_dir);
    assert_eq!(exam_config.test_timeout(), expected_timeout);
}

#[test]
fn exam_config_for_custom_struct() {
    let args = Args::new("custom_base_dir", 60u64, true, true);
    let exam_config = args.exam_config().unwrap();

    let expected_base_dir = std::env::current_dir().unwrap().join("custom_base_dir");
    let expected_timeout = Duration::from_secs(60);

    // TODO: Also test directories.

    assert_eq!(exam_config.base_dir(), expected_base_dir);
    assert_eq!(exam_config.test_timeout(), expected_timeout);
}
