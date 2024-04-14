use std::str::FromStr;

/// List of target-able languages for highlighting
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TargetLanguage {
    Rust,
    Python,
    Json,
    Yaml,
    Toml,
    Html,
    Javascript,
    Shell,
}

impl TargetLanguage {
    /// Retrieve the file extension associated with the language
    pub fn extension(&self) -> Option<&'static str> {
        match self {
            Self::Rust => Some(".rs"),
            Self::Yaml => Some(".yml"),
            Self::Python => Some(".py"),
            Self::Json => Some(".json"),
            Self::Javascript => Some(".rs"),
            Self::Shell => Some(".sh"),
            Self::Html => Some(".html"),
            Self::Toml => Some(".toml"),
        }
    }

    /// Try to parse a query string as a target language.
    /// Caller must provide a default TargetLanguage, and if we can't parse that directly, the
    /// caller will get back the default they provided
    pub fn parse_or_default(query: &str, default: Self) -> Self {
        match query.parse::<Self>() {
            Ok(lang) => lang,
            Err(_) => default,
        }
    }

    /// Get the name of the dynamic library associated with the language
    pub fn soname(&self) -> Option<&'static str> {
        match self {
            Self::Rust => Some("rust.so"),
            Self::Yaml => Some("yaml.so"),
            Self::Python => Some("python.so"),
            Self::Json => Some("json.so"),
            Self::Javascript => Some("javascript.so"),
            Self::Shell => Some("shell.so"),
            Self::Html => Some("html.so"),
            Self::Toml => Some("toml.so"),
        }
    }

    /// Find the git repository associated with the language
    pub fn git_repo(&self) -> Option<&'static str> {
        match self {
            Self::Rust => Some("https://github.com/tree-sitter/tree-sitter-rust.git"),
            Self::Yaml => Some("https://github.com/tree-sitter-grammars/tree-sitter-yaml.git"),
            Self::Python => Some("https://github.com/tree-sitter/tree-sitter-python.git"),
            Self::Json => Some("https://github.com/tree-sitter/tree-sitter-json.git"),
            Self::Javascript => Some("https://github.com/tree-sitter/tree-sitter-javascript.git"),
            Self::Shell => Some("https://github.com/tree-sitter/tree-sitter-bash.git"),
            Self::Html => Some("https://github.com/tree-sitter/tree-sitter-html.git"),
            Self::Toml => Some("https://github.com/ikatyang/tree-sitter-toml.git"),
        }
    }
}

impl FromStr for TargetLanguage {
    type Err = crate::error::HtmlRenderingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let query: &str = &s.to_lowercase();
        match query {
            "rust" => Ok(Self::Rust),
            "rs" => Ok(Self::Rust),
            ".rs" => Ok(Self::Rust),
            "python" => Ok(Self::Python),
            "py" => Ok(Self::Python),
            ".py" => Ok(Self::Python),
            "json" => Ok(Self::Json),
            ".json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            ".yaml" => Ok(Self::Yaml),
            ".yml" => Ok(Self::Yaml),
            "yml" => Ok(Self::Yaml),
            "toml" => Ok(Self::Toml),
            ".toml" => Ok(Self::Toml),
            "html" => Ok(Self::Html),
            ".html" => Ok(Self::Html),
            "javascript" => Ok(Self::Javascript),
            "js" => Ok(Self::Javascript),
            ".js" => Ok(Self::Javascript),
            "shell" => Ok(Self::Shell),
            "sh" => Ok(Self::Shell),
            ".sh" => Ok(Self::Shell),
            "zsh" => Ok(Self::Shell),
            "bash" => Ok(Self::Shell),
            _ => Err(crate::error::HtmlRenderingError::LanguageParserNotImplemented),
        }
    }
}
