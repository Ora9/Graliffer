/// Mod for parsing `act` helper attribute
///
/// ```text
/// #[act(action(human_name = "X"))]
/// #[act(action(machine_name = "X"))]
/// #[act(action(human_name = "X", machine_name = "X"))]
/// #[act(nested(namespace = "X"))]
/// #[act(nested(transparent))]
/// ```
use std::fmt::{Debug, write};

use syn::{
    Attribute, ExprAssign, Ident, LitStr, Token,
    ext::IdentExt,
    parenthesized,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    token,
};

pub enum VariantAttr {
    Action(ActionVariantAttr),
    Nested(NestedVariantAttr),
    Skip,
}

impl VariantAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut this = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("act") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("action") {
                    let mut action_names = ActionVariantAttr::default();
                    meta.parse_nested_meta(|meta| {
                        if meta.path.is_ident("human_name") {
                            let name: LitStr = meta.value()?.parse()?;
                            action_names.human_name = Some(name);
                            Ok(())
                        } else if meta.path.is_ident("machine_name") {
                            let name: LitStr = meta.value()?.parse()?;
                            action_names.machine_name = Some(name);
                            Ok(())
                        } else {
                            Err(meta.error("unsupported action name format, expected either `human_name` or `machine_name`"))
                        }
                    })
                    .and_then(|_| {
                        this = Self::Action(action_names);
                        Ok(())
                    })
                } else if meta.path.is_ident("nested") {
                    if meta.input.peek(token::Paren) {
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("transparent") {
                                this = Self::Nested(NestedVariantAttr::Transparent);
                                Ok(())
                            } else if meta.path.is_ident("namespace") {
                                let namespace: LitStr = meta.value()?.parse()?;
                                this = Self::Nested(NestedVariantAttr::Namespace(Some(namespace)));

                                Ok(())
                            } else {
                                Err(meta.error("unsupported action nesting attribute, expected either `namespace` or `transparent`"))
                            }
                        })
                    } else {
                        this = Self::Nested(NestedVariantAttr::Namespace(None));
                        Ok(())
                    }

                } else if meta.path.is_ident("skip") {
                    this = Self::Skip;
                    Ok(())
                } else {
                    Err(meta.error("unsupported act attribute, expected either `action` or `nested`"))
                }
            })?;
        }

        Ok(this)
    }
}

impl Default for VariantAttr {
    fn default() -> Self {
        Self::Action(ActionVariantAttr::default())
    }
}

#[derive(Default)]
pub struct ActionVariantAttr {
    pub human_name: Option<LitStr>,
    pub machine_name: Option<LitStr>,
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
