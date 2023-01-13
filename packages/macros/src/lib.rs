use parse::Transpile;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_base::css::{self, Source};
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, Meta, NestedMeta, Visibility,
};

use crate::parse::Input;

mod parse;

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
            let item: DeriveInput = parse_macro_input!(item);
            let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
            let name = item.ident;

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

#[proc_macro_derive(ChildElement, attributes(child_element))]
#[proc_macro_error]
pub fn derive_child_element(item: TokenStream) -> TokenStream {
    let item: DeriveInput = parse_macro_input!(item);
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let item_name = item.ident;

    let fields = fields(item.data);
    let target_index = target_field_index("child_element", &fields);

    let target_field = fields[target_index].clone();
    let target_type = target_field.ty;
    let target = field_token(target_index, target_field.ident);
    let dom_type = quote!(<#target_type as ::silkenweb::dom::InDom>::Dom);

    quote!(
        impl #impl_generics ::std::convert::From<#item_name #ty_generics>
        for ::silkenweb::node::element::GenericElement<
            #dom_type,
            ::silkenweb::node::element::Const
        >
        #where_clause
        {
            fn from(value: #item_name #ty_generics) -> Self {
                value.#target.into()
            }
        }

        impl #impl_generics ::std::convert::From<#item_name #ty_generics>
        for ::silkenweb::node::Node<#dom_type>
        #where_clause
        {
            fn from(value: #item_name #ty_generics) -> Self {
                value.#target.into()
            }
        }

        impl #impl_generics ::silkenweb::value::Value
        for #item_name #ty_generics #where_clause {}
    )
    .into()
}

#[proc_macro_derive(Element, attributes(element))]
#[proc_macro_error]
pub fn derive_element(item: TokenStream) -> TokenStream {
    let item: DeriveInput = parse_macro_input!(item);
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let item_name = item.ident;

    let fields = fields(item.data);
    let target_index = target_field_index("element", &fields);

    let field = fields[target_index].clone();
    let target_type = field.ty;

    let other_field_idents = fields.into_iter().enumerate().filter_map(|(index, field)| {
        (index != target_index).then(|| field_token(index, field.ident))
    });
    let other_fields = quote!(#(, #other_field_idents: self.#other_field_idents)*);

    let target = field_token(0, field.ident);

    quote!(
        impl #impl_generics ::silkenweb::node::element::Element
        for #item_name #ty_generics #where_clause {
            type DomType = <#target_type as ::silkenweb::node::element::Element>::DomType;

            fn class<'a, T>(self, class: impl ::silkenweb::value::RefSignalOrValue<'a, Item = T>) -> Self
            where
                T: 'a + AsRef<str>
            {
                Self {#target: self.#target.class(class) #other_fields}
            }

            fn classes<'a, T, Iter>(
                self,
                classes: impl ::silkenweb::value::RefSignalOrValue<'a, Item = Iter>,
            ) -> Self
            where
                T: 'a + AsRef<str>,
                Iter: 'a + IntoIterator<Item = T>,
            {
                    Self {#target: self.#target.classes(classes) #other_fields}
            }

            fn attribute<'a>(
                mut self,
                name: &str,
                value: impl ::silkenweb::value::RefSignalOrValue<'a, Item = impl ::silkenweb::attribute::Attribute>,
            ) -> Self {
                Self{#target: self.#target.attribute(name, value) #other_fields}
            }

            fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
                Self{#target: self.#target.effect(f) #other_fields}
            }

            fn effect_signal<T: 'static>(
                self,
                sig: impl ::silkenweb::macros::Signal<Item = T> + 'static,
                f: impl Fn(&Self::DomType, T) + Clone + 'static,
            ) -> Self {
                Self{#target: self.#target.effect_signal(sig, f) #other_fields}
            }

            fn handle(&self) -> ::silkenweb::node::element::ElementHandle<Self::DomType> {
                self.#target.handle()
            }

            fn spawn_future(self, future: impl ::std::future::Future<Output = ()> + 'static) -> Self {
                Self{#target: self.#target.spawn_future(future) #other_fields}
            }

            fn on(self, name: &'static str, f: impl FnMut(::silkenweb::macros::JsValue) + 'static) -> Self {
                Self{#target: self.#target.on(name, f) #other_fields}
            }
        }
    )
    .into()
}

/// Find the index of the field with `#[<attr_name>(target)]`
fn target_field_index(attr_name: &str, fields: &[Field]) -> usize {
    let mut target_index = None;

    for (index, field) in fields.iter().enumerate() {
        for attr in &field.attrs {
            if target_index.is_some() {
                abort!(attr, "Only one target field can be specified");
            }

            check_attr_matches(attr, attr_name, "target");
            target_index = Some(index);
        }
    }

    target_index.unwrap_or_else(|| {
        if fields.len() != 1 {
            abort_call_site!(
                "There must be exactly one field, or specify `#[{}(target)]` on a single field",
                attr_name
            );
        }

        0
    })
}

/// Make sure an attribute matches #[<name>(<value>)]
fn check_attr_matches(attr: &Attribute, name: &str, value: &str) {
    if !attr.path.is_ident(name) {
        abort!(attr.path, "Expected `{}`", name);
    }

    if let Meta::List(list) = attr.parse_meta().unwrap() {
        let mut target_list = list.nested.iter();

        if let Some(NestedMeta::Meta(Meta::Path(target_path))) = target_list.next() {
            if !target_path.is_ident(value) {
                abort!(target_path, "Expected `{}`", value);
            }

            if target_list.next().is_some() {
                abort!(list, "Expected `{}({})`", name, value);
            }

            return;
        } else {
            abort!(list, "Expected `{}({})`", name, value);
        }
    }

    abort!(attr, "Expected `{}({})`", name, value);
}

fn fields(struct_data: Data) -> Vec<Field> {
    let fields = match struct_data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => abort_call_site!("Only structs are supported"),
    };

    match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
        Fields::Named(FieldsNamed { named, .. }) => named,
        Fields::Unit => abort!(fields, "There must be at least one field"),
    }
    .into_iter()
    .collect()
}

fn field_token(index: usize, ident: Option<Ident>) -> proc_macro2::TokenStream {
    if let Some(ident) = ident {
        quote!(#ident)
    } else {
        let index = Index::from(index);
        quote!(#index)
    }
}

/// Define `&str` constants for each class in a CSS file.
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
/// Parameters take the form:
///
/// ```
/// # use silkenweb_macros::css;
/// css!(
///     path: "my-css-file.css",
///     visibility: pub,
///     prefix: "prefix",
///     include_prefixes: ["included-"],
///     exclude_prefixes: ["excluded-"],
///     validate,
///     transpile: (
///         minify,
///         pretty,
///         nesting,
///         browsers: (
///             android: (1, 0, 0),
///             chrome: (1, 0, 0),
///             edge: (1, 0, 0),
///             firefox: (1, 0, 0),
///             ie: (1, 0, 0),
///             ios_saf: (1, 0, 0),
///             opera: (1, 0, 0),
///             safari: (1, 0, 0),
///             samsung: (1, 0, 0),
///         )
///     )
/// );
/// ```
///
/// All are optional, but one of `path` or `inline` must be specified.
///
/// - `path` is the path to the CSS /SCSS/SASS file.
/// - `inline` is the css content.
/// - `visibility` is any visibility modifier, and controls the visibility of
///   class constants. Private is the default.
/// - `prefix`: only classes starting with `prefix` should be included. Their
///   Rust names will have the prefix stripped.
/// - `include_prefixes`: a list of prefixes to include, without stripping the
///   prefix. Rust constants will only be defined for classes starting with one
///   or more of these prefixes.
/// - `exclude_prefixes`: a list of prefixes to exclude. No Rust constants will
///   be defined for a class starting with any of these prefixes.
///   `exclude_prefixes` takes precedence over `include_prefixes`.
/// - `validate`: validate the CSS.
/// - `transpile`: transpile the CSS with [lightningcss].
///
/// ## `transpile`
///
/// - `minify`: Minify the CSS returned by `stylesheet()`. Minification also
///   adds/removes vendor prefixes, so it's a good idea to keep this the same
///   between debug and release builds. Use `pretty` if you want legible CSS in
///   debug.
/// - `pretty`: Pretty print the final output. This is the default unless minify
///   is specified.
/// - `nesting`: Allow CSS nesting.
/// - `browsers` is a comma seperated list of the minimum supported browser
///   versions. This will add vendor prefixes to the CSS from `stylesheet()`.
///   The version is a paranthesized `,` seperated string of major, minor, and
///   patch versions. For example, to support firefox 110  + and chrome 111+,
///   use `browsers: ( firefox: (110, 0, 0), chrome: (111, 0, 0) )`.
///
/// # Examples
///
/// Define private constants for all CSS classes:
///
/// ```
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
/// 
/// [lightningcss]: https://lightningcss.dev/
#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let Input {
        visibility,
        mut source,
        prefix,
        include_prefixes,
        exclude_prefixes,
        validate,
        transpile,
    } = parse_macro_input!(input);

    source
        .transpile(validate, transpile.map(Transpile::into))
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
