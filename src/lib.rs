mod autoprops;
mod function_component;

use quote::ToTokens;

//#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn autoprops(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut autoprops = syn::parse_macro_input!(item as autoprops::Autoprops);
    let args = syn::parse_macro_input!(attr as autoprops::AutopropsArgs);
    autoprops.apply_args(args);

    proc_macro::TokenStream::from(autoprops.into_token_stream())
}
