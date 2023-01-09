use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_base::css::{self, Source};
use syn::{
    braced, bracketed,
    parse::{Lookahead1, Parse, ParseStream, Peek},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Colon, Comma, CustomToken},
    Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Index, LitInt, LitStr, Meta, NestedMeta, Path, Visibility,
};

macro_rules! derive_empty(
    (
        $($proc_name:ident ( $type_path:path, $type_name:ident ); )*
    ) => {$(
        #[doc = concat!("Derive `", stringify!($type_name), "`")]
        #[doc = ""]
        #[doc = "This will derive an instance with an empty body:"]
        #[doc = ""]
        #[doc = concat!("`impl ", stringify!($type_name), " for MyType {}`")]
        #[doc = ""]
        #[doc = "Types with generic parameters are supported."]
        #[proc_macro_derive($type_name)]
        pub fn $proc_name(item: TokenStream) -> TokenStream {
            let new_type: DeriveInput = parse_macro_input!(item);
            let (impl_generics, ty_generics, where_clause) = new_type.generics.split_for_impl();
            let name = new_type.ident;

            quote!(
                impl #impl_generics ::silkenweb::$type_path::$type_name
                for #name #ty_generics #where_clause {}
            ).into()
        }
    )*}
);

derive_empty!(
    derive_value(value, Value);
    derive_html_element(elements, HtmlElement);
    derive_aria_element(elements, AriaElement);
    derive_html_element_events(elements, HtmlElementEvents);
    derive_element_events(elements, ElementEvents);
);

