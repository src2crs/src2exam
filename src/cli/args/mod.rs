use crate::ExamConfig;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = ExamConfig::default_base_dirname())]
    directory: PathBuf,
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
