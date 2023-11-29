//! Include inline HTML snippets
//!
//! See [`silkenweb_parse`] for details on the parsing.
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_parse::html_to_tokens;
use syn::{parse_macro_input, LitStr};

/// Include an HTML snippet from a string literal.
///
/// Take a string literal containing a single HTML element, and produce a
/// [`Node`][silkenweb::node::Node] expression. The [`Dom`][silkenweb::dom::Dom]
/// type is not specified, so if it can't be determined by type inference, you
/// may need to provide a type annotation.
///
/// See [`silkenweb_parse`] for details on the parsing.
///
/// # Example
///
/// ```
/// # use silkenweb_inline_html::inline_html;
/// # use silkenweb::node::Node;
/// let node: Node = inline_html!("<p>Inline HTML</p>");
/// assert_eq!(node.to_string(), "<p>Inline HTML</p>");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn inline_html(input: TokenStream) -> TokenStream {
    let html: LitStr = parse_macro_input!(input);
    let html_text = html.value();
    let mut element_iter = html_to_tokens(quote! {D}.into(), &html_text).into_iter();
    let element: proc_macro2::TokenStream = element_iter
        .next()
        .unwrap_or_else(|| abort_call_site!("Unable to parse any elements"))
        .into();

    if element_iter.next().is_some() {
        abort_call_site!("Multiple elements found");
    }

    quote! {{
        pub fn node<D: ::silkenweb::dom::Dom>() -> ::silkenweb::node::Node<D> {
            #element
        }

        node()
    }}
    .into()
}

/// Include an HTML snippet from a file.
///
/// This takes a string literal as a filename, parses the contents of the file
/// and puts the resulting expression into a funcion. The function name is
/// derived from the filename by replacing non alphanumeric characters with an
/// `_`.
///
/// See `examples/inline-html` for a usage example.
///
/// See [`silkenweb_parse`] for details on the parsing.
#[proc_macro]
#[proc_macro_error]
pub fn html_file(input: TokenStream) -> TokenStream {
    let file: LitStr = parse_macro_input!(input);
    let file_path = root_dir().join(file.value());
    html_from_path(&file_path).into()
}

/// Include HTML snippets from a directory of files.
///
/// This takes a string literal as a directory name and is equivalent to running
/// [`html_file!`] on every file directly contained in directory.
///
/// See `examples/inline-html` for a usage example.
///
/// See [`silkenweb_parse`] for details on the parsing.
#[proc_macro]
#[proc_macro_error]
pub fn html_dir(input: TokenStream) -> TokenStream {
    let dir_literal: LitStr = parse_macro_input!(input);
    let dir = dir_literal.value();
    let fns = fs::read_dir(root_dir().join(&dir))
        .unwrap_or_else(|_| abort_call_site!("Unable to read dir '{}'", dir))
        .filter_map(|entry| {
            let path = entry
                .unwrap_or_else(|_| abort_call_site!("Unable to read dir entry"))
                .path();

            if path.is_file() {
                Some(html_from_path(&path))
            } else {
                None
            }
        });

    quote!(#(#fns)*).into()
}

fn html_from_path(file_path: &Path) -> proc_macro2::TokenStream {
    let html_text = fs::read_to_string(file_path)
        .unwrap_or_else(|_| abort_call_site!("Unable to read file '{:?}'", &file_path));
    let mut element_iter = html_to_tokens(quote! {D}.into(), &html_text).into_iter();
    let element: proc_macro2::TokenStream = element_iter
        .next()
        .unwrap_or_else(|| abort_call_site!("Unable to parse any elements for '{:?}'", &file_path))
        .into();

    if element_iter.next().is_some() {
        abort_call_site!("Multiple elements found in '{:?}'", &file_path);
    }

    let fn_name = filename_to_ident(
        file_path
            .file_stem()
            .unwrap_or_else(|| {
                abort_call_site!("Unable to extract file stem from '{:?}'", file_path)
            })
            .to_str()
            .unwrap(),
    );

    quote! {
        pub fn #fn_name<D: ::silkenweb::dom::Dom>() -> ::silkenweb::node::Node<D> {
            #element
        }
    }
}

fn root_dir() -> PathBuf {
    const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

    PathBuf::from(
        env::var(CARGO_MANIFEST_DIR)
            .unwrap_or_else(|_| abort_call_site!("Couldn't read '{CARGO_MANIFEST_DIR}' variable")),
    )
}

fn filename_to_ident(file: &str) -> Ident {
    let ident = file.replace(|c: char| !c.is_alphanumeric(), "_");

    if let Some(first) = ident.chars().next() {
        if !first.is_alphabetic() && first != '_' {
            abort_call_site!("Illegal first char in '{}'", ident);
        }
    } else {
        abort_call_site!("Empty identifier");
    }

    Ident::new(&ident, Span::call_site())
}
