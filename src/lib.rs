//! Create HTML renderings of code with highlighting using
//! [tree-sitter](https://github.com/tree-sitter/tree-sitter)
//!
//! The general workflow of this is to simplify the workflow of creating pretty html code blocks
//! using tree-sitter.
//!
//! ```
//! # use highlight_ez::{render_html, TargetLanguage};
//! let my_pyblock = r#"def fib(a):
//!     if a = 1:
//!         return 1
//!     else:
//!         return fib(a - 1)"#;
//! let lang = TargetLanguage::Python;
//! let html = render_html(my_pyblock, lang);
//! ```
mod error;
mod target_language;

use anyhow::Result;
use std::fmt::Write;
use tree_sitter_cli::{generate, highlight::Theme};
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::{Highlighter, HtmlRenderer};
use tree_sitter_loader::Loader;

pub use target_language::TargetLanguage;

/// Take generated arguments and make the calls to tree-sitter
fn string_html(
    loader: &Loader,
    theme: &Theme,
    source: &[u8],
    config: &HighlightConfiguration,
    _quiet: bool,
    _print_time: bool,
    _cancellation_flag: Option<&usize>,
) -> Result<String> {
    let mut highlighter = Highlighter::new();

    let mut out_str = String::new();

    let events = highlighter.highlight(config, source, None, |string| {
        loader.highlight_config_for_injection_string(string)
    })?;

    let mut renderer = HtmlRenderer::new();
    renderer.render(events, source, &move |highlight| {
        theme.styles[highlight.0]
            .css
            .as_ref()
            .map_or_else(|| "".as_bytes(), |css_style| css_style.as_bytes())
    })?;

    writeln!(&mut out_str, "<table>")?;
    for (i, line) in renderer.lines().enumerate() {
        writeln!(
            &mut out_str,
            "<tr><td class=line-number>{}</td><td class=line>{line}</td></tr>",
            i + 1,
        )?;
    }

    writeln!(&mut out_str, "</table>")?;

    Ok(out_str)
}

/// Render the code block into HTML, using the styling defaulted to by tree-sitter
pub fn render_html(code_block: &str, lang: TargetLanguage) -> Result<String> {
    if check_for_parser(lang).is_err() {
        generate_parser(lang)?;
    }
    // FROM tree-sitter-cli
    let mut loader = tree_sitter_loader::Loader::new().unwrap();
    let config = tree_sitter_config::Config::load(None).unwrap();
    let theme_config: tree_sitter_cli::highlight::ThemeConfig = config.get().unwrap();
    loader.configure_highlights(&theme_config.theme.highlight_names);
    let loader_config = config.get().unwrap();
    loader.find_all_languages(&loader_config).unwrap();

    let extension = match lang.extension() {
        Some(e) => e,
        None => return Err(error::HtmlRenderingError::LanguageParserNotImplemented.into()),
    };

    let (language, language_config) = match loader.language_configuration_for_file_name(
        &std::path::Path::new("dummy-name").with_extension(extension),
    )? {
        Some(lang) => lang,
        None => {
            return Ok("".into());
        }
    };

    let highlight_config = language_config
        .highlight_config(language, None)
        .unwrap()
        .unwrap();

    let source = code_block.as_bytes();
    string_html(
        &loader,
        &theme_config.theme,
        &source,
        highlight_config,
        false,
        false,
        None,
    )
}

