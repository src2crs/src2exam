use clap::Parser;
use std::path::PathBuf;

use super::Language;
use crate::{ExamConfig, ExamTester};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    /// The timeout for running the tests in seconds.
    #[arg(short, long, default_value = ExamConfig::test_timeout_default().as_secs().to_string())]
    timeout: u64,
    /// The language to use for the exam.
    #[arg(short, long, default_value = "de")]
    language: Language,
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

    /// Returns an exam config based on the arguments.
    pub fn exam_config(&self) -> ExamConfig {
        let exam_config = ExamConfig::from(self.language())
            .with_base_dir(self.base_dir())
            .with_test_timeout(std::time::Duration::from_secs(self.timeout));
        exam_config
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
    pub fn language(&self) -> Language {
        self.language.clone()
    }
}

impl From<Args> for ExamConfig {
    fn from(args: Args) -> Self {
        ExamConfig::from(&args)
    }
}

impl From<&Args> for ExamConfig {
    fn from(args: &Args) -> Self {
        args.exam_config()
    }
}

impl From<Args> for ExamTester {
    fn from(args: Args) -> Self {
        ExamTester::from(&args)
    }
}

impl From<&Args> for ExamTester {
    fn from(args: &Args) -> Self {
        let exam_info = ExamConfig::from(args);
        ExamTester::new(exam_info, args.verbose(), args.dry_run())
    }
}
