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
    custom_keyword!(transpile);
    custom_keyword!(minify);
    custom_keyword!(pretty);
    custom_keyword!(nesting);
    custom_keyword!(browsers);
}

pub struct Input {
    pub source: Source,
    pub prefix: Option<String>,
    pub include_prefixes: Option<Vec<String>>,
    pub exclude_prefixes: Vec<String>,
    pub validate: bool,
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
                transpile: None,
            });
        }

        let mut path = None;
        let mut content = None;
        let mut prefix = None;
        let mut include_prefixes = None;
        let mut exclude_prefixes = Vec::new();
        let mut validate = false;
        let mut transpile = None;

        parse_comma_delimited(input, |lookahead, input| {
            if parameter(kw::path, lookahead, input, path.is_some())? {
                path = Some(input.parse::<LitStr>()?.value());
            } else if parameter(kw::content, lookahead, input, content.is_some())? {
                content = Some(input.parse::<LitStr>()?.value());
            } else if parameter(kw::prefix, lookahead, input, prefix.is_some())? {
                prefix = Some(input.parse::<LitStr>()?.value());
            } else if parameter(
                kw::include_prefixes,
                lookahead,
                input,
                include_prefixes.is_some(),
            )? {
                include_prefixes = Some(parse_prefix_list(input)?);
            } else if parameter(
                kw::exclude_prefixes,
                lookahead,
                input,
                !exclude_prefixes.is_empty(),
            )? {
                exclude_prefixes = parse_prefix_list(input)?;
            } else if flag(kw::validate, lookahead, input, validate)? {
                validate = true;
            } else if parameter(kw::transpile, lookahead, input, transpile.is_some())? {
                transpile = Some(input.parse()?);
            } else {
                return Ok(false);
            }

            Ok(true)
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
            exclude_prefixes,
            validate,
            transpile,
        })
    }
}

#[derive(Into)]
pub struct Transpile(css::Transpile);

impl Parse for Transpile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut minify = false;
        let mut pretty = false;
        let mut nesting = false;
        let mut browsers = None;

        parse_comma_delimited(&parenthesized(input)?, |lookahead, input| {
            if flag(kw::minify, lookahead, input, minify)? {
                minify = true;
            } else if flag(kw::pretty, lookahead, input, pretty)? {
                pretty = true;
            } else if flag(kw::nesting, lookahead, input, nesting)? {
                nesting = true;
            } else if parameter(kw::browsers, lookahead, input, browsers.is_some())? {
                browsers = Some(input.parse::<Browsers>()?);
            } else {
                return Ok(false);
            }

            Ok(true)
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
        lookahead: &Lookahead1,
        input: ParseStream,
        version: &mut Option<u32>,
    ) -> syn::Result<bool>
    where
        Keyword: Peek + FnOnce(T) -> KeywordToken,
        KeywordToken: Parse + CustomToken,
    {
        Ok(
            if parameter(keyword, lookahead, input, version.is_some())? {
                *version = Some(input.parse::<Version>()?.encode_for_lightning());

                true
            } else {
                false
            },
        )
    }
}

impl Parse for Browsers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut browsers = lightningcss::targets::Browsers::default();

        parse_comma_delimited(&parenthesized(input)?, |lookahead, input| {
            Ok(
                Self::browser(browsers::android, lookahead, input, &mut browsers.android)?
                    || Self::browser(browsers::chrome, lookahead, input, &mut browsers.chrome)?
                    || Self::browser(browsers::edge, lookahead, input, &mut browsers.edge)?
                    || Self::browser(browsers::firefox, lookahead, input, &mut browsers.firefox)?
                    || Self::browser(browsers::ie, lookahead, input, &mut browsers.ie)?
                    || Self::browser(browsers::ios_saf, lookahead, input, &mut browsers.ios_saf)?
                    || Self::browser(browsers::opera, lookahead, input, &mut browsers.opera)?
                    || Self::browser(browsers::safari, lookahead, input, &mut browsers.safari)?
                    || Self::browser(browsers::samsung, lookahead, input, &mut browsers.samsung)?,
            )
        })?;

        Ok(Self(browsers))
    }
}

fn parse_comma_delimited(
    input: ParseStream,
    mut parser: impl FnMut(&Lookahead1, &ParseStream) -> syn::Result<bool>,
) -> syn::Result<()> {
    let mut trailing_comma = true;

    while !input.is_empty() {
        let lookahead = input.lookahead1();

        if !trailing_comma {
            abort!(input.span(), "Expected ','");
        }

        let matched = parser(&lookahead, &input)?;

        if !matched {
            return Err(lookahead.error());
        }

        trailing_comma = input.peek(Comma);

        if trailing_comma {
            input.parse::<Comma>()?;
        }
    }

    Ok(())
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

impl Parse for Version {
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

fn parameter<Keyword, KeywordToken, T>(
    keyword: Keyword,
    lookahead: &Lookahead1,
    input: ParseStream,
    exists: bool,
) -> syn::Result<bool>
where
    Keyword: Peek + FnOnce(T) -> KeywordToken,
    KeywordToken: Parse + CustomToken,
{
    Ok(if flag(keyword, lookahead, input, exists)? {
        input.parse::<token::Eq>()?;
        true
    } else {
        false
    })
}

fn flag<Keyword, KeywordToken, T>(
    keyword: Keyword,
    lookahead: &Lookahead1,
    input: ParseStream,
    exists: bool,
) -> syn::Result<bool>
where
    Keyword: Peek + FnOnce(T) -> KeywordToken,
    KeywordToken: Parse + CustomToken,
{
    Ok(if lookahead.peek(keyword) {
        if exists {
            abort!(
                input.span(),
                "{} is defined multiple times",
                KeywordToken::display()
            );
        }

        input.parse::<KeywordToken>()?;

        true
    } else {
        false
    })
}

fn parenthesized(input: ParseStream) -> syn::Result<ParseBuffer> {
    let body;
    parenthesized!(body in input);
    Ok(body)
}

fn parse_prefix_list(input: &syn::parse::ParseBuffer) -> Result<Vec<String>, syn::Error> {
    let list;
    bracketed!(list in input);
    Ok(Punctuated::<LitStr, Comma>::parse_terminated(&list)?
        .iter()
        .map(|prefix| prefix.value())
        .collect())
}
