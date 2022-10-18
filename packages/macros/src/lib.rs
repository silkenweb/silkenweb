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
    Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Index, LitStr, Meta, NestedMeta, Path, Visibility,
};

mod parser;

// TODO: Docs
#[proc_macro_derive(ElementBuilder, attributes(element_target, element_dom_type))]
#[proc_macro_error]
pub fn derive_element_builder(item: TokenStream) -> TokenStream {
    let new_type: DeriveInput = parse_macro_input!(item);
    let target = extract_attr_type(&new_type.attrs, "element_target", || {
        abort_call_site!("Use `element_target(TargetType)`")
    });
    let dom_type = extract_attr_type(&new_type.attrs, "element_dom_type", || {
        abort_call_site!("Use `element_dom_type(DomType)`")
    });
    let (impl_generics, ty_generics, where_clause) = new_type.generics.split_for_impl();
    let name = new_type.ident;

    let fields = match new_type.data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => abort_call_site!("Only structs are supported"),
    };

    let mut fields = match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
        Fields::Named(FieldsNamed { named, .. }) => named,
        _ => abort!(fields, "There must be at least one field"),
    }
    .into_iter();

    let Field {
        ty: derive_from,
        ident: derive_ident,
        ..
    } = fields
        .next()
        .unwrap_or_else(|| abort_call_site!("There must be at least one field"));

    let field_tail_names = (1..).zip(fields).map(|(index, field)| {
        if let Some(ident) = field.ident {
            quote!(#ident)
        } else {
            let index = Index::from(index);
            quote!(#index)
        }
    });
    let fields_tail = quote!(#(, #field_tail_names: self.#field_tail_names)*);

    let derive_field = if let Some(derive_ident) = derive_ident {
        quote!(#derive_ident)
    } else {
        quote!(0)
    };

    let dom_type = match dom_type {
        None => quote!(<#derive_from as ::silkenweb::node::element::ElementBuilder>::DomType),
        Some(dom_type) => quote!(#dom_type),
    };
    let (target, build_fn_body) = match target {
        None => (
            quote!(<#derive_from as ::silkenweb::node::element::ElementBuilder>::Target),
            quote!(self.#derive_field.build()),
        ),
        Some(target) => (
            quote!(#target),
            quote!(#target(self.#derive_field.build().into())),
        ),
    };

    quote!(
        impl #impl_generics ::silkenweb::node::element::ElementBuilder
        for #name #ty_generics #where_clause {
            type DomType = #dom_type;
            type Target = #target;

            fn class<'a, T>(self, class: impl ::silkenweb::node::element::RefSignalOrValue<'a, Item = T>) -> Self
            where
                T: 'a + AsRef<str>
            {
                Self {#derive_field: self.#derive_field.class(class) #fields_tail}
            }

            fn classes<'a, T>(
                self,
                classes: impl ::silkenweb::node::element::RefSignalOrValue<'a, Item = impl IntoIterator<Item = T>>,
            ) -> Self
            where
                T: 'a + AsRef<str>
            {
                    Self {#derive_field: self.#derive_field.classes(classes) #fields_tail}
            }

            fn attribute<'a>(
                mut self,
                name: &str,
                value: impl ::silkenweb::node::element::RefSignalOrValue<'a, Item = impl ::silkenweb::attribute::Attribute>,
            ) -> Self {
                Self{#derive_field: self.#derive_field.attribute(name, value) #fields_tail}
            }

            fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
                Self{#derive_field: self.#derive_field.effect(f) #fields_tail}
            }

            fn effect_signal<T: 'static>(
                self,
                sig: impl ::silkenweb::macros::Signal<Item = T> + 'static,
                f: impl Fn(&Self::DomType, T) + Clone + 'static,
            ) -> Self {
                Self{#derive_field: self.#derive_field.effect_signal(sig, f) #fields_tail}
            }

            fn handle(&self) -> ::silkenweb::node::element::ElementHandle<Self::DomType> {
                self.#derive_field.handle()
            }

            fn spawn_future(self, future: impl ::std::future::Future<Output = ()> + 'static) -> Self {
                Self{#derive_field: self.#derive_field.spawn_future(future) #fields_tail}
            }

            fn on(self, name: &'static str, f: impl FnMut(::silkenweb::macros::JsValue) + 'static) -> Self {
                Self{#derive_field: self.#derive_field.on(name, f) #fields_tail}
            }

            fn build(self) -> Self::Target {
                #build_fn_body
            }
        }
    )
    .into()
}

fn extract_attr_type(attrs: &[Attribute], name: &str, syntax_error: impl Fn()) -> Option<Path> {
    for attr in attrs {
        if attr.path.is_ident(name) {
            if let Meta::List(list) = attr.parse_meta().unwrap() {
                let mut target_list = list.nested.into_iter();

                if let Some(NestedMeta::Meta(Meta::Path(target_path))) = target_list.next() {
                    if target_list.next().is_some() {
                        syntax_error();
                    }

                    return Some(target_path);
                } else {
                    syntax_error();
                }
            } else {
                syntax_error();
            }
        }
    }

    None
}

// TODO: Keep an eye on <https://github.com/kaj/rsass>:
// - It provides a parser, so classes can be extracted
// - The repo seems to be actively developed
// - It looks fairly complete

/// Compile SCSS.
///
/// This takes a single string literal containing SCSS. It compiles it down to
/// CSS, and returns a `&'static str` containing the compiled CSS. All
/// valid CSS is also valid SCSS, so you can use this to check your CSS at
/// compile time as well.
///
/// It uses the [Grass] compiler, so has the same [outstanding issues].
///
/// Any imports are relative to `$CARGO_MANIFEST_DIR`.
///
/// # Example
///
/// ```
/// # use silkenweb_macros::css;
/// let css_text: &str = css!(
///     "
///     .text-color {
///         color: limegreen;
///     }
///     "
/// );
/// ```
///
/// [grass]: https://github.com/connorskees/grass
/// [outstanding issues]: https://github.com/connorskees/grass/issues/19
#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let css_input: LitStr = parse_macro_input!(input);

    let root_dir = cargo_manifest_dir();
    let css_text = grass::from_string(
        css_input.value(),
        &grass::Options::default()
            .quiet(true)
            .load_path(&PathBuf::from(root_dir)),
    )
    .unwrap_or_else(|e| abort_call_site!("Error: {}", e));

    quote!(#css_text).into()
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(path);
    custom_keyword!(visibility);
    custom_keyword!(prefix);
    custom_keyword!(include_prefixes);
    custom_keyword!(exclude_prefixes);
}

/// Define `&str` constants for each class in a SASS file.
///
/// For a CSS class called `my-css-class`, a constant called `MY_CSS_CLASS` will
/// be defined.
///
/// The macro takes two forms. Firstly it can take a single string literal which
/// is the path to the CSS/SCSS/SASS file. The path is relative to the
/// `$CARGO_MANIFEST_DIR` environment variable.
///
/// Alternatively, named parameters can be specified:
/// - `path` (mandatory) is the path to the CSS /SCSS/SASS file.
/// - `visibility` (optional) is any visibility modifier, and controls the
///   visibility of class constants.
/// - `prefix` (optional) specifies that only classes starting with `prefix`
///   should be included. Their Rust names will have the prefix stripped.
/// - `include_prefixes` (optional) specifies a list of prefixes to include,
///   without stripping the prefix. Rust constants will only be defined for
///   classes starting with one or more of these prefixes.
/// - `exclude_prefixes` (optional) specifies a list of prefixes to exclude. No
///   Rust constants will be defined for a class starting with any of these
///   prefixes. `exclude_prefixes` takes precedence over `include_prefixes`.
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
/// Include classes starting with `border-`, except classes starting with `border-excluded-`:
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
/// 
/// This won't compile because `exclude_prefixes` takes precedence over
/// `include_prefixes`:
/// ```compile_fail
///     # use silkenweb_macros::css_classes;
///     css_classes!(
///         path: "my-sass-file.scss",
///         include_prefixes: ["border-"]
///         exclude_prefixes: ["border-excluded-"]
///     );
///
///     assert_eq!(BORDER_EXCLUDED_HUGE, "border-excluded-huge");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn css_classes(input: TokenStream) -> TokenStream {
    let Input {
        visibility,
        path,
        prefix,
        include_prefixes,
        exclude_prefixes,
    } = parse_macro_input!(input);

    let root_dir = cargo_manifest_dir();
    let path = PathBuf::from(root_dir)
        .join(path)
        .into_os_string()
        .into_string()
        .expect("Expected path to be convertible to string");

    let classes = parser::class_names(&path)
        .unwrap_or_else(|e| abort_call_site!("'{}': {}", path, e.to_string()))
        .filter(|class| {
            let include = if let Some(include_prefixes) = include_prefixes.as_ref() {
                any_prefix_matches(class, include_prefixes)
            } else {
                true
            };

            let exclude = any_prefix_matches(class, &exclude_prefixes);

            include && !exclude
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

struct Input {
    path: String,
    visibility: Option<Visibility>,
    prefix: Option<String>,
    include_prefixes: Option<Vec<String>>,
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

    fn parse_prefix_list(input: &syn::parse::ParseBuffer) -> Result<Vec<String>, syn::Error> {
        let list;
        bracketed!(list in input);
        Ok(Punctuated::<LitStr, Comma>::parse_terminated(&list)?
            .iter()
            .map(|prefix| prefix.value())
            .collect())
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self {
                path: input.parse::<LitStr>()?.value(),
                visibility: None,
                prefix: None,
                include_prefixes: None,
                exclude_prefixes: Vec::new(),
            });
        }

        let mut path = None;
        let mut visibility = None;
        let mut prefix = None;
        let mut include_prefixes = None;
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
                kw::include_prefixes,
                &lookahead,
                input,
                include_prefixes.is_some(),
            )? {
                include_prefixes = Some(Self::parse_prefix_list(input)?);
            } else if Self::parameter(
                kw::exclude_prefixes,
                &lookahead,
                input,
                !exclude_prefixes.is_empty(),
            )? {
                exclude_prefixes = Self::parse_prefix_list(input)?;
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
                include_prefixes,
                exclude_prefixes,
            })
        } else {
            abort_call_site!("Missing 'path' parameter");
        }
    }
}

fn any_prefix_matches(x: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| x.starts_with(prefix))
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

fn cargo_manifest_dir() -> String {
    const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

    env::var(CARGO_MANIFEST_DIR).unwrap_or_else(|_| {
        abort_call_site!("Unable to read {} from environment", CARGO_MANIFEST_DIR)
    })
}

/// Convert a rust ident to an html ident by stripping any "r#" prefix and
/// replacing '_' with '-'.
#[doc(hidden)]
#[proc_macro]
#[proc_macro_error]
pub fn rust_to_html_ident(input: TokenStream) -> TokenStream {
    let rust_ident: Ident = parse_macro_input!(input);
    let html_ident = rust_ident.to_string().replace('_', "-");
    let html_ident_name = html_ident.strip_prefix("r#").unwrap_or(&html_ident);

    quote!(#html_ident_name).into()
}
