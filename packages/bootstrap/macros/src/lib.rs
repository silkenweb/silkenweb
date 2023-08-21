use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use silkenweb_css::Css;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
#[proc_macro_error]
pub fn define_icons(input: TokenStream) -> TokenStream {
    let path: LitStr = parse_macro_input!(input);
    let path = path.value();
    let css = Css::from_path(path, None).unwrap_or_else(|e| abort_call_site!(e));
    let classes = css.class_names().filter_map(|class| Class::new(&class));

    code_gen(css.dependency(), classes)
}

struct Class {
    css_class: String,
    enum_variant: Ident,
    fn_name: Ident,
}

impl Class {
    fn new(class: &str) -> Option<Self> {
        class.strip_prefix("bi-").map(|unprefixed_class| {
            let class_ident = if unprefixed_class.starts_with(char::is_alphabetic) {
                unprefixed_class
            } else {
                class
            };

            let css_class = class.to_owned();
            let enum_variant = Ident::new(&class_ident.to_upper_camel_case(), Span::call_site());
            let snake_ident = class_ident.to_snake_case();
            let fn_name = match snake_ident.as_str() {
                "box" | "type" => Ident::new_raw(&snake_ident, Span::call_site()),
                _ => Ident::new(&snake_ident, Span::call_site()),
            };

            Self {
                css_class,
                enum_variant,
                fn_name,
            }
        })
    }
}

fn code_gen(dependency: &Option<String>, classes: impl Iterator<Item = Class>) -> TokenStream {
    let classes: Vec<_> = classes.collect();

    let enum_variants = classes.iter().map(|class| &class.enum_variant);
    let match_arms = classes.iter().map(|class| {
        let enum_variant = &class.enum_variant;
        let css_class = &class.css_class;

        quote!(
            Self::#enum_variant => #css_class
        )
    });

    let icon_fns = classes.iter().map(|class| {
        let enum_variant = &class.enum_variant;
        let fn_name = &class.fn_name;

        quote!(
            pub fn #fn_name() -> Icon {
                icon(IconType::#enum_variant)
            }
        )
    });

    let dependency = dependency.iter();

    quote!(
        #(const _: &[u8] = ::std::include_bytes!(#dependency);)*

        #[derive(Copy, Clone, Eq, PartialEq, Value)]
        pub enum IconType {
            #(#enum_variants,)*
        }

        impl IconType {
            pub fn class(self) -> Class {
                match self {
                    #(#match_arms,)*
                }
            }
        }

        impl Icon {
            #(#icon_fns)*
        }
    )
    .into()
}
