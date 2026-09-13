use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    AppAction,
    input::{KeyContext, KeyContextPredicate, Keystroke},
};

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

    /// Find an [`AppAction`] tied to a [`Keystroke`] depending on the given [`Context`]
    pub fn find_action(&self, keystroke: Keystroke, in_context: &KeyContext) -> Option<AppAction> {
        // todo: make more specific context predicate have a higher priorities
        // or maybe just by order of declaration?

        let mut potential_action: Option<AppAction> = None;

        self.0
            .iter()
            .filter(|binding_group| in_context.matches(&binding_group.context))
            .find_map(|binding_group| binding_group.find_action(keystroke))
    }

    /// Find a [`Keystroke`] tied to an [`AppAction`] depending on the given [`Context`]
    pub fn find_keystroke(&self, action: AppAction, in_context: &KeyContext) -> Option<Keystroke> {
        self.0
            .iter()
            .filter(|binding_group| in_context.matches(&binding_group.context))
            .find_map(|binding_group| binding_group.find_keystroke(&action))
    }
}

impl Default for Keymap {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_KEYMAP).expect("default keymap must be valid!")
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeymapBindingGroup {
    context: KeyContextPredicate,
    bindings: HashMap<Keystroke, AppAction>,
}

impl KeymapBindingGroup {
    // fn push(&mut self, keystroke: Keystroke, action: AppAction) {
    //     self.bindings.insert(keystroke, action);
    // }

    fn find_action(&self, keystroke: Keystroke) -> Option<AppAction> {
        self.bindings
            .iter()
            .find(|(entry_keystroke, _)| **entry_keystroke == keystroke)
            .map(|(_, action)| action.clone())
    }

    fn find_keystroke(&self, action: &AppAction) -> Option<Keystroke> {
        self.bindings
            .iter()
            .find(|(_, entry_action)| *entry_action == action)
            .map(|(keystroke, _)| *keystroke)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binding {
    keystroke: Keystroke,
    action: AppAction,
}

impl Binding {
    pub fn new(keystroke: Keystroke, action: AppAction) -> Self {
        Self { keystroke, action }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        GralifferAction::{self, FocusGrid},
        input::{Key, KeyContext, key},
    };

    use super::*;

    fn test_keymap() -> Keymap {
        let keymap = json!([
            {
                "context": "",
                "bindings": {
                    "1": "FocusGrid",
                    "2": "FocusStack",
                    "3": "FocusConsole"
                }
            },
            {
                "context": "A",
                "bindings": {
                    "1": "ToggleCommandPicker",
                    "2": "ToggleAbout"
                }
            },
            {
                "context": "B",
                "bindings": {
                    "1": "InsertMode",
                    "2": "CommandMode"
                }
            }
        ]);

        serde_json::from_value(keymap).expect("test_keymap must be valid")
    }

    #[test]
    fn parse_test_keymap() {
        test_keymap();
    }

    #[test]
    fn parse_default_keymap() {
        Keymap::default();
    }

    #[test]
    fn find_action() {
        let keymap = test_keymap();

        let mut context = KeyContext::default();

        // context.insert("A".into());
        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('1')), &context),
            Some(AppAction::GralifferAction(GralifferAction::FocusGrid))
        );

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('2')), &context),
            Some(AppAction::GralifferAction(GralifferAction::FocusStack))
        );

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('3')), &context),
            Some(AppAction::GralifferAction(GralifferAction::FocusConsole))
        );

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('4')), &context),
            None
        );

        context.insert("A".into());

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('1')), &context),
            Some(AppAction::GralifferAction(
                GralifferAction::ToggleCommandPicker
            ))
        );

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('2')), &context),
            Some(AppAction::GralifferAction(GralifferAction::ToggleAbout))
        );

        assert_eq!(
            keymap.find_action(Keystroke::from_key(Key::Char('3')), &context),
            None
        );
    }

    #[test]
    fn find_keystroke() {
        let keymap = test_keymap();

        let mut context = KeyContext::default();

        context.insert("A".into());
        assert_eq!(
            keymap.find_keystroke(AppAction::GralifferAction(FocusGrid), &context),
            Some(Keystroke::from_key(Key::Char('1')))
        );
    }
}
