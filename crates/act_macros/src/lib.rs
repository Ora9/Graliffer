use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};

mod attributes;
mod display;

use crate::display::derive_action_display;

/// Convert actions to string
///
/// Currently only derivable on enums
///
/// As a convention, actions have multiples serialisation/deserialisation format :
///  - "machine" name, used by and for machines
///  - "human" name, only used as a
///
/// See trait [`act::ActionDisplay`] for more infos on differents serialisation, but in short :
///  - "human" name, used only as
///
/// machine name priority : (UpperCamelCase convention)
///  - attribute `name`
///  - variant identifier
///
/// human name priority : (lower case with spaces)
///  - attribute `human_name`
///  - machine name (in lowercase)
///
///     ```
///     #[derive(act::ActionDisplay)]
///     enum PondAction {
///         #[act(human_name = "say hi")]
///         SayHi,
///
///         #[act(human_name = "set camp fire")]
///         SetCampFire,
///
///         #[act(human_name = "swim")]
///         Swim,
///     }
///     ```
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
