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
