use crate::ExamConfig;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = ExamConfig::default_base_dirname())]
    base_directory: PathBuf,
    /// The name of the subdirectory for the tasks.
    #[arg(long, default_value = ExamConfig::default_tasks_dirname())]
    tasks_dirname: String,
    /// The name of the subdirectory for the submissions.
    #[arg(long, default_value = ExamConfig::default_submissions_dirname())]
    submissions_dirname: String,
    /// The name of the subdirectory to be used for grading.
    #[arg(long, default_value = ExamConfig::default_grading_dirname())]
    grading_dirname: String,
    /// The timeout for running the tests in seconds.
    #[arg(short, long, default_value = ExamConfig::default_timeout_secs().to_string(), value_parser = clap::value_parser!(u64).range(1..))]
    timeout: u64,
    /// Print information about the exam.
    #[arg(short, long)]
    verbose: bool,
    /// Only print information about the exam and exit.
    #[arg(short = 'n', long)]
    dry_run: bool,
}

mod access;
mod constructors;

#[cfg(test)]
mod tests;
