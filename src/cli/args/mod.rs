use clap::Parser;
use std::path::PathBuf;

mod lang;
use lang::Language;

use crate::cli::CliError;
type Result<T> = std::result::Result<T, CliError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The directory to use as the base directory for the exam.
    #[arg(short, long, default_value = Self::default_path_str())]
    directory: PathBuf,
    /// The timeout for running the tests in seconds.
    #[arg(short, long, default_value = Self::default_timeout_str())]
    timeout: u64,
    /// The language to use for the exam.
    #[arg(short, long, default_value = Self::default_lang_str())]
    language: Language,
    /// Print information about the exam.
    #[arg(short, long)]
    verbose: bool,
    /// Only print information about the exam and exit.
    #[arg(short = 'n', long)]
    dry_run: bool,
}

/// Default values.
impl Args {
    pub fn default_path_str() -> &'static str {
        "."
    }

    pub fn default_path() -> PathBuf {
        PathBuf::from(Self::default_path_str())
    }

    pub fn default_timeout_str() -> &'static str {
        "30"
    }

    pub fn default_timeout() -> u64 {
        Self::default_timeout_str().parse().unwrap()
    }

    pub fn default_lang_str() -> &'static str {
        "de"
    }

    pub fn default_lang() -> Language {
        Self::default_lang_str().into()
    }
}

mod access;
mod constructors;

#[cfg(test)]
mod tests;