/// Generate a parser for the language by calling tree-sitter
#[cfg(target_os = "linux")]
pub fn generate_parser(lang: TargetLanguage) -> Result<()> {
    /*
    Source for ABI version: https://github.com/tree-sitter/tree-sitter/blob/master/cli/src/main.rs
    */
    const ABI_VERSION: usize = 14;
    let current_dir = std::path::PathBuf::from(std::env::current_dir()?);
    let home_path = match std::env::home_dir() {
        Some(p) => p,
        None => return Err(error::HtmlRenderingError::SharedLibDoesntExist.into()),
    };
    log::trace!("found home path {:?}", home_path);

    let git_url = match lang.git_repo() {
        Some(n) => n,
        None => return Err(error::HtmlRenderingError::LanguageParserNotImplemented.into()),
    };
    let cache_path = home_path.join(".cache").join("tree-sitter").join("lib");
    let soname = match lang.soname() {
        Some(s) => s,
        None => return Err(error::HtmlRenderingError::SharedLibDoesntExist.into()),
    };
    let sopath = cache_path.join(soname);
    log::trace!("found sopath path {:?}", sopath);
    let repo_name = std::path::Path::new(git_url)
        .file_name()
        .unwrap()
        .to_string_lossy();
    let repo_name = repo_name.split(".").next().unwrap();

    let repo_path = home_path
        .join(".cache")
        .join("tree-sitter")
        .join("parsers")
        .join(repo_name);

    if !repo_path.exists() {
        log::debug!("Cloning Git repo {:?} to path {:?}", git_url, repo_path);
        git2::Repository::clone(git_url, repo_path.clone())?;
    }

    let grammar_path = repo_path.join("grammar.js");
    log::debug!("Grammar path is {}", grammar_path.display());
    if !grammar_path.exists() {
        log::debug!("Grammar path doesn't exist, this will probably not work");
    }
    let grammar_path = grammar_path.as_os_str();
    let grammar_path_str = grammar_path.to_str().unwrap();
    generate::generate_parser_in_directory(
        &repo_path,
        Some(grammar_path_str),
        ABI_VERSION,
        true,
        None,
        None,
    )?;
    log::trace!("generated parser for {:?}", lang);

    let mut loader = tree_sitter_loader::Loader::new().unwrap();
    loader.use_debug_build(false);
    loader.languages_at_path(&current_dir)?;
    // grammar path below is to git repo, not grammar.js/json
    loader.compile_parser_at_path(&repo_path, std::path::PathBuf::from(sopath), &[""])?;
    log::trace!("compiled parser for {:?}", lang);

    Ok(())
}

#[cfg(target_os = "linux")]
fn check_for_parser(target_lang: TargetLanguage) -> Result<()> {
    let home_path = match std::env::home_dir() {
        Some(p) => p,
        None => return Err(error::HtmlRenderingError::SharedLibDoesntExist.into()),
    };
    log::trace!("found home path {:?}", home_path);

    let cache_path = home_path.join(".cache").join("tree-sitter").join("lib");
    let soname = match target_lang.soname() {
        Some(s) => s,
        None => return Err(error::HtmlRenderingError::SharedLibDoesntExist.into()),
    };
    log::trace!("found cache path {:?}", cache_path);

    let sopath = cache_path.join(soname);
    log::trace!("Looking for sofile {:?}", sopath);

    if !sopath.exists() {
        log::error!("Unable to find sofile for treesitter parser");
        Err(error::HtmlRenderingError::SharedLibDoesntExist.into())
    } else {
        log::debug!("Found treesitter parser");
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::io::{Read, Write};

    #[test]
    fn render_html() {
        let mut source_file = std::fs::File::options()
            .read(true)
            .open("src/lib.rs")
            .unwrap();
        let mut source = String::new();
        source_file.read_to_string(&mut source).unwrap();

        let out = super::render_html(&source, super::TargetLanguage::Rust);
        let mut test_output = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open("test-html")
            .unwrap();
        write!(test_output, "{}", out.unwrap()).unwrap();
    }

    #[test]
    fn generate_parser() {
        if let Err(e) = super::generate_parser(crate::TargetLanguage::Rust) {
            eprintln!("Unable to create rust parser: {e}");
        }
        if let Err(e) = super::generate_parser(crate::TargetLanguage::Python) {
            eprintln!("Unable to create python parser: {e}");
        }
        if let Err(e) = super::generate_parser(crate::TargetLanguage::Html) {
            eprintln!("Unable to create html parser: {e}");
        }
        if let Err(e) = super::generate_parser(crate::TargetLanguage::Json) {
            eprintln!("Unable to create json parser: {e}");
        }
        if let Err(e) = super::generate_parser(crate::TargetLanguage::Javascript) {
            eprintln!("Unable to create js parser: {e}");
        }
        super::check_for_parser(crate::TargetLanguage::Rust).unwrap();
        super::check_for_parser(crate::TargetLanguage::Python).unwrap();
        super::check_for_parser(crate::TargetLanguage::Html).unwrap();
        super::check_for_parser(crate::TargetLanguage::Json).unwrap();
        super::check_for_parser(crate::TargetLanguage::Javascript).unwrap();
    }
}
