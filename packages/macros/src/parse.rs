use std::str::FromStr;

use css::{Browsers, Css, CssSyntax, Version};
use derive_more::Into;
use proc_macro_error::{abort, abort_call_site};
use silkenweb_css as css;
use syn::{
    bracketed, parenthesized,
    parse::{Lookahead1, Parse, ParseBuffer, ParseStream, Peek},
    token::{self, Comma, CustomToken},
    LitInt, LitStr,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(path);
    custom_keyword!(syntax);
    custom_keyword!(content);
    custom_keyword!(public);
    custom_keyword!(prefix);
    custom_keyword!(include_prefixes);
    custom_keyword!(exclude_prefixes);
    custom_keyword!(validate);
    custom_keyword!(auto_mount);
    custom_keyword!(transpile);
    custom_keyword!(minify);
    custom_keyword!(pretty);
    custom_keyword!(modules);
    custom_keyword!(nesting);
    custom_keyword!(browsers);
}

mod functions {
    use syn::custom_keyword;

    custom_keyword!(concat);
}

trait ParseValue: Sized {
    fn parse(input: ParseStream) -> syn::Result<Self>;
}

impl ParseValue for String {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first = input.lookahead1();

        if first.peek(LitStr) {
            Ok(input.parse::<LitStr>()?.value())
        } else if first.peek(functions::concat) {
            input.parse::<functions::concat>()?;
            Ok(parse_comma_delimited(&parenthesized(input)?, String::parse)?.concat())
        } else {
            Err(first.error())
        }
    }
}

impl<T: ParseValue> ParseValue for Vec<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list;
        bracketed!(list in input);
        parse_comma_delimited(&list, T::parse)
    }
}

impl ParseValue for CssSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let syntax_lit = &input.parse::<LitStr>()?;
        Self::from_str(syntax_lit.value().as_str()).map_err(|()| {
            syn::Error::new(
                syntax_lit.span(),
                r#"expected one of  "css", "scss" or "sass" "#,
            )
        })
    }
}

pub struct Input {
    pub source: Css,
    pub public: bool,
    pub prefix: Option<String>,
    pub include_prefixes: Option<Vec<String>>,
    pub exclude_prefixes: Vec<String>,
    pub validate: bool,
    pub auto_mount: bool,
    pub transpile: Option<Transpile>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let path = input.parse::<LitStr>()?.value();
            return Ok(Self {
                source: Css::from_path(path, None).unwrap_or_else(|e| abort_call_site!(e)),
                public: false,
                prefix: None,
                include_prefixes: None,
                exclude_prefixes: Vec::new(),
                validate: false,
                auto_mount: false,
                transpile: None,
            });
        }

        let mut path: Option<String> = None;
        let mut syntax: Option<CssSyntax> = None;
        let mut public = false;
        let mut content: Option<String> = None;
        let mut prefix = None;
        let mut include_prefixes = None;
        let mut exclude_prefixes = None;
        let mut validate = false;
        let mut auto_mount = false;
        let mut transpile = None;

        parse_fields(input, |field, input| {
            Ok(parameter(kw::path, field, input, &mut path)?
                || parameter(kw::syntax, field, input, &mut syntax)?
                || flag(kw::public, field, input, &mut public)?
                || parameter(kw::content, field, input, &mut content)?
                || parameter(kw::prefix, field, input, &mut prefix)?
                || parameter(kw::include_prefixes, field, input, &mut include_prefixes)?
                || parameter(kw::exclude_prefixes, field, input, &mut exclude_prefixes)?
                || flag(kw::validate, field, input, &mut validate)?
                || flag(kw::auto_mount, field, input, &mut auto_mount)?
                || parameter(kw::transpile, field, input, &mut transpile)?)
        })?;

        let source = match (&path, content) {
            (None, None) => abort_call_site!("Must specify either 'path' or `content` parameter"),
            (None, Some(content)) => Css::from_content(content, syntax.unwrap_or_default()),
            (Some(path), None) => Css::from_path(path, syntax),
            (Some(_), Some(_)) => {
                abort_call_site!("Only one of 'path' or `content` can be specified")
            }
        }
        .unwrap_or_else(|e| abort_call_site!(e));

        Ok(Self {
            source,
            public,
            prefix,
            include_prefixes,
            exclude_prefixes: exclude_prefixes.unwrap_or_default(),
            validate,
            auto_mount,
            transpile,
        })
    }
}

#[derive(Into)]
pub struct Transpile(css::Transpile);

