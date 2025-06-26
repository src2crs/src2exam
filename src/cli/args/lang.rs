use crate::ExamConfig;

use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Language {
    #[default]
    De,
    En,
}

impl Language {
    /// Tries to determine the language from the given base directory.
    /// Will check the subdirectories of the base directory
    /// for existing default tasks, submissions, and grading subdirectories.
    /// If the base directory does not contain any of the default subdirectories,
    /// or if the base directory is emptym, does not exist, or contains
    /// conflicting subdirectories, the default language will be returned.
    pub fn from_base_dir<P: Into<PathBuf>>(base_dir: P) -> Self {
        let base_dir = base_dir.into();

        if !base_dir.exists() || !base_dir.is_dir() {
            return Language::default();
        }

        let subdirs = match crate::filesystem::subdir_names(&base_dir) {
            Ok(subdirs) => subdirs,
            Err(_) => return Language::default(),
        };

        let dirs_de = [
            Language::De.default_tasks_subdir(),
            Language::De.default_submissions_subdir(),
            Language::De.default_grading_subdir(),
        ];

        let dirs_en = [
            Language::En.default_tasks_subdir(),
            Language::En.default_submissions_subdir(),
            Language::En.default_grading_subdir(),
        ];

        let has_any_de = dirs_de.iter().any(|d| subdirs.contains(d));
        let has_any_en = dirs_en.iter().any(|d| subdirs.contains(d));

        if has_any_de && !has_any_en {
            return Language::De;
        } else if has_any_en && !has_any_de {
            return Language::En;
        }

        Language::default()
    }

    /// Tries to determine the language either from the given base directory
    /// or from the provided string.
    /// If neither is possible, the default language will be returned.
    pub fn from_base_dir_or_str<P: Into<PathBuf>, S: Into<String>>(base_dir: P, lang: S) -> Self {
        let base_lang = Language::from_base_dir(base_dir);
        if base_lang != Language::default() {
            return base_lang;
        }

        let lang_str = lang.into().to_lowercase();
        match lang_str.as_str() {
            "de" => Language::De,
            "en" => Language::En,
            _ => Language::default(),
        }
    }

    /// Returns the default tasks subdirectory name for the language.
    pub fn default_tasks_subdir(&self) -> String {
        match self {
            Language::De => "aufgaben",
            Language::En => "tasks",
        }
        .to_string()
    }

    /// Returns the default submissions subdirectory name for the language.
    pub fn default_submissions_subdir(&self) -> String {
        match self {
            Language::De => "abgaben",
            Language::En => "submissions",
        }
        .to_string()
    }

    /// Returns the default grading subdirectory name for the language.
    pub fn default_grading_subdir(&self) -> String {
        match self {
            Language::De => "bewertung",
            Language::En => "grading",
        }
        .to_string()
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
            "auto" => Language::default(),
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

    #[test]
    fn language_from_base_dir_de() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let tasks_dir = temp_dir_path.join("aufgaben");
        let submissions_dir = temp_dir_path.join("abgaben");
        let grading_dir = temp_dir_path.join("bewertung");

        use std::fs;
        fs::create_dir(&tasks_dir).unwrap();
        fs::create_dir(&submissions_dir).unwrap();
        fs::create_dir(&grading_dir).unwrap();

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::De);
    }

    #[test]
    fn language_from_base_dir_en() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let tasks_dir = temp_dir_path.join("tasks");
        let submissions_dir = temp_dir_path.join("submissions");
        let grading_dir = temp_dir_path.join("grading");

        use std::fs;
        fs::create_dir(&tasks_dir).unwrap();
        fs::create_dir(&submissions_dir).unwrap();
        fs::create_dir(&grading_dir).unwrap();

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::En);
    }

    #[test]
    fn language_from_base_dir_mixed() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let tasks_dir = temp_dir_path.join("aufgaben");
        let submissions_dir = temp_dir_path.join("submissions");
        let grading_dir = temp_dir_path.join("grading");

        use std::fs;
        fs::create_dir(&tasks_dir).unwrap();
        fs::create_dir(&submissions_dir).unwrap();
        fs::create_dir(&grading_dir).unwrap();

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::default());
    }

    #[test]
    fn language_from_base_dir_empty() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::default());
    }

    #[test]
    fn language_from_base_dir_nonexistent() {
        let temp_dir_path = PathBuf::from("nonexistent_directory");

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::default());
    }

    #[test]
    fn language_from_base_dir_invalid() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Create a file instead of a directory
        let file_path = temp_dir_path.join("not_a_directory.txt");
        std::fs::write(&file_path, "This is a file, not a directory.").unwrap();

        let lang = Language::from_base_dir(temp_dir_path);
        assert_eq!(lang, Language::default());
    }

    #[test]
    fn language_from_str() {
        assert_eq!(Language::from("de"), Language::De);
        assert_eq!(Language::from("en"), Language::En);
    }
}
