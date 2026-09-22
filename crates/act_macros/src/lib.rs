use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};

mod attributes;
mod display;

use crate::display::derive_action_display;

#[proc_macro_derive(ActionDisplay, attributes(act))]
pub fn action_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    derive_action_display(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "this macro only supports enums")
}
