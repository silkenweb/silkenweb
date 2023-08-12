mod parse;

use grass::InputSyntax;
use parse::Transpile;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_style::{css, css::Source};
use syn::{parse_macro_input, Ident};

use crate::parse::Input;

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

    let variables =
        css::variable_names(&source).map(|variable| (variable.clone(), format!("--{variable}")));

    let classes = if let Some(name_mappings) = name_mappings {
        let mut classes = Vec::new();

        for (ident, mapping) in name_mappings {
            if !mapping.composes.is_empty() {
                abort_call_site!("`composes` is unsupported");
            }

            classes.push((ident, mapping.name));
        }

        classes
    } else {
        css::class_names(&source)
            .map(|class| (class.clone(), class))
            .collect()
    };

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
    names: impl Iterator<Item = (String, String)> + 'a,
) -> impl Iterator<Item = (String, String)> + 'a {
    names.filter(move |(class_ident, _css_class)| {
        let include = if let Some(include_prefixes) = include_prefixes.as_ref() {
            any_prefix_matches(class_ident, include_prefixes)
        } else {
            true
        };

        let exclude = any_prefix_matches(class_ident, exclude_prefixes);

        include && !exclude
    })
}

fn strip_prefixes<'a>(
    prefix: &'a str,
    names: impl Iterator<Item = (String, String)> + 'a,
) -> impl Iterator<Item = (String, String)> + 'a {
    names.filter_map(move |(ident, mapping)| {
        ident
            .strip_prefix(prefix)
            .map(str::to_string)
            .map(|ident| (ident, mapping))
    })
}

fn any_prefix_matches(x: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| x.starts_with(prefix))
}

fn code_gen(
    source: &Source,
    public: bool,
    auto_mount: bool,
    classes: impl Iterator<Item = (String, String)>,
    variables: impl Iterator<Item = (String, String)>,
) -> TokenStream {
    let classes = classes.map(|(ident, name)| define_css_entity(ident, name, auto_mount));
    let variables = variables.map(|(ident, name)| define_css_entity(ident, name, auto_mount));

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
                    render::DocumentRender,
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

fn define_css_entity(ident: String, name: String, auto_mount: bool) -> proc_macro2::TokenStream {
    if !ident.starts_with(char::is_alphabetic) {
        abort_call_site!(
            "Identifier '{}' doesn't start with an alphabetic character",
            ident
        );
    }

    let ident = ident.replace(|c: char| !c.is_alphanumeric(), "_");

    if auto_mount {
        let ident = Ident::new(&ident.to_lowercase(), Span::call_site());
        quote!(pub fn #ident() -> &'static str {
            use ::std::{panic::Location, sync::Once};

            static INIT: Once = Once::new();

            INIT.call_once(|| {
                super::stylesheet::mount()
            });

            #name
        })
    } else {
        let ident = Ident::new(&ident.to_uppercase(), Span::call_site());
        quote!(pub const #ident: &str = #name;)
    }
}
