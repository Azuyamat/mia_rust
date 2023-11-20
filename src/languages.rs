use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LanguageConfig {
    extensions: Vec<String>,
    language: Language,
}

#[derive(Debug, Deserialize)]
struct Config {
    languages: Vec<LanguageConfig>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq, Hash, Deserialize, Copy, Clone)]
pub enum Language {
    None,
    Rust,
    Python,
    Java,
    C,
    CPP,
    JavaScript,
    HTML,
    CSS,
    PHP,
    Swift,
    Ruby,
    Go,
    Kotlin,
    Scala,
    TypeScript,
    Lua,
    Dart,
    Markdown
}

const LANGUAGES_TOML: &str = include_str!("languages.toml");

fn lang_config() -> Config {
    toml::from_str(LANGUAGES_TOML).expect("Failed to parse configuration")
}

pub fn detect_language(extension: &String) -> Language {
    let config = lang_config();
    config
        .languages
        .iter()
        .find(|lang| lang.extensions.iter().any(|ext| ext == extension))
        .map(|lang| lang.language)
        .unwrap_or(Language::None)
}