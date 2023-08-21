use grass::InputSyntax;
use parse::Transpile;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_base::css::{NameMapping, Source};
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, LitBool,
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
            type Dom = <#target_type as ::silkenweb::node::element::Element>::Dom;
            type DomElement = <#target_type as ::silkenweb::node::element::Element>::DomElement;

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

            fn style_property<'a>(
                self,
                name: impl Into<String>,
                value: impl ::silkenweb::value::RefSignalOrValue<'a, Item = impl AsRef<str> + 'a>
            ) -> Self {
                Self{#target: self.#target.style_property(name, value) #other_fields}
            }

            fn effect(self, f: impl FnOnce(&Self::DomElement) + 'static) -> Self {
                Self{#target: self.#target.effect(f) #other_fields}
            }

            fn effect_signal<T: 'static>(
                self,
                sig: impl ::silkenweb::macros::Signal<Item = T> + 'static,
                f: impl Fn(&Self::DomElement, T) + Clone + 'static,
            ) -> Self {
                Self{#target: self.#target.effect_signal(sig, f) #other_fields}
            }

            fn handle(&self) -> ::silkenweb::node::element::ElementHandle<Self::Dom, Self::DomElement> {
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
    let path = attr.path();

    if !path.is_ident(name) {
        abort!(path, "Expected `{}`", name);
    }

    attr.parse_nested_meta(|meta| {
        if !meta.path.is_ident(value) {
            abort!(meta.path, "Expected `{}`", value);
        }

        if !meta.input.is_empty() {
            abort!(meta.input.span(), "Unexpected token");
        }

        Ok(())
    })
    .unwrap()
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

#[proc_macro_attribute]
#[proc_macro_error]
pub fn cfg_browser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let in_browser: LitBool = parse_macro_input!(attr);

    let cfg_check = if in_browser.value() {
        quote!(#[cfg(all(target_arch = "wasm32", target_os = "unknown"))])
    } else {
        quote!(#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))])
    };
    let item = proc_macro2::TokenStream::from(item);

    quote!(
        #cfg_check
        #item
    )
    .into()
}

#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let Input {
        mut source,
        syntax,
        public,
        prefix,
        include_prefixes,
        exclude_prefixes,
        validate,
        auto_mount,
        transpile,
    } = parse_macro_input!(input);

    let syntax = syntax.into();

    if syntax != InputSyntax::Css {
        source = source
            .map_content(|content| {
                grass::from_string(content, &grass::Options::default().input_syntax(syntax))
            })
            .unwrap_or_else(|e| abort_call_site!(e));
    }

    let name_mappings = source
        .transpile(validate, transpile.map(Transpile::into))
        .unwrap_or_else(|e| abort_call_site!(e));

    let variables = source.variable_names().map(|variable| NameMapping {
        plain: variable.clone(),
        mangled: format!("--{variable}"),
    });

    let classes = name_mappings.unwrap_or_else(|| {
        source
            .class_names()
            .map(|class| NameMapping {
                plain: class.clone(),
                mangled: class,
            })
            .collect()
    });

    let classes = only_matching_prefixes(&include_prefixes, &exclude_prefixes, classes.into_iter());
    let variables = only_matching_prefixes(&include_prefixes, &exclude_prefixes, variables);

    if let Some(prefix) = prefix {
        let classes = strip_prefixes(&prefix, classes);
        let variables = strip_prefixes(&prefix, variables);
        code_gen(&source, public, auto_mount, classes, variables)
    } else {
        code_gen(&source, public, auto_mount, classes, variables)
    }
}

fn only_matching_prefixes<'a>(
    include_prefixes: &'a Option<Vec<String>>,
    exclude_prefixes: &'a [String],
    names: impl Iterator<Item = NameMapping> + 'a,
) -> impl Iterator<Item = NameMapping> + 'a {
    names.filter(move |mapping| {
        let ident = &mapping.plain;

        let include = if let Some(include_prefixes) = include_prefixes.as_ref() {
            any_prefix_matches(ident, include_prefixes)
        } else {
            true
        };

        let exclude = any_prefix_matches(ident, exclude_prefixes);

        include && !exclude
    })
}

fn strip_prefixes<'a>(
    prefix: &'a str,
    names: impl Iterator<Item = NameMapping> + 'a,
) -> impl Iterator<Item = NameMapping> + 'a {
    names.filter_map(move |NameMapping { plain, mangled }| {
        plain
            .strip_prefix(prefix)
            .map(str::to_string)
            .map(|plain| NameMapping { plain, mangled })
    })
}

fn any_prefix_matches(x: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| x.starts_with(prefix))
}

fn code_gen(
    source: &Source,
    public: bool,
    auto_mount: bool,
    classes: impl Iterator<Item = NameMapping>,
    variables: impl Iterator<Item = NameMapping>,
) -> TokenStream {
    let classes = classes.map(|name| define_css_entity(name, auto_mount));
    let variables = variables.map(|name| define_css_entity(name, auto_mount));

    let dependency = source.dependency().iter();
    let content = source.content();
    let visibility = if public { quote!(pub) } else { quote!() };

    quote!(
        #(const _: &[u8] = ::std::include_bytes!(#dependency);)*

        #visibility mod class {
            #(#classes)*
        }

        #visibility mod var {
            #(#variables)*
        }

        #visibility mod stylesheet {
            pub fn mount() {
                use ::std::panic::Location;
                use silkenweb::{
                    document::Document,
                    dom::DefaultDom,
                    node::element::ParentElement,
                    elements::html::style,
                };

                let location = Location::caller();
                DefaultDom::mount_in_head(
                    &format!(
                        "silkenweb-style:{}:{}:{}",
                        location.file(),
                        location.line(),
                        location.column()
                    ),
                    style().text(text())
                );
            }

            pub fn text() -> &'static str {
                #content
            }
        }
    )
    .into()
}

fn define_css_entity(name: NameMapping, auto_mount: bool) -> proc_macro2::TokenStream {
    let NameMapping { plain, mangled } = name;

    if !plain.starts_with(char::is_alphabetic) {
        abort_call_site!(
            "Identifier '{}' doesn't start with an alphabetic character",
            plain
        );
    }

    let ident = plain.replace(|c: char| !c.is_alphanumeric(), "_");

    if auto_mount {
        let ident = Ident::new(&ident.to_lowercase(), Span::call_site());
        quote!(pub fn #ident() -> &'static str {
            use ::std::{panic::Location, sync::Once};

            static INIT: Once = Once::new();

            INIT.call_once(|| {
                super::stylesheet::mount()
            });

            #mangled
        })
    } else {
        let ident = Ident::new(&ident.to_uppercase(), Span::call_site());
        quote!(pub const #ident: &str = #mangled;)
    }
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
