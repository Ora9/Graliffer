use quote::quote;
use syn::DeriveInput;

pub fn derive_action(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let item_ident = ast.ident;

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::act::Action for #item_ident #ty_generics #where_clause {};
    };

    Ok(proc_macro2::TokenStream::from(expanded))
}
