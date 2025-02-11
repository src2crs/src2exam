use clap::Parser;
use std::path::PathBuf;

use crate::exam_tester::exam::{ExamInfo, ExamInfoLanguage, ExamTester};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    /// The timeout for running the tests in seconds.
    #[arg(short, long, default_value = ExamInfo::test_timeout_default().as_secs().to_string())]
    timeout: u64,
    /// The language to use for the exam.
    #[arg(short, long, default_value = "de")]
    language: ExamInfoLanguage,
    /// Print information about the exam.
    #[arg(short, long)]
    verbose: bool,
    /// Only print information about the exam and exit.
    #[arg(short = 'n', long)]
    dry_run: bool,
}

impl Args {
    /// Returns the base directory to use.
    /// This is the stored directory if it is absolute,
    /// otherwise it is the stored directory relative to the current directory.
    pub fn base_dir(&self) -> PathBuf {
        let base_dir = &self.directory;
        if base_dir.is_relative() {
            std::env::current_dir().unwrap().join(base_dir)
        } else {
            base_dir.clone()
        }
    }

    /// Returns an exam info based on the arguments.
    pub fn exam_info(&self) -> ExamInfo {
        let mut exam_info = ExamInfo::new(self.language());
        exam_info.set_base_dir(self.base_dir());
        exam_info.set_test_timeout(self.timeout);
        exam_info
    }

    /// Returns whether the verbose mode option is set.
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    /// Returns whether the dry run mode option is set.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Returns the language to use for the exam.
    pub fn language(&self) -> ExamInfoLanguage {
        self.language.clone()
    }
}

impl From<Args> for ExamInfo {
    fn from(args: Args) -> Self {
        ExamInfo::from(&args)
    }
}

impl From<&Args> for ExamInfo {
    fn from(args: &Args) -> Self {
        args.exam_info()
    }
}

impl From<Args> for ExamTester {
    fn from(args: Args) -> Self {
        ExamTester::from(&args)
    }
}

impl From<&Args> for ExamTester {
    fn from(args: &Args) -> Self {
        let exam_info = ExamInfo::from(args);
        ExamTester::new(exam_info, args.verbose(), args.dry_run())
    }
}
