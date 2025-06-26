use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct CliError {
    message: String,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        CliError { message }
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        CliError::from(message.to_string())
    }
}
