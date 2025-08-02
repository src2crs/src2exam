use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum CodeLang {
    #[default]
    #[serde(alias = "go")]
    Go,
    #[serde(alias = "cpp")]
    #[serde(alias = "c++")]
    #[serde(alias = "CPP")]
    #[serde(alias = "C++")]
    Cpp,
}

impl std::fmt::Display for CodeLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLang::Go => write!(f, "Go"),
            CodeLang::Cpp => write!(f, "C++"),
        }
    }
}

impl TryFrom<String> for CodeLang {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Go" => Ok(CodeLang::Go),
            "go" => Ok(CodeLang::Go),
            "C++" => Ok(CodeLang::Cpp),
            "c++" => Ok(CodeLang::Cpp),
            "CPP" => Ok(CodeLang::Cpp),
            "cpp" => Ok(CodeLang::Cpp),
            _ => Err(format!("Invalid coding language: {}", value)),
        }
    }
}
