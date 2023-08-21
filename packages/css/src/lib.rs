use std::{
    collections::HashSet,
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use cssparser::{Parser, ParserInput, Token};
use derive_more::Into;
use grass::InputSyntax;

#[cfg_attr(feature = "css-transpile", path = "transpile-enabled.rs")]
#[cfg_attr(not(feature = "css-transpile"), path = "transpile-disabled.rs")]
mod transpile;

pub use transpile::Version;

pub struct NameMapping {
    pub plain: String,
    pub mangled: String,
}

#[derive(Into)]
pub struct CssSyntax(InputSyntax);

impl Default for CssSyntax {
    fn default() -> Self {
        Self(InputSyntax::Css)
    }
}

impl FromStr for CssSyntax {
    type Err = ();

    fn from_str(syntax: &str) -> Result<Self, Self::Err> {
        let syntax = match syntax {
            "css" => InputSyntax::Css,
            "scss" => InputSyntax::Scss,
            "sass" => InputSyntax::Sass,
            _ => return Err(()),
        };

        Ok(Self(syntax))
    }
}

impl CssSyntax {
    fn from_path(path: impl AsRef<Path>) -> Self {
        path.as_ref()
            .extension()
            .and_then(OsStr::to_str)
            .and_then(|ext| Self::from_str(ext.to_lowercase().as_str()).ok())
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct Css {
    content: String,
    dependency: Option<String>,
}

impl Css {
    // TODO: Don't use `String` errors
    pub fn from_content(content: impl Into<String>, syntax: CssSyntax) -> Result<Self, String> {
        Ok(Self {
            content: Self::css_content(content.into(), syntax)?,
            dependency: None,
        })
    }

    pub fn from_path(path: impl AsRef<Path>, syntax: Option<CssSyntax>) -> Result<Self, String> {
        let syntax = syntax.unwrap_or_else(|| CssSyntax::from_path(path.as_ref()));
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
            content: Self::css_content(
                fs::read_to_string(&path).map_err(|e| format!("Failed to read '{path}': {e}"))?,
                syntax,
            )?,
            dependency: Some(path),
        })
    }

    fn css_content(source: String, syntax: CssSyntax) -> Result<String, String> {
        let syntax = syntax.into();

        if syntax != InputSyntax::Css {
            grass::from_string(source, &grass::Options::default().input_syntax(syntax))
                .map_err(|e| e.to_string())
        } else {
            Ok(source)
        }
    }

    pub fn transpile(
        &mut self,
        validate: bool,
        transpile: Option<Transpile>,
    ) -> Result<Option<Vec<NameMapping>>, String> {
        if validate || transpile.is_some() {
            transpile::transpile(self, validate, transpile)
        } else {
            Ok(None)
        }
    }

    pub fn dependency(&self) -> &Option<String> {
        &self.dependency
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn class_names(&self) -> impl Iterator<Item = String> {
        let mut parser_input = ParserInput::new(&self.content);
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

    pub fn variable_names(&self) -> impl Iterator<Item = String> {
        let mut parser_input = ParserInput::new(&self.content);
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
