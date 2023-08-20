use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use clonelet::clone;
use cssparser::{Parser, ParserInput, Token};
use itertools::Itertools;
use lightningcss::{
    css_modules::{self, CssModuleExport},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Features, Targets},
};

pub struct NameMapping {
    pub plain: String,
    pub mangled: String,
}

#[derive(Debug)]
pub struct Source {
    content: String,
    dependency: Option<String>,
}

impl Source {
    // TODO: Take a `Syntax` enum, and convert to css
    pub fn from_content(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            dependency: None,
        }
    }

    // TODO: Take an `Option<Syntax>` and convert to CSS
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

    // TODO: Remove
    pub fn map_content<E>(self, f: impl FnOnce(String) -> Result<String, E>) -> Result<Self, E> {
        Ok(Self {
            content: f(self.content)?,
            dependency: self.dependency,
        })
    }

    pub fn transpile(
        &mut self,
        validate: bool,
        transpile: Option<Transpile>,
    ) -> Result<Option<Vec<NameMapping>>, String> {
        if validate || transpile.is_some() {
            let modules = transpile.as_ref().map_or(false, |t| t.modules);
            let nesting = transpile.as_ref().map_or(false, |t| t.nesting);

            clone!(self.content);
            let warnings = validate.then(|| Arc::new(RwLock::new(Vec::new())));
            let filename = self
                .dependency
                .as_ref()
                .map_or_else(|| "<content>".to_string(), String::clone);
            let css_modules = modules.then(|| css_modules::Config {
                pattern: css_modules::Pattern::default(),
                dashed_idents: false,
            });
            let mut flags = ParserFlags::empty();
            flags.set(ParserFlags::NESTING, nesting);

            let mut stylesheet: StyleSheet = StyleSheet::parse(
                &content,
                ParserOptions {
                    filename,
                    css_modules,
                    source_index: 0,
                    error_recovery: !validate,
                    warnings: warnings.as_ref().map(Arc::clone),
                    flags,
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

            if let Some(Transpile {
                minify,
                pretty,
                browsers,
                ..
            }) = transpile
            {
                let targets = Targets {
                    browsers: browsers.map(Browsers::into),
                    include: Features::empty(),
                    exclude: Features::empty(),
                };

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

                return css.exports.map(Self::name_mappings).transpose();
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

    fn name_mappings(
        exports: HashMap<String, CssModuleExport>,
    ) -> Result<Vec<NameMapping>, String> {
        exports
            .into_iter()
            .map(|(plain, mapping)| {
                if mapping.composes.is_empty() {
                    Ok(NameMapping {
                        plain,
                        mangled: mapping.name,
                    })
                } else {
                    Err("`composes` is unsupported".to_string())
                }
            })
            .collect()
    }
}

pub struct Transpile {
    pub minify: bool,
    pub pretty: bool,
    pub modules: bool,
    pub nesting: bool,
    pub browsers: Option<Browsers>,
}

#[derive(Default)]
pub struct Browsers {
    pub android: Option<Version>,
    pub chrome: Option<Version>,
    pub edge: Option<Version>,
    pub firefox: Option<Version>,
    pub ie: Option<Version>,
    pub ios_saf: Option<Version>,
    pub opera: Option<Version>,
    pub safari: Option<Version>,
    pub samsung: Option<Version>,
}

impl From<Browsers> for lightningcss::targets::Browsers {
    fn from(value: Browsers) -> Self {
        Self {
            android: value.android.map(Version::encode_for_lightning),
            chrome: value.chrome.map(Version::encode_for_lightning),
            edge: value.edge.map(Version::encode_for_lightning),
            firefox: value.firefox.map(Version::encode_for_lightning),
            ie: value.ie.map(Version::encode_for_lightning),
            ios_saf: value.ios_saf.map(Version::encode_for_lightning),
            opera: value.opera.map(Version::encode_for_lightning),
            safari: value.safari.map(Version::encode_for_lightning),
            samsung: value.samsung.map(Version::encode_for_lightning),
        }
    }
}

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    fn encode_for_lightning(self) -> u32 {
        u32::from_be_bytes([0, self.major, self.minor, self.patch])
    }
}

// TODO: Make this a method
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

// TODO: Make this a method
pub fn variable_names(css: &Source) -> impl Iterator<Item = String> {
    let mut parser_input = ParserInput::new(&css.content);
    let mut input = Parser::new(&mut parser_input);
    let mut variables = HashSet::new();
    let mut tokens = Vec::new();

    flattened_tokens(&mut tokens, &mut input);

    for token in tokens {
        if let Token::Ident(ident) = token {
            if let Some(var) = ident.strip_prefix("--") {
                variables.insert(var.to_string());
            }
        }
    }

    variables.into_iter()
}

fn flattened_tokens<'i>(tokens: &mut Vec<Token<'i>>, input: &mut Parser<'i, '_>) {
    while let Ok(token) = input.next_including_whitespace_and_comments() {
        tokens.push(token.clone());

        match token {
            Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock
            | Token::Function(_) => {
                input
                    .parse_nested_block(|parser| -> Result<(), cssparser::ParseError<()>> {
                        flattened_tokens(tokens, parser);
                        Ok(())
                    })
                    .unwrap();
            }
            _ => (),
        }
    }
}
