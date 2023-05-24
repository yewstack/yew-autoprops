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

    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let visibility = &function.vis;
    let generics = &function.sig.generics;

    let component_name = match args.len() {
        0 => None,
        1 => {
            let TokenTree::Ident(name) = &args[0] else {
                panic!("Invalid argument: {}", args[0].to_string());
            };

            Some(name)
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

    let mut fields = Vec::new();
    let mut arg_types = Vec::new();
    for input in function.sig.inputs.iter() {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = input {
            let Type::Reference(ty) = ty.as_ref() else {
                panic!(
                    "Invalid argument: {} (must be a reference)",
                    input.to_token_stream()
                );
            };

            let ty = &ty.elem;
            fields.push(quote! {
                #(#attrs)*
                pub #pat: #ty
            });
            arg_types.push(ty.clone());
        } else {
            panic!("Invalid argument");
        }
    }

    let partial_eq_constraints = arg_types.iter().map(|ty| quote! { #ty: PartialEq });

    let struct_name = syn::Ident::new(&format!("{}Props", fn_name), Span::call_site().into());
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let bounds = generics.where_clause.clone();

    let where_clause = if generics.params.is_empty() {
        quote! {}
    } else {
        quote! {
            where
                #(#partial_eq_constraints),*
                #bounds,
        }
    };

    let destructure = quote! {
        #struct_name { #(#arg_names),* }
    };

    let function_block = function.block;

    let tokens = quote! {
        #[derive(::yew::Properties, PartialEq)]
        #visibility struct #struct_name #impl_generics #where_clause {
            #(#fields),*
        }

        #[::yew::function_component(#component_name)]
        #[allow(non_snake_case)]
        #visibility fn #fn_name #impl_generics (#destructure: &#struct_name #ty_generics) -> ::yew::Html #where_clause {
            #function_block
        }
    };

    // panic!("\n{}", tokens.to_string());

    // Return the new tokens
    tokens.into()
}
