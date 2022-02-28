use std::{
    cell::RefCell,
    collections::HashSet,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use cssparser::{
    AtRuleParser, AtRuleType, DeclarationListParser, DeclarationParser, Delimiter, ParseError,
    Parser, ParserInput, QualifiedRuleParser, RuleListParser, SourceLocation, ToCss,
};
use grass::{Fs, OutputStyle, StdFs};
use selectors::{
    parser::{Component, NonTSPseudoClass, PseudoElement, SelectorParseErrorKind},
    SelectorImpl, SelectorList,
};

struct ParseRules;

struct Classes(Vec<String>);

impl<'i> QualifiedRuleParser<'i> for ParseRules {
    type Error = SelectorParseErrorKind<'i>;
    type Prelude = Classes;
    type QualifiedRule = Classes;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        parse_prelude(input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>> {
        skip_block(input);

        Ok(prelude)
    }
}

fn parse_prelude<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Classes, ParseError<'i, SelectorParseErrorKind<'i>>> {
    let selectors = SelectorList::parse(&ParseSelectors, input)?;
    let mut classes = Vec::new();
    for selector in selectors.0 {
        for component in selector.iter_raw_match_order() {
            if let Component::Class(class) = component {
                classes.push(class.0.clone());
            }
        }
    }
    Ok(Classes(classes))
}

fn skip_block(input: &mut Parser) {
    let parser = DeclarationListParser::new(input, ParseDeclarations);
    for _ in parser {}
}

type AtRuleClasses = AtRuleType<Classes, Classes>;

impl<'i> AtRuleParser<'i> for ParseRules {
    type AtRule = Classes;
    type Error = SelectorParseErrorKind<'i>;
    type PreludeBlock = Classes;
    type PreludeNoBlock = Classes;

    fn parse_prelude<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleClasses, ParseError<'i, Self::Error>> {
        let prelude = parse_prelude(input)?;

        Ok(match name.as_ref() {
            "charset" | "import" | "namespace" => AtRuleType::WithoutBlock(prelude),
            _ => AtRuleType::WithBlock(prelude),
        })
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::PreludeBlock,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, cssparser::ParseError<'i, Self::Error>> {
        skip_block(input);
        Ok(prelude)
    }
}

struct ParseDeclarations;

impl<'i> AtRuleParser<'i> for ParseDeclarations {
    type AtRule = ();
    type Error = SelectorParseErrorKind<'i>;
    type PreludeBlock = ();
    type PreludeNoBlock = ();
}

impl<'i> DeclarationParser<'i> for ParseDeclarations {
    type Declaration = ();
    type Error = SelectorParseErrorKind<'i>;

    fn parse_value<'t>(
        &mut self,
        _name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        input.parse_until_after(Delimiter::Semicolon, |input| {
            while !input.is_exhausted() {
                input.next()?;
            }

            Ok(())
        })
    }
}

struct ParseSelectors;

impl<'i> selectors::Parser<'i> for ParseSelectors {
    type Error = SelectorParseErrorKind<'i>;
    type Impl = SelectorTypes;

    fn parse_slotted(&self) -> bool {
        true
    }

    fn parse_part(&self) -> bool {
        true
    }

    fn parse_is_and_where(&self) -> bool {
        true
    }

    fn parse_host(&self) -> bool {
        true
    }

    fn parse_non_ts_pseudo_class(
        &self,
        _location: cssparser::SourceLocation,
        _name: cssparser::CowRcStr<'i>,
    ) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<'i, Self::Error>> {
        Ok(Empty)
    }

    fn parse_non_ts_functional_pseudo_class<'t>(
        &self,
        _name: cssparser::CowRcStr<'i>,
        _arguments: &mut Parser<'i, 't>,
    ) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<'i, Self::Error>> {
        Ok(Empty)
    }

    fn parse_pseudo_element(
        &self,
        _location: cssparser::SourceLocation,
        _name: cssparser::CowRcStr<'i>,
    ) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<'i, Self::Error>> {
        Ok(Empty)
    }

    fn parse_functional_pseudo_element<'t>(
        &self,
        _name: cssparser::CowRcStr<'i>,
        _arguments: &mut Parser<'i, 't>,
    ) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<'i, Self::Error>> {
        Ok(Empty)
    }
}

#[derive(Clone, Debug)]
struct SelectorTypes;

impl SelectorImpl for SelectorTypes {
    type AttrValue = Empty;
    type BorrowedLocalName = Empty;
    type BorrowedNamespaceUrl = Empty;
    type ExtraMatchingData = Empty;
    type Identifier = Identifier;
    type LocalName = Empty;
    type NamespacePrefix = Empty;
    type NamespaceUrl = Empty;
    type NonTSPseudoClass = Empty;
    type PseudoElement = Empty;
}

#[derive(Clone, PartialEq, Eq)]
struct Identifier(String);

impl ToCss for Identifier {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl From<&str> for Identifier {
    fn from(ident: &str) -> Self {
        Self(ident.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
struct Empty;

impl NonTSPseudoClass for Empty {
    type Impl = SelectorTypes;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

impl ToCss for Empty {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        Ok(())
    }
}

impl<'a> From<&'a str> for Empty {
    fn from(_: &'a str) -> Self {
        Self
    }
}

impl PseudoElement for Empty {
    type Impl = SelectorTypes;

    fn accepts_state_pseudo_classes(&self) -> bool {
        true
    }

    fn valid_after_slotted(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct DependencyTracker {
    dependencies: RefCell<HashSet<PathBuf>>,
    fs: StdFs,
}

impl DependencyTracker {
    fn new() -> Self {
        Self {
            dependencies: RefCell::new(HashSet::new()),
            fs: StdFs,
        }
    }
}

impl Fs for DependencyTracker {
    fn is_dir(&self, path: &Path) -> bool {
        self.fs.is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.fs.is_file(path)
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.dependencies
            .borrow_mut()
            .insert(fs::canonicalize(path)?);
        self.fs.read(path)
    }
}

pub fn class_names(
    filename: &str,
) -> Result<(impl Iterator<Item = PathBuf>, impl Iterator<Item = String>), Box<dyn Error>> {
    let dependency_tracker = DependencyTracker::new();
    let css = grass::from_path(
        filename,
        &grass::Options::default()
            .style(OutputStyle::Expanded)
            .quiet(true)
            .fs(&dependency_tracker),
    )?;

    let mut parser_input = ParserInput::new(&css);
    let mut input = Parser::new(&mut parser_input);
    let parser = RuleListParser::new_for_stylesheet(&mut input, ParseRules);
    let mut classes = HashSet::new();

    for rule in parser {
        let rule = rule.map_err(|e| {
            let SourceLocation {
                mut line,
                mut column,
            } = e.0.location;
            line += 1;
            column += 1;

            format!("CSS parse error at line {line} column {column}. CSS was: \n\n{css}\n",)
        })?;

        for class in rule.0 {
            classes.insert(class);
        }
    }

    Ok((
        dependency_tracker.dependencies.take().into_iter(),
        classes.into_iter(),
    ))
}
