use proc_macro::TokenStream;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::{Pub, Semi},
    FnArg, Pat, PatIdent, PatType, Signature, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(fallible);
    custom_keyword!(infallible);
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn client_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Fallible(fallible) = parse_macro_input!(attr);
    let ClientCommand {
        visibility,
        signature,
    } = parse_macro_input!(item);

    if let Some(constness) = signature.constness {
        abort!(constness, "Function can't be const");
    }

    if signature.asyncness.is_none() {
        abort_call_site!("Function must be async");
    }

    if let Some(variadic) = signature.variadic {
        abort!(variadic, "Function can't be variadic");
    }

    let fn_name = signature.ident.to_string();
    let arg_names = signature.inputs.iter().map(|arg| match arg {
        FnArg::Receiver(self_arg) => abort!(self_arg, "self arguments are not allowed"),
        FnArg::Typed(PatType { pat, .. }) => match pat.as_ref() {
            Pat::Ident(PatIdent { ident, .. }) => ident,
            _ => abort!(pat, "Arguments must be named"),
        },
    });

    let result_handler = if fallible {
        quote!(result
            .map(|ok| serde_wasm_bindgen::from_value(ok).unwrap())
            .map_err(|e| serde_wasm_bindgen::from_value(e).unwrap()))
    } else {
        quote!(serde_wasm_bindgen::from_value(result.unwrap()).unwrap())
    };

    quote!(
        #visibility #signature {
            use ::std::stringify;

            use ::silkenweb_tauri::{
                js_sys::{self, Object, Reflect},
                wasm_bindgen::{self, prelude::wasm_bindgen, JsValue},
                wasm_bindgen_futures,
                serde_wasm_bindgen,
            };

            #[wasm_bindgen(inline_js = r#"
                export async function invoke(name, args) {
                    return await window.__TAURI__.core.invoke(name, args);
                }
            "#)]

            extern "C" {
                #[wasm_bindgen(catch)]
                async fn invoke(name: String, args: JsValue) -> Result<JsValue, JsValue>;
            }

            let args = Object::new();

            #(Reflect::set(&args, &stringify!(#arg_names).into(), &serde_wasm_bindgen::to_value(&#arg_names).unwrap()).unwrap();)*

            let result = invoke(#fn_name.to_string(), args.into())
                .await;

            #result_handler
        }
    )
    .into()
}

struct Fallible(bool);

impl Parse for Fallible {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let fallible = if lookahead.peek(kw::fallible) {
            input.parse::<kw::fallible>()?;
            true
        } else if lookahead.peek(kw::infallible) {
            input.parse::<kw::infallible>()?;
            false
        } else {
            return Err(lookahead.error());
        };

        Ok(Self(fallible))
    }
}

struct ClientCommand {
    visibility: Option<Visibility>,
    signature: Signature,
}

impl Parse for ClientCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = if input.peek(Pub) {
            Some(input.parse()?)
        } else {
            None
        };

        let signature = input.parse()?;
        input.parse::<Semi>()?;

        Ok(Self {
            visibility,
            signature,
        })
    }
}
