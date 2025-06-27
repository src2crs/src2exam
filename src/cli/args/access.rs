use super::Args;

use crate::{ExamConfig, ExamTester};
use std::path::PathBuf;

/// Access
impl Args {
    /// Returns the base directory to use.
    /// This is the stored directory if it is absolute,
    /// otherwise it is the stored directory relative to the current directory.
    pub fn base_dir(&self) -> PathBuf {
        let base_dir = &self.base_directory;
        if base_dir.is_relative() {
            PathBuf::default().join(base_dir)
        } else {
            base_dir.clone()
        }
    }

    /// Returns an exam config based on the arguments.
    pub fn exam_config(&self) -> ExamConfig {
        ExamConfig::default()
            .with_base_dir(self.base_dir())
            .with_tasks_subdir(&self.tasks_dirname)
            .with_submissions_subdir(&self.submissions_dirname)
            .with_grading_subdir(&self.grading_dirname)
            .with_test_timeout(std::time::Duration::from_secs(self.timeout))
    }

    /// Returns an exam config from the config file path if that is set
    /// and the file contains a valid configuration.
    pub fn config_from_file(&self) -> Option<ExamConfig> {
        let path = self.config_file_path()?;
        if !path.exists() {
            eprintln!("Error: Configuration file {:?} does not exist.", path);
            return None;
        }

        let file_content = std::fs::read_to_string(&path).ok()?;
        match ExamConfig::from_toml(&file_content) {
            Ok(config) => Some(config.with_base_dir(self.base_dir())),
            Err(_) => None,
        }
    }

    /// Writes the exam configuration to config file path
    /// if the path is set and the option to create the config is enabled.
    /// If the path is not set, will log an error.
    pub fn write_config_to_file(&self) {
        if !self.create_config {
            return;
        }
        match &self.config_path {
            // TODO: Replace eprintln! with a proper logging mechanism.
            Some(path) => {
                std::fs::create_dir_all(path.parent().unwrap()).ok();
                let exam_config = self.exam_config();
                if let Ok(exam_config_toml) = exam_config.to_toml() {
                    std::fs::write(path, exam_config_toml)
                        .unwrap_or_else(|_| eprintln!("Error: Could not write to file {:?}", path));
                } else {
                    eprintln!("Error: Could not convert exam configuration to TOML.");
                }
            }
            None => {
                eprintln!("Error: No configuration file path specified.");
            }
        }
    }

    /// Returns an exam tester based on the arguments.
    pub fn exam_tester(&self) -> ExamTester {
        let exam_config = match self.config_from_file() {
            Some(config) => {
                eprintln!("Using exam configuration from file: {:?}", self.config_path);
                config
            }
            None => {
                eprintln!("Using exam configuration from parameters.");
                self.exam_config()
            }
        };

        ExamTester::new(exam_config, self.verbose(), self.dry_run())
    }

    /// Returns whether the verbose mode option is set.
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    /// Returns whether the dry run mode option is set.
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    /// Returns the path to the configuration file, if any.
    pub fn config_path(&self) -> Option<PathBuf> {
        self.config_path.clone()
    }

    /// Returns the absolute path to the base directory.
    pub fn working_dir(&self) -> PathBuf {
        self.base_dir().canonicalize().unwrap_or_else(|_| {
            eprintln!(
                "Error: Could not canonicalize base directory {:?}",
                self.base_dir()
            );
            PathBuf::default()
        })
    }

    /// Returns the absolute path to the config file, if it is set.
    pub fn config_file_path(&self) -> Option<PathBuf> {
        Some(self.working_dir().join(self.config_path.as_ref()?))
    }
}
