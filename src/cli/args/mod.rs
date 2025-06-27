use crate::ExamConfig;
use clap::Parser;
use std::path::PathBuf;

// TODO: Don't export the base_directory to files.
// TODO: Is it cleaner to distinguish between configs that are read from files,
//       configs that are created from CLI arguments,
//       and the config that is actually used by the exam tester?
//       Or maybe the base directory should be a property of the exam tester,
//       instead of the config? Use a trait to describe the config and parameterize
//       the exam tester over it?
// TODO: Decide what role the CLI args play in this context.

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
    /// The path to the file containing the exam configuration.
    /// If it exists, it will be used and all other CLI arguments will be ignored.
    #[arg(short, long)]
    config_path: Option<PathBuf>,
    /// If set, a new config file will be created with the default values.
    /// The name of the file will be determined by the `config_path` argument.
    #[arg(long)]
    create_config: bool,
}

mod access;
mod constructors;

#[cfg(test)]
mod tests;
