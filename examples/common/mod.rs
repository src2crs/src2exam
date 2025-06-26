use std::path::PathBuf;

pub fn exam_dir() -> PathBuf {
    let cargo_manifest_dir_variable = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let base_dir = PathBuf::from(cargo_manifest_dir_variable);
    base_dir.join("testdata").join("go-exam")
}
