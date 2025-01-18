/// Represents the result of a test run.
/// Note that, in contrast to regular testing in software development,
/// everything is a regular test result, including crashes, build failures, etc.
#[derive(Debug, PartialEq)]
pub enum TestResult {
    /// Successful test run.
    Success,
    /// Regular test run failure.
    /// I.e. tests run, but assertions fail.
    TestFailure,
    /// Stack Overflow during test run.
    StackOverflow,
    /// General crash during run (e.g. panic/exception, segfault).
    Crash,
    /// Test times out.
    Timeout,
    /// Build failed.
    BuildFailure,
}

impl TestResult {
    /// Parses the output of a go test and returns the corresponding TestResult.
    /// Note: This parses a string and thus may not be reliable/future proof.
    pub fn from_go_test_output(test_output: &str) -> Self {
        let build_failure = test_output.contains("[build failed]");
        let timeout = test_output.contains("panic: test timed out after");
        let stack_overflow = test_output.contains("runtime: goroutine stack exceeds");
        let panic = test_output.contains("panic:");
        let test_failure = test_output.contains("FAIL");

        use TestResult::*;
        if build_failure {
            BuildFailure
        } else if timeout {
            Timeout
        } else if stack_overflow {
            StackOverflow
        } else if panic {
            Crash
        } else if test_failure {
            TestFailure
        } else {
            Success
        }
    }

    /// Returns a string representation of the TestResult in English.
    pub fn to_string_en(&self) -> String {
        use TestResult::*;
        match self {
            Success => "Success",
            TestFailure => "Test Failure",
            StackOverflow => "Stack Overflow",
            Crash => "Crash",
            Timeout => "Timeout",
            BuildFailure => "Build Failure",
        }
        .to_string()
    }

    /// Returns a string representation of the TestResult in German.
    pub fn to_string_de(&self) -> String {
        use TestResult::*;
        match self {
            Success => "Ok",
            TestFailure => "Test-Fehlschlag",
            StackOverflow => "Stack Overflow",
            Crash => "Absturz",
            Timeout => "Timeout",
            BuildFailure => "Build-Fehlschlag",
        }
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod build_results {
        pub fn build_failure() -> String {
            vec!["...", "FAIL    path/to/module.go [build failed]", "..."].join("\n")
        }

        pub fn test_failure() -> String {
            vec!["...", "FAIL    path/to/module.go      0.123s", "..."].join("\n")
        }

        pub fn timeout() -> String {
            vec![
                "...",
                "panic: test timed out after 5s",
                "...",
                "FAIL    path/to/module.go      5.432s",
                "...",
            ]
            .join("\n")
        }

        pub fn stack_overflow() -> String {
            vec![
                "...",
                "runtime: goroutine stack exceeds 1000000000-byte limit",
                "...",
                "panic: runtime error: makeslice: len out of range",
                "...",
                "FAIL    path/to/module.go      0.123s",
                "...",
            ]
            .join("\n")
        }

        pub fn crash() -> String {
            vec![
                "...",
                "panic: runtime error: makeslice: len out of range",
                "...",
                "FAIL    path/to/module.go      0.123s",
                "...",
            ]
            .join("\n")
        }

        pub fn success() -> String {
            return vec!["...", "ok", "..."].join("\n");
        }
    }

    #[test]
    fn build_failure() {
        let result = TestResult::from_go_test_output(&build_results::build_failure());
        assert_eq!(result, TestResult::BuildFailure);
    }

    #[test]
    fn test_failure() {
        let result = TestResult::from_go_test_output(&build_results::test_failure());
        assert_eq!(result, TestResult::TestFailure);
    }

    #[test]
    fn timeout() {
        let result = TestResult::from_go_test_output(&build_results::timeout());
        assert_eq!(result, TestResult::Timeout);
    }

    #[test]
    fn stack_overflow() {
        let result = TestResult::from_go_test_output(&build_results::stack_overflow());
        assert_eq!(result, TestResult::StackOverflow);
    }

    #[test]
    fn crash() {
        let result = TestResult::from_go_test_output(&build_results::crash());
        assert_eq!(result, TestResult::Crash);
    }

    #[test]
    fn success() {
        let result = TestResult::from_go_test_output(&build_results::success());
        assert_eq!(result, TestResult::Success);
    }
}
