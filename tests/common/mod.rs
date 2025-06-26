pub fn crate_root_dir() -> std::path::PathBuf {
    let cargo_manifest_dir_variable = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::path::PathBuf::from(cargo_manifest_dir_variable)
}

pub fn tests_dir() -> std::path::PathBuf {
    crate_root_dir().join("tests")
}

pub fn testdata_dir() -> std::path::PathBuf {
    tests_dir().join("testdata")
}

pub fn go_exams_dir() -> std::path::PathBuf {
    testdata_dir().join("go_exams")
}

pub enum TestCases {
    GoExamDe,
    GoExamEn,
}

impl TestCases {
    pub fn dir(&self) -> std::path::PathBuf {
        match self {
            TestCases::GoExamDe => go_exams_dir().join("de"),
            TestCases::GoExamEn => go_exams_dir().join("en"),
        }
    }

    pub fn dir_str(&self) -> String {
        self.dir().to_string_lossy().to_string()
    }
}
