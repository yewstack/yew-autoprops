// copy-pasted from yew-macro because proc-macro crates cannot export any items

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
