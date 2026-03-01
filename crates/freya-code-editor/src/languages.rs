use std::fmt::Display;

#[derive(Default, Clone, Debug, PartialEq, Copy)]
pub enum LanguageId {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Markdown,
    Toml,
    Json,
    #[default]
    Unknown,
}

impl Display for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => f.write_str("Rust"),
            Self::Python => f.write_str("Python"),
            Self::JavaScript => f.write_str("JavaScript"),
            Self::TypeScript => f.write_str("TypeScript"),
            Self::Markdown => f.write_str("Markdown"),
            Self::Toml => f.write_str("TOML"),
            Self::Json => f.write_str("JSON"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl LanguageId {
    pub fn parse(id: &str) -> Self {
        match id {
            "rs" => LanguageId::Rust,
            "py" => LanguageId::Python,
            "js" => LanguageId::JavaScript,
            "ts" => LanguageId::TypeScript,
            "md" => LanguageId::Markdown,
            "toml" => LanguageId::Toml,
            "json" => LanguageId::Json,
            _ => LanguageId::Unknown,
        }
    }
}
