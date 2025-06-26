use super::ExamConfig;

/// Conversion from/to the TOML format.
impl ExamConfig {
    /// Converts the ExamConfig to a TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Converts a TOML string to an ExamConfig.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}
