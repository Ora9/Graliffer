use std::fmt::{Debug, write};

use syn::{
    Attribute, ExprAssign, Ident, LitStr, Token,
    ext::IdentExt,
    parenthesized,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
};

#[derive(Debug)]
pub enum VariantAttr {
    Action(ActionVariantAttr),
    Nested(NestedVariantAttr),
}

impl VariantAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut this = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("act") {
                continue;
            }

            attr.parse_nested_meta(|meta| match meta.path.get_ident() {
                Some(ident) if ident == "action" => {
                    // #[act(action(..)))]
                    let content;
                    parenthesized!(content in meta.input);
                    let action_variant_attr: ActionVariantAttr = content.parse()?;

                    this = Self::Action(action_variant_attr);

                    Ok(())
                }
                Some(ident) if ident == "nested" => {
                    let nested = meta.value()?;

                    Ok(())
                }
                Some(_) | None => {
                    Err(meta
                        .error("unrecognized act attribute, expected either `action` or `nested`"))
                }
            })?;
        }

        Ok(this)
        // unimplemented!()
    }
}

impl Default for VariantAttr {
    fn default() -> Self {
        Self::Action(ActionVariantAttr::default())
    }
}

mod keyword {
    syn::custom_keyword!(human_name);
    syn::custom_keyword!(machine_name);
}

#[derive(Debug)]
pub enum ActionNameKind {
    Human,
    Machine,
}

pub struct ActionName(ActionNameKind, LitStr);

impl Parse for ActionName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let action_name = if lookahead.peek(keyword::human_name) {
            Ok(ActionNameKind::Human)
        } else if lookahead.peek(keyword::machine_name) {
            Ok(ActionNameKind::Machine)
        } else {
            Err(lookahead.error())
        }?;
        let _ident = Ident::parse_any(input)?;

        let _: Token![=] = input.parse()?;

        let name = input.parse()?;

        Ok(Self(action_name, name))
    }
}

impl Debug for ActionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionName {{{:?}: {}}}", self.0, self.1.value().clone())
    }
}

#[derive(Default)]
pub struct ActionVariantAttr {
    pub human_name: Option<LitStr>,
    pub machine_name: Option<LitStr>,
}

impl Parse for ActionVariantAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(input
            .parse_terminated(ActionName::parse, Token![,])?
            .into_iter()
            .fold(Self::default(), |mut acc, action_name| {
                match action_name.0 {
                    ActionNameKind::Human => acc.human_name = Some(action_name.1),
                    ActionNameKind::Machine => acc.machine_name = Some(action_name.1),
                }
                acc
            }))
    }
}

impl Debug for ActionVariantAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ActionVariantAttr {{ human_name: {}, machine_name: {} }}",
            match self.human_name {
                None => "None".to_string(),
                Some(ref lit_str) => format!("Some(\"{}\")", lit_str.value()),
            },
            match self.machine_name {
                None => "None".to_string(),
                Some(ref lit_str) => format!("Some(\"{}\")", lit_str.value()),
            }
        )
    }
}

pub enum NestedVariantAttr {
    Transparent,
    Namespace(Option<LitStr>),
}

impl Default for NestedVariantAttr {
    fn default() -> Self {
        Self::Namespace(None)
    }
}

impl Debug for NestedVariantAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NestedVariantAttr::{}",
            match self {
                NestedVariantAttr::Transparent => "Transparent".to_string(),
                NestedVariantAttr::Namespace(namespace) => match namespace {
                    None => "Namespace(None)".to_string(),
                    Some(lit_str) => format!("Namespace(Some(\"{}\"))", lit_str.value()),
                },
            }
        )
    }
}
