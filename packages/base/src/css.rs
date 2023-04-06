use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use cssparser::{Parser, ParserInput, Token};
use itertools::Itertools;
use lightningcss::{
    css_modules::{self, CssModuleExports},
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Browsers,
};

#[derive(Debug)]
pub struct Source {
    content: String,
    dependency: Option<String>,
}

impl Source {
    pub fn from_content(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            dependency: None,
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

        let root_dir = env::var(CARGO_MANIFEST_DIR).map_err(|e| {
            format!("Error reading environment variable '{CARGO_MANIFEST_DIR}': {e}")
        })?;
        let path = PathBuf::from(root_dir)
            .join(path)
            .into_os_string()
            .into_string()
            .expect("Expected path to be convertible to string");

        Ok(Self {
            content: fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read '{path}': {e}"))?,
            dependency: Some(path),
        })
    }

    pub fn transpile(
        &mut self,
        validate: bool,
        transpile: Option<Transpile>,
    ) -> Result<Option<CssModuleExports>, String> {
        let write_content = transpile.is_some();

        if validate || write_content {
            let minify = transpile.as_ref().map_or(false, |t| t.minify);
            let pretty = transpile.as_ref().map_or(false, |t| t.pretty);
            let modules = transpile.as_ref().map_or(false, |t| t.modules);
            let nesting = transpile.as_ref().map_or(false, |t| t.nesting);
            let targets = transpile.and_then(|t| t.browsers);

            let content = self.content.clone();
            let warnings = validate.then(|| Arc::new(RwLock::new(Vec::new())));
            let filename = self
                .dependency
                .as_ref()
                .map_or_else(|| "<content>".to_string(), String::clone);
            let css_modules = modules.then(|| css_modules::Config {
                pattern: css_modules::Pattern::default(),
                dashed_idents: false,
            });
            let mut stylesheet: StyleSheet = StyleSheet::parse(
                &content,
                ParserOptions {
                    filename,
                    nesting,
                    custom_media: false,
                    css_modules,
                    source_index: 0,
                    error_recovery: !validate,
                    warnings: warnings.as_ref().map(Arc::clone),
                },
            )
            .map_err(|e| e.to_string())?;

            if let Some(warnings) = warnings {
                let warnings = warnings.read().unwrap();

                if !warnings.is_empty() {
                    let warnings_text = warnings.iter().map(|w| w.to_string()).join("\n");

                    return Err(warnings_text);
                }
            }

            if write_content {
                if minify {
                    // This does the structural minification and add/removes vendor prefixes.
                    stylesheet
                        .minify(MinifyOptions {
                            targets,
                            unused_symbols: HashSet::new(),
                        })
                        .map_err(|e| e.to_string())?;
                }

                let css = stylesheet
                    .to_css(PrinterOptions {
                        // `minify` just controls the output format without doing more structural
                        // minification.
                        minify: !pretty && minify,
                        source_map: None,
                        project_root: None,
                        targets,
                        analyze_dependencies: None,
                        pseudo_classes: None,
                    })
                    .map_err(|e| e.to_string())?;
                self.content = css.code;

                return Ok(css.exports);
            }
        }

        Ok(None)
    }

    pub fn dependency(&self) -> &Option<String> {
        &self.dependency
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct Transpile {
    pub minify: bool,
    pub pretty: bool,
    pub modules: bool,
    pub nesting: bool,
    pub browsers: Option<Browsers>,
}

pub fn class_names(css: &Source) -> impl Iterator<Item = String> {
    let mut parser_input = ParserInput::new(&css.content);
    let mut input = Parser::new(&mut parser_input);
    let mut classes = HashSet::new();
    let mut prev_dot = false;

    while let Ok(token) = input.next_including_whitespace_and_comments() {
        if prev_dot {
            if let Token::Ident(class) = token {
                classes.insert(class.to_string());
            }
        }

        prev_dot = matches!(token, Token::Delim('.'));
    }

    classes.into_iter()
}

pub fn variable_names(css: &Source) -> impl Iterator<Item = String> {
    let mut parser_input = ParserInput::new(&css.content);
    let mut input = Parser::new(&mut parser_input);
    let mut variables = HashSet::new();

    while let Ok(token) = input.next_including_whitespace_and_comments() {
        if let Token::Ident(ident) = token {
            if let Some(variable) = ident.strip_prefix("--") {
                variables.insert(variable.to_string());
            }
        }
    }

    variables.into_iter()
}