impl ParseValue for Transpile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut minify = false;
        let mut pretty = false;
        let mut modules = false;
        let mut nesting = false;
        let mut browsers = None;

        parse_fields(&parenthesized(input)?, |field, input| {
            Ok(flag(kw::minify, field, input, &mut minify)?
                || flag(kw::pretty, field, input, &mut pretty)?
                || flag(kw::modules, field, input, &mut modules)?
                || flag(kw::nesting, field, input, &mut nesting)?
                || parameter(kw::browsers, field, input, &mut browsers)?)
        })?;

        Ok(Self(css::Transpile {
            minify,
            pretty,
            modules,
            nesting,
            browsers: browsers.map(Browsers::into),
        }))
    }
}

mod browsers {
    use syn::custom_keyword;

    custom_keyword!(android);
    custom_keyword!(chrome);
    custom_keyword!(edge);
    custom_keyword!(firefox);
    custom_keyword!(ie);
    custom_keyword!(ios_saf);
    custom_keyword!(opera);
    custom_keyword!(safari);
    custom_keyword!(samsung);
}

impl ParseValue for Browsers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut browsers = Browsers::default();

        parse_fields(&parenthesized(input)?, |field, input| {
            Ok(
                parameter(browsers::android, field, input, &mut browsers.android)?
                    || parameter(browsers::chrome, field, input, &mut browsers.chrome)?
                    || parameter(browsers::edge, field, input, &mut browsers.edge)?
                    || parameter(browsers::firefox, field, input, &mut browsers.firefox)?
                    || parameter(browsers::ie, field, input, &mut browsers.ie)?
                    || parameter(browsers::ios_saf, field, input, &mut browsers.ios_saf)?
                    || parameter(browsers::opera, field, input, &mut browsers.opera)?
                    || parameter(browsers::safari, field, input, &mut browsers.safari)?
                    || parameter(browsers::samsung, field, input, &mut browsers.samsung)?,
            )
        })?;

        Ok(browsers)
    }
}

impl ParseValue for Version {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = parenthesized(input)?;
        let major = version_component(&input)?;
        input.parse::<Comma>()?;
        let minor = version_component(&input)?;
        input.parse::<Comma>()?;
        let patch = version_component(&input)?;

        Ok(Self::new(major, minor, patch))
    }
}

fn version_component(input: &syn::parse::ParseBuffer) -> Result<u8, syn::Error> {
    input.parse::<LitInt>()?.base10_parse()
}

fn parameter<Keyword, KeywordToken, T, V>(
    keyword: Keyword,
    field: &Lookahead1,
    input: ParseStream,
    value: &mut Option<V>,
) -> syn::Result<bool>
where
    Keyword: Peek + FnOnce(T) -> KeywordToken,
    KeywordToken: Parse + CustomToken,
    V: ParseValue,
{
    let mut exists = value.is_some();

    Ok(if flag(keyword, field, input, &mut exists)? {
        input.parse::<token::Eq>()?;
        value.replace(V::parse(input)?);
        true
    } else {
        false
    })
}

fn flag<Keyword, KeywordToken, T>(
    keyword: Keyword,
    field: &Lookahead1,
    input: ParseStream,
    value: &mut bool,
) -> syn::Result<bool>
where
    Keyword: Peek + FnOnce(T) -> KeywordToken,
    KeywordToken: Parse + CustomToken,
{
    Ok(if field.peek(keyword) {
        if *value {
            abort!(
                input.span(),
                "{} is defined multiple times",
                KeywordToken::display()
            );
        }

        *value = true;
        input.parse::<KeywordToken>()?;

        true
    } else {
        false
    })
}

fn parse_comma_delimited<T>(
    input: ParseStream,
    mut parser: impl FnMut(ParseStream) -> syn::Result<T>,
) -> syn::Result<Vec<T>> {
    let mut trailing_comma = true;
    let mut result = Vec::new();

    while !input.is_empty() {
        if !trailing_comma {
            abort!(input.span(), "Expected ','");
        }

        result.push(parser(input)?);

        trailing_comma = input.peek(Comma);

        if trailing_comma {
            input.parse::<Comma>()?;
        }
    }

    Ok(result)
}

fn parse_fields(
    input: ParseStream,
    mut parser: impl FnMut(&Lookahead1, ParseStream) -> syn::Result<bool>,
) -> syn::Result<()> {
    parse_comma_delimited(input, |input| {
        let field = input.lookahead1();
        let matched = parser(&field, input)?;

        if matched {
            Ok(())
        } else {
            Err(field.error())
        }
    })?;

    Ok(())
}

fn parenthesized(input: ParseStream) -> syn::Result<ParseBuffer> {
    let body;
    parenthesized!(body in input);
    Ok(body)
}
