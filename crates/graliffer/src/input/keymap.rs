use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{AnyAppAction, Context, KeyContextPredicate, Keystroke};

static DEFAULT_KEYMAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/default_keymap.jsonc"
));

#[derive(Debug, Serialize, Deserialize)]
pub struct Keymap(Vec<KeymapBindingGroup>);

impl Keymap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, binding_group: KeymapBindingGroup) {
        self.0.push(binding_group);
    }

    pub fn find(&self, app_context: Context, keystroke: Keystroke) -> Option<AnyAppAction> {
        // todo: make more specific context predicate have a higher priorities
        // or maybe just by order of declaration?

        self.0
            .iter()
            .filter(|binding_group| app_context.matches_key_context(&binding_group.context))
            .find_map(|binding_group| binding_group.find_keystroke(keystroke))
    }
}

impl Default for Keymap {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_KEYMAP).expect("default keymap must be valid!")
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeymapBindingGroup {
    // #[serde(rename = "context")]
    context: KeyContextPredicate,
    bindings: HashMap<Keystroke, AnyAppAction>,
}

impl KeymapBindingGroup {
    fn new(context: KeyContextPredicate) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }

    fn push(&mut self, keystroke: Keystroke, action: AnyAppAction) {
        self.bindings.insert(keystroke, action);
    }

    fn find_keystroke(&self, keystroke: Keystroke) -> Option<AnyAppAction> {
        self.bindings
            .iter()
            .find(|(entry_keystroke, _)| **entry_keystroke == keystroke)
            .and_then(|(_, action)| Some(action.clone()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binding {
    keystroke: Keystroke,
    action: AnyAppAction,
}

impl Binding {
    pub fn new(keystroke: Keystroke, action: AnyAppAction) -> Self {
        Self { keystroke, action }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_keymap() {
        Keymap::default();
    }
}
