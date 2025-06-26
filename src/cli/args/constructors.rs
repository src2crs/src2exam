use super::Args;

use crate::cli::args::lang::Language;
use std::path::PathBuf;

/// Constructors
impl Args {
    /// Creates a new Args instance from the given values.
    pub fn new<P, T, L>(directory: P, timeout: T, language: L, verbose: bool, dry_run: bool) -> Self
    where
        P: Into<PathBuf>,
        T: Into<u64>,
        L: Into<Language>,
    {
        Self {
            directory: directory.into(),
            timeout: timeout.into(),
            language: language.into(),
            verbose,
            dry_run,
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new(
            Self::default_path(),
            Self::default_timeout(),
            Self::default_lang(),
            false,
            false,
        )
    }
}
