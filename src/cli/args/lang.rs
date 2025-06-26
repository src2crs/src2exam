use crate::ExamConfig;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Language {
    #[default]
    De,
    En,
}

impl Language {
    /// Returns the default tasks subdirectory name for the language.
    pub fn default_tasks_subdir(&self) -> &str {
        match self {
            Language::De => "aufgaben",
            Language::En => "tasks",
        }
    }

    /// Returns the default submissions subdirectory name for the language.
    pub fn default_submissions_subdir(&self) -> &str {
        match self {
            Language::De => "abgaben",
            Language::En => "submissions",
        }
    }

    /// Returns the default grading subdirectory name for the language.
    pub fn default_grading_subdir(&self) -> &str {
        match self {
            Language::De => "bewertung",
            Language::En => "grading",
        }
    }

    /// Returns a string representation of the language.
    pub fn as_str(&self) -> &str {
        match self {
            Language::De => "de",
            Language::En => "en",
        }
    }
}

impl From<&str> for Language {
    fn from(lang: &str) -> Self {
        match lang.to_lowercase().as_str() {
            "de" => Language::De,
            "en" => Language::En,
            _ => panic!("Unsupported language: {}", lang),
        }
    }
}

impl From<Language> for ExamConfig {
    fn from(lang: Language) -> Self {
        ExamConfig::default()
            .with_tasks_subdir(lang.default_tasks_subdir())
            .with_submissions_subdir(lang.default_submissions_subdir())
            .with_grading_subdir(lang.default_grading_subdir())
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn new_examconfig_de_default_dir_paths() {
        let exam_info = ExamConfig::from(Language::De);

        let base_dir = PathBuf::from(".");
        let submissions_dir = PathBuf::from("./abgaben");
        let tasks_dir = PathBuf::from("./aufgaben");
        let grading_dir = PathBuf::from("./bewertung");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }

    #[test]
    fn new_examconfig_en_default_dir_paths() {
        let exam_info = ExamConfig::from(Language::En);

        let base_dir = PathBuf::from(".");
        let submissions_dir = PathBuf::from("./submissions");
        let tasks_dir = PathBuf::from("./tasks");
        let grading_dir = PathBuf::from("./grading");

        assert_eq!(exam_info.base_dir().to_owned(), base_dir);
        assert_eq!(exam_info.submissions_dir(), submissions_dir,);
        assert_eq!(exam_info.tasks_dir(), tasks_dir);
        assert_eq!(exam_info.grading_dir(), grading_dir);
    }
}
