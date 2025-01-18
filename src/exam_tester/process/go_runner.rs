use crate::exam_tester::exam::TestResult;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::Duration;

pub struct GoRunner {
    dir: PathBuf,
    timeout: Duration,
}

impl GoRunner {
    pub fn new<P: Into<PathBuf>, D: Into<Duration>>(dir: P, timeout: D) -> Self {
        Self {
            dir: dir.into(),
            timeout: timeout.into(),
        }
    }

    /// Runs the go build command in the given directory.
    /// Returns the output of the command.
    pub fn build(&self) -> Output {
        let output = Command::new("go")
            .current_dir(&self.dir)
            .arg("build")
            .arg(".")
            .output()
            .expect("Failed to run go build");
        output
    }

    /// Runs the go test command in the given directory with the given timeout.
    /// Returns the output of the command.
    pub fn run_tests(&self) -> TestResult {
        let timeout = format!("-timeout={}s", self.timeout.as_secs());
        let output = Command::new("go")
            .current_dir(&self.dir)
            .arg("test")
            .arg("-v")
            .arg(timeout)
            .arg(".")
            .output()
            .expect("Failed to run go test");

        let output_string = String::from_utf8(output.stdout).unwrap();
        TestResult::from_go_test_output(output_string.as_str())
    }
}
