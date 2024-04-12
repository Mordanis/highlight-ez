use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug, Clone)]
pub enum HtmlRenderingError {
    #[error("Could not find shared library for language parser")]
    SharedLibDoesntExist,
    #[error("Language is not imported")]
    LanguageParserNotImplemented,
}
