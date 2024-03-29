use crate::function_component::FunctionComponentName;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[derive(Clone)]
pub struct Autoprops {
    item_fn: syn::ItemFn,
    properties_name: syn::Ident,
}

impl syn::parse::Parse for Autoprops {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed: syn::Item = input.parse()?;

        let item_fn = match parsed {
            syn::Item::Fn(m) => m,
            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`autoprops` attribute can only be applied to functions",
                ))
            }
        };

        let syn::ItemFn { attrs, sig, .. } = &item_fn;

        let mut component_name = item_fn.sig.ident.clone();

        for attr in attrs {
            match &attr.meta {
                syn::Meta::List(syn::MetaList { path, tokens, .. }) => {
                    if let Some(last_segment) = path.segments.last() {
                        if last_segment.ident == "function_component" {
                            if let Ok(attr) = syn::parse2::<FunctionComponentName>(tokens.clone()) {
                                if let Some(name) = attr.component_name {
                                    component_name = name;
                                }
                            }
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        for input in &sig.inputs {
            if let syn::FnArg::Typed(syn::PatType { pat, .. }) = input {
                if let syn::Pat::Wild(wild) = pat.as_ref() {
                    return Err(syn::Error::new_spanned(
                        wild,
                        "cannot use `_` as field name",
                    ));
                }
            }
        }

        let properties_name = quote::format_ident!("{}Props", component_name);

        Ok(Self {
            properties_name,
            item_fn,
        })
    }
}

impl Autoprops {
    pub fn apply_args(&mut self, args: AutopropsArgs) {
        if let Some(name) = args.properties_name {
            self.properties_name = name;
        }
    }

    fn print_function_component(&self) -> proc_macro2::TokenStream {
        let properties_name = &self.properties_name;
        let syn::ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = &self.item_fn;

        let fn_name = &sig.ident;
        let (_impl_generics, type_generics, where_clause) = sig.generics.split_for_impl();
        let inputs = if self.needs_a_properties_struct() {
            // NOTE: function components currently don't accept receivers, we're just passing the
            //       information to the next macro to fail and give its own error message
            let receivers = sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Receiver(receiver) => Some(receiver),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let args = sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(syn::PatType { pat, .. }) => Some(quote! { #pat, }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let phantom = sig.generics.type_params().next().is_some().then(|| {
                quote! {
                    __yew_autoprops_phantom: _,
                }
            });
            quote! {
                #(#receivers,)* #properties_name {
                    #(#args)*
                    #phantom
                }: &#properties_name #type_generics
            }
        } else {
            quote! {}
        };
        let clones = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { pat, ty, .. })
                    if !matches!(**ty, syn::Type::Reference(_)) =>
                {
                    Some(quote! {
                        let #pat = ::yew::html::ImplicitClone::implicit_clone(#pat);
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let output = &sig.output;
        let impl_generics_with_defaults = &sig.generics;

        quote! {
            #(#attrs)*
            #vis fn #fn_name #impl_generics_with_defaults (#inputs) #output #where_clause {
                #(#clones)*
                #block
            }
        }
    }

    fn print_properties_struct(&self) -> proc_macro2::TokenStream {
        let properties_name = &self.properties_name;
        let syn::ItemFn { vis, sig, .. } = &self.item_fn;

        if !self.needs_a_properties_struct() {
            return quote! {};
        }

        let (impl_generics, type_generics, where_clause) = sig.generics.split_for_impl();
        let fields = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { attrs, pat, ty, .. }) => match ty.as_ref() {
                    syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                        Some(quote! { #(#attrs)* #vis #pat: #elem, })
                    }
                    _ => Some(quote! { #(#attrs)* #vis #pat: #ty, }),
                },
                _ => None,
            })
            .collect::<Vec<_>>();
        let type_params = sig
            .generics
            .type_params()
            .map(|param| &param.ident)
            .collect::<Vec<_>>();
        let phantom = (!type_params.is_empty()).then(|| {
            quote! {
                #[prop_or_default]
                #vis __yew_autoprops_phantom: ::std::marker::PhantomData <( #(#type_params),* )>,
            }
        });
        let fields_eq = sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => Some(quote_spanned! {
                    ty.span() => self.#pat == rhs.#pat &&
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        let impl_generics_with_defaults = &sig.generics;

        quote! {
            #[derive(::yew::Properties)]
            #vis struct #properties_name #impl_generics_with_defaults #where_clause {
                #(#fields)*
                #phantom
            }

            impl #impl_generics ::std::cmp::PartialEq for #properties_name #type_generics
            #where_clause {
                fn eq(&self, rhs: &Self) -> ::std::primitive::bool {
                    #(#fields_eq)* true
                }
            }
        }
    }

    fn needs_a_properties_struct(&self) -> bool {
        let syn::ItemFn { sig, .. } = &self.item_fn;
        !sig.inputs.is_empty()
    }
}

impl quote::ToTokens for Autoprops {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let function_component = self.print_function_component();
        let properties_struct = self.print_properties_struct();

        tokens.extend(quote! {
            #function_component
            #properties_struct
        })
    }
}

pub struct AutopropsArgs {
    pub properties_name: Option<syn::Ident>,
}

impl syn::parse::Parse for AutopropsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                properties_name: None,
            });
        }

        let properties_name = input.parse()?;

        Ok(Self {
            properties_name: Some(properties_name),
        })
    }
}
