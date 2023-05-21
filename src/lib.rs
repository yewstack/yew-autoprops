//! proc-macro to automatically derive Properties structs from args for Yew components
//!
//! No more extra one-off Props structs!
//!
//!
//! # Examples
//!
//! ```
//! use yew_autoprops::autoprops_component;
//! use yew::prelude::*;
//!
//! #[autoprops_component]
//! fn CoolComponent(#[prop_or_default] test: &i8, smth: &usize) -> Html {
//!     println!("test: {}", test);
//!
//!     html! {
//!         <div>
//!             <p>{ smth }</p>
//!         </div>
//!     }
//! }
//! ```
//!
//! ```
//! use yew_autoprops::autoprops_component;
//! use yew::prelude::*;
//!
//! #[autoprops_component(CoolComponent)]
//! fn cool_component(#[prop_or_default] test: &i8, smth: &usize) -> Html {
//!     println!("test: {}", test);
//!
//!     html! {
//!         <div>
//!             <p>{ smth }</p>
//!         </div>
//!     }
//! }
//! ```
//!
extern crate proc_macro;

use core::panic;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, Type};

#[proc_macro_attribute]
pub fn autoprops_component(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = TokenStream::from(args).into_iter().collect::<Vec<_>>();

    let mut function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;

    let component_name = match args.len() {
        0 => fn_name,
        1 => {
            let TokenTree::Ident(name) = &args[0] else {
                panic!("Invalid argument: {}", args[0].to_string());
            };

            name
        }
        _ => panic!("Invalid arguments: {:?}", args),
    };

    let arg_names = function
        .sig
        .inputs
        .iter()
        .map(|input| {
            if let FnArg::Typed(PatType { pat, .. }) = input {
                if let Pat::Ident(ident) = pat.as_ref() {
                    return &ident.ident;
                }
            }
            panic!("Invalid argument: {}", input.to_token_stream());
        })
        .collect::<Vec<_>>();

    let fields = function
        .sig
        .inputs
        .iter()
        .map(|input| {
            if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = input {
                let Type::Reference(ty) = ty.as_ref() else {
                    panic!("Invalid argument: {} (must be a reference)", input.to_token_stream());
                };

                let ty = &ty.elem;

                return quote! {
                    #(#attrs)*
                    pub #pat: #ty
                };
            }
            panic!("Invalid argument");
        })
        .collect::<Vec<_>>();

    let struct_name = syn::Ident::new(&format!("{}Props", fn_name), Span::call_site().into());

    let fn_name_outer = syn::Ident::new(&format!("{}_outer", fn_name), Span::call_site().into());

    let destructure_and_call = quote! {
        let #struct_name { #(#arg_names),* } = props;

        #fn_name(#(#arg_names),*)
    };

    for inp in &mut function.sig.inputs {
        let pat = match inp {
            FnArg::Typed(pat) => pat,
            _ => panic!("Invalid argument: self?"),
        };

        pat.attrs = vec![];
    }

    let tokens = quote! {
        #[derive(::yew::Properties, PartialEq)]
        pub struct #struct_name {
            #(#fields),*
        }

        #[::yew::function_component(#component_name)]
        #[allow(non_snake_case)]
        fn #fn_name_outer(props: &#struct_name) -> ::yew::Html {

            #[allow(non_snake_case)]
            #function

            #destructure_and_call
        }
    };

    // Return the new tokens
    tokens.into()
}
