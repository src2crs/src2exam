use crate::ExamConfig;

#[derive(Debug, Clone)]
pub enum Language {
    De,
    En,
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
        match lang {
            Language::De => ExamConfig::new_de(),
            Language::En => ExamConfig::new_en(),
        }
    }
}
