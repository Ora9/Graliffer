use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

use crate::attributes::VariantAttr;
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
    let namespace = LitStr::new(&item_ident.to_string(), item_ident.span());

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_name =
            syn::LitStr::new(variant_ident.to_string().as_str(), variant_ident.span());

        let variant_attr = VariantAttr::from_attrs(&variant.attrs)?;

        let (machine_arm, human_arm) = match variant_attr {
            VariantAttr::Action(action_variant_attr) => {
                let machine_name = action_variant_attr.machine_name.unwrap_or(variant_name);
                let human_name = action_variant_attr.human_name.unwrap_or(LitStr::new(
                    &machine_name.value().to_case(Case::Lower),
                    machine_name.span(),
                ));

                match variant.fields {
                    Fields::Unit => {
                        let machine_arm = quote! { #item_ident::#variant_ident => #machine_name };
                        let human_arm = quote! { #item_ident::#variant_ident => #human_name};

                        Ok((machine_arm, human_arm))
                    }
                    _ => Err(syn::Error::new_spanned(
                        variant,
                        "non unit variants cannot be constructed or displayed, you might want to use `skip` or `namespace` attributes",
                    )),
                }?
            }
            VariantAttr::Nested(nested_variant_attr) => {
                match variant.fields {
                    Fields::Unnamed(ref field) => {
                        unimplemented!()
                    }
                    _ => Err(syn::Error::new_spanned(
                        variant,
                        "nested action can only be defined with a tuple like variant (eg. `Variant(NestedAction)`)",
                    )),
                }?;

                unimplemented!()
            }
            VariantAttr::Skip => {
                unimplemented!()
            }
        };

        machine_arms.push(machine_arm);
        human_arms.push(human_arm);
    }

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::act::ActionDisplay for #item_ident #ty_generics #where_clause {
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
