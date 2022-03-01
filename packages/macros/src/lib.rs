use std::{env, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{
    bracketed,
    parse::{Lookahead1, Parse, ParseStream, Peek},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Colon, Comma, CustomToken},
    Ident, LitStr, Visibility,
};

mod parser;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(path);
    custom_keyword!(visibility);
    custom_keyword!(prefix);
    custom_keyword!(exclude_prefixes);
}

struct Input {
    path: String,
    visibility: Option<Visibility>,
    prefix: Option<String>,
    exclude_prefixes: Vec<String>,
}

impl Input {
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
        Ok(if lookahead.peek(keyword) {
            if exists {
                abort!(
                    input.span(),
                    "{} is defined multiple times",
                    KeywordToken::display()
                );
            }

            input.parse::<KeywordToken>()?;
            input.parse::<Colon>()?;

            true
        } else {
            false
        })
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self {
                path: input.parse::<LitStr>()?.value(),
                visibility: None,
                prefix: None,
                exclude_prefixes: Vec::new(),
            });
        }

        let mut path = None;
        let mut visibility = None;
        let mut prefix = None;
        let mut exclude_prefixes = Vec::new();
        let mut trailing_comma = true;

        while !input.is_empty() {
            if !trailing_comma {
                abort!(input.span(), "Expected ','");
            }

            let lookahead = input.lookahead1();

            if Self::parameter(kw::path, &lookahead, input, path.is_some())? {
                path = Some(input.parse::<LitStr>()?.value());
            } else if Self::parameter(kw::visibility, &lookahead, input, visibility.is_some())? {
                visibility = Some(input.parse()?);
            } else if Self::parameter(kw::prefix, &lookahead, input, prefix.is_some())? {
                prefix = Some(input.parse::<LitStr>()?.value());
            } else if Self::parameter(
                kw::exclude_prefixes,
                &lookahead,
                input,
                !exclude_prefixes.is_empty(),
            )? {
                let list;

                bracketed!(list in input);
                exclude_prefixes = Punctuated::<LitStr, Comma>::parse_terminated(&list)?
                    .iter()
                    .map(|prefix| prefix.value())
                    .collect();
            } else {
                return Err(lookahead.error());
            }

            trailing_comma = input.peek(Comma);

            if trailing_comma {
                input.parse::<Comma>()?;
            }
        }

        if let Some(path) = path {
            Ok(Self {
                visibility,
                path,
                prefix,
                exclude_prefixes,
            })
        } else {
            abort_call_site!("Missing 'path' parameter");
        }
    }
}

/// Define `&str` constants for each class in a SASS file.
///
/// For a CSS class called `my-css-class`, a constant called `MY_CSS_CLASS` will
/// be defined.
///
/// The input file is relative to the `$CARGO_MANIFEST_DIR` environment
/// variable.
///
/// # Examples
///
/// Define private constants for all CSS classes:
///
///  ```
/// # use silkenweb_macros::css_classes;
/// css_classes!("my-sass-file.scss");
/// assert_eq!(MY_CLASS, "my-class");
/// ```
/// 
/// Optional `visibility`, `prefix`, and `exclude-prefixes` parameters can be specified.
/// - `visibility` is any visibility modifier, and controls the visibility of class constants.
/// - `prefix` specifies that only classes starting with `prefix` should have constants defined.
///   Their Rust names will have the prefix stripped.
/// - `exclude-prefixes` specifies a list of prefixes to exclude completely. No constants will be
///   defined for a class starting with any of these prefixes.
/// ```
/// mod border {
///     # use silkenweb_macros::css_classes;
///     css_classes!(
///         visibility: pub,
///         path: "my-sass-file.scss",
///         prefix:"border-",
///         exclude_prefixes: ["border-excluded-"]
///     );
/// }
///
/// assert_eq!(border::SMALL, "border-small");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn css_classes(input: TokenStream) -> TokenStream {
    let Input {
        visibility,
        path,
        prefix,
        exclude_prefixes,
    } = parse_macro_input!(input);

    let root_dir = env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| abort_call_site!("Unable to read {}", CARGO_MANIFEST_DIR));
    let path = PathBuf::from(root_dir)
        .join(path)
        .into_os_string()
        .into_string()
        .expect("Expected path to be convertible to string");

    let classes = parser::class_names(&path)
        .unwrap_or_else(|e| abort_call_site!("'{}': {}", path, e.to_string()))
        .filter(|class| {
            for excluded_prefix in &exclude_prefixes {
                if class.starts_with(excluded_prefix) {
                    return false;
                }
            }

            true
        });

    if let Some(prefix) = prefix {
        code_gen(
            visibility,
            &path,
            classes.filter_map(|class| {
                let class_ident = class.strip_prefix(&prefix).map(str::to_string);
                class_ident.map(|class_ident| {
                    println!("{}, {}", class_ident, class);
                    (class_ident, class)
                })
            }),
        )
    } else {
        code_gen(
            visibility,
            &path,
            classes.map(|class| (class.clone(), class)),
        )
    }
}

fn code_gen(
    visibility: Option<Visibility>,
    path: &str,
    classes: impl Iterator<Item = (String, String)>,
) -> TokenStream {
    let classes = classes.map(|(class_ident, class_name)| {
        if !class_ident.starts_with(char::is_alphabetic) {
            abort_call_site!(
                "Identifier '{}' doesn't start with an alphabetic character",
                class_ident
            );
        }

        let class_ident = Ident::new(
            &class_ident
                .replace(|c: char| !c.is_alphanumeric(), "_")
                .to_uppercase(),
            Span::call_site(),
        );
        quote!(#visibility const #class_ident: &str = #class_name;)
    });

    quote!(
        const _: &[u8] = ::std::include_bytes!(#path);
        #(#classes)*
    )
    .into()
}

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
