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
//! fn CoolComponent(#[prop_or_default] hidden: bool, smth: &AttrValue) -> Html {
//!     html! {
//!         <div class={classes!(hidden.then_some("hidden"))}>
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
//! fn cool_component(#[prop_or_default] hidden: bool, smth: &AttrValue) -> Html {
//!     html! {
//!         <div class={classes!(hidden.then_some("hidden"))}>
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

    let (component_name, struct_name) = match args.len() {
        0 => (
            None,
            Some(syn::Ident::new(
                &format!("{}Props", fn_name),
                Span::call_site().into(),
            )),
        ),
        1 => {
            let TokenTree::Ident(name) = &args[0] else {
                panic!("Invalid argument: {}", args[0].to_string());
            };

            (
                Some(name),
                Some(syn::Ident::new(
                    &format!("{}Props", name),
                    Span::call_site().into(),
                )),
            )
        }
        3 => {
            let TokenTree::Ident(name) = &args[0] else {
                panic!("Invalid argument: {}", args[0].to_string());
            };

            let TokenTree::Ident(props) = args[2].clone() else {
                panic!("Invalid argument: {}", args[2].to_string());
            };

            (Some(name), Some(props))
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
    let mut clones = Vec::new();
    let mut partial_eq_constraints = Vec::new();
    for input in function.sig.inputs.iter() {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = input {
            let mut end_ty = ty;

            if let Type::Reference(ty_ref) = ty.as_ref() {
                end_ty = &ty_ref.elem;
            } else {
                clones.push(quote! {
                    let #pat = #pat.clone();
                });
            }

            fields.push(quote! {
                #(#attrs)*
                pub #pat: #end_ty
            });
            arg_types.push(ty.clone());
            partial_eq_constraints.push(quote! { #end_ty: PartialEq, });
        } else {
            panic!("Invalid argument");
        }
    }

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let bounds = generics.where_clause.clone();

    let where_clause = if generics.params.is_empty() {
        quote! {}
    } else {
        quote! {
            where
                #(#partial_eq_constraints)*
                #bounds
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
            #(#clones)*
            #function_block
        }
    };

    // panic!("\n{}", tokens.to_string());

    // Return the new tokens
    tokens.into()
}
