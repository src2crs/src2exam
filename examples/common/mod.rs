use std::path::PathBuf;

pub fn go_exam_dir() -> PathBuf {
    let cargo_manifest_dir_variable = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let base_dir = PathBuf::from(cargo_manifest_dir_variable);
    let examples_dir = base_dir.join("examples");
    let testdata_dir = examples_dir.join("testdata");
    testdata_dir.join("go_exam")
}

pub fn go_exam_dir_str() -> String {
    go_exam_dir().to_str().unwrap().to_string()
}