#[proc_macro_derive(Element, attributes(element_target, element_dom_type))]
#[proc_macro_error]
pub fn derive_element(item: TokenStream) -> TokenStream {
    let new_type: DeriveInput = parse_macro_input!(item);
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
        None => quote!(<#derive_from as ::silkenweb::node::element::Element>::DomType),
        Some(dom_type) => quote!(#dom_type),
    };

    quote!(
        impl #impl_generics ::silkenweb::node::element::Element
        for #name #ty_generics #where_clause {
            type DomType = #dom_type;

            fn class<'a, T>(self, class: impl ::silkenweb::value::RefSignalOrValue<'a, Item = T>) -> Self
            where
                T: 'a + AsRef<str>
            {
                Self {#derive_field: self.#derive_field.class(class) #fields_tail}
            }

            fn classes<'a, T, Iter>(
                self,
                classes: impl ::silkenweb::value::RefSignalOrValue<'a, Item = Iter>,
            ) -> Self
            where
                T: 'a + AsRef<str>,
                Iter: 'a + IntoIterator<Item = T>,
            {
                    Self {#derive_field: self.#derive_field.classes(classes) #fields_tail}
            }

            fn attribute<'a>(
                mut self,
                name: &str,
                value: impl ::silkenweb::value::RefSignalOrValue<'a, Item = impl ::silkenweb::attribute::Attribute>,
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

mod kw {
    use syn::custom_keyword;

    custom_keyword!(path);
    custom_keyword!(inline);
    custom_keyword!(visibility);
    custom_keyword!(prefix);
    custom_keyword!(include_prefixes);
    custom_keyword!(exclude_prefixes);
    custom_keyword!(validate);
    custom_keyword!(minify);
    custom_keyword!(nesting);
    custom_keyword!(browsers);
}

/// Define `&str` constants for each class in a SASS file.
///
/// For a CSS class called `my-css-class`, a constant called `MY_CSS_CLASS` will
/// be defined.
///
/// An `fn stylesheet() -> &'static str` will also be defined, which gets the
/// content of the stylesheet.
///
/// The macro takes two forms. Firstly it can take a single string literal which
/// is the path to the CSS/SCSS/SASS file. The path is relative to the
/// `$CARGO_MANIFEST_DIR` environment variable.
///
/// Alternatively, named parameters can be specified.
///
/// # Parameters
///
/// All are optional, but either `path` or `inline` must be specified:
///
/// - `path` is the path to the CSS /SCSS/SASS file.
/// - `inline` is the css content.
/// - `visibility` is any visibility modifier, and controls the visibility of
///   class constants. Private is the default.
/// - `prefix` specifies that only classes starting with `prefix` should be
///   included. Their Rust names will have the prefix stripped.
/// - `include_prefixes` specifies a list of prefixes to include, without
///   stripping the prefix. Rust constants will only be defined for classes
///   starting with one or more of these prefixes.
/// - `exclude_prefixes` specifies a list of prefixes to exclude. No Rust
///   constants will be defined for a class starting with any of these prefixes.
///   `exclude_prefixes` takes precedence over `include_prefixes`.
/// - `browsers` is a comma seperated list of the minimum supported browser
///   versions. This will add vendor prefixes to the CSS from `stylesheet()`.
///   The version is a `:` seperated string of major, minor, and patch versions.
///   For example, to support firefox 110  + and chrome 111+, use `browsers: {
///   firefox: 110:0:0, chrome: 111:0:0 }`. Supported browsers:
///     - android
///     - chrome
///     - edge
///     - firefox
///     - ie
///     - ios_saf
///     - opera
///     - safari
///     - samsung
///
/// ## Flags
///
/// - `validate`: Validate the CSS.
/// - `minify`: Minify the CSS returned by `stylesheet()`.
/// - `nesting`: Allow CSS nesting.
///
/// # Examples
///
/// Define private constants for all CSS classes:
///
///  ```
/// # use silkenweb_macros::css;
/// css!("my-css-file.css");
/// assert_eq!(MY_CLASS, "my-class");
/// ```
/// 
/// Define private constants for all inline CSS classes:
///
///  ```
/// # use silkenweb_macros::css;
/// css!(inline: r#"
///     .my-class {
///         color: hotpink;
///     }
/// "#);
/// assert_eq!(MY_CLASS, "my-class");
/// assert_eq!(stylesheet(), r#"
///     .my-class {
///         color: hotpink;
///     }
/// "#);
/// ```
///
/// Include classes starting with `border-`, except classes starting with
/// `border-excluded-`:
///
/// ```
/// mod border {
///     # use silkenweb_macros::css;
///     css!(
///         visibility: pub,
///         path: "my-css-file.css",
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
///     # use silkenweb_macros::css;
///     css!(
///         path: "my-css-file.css",
///         include_prefixes: ["border-"]
///         exclude_prefixes: ["border-excluded-"]
///     );
///
///     assert_eq!(BORDER_EXCLUDED_HUGE, "border-excluded-huge");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let Input {
        visibility,
        mut source,
        prefix,
        include_prefixes,
        exclude_prefixes,
        minify,
        validate,
        nesting,
        browsers,
    } = parse_macro_input!(input);

    source
        .transpile(validate, minify, nesting, browsers.map(|b| b.0))
        .unwrap_or_else(|e| abort_call_site!(e));

    let classes = css::class_names(&source).filter(|class| {
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
            &source,
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
            &source,
            classes.map(|class| (class.clone(), class)),
        )
    }
}

struct Input {
    source: Source,
    visibility: Option<Visibility>,
    prefix: Option<String>,
    include_prefixes: Option<Vec<String>>,
    exclude_prefixes: Vec<String>,
    validate: bool,
    minify: bool,
    nesting: bool,
    browsers: Option<Browsers>,
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
        Ok(if Self::flag(keyword, lookahead, input, exists)? {
            input.parse::<Colon>()?;
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
                source: Source::path(input.parse::<LitStr>()?.value())
                    .unwrap_or_else(|e| abort_call_site!(e)),
                visibility: None,
                prefix: None,
                include_prefixes: None,
                exclude_prefixes: Vec::new(),
                minify: false,
                validate: false,
                nesting: false,
                browsers: None,
            });
        }

        let mut path = None;
        let mut inline = None;
        let mut visibility = None;
        let mut prefix = None;
        let mut include_prefixes = None;
        let mut exclude_prefixes = Vec::new();
        let mut validate = false;
        let mut minify = false;
        let mut nesting = false;
        let mut browsers = None;

        parse_comma_delimited(input, |lookahead, input| {
            if Self::parameter(kw::path, lookahead, input, path.is_some())? {
                path = Some(input.parse::<LitStr>()?.value());
            } else if Self::parameter(kw::inline, lookahead, input, inline.is_some())? {
                inline = Some(input.parse::<LitStr>()?.value());
            } else if Self::parameter(kw::visibility, lookahead, input, visibility.is_some())? {
                visibility = Some(input.parse()?);
            } else if Self::parameter(kw::prefix, lookahead, input, prefix.is_some())? {
                prefix = Some(input.parse::<LitStr>()?.value());
            } else if Self::parameter(
                kw::include_prefixes,
                lookahead,
                input,
                include_prefixes.is_some(),
            )? {
                include_prefixes = Some(Self::parse_prefix_list(input)?);
            } else if Self::parameter(
                kw::exclude_prefixes,
                lookahead,
                input,
                !exclude_prefixes.is_empty(),
            )? {
                exclude_prefixes = Self::parse_prefix_list(input)?;
            } else if Self::flag(kw::validate, lookahead, input, validate)? {
                validate = true;
            } else if Self::flag(kw::minify, lookahead, input, minify)? {
                minify = true;
            } else if Self::flag(kw::nesting, lookahead, input, nesting)? {
                nesting = true;
            } else if Self::parameter(kw::browsers, lookahead, input, include_prefixes.is_some())? {
                browsers = Some(input.parse::<Browsers>()?);
            } else {
                return Ok(false);
            }

            Ok(true)
        })?;

        let source = match (path, inline) {
            (None, None) => abort_call_site!("Must specify either 'path' or `inline` parameter"),
            (None, Some(inline)) => Source::inline(inline),
            (Some(path), None) => Source::path(path).unwrap_or_else(|e| abort_call_site!(e)),
            (Some(_), Some(_)) => {
                abort_call_site!("Only one of 'path' or `inline` can be specified")
            }
        };

        Ok(Self {
            visibility,
            source,
            prefix,
            include_prefixes,
            exclude_prefixes,
            validate,
            minify,
            nesting,
            browsers,
        })
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

struct Browsers(lightningcss::targets::Browsers);

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
        Ok(if lookahead.peek(keyword) {
            if version.is_some() {
                abort!(
                    input.span(),
                    "{} is defined multiple times",
                    KeywordToken::display()
                );
            }

            input.parse::<KeywordToken>()?;
            input.parse::<Colon>()?;
            *version = Some(input.parse::<Version>()?.encode_for_lightning());

            true
        } else {
            false
        })
    }
}

impl Parse for Browsers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut browsers = lightningcss::targets::Browsers::default();
        let body;

        braced!(body in input);

        parse_comma_delimited(&body, |lookahead, input| {
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
        let major = Self::component(input)?;
        input.parse::<Colon>()?;
        let minor = Self::component(input)?;
        input.parse::<Colon>()?;
        let patch = Self::component(input)?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

fn any_prefix_matches(x: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| x.starts_with(prefix))
}

fn code_gen(
    visibility: Option<Visibility>,
    source: &Source,
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

    let dependency = source.dependency().iter();
    let content = source.content();

    quote!(
        #(const _: &[u8] = ::std::include_bytes!(#dependency);)*
        #(#classes)*
        #visibility fn stylesheet() -> &'static str {
            #content
        }
    )
    .into()
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
