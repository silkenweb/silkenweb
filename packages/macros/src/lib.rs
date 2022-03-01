use std::{env, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, LitStr, Visibility,
};

mod parser;

struct Input {
    visibility: Option<Visibility>,
    css_file: String,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = if input.peek(token::Pub) {
            Some(input.parse()?)
        } else {
            None
        };

        let css_file = input.parse::<LitStr>()?.value();

        Ok(Self {
            visibility,
            css_file,
        })
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
/// # Example
///
///  ```
/// # use silkenweb_macros::css_classes;
/// css_classes!("my-sass-file.scss");
/// assert_eq!(MY_CLASS, "my-class");
/// ```
/// 
/// An optional visibility modifier can be specified:
/// ```
/// mod css {
///     # use silkenweb_macros::css_classes;
///     css_classes!(pub "my-sass-file.scss");
/// }
///
/// assert_eq!(css::MY_CLASS, "my-class");
/// ```
///
#[proc_macro]
#[proc_macro_error]
pub fn css_classes(input: TokenStream) -> TokenStream {
    let Input {
        visibility,
        css_file,
    } = parse_macro_input!(input);

    let root_dir = env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| abort_call_site!("Unable to read {}", CARGO_MANIFEST_DIR));
    let css_file = PathBuf::from(root_dir)
        .join(css_file)
        .into_os_string()
        .into_string()
        .expect("Expected path to be convertible to string");

    let classes = parser::class_names(&css_file)
        .unwrap_or_else(|e| abort_call_site!("'{}': {}", css_file, e.to_string()));

    let classes = classes.map(|class| {
        if !class.starts_with(char::is_alphabetic) {
            abort_call_site!(
                "Identifier '{}' doesn't start with an alphabetic character",
                class
            );
        }

        let class_ident = Ident::new(
            &class
                .replace(|c: char| !c.is_alphanumeric(), "_")
                .to_uppercase(),
            Span::call_site(),
        );
        quote!(#visibility const #class_ident: &str = #class;)
    });

    quote!(
        const _: &[u8] = ::std::include_bytes!(#css_file);
        #(#classes)*
    )
    .into()
}

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
