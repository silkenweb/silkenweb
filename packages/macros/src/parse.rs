use derive_more::Into;
use proc_macro_error::{abort, abort_call_site};
use silkenweb_base::css::{self, Source};
use syn::{
    bracketed, parenthesized,
    parse::{Lookahead1, Parse, ParseBuffer, ParseStream, Peek},
    punctuated::Punctuated,
    token::{self, Comma, CustomToken},
    LitInt, LitStr,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(path);
    custom_keyword!(content);
    custom_keyword!(prefix);
    custom_keyword!(include_prefixes);
    custom_keyword!(exclude_prefixes);
    custom_keyword!(validate);
    custom_keyword!(auto_mount);
    custom_keyword!(transpile);
    custom_keyword!(minify);
    custom_keyword!(pretty);
    custom_keyword!(nesting);
    custom_keyword!(browsers);
}

trait ParseValue: Sized {
    fn parse(input: ParseStream) -> syn::Result<Self>;
}

impl ParseValue for String {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(input.parse::<LitStr>()?.value())
    }
}

impl ParseValue for Vec<String> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list;
        bracketed!(list in input);
        Ok(Punctuated::<LitStr, Comma>::parse_terminated(&list)?
            .into_iter()
            .map(|x| x.value())
            .collect())
    }
}

pub struct Input {
    pub source: Source,
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
            return Ok(Self {
                source: Source::from_path(input.parse::<LitStr>()?.value())
                    .unwrap_or_else(|e| abort_call_site!(e)),
                prefix: None,
                include_prefixes: None,
                exclude_prefixes: Vec::new(),
                validate: false,
                auto_mount: false,
                transpile: None,
            });
        }

        let mut path: Option<String> = None;
        let mut content: Option<String> = None;
        let mut prefix = None;
        let mut include_prefixes = None;
        let mut exclude_prefixes = None;
        let mut validate = false;
        let mut auto_mount = false;
        let mut transpile = None;

        parse_comma_delimited(input, |field, input| {
            Ok(parameter(kw::path, field, input, &mut path)?
                || parameter(kw::content, field, input, &mut content)?
                || parameter(kw::prefix, field, input, &mut prefix)?
                || parameter(kw::include_prefixes, field, input, &mut include_prefixes)?
                || parameter(kw::exclude_prefixes, field, input, &mut exclude_prefixes)?
                || flag(kw::validate, field, input, &mut validate)?
                || flag(kw::auto_mount, field, input, &mut auto_mount)?
                || parameter(kw::transpile, field, input, &mut transpile)?)
        })?;

        let source = match (path, content) {
            (None, None) => abort_call_site!("Must specify either 'path' or `content` parameter"),
            (None, Some(content)) => Source::from_content(content),
            (Some(path), None) => Source::from_path(path).unwrap_or_else(|e| abort_call_site!(e)),
            (Some(_), Some(_)) => {
                abort_call_site!("Only one of 'path' or `content` can be specified")
            }
        };

        Ok(Self {
            source,
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
        let mut nesting = false;
        let mut browsers = None;

        parse_comma_delimited(&parenthesized(input)?, |field, input| {
            Ok(flag(kw::minify, field, input, &mut minify)?
                || flag(kw::pretty, field, input, &mut pretty)?
                || flag(kw::nesting, field, input, &mut nesting)?
                || parameter(kw::browsers, field, input, &mut browsers)?)
        })?;

        Ok(Self(css::Transpile {
            minify,
            pretty,
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

#[derive(Into)]
pub struct Browsers(lightningcss::targets::Browsers);

impl Browsers {
    fn browser<Keyword, KeywordToken, T>(
        keyword: Keyword,
        field: &Lookahead1,
        input: ParseStream,
        version: &mut Option<u32>,
    ) -> syn::Result<bool>
    where
        Keyword: Peek + FnOnce(T) -> KeywordToken,
        KeywordToken: Parse + CustomToken,
    {
        let mut exists = version.is_some();

        Ok(if flag(keyword, field, input, &mut exists)? {
            input.parse::<token::Eq>()?;
            version.replace(Version::parse(input)?.encode_for_lightning());
            true
        } else {
            false
        })
    }
}

impl ParseValue for Browsers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut browsers = lightningcss::targets::Browsers::default();

        parse_comma_delimited(&parenthesized(input)?, |field, input| {
            Ok(
                Self::browser(browsers::android, field, input, &mut browsers.android)?
                    || Self::browser(browsers::chrome, field, input, &mut browsers.chrome)?
                    || Self::browser(browsers::edge, field, input, &mut browsers.edge)?
                    || Self::browser(browsers::firefox, field, input, &mut browsers.firefox)?
                    || Self::browser(browsers::ie, field, input, &mut browsers.ie)?
                    || Self::browser(browsers::ios_saf, field, input, &mut browsers.ios_saf)?
                    || Self::browser(browsers::opera, field, input, &mut browsers.opera)?
                    || Self::browser(browsers::safari, field, input, &mut browsers.safari)?
                    || Self::browser(browsers::samsung, field, input, &mut browsers.samsung)?,
            )
        })?;

        Ok(Self(browsers))
    }
}

struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    fn encode_for_lightning(&self) -> u32 {
        u32::from_be_bytes([0, self.major, self.minor, self.patch])
    }

    fn component(input: &syn::parse::ParseBuffer) -> Result<u8, syn::Error> {
        input.parse::<LitInt>()?.base10_parse()
    }
}

impl ParseValue for Version {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = parenthesized(input)?;
        let major = Self::component(&input)?;
        input.parse::<Comma>()?;
        let minor = Self::component(&input)?;
        input.parse::<Comma>()?;
        let patch = Self::component(&input)?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
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

fn parse_comma_delimited(
    input: ParseStream,
    mut parser: impl FnMut(&Lookahead1, &ParseStream) -> syn::Result<bool>,
) -> syn::Result<()> {
    let mut trailing_comma = true;

    while !input.is_empty() {
        let field = input.lookahead1();

        if !trailing_comma {
            abort!(input.span(), "Expected ','");
        }

        let matched = parser(&field, &input)?;

        if !matched {
            return Err(field.error());
        }

        trailing_comma = input.peek(Comma);

        if trailing_comma {
            input.parse::<Comma>()?;
        }
    }

    Ok(())
}

fn parenthesized(input: ParseStream) -> syn::Result<ParseBuffer> {
    let body;
    parenthesized!(body in input);
    Ok(body)
}
