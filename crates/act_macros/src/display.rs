use quote::quote;
use syn::LitStr;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

use crate::attributes::parse_variant_attrs;
use crate::non_enum_error;

pub fn derive_action_display(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let item_ident = ast.ident;

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variants = match &ast.data {
        Data::Enum(data) => &data.variants,
        _ => return Err(non_enum_error()),
    };

    let mut machine_arms: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut human_arms: Vec<proc_macro2::TokenStream> = Vec::new();
    // let namespace = LitStr::new(&item_ident.to_string(), Span::call_site());

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_name = syn::LitStr::new(variant_ident.to_string().as_str(), variant.span());

        let variant_attr = parse_variant_attrs(&variant.attrs)?;

        // machine name priority : (UpperCamelCase convention)
        //  - attribute `name`
        //  - variant identifier
        //
        // human name priority : (lower case with spaces)
        //  - attribute `human_name`
        //  - machine name (in lowercase)

        let machine_name = variant_attr.machine_name.unwrap_or(variant_name);
        let human_name = variant_attr.human_name.unwrap_or(LitStr::new(
            // TODO: use a better to_lowercase, currently TwoWords results in twowords
            // we should insert space at each word boudaries, see crate heck
            &machine_name.clone().value().to_lowercase(),
            machine_name.span(),
        ));

        let (machine_arm, human_arm) = match variant.fields {
            Fields::Unit => {
                let machine = quote! { #item_ident::#variant_ident => #machine_name };
                let human = quote! { #item_ident::#variant_ident => #human_name};

                (machine, human)
            }
            _ => {
                // TODO: point to #[act(skip)]
                return Err(syn::Error::new_spanned(
                    variant,
                    "non unit variants cannot be constructed or displayed",
                ));
            }
        };

        machine_arms.push(machine_arm);
        human_arms.push(human_arm);
    }

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::act::ActionDisplay for #item_ident #ty_generics #where_clause {
            // fn namespace() -> &'static str {
            //     #namespace
            // }

            fn machine_name(&self) -> &'static str {
                match *self {
                    #(#machine_arms),*
                }
            }

            fn human_name(&self) -> &'static str {
                match *self {
                    #(#human_arms),*
                }
            }
        }
    };

    Ok(proc_macro2::TokenStream::from(expanded))
}
