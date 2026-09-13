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
    ///
    /// # Precedence
    /// When a keystroke leads to multiples keybind, precedence is resolved with two rules :
    /// - [`Context`] that are more "specific" takes precedence (simply meaning having more operations)
    /// - Declaration order with latest declared taking precedence. User defined keymap are loaded
    ///   after default ones, allowing user bindings to take overwrite defaults
    pub fn find_action(&self, keystroke: Keystroke, in_context: &KeyContext) -> Option<AppAction> {
        let mut selected_action: Option<AppAction> = None;
        let mut highest_score = 0;

        self.0
            .iter()
            .filter(|binding_group| in_context.matches(&binding_group.context))
            .for_each(|binding_group| {
                let score = binding_group.context.specificity_score();
                if score >= highest_score
                    && let Some(action) = binding_group.find_action(keystroke)
                {
                    highest_score = score;
                    selected_action = Some(action);
                }
            });

        selected_action
    }

    /// Find a [`Keystroke`] tied to an [`AppAction`] depending on the given [`Context`]
    ///
    /// # Precedence
    /// When an action is used in multiple bindings, precedence is resolved with two rules :
    /// - [`Context`] that are more "specific" takes precedence (simply meaning having more operations)
    /// - Declaration order with latest declared taking precedence. User defined keymap are loaded
    ///   after default ones, allowing user bindings to take overwrite defaults
    pub fn find_keystroke(&self, action: AppAction, in_context: &KeyContext) -> Option<Keystroke> {
        let mut selected_keystroke: Option<Keystroke> = None;
        let mut highest_score = 0;

        self.0
            .iter()
            .filter(|binding_group| in_context.matches(&binding_group.context))
            .for_each(|binding_group| {
                let score = binding_group.context.specificity_score();
                if score >= highest_score
                    && let Some(keystroke) = binding_group.find_keystroke(&action)
                {
                    highest_score = score;
                    selected_keystroke = Some(keystroke);
                }
            });

        selected_keystroke
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
        ConsoleAction,
        GralifferAction::{self},
        GridAction,
        input::{Key, KeyContext},
    };

    use super::*;

    #[test]
    fn parse_default_keymap() {
        Keymap::default();
    }

    #[test]
    fn find() {
        let keymap_json = json!([
            {
                "context": "",
                "bindings": {
                    "1": "FocusGrid",
                    "2": "FocusStack",
                    "3": "FocusConsole"
                }
            },
            {
                "context": "A B &&",
                "bindings": {
                    "1": "console::ScrollUp",
                    "2": "console::ScrollDown",
                    "3": "console::ScrollBottom"
                }
            },
            {
                "context": "A",
                "bindings": {
                    "1": "grid::Undo",
                    "2": "grid::Redo"
                }
            },
            {
                "context": "B",
                "bindings": {
                    "1": "picker::SelectionUp",
                    "2": "picker::SelectionDown"
                }
            }
        ]);

        let keymap: Keymap = serde_json::from_value(keymap_json).unwrap();

        let mut context = KeyContext::default();

        let find =
            |in_context: &KeyContext, keystroke: Option<Keystroke>, action: Option<AppAction>| {
                if let Some(keystroke) = keystroke {
                    assert_eq!(keymap.find_action(keystroke, in_context), action);
                }

                if let Some(action) = action {
                    assert_eq!(keymap.find_keystroke(action, in_context), keystroke);
                }
            };

        find(&context, Some(Keystroke::from_key(Key::Up)), None);

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusGrid)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusGrid)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusConsole)),
        );

        context.insert("A".into());

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GridAction(GridAction::Undo)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusConsole)),
        );

        context.insert("B".into());

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::ConsoleAction(ConsoleAction::ScrollUp)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::ConsoleAction(ConsoleAction::ScrollBottom)),
        );
    }
}
