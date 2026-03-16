// copy-pasted from yew-macro because proc-macro crates cannot export any items
// TODO separate yew's macros into their own library common for yew-macro and yew-autoprops

use syn::parse::{Parse, ParseStream};
use syn::Ident;

pub struct FunctionComponentName {
    pub component_name: Option<Ident>,
}

impl Parse for FunctionComponentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                component_name: None,
            });
        }

        let component_name = input.parse()?;

        Ok(Self {
            component_name: Some(component_name),
        })
    }
}
