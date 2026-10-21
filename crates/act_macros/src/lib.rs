#![allow(unused)]

use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};

mod action;
mod display;

mod attributes;

use crate::{action::derive_action, display::derive_action_display};

#[proc_macro_derive(Action)]
pub fn action_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    derive_action(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Convert [`Actions`] to strings
///
/// Currently only derivable on enums
///
/// `ActionDisplay` exposes two display format :
///  - "machine" name, used by and for machines (e.g. in a config/keymap file)
///  - "human" name, primarly used to display to the user (e.g. in a command picker ..)
///
/// # Action naming convention
///
/// Human format should be :
/// - **short / concise**, use surrounding words to clarify meaning (eg. `InsertMode` could be expressed as `switch to insert mode`, or `Quit` as `Quit application`)
/// - **in all lowercase, with spaces as word boudaries**,
///
/// Machine format should be :
/// - **close to the variant identifier**, should only be set if there would be dilemma between
///   enum variant identifier and
///
/// # Fallbacks
/// Default values are given to each variant when no specific name is given (e.g. with no
/// `#[act(action(human_name = "some_name"))]`) using the following logic :
///
/// machine name is set to :
///  - helper attribute `machine_name`
///  - enum variant identifier
///
/// human name is set to :
///  - helper attribute `human_name`
///  - machine name (transformed to lowercase, with spaces between word boudaries)
///
/// //    ```rust
/// //     #[derive(act::ActionDisplay)]
/// //     enum PondAction {
/// //         #[act(action(human_name = "say hi"))]
/// //         SayHi,
/// //
/// //         #[act(action(human_name = "set camp fire"))]
/// //         SetCampFire,
/// //
/// //         #[act(action(human_name = "swim"))]
/// //         Swim,
/// //     }
/// //     ```
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
