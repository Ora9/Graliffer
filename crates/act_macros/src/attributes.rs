use syn::{Attribute, LitStr};

#[derive(Default)]
pub(crate) struct VariantAttr {
    pub machine_name: Option<LitStr>,
    pub human_name: Option<LitStr>,
}

pub fn parse_variant_attrs(attrs: &[Attribute]) -> syn::Result<VariantAttr> {
    let mut variant_attr = VariantAttr::default();

    for attr in attrs {
        if !attr.path().is_ident("act") {
            continue;
        }

        attr.parse_nested_meta(|meta| match meta.path.get_ident() {
            Some(ident) if ident == "machine_name" => {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;

                variant_attr.machine_name = Some(lit);
                Ok(())
            }
            Some(ident) if ident == "human_name" => {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;

                variant_attr.human_name = Some(lit);
                Ok(())
            }
            Some(_) | None => Err(meta.error(
                "unrecognized act attribute, expected either `machine_name` or `human_name`",
            )),
        })?;
    }

    Ok(variant_attr)
}
